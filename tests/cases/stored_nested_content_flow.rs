//! 저장 위치가 없는 셀의 세로 정렬과 본문 표 뒤 흐름의 독립 합성 계약.
//!
//! 빈 문서에서 생성한 fixture이며 사용자 양식의 내용은 포함하지 않는다.
//! Top 배치에서 확인한 점유 영역으로 Center/Bottom의 이동량을 정한다.
//! HWP5 계보와 순수 HWPX, 유효한 저장 앵커와 0으로 초기화된 앵커를 구분한다.
#![cfg(not(target_arch = "wasm32"))]

use rhwp::document_core::DocumentCore;
use rhwp::renderer::render_tree::{BoundingBox, RenderNode, RenderNodeType};
use std::path::Path;

fn nodes(name: &str) -> Vec<RenderNode> {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("samples/stored-nested-content-flow")
        .join(name);
    let core = DocumentCore::from_bytes(&std::fs::read(file).expect("read fixture"))
        .expect("parse fixture");
    assert_eq!(core.page_count(), 1);
    let tree = core.build_page_render_tree(0).expect("render page");
    fn collect(node: &RenderNode, output: &mut Vec<RenderNode>) {
        output.push(node.clone());
        for child in &node.children {
            collect(child, output);
        }
    }
    let mut output = Vec::new();
    collect(&tree.root, &mut output);
    output
}

fn text(nodes: &[RenderNode], expected: &str) -> BoundingBox {
    nodes
        .iter()
        .find_map(|node| match &node.node_type {
            RenderNodeType::TextRun(run) if run.text == expected => Some(node.bbox),
            _ => None,
        })
        .expect("expected text")
}

fn table(nodes: &[RenderNode], width: f64) -> BoundingBox {
    let matches: Vec<_> = nodes
        .iter()
        .filter(|node| {
            matches!(node.node_type, RenderNodeType::Table { .. })
                && (node.bbox.width - width).abs() < 0.5
        })
        .collect();
    assert_eq!(matches.len(), 1, "unique table width {width}");
    matches[0].bbox
}

#[test]
fn nested_cell_alignment_uses_the_sequential_anchor_when_stored_positions_reset() {
    for prefix in ["", "pure-"] {
        for stored in ["collapsed", "intact"] {
            let top = nodes(&format!("{prefix}cell-top-{stored}.hwpx"));
            let outer = table(&top, 320.0);
            let nested = table(&top, 160.0);
            let slack = (outer.y + outer.height - nested.y - nested.height).max(0.0);
            let top_y = text(&top, "Start").y;
            for (align, fraction) in [("center", 0.5), ("bottom", 1.0)] {
                let aligned = nodes(&format!("{prefix}cell-{align}-{stored}.hwpx"));
                let advance = text(&aligned, "Start").y - top_y;
                assert!(
                    (advance - slack * fraction).abs() < 0.6,
                    "{prefix}{stored}/{align}: advance {advance}, expected {}",
                    slack * fraction
                );
            }
        }
    }
}

#[test]
fn preceding_negative_spacing_is_not_used_as_the_inline_table_host_spacing() {
    for prefix in ["", "pure-"] {
        for spacing in [-600, 0] {
            let page = nodes(&format!("{prefix}body-{spacing}.hwpx"));
            let outer = table(&page, 320.0);
            let footer = text(&page, "Footer");
            assert!(
                footer.y >= outer.y + outer.height - 0.5,
                "{prefix}{spacing}: footer {} precedes table bottom {}",
                footer.y,
                outer.y + outer.height
            );
        }
    }
}

#[test]
fn nested_alignment_uses_rewrapped_text_and_actual_float_flow() {
    for kind in ["overlay", "square", "flow", "width"] {
        let extension = if kind == "width" { "hwp" } else { "hwpx" };
        let top = nodes(&format!("{kind}-top.{extension}"));
        let outer = table(&top, 320.0);
        let nested_bottom = top
            .iter()
            .filter(|node| {
                matches!(node.node_type, RenderNodeType::Table { .. }) && node.bbox.width < 319.5
            })
            .map(|node| node.bbox.y + node.bbox.height)
            .reduce(f64::max)
            .expect("nested tables");
        let first_y = |page: &[RenderNode]| {
            page.iter()
                .find_map(|node| match &node.node_type {
                    RenderNodeType::TextRun(run)
                        if run
                            .text
                            .starts_with(if kind == "width" { "Wide" } else { "Start" }) =>
                    {
                        Some(node.bbox.y)
                    }
                    _ => None,
                })
                .expect("first text")
        };
        let top_y = first_y(&top);
        if kind == "width" {
            assert!(
                top.iter().any(|node| {
                    matches!(&node.node_type, RenderNodeType::TextRun(run) if run.text.contains("text"))
                        && node.bbox.y > top_y + 1.0
                }),
                "stored text must actually wrap onto a later line"
            );
        }
        let slack = (outer.y + outer.height - nested_bottom).max(0.0);
        for (align, fraction) in [("center", 0.5), ("bottom", 1.0)] {
            let aligned = nodes(&format!("{kind}-{align}.{extension}"));
            let actual = first_y(&aligned) - top_y;
            assert!(
                (actual - slack * fraction).abs() < 0.6,
                "{kind}/{align}: advance {actual}, expected {}",
                slack * fraction
            );
        }
    }
}

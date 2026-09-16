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
    nodes_from_file(&file)
}

fn nodes_from_file(file: &Path) -> Vec<RenderNode> {
    let core = DocumentCore::from_bytes(&std::fs::read(file).expect("read fixture"))
        .expect("parse fixture");
    nodes_from_core(&core)
}

fn nodes_from_core(core: &DocumentCore) -> Vec<RenderNode> {
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

#[test]
fn empty_leading_paragraph_keeps_its_line_space_in_nested_table_alignment() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pr7200_empty_leading_paragraph");
    let top = nodes_from_file(&fixture.join("empty-first-top.hwpx"));
    let outer = table(&top, 320.0);
    let nested = table(&top, 160.0);
    // 한컴 PDF의 LEFT/offset=0 앵커는 x=48.32px다. 가운데 배치(128px) 금지.
    assert!((nested.x - 48.32).abs() < 1.0);
    // The source line reserves (1000 + 200) HWPUNIT at 96 DPI.
    // An empty glyph run does not erase this explicit line box.
    assert!((nested.y - outer.y - 16.0).abs() < 0.6);
    let slack = outer.y + outer.height - nested.y - nested.height;
    for (align, fraction) in [("center", 0.5), ("bottom", 1.0)] {
        let aligned = nodes_from_file(&fixture.join(format!("empty-first-{align}.hwpx")));
        let advance = table(&aligned, 160.0).y - nested.y;
        assert!(
            (advance - slack * fraction).abs() < 0.6,
            "empty/{align}: advance {advance}, expected {}",
            slack * fraction
        );
    }
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

#[test]
fn hancom_recomposed_lines_and_nested_table_match_pdf_positions() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/pr7200_hancom_recomposed");
    // 같은 HWP를 Hancom 12.0.0.4605로 출력한 PDF 텍스트 상단(96 DPI).
    // 원본의 잘못된 단일 저장 줄은 별도 합성 경계로 유지한다.
    for (align, expected) in [
        (
            "top",
            [127.718, 150.094, 172.629, 195.164, 217.540, 259.734],
        ),
        (
            "center",
            [244.550, 267.086, 289.621, 311.997, 334.532, 376.566],
        ),
        (
            "bottom",
            [361.542, 384.078, 406.453, 428.989, 451.524, 493.558],
        ),
    ] {
        let rendered = nodes_from_file(&fixture.join(format!("width-{align}.hwp")));
        let text_nodes: Vec<_> = rendered.iter().filter(|node| {
            matches!(&node.node_type, RenderNodeType::TextRun(run) if !run.text.trim().is_empty())
        }).collect();
        assert_eq!(text_nodes.len(), expected.len(), "{align}: 5줄과 End 보존");
        for (node, pdf_y) in text_nodes.iter().zip(expected) {
            assert!((node.bbox.x - 48.774).abs() < 1.0, "{align}: PDF 가로 원점");
            assert!(
                (node.bbox.y - pdf_y).abs() < 1.0,
                "{align}: y={} PDF={pdf_y}",
                node.bbox.y
            );
        }
        assert!((table(&rendered, 160.0).x - 48.32).abs() < 1.0);
        // PDF의 End 문단 테두리는 마지막 줄간격까지 포함한 약 22.5px다.
        // 테두리 연결이 꺼진 셀 문단을 본문/부모 셀 테두리와 병합하지 않는다.
        assert!(
            rendered.iter().any(|node| {
                matches!(node.node_type, RenderNodeType::Rectangle(_))
                    && (node.bbox.width - 160.0).abs() < 0.5
                    && (node.bbox.y - expected[5]).abs() < 1.0
                    && (node.bbox.height - 22.5).abs() < 0.5
            }),
            "{align}: 셀의 마지막 문단 테두리 누락/축소"
        );
        assert!(
            rendered.iter().any(|node| {
                matches!(node.node_type, RenderNodeType::Rectangle(_))
                    && (node.bbox.width - 384.0).abs() < 0.5
                    && node.bbox.height > 400.0
            }),
            "{align}: TAC 표 호스트의 본문 문단 테두리 누락"
        );
    }
}

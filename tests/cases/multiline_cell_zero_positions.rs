//! 저장 세로 위치가 모두 0인 두 줄은 한 줄 높이로 가운데 정렬하지 않는다.
use rhwp::document_core::DocumentCore;
use rhwp::model::control::Control;
use rhwp::model::table::VerticalAlign;
use serde_json::Value;

const FIXTURE: &[u8] = include_bytes!("../fixtures/multiline_cell_zero_positions/two_lines.hwpx");

fn check(core: &DocumentCore, align: VerticalAlign, height: u32) {
    let controls: Value =
        serde_json::from_str(&core.get_page_control_layout_native(0).unwrap()).unwrap();
    let cell = &controls["controls"][0]["cells"][0];
    let top = cell["y"].as_f64().unwrap();
    let bottom = top + f64::from(height) / 75.0;
    let text: Value = serde_json::from_str(&core.get_page_text_layout_native(0).unwrap()).unwrap();
    let runs: Vec<_> = text["runs"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|run| matches!(run["text"].as_str(), Some("First line" | "Second line")))
        .collect();
    assert_eq!(runs.len(), 2, "두 줄의 텍스트를 보존한다");
    let first = runs[0]["y"].as_f64().unwrap();
    let last = runs[1]["y"].as_f64().unwrap() + runs[1]["h"].as_f64().unwrap();
    assert!(
        first >= top && last <= bottom + 0.2,
        "{align:?}: 두 줄 {first}..{last}가 셀 {top}..{bottom} 안에 있어야 한다"
    );
    assert!(
        (runs[1]["y"].as_f64().unwrap() - first - 1000.0 / 75.0).abs() < 0.2,
        "두 줄의 간격은 저장된 1000 HU이다"
    );
    let expected = match align {
        VerticalAlign::Top => top + 140.0 / 75.0,
        VerticalAlign::Center => top + (f64::from(height) - 2000.0) / 150.0,
        VerticalAlign::Bottom => bottom - 2140.0 / 75.0,
    };
    assert!(
        (first - expected).abs() < 0.2,
        "{align:?}: first={first}, expected={expected}"
    );
}

#[test]
fn two_lines_with_zero_positions_keep_their_full_alignment_height() {
    for align in [
        VerticalAlign::Top,
        VerticalAlign::Center,
        VerticalAlign::Bottom,
    ] {
        for height in [2280, 4280] {
            for second_position in [0, 1000] {
                let source = DocumentCore::from_bytes(FIXTURE).unwrap();
                let mut doc = source.document().clone();
                let Control::Table(table) = &mut doc.sections[0].paragraphs[1].controls[0] else {
                    panic!("합성 표");
                };
                table.common.height = height;
                table.cells[0].height = height;
                table.cells[0].vertical_align = align;
                table.cells[0].paragraphs[0].line_segs[1].vertical_pos = second_position;
                let mut core = DocumentCore::new_empty();
                core.set_document(doc);
                check(&core, align, height);
                let reopened =
                    DocumentCore::from_bytes(&core.export_hwp_native().unwrap()).unwrap();
                check(&reopened, align, height);
            }
        }
    }
}

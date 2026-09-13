//! Canvas measurement lifetime and output isolation, using the original #7084 fixtures.
use rhwp::document_core::{CanvasMetricBatch, CanvasMetricReply};
use rhwp::paint::layer_tree::{LayerNode, LayerNodeKind};
use rhwp::paint::paint_op::PaintOp;
use rhwp::paint::profile::RenderProfile;
use rhwp::renderer::canvas_text_font::CanvasTextFont;
use rhwp::renderer::layout::{EmbeddedTextMeasurer, TextMeasurer};
use rhwp::renderer::render_tree::TextRunNode;
use rhwp::renderer::supplemental_metrics::{
    MetricBackend, MetricContext, MetricError, SupplementalMetric,
};
use rhwp::DocumentCore;

fn replies(batch: &CanvasMetricBatch) -> Vec<CanvasMetricReply> {
    batch
        .requests
        .iter()
        .map(|r| CanvasMetricReply {
            key: r.key.clone(),
            cluster: r.cluster.clone(),
            font: r.font.clone(),
            resolved_font: r.font.clone(),
            measured_advance_px: 18.302703857,
        })
        .collect()
}

#[test]
fn collected_original_requests_register_and_preserve_ir_and_portable_svg() {
    for ext in ["hwp", "hwpx"] {
        let mut core = core(ext);
        let original = format!("{:?}", core.document());
        let baseline = run(&core);
        let svg = core.render_page_svg_native(0).unwrap();
        let batch = core.collect_canvas_metric_requests(context()).unwrap();
        assert!(batch.requests.iter().any(|r| r.cluster == "😀"), "{ext}");
        assert!(
            batch
                .requests
                .iter()
                .all(|r| r.cluster.chars().count() == 1
                    && !r.cluster.chars().any(char::is_whitespace))
        );
        assert!(core
            .register_canvas_metric_replies(context(), batch.revision, replies(&batch))
            .unwrap());
        core.select_canvas_metrics(true).unwrap();
        assert_ne!(positions(&run(&core)), positions(&baseline));
        let warm = core.collect_canvas_metric_requests(context()).unwrap();
        assert!(!core
            .register_canvas_metric_replies(context(), warm.revision, replies(&warm))
            .unwrap());
        core.select_canvas_metrics(false).unwrap();
        assert_eq!(core.render_page_svg_native(0).unwrap(), svg);
        assert_eq!(format!("{:?}", core.document()), original);
    }
}

#[test]
fn request_envelope_rejects_partial_duplicate_and_descriptor_mismatch_atomically() {
    let mut core = core("hwp");
    let baseline = run(&core);
    let batch = core.collect_canvas_metric_requests(context()).unwrap();
    let mut partial = replies(&batch);
    partial.pop().unwrap();
    assert_eq!(
        core.register_canvas_metric_replies(context(), batch.revision, partial),
        Err(MetricError::ContextMismatch)
    );
    let mut invalid = replies(&batch);
    invalid[0].font = "99px serif".into();
    assert_eq!(
        core.register_canvas_metric_replies(context(), batch.revision, invalid),
        Err(MetricError::InvalidDescriptor)
    );
    if batch.requests.len() > 1 {
        let mut duplicate = replies(&batch);
        duplicate[1].key = duplicate[0].key.clone();
        assert_eq!(
            core.register_canvas_metric_replies(context(), batch.revision, duplicate),
            Err(MetricError::DuplicateKey)
        );
    }
    assert!(!core.canvas_metrics_active());
    assert_eq!(positions(&run(&core)), positions(&baseline));
    assert!(core
        .register_canvas_metric_replies(context(), batch.revision, replies(&batch))
        .unwrap());
}

#[test]
fn requests_are_bound_to_latest_ticket_and_source_revision() {
    let mut core = core("hwp");
    let old = core.collect_canvas_metric_requests(context()).unwrap();
    let current = core.collect_canvas_metric_requests(context()).unwrap();
    assert_ne!(old.revision, current.revision);
    assert_eq!(
        core.register_canvas_metric_replies(context(), old.revision, replies(&old)),
        Err(MetricError::ContextMismatch)
    );
    core.apply_char_format_in_cell_native(0, 12, 1, 5, 0, 0, 3, r##"{"textColor":"#ff0000"}"##)
        .unwrap();
    assert_eq!(
        core.register_canvas_metric_replies(context(), current.revision, replies(&current)),
        Err(MetricError::ContextMismatch)
    );
    let fresh = core.collect_canvas_metric_requests(context()).unwrap();
    assert!(core
        .register_canvas_metric_replies(context(), fresh.revision, replies(&fresh))
        .unwrap());
}

fn context() -> MetricContext {
    MetricContext {
        document_generation: 1,
        font_generation: 1,
        backend: MetricBackend::Canvas2d,
    }
}

fn core(ext: &str) -> DocumentCore {
    let path = format!(
        "{}/samples/issue3587/c-form-labnote-001-stage11-filled.{ext}",
        env!("CARGO_MANIFEST_DIR")
    );
    DocumentCore::from_bytes(&std::fs::read(path).unwrap()).unwrap()
}

fn find(node: &LayerNode) -> Option<TextRunNode> {
    match &node.kind {
        LayerNodeKind::Group { children, .. } => children.iter().find_map(find),
        LayerNodeKind::ClipRect { child, .. } => find(child),
        LayerNodeKind::Leaf { ops } => ops.iter().find_map(|op| match op {
            PaintOp::TextRun { run, .. } if run.text.contains('😀') => Some((**run).clone()),
            _ => None,
        }),
    }
}

fn run(core: &DocumentCore) -> TextRunNode {
    for page in 0..2 {
        let tree = core
            .build_canvas_page_layer_tree_with_profile(page, RenderProfile::Screen)
            .unwrap();
        if let Some(run) = find(&tree.root) {
            return run;
        }
    }
    panic!("fixture must contain the reported emoji");
}

fn entries(run: &TextRunNode) -> Vec<SupplementalMetric> {
    let font = CanvasTextFont::for_positioned_text(&run.style, 0.0);
    vec![SupplementalMetric::from_canvas_measurement(
        &run.style,
        "😀",
        font.descriptor(),
        font.descriptor().into(),
        18.302703857,
    )
    .unwrap()]
}

fn positions(run: &TextRunNode) -> Vec<f64> {
    // K0 runs intentionally omit layout_positions. Their painter computes the
    // replay positions from the tree's resolved style, not a stored position array.
    run.layout_positions
        .clone()
        .unwrap_or_else(|| EmbeddedTextMeasurer.compute_char_positions(&run.text, &run.style))
}

#[test]
fn canvas_session_switch_rebuilds_positions_and_protects_portable_output() {
    for ext in ["hwp", "hwpx"] {
        let mut core = core(ext);
        let original = format!("{:?}", core.document());
        let baseline = run(&core);
        let svg = core.render_page_svg_native(0).unwrap();
        core.begin_canvas_metric_session(context()).unwrap();
        assert!(core
            .register_canvas_metrics(context(), entries(&baseline))
            .unwrap());
        assert_eq!(core.render_page_svg_native(0).unwrap(), svg);
        assert!(core.select_canvas_metrics(true).unwrap());
        let active = run(&core);
        assert!(active.style.supplemental_metrics.is_some());
        assert_ne!(
            positions(&active),
            positions(&baseline),
            "cached positions must be rebuilt: {ext}"
        );
        assert!(core.render_page_svg_native(0).is_err());
        assert!(core.build_page_render_tree(0).is_err());
        assert!(core
            .build_page_layer_tree_with_profile(0, RenderProfile::Screen)
            .is_err());
        assert!(!core
            .register_canvas_metrics(context(), entries(&baseline))
            .unwrap());
        assert!(!core.select_canvas_metrics(true).unwrap());
        assert!(core.select_canvas_metrics(false).unwrap());
        assert_eq!(positions(&run(&core)), positions(&baseline));
        assert_eq!(core.render_page_svg_native(0).unwrap(), svg);
        assert_eq!(
            format!("{:?}", core.document()),
            original,
            "metrics must not edit document IR"
        );
    }
}

#[test]
fn new_generation_invalidates_retained_styles_and_rejects_late_results() {
    let mut core = core("hwp");
    let baseline = run(&core);
    core.begin_canvas_metric_session(context()).unwrap();
    core.register_canvas_metrics(context(), entries(&baseline))
        .unwrap();
    core.select_canvas_metrics(true).unwrap();
    let retained = run(&core);
    core.begin_canvas_metric_session(MetricContext {
        font_generation: 2,
        ..context()
    })
    .unwrap();
    assert!(!core.canvas_metrics_active());
    assert_eq!(
        core.register_canvas_metrics(context(), entries(&baseline)),
        Err(MetricError::ContextMismatch)
    );
    assert_eq!(positions(&run(&core)), positions(&baseline));
    assert_eq!(
        EmbeddedTextMeasurer.compute_char_positions("😀", &retained.style),
        EmbeddedTextMeasurer.compute_char_positions("😀", &baseline.style)
    );
}

#[test]
fn document_replacement_drops_owner_and_batch_rejects_metric_mutation() {
    let mut core = core("hwpx");
    let baseline = run(&core);
    core.begin_canvas_metric_session(context()).unwrap();
    core.register_canvas_metrics(context(), entries(&baseline))
        .unwrap();
    core.select_canvas_metrics(true).unwrap();
    core.begin_batch_native().unwrap();
    assert_eq!(
        core.select_canvas_metrics(false),
        Err(MetricError::EditInProgress)
    );
    assert_eq!(
        core.register_canvas_metrics(context(), entries(&baseline)),
        Err(MetricError::EditInProgress)
    );
    assert_eq!(
        core.begin_canvas_metric_session(context()),
        Err(MetricError::EditInProgress)
    );
    core.end_batch_native().unwrap();
    let doc = core.document().clone();
    core.set_document(doc);
    assert!(!core.canvas_metrics_active());
    assert_eq!(
        core.select_canvas_metrics(true),
        Err(MetricError::ContextMismatch)
    );
    assert_eq!(positions(&run(&core)), positions(&baseline));
}

#[test]
fn cell_format_batch_keeps_metric_binding_when_style_ids_change() {
    let mut core = core("hwp");
    let baseline = run(&core);
    core.begin_canvas_metric_session(context()).unwrap();
    core.register_canvas_metrics(context(), entries(&baseline))
        .unwrap();
    core.select_canvas_metrics(true).unwrap();
    let before = run(&core);
    core.begin_batch_native().unwrap();
    core.apply_char_format_in_cell_native(0, 12, 1, 5, 0, 0, 3, r##"{"textColor":"#ff0000"}"##)
        .unwrap();
    core.end_batch_native().unwrap();
    let after = run(&core);
    assert_ne!(after.char_shape_id, before.char_shape_id);
    assert!(after.style.supplemental_metrics.is_some());
    assert_eq!(positions(&after), positions(&before));
    core.select_canvas_metrics(false).unwrap();
    assert_ne!(positions(&run(&core)), positions(&after));
}

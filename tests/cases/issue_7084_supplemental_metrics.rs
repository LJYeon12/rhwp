//! #7084 A: controlled provider contracts, not a Hancom visual oracle.
//! Advances below are independent provider inputs; no universal emoji width.
use std::sync::Arc;

use rhwp::renderer::layout::{EmbeddedTextMeasurer, TextMeasurer};
use rhwp::renderer::supplemental_metrics::{
    MetricBackend, MetricContext, MetricError, MetricEvidence, SupplementalMetric,
    SupplementalMetricStore, MAX_SUPPLEMENTAL_ENTRIES, MAX_SUPPLEMENTAL_KEY_BYTES,
};
use rhwp::renderer::TextStyle;

fn context() -> MetricContext {
    MetricContext {
        document_generation: 1,
        font_generation: 2,
        backend: MetricBackend::Canvas2d,
    }
}

fn style() -> TextStyle {
    TextStyle {
        font_family: "HCR Batang".into(),
        font_size: 20.0,
        ..Default::default()
    }
}

fn measured(style: &TextStyle, ch: &str, width: f64) -> SupplementalMetric {
    SupplementalMetric::backend_measured(style, ch, "20px test-provider".into(), width).unwrap()
}

fn bound(style: &mut TextStyle, entries: Vec<SupplementalMetric>) -> SupplementalMetricStore {
    let mut store = SupplementalMetricStore::new(context());
    store.replace(context(), entries).unwrap();
    store.bind_style(context(), style).unwrap();
    store
}

fn positions(text: &str, style: &TextStyle) -> Vec<f64> {
    EmbeddedTextMeasurer.compute_char_positions(text, style)
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-8, "actual={a}, expected={b}");
}

#[test]
fn missing_advance_is_shared_by_total_and_following_positions() {
    let original = style();
    near(positions("😀", &original)[1], 10.0); // observed generic 0.5em path
    let mut prepared = original.clone();
    let _store = bound(&mut prepared, vec![measured(&original, "😀", 27.5)]);
    let old = positions("😀A한", &original);
    let new = positions("😀A한", &prepared);
    near(new[1], 27.5);
    for i in 1..new.len() {
        near(new[i] - old[i], 17.5);
    }
    // The public estimate intentionally retains its pre-existing integer rounding.
    let total = EmbeddedTextMeasurer.estimate_text_width("😀A한", &prepared);
    near(total, new.last().unwrap().round());
    let snapshot = prepared.supplemental_metrics.as_ref().unwrap();
    assert_eq!(
        snapshot
            .lookup(context(), &prepared, '😀')
            .unwrap()
            .width_source(),
        "supplementalBackendMeasured"
    );
}

#[test]
fn unknown_non_emoji_glyph_uses_provider_width_not_square_rule() {
    let mut prepared = style();
    let entry = measured(&prepared, "𝄞", 11.25);
    let _store = bound(&mut prepared, vec![entry]);
    near(positions("𝄞", &prepared)[1], 11.25);
}

#[test]
fn existing_metrics_and_explicit_hwp_rules_are_not_overridden() {
    let original = style();
    // ASCII DB hit, Hangul, HWP figure space, ordinary space, halfwidth form,
    // narrow punctuation, synthetic PUA number and inline object placeholder.
    let text = "A가\u{2007} ｢(\u{F02B0}\u{FFFC}";
    let entries = text
        .chars()
        .map(|ch| measured(&original, &ch.to_string(), 111.0))
        .collect();
    let mut prepared = original.clone();
    let _store = bound(&mut prepared, entries);
    assert_eq!(positions(text, &original), positions(text, &prepared));
    assert_eq!(positions("A\t가", &original), positions("A\t가", &prepared));
}

#[test]
fn ratio_and_spacing_are_applied_once_after_natural_advance() {
    for (spacing, expected) in [(0.0, 24.0), (2.0, 26.4), (-2.0, 21.6), (-40.0, 12.0)] {
        let mut prepared = TextStyle {
            ratio: 0.8,
            letter_spacing: spacing,
            ..style()
        };
        let entry = measured(&prepared, "😀", 30.0);
        let _store = bound(&mut prepared, vec![entry]);
        near(positions("😀", &prepared)[1], expected);
    }
}

#[test]
fn dash_leader_keeps_explicit_advance_with_missing_family() {
    let original = TextStyle {
        font_family: "unknown-test-face".into(),
        ..style()
    };
    let mut prepared = original.clone();
    let _store = bound(&mut prepared, vec![measured(&original, "-", 1.0)]);
    assert_eq!(positions("---", &prepared), positions("---", &original));
}

#[test]
fn replacing_valid_batch_invalidates_old_styles_until_rebound() {
    let mut prepared = style();
    let original = prepared.clone();
    let entry = measured(&prepared, "😀", 27.5);
    let mut store = bound(&mut prepared, vec![entry]);
    store
        .replace(context(), vec![measured(&original, "😀", 31.0)])
        .unwrap();
    assert_eq!(positions("😀", &prepared), positions("😀", &original));
    store.bind_style(context(), &mut prepared).unwrap();
    near(positions("😀", &prepared)[1], 31.0);
}

#[test]
fn effective_size_and_face_style_are_part_of_lookup_identity() {
    let original = style();
    let mut prepared = original.clone();
    let _store = bound(&mut prepared, vec![measured(&original, "😀", 27.5)]);
    for variant in [
        TextStyle {
            bold: true,
            ..prepared.clone()
        },
        TextStyle {
            italic: true,
            ..prepared.clone()
        },
        TextStyle {
            font_size: 21.0,
            ..prepared.clone()
        },
        TextStyle {
            font_family: "other face".into(),
            ..prepared.clone()
        },
        TextStyle {
            superscript: true,
            ..prepared.clone()
        },
    ] {
        let mut legacy = variant.clone();
        legacy.supplemental_metrics = None;
        assert_eq!(positions("😀", &variant), positions("😀", &legacy));
    }
    for (superscript, subscript) in [(true, false), (false, true)] {
        let mut scripted = TextStyle {
            superscript,
            subscript,
            ..original.clone()
        };
        let entry = measured(&scripted, "😀", 19.25); // already measured at 14px
        let _store = bound(&mut scripted, vec![entry]);
        near(positions("😀", &scripted)[1], 19.25); // no second 0.7 multiplication
    }
}

#[test]
fn multi_scalar_graphemes_are_never_partially_overridden() {
    let original = style();
    let mut prepared = original.clone();
    let _store = bound(&mut prepared, vec![measured(&original, "😀", 27.5)]);
    for text in ["😀\u{FE0F}", "😀\u{200D}😀", "😀\u{301}"] {
        assert_eq!(
            positions(text, &original),
            positions(text, &prepared),
            "{text}"
        );
    }
    let mixed = positions("😀\u{FE0F}😀", &prepared);
    near(mixed[3] - mixed[2], 27.5);
}

#[test]
fn malformed_measurements_are_rejected_and_zero_is_valid() {
    for width in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert_eq!(
            SupplementalMetric::backend_measured(&style(), "😀", "20px x".into(), width),
            Err(MetricError::InvalidAdvance)
        );
    }
    for cluster in ["", "ab", "😀\u{FE0F}", "😀\u{200D}😀", "\n"] {
        assert_eq!(
            SupplementalMetric::backend_measured(&style(), cluster, "20px x".into(), 20.0),
            Err(MetricError::UnsupportedCluster)
        );
    }
    for descriptor in ["", "20px\nx"] {
        assert_eq!(
            SupplementalMetric::backend_measured(&style(), "😀", descriptor.into(), 20.0),
            Err(MetricError::InvalidDescriptor)
        );
    }
    for size in [f64::NAN, f64::INFINITY, 0.0, -1.0] {
        let invalid = TextStyle {
            font_size: size,
            ..style()
        };
        assert_eq!(
            SupplementalMetric::backend_measured(&invalid, "😀", "20px x".into(), 20.0),
            Err(MetricError::InvalidStyle)
        );
    }
    let mut prepared = style();
    let entry = measured(&prepared, "😀", 0.0);
    let _store = bound(&mut prepared, vec![entry]);
    near(positions("😀", &prepared)[1], 0.0);
}

#[test]
fn rejected_or_identical_batches_leave_current_snapshot_intact() {
    let mut prepared = style();
    let entry = measured(&prepared, "😀", 27.5);
    let mut store = bound(&mut prepared, vec![entry.clone()]);
    let old = store.snapshot().unwrap();
    store.replace(context(), vec![entry.clone()]).unwrap();
    assert!(Arc::ptr_eq(&old, &store.snapshot().unwrap()));
    let wrong = MetricContext {
        document_generation: 9,
        ..context()
    };
    assert_eq!(
        store.replace(wrong, vec![entry.clone()]),
        Err(MetricError::ContextMismatch)
    );
    assert_eq!(
        store.replace(context(), vec![entry.clone(), entry.clone()]),
        Err(MetricError::DuplicateKey)
    );
    assert_eq!(
        store.replace(context(), vec![entry; MAX_SUPPLEMENTAL_ENTRIES + 1]),
        Err(MetricError::LimitExceeded)
    );
    let oversized = "x".repeat(MAX_SUPPLEMENTAL_KEY_BYTES);
    assert_eq!(
        SupplementalMetric::backend_measured(&prepared, "😀", oversized, 20.0),
        Err(MetricError::LimitExceeded)
    );
    assert!(Arc::ptr_eq(&old, &store.snapshot().unwrap()));
    near(positions("😀", &prepared)[1], 27.5);
}

#[test]
fn total_key_budget_is_atomic_not_just_per_entry() {
    let mut prepared = style();
    let entry = measured(&prepared, "😀", 27.5);
    let mut store = bound(&mut prepared, vec![entry]);
    let descriptor = "x".repeat(MAX_SUPPLEMENTAL_KEY_BYTES / 2);
    let entries = ['😀', '𝄞']
        .iter()
        .map(|ch| {
            SupplementalMetric::backend_measured(
                &prepared,
                &ch.to_string(),
                descriptor.clone(),
                20.0,
            )
            .unwrap()
        })
        .collect();
    assert_eq!(
        store.replace(context(), entries),
        Err(MetricError::LimitExceeded)
    );
    near(positions("😀", &prepared)[1], 27.5);
}

#[test]
fn session_changes_invalidate_retained_styles_and_reject_late_responses() {
    for next in [
        MetricContext {
            document_generation: 2,
            ..context()
        },
        MetricContext {
            font_generation: 3,
            ..context()
        },
        MetricContext {
            backend: MetricBackend::NativeSkia,
            ..context()
        },
    ] {
        let original = style();
        let mut prepared = original.clone();
        let entry = measured(&prepared, "😀", 27.5);
        let mut store = bound(&mut prepared, vec![entry.clone()]);
        let snapshot = store.snapshot().unwrap();
        assert!(snapshot.lookup(next, &prepared, '😀').is_none());
        store.reset_context(next);
        assert!(!snapshot.is_active());
        assert_eq!(positions("😀", &prepared), positions("😀", &original));
        assert_eq!(
            store.replace(context(), vec![entry]),
            Err(MetricError::ContextMismatch)
        );
    }
}

#[test]
fn canvas_measurements_cannot_be_registered_for_other_backends() {
    for backend in [
        MetricBackend::CanvasKit,
        MetricBackend::NativeSkia,
        MetricBackend::Svg,
    ] {
        let target = MetricContext {
            backend,
            ..context()
        };
        let mut store = SupplementalMetricStore::new(target);
        assert_eq!(
            store.replace(target, vec![measured(&style(), "😀", 27.5)]),
            Err(MetricError::ContextMismatch)
        );
        assert!(store.snapshot().is_none());
    }
}

#[test]
fn closing_session_invalidates_styles_without_serializing_resources() {
    let original = style();
    let mut prepared = original.clone();
    let store = bound(&mut prepared, vec![measured(&original, "😀", 27.5)]);
    assert_eq!(
        serde_json::to_value(&prepared).unwrap(),
        serde_json::to_value(&original).unwrap()
    );
    drop(store);
    assert_eq!(positions("😀", &prepared), positions("😀", &original));
}

#[test]
fn verified_source_uses_real_hmtx_and_preserves_source_identity() {
    let bytes: Arc<[u8]> = std::fs::read("ttfs/opensource/NotoSansKR-Regular.ttf")
        .unwrap()
        .into();
    let face = ttf_parser::Face::parse(&bytes, 0).unwrap();
    let glyph = face.glyph_index('A').unwrap();
    let expected =
        f64::from(face.glyph_hor_advance(glyph).unwrap()) * 20.0 / f64::from(face.units_per_em());
    let entry = SupplementalMetric::from_font(&style(), "A", bytes.clone(), 0).unwrap();
    near(entry.natural_advance_px(), expected);
    assert_eq!(entry.width_source(), "supplementalVerifiedSource");
    match entry.evidence() {
        MetricEvidence::VerifiedSource {
            bytes: source,
            face_index,
            glyph_id,
            ..
        } => {
            assert!(Arc::ptr_eq(source, &bytes));
            assert_eq!(*face_index, 0);
            assert_eq!(*glyph_id, glyph.0);
        }
        _ => panic!("font bytes must not be mislabeled as browser measurement"),
    }
    assert_eq!(
        SupplementalMetric::from_font(&style(), "A", Arc::from(&b"invalid"[..]), 0),
        Err(MetricError::MalformedFont)
    );
    assert_eq!(
        SupplementalMetric::from_font(&style(), "\u{10FFFF}", bytes, 0),
        Err(MetricError::MissingGlyph)
    );
}

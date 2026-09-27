//! WS-5 — Croft / Loft entrypoints + plugin stub API surface.

use mcload::plugins::stubs::{
    EngagementPlugin, FingerprintPlugin, SimilarityPlugin, StubEngagement,
    StubFingerprint, StubSimilarity,
};
use mcload::ui::{croft, loft};

#[test]
fn croft_dry_run_returns_ok() {
    croft::run(true).expect("croft dry-run stub should Ok(())");
}

#[test]
fn loft_dry_run_returns_ok() {
    loft::run(true).expect("loft dry-run stub should Ok(())");
}

#[test]
fn engagement_plugin_stub_engage_ok() {
    let p = StubEngagement;
    assert_eq!(p.name(), "stub-engagement");
    p.engage().expect("engagement stub should Ok(())");
}

#[test]
fn fingerprint_plugin_stub_returns_placeholder() {
    let p = StubFingerprint;
    assert_eq!(p.name(), "stub-fingerprint");
    let fp = p.fingerprint("/tmp/x").expect("fingerprint stub");
    assert!(
        !fp.is_empty(),
        "fingerprint stub should return a non-empty placeholder"
    );
}

#[test]
fn similarity_plugin_stub_returns_score() {
    let p = StubSimilarity;
    assert_eq!(p.name(), "stub-similarity");
    let score = p.similarity("a", "b").expect("similarity stub");
    assert!(
        (0.0..=1.0).contains(&score),
        "similarity stub score in [0,1], got {score}"
    );
}

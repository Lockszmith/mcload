//! Empty plugin hooks for engagement, fingerprint, and similarity (WS-5).

use crate::error::{Error, Result};

pub trait EngagementPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn engage(&self) -> Result<()>;
}

pub trait FingerprintPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn fingerprint(&self, path: &str) -> Result<String>;
}

pub trait SimilarityPlugin: Send + Sync {
    fn name(&self) -> &str;
    fn similarity(&self, a: &str, b: &str) -> Result<f64>;
}

/// Default no-op stubs — methods return NotImplemented so API-surface tests fail
/// until WS-5 wires real stub Ok(()) / placeholder results.
#[derive(Debug, Default)]
pub struct StubEngagement;

impl EngagementPlugin for StubEngagement {
    fn name(&self) -> &str {
        "stub-engagement"
    }

    fn engage(&self) -> Result<()> {
        Err(Error::NotImplemented("StubEngagement::engage — WS-5"))
    }
}

#[derive(Debug, Default)]
pub struct StubFingerprint;

impl FingerprintPlugin for StubFingerprint {
    fn name(&self) -> &str {
        "stub-fingerprint"
    }

    fn fingerprint(&self, _path: &str) -> Result<String> {
        Err(Error::NotImplemented(
            "StubFingerprint::fingerprint — WS-5",
        ))
    }
}

#[derive(Debug, Default)]
pub struct StubSimilarity;

impl SimilarityPlugin for StubSimilarity {
    fn name(&self) -> &str {
        "stub-similarity"
    }

    fn similarity(&self, _a: &str, _b: &str) -> Result<f64> {
        Err(Error::NotImplemented(
            "StubSimilarity::similarity — WS-5",
        ))
    }
}

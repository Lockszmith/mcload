//! Empty plugin hooks for engagement, fingerprint, and similarity (WS-5).

use crate::error::Result;

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

/// Default no-op stubs returning Ok / placeholder results for WS-5 scaffolding.
#[derive(Debug, Default)]
pub struct StubEngagement;

impl EngagementPlugin for StubEngagement {
    fn name(&self) -> &str {
        "stub-engagement"
    }

    fn engage(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct StubFingerprint;

impl FingerprintPlugin for StubFingerprint {
    fn name(&self) -> &str {
        "stub-fingerprint"
    }

    fn fingerprint(&self, _path: &str) -> Result<String> {
        Ok("stub-fingerprint-placeholder".to_string())
    }
}

#[derive(Debug, Default)]
pub struct StubSimilarity;

impl SimilarityPlugin for StubSimilarity {
    fn name(&self) -> &str {
        "stub-similarity"
    }

    fn similarity(&self, _a: &str, _b: &str) -> Result<f64> {
        Ok(0.0)
    }
}

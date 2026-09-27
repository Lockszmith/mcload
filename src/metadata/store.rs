//! `MetadataStore` trait + concurrent in-memory stub (WS-4).

use std::sync::Arc;

use crate::error::{Error, Result};
use crate::metadata::types::SnapshotRecord;

/// Concurrent metadata store contract (`Send + Sync`).
pub trait MetadataStore: Send + Sync {
    fn get(&self, id: &str) -> Result<Option<SnapshotRecord>>;
    fn put(&self, record: SnapshotRecord) -> Result<()>;
    fn list(&self) -> Result<Vec<SnapshotRecord>>;
}

/// In-memory store intended to use `Arc<RwLock<_>>` (or tokio RwLock).
///
/// Stub methods always return [`Error::NotImplemented`] so concurrent contract
/// tests fail until WS-4 implements real locking + last-write-wins semantics.
#[derive(Debug, Default, Clone)]
pub struct ConcurrentMemoryStore {
    _inner: Arc<()>,
}

impl ConcurrentMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MetadataStore for ConcurrentMemoryStore {
    fn get(&self, _id: &str) -> Result<Option<SnapshotRecord>> {
        Err(Error::NotImplemented(
            "ConcurrentMemoryStore::get — WS-4",
        ))
    }

    fn put(&self, _record: SnapshotRecord) -> Result<()> {
        Err(Error::NotImplemented(
            "ConcurrentMemoryStore::put — WS-4",
        ))
    }

    fn list(&self) -> Result<Vec<SnapshotRecord>> {
        Err(Error::NotImplemented(
            "ConcurrentMemoryStore::list — WS-4",
        ))
    }
}

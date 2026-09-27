//! `MetadataStore` trait + concurrent in-memory store (WS-4).

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::error::Result;
use crate::metadata::types::SnapshotRecord;

/// Concurrent metadata store contract (`Send + Sync`).
pub trait MetadataStore: Send + Sync {
    fn get(&self, id: &str) -> Result<Option<SnapshotRecord>>;
    fn put(&self, record: SnapshotRecord) -> Result<()>;
    fn list(&self) -> Result<Vec<SnapshotRecord>>;
}

/// In-memory store backed by `Arc<RwLock<HashMap>>`.
///
/// Concurrent `put`/`get`/`list` are lock-safe. Same-key writes use
/// last-write-wins (HashMap overwrite).
#[derive(Debug, Default, Clone)]
pub struct ConcurrentMemoryStore {
    inner: Arc<RwLock<HashMap<String, SnapshotRecord>>>,
}

impl ConcurrentMemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MetadataStore for ConcurrentMemoryStore {
    fn get(&self, id: &str) -> Result<Option<SnapshotRecord>> {
        let map = self.inner.read().expect("metadata store lock poisoned");
        Ok(map.get(id).cloned())
    }

    fn put(&self, record: SnapshotRecord) -> Result<()> {
        let mut map = self.inner.write().expect("metadata store lock poisoned");
        map.insert(record.id.clone(), record);
        Ok(())
    }

    fn list(&self) -> Result<Vec<SnapshotRecord>> {
        let map = self.inner.read().expect("metadata store lock poisoned");
        Ok(map.values().cloned().collect())
    }
}

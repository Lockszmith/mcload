//! Snapshot record stubs for metadata store.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotRecord {
    pub id: String,
    pub path: String,
    pub payload: String,
}

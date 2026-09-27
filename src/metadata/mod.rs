//! Concurrent metadata persistence API (WS-4).

pub mod store;
pub mod types;

pub use store::{ConcurrentMemoryStore, MetadataStore};
pub use types::SnapshotRecord;

//! Gathering queue: marks intake items as Fresh (WS-2b).

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    Fresh,
    Ready,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatheringItem {
    pub id: String,
    pub marker: Marker,
}

#[derive(Debug, Default)]
pub struct GatheringQueue {
    items: Vec<GatheringItem>,
}

impl GatheringQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(&self) -> &[GatheringItem] {
        &self.items
    }

    /// Intake metadata and mark it Fresh. Stub stays RED.
    pub fn mark_fresh(&mut self, id: impl Into<String>) -> Result<()> {
        let _ = id;
        Err(Error::NotImplemented(
            "GatheringQueue::mark_fresh — WS-2b",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WS-2b: Gathering marks items Fresh.
    #[test]
    fn gathering_marks_items_fresh() {
        let mut q = GatheringQueue::new();
        q.mark_fresh("snap-1").expect("mark_fresh should succeed");
        let items = q.items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "snap-1");
        assert_eq!(items[0].marker, Marker::Fresh);
    }
}

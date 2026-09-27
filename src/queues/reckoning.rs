//! Reckoning queue: promote Fresh → Ready (WS-2b).

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    Fresh,
    Ready,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReckoningItem {
    pub id: String,
    pub marker: Marker,
}

#[derive(Debug, Default)]
pub struct ReckoningQueue {
    items: Vec<ReckoningItem>,
}

impl ReckoningQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(&self) -> &[ReckoningItem] {
        &self.items
    }

    /// Seed a Fresh item for promotion tests.
    pub fn seed_fresh(&mut self, id: impl Into<String>) {
        self.items.push(ReckoningItem {
            id: id.into(),
            marker: Marker::Fresh,
        });
    }

    /// Promote Fresh → Ready.
    pub fn promote_to_ready(&mut self, id: &str) -> Result<()> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .ok_or_else(|| Error::Queue(format!("ReckoningQueue: unknown id {id}")))?;
        match item.marker {
            Marker::Fresh => {
                item.marker = Marker::Ready;
                Ok(())
            }
            Marker::Ready => Err(Error::Queue(format!(
                "ReckoningQueue: {id} already Ready"
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WS-2b: Reckoning promotes Fresh → Ready.
    #[test]
    fn reckoning_promotes_fresh_to_ready() {
        let mut q = ReckoningQueue::new();
        q.seed_fresh("snap-1");
        q.promote_to_ready("snap-1")
            .expect("promote_to_ready should succeed");
        assert_eq!(q.items().len(), 1);
        assert_eq!(q.items()[0].marker, Marker::Ready);
    }
}

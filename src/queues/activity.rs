//! Activity queue: Pause / Resume / Abort (WS-2b).

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityState {
    Running,
    Paused,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct ActivityQueue {
    state: ActivityState,
}

impl ActivityQueue {
    pub fn new() -> Self {
        Self {
            state: ActivityState::Running,
        }
    }

    pub fn state(&self) -> ActivityState {
        self.state
    }

    /// Transition to Paused. Stub: no-op / error so tests stay RED.
    pub fn pause(&mut self) -> Result<()> {
        Err(Error::NotImplemented("ActivityQueue::pause — WS-2b"))
    }

    /// Transition to Running from Paused. Stub stays RED.
    pub fn resume(&mut self) -> Result<()> {
        Err(Error::NotImplemented("ActivityQueue::resume — WS-2b"))
    }

    /// Transition to Aborted. Stub stays RED.
    pub fn abort(&mut self) -> Result<()> {
        Err(Error::NotImplemented("ActivityQueue::abort — WS-2b"))
    }
}

impl Default for ActivityQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WS-2b: Activity Pause / Resume / Abort transitions.
    #[test]
    fn activity_pause_resume_abort_transitions() {
        let mut q = ActivityQueue::new();
        assert_eq!(q.state(), ActivityState::Running);

        q.pause().expect("pause should succeed");
        assert_eq!(q.state(), ActivityState::Paused);

        q.resume().expect("resume should succeed");
        assert_eq!(q.state(), ActivityState::Running);

        q.abort().expect("abort should succeed");
        assert_eq!(q.state(), ActivityState::Aborted);
    }
}

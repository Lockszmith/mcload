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

    /// Transition Running → Paused.
    pub fn pause(&mut self) -> Result<()> {
        match self.state {
            ActivityState::Running => {
                self.state = ActivityState::Paused;
                Ok(())
            }
            other => Err(Error::Queue(format!(
                "ActivityQueue::pause from {other:?}"
            ))),
        }
    }

    /// Transition Paused → Running.
    pub fn resume(&mut self) -> Result<()> {
        match self.state {
            ActivityState::Paused => {
                self.state = ActivityState::Running;
                Ok(())
            }
            other => Err(Error::Queue(format!(
                "ActivityQueue::resume from {other:?}"
            ))),
        }
    }

    /// Transition to Aborted (from Running or Paused).
    pub fn abort(&mut self) -> Result<()> {
        match self.state {
            ActivityState::Running | ActivityState::Paused => {
                self.state = ActivityState::Aborted;
                Ok(())
            }
            ActivityState::Aborted => Err(Error::Queue(
                "ActivityQueue::abort already aborted".into(),
            )),
        }
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

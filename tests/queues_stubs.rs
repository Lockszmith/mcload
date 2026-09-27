//! WS-2b — Queue stubs integration surface (unit tests also live under `src/queues/`).
//!
//! Parallel-safe with WS-2 / WS-4 (no shared `cli.rs` / metadata ownership).

use mcload::queues::{
    ActivityQueue, ActivityState, GatheringQueue, ReckoningQueue,
};
use mcload::queues::gathering::Marker as GatheringMarker;
use mcload::queues::reckoning::Marker as ReckoningMarker;

#[test]
fn activity_pause_resume_abort() {
    let mut q = ActivityQueue::new();
    assert_eq!(q.state(), ActivityState::Running);
    q.pause().expect("pause");
    assert_eq!(q.state(), ActivityState::Paused);
    q.resume().expect("resume");
    assert_eq!(q.state(), ActivityState::Running);
    q.abort().expect("abort");
    assert_eq!(q.state(), ActivityState::Aborted);
}

#[test]
fn gathering_marks_fresh() {
    let mut q = GatheringQueue::new();
    q.mark_fresh("item-a").expect("mark_fresh");
    assert_eq!(q.items().len(), 1);
    assert_eq!(q.items()[0].marker, GatheringMarker::Fresh);
}

#[test]
fn reckoning_fresh_to_ready() {
    let mut q = ReckoningQueue::new();
    q.seed_fresh("item-a");
    q.promote_to_ready("item-a").expect("promote");
    assert_eq!(q.items()[0].marker, ReckoningMarker::Ready);
}

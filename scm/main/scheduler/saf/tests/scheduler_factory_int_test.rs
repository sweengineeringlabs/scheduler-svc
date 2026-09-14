//! Integration tests for [`SchedulerFactory`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use scheduler_pattern::{Job, Trigger};
use scheduler_svc_saf::SchedulerFactory;

/// @covers: SchedulerFactory::in_memory — returns a real, working
/// scheduler, not just a value of the right type.
#[test]
fn test_in_memory_returns_a_working_scheduler() {
    let (tx, rx) = mpsc::channel();
    let job: Job = Arc::new(move || {
        let tx = tx.clone();
        Box::pin(async move {
            tx.send(()).expect("receiver dropped");
            Ok(())
        })
    });

    let scheduler = SchedulerFactory::in_memory();
    scheduler
        .schedule(Trigger::Once(Duration::from_millis(5)), job)
        .expect("schedule must succeed");

    assert!(
        rx.recv_timeout(Duration::from_millis(500)).is_ok(),
        "scheduled job must actually fire"
    );
}

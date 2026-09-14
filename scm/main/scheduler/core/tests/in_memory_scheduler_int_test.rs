//! Integration tests for [`scheduler_svc_core::InMemoryScheduler`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::time::Duration;

use scheduler_pattern::{Job, Scheduler, SchedulerError, Trigger};
use scheduler_svc_core::InMemoryScheduler;

/// @covers: schedule/Once — the job actually fires after its delay, not
/// immediately and not never.
#[test]
fn test_once_job_fires_after_delay() {
    let (tx, rx) = mpsc::channel();
    let job: Job = Arc::new(move || {
        let tx = tx.clone();
        Box::pin(async move {
            tx.send(()).expect("receiver dropped");
            Ok(())
        })
    });

    let scheduler = InMemoryScheduler::new();
    scheduler
        .schedule(Trigger::Once(Duration::from_millis(20)), job)
        .expect("schedule must succeed");

    // Fired signal must not arrive before the delay elapses.
    assert!(rx.recv_timeout(Duration::from_millis(5)).is_err());
    // ...but must arrive well within a generous margin after it.
    assert!(rx.recv_timeout(Duration::from_millis(500)).is_ok());
}

/// @covers: schedule/Every — the job fires more than once, proving this
/// isn't just a Once job in disguise.
#[test]
fn test_every_job_fires_repeatedly() {
    let count = Arc::new(AtomicUsize::new(0));
    let job_count = Arc::clone(&count);
    let job: Job = Arc::new(move || {
        job_count.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    });

    let scheduler = InMemoryScheduler::new();
    let id = scheduler
        .schedule(Trigger::Every(Duration::from_millis(10)), job)
        .expect("schedule must succeed");

    std::thread::sleep(Duration::from_millis(80));
    scheduler.cancel(&id).expect("cancel must succeed");
    let fired_before_cancel = count.load(Ordering::SeqCst);
    assert!(
        fired_before_cancel >= 2,
        "expected at least 2 firings in 80ms at a 10ms interval, got {fired_before_cancel}"
    );
}

/// @covers: cancel — cancelling before the delay elapses actually prevents
/// the job from running, not just marking it cancelled cosmetically.
#[test]
fn test_cancel_before_firing_prevents_the_job_from_running() {
    let ran = Arc::new(AtomicUsize::new(0));
    let job_ran = Arc::clone(&ran);
    let job: Job = Arc::new(move || {
        job_ran.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    });

    let scheduler = InMemoryScheduler::new();
    let id = scheduler
        .schedule(Trigger::Once(Duration::from_millis(50)), job)
        .expect("schedule must succeed");
    scheduler.cancel(&id).expect("cancel must succeed");

    std::thread::sleep(Duration::from_millis(150));
    assert_eq!(
        ran.load(Ordering::SeqCst),
        0,
        "cancelled job must never run"
    );
}

/// @covers: cancel — a completed Once job can no longer be cancelled;
/// JobNotFound is real, not a stale success.
#[test]
fn test_cancel_after_once_job_completed_returns_job_not_found() {
    let (tx, rx) = mpsc::channel();
    let job: Job = Arc::new(move || {
        let tx = tx.clone();
        Box::pin(async move {
            tx.send(()).expect("receiver dropped");
            Ok(())
        })
    });

    let scheduler = InMemoryScheduler::new();
    let id = scheduler
        .schedule(Trigger::Once(Duration::from_millis(5)), job)
        .expect("schedule must succeed");
    rx.recv_timeout(Duration::from_millis(500))
        .expect("job must have fired");
    // Give the job thread a moment to clean up its own map entry after
    // firing, before asserting cancel() sees it as gone.
    std::thread::sleep(Duration::from_millis(20));

    assert!(matches!(
        scheduler.cancel(&id),
        Err(SchedulerError::JobNotFound(found)) if found == id
    ));
}

/// @covers: schedule — an invalid trigger (zero-duration interval) is
/// rejected before any thread is spawned.
#[test]
fn test_schedule_rejects_zero_duration_interval() {
    let job: Job = Arc::new(|| Box::pin(async { Ok(()) }));
    let scheduler = InMemoryScheduler::new();
    let result = scheduler.schedule(Trigger::Every(Duration::ZERO), job);
    assert!(matches!(result, Err(SchedulerError::InvalidTrigger(_))));
}

/// @covers: cancel — an unknown JobId returns JobNotFound.
#[test]
fn test_cancel_unknown_job_id_returns_job_not_found() {
    let scheduler = InMemoryScheduler::new();
    let unknown = scheduler_pattern::JobId::new();
    assert!(matches!(
        scheduler.cancel(&unknown),
        Err(SchedulerError::JobNotFound(id)) if id == unknown
    ));
}

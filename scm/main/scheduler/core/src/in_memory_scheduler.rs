//! [`InMemoryScheduler`] — fires jobs on dedicated OS threads, no external
//! scheduling technology.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use scheduler_pattern::{Job, JobId, Scheduler, SchedulerError, Trigger};

type CancelFlags = Arc<Mutex<HashMap<JobId, Arc<AtomicBool>>>>;

/// In-process [`Scheduler`]: each scheduled job gets its own dedicated OS
/// thread that sleeps for the trigger's delay/interval, then runs the job
/// via `futures::executor::block_on`.
///
/// Jobs and their schedules are lost if the process exits — no
/// persistence, no distributed coordination. That's the reference-impl
/// trade-off; a persistent or distributed backend is a `spi` crate this
/// repo doesn't have yet (no real consumer has needed one).
#[derive(Default, Clone)]
pub struct InMemoryScheduler {
    cancel_flags: CancelFlags,
}

impl InMemoryScheduler {
    /// Construct a fresh scheduler with no jobs scheduled.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scheduler for InMemoryScheduler {
    fn schedule(&self, trigger: Trigger, job: Job) -> Result<JobId, SchedulerError> {
        if let Trigger::Every(interval) = trigger {
            if interval.is_zero() {
                return Err(SchedulerError::InvalidTrigger(
                    "interval must be non-zero".to_string(),
                ));
            }
        }

        let job_id = JobId::new();
        let cancelled = Arc::new(AtomicBool::new(false));
        self.cancel_flags
            .lock()
            .map_err(|_| SchedulerError::ScheduleFailed("lock poisoned".to_string()))?
            .insert(job_id, Arc::clone(&cancelled));

        let flags = Arc::clone(&self.cancel_flags);
        std::thread::Builder::new()
            .name(format!("scheduler-job-{job_id}"))
            .spawn(move || run_job(trigger, job, &cancelled, &flags, job_id))
            .map_err(|e| SchedulerError::ScheduleFailed(format!("thread spawn failed: {e}")))?;

        Ok(job_id)
    }

    fn cancel(&self, job_id: &JobId) -> Result<(), SchedulerError> {
        let mut flags = self
            .cancel_flags
            .lock()
            .map_err(|_| SchedulerError::CancelFailed("lock poisoned".to_string()))?;
        match flags.remove(job_id) {
            Some(flag) => {
                flag.store(true, Ordering::SeqCst);
                Ok(())
            }
            None => Err(SchedulerError::JobNotFound(*job_id)),
        }
    }
}

/// Runs on the job's dedicated thread. `Once` sleeps then fires once (if not
/// cancelled first); `Every` loops, sleeping and firing until cancelled.
/// Removes its own entry from `flags` once no more firings will happen, so
/// a later `cancel()` on a completed `Once` job correctly returns
/// `JobNotFound` rather than a misleading `Ok(())`.
fn run_job(trigger: Trigger, job: Job, cancelled: &AtomicBool, flags: &CancelFlags, job_id: JobId) {
    match trigger {
        Trigger::Once(delay) => {
            std::thread::sleep(delay);
            if !cancelled.load(Ordering::SeqCst) {
                let _ = futures::executor::block_on(job());
            }
        }
        Trigger::Every(interval) => loop {
            std::thread::sleep(interval);
            if cancelled.load(Ordering::SeqCst) {
                break;
            }
            let _ = futures::executor::block_on(job());
        },
    }
    if let Ok(mut flags) = flags.lock() {
        flags.remove(&job_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_scheduler_is_send_and_sync() {
        fn _assert_send_sync<T: Send + Sync>() {}
        _assert_send_sync::<InMemoryScheduler>();
        assert!(
            std::hint::black_box(true),
            "InMemoryScheduler is Send + Sync (checked above at compile time)"
        );
    }
}

//! [`SchedulerFactory`] — public scheduler construction surface.

use scheduler_pattern::Scheduler;

/// Zero-size factory type for constructing scheduler instances.
pub struct SchedulerFactory;

impl SchedulerFactory {
    /// Construct the in-process reference scheduler.
    ///
    /// Jobs and schedules are lost if the process exits — no persistence,
    /// no distributed coordination. See
    /// [`scheduler_svc_core::InMemoryScheduler`]'s own doc comment.
    pub fn in_memory() -> Box<dyn Scheduler> {
        Box::new(scheduler_svc_core::InMemoryScheduler::new())
    }
}

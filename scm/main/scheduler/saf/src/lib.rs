//! `scheduler_svc_saf` — scheduler construction facade.
//!
//! The `Scheduler` contract (trait + value types + error) lives in
//! `scheduler-pattern`; this crate owns the construction factory that
//! selects among real implementations — currently just
//! `scheduler-svc-core`'s `InMemoryScheduler`.
//!
//! Unlike `executor-svc-saf`'s `ExecutorFactory` (which must return `impl
//! Executor` — `Executor::run` is generic, not object-safe), `Scheduler`
//! has no generic methods, so `SchedulerFactory` returns `Box<dyn
//! Scheduler>` uniformly, matching `message-broker-svc-saf`'s
//! `MessageBrokerFactory` shape.

mod scheduler_factory;

pub use scheduler_factory::SchedulerFactory;

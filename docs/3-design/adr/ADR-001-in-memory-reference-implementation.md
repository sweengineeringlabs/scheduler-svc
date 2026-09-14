# ADR-001: `InMemoryScheduler`'s implementation shape

**Status**: Accepted
**Date**: 2026-09-14

## Context

`scheduler-pattern` has no existing pilot (see that repo's own ADR-001), so
this crate's implementation choices are new design, not extraction. Two
real alternatives were considered for how `InMemoryScheduler` fires jobs:

1. **One dedicated OS thread per scheduled job** (chosen): `schedule()`
   spawns a thread that sleeps for the trigger's delay/interval, then runs
   the job via `futures::executor::block_on`.
2. **A single shared timer-wheel thread** managing all scheduled jobs'
   firing times in one data structure, dispatching to a thread/task pool
   when each fires.

## Decision

Chose (1), one thread per job, for this first version:

- Simpler to implement and verify correctly — no shared timing data
  structure to get right, no false-wakeup/precision-drift concerns a timer
  wheel has to handle.
- Matches this org's own `edge-runtime`'s `SchedulerJobRuntime` precedent
  (`scheduler_job_runtime.rs`): one dedicated OS thread per scheduled unit
  of work is an established, accepted shape here, not a new pattern being
  introduced.
- Honest, known limitation: this does not scale to a large number of
  concurrently-scheduled jobs (thread-per-job has real OS thread-count
  limits). If a real consumer needs thousands of concurrent schedules, a
  shared-timer-wheel `spi` backend (or a redesign of `core` itself) is the
  right fix — not attempted here without a real consumer to validate the
  redesign against.

## Why no dependency on `executor-pattern`/`executor-svc`

`InMemoryScheduler` needs to run each fired job's `BoxFuture` to
completion. `executor-svc-core`'s `BlockingExecutor` does exactly this —
using it here would have been real code reuse. Decided against it for this
first version: it would make `scheduler-svc-core` depend on two other
not-yet-published, freshly-created crates
(`executor-pattern`+`executor-svc-core`) for a single call
(`futures::executor::block_on`, which `executor_svc_core::BlockingExecutor`
itself just wraps) — a real but small amount of indirection for a genuine
but not urgent reuse win. `scheduler-svc-core` depends on `futures`
directly instead, calling `block_on` itself. Revisit this once both crate
pairs have shipped a real v0.1.0 and this reuse can be validated against
actual, not hypothetical, need — tracked as follow-on work, not forgotten.

## Consequences

- `scheduler-svc-core` depends on `scheduler-pattern` and `futures` only.
- `InMemoryScheduler::cancel` removes the job's cancellation flag from its
  internal map the moment the job's own thread determines no more firings
  will happen (a completed `Once`, or a cancelled `Every`) — so a later
  `cancel()` call on an already-completed job correctly returns
  `SchedulerError::JobNotFound`, not a misleading `Ok(())`. Verified by a
  real test (`test_cancel_after_once_job_completed_returns_job_not_found`).

# Glossary

Alphabetized list of terms used in `scheduler-svc`.

---

**InMemoryScheduler** - `scheduler-svc-core`'s technology-free reference `Scheduler`: one dedicated OS thread per scheduled job (sleep + `futures::executor::block_on`). No persistence, no distributed coordination.

**SchedulerFactory** - Construction facade in `scheduler-svc-saf`: `in_memory`, returning `Box<dyn Scheduler>` (unlike `executor-svc-saf`'s `ExecutorFactory`, `Scheduler` is object-safe).

[← Docs index](README.md)

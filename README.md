# scheduler-svc

> **TLDR:** The in-process `InMemoryScheduler`, on top of
> [`scheduler-pattern`](https://github.com/sweengineeringlabs/scheduler-pattern)'s
> contract. See [Architecture](docs/3-design/architecture.md) for the full design.

Companion implementation repo to
[`scheduler-pattern`](https://github.com/sweengineeringlabs/scheduler-pattern) —
see that repo's own ADR-001 for why this domain was designed contract-first,
with no existing pilot to extract from.

## Quick Start

```rust
use std::sync::Arc;
use std::time::Duration;

use scheduler_svc_saf::SchedulerFactory;
use scheduler_pattern::{Scheduler, Trigger};

let scheduler = SchedulerFactory::in_memory();
let job = Arc::new(|| Box::pin(async {
    // ... do the work ...
    Ok(())
}) as _);
let _job_id = scheduler.schedule(Trigger::Every(Duration::from_secs(30)), job);
```

## Crates

| Crate | What it is |
|-------|------------|
| [`scheduler-svc-core`](scm/main/scheduler/core) | The technology-free reference implementation: `InMemoryScheduler` (dedicated-thread timers, no persistence) |
| [`scheduler-svc-saf`](scm/main/scheduler/saf) | `SchedulerFactory` — construction facade consumers depend on |

No `spi` crate yet — no real consumer has needed a persistent or
distributed scheduling backend. See
[Architecture](docs/3-design/architecture.md).

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, why `SchedulerFactory` returns zero-cost `impl Scheduler`, not `Box<dyn Scheduler>` |
| [ADR-001](docs/3-design/adr/ADR-001-in-memory-reference-implementation.md) | Why `InMemoryScheduler` uses dedicated threads, not a shared timer wheel |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, working on this crate |

## License

MIT OR Apache-2.0

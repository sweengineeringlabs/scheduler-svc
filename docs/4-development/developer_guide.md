# scheduler-svc Developer Guide

**Audience**: Developers, contributors.

## Repo Structure

```
scheduler-svc/
├── README.md
├── docs/
│   ├── README.md
│   ├── glossary.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/compliance/compliance_checklist.md
│   ├── 3-design/adr/README.md, ADR-001-in-memory-reference-implementation.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # workspace: [core, saf] -- no spi/ yet
    └── main/scheduler/
        ├── core/              # scheduler-svc-core -- InMemoryScheduler (technology-free)
        └── saf/               # scheduler-svc-saf -- SchedulerFactory
```

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Not yet published to crates.io. First publish will be v0.1.0 for both crates.
- Depends on [`scheduler-pattern`](https://github.com/sweengineeringlabs/scheduler-pattern)
  by version (once published), not `git`/`path`.

## Working on Any Crate

Both crates are members of `scm/Cargo.toml`, so from `scm/`:

```
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --check
```

`SchedulerFactory::in_memory()` is always available, no feature required —
only one backend exists.

## Timing-Sensitive Tests

`InMemoryScheduler`'s own tests use real delays (millisecond-scale) and
`std::sync::mpsc::recv_timeout` rather than polling loops, to prove actual
trigger timing (a job fires after its delay, not before; a cancelled job
never fires) without being flaky under normal CI load. If a test in
`in_memory_scheduler_int_test.rs` becomes flaky, widen its margins — don't
delete the timing assertion, it's the point of the test.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-in-memory-reference-implementation.md)
- [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)

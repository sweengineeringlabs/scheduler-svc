# Architecture Compliance Checklist

**Audience**: Architects, contributors, reviewers.

Derived from [architecture.md](../architecture.md). Every rule here is enforceable —
re-run the listed command after any change and expect the stated result.

## 1. `core` is technology-free

| # | Rule | Verify |
|---|------|--------|
| 1 | `scheduler-svc-core` names no scheduling technology and has no external dependency beyond `futures` | `grep -nE "^\s*(pub )?(struct\|enum\|fn) \w*(Redis\|Postgres\|Quartz)" main/scheduler/core/src/*.rs` returns nothing; `main/scheduler/core/Cargo.toml`'s `[dependencies]` lists only `scheduler-pattern`, `futures` |

## 2. `SchedulerFactory` returns zero-cost `impl Scheduler`, not `Box<dyn Scheduler>`

| # | Rule | Verify |
|---|------|--------|
| 2 | `SchedulerFactory::in_memory` returns `impl Scheduler`, no heap allocation or vtable dispatch | `grep -n "Box<dyn Scheduler>" main/scheduler/saf/src/*.rs` returns nothing; `grep -n "impl Scheduler" main/scheduler/saf/src/*.rs` shows the constructor's return type |
| 3 | `saf`'s own `lib.rs` never re-exports a concrete backend type (`InMemoryScheduler`) | `grep -n "^pub use" main/scheduler/saf/src/lib.rs` shows only `SchedulerFactory` |

## 3. Lint gates

| # | Rule | Verify |
|---|------|--------|
| 4 | `#![deny(unsafe_code)]` enforced across every crate | `cargo build --workspace` fails on any `unsafe` block |
| 5 | `#![warn(missing_docs)]` enforced across every crate | `cargo doc --workspace --no-deps` warns on any undocumented public item |
| 6 | `cargo clippy --workspace --all-targets -- -D warnings` clean | Run before every commit |
| 7 | `cargo fmt --check` clean across every crate | Run before every commit |

[← 3-design index](../README.md)

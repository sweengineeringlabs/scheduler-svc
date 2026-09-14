# Architecture Decision Records

**Audience**: Architects, technical leads, contributors.

| ADR | Status | Date | Decision |
|-----|--------|------|----------|
| [ADR-001](ADR-001-in-memory-reference-implementation.md) | Accepted | 2026-09-14 | `InMemoryScheduler` uses one dedicated OS thread per scheduled job (sleep + `futures::executor::block_on`), not a shared timer wheel or a dependency on `executor-pattern`; no `spi` backend exists yet |

[← 3-design index](../README.md)

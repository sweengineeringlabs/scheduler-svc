//! `scheduler_svc_core` — the technology-free reference implementation of
//! `scheduler-pattern`'s `Scheduler` trait.
//!
//! [`InMemoryScheduler`] fires each scheduled job on its own dedicated OS
//! thread (`std::thread::spawn` + `std::thread::sleep` for the delay,
//! `futures::executor::block_on` to run the fired job's future) — no
//! external scheduling technology, no persistence, no distributed
//! coordination. Jobs and their schedules are lost if the process exits.

mod in_memory_scheduler;

pub use in_memory_scheduler::InMemoryScheduler;

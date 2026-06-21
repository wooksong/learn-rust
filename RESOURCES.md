# Practical Rust for Systems Work and TUI Applications Resources

## Knowledge

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
  Canonical narrative introduction. Use for refreshing ownership, traits, closures, iterators, smart pointers, concurrency, and async foundations when a project task exposes a gap.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  Compact example-driven reference. Use for quick syntax refreshers before TDD tasks.
- [The Rust Reference](https://doc.rust-lang.org/reference/)
  Authoritative language semantics. Use when exact rules matter, especially around closures, trait bounds, lifetimes, unsafe, and concurrency guarantees.
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
  Idiomatic library-design checklist. Use when shaping project modules and public interfaces.
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
  Community-maintained idiom and anti-pattern catalog. Use to compare C/C++ instincts with Rust-native designs.
- [Rust Atomics and Locks — Mara Bos](https://marabos.nl/atomics/)
  Deep systems-level treatment of Rust concurrency and memory ordering. Use when project work reaches threads, synchronization, atomics, or performance-sensitive shared state.
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
  Practical async Rust guide. Use when the project needs async IO, tasks, channels, or timers.
- [Ratatui Documentation](https://ratatui.rs/)
  Primary documentation for building terminal user interfaces in Rust. Use for layout, widgets, rendering, event handling, and application structure.
- [Ratatui GitHub Examples](https://github.com/ratatui/ratatui/tree/main/examples)
  Concrete examples for TUI patterns. Use when designing or testing screens and interactions.
- [The ZeroMQ Guide](https://zguide.zeromq.org/)
  Canonical ZeroMQ patterns guide. Use for PUB/SUB behavior, socket patterns, message framing, and the reliability caveats around topic subscription.
- [ZeroMQ Rust language page](https://zeromq.org/languages/rust/)
  Official ZeroMQ pointer to Rust ecosystem options. Use when choosing between libzmq bindings and pure Rust crates.
- [zmq crate documentation](https://docs.rs/crate/zmq/latest)
  Rust bindings for libzmq. Use for the first real ZMQ ingestion implementation, especially if we want behavior close to existing C/libzmq experience.
- [zeromq crate documentation](https://docs.rs/zeromq/latest/zeromq/)
  Async-oriented Rust ZeroMQ implementation. Use as a comparison point if the project should integrate more directly with async Rust instead of isolating blocking ZMQ IO on a thread.

## Wisdom (Communities)

- [The Rust Users Forum](https://users.rust-lang.org/)
  High-signal Q&A community for idiomatic Rust design questions, compiler errors, and library choices.
- [Ratatui Discord](https://ratatui.rs/discord/)
  Project community for Ratatui-specific architecture and widget questions.
- [This Week in Rust](https://this-week-in-rust.org/)
  Weekly ecosystem signal. Use to stay aware of library evolution and community practices.

## Gaps

- The TUI application direction is now a ZMQ topic monitor, but the exact message format, topic taxonomy, transport endpoint style, and required actions are still unknown.
- Company codebase conventions are unknown. Add internal style guides, architecture docs, or representative modules if they can be safely referenced without exposing confidential material.

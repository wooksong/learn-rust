# Teaching Notes

- User has strong C/systems background: Linux kernel, Android platform, performance/power/thermal profiling, NNStreamer, device drivers, Android MLOps.
- User is beginner-to-intermediate in Rust: has read TRPL once and completed Rustlings + Rustify.
- Avoid re-teaching basics unless needed; assume conceptual exposure but not fluency.
- Weak spots to target early: closures and concurrency.
- Preferred style: project-first, concept-reinforced, TDD-driven. Each concept should arise from the project, then get a focused review, short quiz, and implementation task.
- User wants depth and does not want to avoid unsafe, lifetimes, macros, or async if they arise naturally.
- The TUI app should be real and personally useful, not a toy.
- Project direction selected: a TUI similar to an async job monitor, but its data source is ZMQ topics. This should naturally teach closures, channels, ownership across threads/tasks, message parsing, backpressure, and UI state updates.

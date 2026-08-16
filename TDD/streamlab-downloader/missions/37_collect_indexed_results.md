# Mission 37 — Collect indexed results

## Goal

Have tasks return `(index, bytes)` instead of writing immediately.

## Red: test prompt

Concurrent tasks complete in arbitrary order but results keep indexes.

## Rust focus

tokio::spawn, JoinHandle, ownership.

## 100-exercises references

- `07_threads/11_locks`
- `08_futures/02_spawn`
- `08_futures/07_cancellation`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

Task output type must be `Send + static`.

## Compiler traps

- If a value must outlive the function input, return an owned type like `String` or `Vec<T>`.
- If async tasks complain about lifetimes, move owned values into the task.
- If type inference gets confusing, add explicit local variable types in tests first.


## Completion checklist

- [ ] I wrote the red test or test prompt before changing production code.
- [ ] I saw the test fail for the expected reason.
- [ ] I implemented the smallest green change.
- [ ] I refactored without changing behavior.
- [ ] I reviewed the Rust focus notes and linked 100-exercises references.
- [ ] I recorded any surprise, compiler error, or design decision in my notes.
- [ ] I ran the relevant check for this scope (`cargo test`, or explain why not yet).

## Spoiler

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 37` for hints and solution shape.

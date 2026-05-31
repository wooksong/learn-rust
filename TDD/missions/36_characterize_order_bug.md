# Mission 36 — Characterize out-of-order risk

## Goal

Create a failing test for concurrent writes that could produce wrong order.

## Red: test prompt

Slow A, fast B/C; expected output is ABC.

## Rust focus

characterization tests, concurrency bugs.

## 100-exercises references

- `07_threads/11_locks`
- `08_futures/02_spawn`
- `08_futures/07_cancellation`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

This documents why the refactor is needed.

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

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 36` for hints and solution shape.

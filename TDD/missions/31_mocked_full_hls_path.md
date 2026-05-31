# Mission 31 — Mocked full HLS path

## Goal

Exercise master -> media -> segment flow with mock HTTP.

## Red: test prompt

Mock master/media/segments; expect concatenated bytes.

## Rust focus

integration tests, orchestration.

## 100-exercises references

- `08_futures/01_async_fn`
- `08_futures/03_runtime`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

Keep fixture small: 2–3 segments.

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

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 31` for hints and solution shape.

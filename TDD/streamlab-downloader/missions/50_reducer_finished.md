# Mission 50 — Reducer for completion

## Goal

Mark download complete and display output path.

## Red: test prompt

DownloadFinished sets status complete and progress done.

## Rust focus

enums, PathBuf display.

## 100-exercises references

- `04_traits/01_trait`
- `06_ticket_management/04_iterators`
- `08_futures/06_async_aware_primitives`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

Completion should be idempotent if practical.

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

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 50` for hints and solution shape.

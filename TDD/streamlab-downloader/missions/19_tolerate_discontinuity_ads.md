# Mission 19 — Tolerate discontinuity and ad markers

## Goal

Ignore practical HLS tags that do not block basic downloading.

## Red: test prompt

Given cue/discontinuity tags around segments, expect segment list unchanged.

## Rust focus

defensive parsing, HLS metadata.

## 100-exercises references

- `05_ticket_v2/06_fallibility`
- `05_ticket_v2/08_error_enums`
- `06_ticket_management/04_iterators`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

Unknown tags should not become URIs.

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

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 19` for hints and solution shape.

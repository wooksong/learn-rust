# Mission 13 — Extract BANDWIDTH

## Goal

Parse `BANDWIDTH` from stream-info attributes.

## Red: test prompt

Given `BANDWIDTH=3956044`, expect integer `3956044`.

## Rust focus

u64 parsing, Result, parse errors.

## 100-exercises references

- `05_ticket_v2/06_fallibility`
- `05_ticket_v2/08_error_enums`
- `06_ticket_management/04_iterators`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

Start simple; quoted comma handling matters later for CODECS.

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

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 13` for hints and solution shape.

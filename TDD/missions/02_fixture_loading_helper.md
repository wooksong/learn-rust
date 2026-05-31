# Mission 02 — Fixture loading helper

## Goal

Add a small helper or convention for loading local `.m3u8` fixtures.

## Red: test prompt

Write a test using `apple_vod_relative.m3u8` and assert parsed segment URIs.

## Rust focus

std::fs::read_to_string, test file paths, unwrap in tests.

## 100-exercises references

- `02_basic_calculator/07_for`
- `06_ticket_management/02_vec`

## Green

Implement the smallest production change that satisfies the red prompt. Prefer simple, readable Rust over clever abstractions.

## Refactor

Keep helper test-local until duplicated.

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

See `../HINTS_AND_SOLUTIONS.md` and search for `Mission 02` for hints and solution shape.

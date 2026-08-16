# Mission 00 — Project split: library + thin CLI

## Goal

Prepare `streamlab-native` for test-driven development by moving reusable logic out of `src/main.rs` and into a library module.

## Red: test prompts

Start with one tiny library-level test.

Test name: `library_exposes_media_playlist_parser`

Given:

```m3u8
#EXTM3U
seg001.ts
```

Expect:

```text
parse_media_playlist(input) returns ["seg001.ts"]
```

## Rust focus

- crate layout: `main.rs` vs `lib.rs`
- `pub fn`
- module visibility
- unit tests with `#[cfg(test)]`
- integration tests later via `tests/`

## 100-exercises references

Required:

- `01_intro/00_welcome`
- `01_intro/01_syntax`

Optional:

- `03_ticket_v1/03_modules`
- `03_ticket_v1/04_visibility`

## Green

Create the smallest possible `src/lib.rs` exposing `parse_media_playlist`. It can temporarily duplicate logic from `main.rs`.

## Refactor

After the test passes:

- create `src/playlist.rs`
- re-export from `lib.rs`
- keep `main.rs` compiling

## Compiler traps

- `function is private`: add `pub` where the test needs it.
- `module not found`: check `mod playlist;` and file name.
- `unused function`: acceptable temporarily, or call through `main.rs` later.


## Completion checklist

- [ ] I wrote the red test or test prompt before changing production code.
- [ ] I saw the test fail for the expected reason.
- [ ] I implemented the smallest green change.
- [ ] I refactored without changing behavior.
- [ ] I reviewed the Rust focus notes and linked 100-exercises references.
- [ ] I recorded any surprise, compiler error, or design decision in my notes.
- [ ] I ran the relevant check for this scope (`cargo test`, or explain why not yet).

## Stretch

Add one integration-style test under `tests/` only after the unit test works.

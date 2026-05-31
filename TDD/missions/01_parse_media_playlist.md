# Mission 01 — Parse media playlist segment URIs

## Goal

Parse a media playlist and return only segment URI lines. This is the first real downloader behavior.

## Red: test prompts

### Test 1: ignores comments and blank lines

Given:

```m3u8
#EXTM3U
#EXT-X-TARGETDURATION:10

seg001.ts
# comment
seg002.ts
```

Expect:

```text
["seg001.ts", "seg002.ts"]
```

### Test 2: trims whitespace

Given:

```m3u8
#EXTM3U
  seg001.ts  
	seg002.ts	
```

Expect:

```text
["seg001.ts", "seg002.ts"]
```

### Test 3: preserves query strings

Given:

```m3u8
#EXTM3U
index_1_8779957.ts?m=1566416212
```

Expect:

```text
["index_1_8779957.ts?m=1566416212"]
```

## Rust focus

- `&str` input
- owned `String` output
- `Vec<String>`
- `.lines()`
- `.trim()`
- `continue`
- assertions with expected vectors

## 100-exercises references

Required:

- `02_basic_calculator/02_variables`
- `02_basic_calculator/07_for`

Helpful soon:

- `03_ticket_v1/06_ownership`
- `06_ticket_management/02_vec`

## Green

Implement the simplest loop:

- iterate over `content.lines()`
- trim each line
- skip if empty
- skip if starts with `#`
- push `line.to_string()`

## Refactor

Keep the signature simple for now:

```text
parse_media_playlist(content: &str) -> Vec<String>
```

Do not introduce `Result` until a later mission needs meaningful parser errors.

## Compiler traps

- `expected String, found &str`: call `.to_string()`.
- returning `Vec<&str>` may borrow from input; okay in some designs, but use `Vec<String>` now to avoid lifetime complexity.
- `assert_eq!` on vectors requires the same element type on both sides.


## Completion checklist

- [ ] I wrote the red test or test prompt before changing production code.
- [ ] I saw the test fail for the expected reason.
- [ ] I implemented the smallest green change.
- [ ] I refactored without changing behavior.
- [ ] I reviewed the Rust focus notes and linked 100-exercises references.
- [ ] I recorded any surprise, compiler error, or design decision in my notes.
- [ ] I ran the relevant check for this scope (`cargo test`, or explain why not yet).

## Stretch

Load `fixtures/hls/apple_vod_relative.m3u8` and assert it returns the four `fileSequence*.ts` entries.

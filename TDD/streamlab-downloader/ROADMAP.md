# Rust + TDD Learning Track: StreamLab Downloader

This track uses `100-exercises-to-learn-rust` as the just-in-time Rust concept source and `streamlab-native` as the real project spine.

## Agreed target

Build a portfolio-grade Rust downloader through TDD:

- tested library + thin CLI
- practical HLS parser
- typed library errors, `anyhow` in CLI
- mocked HTTP tests
- retry/backoff with Tokio simulated time
- concurrent downloads with ordered output
- staged Ratatui dashboard
- README and TDD mission checklist

## Working rules

- Default session length: 60–90 minutes.
- Unit of progress: TDD missions.
- Strict TDD for core logic; exploratory spikes allowed for unfamiliar integration APIs.
- Use `HINTS_AND_SOLUTIONS.md` only after a serious attempt; it contains spoiler hints and solution shapes for all missions.
- Standard quality loop per phase:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

- Commit after each phase, not after every tiny mission.
- Keep learning materials in this track directory (`TDD/streamlab-downloader/`). Apply code changes to `streamlab-native` only when executing a mission.

## Mission template

Each mission should include:

1. Goal
2. Red: exact test prompts, not copy-paste full test code
3. Rust focus: short concept brief
4. 100-exercises references
5. Green: implementation direction
6. Refactor: cleanup target
7. Compiler traps
8. Stretch

## Architecture target for `streamlab-native`

```text
streamlab-native/
  src/
    main.rs          # CLI only
    lib.rs           # public exports
    playlist.rs      # HLS parsing
    url.rs           # URL resolution
    error.rs         # typed library errors
    http.rs          # fetch/download primitives
    retry.rs         # retry/backoff policy
    downloader.rs    # orchestration and ordered output
    tui/             # Ratatui app state/rendering later
  tests/
    ...              # added when executing missions
```

## Phase overview

### Phase 0 — Setup and TDD workflow
Estimated: 2–3 sessions

Outcome: project can expose library functions and run first tests.

Missions:

00. Project split: `lib.rs` + thin `main.rs`
01. Parse media playlist: first pure function test
02. Add fixture loading helper for local `.m3u8` examples

### Phase 1 — Tiny Rust basics through downloader functions
Estimated: 5–7 sessions

Outcome: comfortable with `&str`, `String`, `Vec`, iteration, assertions, and small refactors.

Missions:

03. Ignore blank lines
04. Ignore comment/tag lines
05. Trim whitespace around segment URIs
06. Preserve query strings in segment URIs
07. Preserve absolute segment URLs
08. Preserve relative paths and nested segment paths
09. Add Apple VOD fixture prompt
10. Add RFC simple media fixture prompt
11. Add AWS ad-marker fixture prompt

### Phase 2 — Practical HLS parser
Estimated: 6–8 sessions

Outcome: parse useful HLS media/master structures without trying to implement all RFC 8216.

Missions:

12. Parse master playlist variants
13. Extract `BANDWIDTH`
14. Select highest bandwidth variant
15. Introduce `PlaylistVariant` struct
16. Extract optional `RESOLUTION`
17. Extract optional `CODECS`
18. Parse `MediaSegment { duration, uri }`
19. Tolerate discontinuity and ad marker tags
20. Introduce typed parser errors
21. Stretch: byte-range support

### Phase 3 — URL resolution
Estimated: 3–4 sessions

Outcome: robust enough URL joining for HLS playlists.

Missions:

22. Resolve absolute references unchanged
23. Resolve same-directory relative references
24. Resolve nested relative references
25. Resolve parent-directory references
26. Replace naive string concatenation with URL-aware logic if needed

### Phase 4 — HTTP and async testing
Estimated: 5–7 sessions

Outcome: deterministic HTTP tests using mock servers.

Missions:

27. `fetch_text` success with `mockito`
28. `fetch_text` maps 404 into typed error
29. `download_segment` returns bytes
30. Compare one equivalent test using `wiremock`
31. Integration path: master -> media -> segments using mock server

### Phase 5 — Retry/backoff
Estimated: 3–5 sessions

Outcome: fast retry tests without real sleeping.

Missions:

32. Retry transient 500
33. Stop after max retries
34. Use Tokio simulated time for backoff
35. Stretch: extract retry policy or sleeper abstraction

### Phase 6 — Concurrent ordered downloads
Estimated: 5–7 sessions

Outcome: concurrent fetch with deterministic output order.

Missions:

36. Characterize current out-of-order risk
37. Download concurrently but collect indexed results
38. Write output in playlist order
39. Propagate one segment failure cleanly
40. Enforce concurrency limit
41. Stretch: streaming ordered output as soon as contiguous bytes are ready

### Phase 7 — CLI
Estimated: 3–4 sessions

Outcome: CLI is thin and tested at boundary level.

Missions:

42. `download <url> --output <path>` command shape
43. `inspect <url>` prints variants/segments
44. Friendly error messages via `anyhow::Context`
45. Keep CLI parsing separate from downloader library

### Phase 8 — Ratatui state model
Estimated: 4–6 sessions

Outcome: TUI behavior testable without terminal IO.

Missions:

46. Define `DownloadEvent`
47. Define `AppState`
48. Reducer: master fetched / variants found / selected
49. Reducer: segment started / finished / failed
50. Reducer: download finished
51. Channel-based event bridge from downloader to TUI

### Phase 9 — Ratatui dashboard rendering
Estimated: 5–7 sessions

Outcome: display-only dashboard for `streamlab-downloader tui <url>`.

Missions:

52. Render static title/status panel
53. Render progress from state
54. Render selected variant
55. Render recent logs
56. Whole-screen render test with `TestBackend`
57. Stretch: `insta` snapshots after UI stabilizes

### Phase 10 — YouTube resolver learning track
Estimated: 4–6 sessions

Outcome: understand YouTube extraction as a resolver layer, not as part of the HLS downloader core.

Missions:

58. Learn the boundary: YouTube URL -> metadata/formats -> direct media or manifest URL
59. Shell out to `yt-dlp --dump-single-json` in a spike
60. Parse minimal `yt-dlp` JSON fixture into `YouTubeMedia` structs
61. Choose an HLS/DASH/progressive format from metadata
62. Feed extracted HLS manifest URL into existing downloader
63. Stretch: playlist enumeration with `--flat-playlist` vs full extraction

Important constraint: do not implement YouTube signature deciphering yourself in this track. Use `yt-dlp` as an external resolver for legitimate content you have rights to access. YouTube URLs can be signed, throttled, region-gated, DRM-protected, and short-lived.

### Phase 11 — Capstone polish
Estimated: 4–6 sessions

Outcome: portfolio-ready demonstration.

Missions:

64. README: architecture and TDD journey
65. README: run/test instructions
66. Add mission checklist
67. Add example screenshots or terminal capture
68. Run full quality loop and phase commit

## Stretch topics

- property-based parser tests with `proptest`
- `nom` parser comparison
- existing HLS crate comparison
- `cargo nextest`
- coverage
- Ratatui snapshots with `insta`
- Chrome extension/native messaging integration later

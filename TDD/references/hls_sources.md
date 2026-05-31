# HLS / M3U8 reference sources

Use these as inspiration for fixtures and edge cases.

## Apple TN2288

URL: https://developer.apple.com/library/archive/technotes/tn2288/_index.html

Good for:

- VOD playlist
- relative segment paths
- live sliding window
- event playlists
- variant playlists
- discontinuities
- encryption tags
- byte-range examples

## RFC 8216

URL: https://www.rfc-editor.org/rfc/rfc8216

Good for:

- authoritative syntax
- simple media playlist examples
- protocol terminology
- later parser correctness checks

## AWS MediaTailor examples

URL: https://docs.aws.amazon.com/mediatailor/latest/ug/manifest-hls-example.html

Good for:

- realistic multivariant playlists
- subtitles
- ad markers
- discontinuity tags
- query-string segment URLs
- personalized manifests

## Fixture policy

- Keep tiny edge cases inline in mission prompts.
- Keep realistic examples in `TDD/fixtures/hls/`.
- Do not attempt full RFC 8216 coverage in the core track.

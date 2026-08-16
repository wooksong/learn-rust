# YouTube playlist and media extraction notes

YouTube support should be treated as a separate resolver layer.

## Mental model

```text
YouTube watch/playlist URL
  -> resolver extracts metadata and available formats
  -> choose HLS, DASH, or progressive media format
  -> feed direct manifest/media URL into downloader core
```

The HLS downloader should not know about YouTube pages, signatures, cookies, or format selection. It should accept ordinary URLs and metadata produced by a resolver.

## Recommended resolver for this track

Use `yt-dlp` as an external tool during learning.

Useful commands to study:

```bash
# Single video metadata/formats as JSON
yt-dlp --dump-single-json --no-download "https://www.youtube.com/watch?v=VIDEO_ID"

# One JSON object per playlist entry/video
yt-dlp --dump-json --no-download "https://www.youtube.com/playlist?list=PLAYLIST_ID"

# Playlist metadata without fully resolving every entry
yt-dlp --flat-playlist --dump-single-json --no-download "https://www.youtube.com/playlist?list=PLAYLIST_ID"

# Show format table for manual inspection
yt-dlp -F "https://www.youtube.com/watch?v=VIDEO_ID"

# Print selected final URL only; useful for experiments, but URLs are short-lived
yt-dlp -f "best" -g "https://www.youtube.com/watch?v=VIDEO_ID"
```

## TDD-friendly design

Define small structs first from saved JSON fixtures:

```text
YouTubeVideo
  id
  title
  duration
  formats: Vec<YouTubeFormat>

YouTubeFormat
  format_id
  protocol
  url
  ext
  width
  height
  tbr
```

Then add pure selection functions:

```text
select_best_hls_format(formats)
select_best_progressive_format(formats)
select_audio_video_pair(formats)
```

## Important constraints

- Use only for content you have rights to access.
- Do not implement signature deciphering yourself in this course.
- YouTube direct media URLs are often signed and short-lived.
- Some formats are DASH audio/video split; merging may require `ffmpeg`.
- Some content may be DRM-protected, age-gated, private, region-gated, or require cookies; treat those as out of scope unless you explicitly decide otherwise.

## Where this fits in the roadmap

This belongs after the core HLS downloader works. The downloader core should be tested against ordinary HLS fixtures before adding YouTube resolver complexity.

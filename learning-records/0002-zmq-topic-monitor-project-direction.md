# ZMQ topic monitor project direction

The TUI project has narrowed from a generic async job monitor to a ZMQ topic-monitor application: it will subscribe to topics, ingest messages, and present live state in a terminal UI. This makes the learning path more authentic for the user's systems background and creates natural pressure to learn closures, concurrency, channels, message parsing, backpressure, and UI state management.

**Evidence**

The user said they wanted something similar to an async job monitor, but based on getting data from ZMQ topics.

**Implications**

Start with a fake in-process message source and a testable domain model before connecting real ZMQ. Treat the ZMQ boundary as an adapter so early lessons teach Rust design and TDD without being blocked by networking setup.

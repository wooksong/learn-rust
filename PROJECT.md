# Project: ZMQ Topic Monitor TUI

## Working idea

Build a terminal UI that subscribes to one or more ZMQ topics, ingests messages, and turns the stream into useful live state: recent messages, topic rates, parse errors, connection status, and filters.

## Learning purpose

This project is the spine of the Rust learning track. It should force useful Rust concepts to appear naturally:

- Closures: filtering, mapping, subscriptions, callbacks/adapters.
- Ownership and borrowing: message buffers, parsed events, UI state snapshots.
- Traits: source abstraction, parser abstraction, rendering boundaries.
- Error handling: malformed frames, disconnected sockets, invalid configuration.
- Concurrency: ingest loop separated from UI loop via channels.
- Async/threading tradeoffs: blocking `zmq` adapter first, possible async adapter later.
- Testing: fake message sources and deterministic state updates before real sockets.

## First architecture

```text
FakeSource / ZmqSource
        |
        v
  RawMessage { topic, payload, received_at }
        |
        v
     Parser trait
        |
        v
  DomainEvent / ParseError
        |
        v
   AppState::apply(event)
        |
        v
      Ratatui views
```

## Rule

Do not start by connecting to real ZMQ. Start with a fake in-process source so the domain model, tests, and UI state transitions are deterministic. Add the real ZMQ adapter only after the core behavior is test-backed.

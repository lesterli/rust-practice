## async-primer

This crate contains three examples:
- `src/bin/sync_example.rs`: simple blocking file I/O example
- `src/bin/multi_example.rs`: concurrency example
- `src/bin/async_example.rs`: async file I/O example

Run examples:

```rust
cd hands-on/async-primer
cargo run --bin sync_example
cargo run --bin multi_example
cargo run --features async-tokio --bin async_example
```

Run tests:
```rust
cargo test
cargo test --features async-tokio
```
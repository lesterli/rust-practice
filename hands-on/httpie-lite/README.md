## httpie-lite

A [HTTPie](https://github.com/httpie/cli)-like CLI demo written in Rust. Implements async HTTP requests (`GET`/`POST`) with pretty printing and optional syntax highlighting.

- `clap` for CLI parsing (subcommands: `get`, `post`).
- `reqwest` + `tokio` for async HTTP requests.
- Prints status, headers and body; uses `mime` to detect content type.
- Syntax highlighting for JSON/HTML via `syntect` and `colored` output.
- Error handling with `anyhow`.

### Quick start

- Build

```bash
cd hands-on/httpie-lite
cargo build
```

- GET

```bash
cargo run --bin httpie-lite -- get https://httpbin.org/get
```

- POST (simple key=value form -> JSON body)

```bash
cargo run --bin httpie-lite -- post https://httpbin.org/post name=alice age=30
```
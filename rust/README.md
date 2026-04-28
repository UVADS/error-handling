# Error Handling & Logging in Rust

- [Errors & Results](#errors--results)
- [Logging](#logging)

## Errors & Results

Rust deliberately omits the `try` / `catch` / exception model used by Python
and many other languages. In Rust, errors are *values*. There are two core
types:

- `Result<T, E>` — a function that may succeed (`Ok(T)`) or fail (`Err(E)`).
- `Option<T>` — a value that may be present (`Some(T)`) or absent (`None`).

The compiler forces you to handle both cases. You cannot accidentally
ignore an error: a `Result` you don't read produces a warning, and one you
unwrap without thinking can `panic!`. This makes failure paths explicit at
every call site.

### The `?` operator

Writing a `match` statement at every call would be exhausting. The `?`
operator propagates errors up the call stack — if the expression returns
`Err`, `?` returns from the current function with that error; otherwise it
unwraps the `Ok` value:

```rust
fn read_count(path: &str) -> Result<usize, std::io::Error> {
    let contents = std::fs::read_to_string(path)?; // returns Err on failure
    Ok(contents.lines().count())
}
```

This is roughly equivalent to a Python `try` block where every fallible
call raises — except that the error type is checked at compile time and
the propagation is visible in the source.

### `panic!` for unrecoverable errors

`panic!` aborts the current thread (and by default the whole program). Use
it only for situations the program cannot reasonably continue from —
programming bugs, broken invariants, or impossible states. It is *not* a
general-purpose error mechanism; that is what `Result` is for.

```rust
let port: u16 = std::env::var("PORT")
    .expect("PORT must be set")          // panics with this message if unset
    .parse()
    .expect("PORT must be a number");    // panics if not parseable
```

### Custom error types

For library code, define an enum that names every failure mode the library
can produce. The `thiserror` crate derives the `Display` and `Error`
boilerplate for you, and `#[from]` lets the `?` operator convert source
errors automatically:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read config: {0}")]
    Io(#[from] std::io::Error),

    #[error("missing required key: {0}")]
    MissingKey(String),
}
```

For application code where you don't need to match on specific variants,
the `anyhow` crate offers a single `anyhow::Error` type that wraps any
error and adds `.context(...)` breadcrumbs:

```rust
use anyhow::{Context, Result};

fn load() -> Result<Config> {
    let raw = std::fs::read_to_string("app.toml")
        .context("reading app.toml")?;
    // ...
}
```

A common rule of thumb: **`thiserror` for libraries, `anyhow` for binaries.**

## Logging

Rust's logging story has two layers. The `log` crate provides a *facade* —
the macros `error!`, `warn!`, `info!`, `debug!`, `trace!` — but no
implementation. The binary picks an implementation, which then receives
every log event from every dependency.

### `env_logger`

The simplest implementation. Configures itself from the `RUST_LOG`
environment variable:

```rust
use log::{info, error};

fn main() {
    env_logger::init();
    info!("starting up");
    error!("something went wrong: {}", "details");
}
```

```sh
RUST_LOG=info  cargo run                        # info, warn, error
RUST_LOG=debug cargo run                        # everything down to debug
RUST_LOG=myapp=trace,hyper=warn cargo run       # per-module filters
```

### `tracing`

For async code, structured fields, and span-based context, the `tracing`
crate is preferred. It supersedes `log` in most modern (tokio-based)
codebases. The macros look the same:

```rust
use tracing::{info, instrument};

#[instrument]
async fn handle_request(user_id: u64) -> Result<(), MyError> {
    info!(user_id, "request received");
    // ...
    Ok(())
}
```

`#[instrument]` automatically attaches the function's arguments to every
log event emitted within it — invaluable when correlating logs in a
distributed system.

## Read More

- [Result & Option basics](result_basic.rs)
- [Custom error types with `thiserror`](custom_error.rs)
- [Logging with `log` + `env_logger`](logging.rs)
- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [`thiserror`](https://docs.rs/thiserror/) · [`anyhow`](https://docs.rs/anyhow/) · [`log`](https://docs.rs/log/) · [`tracing`](https://docs.rs/tracing/)

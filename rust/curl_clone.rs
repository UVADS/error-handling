//! A minimal `curl`-like CLI: fetch a URL and print headers + body.
//!
//! This file is the showcase example for the rust/ folder — it pulls
//! together the patterns covered by the other snippets:
//!
//!   - `anyhow::Result` + `.context(...)` for breadcrumb error chains
//!     (the recommended pattern for binaries; see custom_error.rs for
//!     the library-side `thiserror` counterpart).
//!   - `?` for propagation; `main` returns `Result` so any error renders
//!     via the `Display` impl with the full context chain attached.
//!   - The `log` facade with deliberate level choices: INFO for the
//!     headline events a user wants in their terminal, DEBUG for noisier
//!     diagnostics, WARN for non-fatal anomalies (non-2xx responses).
//!
//! Cargo.toml:
//!     [dependencies]
//!     anyhow     = "1"
//!     clap       = { version = "4", features = ["derive"] }
//!     env_logger = "0.11"
//!     log        = "0.4"
//!     reqwest    = { version = "0.12", features = ["blocking"] }
//!
//! Usage:
//!     RUST_LOG=info  cargo run -- https://example.com
//!     RUST_LOG=debug cargo run -- https://httpbin.org/get

use std::io::{self, Write};
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info, warn};

/// A tiny curl-like fetch tool.
#[derive(Parser, Debug)]
#[command(name = "curl_clone", about = "Fetch a URL and print headers + body")]
struct Args {
    /// The remote URL to fetch.
    url: String,
}

fn fetch(url: &str) -> Result<()> {
    info!("GET {url}");

    let client = reqwest::blocking::Client::builder()
        .user_agent("curl_clone/0.1")
        .timeout(Duration::from_secs(30))
        .build()
        .context("building HTTP client")?;

    // `with_context` takes a closure so the format! only runs on error —
    // prefer it over `.context(format!(...))` whenever the message
    // allocates.
    let response = client
        .get(url)
        .send()
        .with_context(|| format!("requesting {url}"))?;

    let version = response.version();
    let status = response.status();
    let headers = response.headers().clone();

    info!("response: {status}");
    debug!("response carried {} headers", headers.len());

    if !status.is_success() {
        // curl still prints the body on a non-2xx response; we follow
        // suit, but surface it at WARN so it stands out in the log.
        warn!("non-success status: {status}");
    }

    // Status line + headers + blank line + body, à la `curl -i`. Locking
    // stdout up front avoids interleaving across many small writeln!s.
    let stdout = io::stdout();
    let mut out = stdout.lock();
    writeln!(out, "{version:?} {status}").context("writing status line")?;
    for (name, value) in &headers {
        // HeaderValue is `&[u8]`, not guaranteed UTF-8. Render lossily
        // so an exotic response can't crash the program.
        writeln!(out, "{name}: {}", String::from_utf8_lossy(value.as_bytes()))
            .context("writing header")?;
    }
    writeln!(out).context("writing header/body separator")?;

    // Read the body as bytes so binary responses (images, gzip, …)
    // pass through untouched.
    let body = response.bytes().context("reading response body")?;
    debug!("body: {} bytes", body.len());
    out.write_all(&body).context("writing body to stdout")?;

    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    fetch(&args.url)
}

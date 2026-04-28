//! Logging in Rust with the `log` facade and `env_logger`.
//!
//! `log` is the de-facto standard logging API: it provides macros
//! (`error!`, `warn!`, `info!`, `debug!`, `trace!`) but no implementation.
//! A binary crate picks an implementation — here, `env_logger`, which
//! configures itself from the RUST_LOG environment variable.
//!
//! For richer structured/async logging, see the `tracing` crate, which is
//! preferred in modern async (tokio) codebases.
//!
//! Cargo.toml:
//!     [dependencies]
//!     log = "0.4"
//!     env_logger = "0.11"
//!
//! Run with:
//!     RUST_LOG=info  cargo run         # info, warn, error
//!     RUST_LOG=debug cargo run         # everything down to debug
//!     RUST_LOG=mycrate=trace cargo run # per-module filtering

use log::{debug, error, info, warn};

fn divide(numerator: f64, denominator: f64) -> Result<f64, &'static str> {
    debug!("divide called with {numerator} / {denominator}");

    if denominator == 0.0 {
        // Pick the level that matches severity. A bad input the caller is
        // expected to handle is `warn!`; an internal failure is `error!`.
        warn!("denominator was zero — returning error");
        return Err("division by zero");
    }

    Ok(numerator / denominator)
}

fn main() {
    // Initializes from RUST_LOG. Default level is `error` if the var is unset.
    env_logger::init();

    info!("application starting");

    match divide(10.0, 2.0) {
        Ok(result) => info!("10 / 2 = {result}"),
        Err(e) => error!("divide failed: {e}"),
    }

    match divide(10.0, 0.0) {
        Ok(result) => info!("10 / 0 = {result}"),
        // `error!` records the message but does not capture a backtrace
        // by default. For backtraces in error chains, use `tracing` with
        // `tracing-error`, or set RUST_BACKTRACE=1 for panics.
        Err(e) => error!("divide failed: {e}"),
    }

    info!("application exiting");
}

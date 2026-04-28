//! Defining a custom error type with `thiserror`.
//!
//! Library code typically defines a single error enum that names every kind
//! of failure the library can produce. `thiserror` derives `Display` and
//! `std::error::Error` for you, and `#[from]` lets the `?` operator convert
//! source errors into your variant automatically.
//!
//! Cargo.toml:
//!     [dependencies]
//!     thiserror = "1"

use std::fs;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read config file: {0}")]
    Io(#[from] std::io::Error),

    // `#[source]` keeps the original error available via `.source()` while
    // letting us attach extra context (the offending key name).
    #[error("config value '{key}' is not a valid number: {source}")]
    InvalidNumber {
        key: String,
        #[source]
        source: ParseIntError,
    },

    #[error("missing required key: {0}")]
    MissingKey(String),
}

/// Load `key` from a flat KEY=VALUE config file and parse it as a `u16`.
///
/// `?` converts `io::Error` into `ConfigError::Io` automatically thanks to
/// `#[from]`. The parse failure is mapped by hand because we want to
/// attach the key name as context.
pub fn load_port(path: &str, key: &str) -> Result<u16, ConfigError> {
    let contents = fs::read_to_string(path)?;

    for line in contents.lines() {
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return v
                    .trim()
                    .parse::<u16>()
                    .map_err(|source| ConfigError::InvalidNumber {
                        key: key.to_string(),
                        source,
                    });
            }
        }
    }

    Err(ConfigError::MissingKey(key.to_string()))
}

fn main() {
    match load_port("config.ini", "port") {
        Ok(port) => println!("listening on {port}"),
        Err(e) => {
            // The `Display` impl from `#[error("...")]` is the human message;
            // `{:?}` (Debug) shows the structural form, useful for logs and
            // bug reports.
            eprintln!("startup failed: {e}");
            eprintln!("debug: {e:?}");
        }
    }
}

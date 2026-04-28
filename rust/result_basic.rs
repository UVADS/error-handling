//! Basic error handling in Rust: `Result`, `Option`, the `?` operator, and panic.
//!
//! Rust does not have exceptions. Recoverable errors are values of type
//! `Result<T, E>`, and missing-but-not-erroneous values are `Option<T>`.
//! Unrecoverable errors call `panic!`, which aborts the current thread.
//!
//! Compile and run (no external crates required):
//!     rustc result_basic.rs && ./result_basic /etc/hosts

use std::env;
use std::fs;
use std::num::ParseIntError;

/// Parse a string to a u32. Returns `Result` so the caller decides what
/// to do on failure rather than the function deciding for them.
fn parse_port(s: &str) -> Result<u32, ParseIntError> {
    s.parse::<u32>()
}

/// Read a file and count its lines. The `?` operator propagates any
/// `io::Error` to the caller without nested matches.
fn count_lines(path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().count())
}

/// Find the first line containing `needle`. Returns `Option` because a
/// missing match is not an error — it's just the absence of a value.
fn first_match<'a>(haystack: &'a str, needle: &str) -> Option<&'a str> {
    haystack.lines().find(|line| line.contains(needle))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // `expect` panics with the given message if the Option is None. Use
    // it for "this should never fail" situations, not for user errors.
    let path = args.get(1).expect("usage: result_basic <path>");

    // Pattern-matching the Result lets us act differently on each outcome.
    match count_lines(path) {
        Ok(n) => println!("{path}: {n} lines"),
        Err(e) => eprintln!("could not read {path}: {e}"),
    }

    match parse_port("8080") {
        Ok(port) => println!("parsed port: {port}"),
        Err(e) => eprintln!("invalid port: {e}"),
    }

    // `if let Some(...)` is the concise way to act only on the present case.
    if let Some(line) = first_match("foo\nbar\nbaz", "ba") {
        println!("first match: {line}");
    }
}

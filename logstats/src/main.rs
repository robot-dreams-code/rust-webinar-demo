#![cfg_attr(any(doc, test), doc = include_str!("../README.md"))]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

/// Aggregated statistics of an access log.
#[derive(Default)]
struct Stats {
    /// Number of well-formed lines.
    total: u64,

    /// Number of lines with a `5xx` status.
    errors: u64,

    /// Latencies in milliseconds, grouped by request path.
    latencies: HashMap<String, Vec<u32>>,
}

/// Possible errors of parsing an access log line.
#[derive(Debug)]
enum ParseError {
    /// Line doesn't consist of exactly 4 whitespace-separated fields.
    WrongFieldCount,

    /// Status or latency field isn't a valid number.
    BadNumber(ParseIntError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::WrongFieldCount => write!(f, "Expected 4 fields"),
            ParseError::BadNumber(value) => write!(f, "Bad number: {}", value),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::WrongFieldCount => None,
            ParseError::BadNumber(e) => Some(e),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::BadNumber(e)
    }
}

/// Parses the provided access log line into its path, status and latency.
///
/// # Errors
///
/// See the [`ParseError`] for details.
fn parse_line(line: &str) -> Result<(&str, u16, u32), ParseError> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let [_method, path, status, ms] = fields[..] else {
        return Err(ParseError::WrongFieldCount);
    };
    Ok((path, status.parse()?, ms.parse()?))
}

/// Analyzes the access log file at the provided path, skipping malformed lines.
///
/// # Errors
///
/// If opening or reading the file fails.
fn analyze(filename: &str) -> std::io::Result<Stats> {
    let reader = BufReader::new(File::open(filename)?);
    let mut stats = Stats::default();
    let mut skipped = 0;
    for line in reader.lines() {
        let line = line?;
        match parse_line(&line) {
            Ok((path, status, ms)) => {
                stats.total += 1;
                if status >= 500 {
                    stats.errors += 1;
                }
                stats
                    .latencies
                    .entry(path.to_string())
                    .or_default()
                    .push(ms);
            }
            Err(e) => {
                if skipped == 0 {
                    eprintln!("first bad line: {e}");
                }
                skipped += 1;
            }
        }
    }
    eprintln!("skipped: {skipped}");
    Ok(stats)
}

/// Calculates the 95th percentile of the provided `values`, sorting them in place.
///
/// Returns [`None`] if `values` are empty.
fn p95(values: &mut [u32]) -> Option<u32> {
    if values.is_empty() {
        return None;
    }
    values.sort_unstable();
    values.get(values.len() * 95 / 100).copied()
}

/// Runs the [`analyze()`] on the file passed as the first CLI argument and prints
/// [`Stats`] with p95 latency per path.
///
/// # Errors
///
/// If the [`analyze()`] fails.
///
/// # Panics
///
/// If no file path argument is provided.
fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1).expect("usage: logstats <file>");
    let mut stats = analyze(&filename)?;
    println!("total={} errors={}", stats.total, stats.errors);
    let mut paths: Vec<_> = stats.latencies.keys().cloned().collect();
    paths.sort();
    for path in paths {
        let v = stats.latencies.get_mut(&path).unwrap();
        match p95(v) {
            Some(ms) => println!("{path:15} p95={ms}ms"),
            None => println!("{path:15} no data"),
        }
    }
    Ok(())
}

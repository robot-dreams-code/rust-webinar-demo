#![cfg_attr(any(doc, test), doc = include_str!("../README.md"))]

use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Analyzes the access log file at the provided path the same way as `stats.py` does,
/// skipping malformed lines.
///
/// Returns `(total, errors, {path: p95_ms})` tuple.
///
/// # Errors
///
/// If opening or reading the file fails, raised as [`PyIOError`] in Python.
#[pyfunction]
fn analyze(filename: &str) -> PyResult<(u64, u64, HashMap<String, u32>)> {
    let file = File::open(filename).map_err(|e| PyIOError::new_err(e.to_string()))?;
    let mut total = 0;
    let mut errors = 0;
    let mut latencies: HashMap<String, Vec<u32>> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| PyIOError::new_err(e.to_string()))?;
        let fields: Vec<&str> = line.split_whitespace().collect();
        let [_method, path, status, ms] = fields[..] else {
            continue;
        };
        let (Ok(status), Ok(ms)) = (status.parse::<u16>(), ms.parse::<u32>()) else {
            continue;
        };
        total += 1;
        if status >= 500 {
            errors += 1;
        }
        latencies.entry(path.to_string()).or_default().push(ms);
    }

    let p95 = latencies
        .into_iter()
        .map(|(path, mut v)| {
            v.sort_unstable();
            let p = v[v.len() * 95 / 100];
            (path, p)
        })
        .collect();
    Ok((total, errors, p95))
}

/// Python module `logstats_rs` exposing the [`analyze()`] function.
///
/// # Errors
///
/// If registering the [`analyze()`] function in the module fails.
#[pymodule]
fn logstats_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze, m)?)?;
    Ok(())
}

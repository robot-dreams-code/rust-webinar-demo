logstats_rs
===========

[Python] extension module exposing `analyze(filename) -> (total, errors, {path: p95_ms})`: the same access log analysis as `stats.py`, implemented in [Rust] with [PyO3] and built with [maturin].

```bash
uv pip install ./pyext   # from the parent directory
uv run python -c "import logstats_rs; print(logstats_rs.analyze('access.log'))"
```

See the `README.md` of the parent directory for installation and benchmarking.




[maturin]: https://www.maturin.rs
[PyO3]: https://pyo3.rs
[Python]: https://www.python.org
[Rust]: https://www.rust-lang.org

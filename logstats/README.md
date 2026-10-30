logstats
========

CLI binary analyzing an access log of `METHOD PATH STATUS LATENCY_MS` lines: prints total requests, `5xx` errors and p95 latency per path, skipping malformed lines. Native [Rust] counterpart of `stats.py`.

```bash
cargo run --release -- ../access.log
```

See the `README.md` of the parent directory for installation and benchmarking.




[Rust]: https://www.rust-lang.org

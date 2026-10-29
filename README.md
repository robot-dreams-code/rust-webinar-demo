rust-webinar-demo
=================

Demo for the "Why you need [Rust]" webinar: the same access log analysis (total requests, `5xx` errors, p95 latency per path) implemented in pure [Python], as a native [Rust] binary, and as a [Rust] extension module called from [Python] via [PyO3].

| Path           | Purpose                                                                                                        |
|----------------|----------------------------------------------------------------------------------------------------------------|
| `gen_log.py`   | Generates a synthetic access log (`METHOD PATH STATUS LATENCY_MS` lines, ~1% malformed).                       |
| `stats.py`     | Pure [Python] analyzer (baseline).                                                                             |
| [`logstats/`]  | Same analyzer as a [Rust] CLI binary.                                                                          |
| [`pyext/`]     | Same analyzer as the `logstats_rs` [Python] module, built with [PyO3] and [maturin].                           |
| `measure.py`   | Measures wall time and peak memory (max RSS) of a command.                                                     |
| `compare.py`   | Runs the [Python] and [PyO3] analyzers in one process, asserts equal results and prints the speedup.           |




## Installation

Requires Linux or macOS (`measure.py` uses the Unix-only [`resource`] module) and a C linker (`build-essential` on Debian/Ubuntu, `base-devel` on Arch, `xcode-select --install` on macOS).

1. Install [Rust] via [rustup]:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    . "$HOME/.cargo/env"
    ```
2. Install [uv]:
    ```bash
    curl -LsSf https://astral.sh/uv/install.sh | sh
    ```

[Python] needs no separate installation: [uv] downloads the version pinned in `.python-version` ([docs][uv-python]).




## Usage

```bash
uv sync                                   # installs Python 3.11 and creates `.venv/`
uv run gen_log.py 5000000 access.log      # ~100 MB log

uv run stats.py access.log                                             # Python
cargo run --release --manifest-path logstats/Cargo.toml -- access.log  # Rust

uv run measure.py python python stats.py access.log
uv run measure.py rust logstats/target/release/logstats access.log

uv pip install ./pyext                    # builds `logstats_rs` with maturin
uv run compare.py access.log
```


### Profiling (optional)

Requires [`perf`] on Linux ([`dtrace`] on macOS). The release profile keeps debug symbols for readable frames.
```bash
cargo install flamegraph
cd logstats && cargo flamegraph -- ../access.log   # writes `flamegraph.svg`
```




## Further reading

- [The Rust Programming Language][Rust Book]
- [PyO3 user guide][PyO3]
- [maturin user guide][maturin]
- [uv: working on projects][uv-projects]
- [Flamegraphs][flamegraph]




[`dtrace`]: https://github.com/flamegraph-rs/flamegraph#installation
[`logstats/`]: logstats/README.md
[`perf`]: https://perfwiki.github.io/main
[`pyext/`]: pyext/README.md
[`resource`]: https://docs.python.org/3/library/resource.html
[flamegraph]: https://github.com/flamegraph-rs/flamegraph
[maturin]: https://www.maturin.rs
[PyO3]: https://pyo3.rs
[Python]: https://www.python.org
[Rust]: https://www.rust-lang.org
[Rust Book]: https://doc.rust-lang.org/book
[rustup]: https://rustup.rs
[uv]: https://docs.astral.sh/uv
[uv-projects]: https://docs.astral.sh/uv/guides/projects
[uv-python]: https://docs.astral.sh/uv/guides/install-python

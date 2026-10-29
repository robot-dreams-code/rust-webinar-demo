"""Comparison of the pure Python and the Rust (PyO3) analyzers in a single process.

Runs :func:`stats.analyze` and :func:`logstats_rs.analyze` on the same file, asserts
their results are equal and prints timings with the speedup.

Usage:

```bash
uv pip install ./pyext
uv run compare.py access.log
```

Raises:
    AssertionError: If results of the analyzers differ.
"""
import sys
import time

import logstats_rs
import stats

filename = sys.argv[1]

t = time.perf_counter()
s = stats.analyze(filename)
py_p95 = {p: stats.p95(v) for p, v in s.latencies.items()}
t_py = time.perf_counter() - t

t = time.perf_counter()
total, errors, rs_p95 = logstats_rs.analyze(filename)
t_rs = time.perf_counter() - t

assert (s.total, s.errors, py_p95) == (total, errors, rs_p95)
print(f"Python:      {t_py:.2f} s")
print(f"Rust (PyO3): {t_rs:.2f} s   ×{t_py / t_rs:.1f}")

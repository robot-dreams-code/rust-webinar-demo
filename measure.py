"""Measurement of wall time and peak memory (max RSS) of a child process.

Runs `cmd` with its output discarded and prints `label`, elapsed seconds and max RSS
in megabytes. Unix-only, as relies on the :mod:`resource` module.

Usage:

```bash
uv run measure.py <label> <cmd...>
```

Raises:
    subprocess.CalledProcessError: If `cmd` exits with a non-zero status.
"""
import resource
import subprocess
import sys
import time

label, cmd = sys.argv[1], sys.argv[2:]
t = time.perf_counter()
subprocess.run(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, check=True)
dt = time.perf_counter() - t
rss_mb = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss / 1024
print(f"{label:10} {dt:6.2f} s  {rss_mb:7.1f} MB")

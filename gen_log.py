"""Generator of a synthetic access log for benchmarking.

Writes `n` lines of `METHOD PATH STATUS LATENCY_MS` format, ~1% of them malformed,
using a fixed random seed, so the output is reproducible.

Usage:

```bash
uv run gen_log.py [n=5000000] [out=access.log]
```
"""
import random
import sys

n = int(sys.argv[1]) if len(sys.argv) > 1 else 5_000_000
out = sys.argv[2] if len(sys.argv) > 2 else "access.log"
random.seed(42)
paths = ["/", "/login", "/api/users", "/api/orders", "/api/items", "/health", "/static/app.js", "/search"]
methods = ["GET", "GET", "GET", "POST", "PUT", "DELETE"]
statuses = [200] * 90 + [201] * 3 + [404] * 4 + [500] * 3

with open(out, "w") as f:
    for i in range(n):
        if random.random() < 0.01:
            f.write("garbage line without fields\n")
            continue
        f.write(f"{random.choice(methods)} {random.choice(paths)} "
                f"{random.choice(statuses)} {random.randint(1, 900)}\n")

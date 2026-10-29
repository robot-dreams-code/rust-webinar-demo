"""Pure Python analyzer of an access log (baseline for the Rust implementations).

Prints total requests, `5xx` errors and p95 latency per path of the given file,
skipping malformed lines.

Usage:

```bash
uv run stats.py access.log
```
"""

import sys
from dataclasses import dataclass, field


@dataclass
class Stats:
    """Aggregated statistics of an access log.

    Attributes:
        total: Number of well-formed lines.
        errors: Number of lines with a `5xx` status.
        latencies: Latencies in milliseconds, grouped by request path.
    """

    total: int = 0
    errors: int = 0
    latencies: dict[str, list[int]] = field(default_factory=dict)


def parse_line(line: str) -> tuple[str, int, int]:
    """Parses the provided access log line into its path, status and latency.

    Raises:
        ValueError: If the line doesn't consist of exactly 4 whitespace-separated
            fields, or its status or latency isn't a valid integer.

    Examples:
        >>> parse_line("GET /login 200 433")
        ('/login', 200, 433)
    """
    method, path, status, ms = line.split()  # ValueError, якщо полів не 4
    return path, int(status), int(ms)


def analyze(filename: str) -> Stats:
    """Analyzes the access log file at the provided path, skipping malformed lines.

    Prints the number of skipped lines to `stderr`.

    Raises:
        OSError: If opening or reading the file fails.
    """
    stats = Stats()
    skipped = 0
    with open(filename) as f:
        for line in f:
            try:
                path, status, ms = parse_line(line)
            except ValueError:
                skipped += 1
                continue
            stats.total += 1
            if status >= 500:
                stats.errors += 1
            stats.latencies.setdefault(path, []).append(ms)
    print(f"skipped: {skipped}", file=sys.stderr)
    return stats


def p95(values: list[int]) -> int | None:
    """Calculates the 95th percentile of the provided `values`, sorting them in place.

    Returns `None` if `values` are empty.

    Examples:
        >>> p95(list(range(100, 0, -1)))
        96
        >>> p95([]) is None
        True
    """
    if not values:
        return None
    values.sort()
    return values[int(len(values) * 0.95)]


if __name__ == "__main__":
    s = analyze(sys.argv[1])
    print(f"total={s.total} errors={s.errors}")
    for path in sorted(s.latencies):
        print(f"{path:15} p95={p95(s.latencies[path])}ms")

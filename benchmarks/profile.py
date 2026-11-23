"""Profile one calculation with macOS `sample` and write a Markdown profile.

Usage:
    python3 benchmarks/profile.py NAME [--seconds 4] [--warmup 1] -- COMMAND [ARG ...]

COMMAND must repeat the calculation for at least warmup + seconds (for example a release
example given a large repeat count). The profile goes to benchmarks/profiles/NAME.md with
the command, commit, machine, the command's stdout (its timing line), the self-time
("top of stack") table and the inclusive time of every radiomics function.

Measurements from concurrent runs never overlap: the script holds an exclusive lock on
$TMPDIR/rust-radiomics-bench.lock while the command runs. Idle worker-thread frames
(condition waits, work-queue parking) are reported separately and excluded from the
tables, so percentages are shares of busy samples.
"""

import argparse
import fcntl
import os
import platform
import re
import subprocess
import tempfile
import time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
IDLE = (
    "__psynch_cvwait", "__workq_kernreturn", "swtch_pri", "semaphore_wait_trap",
    "mach_msg2_trap", "__ulock_wait", "__ulock_wait2", "__semwait_signal", "kevent",
    "__wait4", "__select", "thread_start", "_pthread_start",
)
LINE = re.compile(r"^(?P<lead>[\s+!:|]*)(?P<count>\d+)\s+(?P<symbol>.+?)\s+\(in (?P<image>[^)]+)\)")
HASH = re.compile(r"::h[0-9a-f]{16}$")


def clean(symbol: str) -> str:
    symbol = HASH.sub("", symbol.strip())
    return (symbol.replace("$LT$", "<").replace("$GT$", ">").replace("$u20$", " ")
            .replace("$C$", ",").replace("$u7b$", "{").replace("$u7d$", "}").replace("$RF$", "&").replace("..", "::"))


def parse(text: str):
    """Self and inclusive sample counts from the `sample` call graph."""
    self_counts, inclusive = Counter(), Counter()
    graph = text.split("Call graph:", 1)[-1].split("Total number in stack", 1)[0]
    nodes = []  # (depth, symbol, count) in call-graph order; depth is the indent width
    for line in graph.splitlines():
        m = LINE.match(line)
        if m:
            nodes.append((len(m.group("lead")), clean(m.group("symbol")), int(m.group("count"))))
    # Inclusive: count a symbol once per stack path, so recursion is not double counted.
    stack: list[tuple[int, str]] = []
    for depth, symbol, count in nodes:
        while stack and stack[-1][0] >= depth:
            stack.pop()
        if symbol not in {s for _, s in stack}:
            inclusive[symbol] += count
        stack.append((depth, symbol))
    # Self: a node's samples minus those of its direct children.
    for index, (depth, symbol, count) in enumerate(nodes):
        child_depth, children = None, 0
        for next_depth, _, next_count in nodes[index + 1:]:
            if next_depth <= depth:
                break
            child_depth = next_depth if child_depth is None else child_depth
            if next_depth == child_depth:
                children += next_count
        if count > children:
            self_counts[symbol] += count - children
    return self_counts, inclusive


def table(counts: Counter, busy: int, limit: int, only=None) -> list[str]:
    rows = ["| Samples | Share | Function |", "| ---: | ---: | --- |"]
    shown = 0
    for symbol, count in counts.most_common():
        if any(symbol.startswith(i) for i in IDLE) or (only and not only(symbol)):
            continue
        rows.append(f"| {count} | {100 * count / max(busy, 1):.1f}% | `{symbol[:160]}` |")
        shown += 1
        if shown == limit:
            break
    return rows


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("name", help="profile name, e.g. roi/glcm or filters/gabor-3d")
    parser.add_argument("--seconds", type=float, default=4.0)
    parser.add_argument("--warmup", type=float, default=1.0)
    parser.add_argument("--limit", type=int, default=30)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not command:
        parser.error("missing COMMAND after --")
    out = ROOT / "benchmarks" / "profiles" / f"{args.name}.md"
    out.parent.mkdir(parents=True, exist_ok=True)
    lock_path = Path(os.environ.get("TMPDIR", "/tmp")) / "rust-radiomics-bench.lock"
    with open(lock_path, "w") as lock, tempfile.TemporaryDirectory() as scratch:
        fcntl.flock(lock, fcntl.LOCK_EX)
        stdout_path = Path(scratch) / "stdout.txt"
        with open(stdout_path, "w") as stdout:
            process = subprocess.Popen(command, stdout=stdout, stderr=subprocess.STDOUT)
            time.sleep(args.warmup)
            sample_path = Path(scratch) / "sample.txt"
            subprocess.run(["sample", str(process.pid), str(args.seconds), "-mayDie",
                            "-file", str(sample_path)], capture_output=True, check=False)
            code = process.wait()
        text = sample_path.read_text() if sample_path.exists() else ""
        output = stdout_path.read_text()
    if code != 0:
        raise SystemExit(f"command exited with {code}:\n{output[-2000:]}")
    if not text:
        raise SystemExit("sample produced no profile; does the command run longer than --warmup?")
    self_counts, inclusive = parse(text)
    idle = sum(c for s, c in self_counts.items() if any(s.startswith(i) for i in IDLE))
    busy = sum(self_counts.values()) - idle
    commit = subprocess.run(["git", "-C", str(ROOT), "rev-parse", "--short", "HEAD"],
                            capture_output=True, text=True).stdout.strip()
    dirty = subprocess.run(["git", "-C", str(ROOT), "status", "--porcelain", "--untracked-files=no"],
                           capture_output=True, text=True).stdout.strip()
    chip = subprocess.run(["sysctl", "-n", "machdep.cpu.brand_string"], capture_output=True, text=True).stdout.strip()
    lines = [
        f"# Profile: {args.name}",
        "",
        f"- Command: `{' '.join(command)}`",
        f"- Commit: `{commit}`{' (with uncommitted changes)' if dirty else ''}",
        f"- Machine: {chip or platform.machine()}, {os.cpu_count()} logical CPUs",
        f"- Date: {datetime.now(timezone.utc).strftime('%Y-%m-%d %H:%M UTC')}",
        f"- Sampling: `sample`, {args.seconds:g} s after {args.warmup:g} s warm-up, 1 ms interval, all threads",
        f"- Busy samples: {busy} (idle worker-thread samples excluded: {idle})",
        "",
        "## Command output",
        "",
        "```text",
        output.strip()[-3000:],
        "```",
        "",
        "## Self time (top of stack)",
        "",
        *table(self_counts, busy, args.limit),
        "",
        "## Inclusive time, radiomics functions",
        "",
        *table(inclusive, busy, args.limit, only=lambda s: s.startswith(("radiomics::", "<radiomics::"))),
        "",
    ]
    out.write_text("\n".join(lines))
    print(f"wrote {out.relative_to(ROOT)} ({busy} busy samples)")


if __name__ == "__main__":
    main()

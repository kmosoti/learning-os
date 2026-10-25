"""Run on the repository filesystem, never the memory-backed /tmp default."""

import hashlib
import json
import platform
import subprocess
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BINARY = ROOT / "experiments/storage/target/release/learningos-storage-experiment"
OUTPUT = ROOT / "docs/experiments/results/storage-native.json"
LOCAL = ROOT / ".local"
LOCAL.mkdir(exist_ok=True)
result = {
    "at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    "platform": platform.platform(),
    "cpu": next(
        x.split(":", 1)[1].strip()
        for x in Path("/proc/cpuinfo").read_text().splitlines()
        if x.startswith("model name")
    ),
    "filesystem": subprocess.check_output(
        ["findmnt", "-T", str(LOCAL), "-o", "TARGET,FSTYPE,SOURCE,OPTIONS"], text=True
    ),
    "rust": subprocess.check_output(["rustc", "+1.99.0", "--version"], text=True).strip(),
    "lock_sha256": hashlib.sha256(
        (ROOT / "experiments/storage/Cargo.lock").read_bytes()
    ).hexdigest(),
    "conditions": (
        "WAL/FULL, no MVCC, fresh DB each run, batch=10, engine order alternated; "
        "full append port incl prepare/receipt; warm OS cache, no dropped caches"
    ),
    "runs": [],
    "failures": [],
}


def save():
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(json.dumps(result, indent=2) + "\n")


with tempfile.TemporaryDirectory(prefix="storage-", dir=LOCAL) as tmp:
    for repetition in range(3):
        for engine in ["sqlite", "turso"] if repetition % 2 == 0 else ["turso", "sqlite"]:
            for mode, count in [("check", 0), ("bench", 10000), ("bench", 100000)]:
                memory = Path(tmp) / "peak-rss.txt"
                command = [
                    str(BINARY),
                    mode,
                    engine,
                    f"{tmp}/{engine}-{count}-{repetition}.db",
                    str(count),
                    "10",
                ]
                p = subprocess.run(
                    ["/usr/bin/time", "-f", "%M", "-o", str(memory), *command],
                    capture_output=True,
                    text=True,
                    timeout=300,
                )
                if p.returncode == 0:
                    result["runs"].append(
                        {
                            **json.loads(p.stdout),
                            "repetition": repetition,
                            "peak_rss_kib": int(memory.read_text()),
                        }
                    )
                    print(engine, mode, count, repetition, "passed", flush=True)
                else:
                    result["failures"].append(
                        {
                            "engine": engine,
                            "mode": mode,
                            "repetition": repetition,
                            "code": p.returncode,
                            "stderr": p.stderr,
                        }
                    )
                    print(engine, mode, "FAILED", p.stderr[:500], flush=True)
                save()
    for engine in ["sqlite", "turso"]:
        for phase, expected in [("ack", 2), ("uncommitted", 0)]:
            for repetition in range(5):
                file = f"{tmp}/{engine}-{phase}-{repetition}.db"
                child = subprocess.Popen(
                    [str(BINARY), "child", engine, file, phase],
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                )
                boundary = child.stdout.readline().strip()
                child.kill()
                child.wait()
                start = time.perf_counter()
                p = subprocess.run(
                    [str(BINARY), "reopen", engine, file],
                    capture_output=True,
                    text=True,
                    timeout=30,
                )
                restart_ms = (time.perf_counter() - start) * 1000
                try:
                    reopened = json.loads(p.stdout)
                    assert boundary == ("ACK" if phase == "ack" else "UNCOMMITTED")
                    assert int(reopened["events"][0][0]) == expected
                    assert int(reopened["revision"][0][0]) == expected
                    assert int(reopened["receipts"][0][0]) == (1 if expected else 0)
                    result["runs"].append(
                        {
                            "engine": engine,
                            "test": "SIGKILL-" + phase,
                            "repetition": repetition,
                            "boundary": boundary,
                            "result": reopened,
                            "restart_ms": restart_ms,
                        }
                    )
                except (ValueError, AssertionError, KeyError) as e:
                    result["failures"].append(
                        {
                            "engine": engine,
                            "test": "SIGKILL-" + phase,
                            "error": str(e),
                            "stderr": p.stderr,
                        }
                    )
                save()
    print("failures:", len(result["failures"]), flush=True)
if result["failures"]:
    raise SystemExit(1)

"""Separate process writer contention, with the first transaction deliberately open."""

import json
import subprocess
import tempfile
import time
from pathlib import Path

root = Path(__file__).resolve().parents[2]
binary = root / "experiments/storage/target/release/learningos-storage-experiment"
results = []
with tempfile.TemporaryDirectory(prefix="contention-", dir=root / ".local") as tmp:
    for engine in ["sqlite", "turso"]:
        path = f"{tmp}/{engine}.db"
        first = subprocess.Popen(
            [str(binary), "child", engine, path, "uncommitted"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        assert first.stdout.readline().strip() == "UNCOMMITTED"
        start = time.perf_counter()
        second = subprocess.run(
            [str(binary), "check", engine, path],
            capture_output=True,
            text=True,
            timeout=15,
        )
        elapsed = (time.perf_counter() - start) * 1000
        first.kill()
        first.wait()
        reopened = subprocess.run(
            [str(binary), "reopen", engine, path],
            capture_output=True,
            text=True,
            check=True,
        )
        state = json.loads(reopened.stdout)
        assert second.returncode != 0
        assert "busy" in second.stderr.lower() or "locked" in second.stderr.lower()
        assert state["events"] == [["0"]] and state["revision"] == [["0"]]
        results.append(
            {
                "engine": engine,
                "gate": "independent process writer blocked, rollback after SIGKILL",
                "second_returncode": second.returncode,
                "second_error": second.stderr,
                "bounded_error_ms": elapsed,
                "reopened": state,
            }
        )
(root / "docs/experiments/results/storage-native-concurrency.json").write_text(
    json.dumps(results, indent=2) + "\n"
)
print(json.dumps(results, indent=2))

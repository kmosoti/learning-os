"""Keep summaries reviewable while retaining every raw timing sample as deterministic gzip."""

import gzip
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parents[2] / "docs/experiments/results"
for path in sorted(root.glob("storage*.json")):
    value = json.loads(path.read_text())
    if not isinstance(value, dict) or not any("samples_ms" in r for r in value.get("runs", [])):
        continue
    raw_path = path.with_suffix(".samples.json.gz")
    samples = []
    for run in value["runs"]:
        if "samples_ms" in run:
            samples.append(
                {
                    key: run[key]
                    for key in ["engine", "events", "batch", "repetition", "samples_ms"]
                    if key in run
                }
            )
            del run["samples_ms"]
    raw = json.dumps(samples, separators=(",", ":")).encode()
    raw_path.write_bytes(gzip.compress(raw, compresslevel=9, mtime=0))
    value["raw_samples"] = {
        "file": raw_path.name,
        "uncompressed_sha256": hashlib.sha256(raw).hexdigest(),
    }
    path.write_text(json.dumps(value, indent=2) + "\n")
    print(path.name, "samples retained in", raw_path.name)

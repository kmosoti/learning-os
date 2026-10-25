"""Pin source content independently of a later stacked-branch rebase."""
import hashlib
import json
import pathlib
import subprocess
import sys

source = pathlib.Path(sys.argv[1]).resolve()
output = pathlib.Path(sys.argv[2])
bundles = [pathlib.Path(value).resolve() for value in sys.argv[3:]]
paths = [source / "Cargo.toml", source / "Cargo.lock", source / "rust-toolchain.toml"]
for folder in ["crates", "apps", "fixtures", "sources", "scripts"]:
    paths += [p for p in (source / folder).rglob("*") if p.is_file() and "pkg" not in p.parts
              and p.suffix != ".gz" and p.name != "sw.js"]
files = {str(p.relative_to(source)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}
artifact_files = {}
for bundle in bundles:
    artifact_files[bundle.name] = {str(p.relative_to(bundle)): {"bytes": p.stat().st_size,
        "sha256": hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(bundle.rglob("*")) if p.is_file()}
output.parent.mkdir(parents=True, exist_ok=True)
output.write_text(json.dumps({"source_head": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=source, text=True).strip(),
    "source_files": files, "source_tree_digest": hashlib.sha256(json.dumps(files, sort_keys=True).encode()).hexdigest(),
    "artifacts": artifact_files}, indent=2) + "\n")

# Storage experiment

Run from the repository root:

```sh
scripts/benchmark-storage.sh
```

Prerequisites: Rust 1.99.0 with `wasm32-unknown-unknown`, a C compiler/libclang for the
native candidate, wasm-bindgen CLI **0.2.129**, Node/npm, Python 3, Chrome at
`/usr/bin/google-chrome`. The script installs the locked Node dependencies and
Playwright Firefox. No account, provider key, remote database or live inference.
Run on an SSD filesystem; `.local/` holds disposable private benchmark databases and
persistent browser profiles. Never point the experiment at a user database. It
creates fresh uniquely named files and profiles; remove those only after collecting
needed results. A memory-backed `/tmp` result is not a durability benchmark.

`src/main.rs` is the shared native contract/harness for Turso and SQLite.
`browser-rust/src/lib.rs` is the standalone Rust/IndexedDB adapter. `browser/harness.js`
is test orchestration and the Turso package's JS interop, not product logic.
`run_native.py` alternates native engines across three repetitions. Browser `run.mjs`
checks Chrome/Firefox and measures two Chromium datasets; `lifecycle.mjs` covers
multi-tab CAS, migration rollback, quota exhaustion, worker death and genuine Chrome
process-group SIGKILL/reopen. It only kills processes that it launched. `memory.mjs`
records a separate process-RSS snapshot after 10k events; it contributes no latency samples.

Baseline or harness failures return a failing exit code. A Turso applicability
failure is retained as a candidate result rather than concealing the comparison.
`STORAGE_CHECK_ONLY=1 node experiments/storage/browser/run.mjs` runs current binding
conformance without replacing the timed result file. Raw sample arrays are packed
losslessly by `pack_results.py`; JSON summaries link their deterministic gzip files.

The fresh-profile quota test verifies the CDP override and uses incompressible
payloads. Earlier reused-origin tests did not force exhaustion and remain documented;
do not replace actual failure injection with an assumption based on quota reporting.

See [selection and limitations](../../docs/experiments/storage-selection.md). No
mechanism automatically falls back or migrates a database after a failure. These
research stores are not the product's data format.

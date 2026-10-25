# Embedded journal selection — experiment B

Issue [#50](https://github.com/kmosoti/learning-os/issues/50). Measured 2026-10-25.
This is a mechanism experiment, not a claim about complete application latency.

## Decision

Use **bundled SQLite through rusqlite 0.41.0 natively** and **Rust IndexedDB in the
browser**, with strict durability requested and acknowledgment after transaction
completion. Keep Turso 0.8.3 as a measured alternative, not a runtime fallback.

Native Turso passed the exercised correctness gates. Its smaller process-restart
latency is useful, but it did not establish a repeatable overall advantage for the
journal workload: replay and peak RSS were worse, and append results varied by run.
The browser candidate cannot open the same OPFS journal concurrently in two tabs in
either tested browser. Killing its I/O worker left an append unresolved after three
seconds. A cross-tab storage broker and explicit worker recovery could change that
result, but neither exists in this pinned candidate. Do not select a mechanism that
fails the slice's required lifecycle merely because its stress throughput is better.

## Artifacts and conditions

- Rust 1.99.0, Turso native/browser 0.8.3 at
  `607bff9fd0e547a25a97b3bf06e83bcc5d99d53c`. Native SQLite reports 3.54.1;
  Turso reports compatibility version 3.50.4 (not a SQLite implementation claim).
- `experiments/storage/Cargo.lock`, `browser-rust/Cargo.lock`, and
  `browser/package-lock.json` pin resolved dependencies. Build artifacts and hashes
  are recorded in [artifact manifest](results/storage-artifacts.json).
- Intel i7-10750H, 32 GiB RAM, Fedora kernel 7.2.9, encrypted btrfs SSD filesystem.
  Native runner records the exact mount. Browser profiles also live on that filesystem.
  Chrome 155; Playwright Firefox 142.0.1. Headless desktop, no mobile claim.
- WAL, synchronous FULL, FK enabled, no MVCC. Both native engines use their default
  1,000-frame automatic checkpoint thresholds (verified in pinned sources:
  `turso_core/storage/wal.rs` and bundled `sqlite3.c`). They implement checkpointing
  differently; this is not a claim that their internal I/O schedules are identical.
- Fresh file per run; warm OS cache, no dropped caches. Three native repetitions,
  alternating engine order. Timed runs were isolated from other task compilation
  and from the browser benchmark. Filesystem/background-system variability remains.
- Same deterministic 10k/100k event payloads, ten events per committed operation,
  operation key/request, revision CAS, projection, receipt and ordered replay.
  Fixture hashes match native and browser results. p50/p95 cover whole append calls,
  including statement preparation and the commit boundary. Total duration also
  includes fixture serialization/hashing; replay materializes payloads in memory.
- Browser connection timing is warm after conformance loads the binding. It excludes
  cold engine download/initialization. The Turso bundle is 15,997,703 bytes raw /
  5,619,716 gzip; the standalone Rust IDB experiment is about 245k WASM + 24k JS raw
  (~83k combined gzip). These are experiment artifacts, not final application sizes.

## Native results

Median of three runs; append unit is one ten-event transaction, milliseconds.
Ranges are retained in the raw data and must not be hidden by these medians.

| Engine | Events | Append p50 | Append p95 | Replay | Peak RSS KiB |
|---|---:|---:|---:|---:|---:|
| SQLite | 10,000 | 7.107 | 10.277 | 3.221 | 12,732 |
| Turso | 10,000 | 7.330 | 9.843 | 5.203 | 27,000 |
| SQLite | 100,000 | 7.127 | 10.713 | 31.844 | 47,384 |
| Turso | 100,000 | 7.458 | 11.532 | 57.475 | 70,260 |

At 10k, SQLite p95 ranged 3.668–12.604ms and Turso 8.579–11.422ms. The warm-up/run
variation is material; this evidence does not support a universal latency ranking.
Process startup plus reopen/query measured roughly 15–21ms SQLite and 2.7–5.6ms Turso.

Both passed atomic append, retry-before-CAS, conflicting payload, stale revision,
rollback, unique/check/FK constraints, aborted migration and ordered replay. Five
SIGKILL-after-ACK and five SIGKILL-before-COMMIT schedules per engine lost zero
acknowledged events; uncommitted events/receipts/projections were absent. Independent
writer processes returned bounded lock errors without partial writes. SQLite waited
its default five-second busy timeout; Turso rejected the file lock immediately.
These are process failures, not power-cut or storage-controller durability tests.

## Browser results

One full workload per size/engine, 1,000/10,000 committed-operation samples. This
stress run was sufficient to expose the tradeoff; it does not establish repeatable
mobile performance. Browser applicability failures dominate selection.

| Engine | Events | Append p50 ms | Append p95 ms | Replay ms |
|---|---:|---:|---:|---:|
| Rust IndexedDB | 10,000 | 5.990 | 6.565 | 67.625 |
| Turso OPFS | 10,000 | 4.570 | 8.705 | 81.925 |
| Rust IndexedDB | 100,000 | 8.405 | 14.145 | 730.460 |
| Turso OPFS | 100,000 | 4.560 | 8.785 | 788.420 |

Both passed basic journal/reopen/receipt conformance in Chromium and Firefox and
three Chromium process-kill-after-ACK schedules per engine. IDB's two-tab race
committed exactly one revision and rejected the other as stale; Turso's second
connection failed with `NoModificationAllowedError` from OPFS access-handle locking.
IDB migration rollback preserved events, a held old connection produced a blocked
upgrade, and an unsupported future version failed closed. These migration checks
exercise the browser primitive; the application must separately test its own upgrade UI.

Early quota injectors on a reused origin did not exhaust effective storage, even
with incompressible payloads and a CDP quota override. Those are retained as failed
test setups. The precise cause of the browser's prior allocation behavior is not
established here. A fresh persistent profile with incompressible payloads and a
verified 1,024-byte CDP override produces `QuotaExceededError`, revision zero, an
empty journal and no acknowledgment. Final lifecycle results use that isolated
setup; no application fault handling is inferred from the failed injections.

## Reproduction and limits

Run `scripts/benchmark-storage.sh` with Rust 1.99.0, Node, npm, wasm-bindgen 0.2.129,
Chrome and Playwright's Firefox available. It builds locked native/WASM experiments,
runs native tests before browser tests (never concurrently), and retains raw samples.
`web_sys_unstable_apis` is required by the pinned binding's strict-durability options;
it does not mean the app silently accepts default durability.

The imported Turso npm package's documented default export path is absent from this
published artifact; the shipped `bundle/main.es.js` entry works and is used explicitly.
OPFS requires COOP/COEP in this harness. No server, account, remote storage or paid API
participates. Browser crash recovery is Chromium-only; Firefox functional checks do
not imply Firefox crash testing. Real Android, Safari, eviction and physical power
failure remain separate gates. Browser quotas/user clearing can still remove data.

The early tmpfs run and the interrupted run with concurrent I/O are retained as
exploratory artifacts, excluded from the selection table. Initial server-path and
quota-injector failures are also retained. Native/browser scripts are measurement
orchestrators; the browser baseline's journal adapter is Rust. The final UI may use
a different bounded snapshot strategy and must measure its own complete save path.

A future reconsideration of Turso requires a working multi-tab broker, bounded worker
recovery, the same conformance suite, a repeated benefit on target hardware, and an
explicit export/replay migration. An error never opens an empty alternate database.

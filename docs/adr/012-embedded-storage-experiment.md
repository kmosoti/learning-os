# ADR-012: Embedded journal storage experiment

Status: Proposed formal record; fixed direction accepted and implementation authorized by the owner on 2026-10-25 with “Implement the proposed plan.” Mechanism choices are deferred to the approved experiments. PR approval/merge remains pending.

Related: [#49](https://github.com/kmosoti/learning-os/issues/49) and [execution contract](../architecture/v2-execution-contract.md).

## Decision
No PostgreSQL now or as fallback. Compare native Turso 0.8.3 with stable bundled SQLite; independently compare browser Turso WASM/OPFS with a Rust IndexedDB adapter. The browser is the personal PWA's evidence owner. Native storage persists the source/lesson catalog and exercises the same journal semantics.

## Contract
Atomic operation-key lookup, expected-revision check, event batch, projection and receipt. Same key/same payload returns the receipt; changed payload or stale revision fails. An acknowledged answer must already be committed. Serialize native writes; browser transactions enforce revision checks across tabs. Snapshots are disposable projections; events and immutable versions are durable evidence.

## Selection
Run the shared conformance and benchmark protocol in [first slice](../experiments/first-vertical-slice.md). Correctness precedes performance. Match durable settings and payloads. Disable experimental MVCC; never run SQLite/Turso simultaneously against one file. Turso must pass required recovery/SQL/compatibility and demonstrate benefit without material regressions. Otherwise choose SQLite/IDB, documenting measurements and gaps rather than familiarity.

## Migration and failure
Use a separate v2 DB. No legacy in-place migration, engine autodetection switch or silent fallback after corruption. Require verified export/replay before an explicit backend migration. Quota/transaction failures retain drafts and never show success. Unsupported versions fail closed with export/recovery instructions. User deletion/OS/browser eviction cannot be prevented by an application durability claim.

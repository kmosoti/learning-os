# First vertical slice: verification protocol

Refs #49, #50, #51, #52. This initial document defines experiments, not results.
Reports must retain failed runs and distinguish desktop emulation from real phones.

## Reproducible fixture

One ownership/borrowing goal; explicit source IDs, immutable span hashes, question
and rubric versions, fixed event IDs/time in tests. Independent Rust compiler checks
establish two valid examples and E0382/E0502 failure examples. Deterministic
observation/evaluation/projection/next-action traces are separately authored oracles.
No provider key or live LLM call is required.

Exercise goal selection, plan, prediction, reading thoughts, check reveal, answer,
optional confidence/reasoning, evidence inspection, next action, correction/retraction,
export/import, and offline reload. Check declined/incorrect/hinted/unknown paths.
No single answer may produce a lifetime-mastery claim.

## Gates and commands

The implementation supplies locked scripts for native tests, WASM tests, release
build, browser tests and benchmarks. Required checks include `cargo fmt --all --check`,
Clippy with warnings denied, `cargo test --locked`, native/WASM replay equality,
compiler fixture tests and legacy `uv run` Ruff/ty/pytest checks where applicable.
Do not combine incompatible SSR/hydrate features through a blanket all-features build.

The browser suite must install/cache, disable networking, hard-reload a session URL,
answer and reload again; kill the browser process after a save acknowledgment and
reopen the same profile. Test lost ACK retries, duplicate IDs, conflicting payloads,
two-tab revision races, quota failure, worker failure, aborted migration, upgrade
with another tab open, and draft preservation. Export/reimport must preserve the
event digest and projection. Never substitute page close for process-kill evidence.

Run axe plus keyboard, screen-reader, 200% zoom/text, reduced-motion and mobile
keyboard checks. Screenshots: 1440×900, 1280×720, 390×844, 360×740. Check concealed
answers in DOM/accessibility output, malicious source text, CSP and public bundles.
Real Android installation/offline/performance remains an explicit owner-run release
gate. Safari is measured separately; never imply blanket support from Chromium.

## A: Hydration and PWA lifecycle

Compare the same fixture/UI/storage as independent islands versus a coherent session
island. SSR static shell in both. Measure raw/compressed WASM/JS/CSS, hydration time,
LCP/CLS, scripted interaction/event timing, memory and state continuity. Use at least
30 cold, warm and offline runs per candidate. Chromium mobile lab profile: 390×844,
4× CPU slowdown, 1.6Mbps down/750Kbps up/150ms latency; also unthrottled desktop and
Firefox functional checks. Record actual browser versions and hardware.

Both must pass accessibility and offline lifecycle. Prefer the coherent island when
results are within 10%; finer boundaries need repeatable benefit without material
regressions. If neither meets budgets, fix traces within Leptos; no framework swap.
Beware opaque SSR children: hidden answer content must not be shipped as children
that can only be conditionally shown after hydration.

## B: Embedded storage

Native candidates: Turso 0.8.3 at 607bff9fd0e547a25a97b3bf06e83bcc5d99d53c,
and bundled SQLite through rusqlite 0.41.0. Browser: matching Turso WASM/OPFS versus
Rust IndexedDB with strict durability requested. Record resolved SQLite version,
compile flags, database filesystem, WAL/FULL settings and checkpoint policy.

Same schema and journal contract: expected revision, operation key and payload,
event batch, receipt, snapshot. Shared tests: transactions, rollback, unique/check/FK
constraints used by the app, duplicate/conflicting IDs, concurrent writers, schema
migrations, reopen, export and replay. Workloads: 10k and 100k seeded events, identical
payload distribution; separate engine timings from full application-port timings.
Measure durable append p50/p95, replay, restart and memory. Crash tests identify the
exact ACK boundary. No experimental MVCC, relaxed-sync advantage or mixed-engine
concurrent access. Check required SQL against the pinned release COMPAT, not main.

Correctness gates dominate performance. Turso must pass all required behavior and
show repeatable benefit without material regression; otherwise select SQLite/IDB.
Unsupported browser/worker/OPFS setup is a failed applicability gate, not a timing of
zero. A runtime error never triggers an automatic switch to an empty second store.

## C: Graph renderer

Use seed 42, fixed positions/camera paths, 1k/10k nodes and explicit multi-premise
arguments. Compare SVG/DOM and Canvas2D at equivalent labels/culling. Record frame
p50/p95, CPU, memory and semantic-list parity. No GPU route unless traces justify it.
The 10k case is a stress fixture, not an automatic scalability claim.

## Budgets and report

Targets: LCP p75 ≤2.5s, INP p75 ≤200ms, CLS ≤0.1; 1k graph frame p95 ≤33ms on the
eventual target phone; local domain decision p95 ≤100ms. Zero acknowledged attempts
lost in the tested crash schedules. Lab event timings are not field INP statistics.

Every report identifies revision, lockfiles, fixture hash, hardware, browser, settings,
cache state, samples, commands, raw artifacts, failures and the resulting choice.
Do not publish invented metrics, hide missing device gates or call a passing subset
completion. The final PR report identifies the next bounded Markdown-source slice.

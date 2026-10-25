# Codex Agent Mission — Pivot `kmosoti/learning-os` to LearningOS 2.0

**Mission type:** Repository re-architecture, design-first, then a real vertical slice.  
**Authority:** Direct owner direction dated 2026-10-25, subject to existing PR/review governance.  
**Canonical specification:** `PRELIMINARY_SPEC.md` supplied with this brief; place an approved copy at `docs/architecture/LearningOS-v2-preliminary.md` or equivalent in a scoped documentation PR.  
**Execution mode:** Autonomously investigate and produce reviewable work. Avoid unapproved destructive operations; do not merge or rewrite history.

## Direct order

You are assigned to rehydrate and completely reengineer **https://github.com/kmosoti/learning-os**. The target is an installable, accessible, highly performant **Rust-native PWA** with **Leptos islands**, an **Axum** backend, a strictly hexagonal domain/application architecture, and **no PostgreSQL**. Reproduce the **presentation, navigation, visual density and interaction choreography** of **https://github.com/amosblomqvist/learnharness** as the *first* UI reference, while replacing the entire conceptual learning engine with the stronger evidence-grounded LearningOS model.

**This is not** an instruction to translate FastAPI endpoints one-for-one, retain HTMX, build a generic React SPA, fork LearnHarness, build a PostgreSQL-backed SaaS, or offer a broad technology-shopping list. Product direction is already settled. Investigate uncertain engineering mechanisms, implement measured experiments, and show evidence.

### User-fixed constraints

- **Rust-first application:** domain, application, backend, UI interaction logic and adapters authored in Rust. Generated WASM bindings and minimal documented browser glue are allowed only where the platform requires them. Do not infer `no_std` as a requirement.
- **Leptos SSR + `#[island]` selective hydration:** avoid adding Yew, Dioxus, React, Next.js, or Electron to the new execution path.
- **Axum + Tokio:** default HTTP/streaming transport and backend composition root; do not overengineer microservices.
- **No PostgreSQL:** use a pluggable embedded SQLite-family database; evaluate **Turso** for native/browser storage while retaining a proven compatible fallback if Turso fails the spike. Do not introduce Postgres as a future 'safety' path.
- **Faithful LearnHarness UI baseline:** independently implement the visible interface; improve only after obtaining screenshots, component/interaction inventories, visual comparisons, and UX acceptance tests.
- **Completely new domain logic:** source assertions, multi-premise reasoning, learner observations, evaluations, capability estimates, and intervention selection have distinct responsibilities and versions.
- **PWA/mobile/performance/offline behavior begins immediately**, not at the end of the roadmap.
- **Keep LearningOS original intellectual substance:** thesis, tests, architecture standards and user-owned project history are valuable even when Python implementation is replaced.

## MANDATORY: understand the actual repositories first

1. On the checked-out LearningOS working tree run and record: `pwd`, `git status --short`, `git branch --show-current`, `git log -5 --oneline`, `find . -maxdepth 3 -type f | sort | head -200` (adapt safely if environment differs).
2. Read `AGENTS.md` **before modifying anything**. It specifies GitHub Project 5, issue branches, ADR gates, owner review and approval. User explicitly requested a new design direction; that authorizes *design/triage* without pretending old issue cards are already Ready. Follow the workflow for PRs and do not merge.
3. Audit existing source and tests: `README.md`, `docs/00-project-brief.md`, `docs/01-requirements.md`, `docs/05-decision-log.md`, `docs/07-engineering-standards.md`, `docs/adr/010-mvp-runtime-and-serving-stack.md`, `docs/thesis/*`, `app/`, `tests/`, `pyproject.toml`, SQLite migrations. Record what actually exists vs what documentation merely planned.
4. Read the current LearnHarness README, architecture, and the actual UI files: `components/home.tsx`, `sidebar.tsx`, `graph-view.tsx`, `graph-canvas.ts`, `chat.tsx`, `plan-window.tsx`, `question-dock.tsx`, `practice-cards.tsx`, `activity-dock.tsx`, `app/globals.css`; read screenshots in `docs/assets/`. Pin commit SHA. Review the prober, teacher, practice and extraction logic to learn **interaction intent and failure cases**, not to copy the logic.
5. Confirm versions/API signatures against the **installed crates and official docs**. For Leptos, begin with https://docs.rs/leptos/latest/leptos/attr.island.html and verify SSR/islands features and restrictions against the exact selected version. For Turso, read https://github.com/tursodatabase/turso/blob/main/COMPAT.md and run a small on-device compatibility probe. No outdated Yew examples or guessed APIs.
6. Preserve as-built facts, provenance, upstream references, and measurements in docs. Do not state that any design target has already been achieved.

## Existing decisions that need supersession

The prior `docs/05-decision-log.md` records **ADR-003** (Python 3.14), **ADR-004** (FastAPI/SQLAlchemy/SQLite/NetworkX), proposed **ADR-005** (HTMX), and **ADR-010** (Python application and later PostgreSQL pathway). These conflict with owner direction. Preserve their historical reasoning, propose an explicit supersession ADR, update project items/roadmap only via verified GitHub APIs, and obtain owner review as required by `AGENTS.md`.

Do not silently overwrite `AGENTS.md`, delete Python history or mark old ADRs accepted under new meanings. After owner review, revise agent guidance additively so future agents cannot resurrect the obsolete stack.

## Required outputs: FIRST DESIGN PR

Create a reviewable design branch/PR under the repository's governance (or present patch files if GitHub project permissions prevent it) containing:

1. `docs/architecture/LearningOS-v2-preliminary.md`: incorporate, review and refine the supplied architecture spec, clarifying fixed choices vs experiments.
2. `docs/architecture/legacy-inventory.md`: paths → keep/adapt/archive/replace, reason, risks, test coverage and data-compatibility consequences. **Not** a mass deletion script.
3. `docs/ui-reference/learnharness-fidelity.md`: reproducible annotated screenshot/source inventory and desktop/mobile acceptance checklist. Record upstream commit and attribution/licensing risks. No invented screenshots.
4. `docs/adr/011-rust-leptos-pwa-pivot.md` (or next available ADR number): propose explicit supersession of old language/framework/runtime decisions, record governance and risk.
5. `docs/adr/012-embedded-storage-experiment.md`: no-PostgreSQL decision, Turso trial vs SQLite-family fallback, browser local store, failure criteria, data migration and sync boundaries.
6. `docs/experiments/first-vertical-slice.md`: detailed replayable fixture, native + `wasm32` build/test commands, PWA offline test protocol, keyboard and visual acceptance, performance capture.
7. GitHub Project 5 proposed replan: slice sized issues with dependencies, acceptance gates and development branches. Use verified project field IDs; never invent project mutations or statuses.

**First PR gate:** docs are internally consistent; previous versus new ADR mapping is explicit; no risky source deletion; review owner remains `@kmosoti`. Document where evidence is missing.

## Implementation objective: FIRST VERTICAL SLICE

After governance gates, implement the smallest **real** product loop in a separate scoped branch/PR, preferably with minimal workspace scaffolding:

```text
Open installed/cached LearningOS
    -> graph/session shell visually recognizably LearnHarness-like
    -> select a seeded learning goal
    -> see a prerequisite plan
    -> predict before explanation
    -> read source-linked explanation
    -> open a post-explanation check
    -> submit answer and optional thinking/confidence
    -> append observation using a stable idempotency key
    -> evaluate deterministically against a fixture rubric
    -> derive a conservative learner-state projection
    -> show one reasoned next action
    -> refresh while offline and resume without data loss
```

**Seed the first example with a Rust ownership/borrowing topic**, plus source IDs and deterministic correct responses. This is deliberately not an LLM integration first. Provide a fake model adapter capable of deterministic scripted output, including errors and out-of-order messages.

### Architecture gate

- `domain` crate contains no framework imports or I/O effects; all time/randomness injected.
- `application` defines typed commands and outbound ports; adapters implement rather than contaminate domain types.
- Rust domain builds for native target and, where intended, `wasm32-unknown-unknown`.
- Browser/server DTOs are versioned. Application decisions are testable without a browser, database or live provider.
- The session event state machine prevents answering a closed question, duplicate conclusion, answer leakage and invalid phase transition.
- All graph arguments preserve AND-hyperedge semantics (no flattening `A ∧ B ⇒ C` to independent edges).

### UI fidelity gate

- Visually coherent graph-first home, left navigation/sidebar, top bar, transcript, plan window, distinct prediction/check cards, learner thinking composer and activity surface.
- Screenshots at 1440×900, 1280×720, 390×844 and 360×740; compare against pinned LearnHarness baseline, noting deliberate differences.
- Keyboard traversal, visible focus, reduced motion, readable contrast, zoom support, screen-reader labeling; Canvas graph provides structured list alternative.
- Question state is a typed application state, not inferred from DOM structure or arbitrary LLM text.

### Offline/persistence gate

- After a successful installation/online visit, disable networking and **hard reload**. Pre-seeded material and interaction controls remain usable.
- An acknowledged response persists after reload and simulated abrupt shutdown. A duplicate submit does not create duplicate events.
- Hidden answer keys remain concealed until permitted by the exercise state machine.
- Cloud research/inference shows a clear unavailable state; no fake “offline AI” claim.
- Storage adapter conformance tests run for every selected backend. If Turso browser storage is immature, ship a Rust-backed IndexedDB persistence adapter and retain the Turso experiment as a separate bounded card.

### Performance gate

- Record cold/warm/offline load profiles, LCP, INP, CLS and memory on a mobile browser plus desktop.
- Benchmark the specific graph renderer and island bundle, not Rust frameworks abstractly; maintain a seeded 1k-node graph fixture now and a 10k stress fixture before declaring scalability.
- Proposed initial targets: LCP p75 ≤2.5s, INP p75 ≤200ms, CLS ≤0.1 and graph interaction p95 frame ≤33ms on target mobile hardware; **targets not achievements**.
- Explain all failed budgets and choose fixes based on trace evidence. Avoid premature WASM-heavy computation on every page.

### Security/reliability gate

- No arbitrary model output may write to the canonical evidence ledger without validated command transitions.
- Rendered source text and models' HTML/code are untrusted; HTML sanitization, CSP and sandboxed demos must be tested before enabling rich artifacts.
- Offline conflicts, duplicate event IDs, browser quota exhaustion, provider outages, cancellation and partial migrations have test cases.
- No secrets in public WASM/JS bundles or logs.

## Experiments and decision criteria (do these rather than arguing indefinitely)

### Experiment A: Islands and PWA lifecycle

Compare a static SSR shell plus independent islands vs a larger coherent session island. Measure compiled WASM size, hydration cost, interaction latency, transient state persistence, a11y and offline reload. Select whichever maintains UX and reliability with less complexity. Do not optimize by island count.

### Experiment B: Embedded database

Use the **same storage-port conformance suite** for native Turso and a stable embedded SQLite implementation. Compare transactionality, WAL/restart behavior, key SQL constructs, concurrency, snapshot/export, migrations and crash recovery. Separately test Turso WASM/OPFS against IndexedDB-backed browser events across browsers, multiple tabs and storage quota limits. Turso is optional; **no PostgreSQL fallback**.

### Experiment C: Graph layout

Use fixed 1k/10k node-and-argument fixtures. Compare a thin SVG/DOM graph for small scale, Canvas2D and optional GPU route only if profiles justify it. Report CPU/frame p50/p95, memory, label visibility and semantic keyboard-list parity.

### Experiment D: Learning engine

Baseline transparent deterministic policies first. Compare proactive probing vs fixed questions, different next-action heuristics, delayed retrieval, and source-checking. Use independent deferred tasks and adversarial input fixtures; separate unsupported LLM assertion rates from learner recall/transfer rates. Do not claim statistical superiority without data.

## Prohibitions

- No PostgreSQL, Neo4j, Kafka, microservice decomposition, React, Next.js, Electron, HTMX or Python production path introduced as a default.
- No massive mechanical Python→Rust port and no reusing LearnHarness agent/store architecture as-is.
- No reflexive long-term scheduler/framework, no assumption that every prerequisite is a strictly probabilistic independence term, and no mastery score disguised as certainty.
- No “brain-state detection,” cognitive diagnosis, neurological claims or personality inference from answer timing/clickstreams.
- No forced user engagement loops or coercive gamification.
- No GitHub issue/project field guesses, rewriting accepted history, unreviewed merges, force pushes, or deleting legacy sources without an approved archive/transition plan.
- No source/asset copying without AGPL compliance and attribution review; do not adopt LearnHarness trademarks or official marks as LearningOS brand assets.
- No fake tool test output, fabricated visual comparisons, or invented performance metrics.

## Reporting format for Codex

At each meaningful checkpoint report:

1. **Verified repository state:** exact branch and inspected files, preserved assets, observed gaps.
2. **Decisions proposed vs accepted:** which owner-fixed choices are implemented and which experiments remain open.
3. **Changed files:** high-level diff, ADR/issue links, no unrelated alterations.
4. **Evidence:** exact commands, tests, screenshots, benchmarks and results, including failures.
5. **Next narrow step:** the smallest dependent slice with a demonstrable acceptance test.

**If all tooling is available, do the design inventory and create the first documentation PR now.** If repository or Project 5 permissions block write operations, still deliver a ready-to-apply patch, issue plan and evidence; state the precise blocker. Do not ask a broad round of design questions or substitute generic architecture advice. Do not merge without `@kmosoti` approval.

## End state

LearningOS v2 must look recognizably like LearnHarness at the UI level, but the observable state, epistemic model, agent boundaries, database choice, offline execution, evaluations, and learning policies must be independently justified and implemented in Rust. If a simpler design wins an experiment, use it. Fidelity and measurable correctness matter more than architectural ornament.

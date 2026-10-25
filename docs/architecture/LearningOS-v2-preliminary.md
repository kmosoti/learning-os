# LearningOS 2.0 — Preliminary Architecture and Product Specification

**Status:** PRELIMINARY / implementation-oriented / architectural pivot  
**Date:** 2026-10-25  
**Canonical repository:** https://github.com/kmosoti/learning-os  
**Presentation reference:** https://github.com/amosblomqvist/learnharness  
**Reference product:** https://learnharness.ai/  
**Audience:** owner, Codex implementation agents, architecture reviewers  
**Authority:** this document records the owner's latest direction. Existing contrary ADRs remain historical records and must be superseded through the repository's governance workflow before dependent implementation merges.

## 0. Executive directive

Rebuild **LearningOS** as a **Rust-first, high-performance, accessible, installable PWA**. Reconstruct **LearnHarness's visual design, navigation, density, and teaching/practice interaction rhythms** with high fidelity, then improve the experience while **completely reengineering its underlying learning logic**. The product is a domain-independent cognitive learning system, not primarily a course-management application, flashcard app, chat wrapper, or fork of LearnHarness.

**Do not translate the old FastAPI app into Rust class-by-class. Do not transplant LearnHarness's Next/Electron and agent internals. Do not introduce PostgreSQL.** Reuse validated *ideas*, research, tests, and durable semantics, not their accidental implementation details.

### Non-negotiable architectural choices

| Concern | Direction | Status |
|---|---|---|
| Canonical target | `kmosoti/learning-os` | **Fixed** |
| Language | Rust for domain, application, server and client application logic | **Fixed** |
| UI | Leptos SSR with **islands-mode selective hydration** | **Fixed** |
| HTTP edge | Axum/Tower with Tokio, modular monolith | **Provisional implementation choice** |
| Product | Responsive PWA, desktop-first density with full mobile usability | **Fixed** |
| UI baseline | High-fidelity LearnHarness reconstruction, then measured enhancements | **Fixed** |
| Database topology | Embedded, SQLite-family; **no PostgreSQL** | **Fixed** |
| Storage implementation | Turso native + browser WASM/OPFS as **experimental candidate**; baseline SQLite-family fallback if a spike fails | **Evaluate** |
| Browser persistence | Verified durable local evidence storage; IndexedDB baseline possible while OPFS spike runs | **Required outcome; mechanism evaluated** |
| Sync | Explicit event replication protocol; no automatic CRDT assumption | **Research / post-initial-vertical-slice** |
| Knowledge | Provenance-aware assertions and multi-premise arguments | **Fixed domain invariant** |
| Learner state | Evidence-derived estimates, not synonymous with concepts taught | **Fixed domain invariant** |
| Agent logic | Model adapters propose; policy engine checks/accepts decisions | **Fixed responsibility split** |
| Observability | Correlated domain event and execution instrumentation | **Fixed** |

**Rust purity boundary:** Rust owns the business logic, reactive UI code, protocol handling, validation, and persistence adapters. Browser hosting requires HTML/CSS, WASM bindings and often generated JavaScript; a tiny service-worker bootstrap or interoperability shim may be unavoidable. Do not silently claim 'zero JS' or introduce a JS application framework. Any hand-authored JS needs a bounded reason, test, and documented exception. `no_std` is **not** automatically required for the browser.

## 1. The product thesis and what survives the prior art

The invariant from the old LearningOS thesis is: **make hidden learning readiness inspectable using evidence, without confusing inferred understanding with observed behavior.** Keep its source-span grounding, distinct domain and learner models, notes-to-claims thinking, competency evidence, prerequisite debt, reversible validation views, readiness policies, and one-next-action control. Keep its independent source/evidence representations; do not collapse them into vectors.

LearnHarness is the **UI and interaction reference**, specifically: graph-first home, persistent sidebar, session transcript, inline AI artifacts, visible plan/DAG window, open predictions versus collapsed comprehension checks, thinking fields, optional confidence, phased probing, progressive hints, practice workspace, and activity/status surfaces. These are interaction patterns; the underlying session records, graphs, tool schemas, prompt strategy, store layout, framework and inference logic are **not** architectural dependencies.

### Intentional departures

1. LearningOS may organize content by **goals, projects, domains, courses or source collections**. 'Course' is optional metadata, not the aggregate root of all knowledge.
2. The worldwide claim graph, session-local prerequisite plan, and learner-state projection are **three different structures**, not one graph with overloaded edge semantics.
3. A statement appearing in a source, an LLM reply, or a learner's answer is not 'verified truth'. Preserve who asserted it, when, context, source span, and whether an independent check exists.
4. An LLM's output is a **proposal** or rubric-evaluation candidate. Domain invariants and accepted evidence/edits are controlled by explicit application workflows.
5. Learning 'mastery' is context-, task-, and time-dependent; expose dimensions and uncertainty rather than treating one score as the human's essence.
6. Do not need Neo4j, a generic graph database, Kubernetes, Kafka, microservices, a heavyweight JS SPA or PostgreSQL to prove this product.

### Prior-art files Codex must read

**LearningOS:** `README.md`, `AGENTS.md`, `docs/00-project-brief.md`, `docs/01-requirements.md`, `docs/05-decision-log.md`, `docs/07-engineering-standards.md`, `docs/adr/010-mvp-runtime-and-serving-stack.md`, `docs/thesis/00-index.md`, `docs/thesis/01-invisible-state-problem.md`, `docs/thesis/02-cognitive-tools-making-invisible-visible.md`, `docs/thesis/03-logical-representation-model.md`, and current `app/`, `tests/` and migrations.  
**LearnHarness:** `README.md`, `docs/architecture.md`, `components/home.tsx`, `components/sidebar.tsx`, `components/graph-view.tsx`, `components/graph-canvas.ts`, `components/chat.tsx`, `components/plan-window.tsx`, `components/question-dock.tsx`, `components/practice-cards.tsx`, `components/reading-thinking.tsx`, `components/activity-dock.tsx`, `app/globals.css`, `docs/assets/teaching-session.png`, `docs/assets/knowledge-graph.png`, plus prober/teacher/practice prompt fragments and `src/agent/subagents/extraction.ts`.

The reference may change. Pin the inspected upstream commit SHA in the UI-reference inventory and record which screenshots correspond to which version.

## 2. UI reconstruction as a first-class specification

**Initial aesthetic acceptance rule:** A user familiar with LearnHarness should recognize the redesigned interface at a glance. The first milestone intentionally favors fidelity over stylistic invention. Afterwards, enhancements must preserve comprehension and information density.

### Major views

| View | Reference behavior to preserve | LearningOS additions |
|---|---|---|
| **Knowledge Home** | Graph-first canvas, pan/zoom, zoom-dependent labels, sidebar and contextual detail | Toggle logical/domain view vs evidence-of-understanding overlay; never conflate topology with mastery |
| **Learning Session** | Streaming conversation, phase indicators, persistent plan viewport, question dock, thinking fields, activity UI | Source links, hypothesis vs observed evidence, pause/resume, explicit offline state |
| **Practice** | Learner-led problems, hidden targets, graded hints and reveal, retest in changed context | Observation/evaluation provenance, delayed transfer attempts and replayable evidence |
| **Source Inspector** | References from lesson without leaving context | Exact source span, versions, conflicts and citation provenance |
| **Next Action** | Small, contextual guidance without navigation overload | One explainable goal-weighted recommendation with override and uncertainty |
| **Settings/Model** | Clear connection and per-session controls | Provider capability, budget ceilings, privacy consent, offline mode; secret-isolated from browser assets |

### Interaction contract for a lesson

1. User states a goal. Prober diagnoses only goal-relevant prerequisite strands; confidence and thinking are separate signals.
2. Agent proposes a **session-local** dependency plan. User sees and can adjust it.
3. Each learning step can begin with an **open prediction** before the explanation.
4. Content renders with readable mathematical text, code and source-linked diagrams. **Checks occur after the teaching**, collapsed until the learner chooses to open them.
5. On answer, persist the verbatim observation *before* showing success. A policy suggests continue, probe, hint or repair based on evidence. Do not equate one answer with permanently mastering a node.
6. A session can stop at any point; refreshing or reopening restores it without replaying submitted turns or leaking concealed answers.
7. Practice must not always announce the target proposition; hint stages are bounded, accessible and logged as context for evaluation.

### Layout details to inventory and reproduce

Codex must capture and annotate actual source screenshots (do not fabricate screenshots or treat image-search substitutes as LearnHarness). Inventory typography, scale, density, spacing, colors, scroll behavior, sidebar state, top bar, transcript gutters, composer, dock overlays, plan window, card reveal transitions, focus order, keyboard shortcuts and graph navigation. Implement CSS tokens and components in Rust/Leptos. Preserve desktop information density while building a distinct mobile responsive layout, not shrinking the desktop view.

**Responsive acceptance devices:** desktop 1440×900, small laptop 1280×720, phone 390×844, compact phone 360×740; keyboard-only and screen-reader passes. Mobile plan/source/activity areas use accessible sheets/drawers; the primary question and answer remain visible and operable. Respect reduced motion, increased text size and browser zoom. Interactive Canvas **must** have a parallel keyboard-accessible graph list/table.

### Islands decomposition (a hypothesis to benchmark)

- SSR by default for shell, route content, notes, static source text and noninteractive graph detail.
- Hydrated islands for graph viewport, composer/question/answer workbench, plan navigator, practice interaction, and contextual live activity.
- Prefer *one coherent session-workspace island* if many tiny islands create shared-state/hydration complexity. Compare route-specific islands against broader hydration with actual bundle/INP tests; do not dogmatically maximize island count.
- Streaming messages use a transport adapter (SSE initially unless bidirectional needs justify WebSockets); only serialize presentation events/DTOs, never domain internals or credentials.
- During offline startup, SSR cannot be assumed available. Serve a cached shell and locally persisted learning data; formally test post-reload island initialization offline.

## 3. Clean hexagonal architecture

```
Browser/PWA                          Native Server
  Leptos islands                       Axum routes / streaming
  browser-store adapter                embedded DB / model adapters
          \                             /
           \ ---- application ports -- /
                    |
             learning-application
                    |
          pure deterministic domain
          (epistemics / competency /
           session / policy engines)
```

**Dependency law:** Domain contains no Axum, Leptos, Tokio, SQL, provider SDK, browser API or filesystem. Application depends inward on domain and its own port contracts. Adapters implement those ports and are wired at composition roots. A UI adapter sends typed commands and displays typed results; it never edits domain-owned tables directly.

### Proposed Rust workspace (target organization, not all crates created on day one)

```text
learning-os/
  Cargo.toml                    # Rust workspace
  rust-toolchain.toml            # pinned AFTER verified toolchain preflight
  apps/
    server/                     # Axum + SSR composition root
    web/                        # Leptos components, islands, PWA shell
  crates/
    domain/                     # typed entities, logic, invariants; no I/O
    application/                # use cases and inbound/outbound ports
    protocol/                   # versioned browser/server DTOs and events
    knowledge/                  # graph and AND-hypergraph algorithms
    learning-policy/            # probe, teach, practice, scheduling policies
    adapters-native/            # embedded DB, provider, source, tracing, jobs
    adapters-web/               # IndexedDB/OPFS, sync client, browser bridges
    evals/                      # experiments, fixtures, policy comparisons
  tests/
    contracts/                  # cross-adapter behavioral conformance
    browser/                    # PWA, accessibility, offline, visual checks
  docs/
    architecture/
    adr/
    ui-reference/
    experiments/
  legacy/python-mvp/            # optional archival MOVE after approved plan
```

**Minimize the initial workspace:** start with `domain`, `application`, `protocol`, `apps/server`, `apps/web`. Introduce additional crates only when a boundary is demonstrably useful. Preserve existing Python code, tests and migrations until preservation strategy is approved; do not delete old history.

### Bounded domains and ownership

- **Epistemic knowledge:** `Source`, `SourceSpan`, `Assertion`, `Proposition`, `Argument`, `Provenance`, `Correction`. Owns claim identity/evidence and logical graph transactions.
- **Learning evidence:** `Observation`, `Attempt`, `RubricEvaluation`, `Revision`, `LearnerProjection`, `Competency`, `MisconceptionHypothesis`. Owns immutable learner observations and derived estimates.
- **Curriculum / goals:** user-selected goals, learning outcomes, prerequisite hypotheses, session-local plans, plans revisions.
- **Teaching/practice:** session event state machine, question state, prediction/check, hints, pause/resume, policy decisions and interactions.
- **Execution:** jobs, provider capabilities, tool permissions, budgets, retries, observability; never silently authoritative about epistemic truth.

### Foundational invariants

1. `source_asserted(P)` is not `verified(P)`. Verification is scoped and revocable, with evaluator/policy version.
2. `presented(P)` is not `understood(P)`; observation is not evaluation; evaluation is not durable learner state.
3. **AND-hyperedges are atomic**: `A ∧ B ⇒ C` is not equivalent to `(A ⇒ C) ∧ (B ⇒ C)`; graph simplification cannot drop necessary joint premises.
4. A plan's prerequisite edge is not automatically a globally entailing epistemic edge.
5. All material state changes originate from typed commands/events with stable IDs, source identity, policy version, timestamp and idempotency key.
6. Event replay must produce the same projection given identical accepted inputs and policy version. Timestamp/randomness is supplied as explicit input, not read from a global clock in pure logic.
7. Retries, reconnections, duplicate device events, and out-of-order delivery never silently create duplicate attempts or contradictory terminal states.
8. LLM responses enter as untrusted candidate data; schemas and deterministic graph rules check shape and consistency, while external factual correctness requires evidence/policies.
9. Stored proof/evaluation objects must bind to the exact immutable claim/attempt and evaluator version. Do not implement empty marker-type “verification.”
10. Users can inspect, correct, retract and export their own learning records; corrections retain relevant audit linkage without implying current validity.

### Representative contract sketch (DESIGN PSEUDOCODE, not compiled Rust)

```rust
// Pure input -> output. Domain does not access persistence or network.
fn advance_session(
    state: &SessionState,
    command: SessionCommand,
    context: &PolicyContext,
) -> Result<SessionTransition, DomainError>;

struct SessionTransition {
    new_state: SessionState,
    events: Vec<DomainEvent>,
    requested_effects: Vec<EffectRequest>,
}

// Application port; actual native/browser async bridging may differ.
trait EvidenceJournal {
    async fn append_once(&self, event: &LearningEvent)
        -> Result<AppendOutcome, JournalError>;
    async fn read_since(&self, cursor: EventCursor)
        -> Result<Vec<LearningEvent>, JournalError>;
}
```

A compilation gate must prove the final trait signatures on native Rust and `wasm32-unknown-unknown`; generated DTO compatibility and API names are never guessed from prose. For server storage, transactions include event write + associated local state + durable outbound-job marker when necessary; a Tokio channel is **not** a durability boundary.

## 4. Knowledge, learner evidence and policies

**Source layer:** source bytes in controlled storage, content hash, immutable spans with page/section location, source version, ingestion status, and attribution. Extraction stores candidate statements with exact span IDs. Conflicts remain representable without forcing a silent 'truth' decision.

**Epistemic graph:** propositions are scoped assertions; `Argument { premises: NonEmptySet<PropositionId>, conclusion, relation, source_evidence, provenance }` is an AND-hyperedge. `supports` is distinct from formal entailment. Do not normalize multi-premise arguments into independent arrows when reasoning; a drawn implication node is just a *visual projection*. Cycles and disagreements can exist in the general knowledge graph. Session prerequisite views may require a DAG projection, with an explicit reason for edge selection/cycle handling.

**Learner evidence:** verbatim response, question/rubric IDs, prompt variation, hint rung, reported confidence, timestamp, optional elapsed time, modality, and evaluation version. Derived capability estimates cover recall, explanation, discrimination, application and delayed retention with 'insufficient evidence' as a first-class state. No permanent personality/intelligence judgments or medical/neurological inference.

**Practice policy:** start with a reproducible deterministic heuristic baseline. Add competing experiments (information-gain probing, spaced reviews, interleaving, worked-example fading, expected readiness gain per cost) only after evaluation fixtures exist. Do not hard-code SM-2/FSRS or probabilistic mastery claims before target-task fit and measurements. User control beats agent optimization when the user opts to change goals or difficulty.

**Agent pipeline:** separate content/reasoning proposal, citation/consistency verification, learning-policy choice, presentation generation, and deterministic persistence. Every LLM role has inputs/outputs, permission set, deadline, token budget, retry cap and failure fallback. Tests use scripted fake models. A misbehaving provider cannot corrupt the local evidence log or silently mark a concept mastered.

## 5. Local data, PWA, offline execution and synchronization

### Capability matrix (first vertical slice)

| Capability | Online | Offline after installation |
|---|---|---|
| View cached app shell and previously downloaded source/lesson | Yes | **Required** |
| Read local graph/proposition projection | Yes | **Required for cached data** |
| Answer a pre-generated deterministic question | Yes | **Required** |
| Save response/hints/evidence across reload | Yes | **Required** |
| Compute local deterministic next-action heuristic | Yes | **Required if required context cached** |
| Generate a novel LLM answer via remote model | Yes | **Unavailable; explain honestly** |
| Web research/new remote source ingestion | Yes | **Unavailable** |
| Sync across devices | Candidate | **Queue events; sync on reconnect in later milestone** |

**Storage decision:** Evaluate native Turso against SQLite via the same adapter conformance suite. Turso claims SQLite-compatibility and browser WASM, but it has incomplete compatibility and pre-1.0 risks (see https://github.com/tursodatabase/turso/blob/main/COMPAT.md). **Do not assert that Turso's browser persistence and sync meet requirements until demonstrated.** Browser baseline may be IndexedDB through a Rust adapter; OPFS/Turso can replace it after cross-browser tests. No PostgreSQL schema, driver, or migration contingency.

**Sync protocol:** Client events have stable event IDs, device IDs, sequence numbers and causal references. Sync uses idempotent append and acknowledgments with cursors; conflicting evaluations and edits are retained as separate revisions for reconciliation. Never conflate server arrival order with learner intent. Deduplication and recovery are tested under duplicate, reordered and interrupted requests. Encryption, privacy and keys are specified before enabling remote storage. No CRDT unless genuine concurrent collaborative editing becomes a documented requirement.

**PWA lifecycle:** manifest + icons, versioned service worker, cache policy, offline fallback, user-safe upgrade path, durable migrations, storage quota warnings, export/backup, and support for clearing private local data. Do not unconditionally force a new service worker to seize active teaching sessions. Browser APIs and browser support must be validated in Chrome/Chromium and Firefox; Safari support becomes a specific measured compatibility gate rather than an unverified blanket claim.

## 6. Performance, robustness and accessibility contracts

These are **design targets, not achieved measurements**. Profile a low-to-midrange Android phone and current desktop browsers, with CPU/network throttling and cold/warm/offline cases. Capture p50/p95 distributions and trace evidence.

| Metric | Preliminary target | Measurement |
|---|---|---|
| PWA home LCP | p75 <= 2.5s on defined mobile network profile | Browser performance trace / Lighthouse corroboration |
| INP after hydration | p75 <= 200ms | Instrumented browser interactions |
| CLS | <= 0.1 | Browser Core Web Vitals |
| Answer durability | 0 acknowledged attempts lost in crash/reload simulation | Kill/restart, offline tests, event IDs |
| Offline resume | Restores submitted turns, hidden answers not leaked | Airplane-mode hard refresh after install |
| Graph 1k nodes | Pan/zoom remains smooth; target p95 frame <= 33ms mobile | Reproducible seeded fixture; frame trace |
| Graph 10k nodes | Zoom-level clustering/virtualized listing; no unbounded labels | Memory and frame-time stress tests |
| Domain application decisions | p95 <= 100ms excluding network/LLM | Criterion or tracing spans with fixed fixtures |
| Accessibility | WCAG 2.2 AA goal | axe + manual keyboard, screen-reader and zoom tests |

Benchmarks record device CPU, memory, browser version, dataset hash, profile, compression mode, cache state and build SHA. Reject “fast because Rust” arguments without measurements. Canvas graphs require a semantic list alternative; focus and question navigation cannot depend on pointer interaction.

## 7. Applied learning science and sociology

Treat claims as **evidence of a possible mechanism**, not proof of product effectiveness. For each policy, preserve the research source, applicable population/task, intervention implementation, confounds, measured outcome and falsification rule.

| Mechanism | Initial policy hypothesis | Testable outcome / boundary |
|---|---|---|
| Retrieval practice | Generate short unprompted recall after teaching | Delayed independent recall, not immediate quiz score |
| Distributed practice | Space reactivation based on observed forgetting | Compare against fixed review schedule and learner self-selection |
| Interleaving | Mix confusable skills after initial scaffolding | Discrimination/transfer vs blocked practice, novice exceptions |
| Worked examples and fading | More guidance early, fade as performance improves | Novel task success, effort, hint dependence |
| Predict–teach–check | Prompt a motivated prediction before the explanation | Distinguish benefit from extra time-on-task |
| Metacognitive calibration | Optional confidence plus delayed outcome comparison | Calibration error / Brier score; no inference from confidence alone |
| Self-determination / agency | Let learner reject or adjust planner recommendations | Completion, autonomy reports and learning outcomes, not addictive streaks |
| Social epistemics | Disclose sources, evaluation authority, contested claims and uncertainty | Correction success, trust calibration and learner control |
| Sleep/stress/neurobiology | Avoid rigid pressure and coercive prompts; support breaks | Do not claim neurological state from clicks or prescribe a universal sleep rule |

Source anchors to verify in an experiment registry: LearningOS's existing thesis and references; Karpicke & Roediger (2008) on retrieval; Dunlosky et al. (2013) on learning strategies; worked-example/expertise reversal reviews; knowledge space theory; self-determination and social learning research. **Scientific literature does not determine exact UI algorithms; experiments do.** No diagnosis of learner mental health, IQ, motivation or neurological state from telemetry.

## 8. Security, reliability, licensing

- Inputs from documents/LLMs/other learners are **untrusted content**, never instructions to the host. Sanitize rendered content; sandbox active demonstrations; test prompt injection and XSS.
- Separate local private records from cloud inference requests. Explicitly show provider, context scope, estimated token cost, opt-in permissions and offline dependency. Never embed long-lived provider credentials in public JS/WASM assets.
- Use operation IDs and schema versions; make migrations reversible where feasible and retain export before irreversible transformations.
- Treat AGPL-3.0 obligations seriously. LearnHarness's code is AGPL-3.0. Reimplementing visual behavior is not equivalent to unrestricted reuse of source/assets; document copied code/asset provenance and licensing, avoid unapproved trademark/branding reuse. LearningOS has AGPL-3.0 prior art.
- Bound retries, resource quotas, output sizes and background-job concurrency. Test server restart, connectivity loss, model outage, graph conflict, stale source version and browser quota exhaustion.
- Instrument domain transitions and effects without indiscriminately logging full answers or sensitive source files.

## 9. Phase plan and acceptance gates

**P0 — Preserve and formally pivot.** Read repository and reference; write an inventory of keep/rewrite/archive; pin reference commit; capture screenshot and interaction reference; propose ADRs superseding old Python/FastAPI/HTMX/Postgres-path decisions; map GitHub Project 5 issues. No destructive deletion. **Gate:** owner-readable decision diff and reproducible UX baseline.

**P1 — Rust/Leptos/PWA proof slice.** New Rust workspace, domain/app boundary tests, Axum + Leptos islands SSR, one visually recognizable teaching session with real keyboard controls, PWA manifest and versioned cache. Deterministic fixture script produces transcript, plan and questions. **Gate:** runs, hydrates, responds on desktop/mobile and shows offline shell after install. No LLM required.

**P2 — Durable offline evidence loop.** Local answer and thinking capture, deterministic event ID/dedupe, persistence through reload/crash, local re-evaluation and next-action heuristic, accessible resumed session. Run browser IndexedDB vs OPFS/Turso spike; document results. **Gate:** source→predict→teach→check→record→resume→next action **works without internet for pre-generated material**.

**P3 — Epistemic graph and evidence.** Source spans, propositions, AND-arguments, provenance drill-down, graph canvas + accessible table, basic conflict display, compare 1k/10k fixtures. **Gate:** logical property tests prevent invalid hyperedge simplification; trace each displayed claim to evidence or label unverified.

**P4 — Real model adapters and adaptive policies.** Provider streaming, tool permissions, source verification, bounded agent jobs, dynamic probe/teach/practice policies, recorded evaluations and calibration experiments. **Gate:** fake-model and adversarial-input suites, budgets and failure recovery; no fabricated mastery claims.

**P5 — Cross-device sync and empirical testing.** Only after offline event format and durability are stable. Compare learning policy variants in delayed, independent transfer evaluations. **Gate:** documented advantages over a simpler baseline or explicit feature rollback.

**Stop / pivot criteria:** if Leptos Islands does not meet interaction and bundle targets, consider changing the *granularity of islands* before abandoning Leptos. If Turso misses durability or compatibility gates, use stable embedded SQLite-family storage behind the same contract, never introduce PostgreSQL. If complex learner models do not outperform transparent heuristics, keep the heuristic. If graph visualization impairs actual study, make it contextual instead of mandatory.

## 10. Required ADRs and unresolved experiments

### Proposed ADRs (do not falsely mark accepted)

- **ADR-011:** LearningOS 2.0 Rust + Axum + Leptos Islands PWA and effective supersession of historical ADR-003, ADR-004, ADR-005 and ADR-010.
- **ADR-012:** Embedded local storage, browser durable storage, Turso-versus-SQLite compatibility experiment, explicit no-PostgreSQL decision.
- **ADR-013:** Typed observations, epistemic authority and multi-premise reasoning model.
- **ADR-014:** Offline capability boundary, event reconciliation and sync ownership.
- **ADR-015:** LearnHarness visual recreation, independent implementation, attribution and accessibility policy.

### High-information research spikes

1. Leptos islands bootstrapping across live SSR, cached offline reload, source-view route and interactive question updates.
2. Turso native transactions/SQLite compatibility versus stable embedded SQLite under real event-log writes; Turso browser WASM/OPFS versus Rust-wrapped IndexedDB with multi-tab and crash tests.
3. Prove domain crate native+WASM compilation and replayable deterministic pure transitions, before designing broader abstraction hierarchies.
4. Canvas/WebGPU/SVG graph fixture compare for 1k and 10k nodes including accessible list. Select by actual UX+resource results.
5. Source-grounded graph extraction: compare an LLM proposal pipeline with deterministic fixture and independent claim verification; measure unsupported claims and over-merges.

## 11. Definition of first meaningful release

A learner can **install** LearningOS, open a visually faithful graph/session interface, select a known goal, receive a pre-generated plan and teaching sequence, make a prediction, read material, answer a check, provide optional thinking, receive a deterministic evaluation with an explicit evidence trail, reload offline, and continue from the correct state with a useful next-action recommendation. The system does not confuse this observed performance with verified subject truth or lifetime mastery. The build is reproducible, tests pass, native/WASM boundaries are validated, and no PostgreSQL or Python runtime is on the application execution path.

## 12. Verified source starting points

- Canonical LearningOS: https://github.com/kmosoti/learning-os
- LearningOS thesis: https://github.com/kmosoti/learning-os/blob/main/docs/thesis/00-index.md
- LearningOS governance: https://github.com/kmosoti/learning-os/blob/main/AGENTS.md
- LearnHarness UI/code: https://github.com/amosblomqvist/learnharness
- LearnHarness architecture: https://github.com/amosblomqvist/learnharness/blob/main/docs/architecture.md
- LearnHarness UI screenshots: https://github.com/amosblomqvist/learnharness/tree/main/docs/assets
- Leptos islands macro: https://docs.rs/leptos/latest/leptos/attr.island.html
- Turso README: https://github.com/tursodatabase/turso
- Turso compatibility: https://github.com/tursodatabase/turso/blob/main/COMPAT.md
- MDN service worker: https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API/Using_Service_Workers
- SQLite WASM and OPFS: https://sqlite.org/wasm/doc/trunk/persistence.md

**No benchmark results, compilation, upstream UI screenshots or multi-device test results are claimed by this draft. Implementation agents must generate evidence for all such claims.**

# LearningOS 2.0 execution contract

Date: 2026-10-25. Design issue: [#49](https://github.com/kmosoti/learning-os/issues/49).

The owner supplied the [preliminary specification](LearningOS-v2-preliminary.md)
and [pivot brief](CODEX_PIVOT_BRIEF.md), then approved the proposed plan with
“Implement the proposed plan.” This authorizes the fixed architecture and the
specified experiments. It does **not** approve any pull request or merge.
Formal ADR records remain proposed for review; uncertain mechanisms are explicitly
deferred to the accepted experiment gates. Do not treat a stale Project field as
evidence that an accepted historical PR is still blocked.

## Authority and boundaries

- Rust owns domain, application, transport and reactive UI logic. Leptos SSR/islands,
  Axum/Tokio and embedded SQLite-family storage are fixed. Axum's provisional label
  in the original preliminary document is resolved by the owner's direct request.
- Personal PWA: one learner; browser evidence stays local. No accounts, remote
  inference, automatic source ingestion or cross-device sync in this release.
- One integrated slice includes UI, accessibility, persistence and offline resume.
  The preliminary document's P1/P2 labels are implementation steps within that slice,
  not permission to ship an online-only scaffold as the first completed milestone.
- Real Android installation/performance is an owner-run release gate, as selected
  during planning. Desktop mobile emulation never satisfies that gate.
- Legacy code, tests, data and thesis remain intact. Use a new v2 database namespace.

## Architecture

Start with `domain`, `application`, `protocol`, `apps/server`, `apps/web`.
Dependency direction: adapters → application → domain. Protocol DTOs are versioned
boundary data, not a backdoor for framework types into the domain. Native/browser
adapters initially live at composition roots. Add crates only for useful boundaries.

The browser runs the application locally and owns its evidence journal. Axum serves
the SSR shell, source views and an immutable lesson bundle from embedded storage.
The same journal contract is exercised against native adapters; there is no implicit
browser/server synchronization or server-arrival ordering assumption.

Pure transitions accept explicit IDs, time and policy versions. Persist a command's
receipt, event batch and session revision atomically. Check idempotency before the
expected-revision check so a lost acknowledgment can be retried safely. Same ID and
different payload is a conflict. Stale revisions preserve the draft and require a
fresh state. Successful UI acknowledgment follows transaction completion.

Separate source assertions, AND-arguments, learner observations, versioned rubric
evaluations and derived estimates. Source assertion is not verified truth; presented
material is not demonstrated capability. Prediction and check observations are
distinct. All joint premises must hold before an argument can fire. A session plan
is a validated DAG projection; the general claim graph need not be acyclic.

## Fixture and behavior

Seed ownership/moves and shared/mutable borrowing from pinned Rust Book material.
Compile examples independently of rubric code: moved `String` reuse yields E0382;
mutation before a shared borrow's last use yields E0502; mutation after that use and
shared borrowing without moving the owner compile. Check diagnostic codes, not
unstable full compiler wording.

Goal → prerequisite plan → open prediction → explanation → closed check → open check
→ answer with optional confidence/reading/answer thinking → committed evidence →
one reasoned next action → offline hard reload and resume.

Use structured choices and “I don't know”; preserve free text without claiming a
semantic evaluation of it. Repair a missed prerequisite, retry without hints,
attempt transfer, then recommend delayed retrieval. A single answer never establishes
durable mastery. Corrections/retractions append linked events and rebuild projections.
Scripted fake models cover errors, malformed proposals, cancellation and reordering;
models cannot directly write journal entries.

## Offline and visibility

SSR emits a neutral boot state. After hydration, load the local journal and render
the actual session. Cache every resource needed by the seeded loop, including fonts
and generated WASM bindings. Direct session navigation and hard reload must work
with networking disabled. Do not require a server function to record an answer.

Do not put withheld explanations/solutions in hidden SSR children or accessibility
content. The offline bundle necessarily contains an inspectable rubric; UI concealment
is not a cryptographic anti-cheating promise. No private keys belong in the bundle.

Use versioned caches and storage; do not force a service worker takeover during an
active session. Fail migrations atomically and explain blocked tabs/quota errors.
Never acknowledge a failed write. Provide export/import and explicit local clearing.
Browser durability remains subject to browser eviction, user clearing and hardware
failure; test process crashes and request persistent storage without claiming immunity.

## Review and delivery

Create scoped design, storage research, hydration research and integrated-slice PRs.
Stack dependent branches when necessary; report base branches explicitly. Keep every
PR unmerged. Since current credentials belong to `kmosoti`, GitHub may reject a
self-review request; record that limitation and name the owner in the PR instead of
claiming a review request succeeded. The owner must separately approve/merge.

No source deletion, force push, old-PR closure, or deployment is authorized by this
contract. The next bounded milestone is one local Markdown source → immutable spans
→ reviewed assertions → a second offline lesson.

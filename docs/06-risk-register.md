# Risk Register

Risks are ranked by MVP impact and likelihood. Review this after each milestone.

| ID | Risk | Impact | Likelihood | Mitigation | Owner |
| --- | --- | --- | --- | --- | --- |
| RISK-001 | Scope expands into automatic science updater before core loop works. | High | High | Keep watcher features in post-MVP; implement only manual evidence notes. | Product/Engineering |
| RISK-002 | LLM extraction produces plausible but unsupported graph facts. | High | High | Require source-span grounding, schema validation, confidence, audit logs, and deterministic EMT golden tests. | Engineering |
| RISK-003 | Medical/protocol content is interpreted as clinical advice. | High | Medium | Add protocol caveats, local-protocol-dependent evidence status, and careful UI language. | Product |
| RISK-004 | Python 3.14 package compatibility causes setup failures. | Medium | Medium | Verify dependencies early with `uv lock` and `uv sync`; isolate vector backend behind adapter. | Engineering |
| RISK-005 | PDF parsing quality is poor for real course materials. | Medium | High | Store raw spans visibly, detect empty/scanned PDFs, add fixture tests, defer OCR. | Engineering |
| RISK-006 | Canonicalization over-merges distinct concepts or medications. | High | Medium | Use deterministic normalization conservatively; preserve exact medication names; require review for high-similarity merges post-MVP. | Engineering |
| RISK-007 | Graph model becomes an unbounded knowledge model. | Medium | Medium | Keep MVP node and edge types constrained; add new types through tests and migrations. | Engineering |
| RISK-008 | Planner gives shallow recommendations. | High | Medium | Start with explicit heuristics: weak prerequisites, due reviews, next outcome. Test each branch. | Product/Engineering |
| RISK-009 | Evidence layer adds complexity without improving MVP. | Medium | Medium | Keep evidence notes manual and concept-level only. No automatic ingest. | Product |
| RISK-010 | UI work consumes time before engine is proven. | Medium | High | Build plain HTMX pages after API slices; avoid complex visualization. | Engineering |
| RISK-011 | Learner mastery scores appear precise but are weak estimates. | Medium | Medium | Show mastery states and trends rather than overclaiming precision; keep algorithm simple and tested. | Product |
| RISK-012 | Source rights/privacy are mishandled. | High | Low | Default source rights to user private copy; store local files; avoid cloud upload except configured LLM calls. | Engineering |
| RISK-013 | Background processing blocks web requests. | Medium | Medium | Add simple job boundary after ingestion works; show statuses and retry paths. | Engineering |
| RISK-014 | Vector search dependency is unstable on Python 3.14. | Medium | Medium | Create adapter and spike sqlite-vec versus LanceDB before relying on either. | Engineering |
| RISK-015 | The EMT fixture corpus is too narrow to validate general extraction. | Medium | Medium | Use it as the first golden baseline, then add more course fixture spans after core loop works. | Product |
| RISK-016 | Assignment outcomes are merged into domain concepts. | Medium | Medium | Keep outcomes first-class; add tests preventing outcome/concept over-merge. | Engineering |
| RISK-017 | Claims become quiz-ready before being grounded. | High | Medium | Enforce claim maturity state transitions and block quiz generation for ungrounded claims unless explicitly marked. | Engineering |
| RISK-018 | Evaluation feedback lacks citations. | Medium | Medium | Require context packer to include source spans; evaluator must cite or state that no source was found. | Engineering |
| RISK-019 | The app has no usable demo until too late. | High | Medium | Follow vertical slices; seed EMT fixture course before general LLM extraction. | Engineering |
| RISK-020 | ty maturity or strictness slows development. | Low | Medium | Keep ty in CI, but configure gradually if third-party typing gaps appear. | Engineering |
| RISK-021 | Core code becomes expensive because retrieval, graph expansion, or LLM calls are unbounded. | High | Medium | Require complexity/cost budgets, limits, and contract tests for ingestion, retrieval, grounding, quiz generation, and planner code. | Engineering |
| RISK-022 | Architecture standards become ceremony instead of improving code. | Medium | Medium | Apply standards most strictly to MVP core workflows; keep small CRUD/UI cards lightweight. | Engineering |

## Watchlist Decisions

These decisions should be made before their cards are pulled into Ready:

| Decision | Needed Before | Default |
| --- | --- | --- |
| First LLM provider and local config format. | KAN-040, KAN-060 | Provider-abstracted adapter with env-based config. |
| Vector backend. | KAN-064 | Try sqlite-vec first; fall back to LanceDB if install or feature gaps appear. |
| HTMX versus React for MVP. | KAN-110 | HTMX. |
| Whether outcome graph is separate tables or typed domain nodes plus mapping table. | KAN-015 | Separate Outcome table plus mapping table. |
| Background job mechanism. | KAN-151 | Simple async task boundary first; APScheduler only when scheduled review needs it. |

## Medical Safety Notes

The MVP must treat EMS and medication content as study support for the learner's course materials. It must not provide operational clinical direction.

Recommended guardrails:

- Display source provenance.
- Mark medications and protocols as local protocol dependent where appropriate.
- Prefer "your course source says" over universal clinical language.
- Ask the learner to follow local protocols and instructor guidance for real practice.
- Keep evidence updates manual in MVP.

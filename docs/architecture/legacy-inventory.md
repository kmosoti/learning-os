# Legacy inventory

Inspected `main` at `59a53e4289b6c37ed4f475dfc081fc6e1e390e2d` on 2026-10-25.
Fresh checkout was clean. `git log -5 --oneline`, `git ls-files`, source/tests and
Project 5 were inspected before editing. This is an inventory, not a deletion plan.

| Paths / prior art | Disposition | Evidence and consequence |
|---|---|---|
| `docs/thesis/` | Keep | Three draft chapters, research note and references separate observed evidence from product hypotheses; no empirical product superiority demonstrated. |
| `docs/07-engineering-standards.md` | Keep principles; adapt examples | Typed contracts, effects at edges, bounded cost and deterministic tests survive. Python imports and linear mastery are not the v2 model. |
| `docs/05-decision-log.md`, `docs/adr/010-*` | Preserve with explicit supersession links | ADR-003/004/005/010 conflict with the mandate. ADR-002/007/008/009 remain useful. ADR-001/006 need goal/fixture scope amendments. |
| `app/core/` and helper tests | Preserve; independently reimplement useful contracts | IDs, explicit time, error envelopes, request correlation and transition tests exist. State enums do not constitute an implemented learner engine. |
| `app/api/`, health/error/tracing tests | Preserve; replace v2 execution path | FastAPI composition root, health and structured error handling are implemented. Port intent, not Python routes/classes. |
| `app/web/`, `tests/test_course_web.py` | Preserve; replace v2 interface | Course list/create/detail and placeholders exist. No working quiz/graph/planner implementation to port. |
| `app/db/`, Alembic and DB tests | Preserve; new v2 database | One course-table migration, SQLAlchemy session factory, no persisted source/evidence schema. Do not assume all user databases are empty. No in-place conversion. |
| `pyproject.toml`, `uv.lock`, `.python-version` | Keep as legacy reproducibility | Python 3.14.5, uv/Ruff/ty/pytest remain independently runnable; no Python production dependency in v2. |
| Static roadmap/requirements | Preserve historical context; link v2 contract | Planned EMT ingestion, retrieval, notes, mastery and planner mostly exceed as-built functionality. |
| `.github/`, `AGENTS.md`, contribution rules | Keep; add v2 guidance | Issue branches, field discovery, review, evidence and no-agent-merge requirements still apply. No CI workflow exists at inspected main. |

## Useful behavior to carry forward

Source-span grounding, separate domain/learner/evidence representations, typed errors,
request IDs, bounded traversal, fixture baselines and one next action. The old
`UNKNOWN → EXPOSED → … → DURABLE` ladder is not a scientific model or an inference
license; v2 keeps observations and estimates separate and supports retraction.

## Compatibility

Legacy Python routes and SQLite files remain available through the legacy command.
The Rust app uses a separate database and origin-local browser namespace. No course
record is silently transformed into evidence. A later import must inventory the
actual source DB, back it up, preserve IDs/provenance, and be independently reviewed.

PR #46 (cognitive-ledger thesis) is open, not merged. Its useful ideas are considered
alongside the already-merged thesis; do not modify or close that branch automatically.
Issue #45 is assigned to another contributor. The v2 PR supplies its own checks and
references that issue without taking over its assignee or claiming it completed.

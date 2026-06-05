# Decision Log

This file records initial architecture decisions. Use it as lightweight ADR history.

## ADR-001 - MVP Scope Is Course-Grounded Study, Not Auto-Updating Science

Status: Accepted

Decision:

The MVP will focus on uploaded course sources, source-grounded graph construction, note maturation, quizzes, learner mastery, and next-action planning. Automatic contemporary evidence monitoring is deferred.

Rationale:

The core product risk is whether source graph + learner graph + note maturation + planner creates a useful learning loop. Building literature monitoring first would expand the reliability, medical safety, and data integration surface before proving the engine.

Consequences:

- Evidence layer exists in MVP, but only with manual evidence notes and caveats.
- PubMed/OpenAlex/guideline watcher stays in the post-MVP parking lot.

## ADR-002 - Keep Graph Layers Separate

Status: Accepted

Decision:

Maintain distinct Source, Domain, Outcome, Learner, Evidence, and Planner layers.

Rationale:

Each layer answers a different question:

- What did the source say?
- What does the domain mean?
- What does the course require?
- What does the learner believe or know?
- What is verified, current, or caveated?
- What should happen next?

Consequences:

- Vector search is used for retrieval, not as the system of record.
- Every generated learning action should trace back to sources, learner state, outcomes, or evidence notes.

## ADR-003 - Use Python 3.14.5 And Astral Tooling

Status: Accepted

Decision:

Use Python 3.14.5, uv, Ruff, and ty.

Rationale:

The user explicitly selected Astral's stack. Current official documentation verifies Python 3.14.5 as the latest Python 3 release on the source downloads page, uv's Python management and universal lockfile support, Ruff's `py314` target, and ty's type-checking role.

Consequences:

- `.python-version` pins `3.14.5`.
- `pyproject.toml` uses `requires-python = ">=3.14,<3.15"`.
- CI should run Ruff, ty, and pytest.
- Dependency compatibility with Python 3.14 should be checked early, especially for vector packages.

## ADR-004 - Use FastAPI, SQLite, SQLAlchemy, Alembic, FTS5, NetworkX

Status: Accepted

Decision:

Use FastAPI for API, SQLite for MVP persistence, SQLAlchemy 2.x for ORM, Alembic for migrations, FTS5 for lexical search, and NetworkX for graph traversal.

Rationale:

This keeps the MVP local-first and understandable while still supporting the core graph and retrieval loop.

Consequences:

- No external database is required for local demo.
- Graph traversal can be rebuilt from database rows.
- Neo4j remains post-MVP unless SQLite/NetworkX becomes a proven blocker.

## ADR-005 - Prefer HTMX For MVP UI

Status: Proposed

Decision:

Start with FastAPI + Jinja + HTMX for MVP screens.

Rationale:

The MVP is workflow-heavy and data-grounding-heavy, not interaction-heavy. Server-rendered pages reduce frontend complexity and keep implementation close to backend state transitions.

Consequences:

- UI cards should remain plain and operational.
- Rich graph visualization is deferred.
- If React is chosen later, preserve API contracts first.

## ADR-006 - Manual EMT Fixture Extraction Comes Before LLM Extraction

Status: Accepted

Decision:

Build deterministic extraction for the EMT fixture corpus before general LLM extraction.

Rationale:

The fixture corpus provides known expected outputs for assignment questions, terminology, medication facts, and common medications. This creates golden tests and a way to evaluate whether LLM extraction is doing useful work.

Consequences:

- Early demo can work even if LLM extraction is not final.
- General extraction must be compared against fixture expectations.

## ADR-007 - Evidence Layer Is Manual In MVP

Status: Accepted

Decision:

Support manual evidence notes, source precedence, outdatedness flags, and concept-level caveats. Do not build automatic literature monitoring in the MVP.

Rationale:

Evidence freshness matters, especially in EMS/medical topics, but automatic monitoring introduces scope and safety risk. Manual caveats are enough to keep the architecture honest.

Consequences:

- EMS medication/protocol guidance can be marked `LOCAL_PROTOCOL_DEPENDENT`.
- The app should avoid acting like a medical authority.

## ADR-008 - One Next Action, Not A Menu

Status: Accepted

Decision:

The planner should return exactly one next useful action.

Rationale:

The product should reduce learner decision load. Planner quality is a core differentiator.

Consequences:

- Planner API returns a single `NextAction`.
- UI can include skip and stuck controls, but not a large recommendation list.

## ADR-009 - Use Deterministic Engineering Standards For Core Code

Status: Accepted

Decision:

Core implementation work must follow a deterministic standard covering black-box boundaries, typed contracts, protocols, functional-core/imperative-shell separation, validated state machines, idempotency, result/evidence objects, error taxonomy, observability, and complexity/cost bounds.

Rationale:

The project is intended to be reliable and educational. The code should make design tradeoffs visible, especially around time complexity, space complexity, LLM/token cost, database query behavior, and failure recovery.

Consequences:

- MVP core cards must describe input/output contracts and complexity/cost expectations.
- External dependencies should sit behind protocols and adapters.
- Pure logic should be testable without network, filesystem, database, or LLM calls.
- Generated learning content must preserve source provenance.
- Review should use the engineering rubric in `docs/07-engineering-standards.md`.

## ADR-010 - MVP Runtime And Serving Stack

Status: Accepted

Decision:

Use a modular monolith with Nginx as the deployed edge, Uvicorn as the ASGI runtime, FastAPI as the HTTP adapter, explicit application use cases, domain logic isolated from operational frameworks, and repositories/unit of work behind application ports.

Local MVP development runs Uvicorn directly through uv. Gunicorn is deferred until deployment evidence justifies it. SQLite remains the local-first MVP persistence choice from ADR-004, but the architecture must keep a clear PostgreSQL/pgvector path through ports and adapters.

Rationale:

The stack should prove the learning loop quickly while teaching reliable backend boundaries: thin routes, use-case transaction scripts, domain-owned rules, protocols for external effects, idempotent jobs, structured errors, and bounded operational cost.

Consequences:

- Bootstrap can start with `uv run uvicorn app.main:app --reload`.
- Nginx deployment configuration is not required for the first local slice.
- Domain code must not import FastAPI, SQLAlchemy sessions, source parsers, LLM SDKs, or deployment tooling.
- A later deployment/persistence ADR can decide PostgreSQL, pgvector, systemd, SOPS/age, and Salt details.

Full ADR: [ADR-010: MVP Runtime And Serving Stack](adr/010-mvp-runtime-and-serving-stack.md).

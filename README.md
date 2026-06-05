# learning-os

Planning package for the MVP of a course-aware learning OS.

The MVP goal is intentionally narrow:

> Ingest course files, build source-grounded course graphs, capture notes, mature notes into grounded claims, quiz the learner, track mastery gaps, and choose the next useful study action.

The project should not start as an automatic contemporary science updater. The architecture keeps an evidence layer so that capability can be added later without mixing course content, learner beliefs, and current evidence into one untraceable index.

## Documentation Map

- [Project Brief](docs/00-project-brief.md)
- [Requirements](docs/01-requirements.md)
- [Architecture And Mermaid Diagrams](docs/02-architecture-mermaid.md)
- [Kanban Board Static Snapshot](docs/03-kanban-board.md)
- [Roadmap And Milestones Static Snapshot](docs/04-roadmap-and-milestones.md)
- [Decision Log](docs/05-decision-log.md)
- [Risk Register](docs/06-risk-register.md)
- [Engineering Standards](docs/07-engineering-standards.md)
- [GitHub Bootstrap](docs/08-github-bootstrap.md)
- [Lightweight Requirements Discovery](docs/09-lightweight-requirements-discovery.md)
- [GitHub Workflow](docs/github-workflow.md)
- [Agent GitHub Operating Guide](docs/agent-github-operating-guide.md)

Operational planning now lives in GitHub Project 5: https://github.com/users/kmosoti/projects/5

## MVP Demo Target

Upload EMT pre-course materials, process them into source spans and graph nodes, help the learner answer assignment questions, ground class notes, generate retrieval-practice quizzes, update learner mastery, and recommend one next study action.

## Tooling Baseline

- Python: 3.14.5
- Project manager: uv
- Lockfile: uv.lock
- Formatter/linter: Ruff
- Type checker: ty
- Backend: FastAPI
- Persistence: SQLite, SQLAlchemy 2.x, Alembic, FTS5
- Graph: NetworkX for MVP traversal
- UI: HTMX server-rendered frontend unless a later decision explicitly changes it

Verified on 2026-06-04:

- Python.org lists Python 3.14.5 as the latest Python 3 release on the source downloads page.
- uv supports Python version management and universal lockfiles.
- Ruff supports `py314` as a target version.
- ty is Astral's Python type checker and language server.

## First Implementation Slice

Build a local app that can:

1. Start with `uv run`.
2. Create one course.
3. Upload one PDF or text source.
4. Parse source spans.
5. Persist those spans.
6. Show the source and span count in a plain UI.

That creates the first observable thread through the architecture before adding LLM extraction or quiz behavior.

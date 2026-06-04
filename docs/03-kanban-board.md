# Kanban Board

This is a static bootstrap snapshot. The operational kanban board, workflow state, completion gates, agent handoff, and active roadmap tracking live in GitHub Project 5:

https://github.com/users/kmosoti/projects/5

This board is the working development backlog for the MVP. It is organized as a kanban workflow plus a card deck grouped by epic.

## Workflow Columns

| Column | Meaning | Entry Rule | Exit Rule |
| --- | --- | --- | --- |
| Backlog | Valid work not yet ready to pull. | Card has title and outcome. | Dependency and acceptance criteria are clear. |
| Ready | Pullable work. | Card has size, dependencies, and acceptance criteria. | Developer starts work. |
| In Progress | Actively being built. | WIP slot is available. | Code/docs are ready for review. |
| Review | Implemented and waiting for verification. | Tests/docs were run or reason is documented. | Acceptance criteria pass. |
| Done | Complete. | User-facing or developer-facing outcome exists. | No known required follow-up remains for that card. |
| Blocked | Cannot proceed without a decision or external input. | Blocker is explicit. | Blocker resolved or card is resized. |

## WIP Limits

- In Progress: 2 cards max.
- Review: 3 cards max.
- Blocked: review every planning session.
- A card larger than 2 focused days should be split.

## Definition Of Ready

A card is ready when:

- The outcome is testable or inspectable.
- Dependencies are named.
- Acceptance criteria are concrete.
- It does not hide unrelated architecture decisions.
- It can be completed without building a post-MVP feature.

## Definition Of Done

A card is done when:

- Code, docs, migrations, or tests are committed in the expected place.
- Relevant checks pass locally.
- Any skipped check is documented with the reason.
- User-facing behavior is visible through API, CLI, UI, or docs.
- Source-grounded behavior keeps provenance.
- MVP core work satisfies the applicable checks in [Engineering Standards](07-engineering-standards.md), especially contracts, state, idempotency, complexity, and bounded external cost.

## Current Board Snapshot

### Ready

- KAN-001 - Initialize uv project skeleton.
- KAN-002 - Add Python 3.14.5 project configuration.
- KAN-003 - Add Ruff, ty, and pytest baseline checks.
- KAN-004 - Create FastAPI application shell.
- KAN-005 - Add core config, logging, IDs, and time helpers.
- KAN-006 - Define invariant state enums.
- KAN-010 - Add SQLAlchemy base and session.
- KAN-011 - Add Alembic migration setup.
- KAN-012 - Implement Course model and migration.
- KAN-020 - Add health check and typed API error envelope.
- KAN-160 - Adopt engineering standards as Definition of Done.
- KAN-161 - Add service contract template.

### Backlog

All remaining MVP cards below.

### Blocked / Needs Decision

- KAN-064 - Choose and implement vector backend: sqlite-vec preferred, LanceDB fallback.
- KAN-060 - Choose first LLM provider and local configuration contract.
- KAN-110 - Confirm HTMX as MVP UI and add base Jinja layout.

### Explicitly Later

- POST-001 - OCR for scanned PDFs.
- POST-002 - Automatic contemporary evidence watcher.
- POST-003 - Neo4j graph database.
- POST-004 - Complex visual graph editor.
- POST-005 - Multi-user classroom mode.
- POST-006 - Mobile app.

## Vertical Release Slices

| Slice | Demo Outcome | Includes |
| --- | --- | --- |
| VS-0 | Repo boots and checks run. | uv, Python pin, Ruff, ty, pytest, FastAPI health. |
| VS-1 | User creates a course. | Course model, migration, API, basic UI list. |
| VS-2 | User uploads and processes a source. | Source model, upload, PDF/TXT loaders, chunker, spans. |
| VS-3 | EMT fixtures create a trustworthy baseline graph. | Deterministic extractor, domain nodes, outcomes, golden tests. |
| VS-4 | User can search course sources and graph context. | FTS5, vector adapter, graph retrieval, context packer. |
| VS-5 | Notes mature into grounded claims. | Note capture, claim extraction, concept linking, grounding. |
| VS-6 | Quiz answer updates mastery. | Quiz generation, evaluation, mastery updates, misconceptions. |
| VS-7 | Planner selects one next action. | Weak prerequisite heuristic, due review, reason display. |
| VS-8 | EMT demo works end to end. | Minimal UI, seed command, evidence caveats, smoke tests. |

## Card Deck

### Epic A - Project Foundation

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-001 | Initialize uv project skeleton. | S | None | `pyproject.toml`, package directory, tests directory, README, and `uv.lock` exist. |
| KAN-002 | Pin Python 3.14.5. | XS | KAN-001 | `.python-version` contains `3.14.5`; `requires-python` is `>=3.14,<3.15`. |
| KAN-003 | Configure Ruff, ty, and pytest. | S | KAN-001 | `uv run ruff check .`, `uv run ruff format --check .`, `uvx ty check`, and `uv run pytest` are documented. |
| KAN-004 | Add FastAPI application shell. | S | KAN-001 | `app/main.py` exposes `app`; local uvicorn command boots. |
| KAN-005 | Add core config, logging, errors, IDs, and time helpers. | M | KAN-004 | Pydantic settings load; stable ID and UTC timestamp helpers are covered by tests. |
| KAN-006 | Define invariant state enums. | S | KAN-005 | Source status, claim maturity, mastery state, and evidence status enums exist and are tested. |
| KAN-007 | Add developer scripts documentation. | XS | KAN-003 | README includes install, run, lint, type-check, test, and reset commands. |
| KAN-008 | Add CI workflow. | S | KAN-003 | CI runs uv sync, Ruff, ty, and pytest on pull requests. |
| KAN-009 | Add project license and contribution notes. | XS | KAN-001 | License and contributor basics exist without overbuilding process. |

### Epic B - Database And Persistence

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-010 | Add SQLAlchemy base and session factory. | S | KAN-005 | App can open and close SQLite sessions in tests. |
| KAN-011 | Add Alembic migration setup. | S | KAN-010 | `alembic upgrade head` creates an empty database successfully. |
| KAN-012 | Implement Course model and migration. | S | KAN-011 | Course table has id, title, description, created_at; migration test passes. |
| KAN-013 | Implement Source and SourceSpan models. | M | KAN-012 | Source and spans persist with checksums and processing status. |
| KAN-014 | Implement DomainNode, DomainEdge, and DomainAlias models. | M | KAN-013 | Nodes, edges, aliases, confidence, and source span provenance persist. |
| KAN-015 | Implement Outcome and outcome-concept models. | M | KAN-014 | Assignment questions can map to related concepts. |
| KAN-016 | Implement Note and NoteClaim models. | M | KAN-015 | Notes and claims persist with learner, course, status, and linked concept. |
| KAN-017 | Implement QuizItem, QuizAnswer, and evaluation models. | M | KAN-016 | Quiz answer can reference generated item and evaluation payload. |
| KAN-018 | Implement LearnerMastery and LearnerMisconception models. | M | KAN-017 | Mastery and misconception rows can be inserted and queried by learner/course. |
| KAN-019 | Implement EvidenceNote model. | S | KAN-014 | Evidence notes link to concepts with status and source label. |

### Epic C - API Foundation

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-020 | Add health check and typed API error envelope. | S | KAN-004 | `/health` returns version/status; errors have consistent JSON shape. |
| KAN-021 | Add course create/list/detail routes. | M | KAN-012, KAN-020 | `POST /courses`, `GET /courses`, and `GET /courses/{id}` work with tests. |
| KAN-022 | Add source upload route. | M | KAN-013, KAN-021 | `POST /courses/{id}/sources/upload` stores file and Source row. |
| KAN-023 | Add source process route. | S | KAN-022 | `POST /sources/{source_id}/process` triggers ingestion service. |
| KAN-024 | Add source spans route. | S | KAN-023 | `GET /sources/{source_id}/spans` returns persisted spans. |
| KAN-025 | Add notes create route. | S | KAN-016, KAN-021 | `POST /courses/{id}/notes` stores raw note text. |
| KAN-026 | Add note digest and claims routes. | M | KAN-025 | `POST /notes/{id}/digest` and `GET /notes/{id}/claims` expose claim lifecycle. |
| KAN-027 | Add quiz generation and answer routes. | M | KAN-017, KAN-026 | Quiz can be generated and answered through API. |
| KAN-028 | Add next action route. | M | KAN-018, KAN-027 | `GET /courses/{id}/next-action` returns one NextAction. |
| KAN-029 | Add concepts, neighbors, and learner-state routes. | M | KAN-014, KAN-018 | Map page can query concepts and learner status. |

### Epic D - Source Ingestion

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-030 | Implement loader contract. | S | KAN-006 | `DocumentLoader`, `LoadedDocument`, and `LoadedPage` exist with tests. |
| KAN-031 | Implement PDF loader with PyMuPDF. | M | KAN-030 | Text PDFs produce pages with page numbers; empty PDF is detected. |
| KAN-032 | Implement TXT and Markdown loader. | S | KAN-030 | Plain text and Markdown create a single loaded document with title. |
| KAN-033 | Implement DOCX loader. | M | KAN-030 | Paragraph text is extracted into section-aware loaded pages where practical. |
| KAN-034 | Implement EPUB loader. | M | KAN-030 | HTML content is extracted with BeautifulSoup and section titles when available. |
| KAN-035 | Implement source resolver and storage service. | M | KAN-022 | Uploaded files are stored by course/source ID with checksum. |
| KAN-036 | Implement simple chunker. | M | KAN-031, KAN-032 | Chunks respect page boundaries and max character budget. |
| KAN-037 | Implement ingestion pipeline orchestration. | M | KAN-035, KAN-036 | Pipeline updates status from uploaded to ready or failed. |
| KAN-038 | Add unsupported OCR state handling. | S | KAN-031, KAN-037 | Scanned or empty PDFs get clear failure reason. |
| KAN-039 | Add ingestion integration test. | M | KAN-037 | Upload/process fixture source creates SourceSpan rows. |

### Epic E - Extraction And Canonicalization

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-040 | Add LLM adapter interface. | M | KAN-005 | Provider-abstracted client supports structured calls and test doubles. |
| KAN-041 | Add extraction Pydantic schemas. | S | KAN-040 | ExtractedNode, ExtractedEdge, ExtractedClaim, and SourceSpanExtraction validate. |
| KAN-042 | Write extraction prompt contracts. | M | KAN-041 | Prompt says extract only supported facts and reduce confidence when uncertain. |
| KAN-043 | Build deterministic EMT terminology extractor. | M | KAN-014 | Roots, prefixes, suffixes, and abbreviations become typed nodes/aliases. |
| KAN-044 | Build deterministic EMT medication chart extractor. | L | KAN-014 | Medication rows create meds, indications, contraindications, doses, and source links. |
| KAN-045 | Build deterministic EMT assignment extractor. | M | KAN-015 | 22 assignment questions become outcome cards. |
| KAN-046 | Implement general source span extraction service. | L | KAN-041, KAN-042 | LLM extraction creates validated graph candidates from spans. |
| KAN-047 | Implement title normalizer and alias service. | M | KAN-014 | Exact normalized match merges; aliases map alternate names to canonical nodes. |
| KAN-048 | Implement graph writer. | M | KAN-046, KAN-047 | Nodes and edges are upserted with confidence and source provenance. |
| KAN-049 | Add extraction audit table or log files. | M | KAN-046 | Raw structured extraction inputs/outputs can be inspected in dev. |
| KAN-050 | Add golden extraction tests. | M | KAN-043, KAN-044, KAN-045 | Oxygen chart row, terminology row, and assignment examples match expected output. |

### Epic F - Search And Retrieval

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-060 | Define LLM and embedding provider config contract. | S | KAN-040 | Env vars and local fallback behavior are documented. |
| KAN-061 | Add SQLite FTS5 setup for source spans. | M | KAN-013 | Spans are indexed for lexical search. |
| KAN-062 | Implement lexical search service. | M | KAN-061 | Query returns span IDs, snippets, and linked concept IDs where available. |
| KAN-063 | Add vector backend adapter interface. | M | KAN-060 | Adapter supports index, search, delete by source/span ID. |
| KAN-064 | Implement selected vector backend. | L | KAN-063 | Basic vector search returns similar source spans behind adapter. |
| KAN-065 | Implement NetworkX graph traversal service. | M | KAN-014 | Service expands neighbors and prerequisite-like relationships by course. |
| KAN-066 | Implement hybrid retrieval contract. | L | KAN-062, KAN-064, KAN-065 | RetrievalRequest returns source spans, concepts, outcomes, and learner flags. |
| KAN-067 | Implement source-grounded context packer. | M | KAN-066 | Tutor services receive bounded context with source span IDs. |
| KAN-068 | Add retrieval integration tests. | M | KAN-066 | Lexical plus graph expansion returns expected EMT concepts and outcomes. |

### Epic G - Notes, Grounding, And Claim Maturity

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-080 | Implement note repository and service. | S | KAN-016 | Notes can be created and listed by course/learner. |
| KAN-081 | Implement note claim extraction. | M | KAN-040, KAN-080 | Raw note text produces atomic NoteClaim rows. |
| KAN-082 | Implement claim-to-concept linking. | M | KAN-047, KAN-081 | Claims link to canonical concept IDs or remain unlinked with reason. |
| KAN-083 | Implement claim grounding service. | L | KAN-067, KAN-082 | Claims become grounded, partially grounded, or conflicting with source span references. |
| KAN-084 | Implement claim maturity transition validation. | M | KAN-006, KAN-083 | Invalid claim status transitions fail tests. |
| KAN-085 | Implement quiz-ready claim selection. | M | KAN-083 | Grounded and partially grounded claims can be selected for quiz generation. |
| KAN-086 | Add note lifecycle integration test. | L | KAN-081, KAN-083 | Note to claim to grounded status works against fixture spans. |

### Epic H - Quiz, Evaluation, And Learner Graph

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-090 | Implement quiz item schema and service. | M | KAN-067, KAN-085 | Quiz items include question, expected answer criteria, concepts, and source spans. |
| KAN-091 | Implement assignment outcome cards. | M | KAN-045, KAN-090 | EMT assignment page can request teach, outline, review, and quiz modes. |
| KAN-092 | Implement answer evaluation schema. | S | KAN-090 | AnswerEvaluation validates score, concepts, misconceptions, and deltas. |
| KAN-093 | Implement answer evaluator service. | L | KAN-040, KAN-092 | Submitted answer produces structured evaluation and source-grounded feedback. |
| KAN-094 | Implement mastery update function. | M | KAN-018, KAN-093 | Mastery is bounded 0 to 1 and hint penalty is tested. |
| KAN-095 | Implement misconception tracking. | M | KAN-018, KAN-093 | Active misconceptions are created from evaluation outputs. |
| KAN-096 | Implement spaced review scheduler. | M | KAN-094 | next_review_at is set after quiz answers and can be queried. |
| KAN-097 | Add quiz-to-mastery integration test. | L | KAN-093, KAN-096 | Answering a quiz updates mastery, attempts, misconceptions, and review schedule. |

### Epic I - Planner

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-100 | Implement NextAction schema. | XS | KAN-028 | action_type, title, reason, IDs, and estimated minutes validate. |
| KAN-101 | Implement weak prerequisite heuristic. | M | KAN-065, KAN-094 | Planner can identify a weak concept that blocks multiple outcomes. |
| KAN-102 | Implement due review heuristic. | M | KAN-096, KAN-101 | Planner selects due review when no high-leverage weak prerequisite exists. |
| KAN-103 | Implement course outcome fallback. | M | KAN-091, KAN-102 | Planner returns next course outcome when no remediation or review is due. |
| KAN-104 | Implement skip and stuck action tracking. | M | KAN-100 | Planner can adapt after skip/stuck without returning a menu. |
| KAN-105 | Add planner tests. | M | KAN-101, KAN-103 | Tests cover weak prerequisite, due review, fallback, and empty-course behavior. |

### Epic J - Minimal HTMX UI

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-110 | Add base Jinja layout and navigation. | M | KAN-004 | Layout has course navigation and shared page shell. |
| KAN-111 | Build courses list and create form. | M | KAN-021, KAN-110 | User can create and open a course from browser. |
| KAN-112 | Build course dashboard. | M | KAN-111 | Dashboard shows counts for sources, notes, outcomes, and mastery. |
| KAN-113 | Build source upload and status page. | M | KAN-024, KAN-112 | User uploads source and sees processing status/spans. |
| KAN-114 | Build notes page. | M | KAN-026, KAN-112 | User adds note and sees parsed/grounded claims. |
| KAN-115 | Build study page. | M | KAN-028, KAN-112 | Page shows today's next action, reason, start, skip, and stuck buttons. |
| KAN-116 | Build quiz page. | M | KAN-027, KAN-115 | User answers quiz and sees evaluation. |
| KAN-117 | Build assignment page. | M | KAN-091, KAN-112 | Outcome cards show related concepts and study actions. |
| KAN-118 | Build basic map page. | M | KAN-029, KAN-112 | Concept neighbors render as simple linked list or lightweight graph. |
| KAN-119 | Add browser smoke test checklist. | S | KAN-111 | Manual or automated smoke checklist covers core pages. |

### Epic K - Evidence Layer

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-120 | Implement evidence note service. | S | KAN-019 | Evidence notes can be added to concepts through service/API. |
| KAN-121 | Add evidence API routes. | M | KAN-120 | Create/list evidence notes by concept. |
| KAN-122 | Add manual evidence UI. | M | KAN-121, KAN-118 | User can add evidence note and see caveat on concept. |
| KAN-123 | Add EMS local protocol caveat behavior. | M | KAN-122, KAN-090 | Medication/protocol quiz contexts include local protocol dependent caveat. |
| KAN-124 | Add evidence status tests. | S | KAN-120 | Invalid statuses fail; concept caveats are retrievable. |

### Epic L - CLI, Fixtures, And Demo

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-130 | Add Typer CLI entrypoint. | S | KAN-005 | `uv run learning-os --help` works. |
| KAN-131 | Add reset local database command. | S | KAN-011, KAN-130 | Local DB reset is explicit and protected. |
| KAN-132 | Add seed EMT course command. | M | KAN-045, KAN-130 | Demo course is created with source fixtures and outcomes. |
| KAN-133 | Add process-source CLI command. | M | KAN-037, KAN-130 | Source ID can be processed from CLI. |
| KAN-134 | Add next-action CLI command. | S | KAN-105, KAN-130 | CLI prints planner action for course/learner. |
| KAN-135 | Create EMT demo script. | M | KAN-132, KAN-134 | Script documents end-to-end demo path. |
| KAN-136 | Add MVP smoke test suite. | L | KAN-119, KAN-135 | One test path covers course, source, note, quiz, mastery, planner. |
| KAN-137 | Update docs after demo. | S | KAN-136 | README includes final local demo instructions and known limitations. |

### Epic M - Hardening And Release

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-150 | Add structured app logging. | M | KAN-005 | Ingestion, extraction, grounding, and evaluation emit useful structured logs. |
| KAN-151 | Add background job boundary. | M | KAN-037 | Long-running processing does not block request path. |
| KAN-152 | Add upload validation limits. | S | KAN-022 | Unsupported extensions and oversized files return clear errors. |
| KAN-153 | Add source rights status display. | S | KAN-013, KAN-113 | Sources show default user private copy rights status. |
| KAN-154 | Add data export for course. | M | KAN-112 | Course export includes sources metadata, graph, notes, mastery, evidence, not private raw file copies by default. |
| KAN-155 | Add backup and restore notes. | S | KAN-154 | Local backup path is documented. |
| KAN-156 | Add release checklist. | S | KAN-136 | MVP release checklist references all completion criteria. |

### Epic N - Engineering Standards And Deterministic Review

| ID | Card | Size | Depends | Acceptance |
| --- | --- | --- | --- | --- |
| KAN-160 | Adopt engineering standards as Definition of Done. | S | None | Definition of Done links to engineering standards and PR checklist. |
| KAN-161 | Add service contract template. | S | KAN-160 | Template captures input, output, mutation, idempotency, failure modes, complexity, external cost, and provenance. |
| KAN-162 | Add architecture boundary rules. | M | KAN-160 | Documentation states dependency direction and banned imports for domain/service layers. |
| KAN-163 | Add complexity budget notes to MVP core cards. | M | KAN-161 | Ingestion, retrieval, grounding, quiz, and planner cards include time/space/external-cost bounds. |
| KAN-164 | Add state transition validator pattern. | M | KAN-006, KAN-160 | Source and claim state transitions are validated in one place and tested. |
| KAN-165 | Add result object standard for workflows. | M | KAN-160 | Ingestion, grounding, evaluation, and planner workflows return structured result/evidence objects. |
| KAN-166 | Add adapter contract test strategy. | M | KAN-040, KAN-063, KAN-160 | LLM, vector, loader, and repository adapters have contract-test expectations. |
| KAN-167 | Add observability field standard. | S | KAN-150, KAN-160 | Structured logs define request IDs, operation status, duration, counts, and warning codes. |
| KAN-168 | Add idempotency review for ingestion and indexing. | M | KAN-037, KAN-061, KAN-064 | Reprocessing the same source does not duplicate spans, nodes, aliases, or indexes. |
| KAN-169 | Add deterministic review score rubric. | S | KAN-160 | PR/review rubric scores boundary clarity, domain modeling, side effects, state, idempotency, failure design, complexity, observability, tests, and groundedness. |

## Post-MVP Parking Lot

| ID | Feature | Reason Deferred |
| --- | --- | --- |
| POST-001 | OCR for scanned PDFs. | Adds a separate quality and layout problem; MVP should detect unsupported scans. |
| POST-002 | Automatic PubMed/OpenAlex/guideline watcher. | Would balloon scope and medical reliability requirements. |
| POST-003 | Neo4j. | SQLite plus NetworkX is enough for local MVP. |
| POST-004 | Complex React graph visualization. | The system needs trustworthy data before rich visualization. |
| POST-005 | Multi-user classroom support. | Adds auth, roles, privacy, and collaboration complexity. |
| POST-006 | Mobile app. | MVP needs engine proof before mobile shell. |
| POST-007 | Real-time lecture transcription. | Separate capture and diarization problem. |
| POST-008 | Autonomous web research. | Conflicts with MVP source-grounded constraint. |

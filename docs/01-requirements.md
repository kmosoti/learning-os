# Requirements

Audience discovery notes live in [Lightweight Requirements Discovery](09-lightweight-requirements-discovery.md). Treat that document as directional product evidence, not as a replacement for the functional requirements below.

## Requirement Status Legend

- `MVP`: required for the first usable demo.
- `MVP-Core`: required before higher-level behavior can be trusted.
- `Post-MVP`: intentionally deferred.
- `Explicitly Out`: should not be built in the MVP.

## Functional Requirements

### Courses

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-COURSE-001 | MVP | Create a course with title and optional description. | Course persists with stable ID and created timestamp. |
| FR-COURSE-002 | MVP | List existing courses. | Courses page and API return created courses. |
| FR-COURSE-003 | MVP | View a course dashboard. | Dashboard shows sources, notes, outcomes, mastery summary, and next action placeholder. |
| FR-COURSE-004 | Post-MVP | Archive or duplicate courses. | Deferred until course lifecycle is better understood. |

### Sources And Spans

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-SOURCE-001 | MVP | Upload PDF, TXT, Markdown, DOCX, and EPUB files. | File upload validates extension, stores file, computes checksum, creates Source row. |
| FR-SOURCE-002 | MVP-Core | Track source processing status. | Status moves through UPLOADED, IDENTIFIED, PARSED, CHUNKED, STRUCTURED, GRAPHED, INDEXED, READY, or FAILED. |
| FR-SOURCE-003 | MVP | Parse source files into page or section-aware loaded pages. | SourceSpan rows are created with text, source ID, page range, section title, and checksum. |
| FR-SOURCE-004 | MVP | Detect scanned or empty PDFs as unsupported. | Source is marked FAILED or OCR_UNSUPPORTED with clear user-facing reason. |
| FR-SOURCE-005 | MVP | Preserve source provenance for every span. | Every extracted node, edge, claim, or outcome can point back to a SourceSpan when available. |
| FR-SOURCE-006 | Explicitly Out | Ingest DRM-protected textbooks. | Not supported in MVP. |
| FR-SOURCE-007 | Explicitly Out | OCR scanned PDFs. | Detected and deferred. |

### Extraction

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-EXTRACT-001 | MVP-Core | Extract only facts supported by source spans. | Prompt and tests reject unsupported outside knowledge. |
| FR-EXTRACT-002 | MVP | Extract concepts, definitions, abbreviations, medications, indications, contraindications, doses, assignment questions, learning outcomes, claims, and relationships. | Structured extraction schema validates all outputs. |
| FR-EXTRACT-003 | MVP-Core | Build deterministic EMT fixture extraction before general LLM extraction. | Golden fixtures exist for medication rows, assignment questions, and terminology examples. |
| FR-EXTRACT-004 | MVP | Use Pydantic schemas for extraction results. | Invalid extraction payloads fail validation and are logged. |
| FR-EXTRACT-005 | MVP | Store extraction confidence. | Edges and claims include confidence in the database. |
| FR-EXTRACT-006 | MVP | Keep prompt contracts in versioned Markdown files. | Prompt files live under `app/llm/prompts/` and are referenced by tests. |

### Canonicalization

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-CANON-001 | MVP-Core | Normalize titles deterministically. | Case, whitespace, and hyphen variations merge consistently. |
| FR-CANON-002 | MVP | Support aliases for abbreviations and alternate names. | Alias table maps alias text to canonical domain node. |
| FR-CANON-003 | MVP | Preserve exact medication names as canonical nodes. | Medication extraction does not over-merge different medications. |
| FR-CANON-004 | MVP | Keep assignment questions separate from concepts. | Outcome nodes are not merged into domain concepts. |
| FR-CANON-005 | Post-MVP | Add candidate vector merge workflow. | Deferred until deterministic baseline is working. |

### Graphs

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-GRAPH-001 | MVP-Core | Persist domain nodes and edges. | DomainNode and DomainEdge tables have migrations and CRUD tests. |
| FR-GRAPH-002 | MVP | Persist source-to-domain provenance. | Edges and claims can link to source spans. |
| FR-GRAPH-003 | MVP | Persist outcome graph items. | Assignment questions and learning outcomes have first-class rows or typed domain nodes. |
| FR-GRAPH-004 | MVP | Build NetworkX graph views from database rows. | Traversal service can expand neighbors and prerequisites. |
| FR-GRAPH-005 | Post-MVP | Move graph storage to Neo4j. | Explicitly deferred. |

### Retrieval

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-RET-001 | MVP-Core | Support lexical search over source spans. | SQLite FTS5 returns matching span IDs for exact terms and abbreviations. |
| FR-RET-002 | MVP | Support basic vector search behind an adapter. | sqlite-vec or LanceDB implementation can be swapped without changing caller contracts. |
| FR-RET-003 | MVP | Support graph expansion from matched concepts. | Retrieval includes related concepts and outcomes. |
| FR-RET-004 | MVP | Include learner state in retrieval context. | Weak concepts and misconceptions are included when relevant. |
| FR-RET-005 | MVP-Core | Keep source spans in retrieved context. | Generated explanations and quiz items can cite source spans. |

### Notes And Claim Maturation

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-NOTE-001 | MVP | Capture raw class notes. | Note row persists learner ID, course ID, raw text, context type, and timestamp. |
| FR-NOTE-002 | MVP | Digest notes into atomic candidate claims. | NoteClaim rows are created from raw note text. |
| FR-NOTE-003 | MVP | Link claims to concepts. | Claims can reference linked concept IDs. |
| FR-NOTE-004 | MVP-Core | Ground claims against source spans. | Claim epistemic status changes to GROUNDED, PARTIALLY_GROUNDED, CONFLICTING, or clarification-needed state. |
| FR-NOTE-005 | MVP | Generate quiz-ready claims. | Grounded claims can produce quiz items. |
| FR-NOTE-006 | MVP | Show raw, parsed, grounded, conflicting, and quiz-ready claims in UI. | Notes page exposes claim lifecycle clearly. |

### Quiz And Evaluation

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-QUIZ-001 | MVP | Generate quiz items from source spans, outcomes, and grounded claims. | Quiz item includes question, answer criteria, linked concepts, and source span references. |
| FR-QUIZ-002 | MVP | Let user submit an answer. | Answer row persists raw answer, timestamps, quiz item ID, and learner ID. |
| FR-QUIZ-003 | MVP-Core | Evaluate answers with structured output. | AnswerEvaluation includes score, correct concepts, missing concepts, misconceptions, next action, and mastery deltas. |
| FR-QUIZ-004 | MVP | Avoid clinical advice language. | EMS medication/protocol content is framed as course/protocol-dependent, not medical direction. |
| FR-QUIZ-005 | MVP | Provide one tiny follow-up when learner is stuck. | Evaluator can produce a constrained remediation prompt. |

### Learner Graph

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-LEARNER-001 | MVP-Core | Track mastery per learner and concept. | LearnerMastery row stores mastery, confidence, state, attempts, correct attempts, last seen, and next review. |
| FR-LEARNER-002 | MVP | Bound mastery updates from 0 to 1. | Unit tests cover low, high, and hint-penalized updates. |
| FR-LEARNER-003 | MVP | Track active misconceptions. | LearnerMisconception rows are created from evaluations and can be marked inactive later. |
| FR-LEARNER-004 | MVP | Support mastery states UNKNOWN, EXPOSED, RECOGNIZED, RECALLED, EXPLAINED, APPLIED, and DURABLE. | State transitions are deterministic and tested. |

### Planner

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-PLAN-001 | MVP-Core | Return exactly one next useful action. | API response includes action type, title, reason, optional concept/outcome ID, and estimated minutes. |
| FR-PLAN-002 | MVP | Prioritize high-leverage weak prerequisites. | Planner selects weak prerequisite when it blocks multiple outcomes. |
| FR-PLAN-003 | MVP | Select due spaced-review items when no urgent weak prerequisite exists. | Scheduler integration test covers due review fallback. |
| FR-PLAN-004 | MVP | Fall back to next course outcome. | Planner never returns an empty action for an active course. |
| FR-PLAN-005 | MVP | Explain why the task matters. | Study page displays the planner reason. |

### Evidence Layer

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-EVID-001 | MVP | Add manual evidence notes to concepts. | EvidenceNote row persists status, summary, source label, optional URL, and timestamp. |
| FR-EVID-002 | MVP | Support evidence statuses. | Status enum includes SUPPORTED, SUPPORTED_BUT_OVERSIMPLIFIED, REFINED_BY_CURRENT_EVIDENCE, CONTEXT_DEPENDENT, CONFLICTS_WITH_CURRENT_EVIDENCE, LOCAL_PROTOCOL_DEPENDENT, and SUPERSEDED. |
| FR-EVID-003 | MVP | Mark EMS protocol and medication guidance as local protocol dependent where appropriate. | UI and quiz contexts show caveat flags. |
| FR-EVID-004 | Explicitly Out | Automatically monitor PubMed, OpenAlex, or guidelines. | Deferred to post-MVP. |

### UI

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-UI-001 | MVP | Use a minimal server-rendered UI unless explicitly changed. | HTMX or plain Jinja pages exist for core workflows. |
| FR-UI-002 | MVP | Provide course list and course detail pages. | `/courses` and `/courses/{id}` are usable. |
| FR-UI-003 | MVP | Provide source upload and source processing pages. | `/courses/{id}/sources` supports upload and shows statuses. |
| FR-UI-004 | MVP | Provide notes page. | `/courses/{id}/notes` shows note and claim lifecycle. |
| FR-UI-005 | MVP | Provide study page. | `/courses/{id}/study` shows today's next action, reason, start, skip, and stuck controls. |
| FR-UI-006 | MVP | Provide quiz page. | User can answer and see evaluation feedback. |
| FR-UI-007 | MVP | Provide assignment page. | 22 EMT assignment questions are shown as outcome cards once seeded. |
| FR-UI-008 | MVP | Provide map page. | Basic concept neighbor view is enough for MVP. |

### API

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-API-001 | MVP | Course routes: `POST /courses`, `GET /courses`, `GET /courses/{course_id}`. | Routes return typed Pydantic responses. |
| FR-API-002 | MVP | Source routes: upload, process, spans. | Upload and processing are testable by integration tests. |
| FR-API-003 | MVP | Note routes: create, digest, claims. | Routes cover raw note to claims. |
| FR-API-004 | MVP | Study routes: next action, quiz generation, answer submission. | Study loop can be driven by API alone. |
| FR-API-005 | MVP | Graph routes: concepts, neighbors, learner state. | Map page can call graph API. |

### CLI

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| FR-CLI-001 | MVP | Use Typer for local developer commands. | CLI entrypoint is callable with `uv run learning-os`. |
| FR-CLI-002 | MVP | Seed EMT fixtures. | `uv run learning-os seed-emt` creates demo course and fixture graph. |
| FR-CLI-003 | MVP | Process one source by ID. | `uv run learning-os process-source SOURCE_ID` runs pipeline. |
| FR-CLI-004 | MVP | Reset local database. | Reset command is clearly local-only and protected from accidental production use. |

## Non-Functional Requirements

| ID | Status | Requirement | Acceptance |
| --- | --- | --- | --- |
| NFR-001 | MVP-Core | Local-first development. | App runs with SQLite and local file storage. |
| NFR-002 | MVP-Core | Source groundedness. | Generated learning content references source spans or explicitly says no source was found. |
| NFR-003 | MVP-Core | Auditability. | Extraction, grounding, and evaluation outputs are stored or reproducible from stored inputs. |
| NFR-004 | MVP | Deterministic state transitions. | Status changes are validated and tested. |
| NFR-005 | MVP | Simple operations. | One command starts the dev server after `uv sync`. |
| NFR-006 | MVP | Testable slices. | Each vertical slice has unit or integration coverage. |
| NFR-007 | MVP | Medical safety language. | EMS content is course/protocol study support, not clinical recommendation. |
| NFR-008 | MVP | Scope control. | Out-of-scope items are tracked as post-MVP, not hidden inside MVP cards. |
| NFR-009 | MVP-Core | Explicit engineering contracts. | Core services document typed inputs, outputs, mutation behavior, idempotency, failure modes, and complexity/cost bounds. |
| NFR-010 | MVP-Core | Ports-and-adapters dependency direction. | Domain and service logic depend on protocols or domain models, not concrete API/UI/database/LLM implementations. |
| NFR-011 | MVP-Core | Bounded complexity and cost. | File processing, retrieval, graph expansion, LLM context, and background work have explicit limits and tests where practical. |

## Core State Machines

### Source Processing Status

- UPLOADED
- IDENTIFIED
- PARSED
- CHUNKED
- STRUCTURED
- GRAPHED
- INDEXED
- READY
- FAILED

### Claim Maturity Status

- RAW_NOTE
- PARSED_CLAIM
- LINKED_TO_CONCEPT
- GROUNDED
- PARTIALLY_GROUNDED
- CONFLICTING
- QUIZ_READY
- MATURED

### Learner Mastery State

- UNKNOWN
- EXPOSED
- RECOGNIZED
- RECALLED
- EXPLAINED
- APPLIED
- DURABLE

## Initial Success Metrics

- First course can be created in under 2 minutes from a fresh local setup.
- One uploaded text or PDF source produces source spans and searchable FTS entries.
- EMT fixture seed creates assignment outcome cards and medication concept cards.
- A raw note can become at least one grounded claim.
- A quiz answer updates at least one learner mastery row.
- Planner returns one next action with a source-grounded or learner-state-grounded reason.

## Requirement To Backlog Traceability

| Requirement Area | Backlog Coverage |
| --- | --- |
| Courses | KAN-012, KAN-021, KAN-111, KAN-112 |
| Sources and spans | KAN-013, KAN-022, KAN-023, KAN-024, KAN-030 through KAN-039, KAN-113 |
| Extraction | KAN-040 through KAN-050 |
| Canonicalization | KAN-047, KAN-048, KAN-050 |
| Graphs | KAN-014, KAN-015, KAN-048, KAN-065, KAN-118 |
| Retrieval | KAN-060 through KAN-068 |
| Notes and claim maturation | KAN-016, KAN-025, KAN-026, KAN-080 through KAN-086, KAN-114 |
| Quiz and evaluation | KAN-017, KAN-027, KAN-090 through KAN-097, KAN-116, KAN-117 |
| Learner graph | KAN-018, KAN-094 through KAN-097 |
| Planner | KAN-028, KAN-100 through KAN-105, KAN-115, KAN-134 |
| Evidence layer | KAN-019, KAN-120 through KAN-124 |
| UI | KAN-110 through KAN-119 |
| API | KAN-020 through KAN-029 |
| CLI | KAN-130 through KAN-134 |
| Fixtures and demo | KAN-132, KAN-135 through KAN-137 |
| Hardening and release | KAN-008, KAN-150 through KAN-156 |
| Engineering standards | KAN-160 through KAN-169 |
| Explicitly deferred scope | POST-001 through POST-008 |

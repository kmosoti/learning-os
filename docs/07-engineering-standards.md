# Engineering Standards

This standard translates the attached architecture/paradigm notes into deterministic rules for `learning-os`.

The purpose is not to maximize ceremony. The purpose is to make the code easy to reason about, hard to misuse, inexpensive to run, and observable when it fails.

## Core Rule

Every meaningful component must answer these questions before or during implementation:

1. What domain concept does this model?
2. What are the explicit inputs and outputs?
3. What state can it be in?
4. What invariants must always hold?
5. What side effects can it perform?
6. Is it a command, a query, or both?
7. Is it idempotent?
8. What can fail?
9. What evidence proves it worked?
10. What is the expected time, space, and external-cost behavior?
11. Can the core logic be tested without a database, filesystem, network, or LLM call?
12. Can the external dependency be swapped through a protocol?

If those answers are unclear, the code is not ready to become infrastructure for the learning loop.

## MUST / SHOULD / AVOID

### MUST

- Use typed domain models for core data.
- Use explicit contracts at module boundaries.
- Use protocols for swappable external dependencies.
- Keep side effects at the edges.
- Validate state transitions.
- Preserve source provenance for generated learning content.
- Bound time, space, token, and external-call costs.
- Test pure logic separately from adapters.
- Emit structured result/evidence objects for important workflows.
- Classify errors with meaningful domain exceptions or result statuses.

### SHOULD

- Prefer composition over inheritance.
- Keep application services thin and use-case oriented.
- Use command/query separation.
- Keep configuration as typed data.
- Make idempotency explicit for ingestion, indexing, graph writes, and scheduling.
- Add contract tests around LLM outputs and external adapter assumptions.
- Add property-style tests for normalizers, chunkers, planners, and state transitions when useful.

### AVOID

- Raw dictionaries flowing through core logic.
- Generic `utils.py`, `manager.py`, or `processor.py` dumping grounds.
- Hidden network, filesystem, database, clock, randomness, or LLM calls inside pure logic.
- Boolean success values for multi-step workflows.
- Unbounded retries, unbounded chunk growth, unbounded graph expansion, or unbounded prompt context.
- Mutation hidden inside functions named like queries.
- Treating vector search as the source of truth.

## Boundary Architecture

Use a ports-and-adapters shape.

```text
app/
  domain/
    Pure models, enums, invariants, value objects.

  services/
    Application use cases: ingest source, digest note, ground claim, evaluate answer.

  ports/
    Protocols that describe external capabilities.

  adapters/
    Concrete implementations: SQLAlchemy, PyMuPDF, LLM provider, vector backend.

  api/
    FastAPI routes. Request/response handling only.

  web/
    Jinja/HTMX views. Presentation only.
```

The current proposed repo structure uses `ingestion/`, `graph/`, `retrieval/`, `tutor/`, and `llm/`. That is fine, but dependency direction still matters:

```text
API/UI -> application service -> domain model/protocol -> adapter
```

Core domain code must not import FastAPI, SQLAlchemy sessions, PyMuPDF, HTTP clients, or concrete LLM provider SDKs.

## Black-Box Component Standard

Every major service should be usable as a black box.

Required shape:

```python
class ClaimGrounder:
    def ground(self, request: GroundingRequest) -> GroundingResult:
        ...
```

The caller should not know:

- Which retriever was used.
- Which prompt was used.
- Which LLM provider was used.
- How many source spans were searched.
- How grounding evidence was stored.

The caller should know:

- The request schema.
- The result schema.
- Possible failure modes.
- Whether the operation mutates state.
- Whether it is idempotent.
- Complexity and cost expectations.

## Protocol Rules

Use `typing.Protocol` when the implementation may change or when tests need fakes.

Good candidates:

- `DocumentLoader`
- `SourceRepository`
- `GraphRepository`
- `LexicalSearchIndex`
- `VectorIndex`
- `EmbeddingProvider`
- `LlmClient`
- `Clock`
- `IdFactory`
- `AuditWriter`

Example:

```python
from typing import Protocol

class VectorIndex(Protocol):
    def index_span(self, span_id: str, text: str) -> None:
        ...

    def search(self, query: str, limit: int) -> list[str]:
        ...
```

Do not create protocols for every tiny object. Create them where replacement, testing, or boundary clarity matters.

## Functional Core, Imperative Shell

Pure logic should live in the functional core:

- Title normalization.
- Chunk boundary decisions.
- State transition validation.
- Mastery update calculation.
- Planner ranking heuristics.
- Graph traversal ranking.
- Evidence status rules.

Side effects belong in the imperative shell:

- File upload and storage.
- PDF/DOCX/EPUB parsing.
- Database writes.
- LLM calls.
- Embedding calls.
- FTS/vector index writes.
- HTTP requests.
- Clock reads, ID generation, and randomness.

If a function is hard to test without mocks, check whether it mixes core logic and side effects.

## Contracts

Every public service method needs an implicit or explicit contract.

Minimum contract fields:

| Field | Meaning |
| --- | --- |
| Input type | Pydantic model or dataclass accepted by the method. |
| Output type | Pydantic model, dataclass, or result object returned by the method. |
| Mutation | What state, if any, it changes. |
| Idempotency | What happens if it runs twice. |
| Failure modes | Domain errors, result statuses, or exceptions. |
| Complexity | Expected time and space behavior. |
| External cost | LLM calls, embedding calls, database queries, file reads, network calls. |
| Provenance | Source span, prompt version, or evidence reference retained. |

## Result And Evidence Objects

Important workflows should return structured result objects, not booleans.

Examples:

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class IngestionResult:
    source_id: str
    status: str
    spans_created: int
    nodes_created: int
    edges_created: int
    warnings: tuple[str, ...]
```

```python
@dataclass(frozen=True)
class GroundingResult:
    claim_id: str
    status: str
    supporting_span_ids: tuple[str, ...]
    conflicting_span_ids: tuple[str, ...]
    confidence: float
    rationale: str
```

This makes workflows auditable and easier to test.

## Error Taxonomy

Do not raise raw `Exception` from application code.

Initial taxonomy:

```text
LearningOsError
  ConfigurationError
  ValidationError
  NotFoundError
  StateTransitionError
  IngestionError
    UnsupportedSourceTypeError
    SourceParseError
    OcrUnsupportedError
  ExtractionError
    LlmContractError
    UnsupportedExtractionError
  GroundingError
  RetrievalError
  EvaluationError
  PlannerError
```

Use exceptions for unexpected or unrecoverable failures. Use result statuses for expected partial outcomes such as unsupported OCR, no grounding found, or skipped optional evidence.

## State Machine Rules

State transitions must be explicit and tested.

Source processing:

```text
UPLOADED -> IDENTIFIED -> PARSED -> CHUNKED -> STRUCTURED -> GRAPHED -> INDEXED -> READY
```

Claim maturity:

```text
RAW_NOTE -> PARSED_CLAIM -> LINKED_TO_CONCEPT -> GROUNDED/PARTIALLY_GROUNDED/CONFLICTING -> QUIZ_READY -> MATURED
```

Learner mastery:

```text
UNKNOWN -> EXPOSED -> RECOGNIZED -> RECALLED -> EXPLAINED -> APPLIED -> DURABLE
```

Rules:

- Invalid transitions fail in one place.
- Failure states store a reason.
- Retrying must be idempotent or explicitly reconciled.
- UI should display state, not infer it from missing data.

## Command And Query Separation

Queries read. Commands mutate.

Good:

```python
spans = source_queries.list_spans(source_id)
result = source_commands.process_source(source_id)
```

Avoid:

```python
get_or_process_source(source_id)
check_and_fix_claim(claim_id)
```

Mutation should be visible in the method name and route.

## Idempotency Standard

These operations must be idempotent:

- Processing the same source by checksum.
- Chunking a source into spans.
- Writing FTS/vector indexes for a span.
- Upserting canonical domain nodes and aliases.
- Seeding the EMT fixture course.
- Scheduling next review after a recorded answer.

Idempotency checklist:

- Does the operation have a natural key?
- Can duplicate work be detected?
- What happens if the process crashes halfway?
- Can the operation resume?
- Are partial writes visible and repairable?

## Complexity And Cost Standard

Every nontrivial component should document expected complexity in the implementation card or docstring.

Use this format:

```text
Time: O(...)
Space: O(...)
External cost: ...
Bound: ...
```

Examples:

| Component | Expected Complexity | Required Bound |
| --- | --- | --- |
| Simple chunker | Time O(n), space O(k) where n is characters and k is chunks. | Max chars per chunk; max file size. |
| Title normalizer | Time O(n), space O(n). | Input title length limit. |
| Exact alias lookup | Average O(1) with indexed normalized alias. | Unique normalized alias per course/type where needed. |
| FTS search | Database indexed lookup. | `limit` required; snippets bounded. |
| Vector search | Backend indexed lookup. | `limit` required; embedding model configured. |
| Graph expansion | O(V + E) within bounded traversal depth. | Depth and node limit required. |
| Hybrid retrieval | Sum of lexical + vector + graph expansion. | Independent limits for each retrieval mode. |
| Note claim extraction | O(tokens) prompt cost. | Max note length and max claims. |
| Claim grounding | O(c * r) where c is claims and r is retrieved spans, plus LLM cost. | Max claims per call and max spans per claim. |
| Quiz generation | O(tokens) prompt cost. | Max context spans and max quiz items. |
| Planner | O(m + e + o) over mastery rows, graph edges, outcomes. | Course-level query limits and graph depth. |

### Complexity Gate

Before merging a card that processes lists, graphs, files, source spans, notes, or LLM context, answer:

```text
1. What input size drives runtime?
2. What input size drives memory?
3. What database indexes are required?
4. What limits prevent runaway work?
5. What happens at the limit?
6. Is the expensive work batched, cached, or resumable?
7. How many LLM/embedding calls can one user action trigger?
```

If those answers are missing, the card is incomplete.

## Token And LLM Cost Controls

LLM calls are part of system complexity.

Rules:

- Prompt context must be bounded by source span count and character/token limits.
- Extraction runs span-by-span or in bounded batches.
- Grounding should retrieve first, then call the LLM on selected context.
- Store prompt version, input span IDs, output schema version, and model metadata when practical.
- Prefer deterministic fixture extraction where source shape is known.
- Never use an LLM to do a simple deterministic transform like normalizing titles.

## Database And Index Rules

Required indexes should follow access patterns.

Initial index expectations:

- `sources.course_id`
- `sources.checksum`
- `source_spans.source_id`
- `domain_nodes.course_id`
- `domain_nodes.course_id, node_type, normalized_title`
- `domain_aliases.course_id, normalized_alias`
- `domain_edges.course_id, from_node_id`
- `domain_edges.course_id, to_node_id`
- `notes.course_id, learner_id`
- `note_claims.note_id`
- `learner_mastery.learner_id, concept_id`
- `learner_mastery.learner_id, next_review_at`
- `outcomes.course_id`
- FTS5 virtual index for source span text.

Each migration that adds a query path should add or justify indexes.

## Observability Standard

Important workflows should emit structured logs with correlation IDs.

Minimum fields:

- `request_id`
- `course_id`
- `learner_id` when applicable
- `source_id` when applicable
- `note_id` or `claim_id` when applicable
- `operation`
- `status`
- `duration_ms`
- `counts`
- `warning_codes`

Never log raw secrets. Be careful with raw note text and uploaded source text; prefer IDs and summaries.

## Test Strategy By Component

| Component | Required Tests |
| --- | --- |
| Domain models and enums | Unit tests for invariants and transitions. |
| Normalizer/canonicalizer | Unit and property-style tests. |
| Chunker | Unit tests for boundaries, empty text, long pages. |
| Loaders | Fixture tests for supported file types. |
| Repositories | Integration tests against SQLite. |
| LLM schemas | Contract tests with valid and invalid payloads. |
| LLM prompts | Golden tests for fixture spans where possible. |
| Graph traversal | Unit tests with small in-memory graphs. |
| Retrieval | Integration tests with fixture spans and graph nodes. |
| Note pipeline | Integration test from note to grounded claim. |
| Evaluator | Contract tests for structured answer evaluation. |
| Planner | Unit tests for each heuristic branch. |
| UI | Smoke tests for core routes. |

## Review Rubric

Score each meaningful card from 0 to 2 in each category.

| Category | 0 | 1 | 2 |
| --- | --- | --- | --- |
| Boundary clarity | Hidden dependencies and vague inputs. | Some typed inputs/outputs. | Explicit contract and black-box behavior. |
| Domain modeling | Raw dicts/strings. | Some typed models. | Domain types enforce invariants. |
| Side effects | Mixed throughout logic. | Mostly isolated. | Pure core, side effects at adapters. |
| State handling | Implicit or inferred. | Some statuses. | Validated state machine. |
| Idempotency | Unknown. | Partially addressed. | Natural keys/retry behavior clear. |
| Failure design | Raw exceptions/booleans. | Basic errors. | Taxonomy or result statuses with evidence. |
| Complexity | Not discussed. | Big-O or limits mentioned. | Time/space/cost bounds enforced or tested. |
| Observability | Minimal logs. | Some useful logs. | Structured evidence and correlation IDs. |
| Testability | Requires real dependencies. | Mock-heavy tests. | Pure logic tested plus adapter contracts. |
| Source groundedness | Missing provenance. | Partial provenance. | Source spans/evidence retained end to end. |

Suggested gate:

- MVP core cards should average at least 1.5.
- Any category scored 0 on source groundedness, state handling, or complexity must be fixed before Done.

## Pull Request Checklist

Use this checklist for every nontrivial implementation PR:

```text
[ ] Inputs and outputs are typed.
[ ] Side effects are isolated or explicitly named.
[ ] Protocols are used for swappable dependencies.
[ ] State transitions are validated.
[ ] Operation idempotency is documented.
[ ] Failure modes use domain errors or structured result statuses.
[ ] Time complexity is understood.
[ ] Space complexity is understood.
[ ] LLM, embedding, database, and file I/O costs are bounded.
[ ] Source provenance is preserved where content is generated.
[ ] Tests cover pure logic and at least one relevant boundary.
[ ] Logs/results provide enough evidence to debug failure.
[ ] The PR links its issue and uses that issue's development branch.
[ ] @kmosoti is requested as reviewer and approval owner.
[ ] Human-readable docs or READMEs are updated when behavior changes.
[ ] Mermaid diagrams render correctly when added or changed.
```

## Learning Notes For This Project

Since the project is also a learning vehicle, each implementation card should include a short "concept focus" note when useful.

Examples:

- Course repository: repository pattern, typed persistence boundary.
- LLM adapter: protocol, contract testing, dependency inversion.
- Chunker: functional core, O(n) text transformation.
- Ingestion pipeline: state machine, idempotency, result object.
- Planner: decision table, graph complexity, deterministic heuristics.
- Claim grounding: evidence-first design, bounded context, source provenance.

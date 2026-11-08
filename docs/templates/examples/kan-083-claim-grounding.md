# Worked Example: KAN-083 Claim Grounding Service

A filled-in [service contract template](../service-contract.md) for
[KAN-083](../../03-kanban-board.md) "Implement claim grounding service", the first
upcoming card that both calls an LLM and writes evidence-backed domain state.
Related contract source: [Engineering Standards](../../07-engineering-standards.md) and
requirement FR-NOTE-004 in [Requirements](../../01-requirements.md).

The card is not implemented yet. Fields marked `pending` depend on an unresolved decision
or an unmerged card and are named as open instead of guessed.

```text
Service: ClaimGrounder.ground (planned in app/services/, created by KAN-083)
Card: KAN-083 - Implement claim grounding service. Depends on KAN-067, KAN-082.
Status: draft
Owner: @kmosoti

Concept modeled:
Whether one learner note claim is supported, partly supported, or contradicted by the
course's own source spans, and which spans justify that judgement.

Kind: command (writes claim status and evidence rows). The retrieval it depends on is a
query.

Black-box boundary:
The caller does not need to know which retriever ran (FTS5, vector, graph expansion), the
prompt text, the provider or model, how many spans were considered, or where evidence rows
are stored. The caller knows only the request type, the result type, the failure modes, what
mutates, whether a retry is safe, the cost bounds, and what provenance comes back.

Input type:
GroundingRequest(claim_id, course_id, max_spans_per_claim=8, max_claim_chars=2000,
prompt_version). The claim text and its linked concept are loaded from persisted rows, not
passed in, so a caller cannot ground text that is not attached to a claim. The optional
max_spans_per_claim may only lower the default limit.

Output type:
GroundingResult(claim_id, status, supporting_span_ids, conflicting_span_ids, confidence,
rationale, evidence_ids, prompt_version, model, schema_version, retrieval_limits,
warning_codes) where status is one of GROUNDED, PARTIALLY_GROUNDED, CONFLICTING, or
NO_EVIDENCE. NO_EVIDENCE and a claim that is only partly supported are expected outcomes
carried in the result status, not exceptions.

Mutation:
- note_claims.maturity_status moves LINKED_TO_CONCEPT -> GROUNDED | PARTIALLY_GROUNDED |
  CONFLICTING, validated by can_transition() in app/core/states.py. NO_EVIDENCE leaves the
  claim at LINKED_TO_CONCEPT and reports a warning code.
- claim_evidence rows are inserted: claim_id, span_id, source_id, relation (supporting or
  conflicting), confidence, prompt_version, model, schema_version, created_at.
- Nothing else is written. source_spans, domain_nodes, domain_edges, and learner mastery
  rows are untouched. The retrieval step is read-only.

Idempotency:
Natural key: (claim_id, prompt_version, span_id, relation).
- Re-running with an unchanged retrieval set upserts the same evidence rows and recomputes
  the same status, so a retry after a lost response is safe.
- A different retrieval set or prompt version appends a new evidence version instead of
  overwriting; status is recomputed from the newest version and older rows remain for audit.
- Evidence inserts and the status update run in one transaction. A crash mid-run leaves the
  claim at its previous status and the retry repeats the work.
- LLM output is not cached across prompt versions; a new version deliberately re-asks.

Failure modes:
- Unknown claim or course -> NotFoundError, before any retrieval or LLM call.
- Claim not in LINKED_TO_CONCEPT -> InvalidStateTransitionError (ConflictError, 409) from
  the existing transition check, before spending an LLM call.
- Retrieval unavailable -> RetrievalError; no partial writes.
- Malformed or schema-incoherent LLM payload -> one retry with the same bounded context,
  then LlmContractError. Unbounded retry is not allowed.
- Provider timeout or transport failure -> GroundingError carrying claim_id and operation;
  the claim is unchanged.
- No supporting spans found -> not an error: status NO_EVIDENCE plus a warning code.
- Planned hierarchy: GroundingError and LlmContractError are added under the base error
  class in app/core/errors.py as part of this card. The taxonomy in the standards names that
  base class LearningOsError while the code currently exports LearningOSError; the card
  settles on one spelling rather than leaving both.

Complexity:
  Time: O(r * s + t) where r is retrieved spans (bounded by max_spans_per_claim), s is
        average span length, and t is prompt tokens; plus one LLM round trip.
  Space: O(r * s) for the packed context and O(r) evidence rows.
  Bound: max_spans_per_claim (default 8), max_claim_chars (default 2000), and one claim per
        call. A request that exceeds a limit is rejected with ValidationFailedError instead
        of being silently truncated, so the caller cannot believe more context was used than
        the service actually read.

External cost:
  LLM: exactly 1 call per claim per attempt, at most 2 attempts.
  Embedding: 1 call for the claim query when vector retrieval is configured; 0 otherwise.
  Database: 1 FTS5 query, 1 vector query when configured, at most 2 bounded graph traversal
        queries, and 2-3 row reads or writes.
  Filesystem/network: none for already-parsed sources; PDF/network work belongs to ingestion.
  Per user action ("ground this claim"): at most 1 LLM call and 1 embedding call.
  pending: whether vector retrieval participates at all depends on KAN-064, which is blocked
  on the vector backend decision; the contract stays valid with or without it.

Provenance:
For every decision the evidence rows retain span_id, source_id, page or section, confidence,
prompt_version, model id, schema_version, retrieval limits, and created_at. The structured
log line carries claim_id, course_id, learner_id, operation, status, duration_ms, counts, and
warning_codes per the Observability Standard. Rationale text is stored as a derived
explanation and is never treated as evidence itself.

Evidence:
Filled contract reviewed against the card's acceptance criteria; unit tests for the status
derivation function; contract tests for valid and invalid LLM payloads; one integration test
from note to grounded claim against fixture spans, extended by KAN-086.

Tests:
- Pure status derivation given a set of matched spans: no database, no clock, no LLM.
- Provider contract test: payload that validates, payload with an unknown span id, payload
  that fails the schema, and a cancelled call.
- Integration test against fixture spans: LINKED_TO_CONCEPT claim becomes GROUNDED with span
  references; a claim with no matches keeps its status and returns NO_EVIDENCE.
- Retry test: the same request twice leaves one evidence set and the same status.
```

## What This Example Demonstrates

- Every required field is answered, including the ones whose honest answer is "none" or
  `pending`.
- Limits are numbers, and over-limit input is rejected rather than truncated.
- Expected partial outcomes are result statuses; genuine failures are checked cheaply before
  expensive work.
- The mutation list is closed: a reviewer can name every table that changes.
- Provenance says what is kept, where, and which of it is derived rather than evidence.

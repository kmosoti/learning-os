# Service Contract Template

Use this template to describe one public service method before or while it is built.
A filled-in contract is what lets a reviewer, a test writer, or another agent use the
service as a black box without reading its implementation.

The canonical field meanings, the complexity format, and the review rubric live in
[Engineering Standards](../07-engineering-standards.md). This file is the copyable shape.

## When To Use It

Fill in one contract per public service method that:

- accepts typed input and returns a typed result,
- mutates or persists domain state,
- calls an LLM, an embedding model, a database, the filesystem, or the network,
- or is called from more than one card or surface.

Skip it for private helpers, pure value-object constructors, and adapters that only
translate data. When a helper grows a second caller or a side effect, give it a contract.

## Contract Block

Copy the block into the card, the PR description, or a service document next to the
code. Keep all eight required fields even when the answer is "none": a blank or "TBD"
field means the contract is unfinished, not that the field did not apply.

```text
Service: <module or type plus method, e.g. ClaimGrounder.ground>
Card: <KAN-### and issue link>
Status: <draft | implemented | verified>
Owner: <who can answer questions about it>

Concept modeled:
Kind: <command | query | both>
Black-box boundary: <what the caller must not need to know>

Input type:
Output type:

Mutation:
Idempotency:
Failure modes:
Complexity:
  Time:
  Space:
  Bound:
External cost:
  LLM:
  Embedding:
  Database:
  Filesystem/network:
Provenance:
Evidence:
Tests:
```

## Field Guidance

| Field | A complete answer says | An unfinished answer looks like |
| --- | --- | --- |
| Service | The named type and method a caller imports. | "the service layer". |
| Concept modeled | The domain question in one sentence. | A restatement of the method name. |
| Kind | Command, query, or both, and it agrees with the method name and route. | A write hidden behind a name that reads like a query. |
| Black-box boundary | What a caller may ignore: which retriever, prompt, provider, or storage detail. | Nothing; the caller is assumed to know internals. |
| Input type | The request type, plus each field that bounds the work. | `dict`, `Any`, or "various arguments". |
| Output type | The result type and how partial outcomes are represented. | A bare boolean or an untyped payload. |
| Mutation | The state written, named by table or aggregate, or "none (query)". | Silent writes that a maintainer discovers later. |
| Idempotency | The natural key, what a second identical call does, and what a crash mid-run leaves behind. | "should be safe to retry". |
| Failure modes | Each domain error or result status and the case that triggers it. | "raises exceptions". |
| Complexity | Time, space, and the limit that enforces the bound, in the `Time: O(...)` format of the Complexity And Cost Standard. | "fast enough for local use". |
| External cost | How many LLM, embedding, database, and I/O calls one user action can trigger, and what caps them. | "uses the LLM". |
| Provenance | The IDs, versions, and timestamps retained for audit, and where they are stored. | "stored somewhere for later". |
| Evidence | The test or artifact that shows the contract holds. | "manually checked once". |
| Tests | The pure-logic tests plus at least one boundary or contract test. | "covered by the integration suite". |

Rules of thumb:

- Every field must be answerable by someone who cannot read the implementation.
- State limits as numbers, not adjectives.
- Use result statuses for expected partial outcomes and domain errors for real failures.
- When a field depends on an unresolved decision, write `pending: <card or ADR>` and say
  what the open decision changes. An honest pending field is better than a guess.

## Completion Checklist

- [ ] Input, output, mutation, idempotency, failure modes, complexity, external cost, and provenance are all answered; "none" is acceptable, "TBD" is not.
- [ ] No mutation hides inside a query, and no query-shaped name hides a write.
- [ ] Second-run and crash-mid-run behavior are named for every write.
- [ ] Failure modes map to the error taxonomy or a result status, never a raw exception or boolean.
- [ ] Complexity names the input that drives time and space plus the limit that enforces the bound.
- [ ] External cost states how many external calls one user action can trigger and what caps them.
- [ ] Provenance names exactly what is retained and where a reviewer can find it.
- [ ] The filled contract is linked from the card that implements or consumes the service.

## Worked Example

[KAN-083 claim grounding service contract](examples/kan-083-claim-grounding.md) shows this
template filled in for an upcoming service card, including the fields that legitimately
stay open until their blocking decision lands.

# Worked Example: Offline Journal Command

A filled-in [service contract template](../service-contract.md) for the
`learningos_application::execute` use case being delivered as part of the active
[offline learning slice, issue #52](https://github.com/kmosoti/learning-os/issues/52)
(draft PR [#56](https://github.com/kmosoti/learning-os/pull/56)). Unlike a hypothetical
service, this example is grounded in the current LearningOS 2.0 implementation and its
contract tests. It is illustrative until that PR is reviewed and merged.

```text
Service: learningos_application::execute<J: Journal>
Card: #52 - Deliver the ownership lesson through durable offline resume
Status: implemented in draft PR #56; review and merge pending
Owner: @kmosoti

Concept modeled:
Apply one learner command to the current session, then acknowledge it only after its
resulting evidence is durably committed.

Kind: command.

Black-box boundary:
Callers provide a versioned request and a Journal port. They do not need to know whether the
journal is backed by browser IndexedDB or a native adapter, how events are replayed, or how a
session projection is stored. The application service uses pure domain transitions; the
adapter owns atomic persistence.

Input type:
Request { schema: u32, id: String, expected: u64, at_ms: u64, command: Command }.
Schema 1 is accepted. The operation ID is required and at most 128 bytes. Command
payload limits and phase invariants are checked by the domain transition (for example, goals
are at most 240 bytes, reasoning and reading thoughts at most 4096 bytes, and session
revision is capped at 10,000).

Output type:
Receipt { request: Request, revision: u64 }. The receipt binds the exact request to the
resulting session revision.

Mutation:
On a new valid command, one Journal commit atomically stores the request receipt, the event
batch, and the resulting session projection. A repeated identical request makes no new
mutation. The pure transition itself does not write storage.

Idempotency:
- Look up a receipt by operation ID before reading/replaying the session or checking the
  expected revision.
- If the stored request is identical, return its original receipt. A retry after a lost
  acknowledgment therefore succeeds even though its expected revision is now old.
- If the same operation ID carries a different request, return Conflict; never overwrite the
  original result.
- The Journal commit repeats duplicate-ID and expected-revision checks atomically to cover
  races between the initial receipt lookup and commit. A storage failure is not acknowledged.

Failure modes:
- Unsupported request schema, invalid command, or invalid phase transition ->
  AppError::Domain(DomainError); no commit.
- Expected revision does not match the current session -> AppError::Stale; the draft remains
  available to the caller for reload and retry.
- Operation ID already belongs to a different request -> AppError::Conflict.
- Journal read or commit failure -> AppError::Storage; no success receipt is returned.
- Repeating the exact request after commit -> success with the existing receipt, not an error.

Complexity:
  Time: O(n log n + n × m) worst-case to replay n events with m observations, plus the
        adapter's receipt lookup and atomic commit.
  Space: O(n + m) for replayed event identities and session observations, plus one bounded
         command's event batch.
  Bound: n is capped by the 10,000 session revision limit; m is capped at 2,000 observations;
         operation IDs at 128 bytes; goals at 240 bytes; reasoning and reading thoughts at
         4096 bytes each. The domain rejects inputs beyond these limits.

External cost:
  LLM: 0.
  Embedding: 0.
  Database/local storage: for a new request, one receipt lookup, one journal read/replay, and
  one atomic commit. The commit contains the receipt, event batch and projection together.
  Filesystem/network: 0 in the application use case; the lesson's source bundle is local.
  Per user action: no remote calls. Journal I/O is through the injected local adapter.

Provenance:
The stored receipt retains the complete request (schema, operation ID, expected revision,
caller-supplied time, command) and resulting revision. Events retain generated event IDs,
command time and policy version; the projection is rebuilt from the event history. This
preserves what command produced each state transition and supports retry and replay.

Evidence:
PR #56 includes the application port contract tests and browser/native journal integration.
In particular, `lost_ack_retry_and_conflicting_id_are_distinct` checks the same-request retry,
same-ID/different-payload conflict, and stale-revision cases. The PR is draft, so this is
implementation evidence for review, not a claim of accepted or merged behavior.

Tests:
- Pure domain tests reject invalid phase transitions and enforce input/session bounds.
- Application port test `lost_ack_retry_and_conflicting_id_are_distinct` proves retry,
  conflict, and stale-revision behavior.
- Adapter tests exercise atomic persistence and offline reload against the local journal.
- Browser tests verify the user flow and that a stale draft remains available.
```

## What This Example Demonstrates

- A stack-specific example can still preserve a black-box boundary: the service accepts a
  port, not a database or browser handle.
- Retries and race handling are stated at the service/adapter boundary, not hidden behind a
  generic "safe to retry" claim.
- Costs identify local adapter calls while explicitly ruling out LLM, embedding, and remote
  network calls.
- Complexity bounds and provenance describe the actual versioned journal, request, and
  event contract.
- The implementation is in a draft PR, so its evidence is clearly distinguished from merged
  or approved behavior.

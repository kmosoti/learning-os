# Roadmap And Milestones

The roadmap is ordered to produce a working vertical slice early, then widen the loop. Each milestone should end with something demonstrable.

## Critical Path

1. Project boots and checks run.
2. Database schema exists.
3. Course can be created.
4. Source can be uploaded.
5. Source can be parsed into spans.
6. EMT fixtures generate trusted baseline graph and outcomes.
7. Search retrieves source spans and graph context.
8. Notes become claims.
9. Claims are grounded.
10. Quiz answer updates learner state.
11. Planner selects one next action.
12. Minimal UI exposes the loop end to end.

## Milestone 0 - Foundation

Outcome: a local developer can sync dependencies, start the app, and run checks.

Cards:

- KAN-001 through KAN-009.
- KAN-020.

Acceptance:

- `uv sync` completes.
- FastAPI app boots.
- Ruff, ty, and pytest are wired.
- State enums exist before domain logic is added.

## Milestone 1 - Course And Source Spine

Outcome: a user can create a course, upload a file, process it, and view source spans.

Cards:

- KAN-010 through KAN-013.
- KAN-021 through KAN-024.
- KAN-030 through KAN-039.
- KAN-110 through KAN-113.

Acceptance:

- Course create/list/detail works through API and UI.
- PDF and text sources produce persisted SourceSpan rows.
- Processing statuses are visible.
- Empty/scanned PDF failure is understandable.

## Milestone 2 - Trusted EMT Fixture Baseline

Outcome: EMT pre-course fixtures can seed a known graph and outcome set without relying on LLM guesses.

Cards:

- KAN-014 through KAN-015.
- KAN-043 through KAN-045.
- KAN-047 through KAN-050.
- KAN-130 through KAN-132.

Acceptance:

- Assignment questions become outcomes.
- Medication chart rows become medication facts with source references.
- Terminology rows become terms and aliases.
- Golden tests prove expected extraction for representative rows.

## Milestone 3 - General Extraction And Retrieval

Outcome: non-fixture source spans can produce structured graph candidates, and the app can retrieve source-grounded context.

Cards:

- KAN-040 through KAN-042.
- KAN-046, KAN-048, KAN-049.
- KAN-060 through KAN-068.

Acceptance:

- LLM extraction is provider-abstracted and schema-validated.
- Invalid outputs fail safely.
- FTS5 lexical search returns source spans.
- Vector adapter has one working implementation or a documented fallback.
- Hybrid retrieval returns source spans, concepts, outcomes, and learner flags.

## Milestone 4 - Notes To Grounded Claims

Outcome: the signature note-maturation loop works.

Cards:

- KAN-016.
- KAN-025 through KAN-026.
- KAN-080 through KAN-086.
- KAN-114.

Acceptance:

- Raw note is stored.
- Candidate claims are extracted.
- Claims link to concepts when possible.
- Claims ground against source spans.
- UI shows raw, parsed, grounded, partially grounded, conflicting, and quiz-ready states.

## Milestone 5 - Quiz And Learner Mastery

Outcome: the learner can answer a quiz and the system updates mastery and misconceptions.

Cards:

- KAN-017 through KAN-018.
- KAN-027.
- KAN-090 through KAN-097.
- KAN-116 through KAN-117.

Acceptance:

- Quiz item includes source references and answer criteria.
- Answer evaluation produces structured score, missing concepts, misconceptions, and next action.
- Mastery updates are bounded 0 to 1.
- Assignment cards support teach, outline, review, and quiz flows.

## Milestone 6 - Planner

Outcome: the study page recommends exactly one next useful action.

Cards:

- KAN-028.
- KAN-100 through KAN-105.
- KAN-115.
- KAN-134.

Acceptance:

- Planner chooses weak prerequisite first.
- Planner falls back to due review.
- Planner falls back to next course outcome.
- Every next action includes a title, reason, and estimated minutes.
- CLI can print the next action for demo/debugging.

## Milestone 7 - Evidence Caveats

Outcome: concepts can carry manual evidence and protocol caveats without expanding into an auto-updater.

Cards:

- KAN-019.
- KAN-120 through KAN-124.

Acceptance:

- Evidence notes can be added manually.
- EMS medication/protocol content can be marked local protocol dependent.
- Evidence status is visible where it changes learner interpretation.

## Milestone 8 - End-To-End EMT Demo

Outcome: the MVP demo can be run from a fresh local setup.

Cards:

- KAN-118 through KAN-119.
- KAN-133, KAN-135 through KAN-137.
- KAN-150 through KAN-156 as needed for release quality.

Acceptance:

- Demo seeds EMT course.
- Demo processes sources or fixture content.
- Demo recommends ventilation/perfusion or another assignment-linked next action.
- Learner answer updates mastery and misconception state.
- Planner changes the next task after evaluation.
- README documents exact demo commands.

## First Two-Week Pull Plan

Week 1:

- KAN-001 - Initialize uv project skeleton.
- KAN-002 - Pin Python 3.14.5.
- KAN-003 - Configure Ruff, ty, and pytest.
- KAN-004 - Add FastAPI application shell.
- KAN-005 - Add core config, logging, errors, IDs, and time helpers.
- KAN-006 - Define invariant state enums.
- KAN-010 - Add SQLAlchemy base and session factory.
- KAN-011 - Add Alembic migration setup.

Week 2:

- KAN-012 - Implement Course model and migration.
- KAN-013 - Implement Source and SourceSpan models.
- KAN-020 - Add health check and typed API error envelope.
- KAN-021 - Add course create/list/detail routes.
- KAN-022 - Add source upload route.
- KAN-030 - Implement loader contract.
- KAN-031 - Implement PDF loader with PyMuPDF.
- KAN-032 - Implement TXT and Markdown loader.

## Development Rule

Do the manual EMT fixture extraction before general LLM extraction. That creates a truth baseline and prevents the project from mistaking plausible structure for useful structure.


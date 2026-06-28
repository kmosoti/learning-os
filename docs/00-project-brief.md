# Project Brief

## Product Thesis

`learning-os` is a course-aware, source-grounded cognitive ledger system. It treats course materials as the primary truth surface, but it does not treat the syllabus sequence as a complete model of learning.

A syllabus is a delivery schedule: what must be covered, assessed, and completed by a given date. A learner needs a second structure underneath it: a prerequisite ledger that tracks what each advanced topic depends on, what the learner has evidence of knowing, what failure modes keep recurring, and what next action would repair the most important prerequisite debt.

The first MVP should prove the engine:

- Source-grounded graph construction.
- Course-to-ledger conversion from syllabi, assignments, charts, and notes.
- Prerequisite tracking for advanced topics.
- Note maturation from raw note to grounded claim.
- Retrieval practice generated from course sources.
- Learner mastery and failure-mode updates.
- Planner selection of one next useful action.

The first MVP should not try to become a general scientific literature monitor, a clinical authority, a mobile app, a generic summarizer, or a multi-user classroom product.

## Thesis Commitments

`learning-os` is built around these commitments:

1. **The syllabus is an external constraint, not the learning model.** The system keeps dates, exams, labs, and required readings visible, but it builds a cognitive dependency graph underneath them.
2. **Rote memory is necessary but insufficient.** Facts, terms, medications, doses, and contraindications still need recall, but they should be attached to concepts, skills, scenarios, and source spans.
3. **Testing should repair, not merely gate.** Quizzes should expose missing prerequisites and misconceptions early enough to schedule low-stakes repair.
4. **Failure modes are first-class data.** A wrong answer should not only lower mastery. It should identify the shape of the miss: missing term, weak concept, bad calculation, unsafe sequence, protocol confusion, or failed transfer.
5. **The planner should reduce cognitive load.** The user should receive one next useful action with a reason, not an undifferentiated list of study options.

## MVP Product Definition

The MVP supports this loop:

1. User creates a course.
2. User uploads source files: PDFs, notes, syllabi, assignments, charts.
3. System parses sources into source spans.
4. System extracts concepts, claims, terms, medications, questions, skills, outcomes, and relationships.
5. System builds a source graph, domain graph, outcome graph, learner graph, note/claim maturity graph, evidence graph, and prerequisite ledger.
6. User takes notes during class.
7. System converts notes into candidate claims.
8. System grounds claims against course sources.
9. System generates quizzes, scenario checks, and assignment help.
10. User answers.
11. System evaluates the answer.
12. System updates learner mastery, prerequisite debt, and failure modes.
13. System chooses the next useful study action.

## MVP Dataset

The EMT/paramedic prep corpus is the first demo dataset:

- Paramedic course information and student schedule.
- A&P pre-course assignment with required OpenStax readings and 22 questions.
- Medical terminology file with roots, prefixes, suffixes, and abbreviations.
- EMT medication chart with medication, mechanism, indication, contraindication, drug class, adult dose, and pediatric dose.
- Top-50 prescribed medications file with common home medications and classes.
- Skills checkoff material when available.
- National EMS scope/protocol material when available, treated as course context rather than clinical authority.

## Ledger Model

The MVP should treat the learner state as ledgers, not as a flat percentage score.

| Ledger | Purpose | Example Data |
| --- | --- | --- |
| Source Ledger | Preserve what each source says | source spans, page/section refs, checksums |
| Concept Ledger | Track conceptual understanding | oxygenation, ventilation, shock, homeostasis |
| Fact Ledger | Track required recall | terms, abbreviations, doses, indications |
| Skill Ledger | Track procedural readiness | BVM, IV/IO, intubation, needle decompression |
| Scenario Ledger | Track transfer into cases | asthma vs CHF, anaphylaxis vs anxiety |
| Failure-Mode Ledger | Track shape of misses | bad calculation, missing contraindication, weak differential |
| Review Ledger | Track spacing and decay | last seen, next review, confidence, attempts |

## Non-Negotiable Architecture Principle

Keep these layers separate:

| Layer | Meaning | Stored As |
| --- | --- | --- |
| Source Graph | What the uploaded source says | Source, SourceSpan, citation links |
| Domain Graph | What the course domain means | concepts, terms, meds, skills, relationships |
| Outcome Graph | What the course requires | assignment questions, learning outcomes, labs, exams |
| Prerequisite Ledger | What blocks advanced topics | dependency edges, readiness gates, prerequisite debt |
| Learner Graph | What the learner believes and can do | mastery, attempts, misconceptions, confidence |
| Failure-Mode Ledger | Why the learner missed | error type, repeated pattern, linked prerequisite |
| Evidence Graph | What is verified, current, or caveated | evidence notes, protocol caveats |
| Planner | What the agent should ask the learner to do next | next action, reason, estimate |

Do not collapse these into one vector database. Vector search is a retrieval aid, not the system of record.

## Primary User

A self-directed learner preparing for EMT/paramedic coursework before official instruction begins.

The product should reduce cognitive load. The study page should recommend one next action with a clear reason, not a menu of 15 choices.

## MVP User Promise

After uploading course materials, the learner can ask:

- What do I need to work on next?
- Which assignment question am I weak on?
- What advanced topic am I not ready for yet?
- What prerequisite is blocking this topic?
- Are my class notes supported by the source material?
- Can you quiz me on the parts I am missing?
- What misconception did my answer reveal?
- What failure pattern am I repeating?

The system answers with citations or source span references whenever it makes a content claim.

## Out Of Scope For MVP

- Multi-user classroom support.
- Real-time lecture transcription.
- Automatic PubMed, OpenAlex, or guideline watching.
- Neo4j.
- Complex React graph visualization.
- Mobile app.
- OCR for scanned PDFs.
- DRM textbook ingestion.
- Autonomous web research.
- Clinical recommendation engine.

## MVP Completion Definition

The MVP is complete when:

- Project uses Python 3.14.5 with uv and a committed `uv.lock`.
- Ruff, ty, and pytest run locally and in CI.
- User can create a course.
- User can upload PDF, TXT or Markdown, DOCX, and EPUB.
- System extracts source spans.
- System extracts concepts, claims, questions, medications, terms, skills, outcomes, and relationships.
- System builds domain nodes and edges.
- System builds prerequisite ledger entries for at least one advanced topic.
- System supports lexical search and a basic vector search adapter.
- User can add class notes.
- Notes become claims.
- Claims are grounded against source spans.
- System generates quiz questions.
- User can answer quizzes.
- System evaluates answers.
- Learner mastery updates.
- Failure modes are captured from answer evaluations.
- Planner selects one next useful action.
- Assignment questions are tracked as outcomes.
- UI supports Today, Notes, Quiz, Assignment, Sources, and Map.

## Verified Tooling Assumptions

Verification date: 2026-06-04.

- Python.org source downloads list Python 3.14.5 as the latest Python 3 release and show its release date as 2026-05-10.
- uv documentation states that uv manages Python versions and uses a universal lockfile.
- Ruff settings list `py314` as a supported target version.
- ty documentation describes ty as Astral's Python type checker and language server.

References:

- https://www.python.org/downloads/source/
- https://www.python.org/downloads/latest/
- https://docs.astral.sh/uv/
- https://docs.astral.sh/uv/concepts/projects/layout/
- https://docs.astral.sh/ruff/settings/
- https://docs.astral.sh/ty/type-checking/

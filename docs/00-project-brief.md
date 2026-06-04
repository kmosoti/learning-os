# Project Brief

## Product Thesis

`learning-os` is a course-aware, source-grounded study system. It treats course materials as the primary truth surface, separates different kinds of knowledge into distinct graph layers, and uses the learner's actual notes and answers to decide the next useful study action.

The first MVP should prove the engine:

- Source-grounded graph construction.
- Note maturation from raw note to grounded claim.
- Retrieval practice generated from course sources.
- Learner mastery updates.
- Planner selection of one next useful action.

The first MVP should not try to become a general scientific literature monitor, a clinical authority, a mobile app, or a multi-user classroom product.

## MVP Product Definition

The MVP supports this loop:

1. User creates a course.
2. User uploads source files: PDFs, notes, syllabi, assignments, charts.
3. System parses sources into source spans.
4. System extracts concepts, claims, terms, medications, questions, and relationships.
5. System builds a source graph, domain graph, outcome graph, learner graph, note/claim maturity graph, and evidence graph.
6. User takes notes during class.
7. System converts notes into candidate claims.
8. System grounds claims against course sources.
9. System generates quizzes and assignment help.
10. User answers.
11. System evaluates the answer.
12. System updates learner mastery and chooses the next useful study action.

## MVP Dataset

The EMT/paramedic prep corpus is the first demo dataset:

- A&P pre-course assignment with required OpenStax readings and 22 questions.
- Medical terminology file with roots, prefixes, suffixes, and abbreviations.
- EMT medication chart with medication, mechanism, indication, contraindication, drug class, adult dose, and pediatric dose.
- Top-50 prescribed medications file with common home medications and classes.

## Non-Negotiable Architecture Principle

Keep these layers separate:

| Layer | Meaning | Stored As |
| --- | --- | --- |
| Source Graph | What the uploaded source says | Source, SourceSpan, citation links |
| Domain Graph | What the course domain means | concepts, terms, meds, relationships |
| Outcome Graph | What the course requires | assignment questions, learning outcomes |
| Learner Graph | What the learner believes and can do | mastery, attempts, misconceptions |
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
- Are my class notes supported by the source material?
- Can you quiz me on the parts I am missing?
- What misconception did my answer reveal?

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
- System extracts concepts, claims, questions, medications, terms, and relationships.
- System builds domain nodes and edges.
- System supports lexical search and a basic vector search adapter.
- User can add class notes.
- Notes become claims.
- Claims are grounded against source spans.
- System generates quiz questions.
- User can answer quizzes.
- System evaluates answers.
- Learner mastery updates.
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

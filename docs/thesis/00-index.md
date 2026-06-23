# learning-os Thesis Workspace

This directory is the scholarly workspace for the `learning-os` thesis. It is not a marketing site, blog, or final dissertation. It is a versioned research and design corpus that should mature into product requirements, architecture decisions, validation protocols, and eventually publishable public writing.

## Working Thesis

`learning-os` is a source-grounded cognitive tool for making hidden learning state visible.

It converts course materials into a graph of concepts, facts, skills, scenarios, outcomes, and prerequisite relations. It then observes learner-generated artifacts, including explanations, diagrams, quiz answers, scenario responses, notes, confidence ratings, and skill checks. These artifacts are recorded as evidence events and evaluated against the source-grounded graph.

From this evidence, `learning-os` builds visible representations of learner state: validated skill towers, failure-mode traces, prerequisite debt, review schedules, and readiness gates. The purpose is not merely to summarize content or generate quizzes. The purpose is to help the learner and AI partner reason about what is understood, what is fragile, what blocks future learning, and what one action should happen next.

## Scholarly Standard

Each chapter should distinguish among:

| Claim Type | Meaning | Required Support |
| --- | --- | --- |
| Established research claim | Supported by existing peer-reviewed or institutional research | Full citation and summary of scope |
| Design inference | An architectural consequence inferred from research | Citation plus explicit reasoning |
| Product hypothesis | A claim about what `learning-os` will improve | Validation plan and falsification criteria |
| Implementation decision | A concrete design choice | ADR or requirements linkage |

Do not present product hypotheses as if they are already established research findings.

## Chapter Map

| Chapter | Status | Purpose |
| --- | --- | --- |
| [01. The Invisible-State Problem](01-invisible-state-problem.md) | Draft | Defines the hidden learner-state problem that motivates the product. |
| [02. Cognitive Tools for Making the Invisible Visible](02-cognitive-tools-making-invisible-visible.md) | Draft | Uses Judy Fan's cognitive-tools research as a theoretical anchor. |
| [03. Logical Representation Model](03-logical-representation-model.md) | Draft | Maps the thesis to graph, event, tower, gate, and planner objects. |
| 04. Generative Evidence | Planned | Treats learner-produced artifacts as evidence about mental state. |
| 05. Validated Skill Towers | Planned | Develops the tower/progression metaphor without blockchain overreach. |
| 06. Failure Modes and Prerequisite Debt | Planned | Models wrong answers as diagnostic evidence. |
| 07. Planner as Cognitive-Cost-Aware Subgoal Selector | Planned | Develops the one-next-action planner. |
| 08. NotebookLM and Sheets Prototype Protocol | Planned | Defines the concierge validation experiment before app completion. |
| 09. Startup and Market Thesis | Planned | Separates product thesis from business thesis. |
| [References](references.md) | Draft | Shared bibliography for thesis chapters. |

## Research Notes

Raw research notes live under `research-notes/`. They can be incomplete and exploratory, but they should still preserve citation metadata and clearly mark unsupported inferences.

Current notes:

- [Judy Fan: Cognitive Tools and Generative Evidence](research-notes/judy-fan-cognitive-tools.md)

## Representation Vocabulary

Use these terms consistently:

| Term | Meaning |
| --- | --- |
| Source span | A cited fragment from uploaded course material or research. |
| Knowledge graph | The course/domain structure: concepts, facts, skills, scenarios, outcomes, and relations. |
| Prerequisite DAG view | A directed acyclic projection used for readiness and dependency reasoning. The full knowledge graph may contain cycles. |
| Learning event log | Append-only record of learner activity and evidence. |
| Validation block | A durable record that a learner demonstrated a competency at a defined level. |
| Skill tower | A learner-facing progression view over validation blocks in a subset domain. |
| Readiness gate | A rule that determines whether a learner can attempt an advanced competency. |
| Failure mode | A typed explanation of why an attempt failed or was fragile. |
| Planner | The service that selects one next useful action from graph structure and learner-state projections. |

## Validation North Star

The thesis becomes product-relevant only if the system can beat the status quo on at least one observable outcome:

- Fewer repeated failure modes after delayed review.
- Better transfer from facts/concepts into scenarios.
- Better confidence calibration.
- Less decision fatigue around what to study next.
- More useful next-action selection than a generic study guide or flashcard deck.

The strongest early validation target is:

> Ledger/tower-first study reduces repeated failure modes compared with NotebookLM-only study guides and normal flashcards.

# Research Note: Judy Fan, Cognitive Tools, and Making Learning State Visible

## Status

Exploratory research note. This note informs Chapter 02 and should be refined as more papers are read.

## Why This Research Matters

Judith Fan's research is relevant because `learning-os` is not only a study planner. It is a representational tool. The platform aims to make invisible learning state visible through graphs, evidence events, failure-mode traces, skill towers, and readiness gates.

Fan's work provides a theoretical anchor for that move: humans use cognitive tools to externalize thought into visible, manipulable, communicable representations.

## Key Sources

### Fan et al. (2023): Drawing as a versatile cognitive tool

Fan, Bainbridge, Chamberlain, and Wammes describe drawing as a cognitive tool that makes the invisible contents of mental life visible. The article reviews drawing production and comprehension across perception, memory, social inference, learning, and communication.

Relevant ideas for `learning-os`:

- Drawings externalize mental contents.
- External representations range from realistic to abstract.
- Visual abstractions can support learning and communication.
- Cognitive tools may include diagrams, maps, visualizations, writing, and numerals.
- A useful cognitive tool must be expressive enough to represent meaning and simple enough for novices to use.

Design implication:

`learning-os` should not expose the learner to raw graph complexity. It should use simple visible representations such as towers, gates, blockers, and next-action cards.

### Fan (2015): Drawing to Learn

Fan argues that producing graphical representations can enhance scientific thinking because graphical representations support observation, explanation, problem solving, and communication.

Important nuance:

The design implication is not simply "make learners draw." Learner-produced artifacts need comparison against reference knowledge and targeted feedback.

Design implication:

Learner explanations, diagrams, scenario answers, and procedure sequences should become evidence events. The platform should evaluate them against source-grounded criteria and update learner state.

### Fan (2026): Generative Behaviors as Key Targets for Cognitive Models

Fan argues that generative behaviors are rich targets for cognitive modeling because they reveal more about mental state than constrained discriminative tasks. Open-ended behavior leaves high-dimensional traces.

Design implication:

Multiple-choice quizzes are not enough. `learning-os` should treat generative artifacts as first-class evidence:

- explain-back answers
- diagrams
- concept maps
- scenario responses
- procedure sequences
- written justifications

This supports failure-mode diagnosis and prerequisite-debt detection.

### Cullen et al. (2018): Argument visualization

Cullen, Fan, van der Brugge, Elga, and Cohen report a quasi-experimental field study in which students practiced visualizing argument structure, completed weekly problem sets, and received individualized feedback. The study found gains in analytical reasoning and argument understanding.

Design implication:

Making hidden logical structure visible can support reasoning, but the intervention includes more than visualization. Practice and feedback matter.

For `learning-os`, the analog is:

```text
learner artifact -> structural evaluation -> feedback -> updated state -> next action
```

## Mapping to learning-os

| Fan Research Theme | learning-os Concept |
| --- | --- |
| Drawing makes invisible mental contents visible | Skill towers and readiness gates make learner state visible |
| Drawings vary from realistic to abstract | Source spans, graphs, towers, and planner cards are different abstraction levels |
| Generative behavior reveals mental state | Explanations and scenario responses become evidence events |
| Visual/logical representations can aid reasoning | Prerequisite graphs and failure traces support study decisions |
| Cognitive tools must be expressive and learnable | UI should show small actionable views, not raw graph sprawl |

## Claims Supported

The following claims are reasonably supported by this research family:

1. External representations can support cognition by making otherwise hidden structure visible.
2. Generative artifacts can reveal more about mental state than fixed-choice responses alone.
3. Visual/logical representations can support learning when paired with practice and feedback.
4. Cognitive tools should balance expressive power with usability for novices.

## Claims Not Yet Supported

The following remain product hypotheses:

1. Skill towers improve paramedic learning outcomes.
2. Failure-mode traces reduce repeated errors.
3. Readiness gates improve study selection.
4. NotebookLM plus Sheets can approximate the system well enough for validation.
5. Learners will pay for or repeatedly use this workflow.

## Open Questions

- What kinds of learner-generated artifacts are easiest to evaluate reliably?
- Should the first prototype require diagrams, or should it begin with written explanations and scenario responses?
- How simple can the visible representation be while still improving learner decisions?
- What failure-mode taxonomy is stable enough across domains?
- Which representations reduce cognitive load rather than adding it?

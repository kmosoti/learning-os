# Chapter 01: The Invisible-State Problem

## Claim

The central problem in course-based learning is that the most important learning state is hidden. A grade, quiz score, completed module, or highlighted reading can show that activity occurred, but it does not directly reveal what the learner understands, what they misunderstand, what they can transfer, what is fragile, or what prerequisite debt will block future topics.

`learning-os` begins from this claim:

> The useful unit of learning software is not the document, the quiz, or the flashcard. It is an evidence-backed model of learner readiness.

## Problem

The ordinary course interface is organized around institutional delivery:

- syllabi
- readings
- lectures
- quizzes
- exams
- grades
- deadlines
- completion checklists

This is necessary for administration, but it is not a sufficient model of cognition. A syllabus can say that airway management occurs in Week 4, but it cannot say whether the learner is ready for airway management. A quiz can say that a learner scored 70%, but it may not say whether the miss was caused by a missing term, weak concept, wrong discrimination between similar cases, unsafe procedural sequence, calculation error, or overconfidence.

The proposed platform exists to make that hidden state visible enough to act on.

## Research Basis

This chapter is not claiming that `learning-os` is already empirically validated. Instead, it anchors the problem in adjacent research.

Dunlosky et al. (2013) reviewed ten learning techniques and argued that students need methods that help them regulate learning effectively. Their review rated practice testing and distributed practice as high-utility techniques because they generalize across many learners, materials, and outcome tasks. This supports the idea that learning state should be updated through retrieval evidence and delayed review, not only passive exposure or rereading.

The Institute of Education Sciences practice guide *Organizing Instruction and Study to Improve Student Learning* recommends spacing learning over time, interleaving worked examples with problem solving, combining graphics with verbal descriptions, connecting abstract and concrete representations, using quizzes for re-exposure, using tests and quizzes to identify content that still needs learning, and asking deep explanatory questions (Pashler et al., 2007). These recommendations imply that learning software should track more than correctness; it should track when to review, what representational form to use, and what kind of prompt is likely to repair understanding.

Knowledge Space Theory offers a formal precedent for treating learner knowledge as a structured state rather than a crude numeric mark. Doignon and Falmagne (2015) frame the problem as designing procedures that accurately assess a student's knowledge and efficiently advise further study, departing from common numerical evaluation. This supports `learning-os` modeling readiness as a structured state over concepts, skills, and prerequisites rather than a flat mastery score.

## Logical Representation

The hidden-state problem can be represented as a mapping from observed traces to inferred learner state.

```text
Hidden learner state
  -> observable learner artifact
  -> source-grounded evaluation
  -> inferred failure mode or validated competency
  -> learner-state projection
  -> next action
```

### Observable Artifacts

```yaml
observable_artifact:
  id: artifact_001
  learner_id: learner_001
  artifact_type: explanation
  target_competency_id: concept.oxygenation_vs_ventilation
  content: "Ventilation is how oxygen gets into the blood..."
  confidence: 4
  created_at: 2026-08-27T20:10:00
```

### Evidence Event

```yaml
learning_event:
  id: evt_001
  learner_id: learner_001
  competency_id: concept.oxygenation_vs_ventilation
  event_type: explain_back_attempt
  score: 0.45
  confidence: 4
  source_refs:
    - source_span.jbl_ch16_004
  inferred_failure_modes:
    - weak_concept
    - overconfidence
```

### Learner-State Projection

```yaml
learner_competency_state:
  learner_id: learner_001
  competency_id: concept.oxygenation_vs_ventilation
  mastery_state: fragile
  confidence_state: overconfident
  last_seen: 2026-08-27
  next_review: 2026-08-29
  blocks:
    - skill.bvm_ventilation
    - scenario.respiratory_failure
```

The event log preserves what happened. The projection summarizes what currently matters.

## learning-os Implication

The platform should not primarily ask, "What content did the learner consume?"

It should ask:

1. What competency was the learner attempting to demonstrate?
2. What artifact did the learner produce?
3. What source-grounded criteria evaluate that artifact?
4. What failure mode or validation does the artifact imply?
5. What future competency does this state block or unlock?
6. What is the one next useful action?

This shifts the product from content delivery to diagnostic learning state.

## Example: Paramedic Learning

A paramedic course may place emergency medications, med math, airway procedures, and scenario work in separate syllabus slots. A learner can pass early recall quizzes and still be fragile when asked to combine these in an RSI scenario.

A normal study tool might show:

```text
You scored 70% on medication questions.
```

`learning-os` should show:

```text
You can recall several medication names, but your med math and contraindication ledgers are fragile.
This blocks emergency medication scenarios and RSI readiness.
Next action: complete one worked example for weight-based dosing, then answer three retrieval questions with confidence ratings.
```

## Validation Question

Does a source-grounded learner-state model produce better study selection than a syllabus-first or flashcard-first workflow?

Early test:

- Give learners a topic packet.
- Compare normal NotebookLM study guides against ledger/tower-first next-action recommendations.
- Measure repeated failure modes at immediate, 24-hour, and 7-day intervals.

## Caveat

This chapter supports the problem framing, not the product solution. It is possible that a simpler tool, such as a well-structured Anki deck plus coaching rubric, solves enough of the problem. `learning-os` must prove that its additional representation layer improves actionability rather than adding overhead.

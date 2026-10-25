# ADR-013: Evidence authority and joint-premise reasoning

Status: Proposed formal record; fixed direction accepted and implementation authorized by the owner on 2026-10-25 with “Implement the proposed plan.” Mechanism choices are deferred to the approved experiments. PR approval/merge remains pending.

Related: [#49](https://github.com/kmosoti/learning-os/issues/49) and [execution contract](../architecture/v2-execution-contract.md).

## Decision
Source/assertion identity, immutable observations, rubric evaluations and capability projections are separate. Evaluations bind exact observation, question, rubric and evaluator versions. Corrections/retractions retain links; projections replay deterministically from accepted inputs. An observation is not a model's interpretation of it.

An argument contains a nonempty set of premises and one conclusion. Preserve AND semantics: A and B implying C does not authorize either A or B alone to imply C. Keep support distinct from entailment. General graphs may cycle; plans are explicit validated DAG projections. Do not import LearnHarness's simplifier or trusted-extractor assumptions.

## Initial policy
Deterministic structured-answer rubrics; free reasoning retained but not semantically graded. Exposure and prediction do not establish mastery. A correct immediate check is limited evidence. Select repair, unaided retry, transfer or delayed retrieval, always one action with a reason and user control. No psychological/neurological inference or unsupported permanent score.

## Confirmation
Exhaustive small AND truth tables, invalid-state/property tests, replay equality native/WASM, golden traces authored separately from implementation, compiler-backed Rust examples and adversarial fake-model proposals. Source and evaluator revisions must remain inspectable.

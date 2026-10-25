use learningos_domain::*;
use std::collections::BTreeSet;
fn step(s: &Session, command: Command, n: u64) -> (Session, Vec<Event>) {
    let events = decide(s, &command, &format!("operation-{n}"), n * 1000).unwrap();
    let mut next = s.clone();
    for e in &events {
        next = apply(&next, e).unwrap();
    }
    (next, events)
}
fn answer(value: Answer) -> Command {
    Command::Answer {
        answer: value,
        confidence: None,
        reasoning: "independent test trace".into(),
    }
}
fn check_ready() -> (Session, Vec<Event>) {
    let mut s = Session::default();
    let mut history = vec![];
    for (n, c) in [
        Command::ChooseGoal {
            goal: "Rust ownership".into(),
        },
        Command::ConfirmPlan,
        answer(Answer::Unknown),
        Command::FinishReading {
            thoughts: "A shared reference does not move ownership".into(),
        },
        Command::OpenCheck,
    ]
    .into_iter()
    .enumerate()
    {
        let (next, e) = step(&s, c, n as u64);
        s = next;
        history.extend(e);
    }
    assert_eq!(s.phase, Phase::Check);
    (s, history)
}
#[test]
fn independent_state_trace_separates_prediction_check_and_retention() {
    let (s, mut events) = check_ready();
    assert_eq!(s.revision, 6);
    assert_eq!(s.observations.len(), 1);
    assert_eq!(s.observations[0].kind, AttemptKind::Prediction);
    assert_eq!(s.estimate().unaided_checks, 0);
    let (checked, batch) = step(&s, answer(Answer::DoesNotCompile), 6);
    events.extend(batch);
    assert_eq!(checked.phase, Phase::Evidence);
    assert_eq!(checked.revision, 8);
    assert_eq!(checked.estimate().unaided_checks, 1);
    assert_eq!(checked.next_action().kind, ActionKind::Transfer);
    assert_eq!(checked.next_action().question, "shared");
    assert!(checked.estimate().uncertainty.contains("Retention"));
    assert_eq!(replay(&events).unwrap(), checked);
}
#[test]
fn all_joint_premises_are_necessary() {
    let argument = joint_argument();
    let p = &argument.premises;
    for mask in 0..4 {
        let established = p
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1 << i) != 0)
            .map(|(_, v)| v.clone())
            .collect();
        assert_eq!(argument.fires(&established), mask == 3);
    }
    assert!(
        !JointArgument {
            id: "empty".into(),
            scope: "test fixture".into(),
            premises: vec![],
            conclusion: argument.conclusion
        }
        .fires(&BTreeSet::new())
    );
}
#[test]
fn plans_are_dags_but_argument_graph_may_cycle() {
    let mut p = plan();
    p.validate().unwrap();
    p.requires.push((p.nodes[2].clone(), p.nodes[0].clone()));
    assert!(p.validate().is_err());
    let a = ClaimVersion {
        id: "a".into(),
        version: 1,
    };
    let b = ClaimVersion {
        id: "b".into(),
        version: 1,
    };
    let claims = [a.clone(), b.clone()].into_iter().collect();
    JointArgument {
        id: "a-b".into(),
        scope: "test fixture".into(),
        premises: vec![a.clone()],
        conclusion: b.clone(),
    }
    .validate(&claims)
    .unwrap();
    JointArgument {
        id: "b-a".into(),
        scope: "test fixture".into(),
        premises: vec![b],
        conclusion: a,
    }
    .validate(&claims)
    .unwrap();
}
#[test]
fn closed_checks_and_bounded_hints_cannot_be_bypassed() {
    assert!(decide(&Session::default(), &answer(Answer::Compiles), "bad", 0).is_err());
    let (s, _) = check_ready();
    let (s, _) = step(&s, Command::Hint, 10);
    let (s, _) = step(&s, Command::Hint, 11);
    assert!(decide(&s, &Command::Hint, "third", 12).is_err());
    let (s, _) = step(&s, answer(Answer::DoesNotCompile), 13);
    assert_eq!(s.next_action().kind, ActionKind::UnaidedRetry);
    assert_eq!(s.estimate().assisted_checks, 1);
    assert_eq!(s.estimate().unaided_checks, 0);
}
#[test]
fn correction_is_append_only_and_retraction_rebuilds_estimate() {
    let (s, mut events) = check_ready();
    let (s, e) = step(&s, answer(Answer::Compiles), 20);
    events.extend(e);
    assert_eq!(s.next_action().kind, ActionKind::Repair);
    let original = s.observations.last().unwrap().id.clone();
    let (s, e) = step(
        &s,
        Command::Correct {
            observation: original.clone(),
            answer: Answer::DoesNotCompile,
            confidence: Some(73),
            reasoning: "I traced the move".into(),
        },
        21,
    );
    events.extend(e);
    assert!(s.retracted.contains(&original));
    assert_eq!(s.observations.len(), 3);
    assert_eq!(s.observations.last().unwrap().replaces, Some(original));
    assert_eq!(s.estimate().unaided_checks, 1);
    let replacement = s.observations.last().unwrap().id.clone();
    let (s, e) = step(
        &s,
        Command::Retract {
            observation: replacement,
            reason: "This was recalled from the explanation".into(),
        },
        22,
    );
    events.extend(e);
    assert_eq!(s.estimate().unaided_checks, 0);
    assert_eq!(s.next_action().kind, ActionKind::UnaidedRetry);
    assert_eq!(replay(&events).unwrap(), s);
}
#[test]
fn immutable_spans_fail_closed_on_corruption() {
    let docs = sources();
    for span in spans() {
        let d = docs.iter().find(|d| d.id == span.document).unwrap();
        assert!(span.text(d).is_ok());
        let mut changed = d.clone();
        changed.content.push('!');
        assert!(span.text(&changed).is_err());
    }
}
#[test]
fn forged_evaluation_and_partial_batch_are_rejected() {
    let (s, mut history) = check_ready();
    let events = decide(&s, &answer(Answer::DoesNotCompile), "check", 999).unwrap();
    history.push(events[0].clone());
    assert!(replay(&history).is_err());
    let observed = apply(&s, &events[0]).unwrap();
    let mut forged = events[1].clone();
    if let EventKind::Evaluated(ref mut e) = forged.kind {
        e.rubric_version = "made-up".into();
    }
    assert!(apply(&observed, &forged).is_err());
}
#[test]
fn optional_confidence_has_bounds_and_free_text_is_not_graded() {
    let (s, _) = check_ready();
    assert!(
        decide(
            &s,
            &Command::Answer {
                answer: Answer::Compiles,
                confidence: Some(101),
                reasoning: "".into()
            },
            "bad-confidence",
            0
        )
        .is_err()
    );
    for reasoning in [
        "The answer is secretly correct",
        "<script>alert(1)</script>",
        "",
    ] {
        let (n, _) = step(
            &s,
            Command::Answer {
                answer: Answer::Compiles,
                confidence: None,
                reasoning: reasoning.into(),
            },
            44,
        );
        assert_eq!(n.evaluations.last().unwrap().outcome, Outcome::Incorrect);
        assert_eq!(n.observations.last().unwrap().reasoning, reasoning);
    }
}

#[test]
fn evaluation_is_bound_to_exact_question_and_rubric_artifacts() {
    let (s, _) = check_ready();
    let (s, _) = step(&s, answer(Answer::DoesNotCompile), 99);
    let observation = s.observations.last().unwrap();
    let evaluation = s.evaluations.last().unwrap();
    let question = question(&observation.question).unwrap();
    assert_eq!(observation.question_hash, question.artifact_hash());
    assert_eq!(evaluation.question_hash, question.artifact_hash());
    assert_eq!(evaluation.rubric_hash, question.rubric_hash());
    let mut changed = question.clone();
    changed.code.push_str("// changed artifact");
    assert_ne!(changed.artifact_hash(), question.artifact_hash());
    let mut forged = observation.clone();
    forged.question_hash = changed.artifact_hash();
    assert!(evaluate(&forged).is_err());
}

#[test]
fn source_consultation_is_assistance_and_is_bound_to_the_observation() {
    let (s, _) = check_ready();
    let (s, _) = step(&s, Command::ConsultSource, 200);
    assert!(s.source_consulted);
    assert!(decide(&s, &Command::ConsultSource, "again", 201).is_err());
    let (s, _) = step(&s, answer(Answer::DoesNotCompile), 202);
    let observation = s.observations.last().unwrap();
    assert!(observation.source_consulted);
    assert_eq!(s.estimate().unaided_checks, 0);
    assert_eq!(s.estimate().assisted_checks, 1);
    assert_eq!(s.next_action().kind, ActionKind::UnaidedRetry);
    let mut forged = observation.clone();
    forged.source_consulted = false;
    assert_ne!(
        evaluate(&forged).unwrap().observation_hash,
        s.evaluations.last().unwrap().observation_hash
    );
}

#[test]
fn all_four_unaided_examples_recommend_delayed_retrieval_without_mastery() {
    let (mut s, _) = check_ready();
    let mut operation = 300;
    for (id, answer) in [
        ("move", Answer::DoesNotCompile),
        ("shared", Answer::Compiles),
        ("overlap", Answer::DoesNotCompile),
        ("last-use", Answer::Compiles),
    ] {
        assert_eq!(s.question, id);
        (s, _) = step(
            &s,
            Command::Answer {
                answer,
                confidence: None,
                reasoning: "Independent compiler-backed expected answer".into(),
            },
            operation,
        );
        operation += 1;
        (s, _) = step(&s, Command::ReviewEvidence, operation);
        operation += 1;
        if id == "last-use" {
            let next = s.next_action();
            assert_eq!(next.kind, ActionKind::DelayedRetrieval);
            assert_eq!(next.due_at_ms, Some(s.last_at_ms + 86_400_000));
        } else {
            assert_eq!(s.next_action().kind, ActionKind::Transfer);
        }
        (s, _) = step(
            &s,
            Command::Continue {
                override_kind: None,
            },
            operation,
        );
        operation += 1;
    }
    assert_eq!(s.phase, Phase::Paused);
    assert_eq!(s.estimate().unaided_checks, 4);
    assert!(s.estimate().uncertainty.contains("untested"));
}
#[test]
fn correcting_a_prediction_preserves_its_kind_and_returns_to_evidence() {
    let (s, _) = check_ready();
    let (s, _) = step(&s, answer(Answer::DoesNotCompile), 400);
    let predicted = s.observations[0].id.clone();
    let (s, _) = step(
        &s,
        Command::Correct {
            observation: predicted.clone(),
            answer: Answer::Compiles,
            confidence: None,
            reasoning: "Corrected an entry error in my original prediction".into(),
        },
        401,
    );
    assert!(s.retracted.contains(&predicted));
    assert_eq!(s.observations.last().unwrap().kind, AttemptKind::Prediction);
    assert_eq!(s.phase, Phase::Evidence);
    assert_eq!(s.estimate().predictions, 1);
    assert_eq!(s.estimate().unaided_checks, 1);
}

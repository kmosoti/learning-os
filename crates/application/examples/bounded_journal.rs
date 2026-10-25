//! Synthetic maximal journal; never touches a learner's store.
use learningos_application::*;
use learningos_domain::*;
use std::{hint::black_box, time::Instant};
fn command(session: &Session) -> Command {
    match session.phase {
        Phase::Goal => Command::ChooseGoal {
            goal: "Synthetic journal capacity measurement".into(),
        },
        Phase::Plan => Command::ConfirmPlan,
        Phase::Predict => Command::Answer {
            answer: Answer::Unknown,
            confidence: None,
            reasoning: "Independent synthetic prediction".into(),
        },
        Phase::Teach => Command::FinishReading {
            thoughts: "Synthetic reading thoughts".into(),
        },
        Phase::CheckClosed => Command::OpenCheck,
        Phase::Check => Command::Answer {
            answer: Answer::Compiles,
            confidence: Some(40),
            reasoning: "Synthetic incorrect answer; bounded fixture".into(),
        },
        Phase::Evidence => Command::ReviewEvidence,
        Phase::NextAction => Command::Continue {
            override_kind: None,
        },
        Phase::Paused => unreachable!(),
    }
}
fn summary(mut samples: Vec<f64>) -> serde_json::Value {
    samples.sort_by(f64::total_cmp);
    serde_json::json!({"samples":samples.len(),"p50_ms":samples[samples.len()/2],"p95_ms":samples[(samples.len()*95/100).min(samples.len()-1)]})
}
fn main() {
    let mut session = Session::default();
    let mut events = vec![];
    let mut receipts = vec![];
    let mut number = 0;
    let mut decision_state = None;
    loop {
        let request = Request {
            schema: 1,
            id: format!("synthetic-{number:05}"),
            expected: session.revision,
            at_ms: 1700000000000 + number,
            command: command(&session),
        };
        let Ok(batch) = decide(&session, &request.command, &request.id, request.at_ms) else {
            break;
        };
        if session.revision >= 9990 && decision_state.is_none() {
            decision_state = Some(session.clone());
        }
        for event in &batch {
            session = apply(&session, event).unwrap();
        }
        events.extend(batch);
        receipts.push(Receipt {
            request,
            revision: session.revision,
        });
        number += 1;
    }
    let backup = Backup {
        schema: 1,
        fixture: FIXTURE_VERSION.into(),
        fixture_digest: fixture_digest(),
        event_digest: digest(&serde_json::to_string(&events).unwrap()),
        events,
        receipts,
    };
    let encoded = serde_json::to_string(&backup).unwrap();
    let state = decision_state.unwrap();
    let cmd = command(&state);
    let mut decision = vec![];
    let mut next = vec![];
    for _ in 0..30 {
        let start = Instant::now();
        black_box(decide(&state, &cmd, "measurement", 1900000000000).unwrap());
        decision.push(start.elapsed().as_secs_f64() * 1000.0);
        let start = Instant::now();
        black_box(state.next_action());
        next.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    let start = Instant::now();
    let replayed = replay(&backup.events).unwrap();
    let replay_ms = start.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(replayed, session);
    let start = Instant::now();
    assert_eq!(backup.validate().unwrap(), session);
    let backup_validation_ms = start.elapsed().as_secs_f64() * 1000.0;
    if let Some(path) = std::env::args().nth(1) {
        std::fs::write(path, &encoded).unwrap();
    }
    println!(
        "{}",
        serde_json::json!({"kind":"synthetic-largest-event-count","events":session.revision,"observations":session.observations.len(),"receipts":backup.receipts.len(),"compact_backup_bytes":encoded.len(),"snapshot_bytes":serde_json::to_string(&session).unwrap().len(),"decision_revision":state.revision,"decision":summary(decision),"next_action":summary(next),"full_replay_ms":replay_ms,"full_backup_validation_ms":backup_validation_ms,"build":"native release; see rust-toolchain.toml/Cargo.lock; fixture digest identifies input","fixture_digest":fixture_digest()})
    );
}

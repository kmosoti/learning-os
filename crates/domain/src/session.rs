use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Phase {
    #[default]
    Goal,
    Plan,
    Predict,
    Teach,
    CheckClosed,
    Check,
    Evidence,
    NextAction,
    Paused,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttemptKind {
    Prediction,
    Check,
    Transfer,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Observation {
    pub id: String,
    pub question: String,
    pub question_version: u32,
    pub question_hash: String,
    pub kind: AttemptKind,
    pub answer: Answer,
    pub confidence: Option<u8>,
    pub reasoning: String,
    pub reading_thoughts: String,
    pub hints: u8,
    pub source_consulted: bool,
    pub replaces: Option<String>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome {
    Correct,
    Incorrect,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evaluation {
    pub observation: String,
    pub observation_hash: String,
    pub question: String,
    pub question_version: u32,
    pub question_hash: String,
    pub rubric_hash: String,
    pub rubric_version: String,
    pub outcome: Outcome,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionKind {
    Repair,
    UnaidedRetry,
    Transfer,
    DelayedRetrieval,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextAction {
    pub kind: ActionKind,
    pub question: String,
    pub reason: String,
    pub due_at_ms: Option<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    ChooseGoal {
        goal: String,
    },
    ConfirmPlan,
    Answer {
        answer: Answer,
        confidence: Option<u8>,
        reasoning: String,
    },
    FinishReading {
        thoughts: String,
    },
    OpenCheck,
    ConsultSource,
    Hint,
    ReviewEvidence,
    Continue {
        override_kind: Option<ActionKind>,
    },
    Retract {
        observation: String,
        reason: String,
    },
    Correct {
        observation: String,
        answer: Answer,
        confidence: Option<u8>,
        reasoning: String,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKind {
    GoalChosen {
        goal: String,
        fixture: String,
        plan: Plan,
    },
    PlanConfirmed,
    Observed(Observation),
    Evaluated(Evaluation),
    ReadingFinished {
        thoughts: String,
    },
    CheckOpened,
    SourceConsulted,
    HintUsed,
    EvidenceReviewed,
    ActionChosen {
        recommended: NextAction,
        chosen: ActionKind,
    },
    Retracted {
        observation: String,
        reason: String,
    },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub at_ms: u64,
    pub policy: String,
    pub kind: EventKind,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Session {
    pub revision: u64,
    pub phase: Phase,
    pub goal: Option<String>,
    pub plan: Option<Plan>,
    pub question: String,
    pub observations: Vec<Observation>,
    pub evaluations: Vec<Evaluation>,
    pub retracted: BTreeSet<String>,
    pub reading_thoughts: String,
    pub hints: u8,
    pub source_consulted: bool,
    pub last_at_ms: u64,
    pub event_ids: BTreeSet<String>,
    pub transfer: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Estimate {
    pub unaided_checks: usize,
    pub assisted_checks: usize,
    pub predictions: usize,
    pub uncertainty: String,
}
impl Session {
    pub fn estimate(&self) -> Estimate {
        let active = |o: &&Observation| !self.retracted.contains(&o.id);
        let correct = |o: &&Observation| {
            self.evaluations
                .iter()
                .any(|e| e.observation == o.id && e.outcome == Outcome::Correct)
        };
        Estimate{unaided_checks:self.observations.iter().filter(active).filter(|o|o.kind!=AttemptKind::Prediction&&o.hints==0&&!o.source_consulted).filter(correct).count(),assisted_checks:self.observations.iter().filter(active).filter(|o|o.kind!=AttemptKind::Prediction&&(o.hints>0||o.source_consulted)).filter(correct).count(),predictions:self.observations.iter().filter(active).filter(|o|o.kind==AttemptKind::Prediction).count(),uncertainty:"Limited to these tasks and conditions. Retention and broader transfer remain untested.".into()}
    }
    pub fn next_action(&self) -> NextAction {
        let last = self
            .observations
            .iter()
            .rev()
            .find(|o| o.kind != AttemptKind::Prediction && !self.retracted.contains(&o.id));
        let Some(o) = last else {
            return NextAction {
                kind: ActionKind::UnaidedRetry,
                question: if self.question.is_empty() {
                    "move".into()
                } else {
                    self.question.clone()
                },
                reason: "No active check evidence remains. Try a fresh answer without hints."
                    .into(),
                due_at_ms: None,
            };
        };
        let outcome = self
            .evaluations
            .iter()
            .find(|e| e.observation == o.id)
            .map(|e| e.outcome);
        let (kind, question, reason) = if outcome != Some(Outcome::Correct) {
            (
                ActionKind::Repair,
                o.question.clone(),
                "Review the prerequisite behind this example, then check again.",
            )
        } else if o.hints > 0 || o.source_consulted {
            (
                ActionKind::UnaidedRetry,
                o.question.clone(),
                "The answer used a hint or consulted source material. An unaided retry will provide different evidence.",
            )
        } else {
            let done: BTreeSet<_> = self
                .observations
                .iter()
                .filter(|a| {
                    a.kind != AttemptKind::Prediction
                        && a.hints == 0
                        && !a.source_consulted
                        && !self.retracted.contains(&a.id)
                        && self
                            .evaluations
                            .iter()
                            .any(|e| e.observation == a.id && e.outcome == Outcome::Correct)
                })
                .map(|a| a.question.as_str())
                .collect();
            match ["move", "shared", "overlap", "last-use"]
                .into_iter()
                .find(|q| !done.contains(q))
            {
                Some(q) => (
                    ActionKind::Transfer,
                    q.into(),
                    "Try a changed example. A correct answer here does not establish retention.",
                ),
                None => (
                    ActionKind::DelayedRetrieval,
                    "move".into(),
                    "These four examples have unaided evidence. Return tomorrow to test retention; this is a recommendation, not mastery.",
                ),
            }
        };
        NextAction {
            kind,
            question,
            reason: reason.into(),
            due_at_ms: if kind == ActionKind::DelayedRetrieval {
                Some(self.last_at_ms.saturating_add(86_400_000))
            } else {
                None
            },
        }
    }
}
pub fn evaluate(o: &Observation) -> Result<Evaluation, DomainError> {
    let q = question(&o.question)?;
    if o.question_version != q.version || o.question_hash != q.artifact_hash() {
        return Err(DomainError::Version("question".into()));
    }
    Ok(Evaluation {
        observation: o.id.clone(),
        observation_hash: digest(
            &serde_json::to_string(o).map_err(|e| DomainError::Invalid(e.to_string()))?,
        ),
        question: q.id.clone(),
        question_version: q.version,
        question_hash: q.artifact_hash(),
        rubric_hash: q.rubric_hash(),
        rubric_version: RUBRIC_VERSION.into(),
        outcome: if o.answer == Answer::Unknown {
            Outcome::Unknown
        } else if o.answer == q.answer {
            Outcome::Correct
        } else {
            Outcome::Incorrect
        },
    })
}
/// IDs and time are supplied at the boundary. Event order, not wall-clock order, is authoritative.
fn event_batch(
    s: &Session,
    c: &Command,
    operation: &str,
    at_ms: u64,
) -> Result<Vec<Event>, DomainError> {
    bounded(operation, 128)?;
    if operation.is_empty() {
        return Err(DomainError::Invalid("empty operation id".into()));
    }
    let observation =
        |answer, confidence, reasoning: &String, replaces: Option<String>| Observation {
            id: format!("{operation}/observation"),
            question: s.question.clone(),
            question_version: 1,
            question_hash: question(&s.question)
                .map(|q| q.artifact_hash())
                .unwrap_or_default(),
            kind: if s.phase == Phase::Predict {
                AttemptKind::Prediction
            } else if s.transfer {
                AttemptKind::Transfer
            } else {
                AttemptKind::Check
            },
            answer,
            confidence,
            reasoning: reasoning.clone(),
            reading_thoughts: s.reading_thoughts.clone(),
            hints: s.hints,
            source_consulted: s.source_consulted,
            replaces,
        };
    let kinds = match c {
        Command::ChooseGoal { goal } => vec![EventKind::GoalChosen {
            goal: goal.clone(),
            fixture: FIXTURE_VERSION.into(),
            plan: plan(),
        }],
        Command::ConfirmPlan => vec![EventKind::PlanConfirmed],
        Command::Answer {
            answer,
            confidence,
            reasoning,
        } => {
            let o = observation(*answer, *confidence, reasoning, None);
            vec![
                EventKind::Observed(o.clone()),
                EventKind::Evaluated(evaluate(&o)?),
            ]
        }
        Command::FinishReading { thoughts } => vec![EventKind::ReadingFinished {
            thoughts: thoughts.clone(),
        }],
        Command::OpenCheck => vec![EventKind::CheckOpened],
        Command::ConsultSource => vec![EventKind::SourceConsulted],
        Command::Hint => vec![EventKind::HintUsed],
        Command::ReviewEvidence => vec![EventKind::EvidenceReviewed],
        Command::Continue { override_kind } => {
            let recommended = s.next_action();
            let chosen = override_kind.unwrap_or(recommended.kind);
            if ![
                recommended.kind,
                ActionKind::Repair,
                ActionKind::UnaidedRetry,
            ]
            .contains(&chosen)
            {
                return Err(DomainError::Invalid("unsupported override".into()));
            }
            vec![EventKind::ActionChosen {
                recommended,
                chosen,
            }]
        }
        Command::Retract {
            observation,
            reason,
        } => vec![EventKind::Retracted {
            observation: observation.clone(),
            reason: reason.clone(),
        }],
        Command::Correct {
            observation: target,
            answer,
            confidence,
            reasoning,
        } => {
            let old = s
                .observations
                .iter()
                .find(|o| o.id == *target)
                .ok_or_else(|| DomainError::Invalid("unknown observation".into()))?;
            let mut o = observation(*answer, *confidence, reasoning, Some(target.clone()));
            o.question = old.question.clone();
            o.question_hash = old.question_hash.clone();
            o.kind = old.kind;
            o.hints = old.hints;
            o.source_consulted = old.source_consulted;
            o.reading_thoughts = old.reading_thoughts.clone();
            vec![
                EventKind::Retracted {
                    observation: target.clone(),
                    reason: "Learner correction; original retained".into(),
                },
                EventKind::Observed(o.clone()),
                EventKind::Evaluated(evaluate(&o)?),
            ]
        }
    };
    let events: Vec<_> = kinds
        .into_iter()
        .enumerate()
        .map(|(i, kind)| Event {
            id: format!("{operation}/{i}"),
            at_ms,
            policy: POLICY_VERSION.into(),
            kind,
        })
        .collect();
    Ok(events)
}
/// Ownership of the candidate allows replay/import to advance without cloning the
/// entire evidence history for each receipt. An error consumes this candidate.
pub fn transition(
    mut state: Session,
    command: &Command,
    operation: &str,
    at_ms: u64,
) -> Result<(Session, Vec<Event>), DomainError> {
    let events = event_batch(&state, command, operation, at_ms)?;
    for event in &events {
        evolve(&mut state, event)?;
    }
    Ok((state, events))
}
pub fn decide(
    state: &Session,
    command: &Command,
    operation: &str,
    at_ms: u64,
) -> Result<Vec<Event>, DomainError> {
    Ok(transition(state.clone(), command, operation, at_ms)?.1)
}

pub fn apply(s: &Session, e: &Event) -> Result<Session, DomainError> {
    let mut n = s.clone();
    evolve(&mut n, e)?;
    Ok(n)
}
fn evolve(n: &mut Session, e: &Event) -> Result<(), DomainError> {
    if e.policy != POLICY_VERSION {
        return Err(DomainError::Version(e.policy.clone()));
    }
    if n.revision >= 10_000 || e.id.is_empty() || e.id.len() > 160 || n.event_ids.contains(&e.id) {
        return Err(DomainError::Invalid("event bound/identity".into()));
    }
    let phase = n.phase;
    let transition = || DomainError::Transition(format!("{phase:?}"));
    match &e.kind {
        EventKind::GoalChosen {
            goal,
            fixture,
            plan: p,
        } => {
            if n.phase != Phase::Goal {
                return Err(transition());
            }
            bounded(goal, 240)?;
            if goal.trim().is_empty() || fixture != FIXTURE_VERSION {
                return Err(DomainError::Invalid("goal/fixture".into()));
            }
            p.validate()?;
            if *p != plan() {
                return Err(DomainError::Invalid("unapproved fixture plan".into()));
            }
            n.goal = Some(goal.clone());
            n.plan = Some(p.clone());
            n.question = "move".into();
            n.phase = Phase::Plan;
        }
        EventKind::PlanConfirmed => {
            if n.phase != Phase::Plan {
                return Err(transition());
            }
            n.phase = Phase::Predict;
        }
        EventKind::Observed(o) => {
            let correction = o.replaces.as_ref().is_some_and(|id| {
                n.retracted.contains(id)
                    && n.observations.iter().any(|old| {
                        old.id == *id
                            && old.question == o.question
                            && old.kind == o.kind
                            && old.hints == o.hints
                            && old.source_consulted == o.source_consulted
                            && old.reading_thoughts == o.reading_thoughts
                    })
            });
            if !(matches!(n.phase, Phase::Predict | Phase::Check)
                || n.phase == Phase::Evidence && correction)
            {
                return Err(transition());
            }
            if n.observations.len() >= 2_000
                || n.observations.iter().any(|x| x.id == o.id)
                || o.id.is_empty()
                || o.id.len() > 160
                || o.confidence.is_some_and(|v| v > 100)
                || o.hints > 2
            {
                return Err(DomainError::Invalid("observation bounds".into()));
            }
            if !correction
                && (o.question != n.question
                    || o.hints != n.hints
                    || o.source_consulted != n.source_consulted
                    || o.reading_thoughts != n.reading_thoughts
                    || o.kind
                        != if n.phase == Phase::Predict {
                            AttemptKind::Prediction
                        } else if n.transfer {
                            AttemptKind::Transfer
                        } else {
                            AttemptKind::Check
                        })
            {
                return Err(DomainError::Invalid("observation context".into()));
            }
            if n.observations.len() != n.evaluations.len() {
                return Err(DomainError::Invalid("pending evaluation".into()));
            }
            bounded(&o.reasoning, 4096)?;
            bounded(&o.reading_thoughts, 4096)?;
            let _ = evaluate(o)?;
            n.observations.push(o.clone());
        }
        EventKind::Evaluated(v) => {
            let o = n
                .observations
                .last()
                .ok_or_else(|| DomainError::Invalid("evaluation without observation".into()))?;
            if *v != evaluate(o)? || n.evaluations.iter().any(|x| x.observation == v.observation) {
                return Err(DomainError::Invalid(
                    "evaluation identity/version/content".into(),
                ));
            }
            n.evaluations.push(v.clone());
            n.phase = if o.kind == AttemptKind::Prediction && o.replaces.is_none() {
                Phase::Teach
            } else {
                Phase::Evidence
            };
        }
        EventKind::ReadingFinished { thoughts } => {
            if n.phase != Phase::Teach {
                return Err(transition());
            }
            bounded(thoughts, 4096)?;
            n.reading_thoughts = thoughts.clone();
            n.hints = 0;
            n.source_consulted = false;
            n.phase = Phase::CheckClosed;
        }
        EventKind::CheckOpened => {
            if n.phase != Phase::CheckClosed {
                return Err(transition());
            }
            n.phase = Phase::Check;
        }
        EventKind::SourceConsulted => {
            if !matches!(n.phase, Phase::Predict | Phase::CheckClosed | Phase::Check)
                || n.source_consulted
            {
                return Err(transition());
            }
            n.source_consulted = true;
        }
        EventKind::HintUsed => {
            if n.phase != Phase::Check || n.hints >= 2 {
                return Err(transition());
            }
            n.hints += 1;
        }
        EventKind::EvidenceReviewed => {
            if n.phase != Phase::Evidence {
                return Err(transition());
            }
            n.phase = Phase::NextAction;
        }
        EventKind::ActionChosen {
            recommended,
            chosen,
        } => {
            if n.phase != Phase::NextAction
                || *recommended != n.next_action()
                || ![
                    recommended.kind,
                    ActionKind::Repair,
                    ActionKind::UnaidedRetry,
                ]
                .contains(chosen)
            {
                return Err(transition());
            }
            n.question = if *chosen == recommended.kind {
                recommended.question.clone()
            } else {
                n.question.clone()
            };
            n.hints = 0;
            n.source_consulted = false;
            n.transfer = *chosen == ActionKind::Transfer;
            n.phase = match chosen {
                ActionKind::Repair => Phase::Teach,
                ActionKind::UnaidedRetry | ActionKind::Transfer => Phase::Check,
                ActionKind::DelayedRetrieval => Phase::Paused,
            };
        }
        EventKind::Retracted {
            observation,
            reason,
        } => {
            if !matches!(n.phase, Phase::Evidence | Phase::NextAction | Phase::Paused)
                || n.retracted.contains(observation)
                || !n.observations.iter().any(|o| o.id == *observation)
            {
                return Err(transition());
            }
            bounded(reason, 512)?;
            if reason.trim().is_empty() {
                return Err(DomainError::Invalid("retraction reason".into()));
            }
            n.retracted.insert(observation.clone());
            n.phase = Phase::Evidence;
        }
    }
    n.event_ids.insert(e.id.clone());
    n.revision += 1;
    n.last_at_ms = e.at_ms;
    Ok(())
}
pub fn replay(events: &[Event]) -> Result<Session, DomainError> {
    let mut state = Session::default();
    for e in events {
        evolve(&mut state, e)?;
    }
    if state.observations.len() != state.evaluations.len() {
        return Err(DomainError::Invalid(
            "incomplete observation/evaluation batch".into(),
        ));
    }
    Ok(state)
}

use learningos_application::*;
use learningos_domain::*;
use std::{
    cell::RefCell,
    collections::VecDeque,
    future::Future,
    task::{Context, Poll, Waker},
};
fn run<F: Future>(future: F) -> F::Output {
    let waker = Waker::noop();
    let mut cx = Context::from_waker(waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(out) => out,
        Poll::Pending => panic!("fake port unexpectedly suspended"),
    }
}
#[derive(Default)]
struct Memory(RefCell<(Vec<Event>, Vec<Receipt>)>);
impl Journal for Memory {
    async fn events(&self) -> Result<Vec<Event>, AppError> {
        Ok(self.0.borrow().0.clone())
    }
    async fn receipt(&self, id: &str) -> Result<Option<Receipt>, AppError> {
        Ok(self
            .0
            .borrow()
            .1
            .iter()
            .find(|r| r.request.id == id)
            .cloned())
    }
    async fn commit(&self, c: &Commit) -> Result<Receipt, AppError> {
        let mut data = self.0.borrow_mut();
        if let Some(r) = data.1.iter().find(|r| r.request.id == c.request.id) {
            return if r.request == c.request {
                Ok(r.clone())
            } else {
                Err(AppError::Conflict)
            };
        }
        if data.0.len() as u64 != c.request.expected {
            return Err(AppError::Stale);
        }
        data.0.extend(c.events.clone());
        data.1.push(c.receipt.clone());
        Ok(c.receipt.clone())
    }
}
fn request(id: &str, expected: u64, command: Command) -> Request {
    Request {
        schema: 1,
        id: id.into(),
        expected,
        at_ms: 42,
        command,
    }
}
#[test]
fn lost_ack_retry_and_conflicting_id_are_distinct() {
    let m = Memory::default();
    let first = request(
        "same",
        0,
        Command::ChooseGoal {
            goal: "Rust".into(),
        },
    );
    let ack = run(execute(&m, first.clone())).unwrap();
    run(execute(&m, request("later", 1, Command::ConfirmPlan))).unwrap();
    assert_eq!(run(execute(&m, first.clone())).unwrap(), ack);
    assert_eq!(m.0.borrow().0.len(), 2);
    let mut conflict = first;
    conflict.command = Command::ChooseGoal {
        goal: "Different".into(),
    };
    assert_eq!(run(execute(&m, conflict)).unwrap_err(), AppError::Conflict);
    assert_eq!(
        run(execute(&m, request("stale", 0, Command::ConfirmPlan))).unwrap_err(),
        AppError::Stale
    );
}
#[test]
fn backup_binds_receipts_and_rejects_tampering() {
    let m = Memory::default();
    run(execute(
        &m,
        request(
            "one",
            0,
            Command::ChooseGoal {
                goal: "Rust".into(),
            },
        ),
    ))
    .unwrap();
    let data = m.0.borrow();
    let mut backup = Backup {
        schema: 1,
        fixture: FIXTURE_VERSION.into(),
        fixture_digest: fixture_digest(),
        events: data.0.clone(),
        receipts: data.1.clone(),
        event_digest: digest(&serde_json::to_string(&data.0).unwrap()),
    };
    let encoded = serde_json::to_string(&backup).unwrap();
    assert_eq!(Backup::parse(&encoded).unwrap(), backup);
    backup.receipts[0].request.at_ms = 43;
    assert!(backup.validate().is_err());
    let mut value: serde_json::Value = serde_json::from_str(&encoded).unwrap();
    value["unexpected"] = true.into();
    assert!(Backup::parse(&value.to_string()).is_err());
}
struct Scripted(RefCell<VecDeque<Result<String, ProposalError>>>);
impl ModelPort for Scripted {
    async fn propose(&self, _: &ModelRequest) -> Result<String, ProposalError> {
        self.0
            .borrow_mut()
            .pop_front()
            .unwrap_or(Err(ProposalError::Unavailable))
    }
}
#[test]
fn scripted_provider_cannot_bypass_cancel_revision_or_shape_validation() {
    let req = ModelRequest {
        id: "new".into(),
        base_revision: 4,
        cancelled: false,
    };
    let good = serde_json::to_string(&PlanProposal {
        request_id: "new".into(),
        base_revision: 4,
        plan: plan(),
    })
    .unwrap();
    let old = serde_json::to_string(&PlanProposal {
        request_id: "old".into(),
        base_revision: 3,
        plan: plan(),
    })
    .unwrap();
    let model = Scripted(RefCell::new(VecDeque::from([
        Err(ProposalError::Unavailable),
        Ok("not json".into()),
        Ok(old),
        Ok(good.clone()),
    ])));
    assert_eq!(
        run(model.propose(&req)).unwrap_err(),
        ProposalError::Unavailable
    );
    for _ in 0..2 {
        let json = run(model.propose(&req)).unwrap();
        assert!(validate_proposal(&req, &json, 4).is_err());
    }
    assert!(validate_proposal(&req, &run(model.propose(&req)).unwrap(), 4).is_ok());
    let mut canceled = req.clone();
    canceled.cancelled = true;
    assert!(validate_proposal(&canceled, &good, 4).is_err());
    assert!(validate_proposal(&req, &good, 5).is_err());
}

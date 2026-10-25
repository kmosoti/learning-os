//! Use cases and ports. Framework/runtime/storage types never enter the domain.
use learningos_domain::*;
use serde::{Deserialize, Serialize};
use std::future::Future;
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(
        "This tab is behind another save. Your draft is kept; reload local state before retrying."
    )]
    Stale,
    #[error("Operation ID was already used for a different request.")]
    Conflict,
    #[error("Local storage failed: {0}. Your answer has not been acknowledged.")]
    Storage(String),
    #[error("Unsupported or damaged backup: {0}")]
    Backup(String),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
    pub schema: u32,
    pub id: String,
    pub expected: u64,
    pub at_ms: u64,
    pub command: Command,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub request: Request,
    pub revision: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commit {
    pub request: Request,
    pub events: Vec<Event>,
    pub projection: Session,
    pub receipt: Receipt,
}
pub trait Journal {
    fn events(&self) -> impl Future<Output = Result<Vec<Event>, AppError>>;
    fn session(&self) -> impl Future<Output = Result<Session, AppError>> {
        async { Ok(replay(&self.events().await?)?) }
    }
    fn receipt(&self, id: &str) -> impl Future<Output = Result<Option<Receipt>, AppError>>;
    /// Atomic duplicate lookup BEFORE CAS, event batch, projection and receipt.
    fn commit(&self, commit: &Commit) -> impl Future<Output = Result<Receipt, AppError>>;
}
pub async fn execute<J: Journal>(journal: &J, request: Request) -> Result<Receipt, AppError> {
    if request.schema != 1 {
        return Err(DomainError::Version("command schema".into()).into());
    }
    if let Some(receipt) = journal.receipt(&request.id).await? {
        return if receipt.request == request {
            Ok(receipt)
        } else {
            Err(AppError::Conflict)
        };
    }
    let projection = journal.session().await?;
    if projection.revision != request.expected {
        return Err(AppError::Stale);
    }
    let (projection, events) =
        transition(projection, &request.command, &request.id, request.at_ms)?;
    let receipt = Receipt {
        request: request.clone(),
        revision: projection.revision,
    };
    journal
        .commit(&Commit {
            request,
            events,
            projection,
            receipt,
        })
        .await
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Backup {
    pub schema: u32,
    pub fixture: String,
    pub fixture_digest: String,
    pub events: Vec<Event>,
    pub receipts: Vec<Receipt>,
    pub event_digest: String,
}
impl Backup {
    pub fn validate(&self) -> Result<Session, AppError> {
        if self.schema != 1
            || self.fixture != FIXTURE_VERSION
            || self.fixture_digest != fixture_digest()
            || self.events.len() > 10_000
            || self.receipts.len() > 10_000
        {
            return Err(AppError::Backup("version or size".into()));
        }
        let encoded =
            serde_json::to_string(&self.events).map_err(|e| AppError::Backup(e.to_string()))?;
        if digest(&encoded) != self.event_digest {
            return Err(AppError::Backup("checksum mismatch".into()));
        }
        let session = replay(&self.events)?;
        let mut prefix = Session::default();
        let mut ids = std::collections::BTreeSet::new();
        let mut offset = 0;
        for receipt in &self.receipts {
            let request = &receipt.request;
            if request.schema != 1
                || request.expected != prefix.revision
                || !ids.insert(&request.id)
            {
                return Err(AppError::Backup("receipt ordering/identity".into()));
            }
            let (next, batch) = transition(prefix, &request.command, &request.id, request.at_ms)?;
            if self.events.get(offset..offset + batch.len()) != Some(batch.as_slice()) {
                return Err(AppError::Backup("receipt does not bind event batch".into()));
            }
            prefix = next;
            offset += batch.len();
            if receipt.revision != prefix.revision {
                return Err(AppError::Backup("receipt revision".into()));
            }
        }
        if prefix != session {
            return Err(AppError::Backup("missing receipts".into()));
        }
        Ok(session)
    }
    pub fn parse(json: &str) -> Result<Self, AppError> {
        if json.len() > 8 * 1024 * 1024 {
            return Err(AppError::Backup("8 MiB limit".into()));
        }
        let value: Self =
            serde_json::from_str(json).map_err(|e| AppError::Backup(e.to_string()))?;
        value.validate()?;
        Ok(value)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRequest {
    pub id: String,
    pub base_revision: u64,
    pub cancelled: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanProposal {
    pub request_id: String,
    pub base_revision: u64,
    pub plan: Plan,
}
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProposalError {
    #[error("model unavailable")]
    Unavailable,
    #[error("invalid or stale proposal")]
    Rejected,
}
pub trait ModelPort {
    fn propose(
        &self,
        request: &ModelRequest,
    ) -> impl Future<Output = Result<String, ProposalError>>;
}
/// Proposals never acquire a journal handle. Acceptance only validates bounded data.
pub fn validate_proposal(
    request: &ModelRequest,
    json: &str,
    current_revision: u64,
) -> Result<PlanProposal, ProposalError> {
    if request.cancelled || request.base_revision != current_revision || json.len() > 16_384 {
        return Err(ProposalError::Rejected);
    }
    let proposal: PlanProposal = serde_json::from_str(json).map_err(|_| ProposalError::Rejected)?;
    if proposal.request_id != request.id
        || proposal.base_revision != current_revision
        || proposal.plan.validate().is_err()
    {
        return Err(ProposalError::Rejected);
    }
    Ok(proposal)
}

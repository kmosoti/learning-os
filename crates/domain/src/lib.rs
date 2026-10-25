//! Pure, bounded, deterministic learning decisions. No clock, randomness, database or UI.
mod epistemic;
mod fixture;
mod session;
pub use epistemic::*;
pub use fixture::*;
use serde::{Deserialize, Serialize};
pub use session::*;
use sha2::{Digest, Sha256};
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, Serialize, Deserialize)]
pub enum DomainError {
    #[error("invalid transition: {0}")]
    Transition(String),
    #[error("invalid input: {0}")]
    Invalid(String),
    #[error("unsupported version: {0}")]
    Version(String),
    #[error("invalid provenance: {0}")]
    Provenance(String),
}
pub fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
pub fn bounded(text: &str, max: usize) -> Result<(), DomainError> {
    if text.len() > max {
        Err(DomainError::Invalid(format!("text exceeds {max} bytes")))
    } else {
        Ok(())
    }
}

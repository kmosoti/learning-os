//! Versioned public boundary. No database rows or framework types.
use learningos_domain::{FIXTURE_VERSION, SourceDocument, SourceSpan, sources, spans};
use serde::{Deserialize, Serialize};
pub const SCHEMA: u32 = 1;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessonBundle {
    pub schema: u32,
    pub fixture: String,
    pub artifact_digest: String,
    pub sources: Vec<SourceDocument>,
    pub spans: Vec<SourceSpan>,
}
pub fn lesson_bundle() -> LessonBundle {
    LessonBundle {
        schema: SCHEMA,
        fixture: FIXTURE_VERSION.into(),
        artifact_digest: learningos_domain::fixture_digest(),
        sources: sources(),
        spans: spans(),
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub schema: u32,
    pub code: String,
    pub message: String,
    pub request_id: String,
}

use crate::*;
use serde::{Deserialize, Serialize};
pub const FIXTURE_VERSION: &str = "ownership-1";
pub const POLICY_VERSION: &str = "learning-policy-1";
pub const RUBRIC_VERSION: &str = "rustc-1.99.0-edition2024-1";
pub const BOOK_REVISION: &str = "1500248d8f230566e4ec9f27fcbb8fe9e2898ab1";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Answer {
    Compiles,
    DoesNotCompile,
    Unknown,
}
impl Answer {
    pub fn label(self) -> &'static str {
        match self {
            Self::Compiles => "It compiles",
            Self::DoesNotCompile => "It does not compile",
            Self::Unknown => "I don't know yet",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub version: u32,
    pub code: String,
    pub answer: Answer,
    pub explanation: String,
    pub source_span: String,
    pub compiler_diagnostic: Option<String>,
}
pub fn question(id: &str) -> Result<Question, DomainError> {
    let (code, answer, explanation, span, diagnostic) = match id {
        "move" => (
            include_str!("../../../fixtures/rust/move.rs"),
            Answer::DoesNotCompile,
            "Assigning this String to next moves its ownership. The later use of name is rejected with E0382. This example does not imply that every assignment moves: Copy types behave differently.",
            "ownership-move",
            Some("E0382"),
        ),
        "shared" => (
            include_str!("../../../fixtures/rust/shared.rs"),
            Answer::Compiles,
            "A shared reference borrows the String. It does not take ownership, so the owner remains usable.",
            "borrow-reference",
            None,
        ),
        "overlap" => (
            include_str!("../../../fixtures/rust/overlap.rs"),
            Answer::DoesNotCompile,
            "The shared reference is used after the attempted mutation. Those uses overlap, so the compiler rejects the mutable borrow with E0502.",
            "borrow-overlap",
            Some("E0502"),
        ),
        "last-use" => (
            include_str!("../../../fixtures/rust/last_use.rs"),
            Answer::Compiles,
            "The shared reference's last use is before the mutation. Its borrow need not last to the closing brace; this program compiles.",
            "borrow-last-use",
            None,
        ),
        _ => return Err(DomainError::Invalid("unknown immutable question".into())),
    };
    Ok(Question {
        id: id.into(),
        version: 1,
        code: code.into(),
        answer,
        explanation: explanation.into(),
        source_span: span.into(),
        compiler_diagnostic: diagnostic.map(str::to_string),
    })
}
pub fn plan() -> Plan {
    Plan {
        nodes: vec![
            "Ownership & moves".into(),
            "Shared borrowing".into(),
            "Borrow overlap".into(),
        ],
        requires: vec![
            ("Ownership & moves".into(), "Shared borrowing".into()),
            ("Shared borrowing".into(), "Borrow overlap".into()),
        ],
    }
}
pub fn sources() -> Vec<SourceDocument> {
    [
        (
            "ownership",
            "ch04-01-what-is-ownership.md",
            include_str!("../../../sources/rust-book/ch04-01-what-is-ownership.md"),
        ),
        (
            "borrowing",
            "ch04-02-references-and-borrowing.md",
            include_str!("../../../sources/rust-book/ch04-02-references-and-borrowing.md"),
        ),
    ]
    .into_iter()
    .map(|(id, file, text)| SourceDocument {
        id: id.into(),
        revision: BOOK_REVISION.into(),
        url: format!("https://github.com/rust-lang/book/blob/{BOOK_REVISION}/src/{file}"),
        content: text.into(),
        sha256: digest(text),
    })
    .collect()
}
pub fn spans() -> Vec<SourceSpan> {
    let docs = sources();
    [
        (
            "ownership-move",
            "ownership",
            "because Rust also invalidates the first variable",
        ),
        ("borrow-reference", "borrowing", "without taking ownership"),
        (
            "borrow-overlap",
            "borrowing",
            "cannot have a mutable reference while we have an immutable one",
        ),
        (
            "borrow-last-use",
            "borrowing",
            "last time that reference is used",
        ),
    ]
    .into_iter()
    .map(|(id, doc, needle)| {
        let d = docs.iter().find(|d| d.id == doc).expect("fixture document");
        let start = d.content.find(needle).expect("pinned source span");
        let end = start + needle.len();
        SourceSpan {
            id: id.into(),
            document: doc.into(),
            start,
            end,
            sha256: digest(&d.content[start..end]),
        }
    })
    .collect()
}
pub fn assertions() -> Vec<Assertion> {
    [
        (
            "owner-available",
            "The binding still owns the String.",
            "ownership-move",
        ),
        (
            "no-overlap",
            "No shared borrow remains in use at the mutation.",
            "borrow-last-use",
        ),
        (
            "mutable-access",
            "The owned mutable String can be mutated in this fixture.",
            "borrow-overlap",
        ),
    ]
    .into_iter()
    .map(|(id, text, span)| Assertion {
        claim: ClaimVersion {
            id: id.into(),
            version: 1,
        },
        text: text.into(),
        source_spans: vec![span.into()],
        asserted_by: format!("LearningOS fixture author, grounded in Rust Book {BOOK_REVISION}"),
    })
    .collect()
}
pub fn joint_argument() -> JointArgument {
    JointArgument {
        id: "mutation-joint-1".into(),
        scope: "ownership-1 fixture: the binding is a mutable String; only a shared borrow is in scope; no competing mutable borrow exists".into(),
        premises: vec![
            ClaimVersion {
                id: "owner-available".into(),
                version: 1,
            },
            ClaimVersion {
                id: "no-overlap".into(),
                version: 1,
            },
        ],
        conclusion: ClaimVersion {
            id: "mutable-access".into(),
            version: 1,
        },
    }
}

impl Question {
    pub fn artifact_hash(&self) -> String {
        digest(&serde_json::to_string(self).expect("immutable question serializes"))
    }
    pub fn rubric_hash(&self) -> String {
        digest(&format!("{RUBRIC_VERSION}:{}", self.artifact_hash()))
    }
}
pub fn fixture_digest() -> String {
    static HASH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HASH.get_or_init(||digest(&serde_json::json!({"fixture":FIXTURE_VERSION,"policy":POLICY_VERSION,"rubric":RUBRIC_VERSION,"questions":(["move","shared","overlap","last-use"].map(|id|question(id).expect("fixture question"))),"sources":sources(),"spans":spans(),"assertions":assertions(),"argument":joint_argument(),"plan":plan()}).to_string())).clone()
}

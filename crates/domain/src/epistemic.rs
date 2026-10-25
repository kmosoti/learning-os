use crate::{DomainError, digest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDocument {
    pub id: String,
    pub revision: String,
    pub url: String,
    pub content: String,
    pub sha256: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub id: String,
    pub document: String,
    pub start: usize,
    pub end: usize,
    pub sha256: String,
}
impl SourceSpan {
    pub fn text<'a>(&self, document: &'a SourceDocument) -> Result<&'a str, DomainError> {
        if self.document != document.id || digest(&document.content) != document.sha256 {
            return Err(DomainError::Provenance(self.id.clone()));
        }
        let text = document
            .content
            .get(self.start..self.end)
            .ok_or_else(|| DomainError::Provenance(self.id.clone()))?;
        if digest(text) != self.sha256 {
            return Err(DomainError::Provenance(self.id.clone()));
        }
        Ok(text)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClaimVersion {
    pub id: String,
    pub version: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assertion {
    pub claim: ClaimVersion,
    pub text: String,
    pub source_spans: Vec<String>,
    pub asserted_by: String,
}
/// An assertion remains an assertion. Compiler checks are separately versioned evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JointArgument {
    pub id: String,
    pub scope: String,
    pub premises: Vec<ClaimVersion>,
    pub conclusion: ClaimVersion,
}
impl JointArgument {
    pub fn fires(&self, established: &BTreeSet<ClaimVersion>) -> bool {
        !self.premises.is_empty() && self.premises.iter().all(|p| established.contains(p))
    }
    pub fn validate(&self, claims: &BTreeSet<ClaimVersion>) -> Result<(), DomainError> {
        let unique: BTreeSet<_> = self.premises.iter().collect();
        if self.scope.is_empty()
            || self.scope.len() > 240
            || self.premises.is_empty()
            || self.premises.len() > 16
            || unique.len() != self.premises.len()
            || !claims.contains(&self.conclusion)
            || self.premises.iter().any(|p| !claims.contains(p))
        {
            return Err(DomainError::Invalid("joint argument references".into()));
        }
        Ok(())
    }
}
/// Only the local plan must be acyclic. This check is never imposed on the world graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Plan {
    pub nodes: Vec<String>,
    pub requires: Vec<(String, String)>,
}
impl Plan {
    pub fn validate(&self) -> Result<(), DomainError> {
        let nodes: BTreeSet<_> = self.nodes.iter().cloned().collect();
        if nodes.len() != self.nodes.len()
            || nodes.is_empty()
            || nodes.len() > 64
            || self.requires.len() > 256
        {
            return Err(DomainError::Invalid(
                "plan bounds or duplicate nodes".into(),
            ));
        }
        let mut degrees: BTreeMap<_, usize> = nodes.iter().map(|n| (n.clone(), 0)).collect();
        let mut edges = BTreeSet::new();
        for (from, to) in &self.requires {
            if !nodes.contains(from) || !nodes.contains(to) || !edges.insert((from, to)) {
                return Err(DomainError::Invalid("plan edge".into()));
            }
            *degrees.get_mut(to).expect("checked node") += 1;
        }
        let mut ready: Vec<_> = degrees
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(n, _)| n.clone())
            .collect();
        let mut count = 0;
        while let Some(n) = ready.pop() {
            count += 1;
            for (from, to) in &self.requires {
                if *from == n {
                    let degree = degrees.get_mut(to).expect("checked node");
                    *degree -= 1;
                    if *degree == 0 {
                        ready.push(to.clone());
                    }
                }
            }
        }
        if count != nodes.len() {
            return Err(DomainError::Invalid("plan cycle".into()));
        }
        Ok(())
    }
}

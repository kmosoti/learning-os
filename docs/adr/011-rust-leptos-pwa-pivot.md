# ADR-011: Rust, Leptos islands and Axum PWA

Status: Proposed formal record; fixed direction accepted and implementation authorized by the owner on 2026-10-25 with “Implement the proposed plan.” Mechanism choices are deferred to the approved experiments. PR approval/merge remains pending.

Related: [#49](https://github.com/kmosoti/learning-os/issues/49) and [execution contract](../architecture/v2-execution-contract.md).

## Context
The Python course CRUD foundation does not implement the intended evidence-grounded learning loop. The owner mandates a ground-up Rust PWA and recognizable LearnHarness interface.

## Decision
Use Rust application logic, Leptos SSR/selective islands, Axum/Tokio and an embedded SQLite-family store. Begin with five workspace members and a complete offline lesson. Generated bindings and a minimal service worker/browser interop are permitted; no JS application framework or Python production dependency.

## Supersession
Supersede ADR-003's language/tooling, ADR-004's FastAPI/ORM/NetworkX mechanisms (retain embedded SQLite), proposed ADR-005's HTMX direction, and ADR-010's runtime and PostgreSQL/pgvector pathway. Preserve their historical rationale. Amend ADR-001's course-only scope and ADR-006's EMT-first sequencing: goals are primary; Rust ownership is the first deterministic fixture. Retain ADR-002/007/008/009 principles.

## Alternatives and consequences
Mechanical Python translation and transplanting LearnHarness internals violate the mandate. A single modular monolith keeps deployment bounded. Islands granularity is measured in #51. Preserve the legacy app/data and PR #46; no source deletion or automatic migration. Installation/accessibility/offline behavior are part of the first slice, not later platform work.

## Confirmation
Native and WASM builds, dependency-boundary checks, functional browser loop, reproducible tests and no forbidden runtime dependencies. Owner review remains required before merge.

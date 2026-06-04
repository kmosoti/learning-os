# Agent Operating Contract

This repository uses GitHub as the operational coordination system.

Agents must treat:

- Repository files as durable product artifacts.
- GitHub Project 5 as the live planning database.
- Issues as executable units of work.
- Pull requests as reviewable proposed changes.
- Milestones as delivery targets.

Operational project: https://github.com/users/kmosoti/projects/5

## Required Workflow

1. Pick an issue from GitHub Project 5 with `Operating Status = Ready`, unless the user explicitly asks for design, triage, or blocked work.
2. Read the Project item fields before editing: `Work Type`, `Agent Suitability`, `Decision Needed`, `Blocked By`, `Development Branch`, `Review Owner`, `Documentation Gate`, and `Done Evidence`.
3. Confirm the issue is assigned to `@kmosoti`.
4. Confirm no unresolved ADR, design decision, dependency, or blocker prevents the work.
5. Create or use the issue's development branch.
6. Keep the work scoped to that issue.
7. Update docs/tests when behavior, commands, architecture, or workflow changes.
8. Open a PR that links the issue.
9. Move the issue to review only when the PR is ready for human review.
10. Do not merge without `@kmosoti` approval.

`Operating Status` is the authoritative workflow state. The older `Workflow State` field is legacy bootstrap history and must not drive new tracking.

## Governance And ADR Gates

Agents must follow the Project 5 governance rules before implementation.

- Work child/card issues, not epics, unless explicitly assigned design work.
- Epics may have `design/...` branches, but implementation belongs on child/card branches.
- If `Decision Needed = Yes`, do not implement dependent code until the decision is accepted or explicitly deferred by `@kmosoti`.
- ADRs are required for architecturally significant decisions: runtime/framework choices, persistence boundaries, retrieval/vector architecture, LLM provider boundaries, source provenance semantics, evidence policy, security/deployment posture, irreversible or expensive-to-change choices, and decisions that materially affect cost or reliability.
- ADRs are not required for routine implementation sequencing, simple CRUD, local refactors, small UI pages, test additions, or decisions already covered by an accepted ADR.
- Block only the child issues that depend on an unresolved decision. Keep independent preparatory work moving when it can proceed safely.
- When a decision changes implementation scope, update the issue body, Project fields, linked docs, and PR notes.

## API And Tooling Discipline

Before using an unfamiliar API, CLI surface, SDK, GraphQL mutation, REST endpoint, or project-management automation, agents must verify the contract from an authoritative source.

Required preflight:

1. Read the official documentation, OpenAPI schema, GraphQL introspection result, CLI `--help`, SDK type signature, or local source that defines the endpoint/tool.
2. Verify required inputs, accepted field names, field limits, authentication scopes, and side effects.
3. Prefer a read-only probe before a mutating call when the API supports it.
4. Resolve project IDs, item IDs, field IDs, option IDs, branch names, and issue IDs from current API reads instead of hard-coding stale values.
5. For GitHub GraphQL, introspect mutation input types before using a mutation that has not already been proven in this repo.
6. Do not retry a failing mutating call by guessing parameter names. Stop, inspect the spec/schema/help output, then retry with the verified contract.
7. Record nontrivial API assumptions and commands in the PR or `Done Evidence`.

If an API does not expose the requested capability, say that clearly and document the supported manual procedure instead of pretending the automation succeeded.

## Branch Standard

Every executable issue gets one development branch.

Branch format:

```text
type/issue-number-short-slug
```

Allowed branch types:

```text
feature
bug
docs
test
refactor
chore
design
research
security
eval
infra
setup
```

Examples:

```text
setup/12-initialize-uv-project-skeleton
docs/22-adopt-engineering-standards-dod
infra/18-sqlalchemy-base-session
```

## PR Requirements

Every PR must:

- Link its issue with `Closes #...`, `Fixes #...`, or `Refs #...`.
- Use the branch for that issue.
- Explain scope and risk in human-readable language.
- Include commands run and results.
- Update linked docs or READMEs when behavior changes.
- Include rendered Mermaid verification when diagrams are added or changed.
- Request review from `@kmosoti`.

Agents must not merge PRs. `@kmosoti` approves all PRs.

## Definition Of Ready

An issue is ready only when:

- Objective is explicit.
- Scope is bounded.
- Out-of-scope is stated or obvious.
- Acceptance criteria exist.
- Validation steps are described.
- Dependencies are named.
- No unresolved design decision blocks the work.

## Definition Of Done

An issue is done only when:

- Acceptance criteria are met.
- PR is merged or non-code output is accepted.
- Tests and docs are updated where needed.
- CI passes if code changed.
- Follow-up work is captured as issues.
- Project 5 `Done Evidence` explains what changed and how it was verified.

## Engineering Gate

For MVP core code, the PR must also prove:

- Inputs and outputs are typed.
- Side effects are isolated or explicitly named.
- Swappable dependencies use protocols/adapters.
- State transitions are validated where relevant.
- Idempotency is documented where relevant.
- Failure modes use domain errors or structured result statuses.
- Time and space complexity are understood.
- LLM, embedding, database, graph, file I/O, and external-call costs are bounded.
- Source provenance is preserved for generated learning content.

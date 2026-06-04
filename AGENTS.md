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

1. Pick an issue from GitHub Project 5 with `Workflow State = Ready`.
2. Confirm the issue is assigned to `@kmosoti`.
3. Create or use the issue's development branch.
4. Keep the work scoped to that issue.
5. Update docs/tests when behavior, commands, architecture, or workflow changes.
6. Open a PR that links the issue.
7. Move the issue to review only when the PR is ready for human review.
8. Do not merge without `@kmosoti` approval.

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


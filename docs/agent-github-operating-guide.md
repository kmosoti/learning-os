# Agent GitHub Operating Guide

This guide defines how agents should use GitHub for `learning-os`.

## Operating Model

Agents should treat GitHub as a coordination system:

```text
Repository = product artifact
Project = planning database
Issue = executable unit of work
Milestone = delivery target
Branch = isolated development line
Pull Request = reviewable proposed change
Docs = durable knowledge
Actions = mechanical enforcement
```

## Issue Rules

Agents should work child/card issues, not epics.

An implementation issue must have:

- Objective.
- Scope.
- Out of scope.
- Acceptance criteria.
- Testing or validation.
- Agent constraints.
- Milestone.
- Project 5 item fields.
- Development branch.

Before starting, agents must read the Project 5 item fields for the issue:

- `Operating Status`
- `Work Type`
- `Agent Suitability`
- `Decision Needed`
- `Blocked By`
- `Dependencies`
- `Development Branch`
- `Review Owner`
- `Documentation Gate`
- `Done Evidence`

`Operating Status` is authoritative. The older `Workflow State` field is legacy bootstrap history.

## Governance Preflight

Run this preflight before changing files:

1. Confirm the issue is in `Operating Status = Ready`, unless the user explicitly asked for design, triage, or blocker resolution.
2. Confirm the issue is assigned to `@kmosoti`.
3. Confirm `Decision Needed = No` for implementation work.
4. Confirm `Blocked By` is empty for implementation work.
5. Confirm the issue has a `Development Branch`.
6. Confirm the branch exists or create it from `main` if the Project field names it.
7. Confirm the issue is inside the correct milestone and release slice.
8. Read acceptance criteria, complexity budget, dependencies, and agent handoff.
9. If the issue is an epic, do not implement directly. Use or create a child/card issue.

If any item fails, update the Project item or report the blocker instead of starting unrelated work.

## ADR And Decision Gates

Every epic must pass an architecture-readiness gate, but not every epic automatically needs a comprehensive ADR.

Create an ADR when the work makes or depends on an architecturally significant decision:

- runtime, framework, or process model
- persistence, migration, indexing, or vector backend
- LLM provider boundary, schema contract, prompt lifecycle, or token/cost policy
- retrieval architecture
- source provenance, grounding, evidence, or learner-state semantics
- security, secrets, uploads, logs, or deployment posture
- irreversible or expensive-to-change choices
- decisions that materially affect cost, reliability, data correctness, or future maintainability

Do not create ADRs for routine implementation sequencing, simple CRUD, local refactors, small UI pages, test additions, or decisions already covered by an accepted ADR.

Block only the child issues that depend on an unresolved decision. Independent preparatory work can proceed when it is clearly safe.

## Agent Suitability

Use Project 5 `Agent Suitability`:

| Value | Meaning |
| --- | --- |
| Agent-ready | Scope is clear, ambiguity is low, tests are defined. |
| Agent-assisted | Agent can help, but human judgment is needed. |
| Human-only | Strategic, risky, sensitive, or ambiguous. |

Agents should not work `Human-only` issues unless explicitly instructed.

## Review Depth

Use Project 5 `Review Depth`:

| Value | Meaning |
| --- | --- |
| normal | Routine correctness review. |
| architecture | Boundary, dependency direction, or design review needed. |
| security | Secret, auth, data, or dependency risk exists. |
| performance | Complexity, cost, latency, or scaling attention needed. |

## PR Checklist For Agents

Before opening a PR:

- Read the linked issue.
- Run the governance preflight.
- Use the issue branch.
- Stay within scope.
- Do not invent product requirements.
- Verify external API, CLI, SDK, GraphQL, and REST contracts before relying on them.
- Update tests and docs where needed.
- Run or document quality checks.
- Link the issue in the PR body.
- Document risk and review focus.
- Confirm Mermaid diagrams render when changed.
- Request `@kmosoti` review.

Before claiming completion:

- Acceptance criteria are met.
- CI-relevant checks pass or failures are disclosed.
- PR is ready for human review.
- Docs and tests are aligned.
- No hidden architecture decision was made.
- Follow-up work is captured as issues.

## API Spec Preflight

Agents must inspect the relevant contract before using APIs or automation surfaces.

Use this process for GitHub, external services, local SDKs, CLIs, MCP tools, and generated clients:

1. Identify the authoritative contract: official docs, OpenAPI schema, GraphQL introspection, CLI `--help`, SDK type definitions, or local source.
2. Read the exact command, endpoint, mutation, or method before calling it.
3. Check required fields, accepted field names, input limits, permissions/scopes, idempotency, and side effects.
4. Prefer read-only discovery calls before mutating calls.
5. Resolve IDs and option values from current state, not memory.
6. For GitHub Projects, read field IDs and option IDs with `gh project field-list` before writing item fields.
7. For GitHub GraphQL, introspect the input type before using an unfamiliar mutation.
8. If a call fails because of schema or argument mismatch, stop and inspect the spec before retrying.
9. Record nontrivial API assumptions in the PR body or Project `Done Evidence`.

Do not guess at endpoint parameters, GraphQL input names, Project field names, or CLI flags after a failure.

## GitHub Project View Rule

GitHub Project V2 saved views can be read through GraphQL and created through the REST Project views API.

For user-owned projects, first verify the current REST contract, then use the Project views endpoint with:

- `X-GitHub-Api-Version: 2026-03-10`
- `POST /users/{username}/projectsV2/{project_number}/views`
- `name`
- `layout`: `table`, `board`, or `roadmap`
- optional `filter`
- optional `visible_fields` using numeric REST field IDs from `GET /users/{username}/projectsV2/{project_number}/fields`

Agents should:

- Read current views through GraphQL before claiming view state.
- Read REST field IDs before setting `visible_fields`.
- Create views idempotently by checking existing view names first.
- Prefer simple, single-field filters unless a combined filter has been verified against the live endpoint.
- Keep canonical view definitions in the Project README and `PROJECT DOC: Views and filters`.
- Use the GitHub UI for view update/delete, grouping, sorting, and board column tuning unless a supported API contract has been verified for that operation.

View configuration boundaries:

- API-safe: create saved views, set layout, set saved filter, set visible fields.
- UI-required unless separately verified: rename, delete, reorder tabs, duplicate views, save unsaved view changes, set table grouping/slicing/sorting, set board column field, show/hide board columns, set board column limits, set roadmap start/target date fields, set roadmap markers, set roadmap zoom, and enable field sums.
- For the Learning OS kanban board, the intended board column field is `Operating Status`, not GitHub's default `Status`.
- For the MVP roadmap, the intended date field is `Target Date`; use milestones as markers when useful.

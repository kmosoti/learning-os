# GitHub Workflow

This repository uses GitHub Project 5 as the operational planning surface.

Project: https://github.com/users/kmosoti/projects/5

Repo documentation is static reference. Project fields, issue comments, PRs, and milestones are the live tracking system.

## Surfaces

| Surface | Purpose |
| --- | --- |
| Repository | Code, static docs, tests, configuration, durable project rules. |
| Project 5 | Live roadmap, workflow state, completion gates, agent handoff. |
| Issue | One executable unit of work or one epic. |
| Milestone | Delivery target. |
| Branch | One development line for one executable issue. |
| PR | Reviewable proposed change. |
| Release | Shipped version. |

## Workflow States

Use Project 5 `Operating Status` as the operational state.

The older `Workflow State` field is retained only as bootstrap history. Do not use it for new tracking.

```text
Inbox
Needs Triage
Needs Design
Ready
In Progress
In Review
Blocked
Done
Won't Do
```

Allowed transitions:

```text
Inbox -> Needs Triage
Needs Triage -> Needs Design
Needs Triage -> Ready
Needs Triage -> Won't Do
Needs Design -> Ready
Ready -> In Progress
In Progress -> In Review
In Progress -> Blocked
Blocked -> Ready
Blocked -> In Progress
In Review -> Done
In Review -> In Progress
Done -> In Progress only when reopened for a concrete missed criterion or regression
```

## Milestones

Use milestones as delivery containers.

Current milestone:

```text
MVP
```

Release slices such as `VS-0 Foundation` and `VS-4 Retrieval` belong in Project 5 fields, not separate milestones.

## Branches

Each open repo issue should have one branch recorded in Project 5.

```text
type/issue-number-short-slug
```

Epics use `design/...` branches for architecture and planning. Agents should not implement directly from epics unless explicitly instructed.

Executable child/card issues use implementation branches such as `setup/...`, `feature/...`, `infra/...`, or `docs/...`.

Agents must use the branch named in the issue's `Development Branch` Project field.

## Pull Requests

Every PR must:

- Target `main`.
- Come from the issue branch.
- Link the issue.
- Request `@kmosoti` review.
- Include testing evidence.
- Include docs evidence when docs changed.
- Confirm Mermaid diagrams render when diagrams changed.
- Confirm governance and API-spec checks were performed when relevant.

`main` requires pull request review. Agents must not merge without `@kmosoti` approval.

## ADR And Decision Gates

Every epic must pass an architecture-readiness gate, but not every epic automatically needs an ADR.

Create an ADR for architecturally significant decisions:

- runtime/framework/process model
- persistence, migrations, indexing, or vector backend
- LLM provider boundary, structured output, prompt lifecycle, or token/cost policy
- retrieval architecture
- source provenance, grounding, evidence, or learner-state semantics
- security, secrets, upload limits, logs, or deployment posture
- irreversible or expensive-to-change choices

Block only child/card issues that depend on unresolved decisions. If independent work can proceed safely, keep it unblocked.

## API Automation Rule

Before using a mutating API, CLI, SDK, REST endpoint, GraphQL mutation, or Project automation:

1. Inspect the official docs, schema, CLI `--help`, SDK type signature, or GraphQL input type.
2. Verify required fields, field limits, permissions/scopes, and side effects.
3. Run a read-only discovery call first when possible.
4. Resolve IDs and option values from current state.
5. Stop and inspect the contract after schema or argument failures. Do not guess retries.

For GitHub Project automation, read field IDs and option IDs before editing items.

For saved Project views:

- Read existing views through GraphQL before claiming view state.
- Create views with the REST Project views endpoint after verifying the current docs.
- Use `X-GitHub-Api-Version: 2026-03-10`.
- Use numeric REST field IDs from the Project fields endpoint when setting `visible_fields`.
- Use the GitHub UI for update/delete, grouping, sorting, and board column tuning unless a supported API contract has been verified for that operation.

After creating saved views through the API, finish view shaping in the GitHub UI:

- Set `01 Kanban` board column field to `Operating Status`.
- Hide board columns that are not part of the active workflow, if needed.
- Apply WIP column limits from `docs/03-kanban-board.md`.
- Group or slice tables by `Release Slice`, `Epic Area`, `Priority`, or `Completion Gate` where useful.
- Set `10 MVP Roadmap` to use `Target Date`; use milestones as roadmap markers.
- Save the view after UI changes so the dot beside the tab disappears.

## Documentation Expectations

Docs should be human readable and linked from the relevant README or docs index.

When Mermaid diagrams change:

- Use fenced `mermaid` blocks.
- Verify they render in GitHub preview or Mermaid tooling.
- Link the diagram-containing docs from the PR.
- Mention the rendered verification in the PR body.

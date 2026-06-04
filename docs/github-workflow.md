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

Use Project 5 `Workflow State` as the operational state.

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

Each executable issue has one branch:

```text
type/issue-number-short-slug
```

Epics do not need implementation branches. Agents work child/card issues, not epics.

## Pull Requests

Every PR must:

- Target `main`.
- Come from the issue branch.
- Link the issue.
- Request `@kmosoti` review.
- Include testing evidence.
- Include docs evidence when docs changed.
- Confirm Mermaid diagrams render when diagrams changed.

`main` requires pull request review. Agents must not merge without `@kmosoti` approval.

## Documentation Expectations

Docs should be human readable and linked from the relevant README or docs index.

When Mermaid diagrams change:

- Use fenced `mermaid` blocks.
- Verify they render in GitHub preview or Mermaid tooling.
- Link the diagram-containing docs from the PR.
- Mention the rendered verification in the PR body.


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
- Use the issue branch.
- Stay within scope.
- Do not invent product requirements.
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


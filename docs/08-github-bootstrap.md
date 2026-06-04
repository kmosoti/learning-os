# GitHub Bootstrap

This document records the GitHub setup for project planning and version control.

## Repository

- Repository: https://github.com/kmosoti/learning-os
- Default branch: `main`
- Local remote: `origin https://github.com/kmosoti/learning-os.git`
- License present before planning bootstrap: AGPL-3.0
- `.gitignore` present before planning bootstrap: Python template

## Project Board

- Project URL: https://github.com/users/kmosoti/projects/5

Project population is currently blocked by GitHub CLI token scope. `gh` is authenticated, but Project v2 operations need the `project` scope.

Run locally:

```powershell
gh auth refresh -h github.com -s project
```

Then add issues to the project:

```powershell
$issues = gh issue list --repo kmosoti/learning-os --state open --limit 100 --json url --jq '.[].url'
foreach ($issue in $issues) {
  gh project item-add 5 --owner kmosoti --url $issue
}
```

Suggested Project columns:

- Backlog
- Ready
- In Progress
- Review
- Done
- Blocked

## Labels

The following labels were created or updated:

- `epic`
- `mvp`
- `ready`
- `release-slice`
- `foundation`
- `architecture`
- `database`
- `ingestion`
- `extraction`
- `retrieval`
- `notes-grounding`
- `quiz-mastery`
- `planner`
- `ui`
- `evidence`
- `engineering-standard`
- `blocked`

## Seeded Issues

Epic issues:

- #1 EPIC: Foundation and tooling (VS-0)
- #2 EPIC: Course and source spine (VS-1/VS-2)
- #3 EPIC: Trusted EMT fixture baseline (VS-3)
- #4 EPIC: General extraction and canonicalization
- #5 EPIC: Search and retrieval (VS-4)
- #6 EPIC: Notes to grounded claims (VS-5)
- #7 EPIC: Quiz, evaluation, and learner mastery (VS-6)
- #8 EPIC: Planner next action (VS-7)
- #9 EPIC: Minimal HTMX UI and EMT demo (VS-8)
- #10 EPIC: Manual evidence layer
- #11 EPIC: Engineering standards and deterministic review

Ready implementation issues:

- #12 KAN-001: Initialize uv project skeleton
- #13 KAN-002: Pin Python 3.14.5
- #14 KAN-003: Configure Ruff, ty, and pytest
- #15 KAN-004: Add FastAPI application shell
- #16 KAN-005: Add core config, logging, errors, IDs, and time helpers
- #17 KAN-006: Define invariant state enums
- #18 KAN-010: Add SQLAlchemy base and session factory
- #19 KAN-011: Add Alembic migration setup
- #20 KAN-012: Implement Course model and migration
- #21 KAN-020: Add health check and typed API error envelope
- #22 KAN-160: Adopt engineering standards as Definition of Done
- #23 KAN-161: Add service contract template

Blocked setup issue:

- #24 BLOCKED: Add backlog issues to GitHub Project 5

## Version Control Bootstrap

Local git has been initialized and connected to the remote repository.

Useful commands:

```powershell
git status
git pull --rebase origin main
git push origin main
```


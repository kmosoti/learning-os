# Contributing

All work should flow through GitHub issues, Project 5, issue branches, and pull requests.

Project board: https://github.com/users/kmosoti/projects/5

## Workflow

1. Select an issue with `Operating Status = Ready` from Project 5, unless doing explicit design or blocker work.
2. Use the issue's development branch.
3. Check `Decision Needed`, `Blocked By`, dependencies, and ADR links before implementation.
4. Keep the diff scoped to that issue.
5. Verify API, CLI, SDK, REST, GraphQL, and Project automation contracts before relying on them.
6. Run or document required checks.
7. Open a PR that links the issue.
8. Request review from `@kmosoti`.
9. Wait for approval before merge.

## Branch Naming

Use:

```text
type/issue-number-short-slug
```

Examples:

```text
setup/12-initialize-uv-project-skeleton
feature/31-markdown-source-loader
docs/44-github-workflow
```

## Commit Naming

Use:

```text
type(scope): short imperative summary
```

Examples:

```text
feat(ingestion): add markdown source loader
docs(github): document project workflow
test(retrieval): add fts smoke test
```

## Pull Request Readiness

A PR is ready for review when:

- It links an issue.
- It has a human-readable summary.
- It documents tests or validation.
- It confirms Project governance gates were checked.
- It confirms unresolved ADR/design decisions are linked or not applicable.
- It confirms external API/tool assumptions were verified against current docs/help/schema when relevant.
- It updates documentation when behavior changes.
- Mermaid diagrams render correctly when added or changed.
- Risk and review focus are clear.

# Git Strategy

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Version control conventions for the TANF Job Search Automation repository.

## Trunk-Based Development

This project uses trunk-based development. All work flows through short-lived feature branches that merge into `main`.

### Main Branch

The `main` branch is the single source of truth.

- Always deployable (or at minimum, always compiles and passes tests).
- All changes arrive via feature branch merge with rebase.

### Feature Branches

All development occurs on feature branches.

**Naming convention**: `<scope>/<short-description>`

| Scope | Purpose | Example |
|-------|---------|---------|
| `feat` | New functionality | `feat/job-application-form` |
| `fix` | Bug fixes | `fix/pdf-field-mapping` |
| `docs` | Documentation only | `docs/process-refinement` |
| `refactor` | Code restructuring | `refactor/extract-pdf-module` |
| `test` | Test additions or fixes | `test/application-tracker` |
| `chore` | Maintenance tasks | `chore/update-dependencies` |

**Lifespan**: Feature branches should not live longer than 24 hours. If work exceeds this window, break the work into smaller increments.

### Creating a Feature Branch

```bash
git checkout main
git pull --rebase origin main
git checkout -b feat/my-feature
```

### Linear History

Enforce rebase, not merge. Linear history keeps the commit log readable for AI agents parsing project history.

When `main` has advanced while working on a feature branch:

```bash
git fetch origin
git rebase origin/main
```

## Commit Conventions

### Message Format

```
<scope>: <imperative summary>

<optional body explaining why, not what>

[Task: <task-identifier>]

Co-Authored-By: Claude <noreply@anthropic.com>
```

**Summary line**:
- Use imperative mood ("add" not "added", "fix" not "fixes").
- Keep under 72 characters.
- Scope matches the type of change.

**Body** (when needed):
- Explain motivation and context.
- Note any assumptions or limitations.

**Task reference** (when applicable):
- Use the Vw-Mx-Py-Tz work item code. See [Communication](./COMMUNICATION.md) for the coding system.

**Co-author**: Include when the AI agent contributed to the changes.

### Scopes

| Scope | Purpose | Example |
|-------|---------|---------|
| `feat` | New functionality | `feat: add job application form` |
| `fix` | Bug fixes | `fix: correct PDF field mapping` |
| `docs` | Documentation and knowledge graph changes | `docs: add git strategy to knowledge graph` |
| `refactor` | Restructuring without behavior change | `refactor: extract PDF module` |
| `chore` | Maintenance tasks | `chore: update dependencies` |
| `test` | Test additions or fixes | `test: add application tracker tests` |

### Commit Timing

The AI agent commits once after all tasks in a prompt are complete, including the `REVERSE_PROMPT.md` update. One commit per prompted request is the standard granularity.

Exceptions where multiple commits are appropriate:
- Logically independent changes that should be separable in history.
- The human pilot explicitly requests intermediate commits.

### Examples

```
docs: establish knowledge graph and process

Adapted from reference projects. Initial documentation structure
for TANF job search automation.

[Task: V0.0-M1-P1]

Co-Authored-By: Claude <noreply@anthropic.com>
```

```
feat: add job application tracking form

[Task: V0.1-M1-P2-T1]

Co-Authored-By: Claude <noreply@anthropic.com>
```

## Pre-Push Checklist

Before pushing:

- [ ] Commit messages follow scoped conventional format
- [ ] No secrets or credentials in committed files
- [ ] `secret/` directory contents are not staged
- [ ] `REVERSE_PROMPT.md` and `TASKLOG.md` are updated if AI-assisted work was performed

## Revision History

| Date | Author | Changes |
|------|--------|---------|
| 2026-02-19 | Claude | Initial creation. Adapted from reference projects. |
| 2026-02-19 | Claude | Adopted trunk-based development with feature branches. Updated work item coding to Vw-Mx-Py-Tz. |

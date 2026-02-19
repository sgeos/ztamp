# Git Strategy

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Version control conventions for the TANF Job Search Automation repository.

## Branch Model

This project uses a single `main` branch for initial development. All commits land directly on `main`. Feature branches and pull requests may be introduced when the project matures or when multiple contributors are active.

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
- Use the Px-Ty work item code. See [Communication](./COMMUNICATION.md) for the coding system.

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

[Task: P1-T1]

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

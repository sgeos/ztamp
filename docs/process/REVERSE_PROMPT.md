# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: P1 Establish Process and Knowledge Graph

---

## Verification

### Knowledge Graph Drafted

Documentation knowledge graph created with four sections and 14 files.

**Structure.** Hierarchical organization with atomic files, breadcrumb navigation, and `README.md` tables of contents at each level. Adapted primarily from the blog project with elements from the crypto project.

```
docs/
├── README.md
├── DOCUMENTATION_STRATEGY.md
├── overview/
│   ├── README.md
│   └── PRODUCT_IDENTITY.md
├── architecture/
│   ├── README.md
│   └── TECHNOLOGY_STACK.md
├── process/
│   ├── README.md
│   ├── COMMUNICATION.md
│   ├── GIT_STRATEGY.md
│   ├── TASKLOG.md
│   ├── PROMPT.md
│   └── REVERSE_PROMPT.md
└── reference/
    ├── README.md
    └── GLOSSARY.md
```

### Process Documented

Six process-related files created.

1. **COMMUNICATION.md.** Bidirectional protocol with forward prompt, reverse prompt, and task log. Work item coding system uses Px-Ty format.
2. **GIT_STRATEGY.md.** Scoped conventional commits on `main` branch. One commit per prompted request.
3. **TASKLOG.md.** Current task state with success criteria, breakdown, and history.
4. **REVERSE_PROMPT.md.** This file. AI-to-human communication channel.
5. **PROMPT.md.** Human-authored. Committed as-is per protocol.
6. **Process README.md.** Table of contents with session startup protocol.

### .gitignore Updated

`secret/` added to `.gitignore` along with standard exclusions for OS artifacts and editor temporary files.

### CLAUDE.md Created

Project-level AI agent instructions created at repository root. Includes project overview, technology stack, repository structure, documentation links, commit conventions, session startup protocol, and sensitive file guidance.

### Process Files Updated

- TASKLOG.md: current task updated, history entry added.
- REVERSE_PROMPT.md: overwritten.

---

## Questions for Human Review

1. The work item coding system uses `Px-Ty` (Prompt-Task) rather than the blog's `Ax-Py-Tz` (Article-Prompt-Task) or the crypto project's `Vw-Mx-Py-Tz` (Version-Milestone-Prompt-Task). The article dimension does not apply to this project. Should a version or milestone dimension be added, or is `Px-Ty` sufficient?

2. The git strategy currently uses direct commits to `main` (following the blog convention for a single-author project). Should this be changed to trunk-based development with feature branches (following the crypto convention) from the start?

3. The technology stack documents Phoenix/Elixir with LiveView and Rust NIFs as specified. PDF manipulation, screenshot capture, and browser automation will require specific library choices. Should these decisions be deferred to a future prompt, or would you like to establish them now?

4. The `secret/` directory exists but is empty except for whatever the human pilot has placed there. Should a `.gitkeep` file be added to ensure the directory is tracked by git while its contents remain excluded?

---

## Notes

- This is the first prompt (P1) for the project. All process conventions are newly established and subject to revision based on human review.
- The blog project uses `_docs/` as its documentation directory. This project uses `docs/` following the crypto project convention. The blog uses `_docs/` because Jekyll treats underscore-prefixed directories specially. Since this project does not use Jekyll, `docs/` is the appropriate choice.
- No application code exists yet. The next logical step would be scaffolding the Phoenix application.

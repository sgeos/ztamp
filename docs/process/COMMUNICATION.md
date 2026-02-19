# Bidirectional Communication Protocol

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Protocol for structured communication between the human pilot and AI agent across sessions.

## Overview

Three working documents maintain state across AI sessions and enable asynchronous collaboration.

| Document | Direction | Persistence |
|----------|-----------|-------------|
| [PROMPT.md](./PROMPT.md) | Human to AI | Committed to preserve prompt history |
| [REVERSE_PROMPT.md](./REVERSE_PROMPT.md) | AI to Human | Overwritten after each completed task |
| [TASKLOG.md](./TASKLOG.md) | Shared | Updated incrementally as tasks complete |

## Forward Prompt (Human to AI)

`PROMPT.md` is a staging area for complex, multi-step instructions. The human pilot drafts and refines prompts here before execution.

### Structure

- **Comments**: Status notes and context for the AI agent.
- **Objectives**: Numbered, hierarchical task descriptions.
- **Context**: Background information relevant to the objectives.
- **Constraints**: Boundaries on what the AI agent should and should not do.
- **Success Criteria**: Verifiable conditions that define task completion.
- **Notes**: Supplementary information.

### Rules

- `PROMPT.md` is **read-only for the AI agent**. The AI agent must never modify this file. Only the human pilot writes to `PROMPT.md`.
- The AI agent must commit `PROMPT.md` along with other changes if it has been modified by the human pilot. This keeps the human prompt and AI reverse prompt in sync with committed work.

## Reverse Prompt (AI to Human)

`REVERSE_PROMPT.md` is the AI-to-human communication channel. The AI agent overwrites this file after completing each task.

### Structure

- **Last Updated**: Date, task identifier, and parent milestone.
- **Verification**: Commands run and their results, one per completed task.
- **Summary**: Implementation summary and status.
- **Questions for Human Review**: Numbered questions requiring human decisions.
- **Notes**: Supplementary observations.

### Rules

- If blocked or uncertain, document the blocker in `REVERSE_PROMPT.md` and stop. Do not proceed with assumptions.
- Every completed task must have a verification command and result documented.

## Task Log

`TASKLOG.md` is the shared source of truth for the current unit of work.

### Structure

- **Task Name and Status**: Descriptive name with status indicator (In-Progress, Blocked, Complete).
- **Success Criteria**: Checkbox list of verifiable completion conditions.
- **Task Breakdown**: Table with task ID, description, status, and verification method.
- **Notes**: Additional context or decisions.
- **History**: Date-based change log.

### Rules

- Update task status as work progresses.
- Every task marked "Complete" must have a corresponding verification entry.
- If blocked, update status to "Blocked" and document the blocker.

### History Maintenance

The History table should be periodically streamlined to minimize token consumption.

- Consolidate same-day entries into one line when all prompts for a logical unit are complete.
- Retain per-prompt granularity only for the current active task.
- The goal is to preserve what was done and when, not how each intermediate step was executed.

## Session Startup Protocol

1. Read `TASKLOG.md` for current task state.
2. Read `REVERSE_PROMPT.md` for last AI communication.
3. Wait for human prompt before proceeding.

## Work Item Coding System

All work items use the **Vw-Mx-Py-Tz** coding system for traceability across versions, milestones, prompts, and tasks.

### Format

`Vw-Mx-Py-Tz`

| Component | Meaning | Example |
|-----------|---------|---------|
| Vw | Version (Phase) | V0.0 = Phase 0, V0.1 = Phase 1 |
| Mx | Milestone within version | M1 = first milestone |
| Py | Prompt within milestone | P3 = third prompt |
| Tz | Task within prompt | T2 = second task |

### Version Mapping

| Phase | Version | Description |
|-------|---------|-------------|
| Phase 0 | V0.0 | Process definition and project setup |
| Phase 1 | V0.1 | Project structure (Phoenix/Elixir scaffolding, Rust NIF) |
| Phase 2 | V0.2 | PDF write PoC (Rust CLI for text/image overlay) |
| Phase 3 | V0.3 | PDF concatenation PoC |
| Phase 4 | V0.4 | Browser screenshot PoC (Wallaby) |
| Phase 5 | V0.5 | Per-platform submission logic |
| Phase 6 | V0.6 | Submission tracking and compliance reporting |
| Phase 7 | V0.7 | Resume and cover letter customization |

See [Roadmap](../roadmap/README.md) for detailed phase descriptions and dependencies.

### Usage

- **TASKLOG.md**: Tasks use Vw-Mx-Py-Tz codes in the ID column.
- **Git commits**: Reference task codes in the commit body using `[Task: Vw-Mx-Py-Tz]`.
- **Decisions**: Decision IDs in `decisions/` reference the version and milestone where they are relevant.

### Examples

- `V0.0-M1-P2-T1` = Phase 0, Milestone 1, Prompt 2, Task 1
- `V0.0-M1-P1` = Phase 0, Milestone 1, Prompt 1 (no specific task)
- `V0.1-M1-P3-T2` = Phase 1 (when defined), Milestone 1, Prompt 3, Task 2

## Task Completion Protocol

1. Complete all implementation tasks.
2. Update `TASKLOG.md` task status.
3. Update `REVERSE_PROMPT.md` with verification and summary.
4. Commit all changes in a single commit. The commit happens after all tasks including the `REVERSE_PROMPT.md` update are complete.
5. Proceed to next prompt or stop if blocked.

## Blocking Protocol

If the AI agent cannot proceed due to missing information, ambiguity, or a technical obstacle, it must follow this protocol.

1. Document the blocker in `REVERSE_PROMPT.md` under Questions or Notes.
2. Update `TASKLOG.md` status to "Blocked."
3. Commit changes.
4. Stop and wait for human direction.

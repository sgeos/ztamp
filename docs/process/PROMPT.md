# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions in the reverse prompt.

1. Text is nominally translated correctly, but placement on the form is totally wrong.
2. Add a `--grid` flag. Interval and text color should be specifiable.
   The grid should be above the form background, but below the text.
3. Are there any computer vision solutions?
   Is there a sane feedback loop for human adjustment?

Positioning comments:

- For most fields, X appears to be too large.
- For most fields, Y appears to be too small.
  For the bottom signature and date, Y is too large.
- Spacing is slightly too distand for application rows.

## Objectives

### Revise Fill Tool

Attempt to adjust fill tool to add grid logic based on the above feedback.

### Repeat Calibaration Test

Repeat calibration test for the next iteration of form output.
It is OK to use the same filename.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- Revise fill tool based on feedback.
- Sample output placed in `secret`
- Comment on automated generate-inspect-adjust in the next reverse prompt.

## Notes

(none)

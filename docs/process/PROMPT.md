# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions in previous reverse prompt.

1. I am open to an automated generate-inspect-adjust loop, if you can do that.
2. A grid lines flag probably makes sense.
3. Compile time is fine. We can figure out feature gates later.
4. Manual testing is fine for now.

Answers to questions in the most recent reverse prompt.

1. Text orientation is correct, but positions are wrong.
   If offsets are from the top-left corner,
   then rotated forms will need to use a different origin.
   Also, a matrix probably needs to be used,
   because X indicates the Y offset, and Y the X offset
   for CW or CCW rotated documents.
2. Assuming offsets were correct to begin with,
   a rotation matrix should be able to produce the
   correct offsets for a rotated document.
3. Ellipse orientation is correct, but the position is wrong.

## Objectives

### Revise Fill Tool

Attempt to adjust fill tool text placement logic based on
the above feedback.

### Repeat Calibaration Test

Repeat calibration test for the next iteration of form output.
It is OK to use the same filename.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- Revise fill tool positioning logic based on feedback.
- Sample output placed in `secret`
- Perform an automated generate-inspect-adjust if you can.

## Notes

(none)

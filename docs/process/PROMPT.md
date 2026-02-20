# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions in the reverse prompt.

1. PoC largely good. Need some minor adjustments.

## Objectives

### Generate Four PoC Files In Preparation for Concatenation

Reexport each PoC file with the new flag.
Each file should have a different name.

#### Production PoC

- No grid.
- All text is black.
- No ellipse labels.
- Only one ellipse in each series is circled.
- No debug test data.
- No watermark.

#### Almost Production PoC

- No grid.
- All text is black.
- No ellipse labels.
- Only one ellipse in each series is circled.
- Fill with debug test data.
- Red watermark.

#### Black Text Testing PoC

- No grid.
- All text is black.
- No ellipse labels.
- All ellipses in each series circled.
- Fill with debug test data.
- Red watermark.

#### Debug PoC

- Green grid.
- Debug colors for text.
- Ellipse labels.
- All ellipses in each series circled.
- Fill with debug test data.
- Red watermark.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- Debug fill flag added to export tool.
- Four PoCs generated.

## Notes

(none)

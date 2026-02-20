# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Answers to questions in the reverse prompt.

1. PoC largely good. Need some minor adjustments.

## Objectives

### Refactor Export Tool

Add a `--debug-fill` flag that fills the form with the current test data.
Production use will need to supply real data.

### Generate Three PoC File In Preparation for Concatenation

Reexport each PoC file with the new flag.
Each file should have a different name.

#### Almost Production PoC

- No grid.
- All text is black.
- No ellipse labels.
- Only one ellipse in each series is circled.
- Red watermark.

#### Black Text Testing PoC

- No grid.
- All text is black.
- No ellipse labels.
- All ellipses in each series circled.
- Red watermark.

#### Debug PoC

- Green grid.
- Debug colors for text.
- Ellipse labels.
- All ellipses in each series circled.
- Red watermark.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- Debug fill flag added to export tool.
- Three PoCs generated.

## Notes

(none)

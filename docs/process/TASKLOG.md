# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Coordinate Transformation (V0.2-M1-P3)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Fill tool positioning logic revised with coordinate transformation.
- [x] Sample output placed in `secret/`.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P3-T1 | Add coordinate transformation to Rotation | Complete | `transform_position()` method maps form-space (fx, fy) to page-space (px, py). CCW 90: px = fy*W/H, py = H - fx*H/W. CW 90: px = W - fy*W/H, py = fx*H/W. 180: px = W-fx, py = H-fy. Corner verification passes for all four modes. |
| V0.2-M1-P3-T2 | Apply transformation to text and circle rendering | Complete | Both text field positions and circle mark centers are transformed from form-space to page-space before PDF coordinate conversion. Circle radii remain swapped for 90-degree rotations. |
| V0.2-M1-P3-T3 | Generate calibration sample | Complete | `secret/calibration_sample.pdf` (90,491 bytes). 112 text fields, 8 circle marks. Rotation: counter-clockwise with coordinate transformation. |
| V0.2-M1-P3-T4 | Update documentation and commit | Complete | TASKLOG, REVERSE_PROMPT updated. |

## Notes

- Form-space coordinates are in mm from the top-left of the rightside-up form. Page-space coordinates are in mm from the top-left of the physical PDF page.
- For 90-degree rotations, the X and Y axes swap, and coordinates are scaled by W/H or H/W because the form's aspect ratio is preserved but its axes are transposed onto the page.
- The transformation was verified algebraically at all four page corners for each rotation mode.
- Previous notes on text rotation angles, bezier circles, and printpdf API patterns remain valid.

## History

| Date | Change |
|------|--------|
| 2026-02-19 | V0.0: Process definition, knowledge graph, PDF analysis, roadmap, decision resolution. |
| 2026-02-19 | V0.1-M1-P1: Project structure generated. rztamp Rust library, ztamp Phoenix project, NIF bridge with Rustler, secrets.toml.example. |
| 2026-02-19 | V0.1-M1-P2: Field extraction and form offsets. secrets.toml populated, form_offsets.toml created in assets/form/, documentation updated. |
| 2026-02-19 | V0.1-M1-P3: Documentation update and V0.1 status report. V0.1 declared complete. |
| 2026-02-19 | V0.2-M1-P1: PDF Write PoC. printpdf integration, rztamp pdf/offsets modules, tanf-fill CLI tool, calibration sample generated. |
| 2026-02-19 | V0.2-M1-P2: Rotation support. Rotation enum added to rztamp::pdf, --rotation flag added to tanf-fill, calibration sample regenerated with counter-clockwise rotation. |
| 2026-02-19 | V0.2-M1-P3: Coordinate transformation. transform_position() added to map form-space to page-space for rotated forms. X/Y swap with aspect-ratio scaling for 90-degree modes. |

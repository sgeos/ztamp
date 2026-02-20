# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Form Dimension Fix (V0.2-M1-P5)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Fill tool revised to correct form dimension assumption.
- [x] Sample output placed in `secret/`.
- [x] Manual calibration command included.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P5-T1 | Fix form dimensions for 90-degree rotations | Complete | Added `form_dimensions()` method: for CCW/CW 90, form_w = page_h, form_h = page_w. Updated `transform_position()` to use form dimensions. CCW 90 simplifies to page_x = fy, page_y = page_h - fx. No non-uniform scaling. |
| V0.2-M1-P5-T2 | Fix grid iteration ranges | Complete | `build_grid_ops()` now iterates form-X from 0 to form_w (279.4 for CCW) and form-Y from 0 to form_h (215.9 for CCW). Grid cells are square. |
| V0.2-M1-P5-T3 | Generate calibration sample | Complete | `secret/calibration_sample.pdf` (100,584 bytes). 5mm grid, green, CCW rotation. X labels to x275, Y labels to y215. Square grid cells. |
| V0.2-M1-P5-T4 | Update documentation and commit | Complete | TASKLOG, REVERSE_PROMPT updated. Manual calibration command included. |

## Notes

- The false assumption was that the form had the same dimensions as the portrait page (215.9 x 279.4). The form is actually landscape (279.4 wide x 215.9 tall) when read rightside-up, rotated 90 degrees CCW to fit in the portrait image.
- For CCW 90: form_w = page_h = 279.4, form_h = page_w = 215.9. The transform becomes page_x = fy, page_y = page_h - fx. No W/H scaling factors.
- Grid cells are now square because both axes use the same mm-per-mm mapping (no aspect ratio distortion).
- The existing form_offsets.toml values (originally estimated for portrait coordinates) will need re-estimation in the landscape coordinate system. The grid overlay enables this.
- Previous notes on text rotation angles, bezier circles, and printpdf API remain valid.

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
| 2026-02-19 | V0.2-M1-P4: Grid overlay. --grid and --grid-color flags, GridConfig, build_grid_ops(). Form-space grid lines with labels for calibration. |
| 2026-02-19 | V0.2-M1-P5: Form dimension fix. For 90-degree rotations, form dimensions are swapped (landscape form in portrait page). Eliminated non-uniform scaling. Square grid cells. |

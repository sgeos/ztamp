# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Employment Date Fix (V0.2-M1-P12)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] If employed dates modified.
- [x] Latest calibration test exported.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P12-T1 | Update employed dates to 2001 | Complete | `employed_start_date` changed from "04/15/2026" to "04/15/2001". `employed_first_check_year` changed from "2026" to "2001". |
| V0.2-M1-P12-T2 | Generate PoC | Complete | `secret/calibration_sample.pdf` (89,524 bytes). No grid, black text, no labels, CCW rotation. 104 text fields, 8 circle marks. |

## Notes

- Only two string values changed in `build_value_map()`. No structural code changes.
- Previous notes on printpdf API, form dimensions, coordinate system, alignment, labels flag, and text width estimation remain valid.

## History

| Date | Change |
|------|--------|
| 2026-02-19 | V0.0: Process definition, knowledge graph, PDF analysis, roadmap, decision resolution. |
| 2026-02-19 | V0.1-M1-P1: Project structure generated. rztamp Rust library, ztamp Phoenix project, NIF bridge with Rustler, secrets.toml.example. |
| 2026-02-19 | V0.1-M1-P2: Field extraction and form offsets. secrets.toml populated, form_offsets.toml created in assets/form/, documentation updated. |
| 2026-02-19 | V0.1-M1-P3: Documentation update and V0.1 status report. V0.1 declared complete. |
| 2026-02-19 | V0.2-M1-P1: PDF Write PoC. printpdf integration, rztamp pdf/offsets modules, tanf-fill CLI tool, calibration sample generated. |
| 2026-02-19 | V0.2-M1-P2: Rotation support. Rotation enum added to rztamp::pdf, --rotation flag added to tanf-fill. |
| 2026-02-19 | V0.2-M1-P3: Coordinate transformation. transform_position() added to map form-space to page-space for rotated forms. |
| 2026-02-19 | V0.2-M1-P4: Grid overlay. --grid and --grid-color flags, GridConfig, build_grid_ops(). |
| 2026-02-19 | V0.2-M1-P5: Form dimension fix. Landscape form in portrait page. Square grid cells. |
| 2026-02-19 | V0.2-M1-P6: Offset rescaling. Portrait to landscape coordinate system. |
| 2026-02-19 | V0.2-M1-P7: Debug data fixes. Four-digit years, larger insurance ellipses, text centering. |
| 2026-02-19 | V0.2-M1-P8: Alignment field. Left, Center, Right alignment support. |
| 2026-02-19 | V0.2-M1-P9: Black text PoC. --text-color flag. |
| 2026-02-19 | V0.2-M1-P10: PoC regeneration with human-adjusted alignments and positions. |
| 2026-02-19 | V0.2-M1-P11: Labels flag. --labels flag, circle labels suppressed by default, signature dates "01/01/2001". |
| 2026-02-19 | V0.2-M1-P12: Employment date fix. Start date and first check year changed to 2001. |

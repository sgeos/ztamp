# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Offset Rescaling (V0.2-M1-P6)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Revise field offsets.
- [x] Sample output placed in `secret/`.
- [x] Manual calibration command included.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P6-T1 | Rescale form_offsets.toml coordinates | Complete | All X values multiplied by 279.4/215.9 (1.2941 stretch). All Y values multiplied by 215.9/279.4 (0.7727 squash). Table first_row_y and row_height scaled by Y factor. Table column X values scaled by X factor. Widths and font sizes unchanged. |
| V0.2-M1-P6-T2 | Generate calibration sample | Complete | `secret/calibration_sample.pdf` (100,615 bytes). 5mm grid, green, CCW rotation. 112 text fields, 8 circle marks. |
| V0.2-M1-P6-T3 | Update documentation and commit | Complete | TASKLOG, REVERSE_PROMPT updated. Manual calibration command included. |

## Notes

- Scaling factors: X stretch = 279.4 / 215.9 = 1.2941, Y squash = 215.9 / 279.4 = 0.7727.
- The rescaling is a linear proportional adjustment. The human pilot will need to manually fine-tune offsets using the grid overlay.
- The office_use_only column X (282.1mm) slightly exceeds the form width (279.4mm) because the original value (218.0mm) was slightly beyond the old coordinate range (215.9mm). This column is not populated.
- Table row_height scaled from 6.0mm to 4.6mm. This may need manual adjustment since row spacing depends on the physical form layout.
- Previous notes on text rotation angles, bezier circles, printpdf API, and form dimensions remain valid.

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
| 2026-02-19 | V0.2-M1-P6: Offset rescaling. All X/Y values in form_offsets.toml rescaled from portrait to landscape coordinate system. X stretched by 1.2941, Y squashed by 0.7727. |

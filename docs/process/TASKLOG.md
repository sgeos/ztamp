# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Debug Data and Text Centering (V0.2-M1-P7)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Debug printing data revised as specified.
- [x] PDF export tool revised as specified.
- [x] Latest calibration test exported.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P7-T1 | Fix four-digit years in debug data | Complete | `employed_start_date` changed to "04/15/2026", `employed_first_check_year` to "2026", table date column to "01/{row}/2026" format. Insurance debug labels changed to "Employer Paid", "Employee Paid", "Both Paid" for larger ellipses. |
| V0.2-M1-P7-T2 | Add text centering in calibration fields | Complete | `center_in_width()` helper added to rztamp::pdf. Centers text within field width when width is available. Applied to both named fields and table columns. Circle-one options (no width) remain left-aligned. |
| V0.2-M1-P7-T3 | Generate calibration sample | Complete | `secret/calibration_sample.pdf` (100,661 bytes). 5mm grid, green, CCW rotation. 112 text fields, 8 circle marks. |
| V0.2-M1-P7-T4 | Update documentation and commit | Complete | TASKLOG, REVERSE_PROMPT updated. |

## Notes

- Text centering uses `estimate_text_width()` (Helvetica average character width approximation at 0.5em). This is a rough estimate. For precise centering, per-character width tables would be needed.
- The `center_in_width()` function falls back to left-aligned positioning when text is wider than the field or when no width is specified.
- Insurance ellipses are now sized based on "Employer Paid" (13 chars), "Employee Paid" (13 chars), and "Both Paid" (9 chars) rather than the previous short labels (8, 8, 4 chars).
- The human pilot manually updated form_offsets.toml positions between P6 and P7. Those values are preserved.
- Previous notes on printpdf API, form dimensions, and coordinate system remain valid.

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
| 2026-02-19 | V0.2-M1-P7: Debug data fixes and text centering. Four-digit years, larger insurance ellipses, text centered within field widths. |

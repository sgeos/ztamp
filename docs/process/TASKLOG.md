# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Black Text PoC (V0.2-M1-P9)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Black text and no grid for PoC.
- [x] Latest calibration test exported.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P9-T1 | Add --text-color flag to CLI | Complete | `--text-color <color>` flag added to tanf-fill. When specified, all text and circle marks use the given color instead of calibration colors (red/blue/magenta). Accepts same named colors as --grid-color. |
| V0.2-M1-P9-T2 | Generate black text PoC | Complete | `secret/calibration_sample.pdf` (90,565 bytes). No grid, black text, CCW rotation. 112 text fields, 8 circle marks. |
| V0.2-M1-P9-T3 | Update documentation and commit | Complete | TASKLOG, REVERSE_PROMPT updated. |

## Notes

- The --text-color flag is optional. Without it, calibration colors (red headers, blue/magenta alternating table rows) are used as before.
- The smaller PDF size (90,565 vs 100,661 bytes) is due to the absence of grid lines.
- The human pilot's manually adjusted alignment and position values in form_offsets.toml are preserved.
- Previous notes on printpdf API, form dimensions, coordinate system, alignment, and text width estimation remain valid.

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
| 2026-02-19 | V0.2-M1-P8: Alignment field. Alignment enum (Left, Center, Right) added to offsets and PDF rendering. All fields set to "center". |
| 2026-02-19 | V0.2-M1-P9: Black text PoC. --text-color flag added to tanf-fill. Black text, no grid output generated. |

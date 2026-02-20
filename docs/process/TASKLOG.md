# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Alignment Field (V0.2-M1-P8)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Offsets file revised as specified.
- [x] PDF export tool revised as specified.
- [x] Latest calibration test exported.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P8-T1 | Add Alignment enum to offsets module | Complete | `Alignment` enum (Left, Center, Right) added to `rztamp/src/offsets.rs`. Added `alignment` field to both `FieldOffset` and `ColumnOffset` structs with `#[serde(default)]` (defaults to Left). Uses `#[serde(rename_all = "lowercase")]` for TOML values. |
| V0.2-M1-P8-T2 | Support alignment in PDF rendering | Complete | `center_in_width()` replaced by `align_in_width()` in `rztamp/src/pdf.rs`. Handles Left (no offset), Center (half remaining space), Right (full remaining space). Falls back to left-aligned when text exceeds width or no width is specified. |
| V0.2-M1-P8-T3 | Add alignment to form_offsets.toml | Complete | `alignment = "center"` added to all 28 field entries and all 9 table column entries. Human will manually adjust individual fields as needed. |
| V0.2-M1-P8-T4 | Generate calibration sample | Complete | `secret/calibration_sample.pdf` (100,661 bytes). 5mm grid, green, CCW rotation. 112 text fields, 8 circle marks. |

## Notes

- The `Alignment` enum defaults to `Left` when not specified in the TOML. All current entries are explicitly set to "center".
- Circle-one options have alignment = "center" in the TOML but no width, so alignment has no effect on them. The ellipse and text positioning for circle-one options is unchanged.
- The human pilot's manually adjusted coordinate values from P7 are preserved. Only the alignment field was added.
- Previous notes on printpdf API, form dimensions, coordinate system, and text width estimation remain valid.

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

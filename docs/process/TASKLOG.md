# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Watermark and Three PoCs (V0.2-M1-P13)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Watermark flag added to export tool.
- [x] Three PoCs generated.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P13-T1 | Add WatermarkConfig and --watermark flag | Complete | `WatermarkConfig` struct added to rztamp::pdf. `--watermark <color>` flag added to CLI. Renders "TEST SAMPLE" at 72pt diagonally (45 degrees) centered on the page, on top of all other content. |
| V0.2-M1-P13-T2 | Add --circle-all flag | Complete | `--circle-all` flag and `circle_all: bool` parameter added. Without it, only one option per circle series is circled (Weekly for pay frequency, None for insurance). With it, all options are circled. |
| V0.2-M1-P13-T3 | Generate Almost Production PoC | Complete | `secret/poc_production.pdf` (87,965 bytes). Black text, no grid, no labels, 2 circle marks (one per series), red watermark. |
| V0.2-M1-P13-T4 | Generate Black Text Testing PoC | Complete | `secret/poc_testing.pdf` (89,632 bytes). Black text, no grid, no labels, 8 circle marks (all), red watermark. |
| V0.2-M1-P13-T5 | Generate Debug PoC | Complete | `secret/poc_debug.pdf` (100,544 bytes). Calibration colors, 5mm green grid, labels, 8 circle marks (all), red watermark. |

## Notes

- The `is_default_selected()` helper determines which single option per series to circle: `employed_pay_frequency_weekly` and `employed_insurance_none`.
- The watermark is positioned using a simple center-of-page calculation with 45-degree rotation. Text width is estimated at 0.5em per character to offset the starting position.
- Previous notes on printpdf API, form dimensions, coordinate system, alignment, labels flag, and text width estimation remain valid.

## History

| Date | Change |
|------|--------|
| 2026-02-19 | V0.0-V0.1: Project setup, structure, field extraction, documentation. |
| 2026-02-19 | V0.2-M1-P1 through P6: PDF Write PoC, rotation, coordinate transform, grid, dimension fix, offset rescaling. |
| 2026-02-19 | V0.2-M1-P7: Debug data fixes. Four-digit years, larger insurance ellipses, text centering. |
| 2026-02-19 | V0.2-M1-P8: Alignment field. Left, Center, Right alignment support. |
| 2026-02-19 | V0.2-M1-P9: Black text PoC. --text-color flag. |
| 2026-02-19 | V0.2-M1-P10: PoC regeneration with human-adjusted alignments. |
| 2026-02-19 | V0.2-M1-P11: Labels flag. --labels, circle labels suppressed by default, signature dates "01/01/2001". |
| 2026-02-19 | V0.2-M1-P12: Employment date fix. Start date and first check year changed to 2001. |
| 2026-02-19 | V0.2-M1-P13: Watermark and three PoCs. --watermark and --circle-all flags. Three PoC variants generated. |

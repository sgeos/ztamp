# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Four PoC Variants (V0.2-M1-P15)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Debug fill flag added to export tool.
- [x] Four PoCs generated.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P15-T1 | Fix build_calibration_fields to skip missing values | Complete | Named fields with no matching value in the map are now skipped entirely (previously rendered as uppercase field name). |
| V0.2-M1-P15-T2 | Gate table generation behind fill_table parameter | Complete | `fill_table: bool` parameter added to `build_calibration_fields`. When false, job search table rows are not generated. Caller passes `args.debug_fill`. |
| V0.2-M1-P15-T3 | Generate Production PoC | Complete | `secret/poc_production.pdf` (77,578 bytes). 8 text fields (secrets only), 0 circle marks, black text, no grid, no labels, no watermark, no debug fill. |
| V0.2-M1-P15-T4 | Generate Almost Production PoC | Complete | `secret/poc_almost_production.pdf` (87,965 bytes). 104 text fields, 2 circle marks, black text, no grid, no labels, red watermark, debug fill. |
| V0.2-M1-P15-T5 | Generate Black Text Testing PoC | Complete | `secret/poc_testing.pdf` (89,632 bytes). 104 text fields, 8 circle marks, black text, no grid, no labels, red watermark, debug fill. |
| V0.2-M1-P15-T6 | Generate Debug PoC | Complete | `secret/poc_debug.pdf` (100,544 bytes). 112 text fields, 8 circle marks, calibration colors, 5mm green grid, labels, red watermark, debug fill. |

## Notes

- The `build_calibration_fields` function now skips named fields with no value in the map instead of falling back to the uppercase field name. This means the `--debug-fill` flag has a meaningful effect: without it, only secrets-sourced fields appear.
- The `fill_table` parameter gates hardcoded sample table data. Without `--debug-fill`, the table is empty.
- Previous notes on printpdf API, form dimensions, coordinate system, alignment, labels flag, watermark, circle-all, and text width estimation remain valid.

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
| 2026-02-19 | V0.2-M1-P14: Debug fill flag. --debug-fill gates hardcoded test data. Three PoCs regenerated. |
| 2026-02-19 | V0.2-M1-P15: Four PoC variants. Fixed missing-value handling (skip instead of uppercase fallback). Gated table behind fill_table. Production PoC with secrets-only data. |

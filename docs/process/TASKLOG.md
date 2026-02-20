# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.2 Debug Fill Flag and PoC Regeneration (V0.2-M1-P14)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Debug fill flag added to export tool.
- [x] Three PoCs generated.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.2-M1-P14-T1 | Add --debug-fill flag | Complete | `--debug-fill` boolean flag added to CLI Args. When set, `build_value_map` includes hardcoded test data (signatures, employment section, pay frequency, insurance). Without it, only secrets-sourced fields are populated. |
| V0.2-M1-P14-T2 | Generate Almost Production PoC | Complete | `secret/poc_production.pdf` (87,965 bytes). Black text, no grid, no labels, 2 circle marks (one per series), red watermark, --debug-fill. |
| V0.2-M1-P14-T3 | Generate Black Text Testing PoC | Complete | `secret/poc_testing.pdf` (89,632 bytes). Black text, no grid, no labels, 8 circle marks (all), red watermark, --debug-fill. |
| V0.2-M1-P14-T4 | Generate Debug PoC | Complete | `secret/poc_debug.pdf` (100,544 bytes). Calibration colors, 5mm green grid, labels, 8 circle marks (all), red watermark, --debug-fill. |

## Notes

- The `build_value_map` function now takes a `debug_fill: bool` parameter. Secrets-sourced fields (case_name, upi_number, job_search, submission) are always included. Debug test data (signatures, employment, pay frequency, insurance) requires `--debug-fill`.
- Production use will supply real data for signature, employment, and other fields through a mechanism to be determined. Without `--debug-fill`, those fields are simply absent from the value map and not rendered.
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

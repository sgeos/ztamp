# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.3 PDF Concatenation Tool (V0.3-M1-P1)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] V0.2 merged into main.
- [x] Concatenation tool generated.
- [x] Four PoCs concatenated.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.3-M1-P1-T1 | Merge V0.2 into main | Complete | `feat/v02-pdf-write-poc` fast-forwarded into `main`. All V0.2 commits now on main. |
| V0.3-M1-P1-T2 | Create feat/v03-pdf-concatenation branch | Complete | Branch created from main at commit 6e91a09. |
| V0.3-M1-P1-T3 | Create tanf-concat tool | Complete | New binary `tanf-concat` added to tools/. Uses `lopdf` 0.39 for PDF merging. CLI: `tanf-concat --output <path> <input1.pdf> <input2.pdf> ...`. Refuses to overwrite existing output. Requires at least two input files. |
| V0.3-M1-P1-T4 | Concatenate four PoCs | Complete | `secret/poc_combined.pdf` (354,455 bytes). Contains all four PoC pages in order: production, almost production, testing, debug. |

## Notes

- The `lopdf` crate (0.39) is used for PDF merging. The merge follows the canonical lopdf pattern: renumber object IDs, collect pages and objects, stitch together the Catalog and Pages tree.
- Bookmark/outline objects from source documents are discarded during merge (standard lopdf behavior).
- The `rayon` feature is enabled by default in lopdf for parallelized PDF parsing.
- Previous notes on printpdf API, form dimensions, coordinate system, alignment, labels flag, watermark, circle-all, debug-fill, and text width estimation remain valid.

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
| 2026-02-19 | V0.2-M1-P15: Four PoC variants. Fixed missing-value handling. Gated table behind fill_table. Production PoC with secrets-only data. |
| 2026-02-19 | V0.3-M1-P1: V0.2 merged to main. PDF concatenation tool created. Four PoCs concatenated. |

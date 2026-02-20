# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.5 Bug Fixes and PDF Export (V0.5-M1-P4)
**Status**: Complete
**Started**: 2026-02-20

## Success Criteria

- [x] Update job search dashboard as requested.
- [x] Fix date filtering.
- [x] Update captured entry display.
- [x] Implement Export to PDF functionality.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.5-M1-P4-T1 | Fix "How Contact Made" default | Complete | Changed from "Online" to "Internet" in `default_attrs/0` and template select options. |
| V0.5-M1-P4-T2 | Fix date filtering crash | Complete | Changed `handle_event("filter_entries", ...)` from pattern matching to `Map.get/3` to handle disabled inputs not submitting values. |
| V0.5-M1-P4-T3 | Update employer display | Complete | Added `format_employer/1` function replicating migration down-function formatting. Template uses it for the Employer column. |
| V0.5-M1-P4-T4 | Add PNG support to rztamp | Complete | Added "png" feature to printpdf dependency. |
| V0.5-M1-P4-T5 | Add `build_table_fields` to rztamp | Complete | New function accepts custom table row data as `Vec<HashMap<String, String>>` and positions text using form_offsets.toml table config. |
| V0.5-M1-P4-T6 | Add `generate_text_page` to rztamp | Complete | New function creates a text-only PDF page for the cover page. |
| V0.5-M1-P4-T7 | Add `generate_image_page` to rztamp | Complete | New function creates a PDF page with a scaled PNG image and header text for screenshot pages. |
| V0.5-M1-P4-T8 | Create `tanf-export` CLI | Complete | New binary reads JSON manifest, secrets, offsets, and template. Generates cover page, TANF form pages (10 entries each), screenshot pages (1 per entry), and merges all into a single PDF. Integration tested with 2 entries producing 4-page PDF. |
| V0.5-M1-P4-T9 | Create `Ztamp.PdfExport` module | Complete | Elixir orchestration module that formats entries, writes JSON manifest, and calls tanf-export CLI. |
| V0.5-M1-P4-T10 | Wire up Export PDF button | Complete | Button enabled when entries exist. Triggers `export_pdf` event. Flash message with output filename on success. |

## Notes

- The export uses the currently listed entries (respecting date filters).
- Entries are sorted oldest-to-newest for the export.
- Cover page includes name, UPI, period, application count, recruiter count, and total time.
- TANF form pages use CCW 90-degree rotation with the template.tiff background and black text.
- The `job_search_hours` field on the form uses the value from secrets.toml (required hours).
- The `job_search_from` and `job_search_to` fields on the form use dates from the oldest and newest entries.
- Screenshot pages show header info (name, UPI, date, employer, contact, time) above the scaled screenshot.
- The tanf-export binary must be built before use: `cd tools && cargo build --release`.

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
| 2026-02-20 | V0.4-M1-P1: V0.3 merged to main. Screenshot capture workflow with Wallaby, LiveView, and PostgreSQL. |
| 2026-02-20 | V0.4-M1-P2: ChromeDriver tested. Local time for form defaults. Submission time checkbox for time_out. README updated. |
| 2026-02-20 | V0.4-M1-P3: V0.4 merged to main. Schema migration, form improvements, date filtering, entry modal, screenshot serving, browser landing page, README documentation. |
| 2026-02-20 | V0.5-M1-P4: Bug fixes (default, filtering, display). PDF export with cover page, TANF forms, and screenshot pages. |

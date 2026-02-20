# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.5 Dashboard, Export, and Landing Page Improvements (V0.5-M1-P5)
**Status**: Complete
**Started**: 2026-02-20

## Success Criteria

- [x] Update job search dashboard as requested.
- [x] Fix date filtering.
- [x] Update Phoenix landing page.
- [x] Style all pages according to the selected landing page style.
- [x] Update export manifest location.
- [x] Update PDF export as requested.
- [x] Add watermarking checkbox.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.5-M1-P5-T1 | Remove Internet Confirmation # from form | Complete | Removed input from main form and edit modal. Auto-set to "Yes" on entry creation. Table column shows Yes/No based on screenshot_path. |
| V0.5-M1-P5-T2 | Fix "To present" checkbox position | Complete | Moved checkbox above the To Date input, beside the label. |
| V0.5-M1-P5-T3 | Update Phoenix landing page | Complete | Replaced default Phoenix page with TANF-specific cards linking to Job Search, LiveView Dashboard, and Mailbox. Updated app navbar with Home, Job Search, Dashboard links and TANF branding. Widened main container to max-w-4xl. |
| V0.5-M1-P5-T4 | Update export manifest location | Complete | Changed manifest temp file from `/tmp` to `secret/`. Manifest is cleaned up after export. |
| V0.5-M1-P5-T5 | Add signatures to PDF export | Complete | Top and bottom signature fields populated with "SIGNATURE HERE" and export date (MM/DD/YYYY). Export date added to manifest JSON. |
| V0.5-M1-P5-T6 | Auto-set internet confirmation in PDF | Complete | PDF export sets internet_confirmation to "Yes" if screenshot_path is not empty, "No" otherwise. Handled in both Elixir manifest builder and Rust table row builder. |
| V0.5-M1-P5-T7 | Rotate landscape screenshots | Complete | Screenshots wider than tall are rotated 90 degrees CCW before placement in the PDF. Uses image crate for detection and rotation. |
| V0.5-M1-P5-T8 | Add watermark support | Complete | Watermark checkbox next to Export PDF button. When enabled, all pages get "TEST SAMPLE" diagonal red text via WatermarkConfig. Extracted shared build_watermark_ops helper in rztamp. Added --watermark flag to tanf-export CLI. |

## Notes

- Theme persistence is handled by the existing root layout JavaScript. Theme selected on any page applies to all pages.
- The watermark uses a washed-out red color (RGB 1.0, 0.5, 0.5) at 72pt to approximate 50% opacity without PDF graphics state manipulation.
- The tanf-export binary must be rebuilt after this update: `cd tools && cargo build --release`.
- Landscape screenshot rotation uses the `image` crate's `rotate270()` (CCW 90) and re-encodes as PNG.

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
| 2026-02-20 | V0.5-M1-P5: Dashboard improvements, landing page, export enhancements (signatures, rotation, watermark), manifest location. |

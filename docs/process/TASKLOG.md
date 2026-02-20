# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.4 UI Improvements and Schema Migration (V0.4-M1-P3)
**Status**: Complete
**Started**: 2026-02-20

## Success Criteria

- [x] V0.4 merged into main.
- [x] Update database schema as requested.
- [x] Update job search dashboard as requested.
  - [x] Job search fields.
  - [x] Captured entries.
  - [x] Detailed entry modification dialog.
- [x] Document installation and startup instructions in relevant READMEs.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.4-M1-P3-T1 | Merge V0.4 into main | Complete | `feat/v04-screenshot-capture` fast-forwarded into `main`. New branch `feat/v05-ui-improvements` created. |
| V0.4-M1-P3-T2 | Database migration | Complete | `employer_name_address` split into `employer_name` and `employer_address`. Added `applied_via_recruiter` and `remote` boolean columns. Existing 23 entries preserved. Reversible `down` function with formatting rules. |
| V0.4-M1-P3-T3 | Screenshot controller | Complete | `ScreenshotController` serves PNGs from `secret/screenshots/` with path traversal protection. Route at `/screenshots/:filename`. |
| V0.4-M1-P3-T4 | Landing page | Complete | `LandingController` serves browser landing page at `/browser-landing` with links to LinkedIn Jobs, Indeed, and Glassdoor. BrowserServer navigates to landing page on session start. |
| V0.4-M1-P3-T5 | Modal component | Complete | `.modal` added to CoreComponents using daisyUI modal classes. Show/hide via `modal-open` class. Close button and backdrop click. |
| V0.4-M1-P3-T6 | Form changes | Complete | Employer Name/Address split with "Applied via Recruiter", "United States", and "Remote" checkboxes. "How Contact Made" defaults to "Online". T/F/E/O selector with "Online Application" default. |
| V0.4-M1-P3-T7 | Date filtering | Complete | From/To date inputs with "To present" checkbox. Entry count and cumulative time badges. |
| V0.4-M1-P3-T8 | Entry detail modal | Complete | View/edit/delete modal with screenshot thumbnail. Click thumbnail opens full size in new tab. Edit form includes all fields. Delete with confirmation. |
| V0.4-M1-P3-T9 | Disabled Export PDF button | Complete | Disabled "Export PDF" button in captured entries header area. Placeholder for future functionality. |
| V0.4-M1-P3-T10 | README documentation | Complete | Created top-level `README.md` and `rztamp/README.md`. Updated `ztamp/README.md` with Rust prerequisite, workflow description, and screenshot route. |

## Notes

- Existing 23 entries have `employer_name` populated from the old `employer_name_address` column. `employer_address` is empty string for these entries. `applied_via_recruiter` and `remote` default to false.
- The edit modal includes "Internet" in the How Contact Made dropdown for backward compatibility with existing entries. New entries default to "Online".
- Cumulative time is computed client-side from the `time_in` and `time_out` strings in `H:MM` format.
- The "United States" checkbox disables the address field and injects "United States" at submission time.
- The "Remote" checkbox sets the `remote` boolean field on the entry.

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

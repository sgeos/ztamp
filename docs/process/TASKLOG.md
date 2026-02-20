# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.4 Screenshot Testing and Polish (V0.4-M1-P2)
**Status**: Complete
**Started**: 2026-02-20

## Success Criteria

- [x] Test ChromeDriver installation and screenshot functionality.
- [x] Update ztamp `README.md`.
- [x] Comment on anything the human pilot needs to do to manually verify.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.4-M1-P2-T1 | Test ChromeDriver installation | Complete | ChromeDriver v145.0.7632.77 matches Chrome v145.0.7632.77. Wallaby starts Chrome session, captures screenshot (13,189 bytes), saves to `secret/screenshots/`. |
| V0.4-M1-P2-T2 | Test screenshot end-to-end | Complete | `Wallaby.start_session()`, `Wallaby.Browser.take_screenshot()`, file verified on disk. Full pipeline works. |
| V0.4-M1-P2-T3 | Update time fields to local time | Complete | `time_in` autopopulates with current local time. `time_out` has "Use submission time" checkbox (checked by default). When checked, field is disabled and time_out is set to local time at submission. When unchecked, manual entry is allowed. |
| V0.4-M1-P2-T4 | Update ztamp README.md | Complete | Added ChromeDriver dependency, `mix ecto.create` and `mix ecto.migrate` instructions. |

## Notes

- ChromeDriver v145.0.7632.77 is installed at `/Users/bsechter/bin/chromedriver` and matches Chrome v145.0.7632.77.
- The ChromeDriver blocker from V0.4-M1-P1 is resolved.
- Local time is obtained via `:calendar.local_time()` which uses the system timezone.
- The "Use submission time" checkbox toggles `time_out` between disabled (auto-populated at submission) and enabled (manual entry).

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

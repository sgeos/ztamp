# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.5 Reverse Prompt Response (V0.5-M1-P6)
**Status**: Complete
**Started**: 2026-02-27

## Success Criteria

- [x] Changes mentioned in the comments implemented.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.5-M1-P6-T1 | Change watermark to pure red | Complete | Changed watermark color from washed-out red (1.0, 0.5, 0.5) to `TextColor::RED` (1.0, 0.0, 0.0) in tanf-export. |
| V0.5-M1-P6-T2 | Gate dev-only links | Complete | Dashboard navbar link gated with `:if={@dev_routes}`. Landing page Dashboard and Mailbox cards gated with `:if={@dev_routes}`. Uses `Application.compile_env(:ztamp, :dev_routes, false)` via module attribute. |

## Notes

- The `dev_routes` flag is evaluated at compile time. In dev mode, the links appear. In prod mode, they are omitted from the rendered HTML.
- The PageController passes `dev_routes` as an assign to the home template. The Layouts module injects it via `assign_new` so it is available in the app layout navbar.

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
| 2026-02-27 | V0.5-M1-P6: Reverse prompt response. Pure red watermark. Dev-only link gating. |

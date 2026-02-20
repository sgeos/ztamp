# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: V0.4 Screenshot Capture Workflow (V0.4-M1-P1)
**Status**: Complete
**Started**: 2026-02-20

## Success Criteria

- [x] V0.3 merged into main.
- [x] V0.4 functionality implemented as best as possible as long as there are no blockers.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.4-M1-P1-T1 | Merge V0.3 into main | Complete | `feat/v03-pdf-concatenation` fast-forwarded into `main`. |
| V0.4-M1-P1-T2 | Add Wallaby dependency | Complete | `wallaby ~> 0.30` added to mix.exs with `runtime: false`. `:ex_unit` added to `extra_applications`. `mix deps.get` succeeded. |
| V0.4-M1-P1-T3 | Configure Wallaby | Complete | `config/dev.exs` configured with `Wallaby.Chrome` driver, `headless: false`, screenshot_dir pointing to `secret/screenshots/`. |
| V0.4-M1-P1-T4 | Create database migration | Complete | `job_search_entries` table created with date, employer_name_address, how_contact_made, telephone_fax, telephone_number, internet_confirmation, time_in, time_out, screenshot_path columns. `mix ecto.migrate` succeeded. |
| V0.4-M1-P1-T5 | Create JobSearch schema and context | Complete | `Ztamp.JobSearch.Entry` schema and `Ztamp.JobSearch` context module with CRUD operations. |
| V0.4-M1-P1-T6 | Create BrowserServer GenServer | Complete | `Ztamp.BrowserServer` manages a single Wallaby Chrome session. Client API: start_browser, stop_browser, take_screenshot, status. Added to supervision tree. |
| V0.4-M1-P1-T7 | Create JobSearchLive LiveView | Complete | Form with autopopulated defaults (today's date, current time). Browser control buttons. Screenshot capture button. Entries table. Route at `/job-search`. |
| V0.4-M1-P1-T8 | Create screenshot directory | Complete | `secret/screenshots/.gitkeep` created. |

## Notes

- **ChromeDriver blocker**: ChromeDriver is not installed. MacPorts provides v136 but Chrome is v145. User must install ChromeDriver v145 from Chrome for Testing (https://googlechromelabs.github.io/chrome-for-testing/) before the browser workflow can be tested end-to-end.
- Wallaby requires `:ex_unit` in `extra_applications` to function outside of test env. This is a known workaround.
- The BrowserServer starts in idle state (no Chrome session). Chrome launches only when the user clicks "Start Browser" on the LiveView page.
- `time_in`, `time_out`, and `screenshot_path` are required fields. The screenshot path is populated by the BrowserServer on capture.
- `row_number` is not stored in the database. Row position is determined at PDF export time.
- Form fields match the job search table columns from form_offsets.toml.

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

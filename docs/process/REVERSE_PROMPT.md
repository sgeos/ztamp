# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-20
**Task**: V0.4-M1-P1 Screenshot Capture Workflow

---

## Verification

### V0.3 Merged into Main

The `feat/v03-pdf-concatenation` branch was fast-forwarded into `main`.

### Screenshot Capture Workflow

All code compiles without warnings. The database migration has been applied. The `/job-search` route is registered.

**New files created:**

| File | Purpose |
|------|---------|
| `ztamp/lib/ztamp/browser_server.ex` | GenServer managing a single Wallaby Chrome session |
| `ztamp/lib/ztamp/job_search.ex` | Context module for CRUD operations on job search entries |
| `ztamp/lib/ztamp/job_search/entry.ex` | Ecto schema for job_search_entries table |
| `ztamp/lib/ztamp_web/live/job_search_live.ex` | LiveView page for the screenshot capture workflow |
| `ztamp/lib/ztamp_web/live/job_search_live.html.heex` | LiveView template with form, browser controls, entries table |
| `ztamp/priv/repo/migrations/20260220073115_create_job_search_entries.exs` | Database migration |
| `secret/screenshots/.gitkeep` | Screenshot storage directory |

**Modified files:**

| File | Change |
|------|--------|
| `ztamp/mix.exs` | Added `wallaby ~> 0.30`, added `:ex_unit` to `extra_applications` |
| `ztamp/config/dev.exs` | Added Wallaby config (Chrome driver, non-headless, screenshot_dir) |
| `ztamp/lib/ztamp/application.ex` | Added `Ztamp.BrowserServer` to supervision tree |
| `ztamp/lib/ztamp_web/router.ex` | Added `live "/job-search", JobSearchLive` route |

### Database Schema

The `job_search_entries` table has the following columns:

| Column | Type | Required |
|--------|------|----------|
| date | date | Yes |
| employer_name_address | string | Yes |
| how_contact_made | string | Yes |
| telephone_fax | string | No |
| telephone_number | string | No |
| internet_confirmation | string | No |
| time_in | string | Yes |
| time_out | string | Yes |
| screenshot_path | string | Yes |

### LiveView Page

Available at `http://localhost:4000/job-search`. The page provides:

1. **Browser Control** section with Start/Stop Browser and Refresh Status buttons, plus a status indicator.
2. **Entry Form** with all job search fields, autopopulated with today's date and current time.
3. **Take Screenshot & Save Entry** button that captures a screenshot via BrowserServer, then saves both the form data and screenshot path to the database.
4. **Captured Entries** table showing previously saved entries.

---

## Blocker: ChromeDriver

ChromeDriver v145 must be installed before the browser workflow can be tested end-to-end. MacPorts provides v136, which is incompatible with Chrome v145.

**To install ChromeDriver v145:**
1. Visit https://googlechromelabs.github.io/chrome-for-testing/
2. Download the ChromeDriver matching Chrome v145 for macOS (arm64 or x64)
3. Extract and place the `chromedriver` binary on `$PATH`
4. Alternatively, configure the path in `config/dev.exs`:
   ```elixir
   config :wallaby,
     chromedriver: [
       headless: false,
       path: "/path/to/chromedriver"
     ]
   ```

---

## Commands

### Start the Phoenix Server

```
cd ztamp && mix phx.server
```

Then navigate to `http://localhost:4000/job-search`.

### Reset the Database

```
cd ztamp && mix ecto.reset
```

---

## Questions for Human Review

1. **ChromeDriver installation.** ChromeDriver v145 is needed to test the full workflow. Should I proceed with other work while this prerequisite is resolved, or is there a preferred installation method?

2. **Form defaults.** The form autopopulates with today's date and the current UTC time for time_in and time_out. Should the time use a local timezone instead?

3. **V0.4 scope.** The core screenshot workflow is implemented. Are additional features needed on this branch (such as entry deletion, editing, or export to the PDF form) before merging?

---

## Notes

- Wallaby is configured with `runtime: false` to prevent automatic startup. The `BrowserServer` calls `Application.ensure_all_started(:wallaby)` on demand.
- The `:ex_unit` addition to `extra_applications` is required for Wallaby to function outside of test mode. This is a known community workaround.
- The `BrowserServer` GenServer is a named singleton. It starts idle and only launches Chrome when `start_browser/0` is called.
- Screenshots are saved to `secret/screenshots/` with timestamp-based filenames.

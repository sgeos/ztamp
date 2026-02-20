# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-20
**Task**: V0.4-M1-P2 Screenshot Testing and Polish

---

## Verification

### ChromeDriver

ChromeDriver v145.0.7632.77 is installed at `/Users/bsechter/bin/chromedriver` and matches Chrome v145.0.7632.77. The end-to-end test confirmed that Wallaby can start a Chrome session, capture a screenshot, and save it to `secret/screenshots/`.

### Time Field Changes

The form now uses local time instead of UTC.

- `time_in` autopopulates with the current local time.
- `time_out` has a "Use submission time" checkbox, checked by default.
  - When checked, the field is disabled. At submission, `time_out` is set to the current local time.
  - When unchecked, manual entry is allowed.

### README Updated

`ztamp/README.md` now documents the ChromeDriver dependency and database setup commands (`mix ecto.create`, `mix ecto.migrate`).

---

## Manual Verification

To verify the full workflow end-to-end, please do the following.

1. Start the Phoenix server: `cd ztamp && mix phx.server`
2. Navigate to `http://localhost:4000/job-search`
3. Click "Start Browser" to launch a Chrome window.
4. In the Chrome window, navigate to any job application site and reach a confirmation page.
5. Fill in the form fields on the LiveView page.
6. Click "Take Screenshot & Save Entry" to capture the screenshot and persist the entry.
7. Verify the entry appears in the "Captured Entries" table.
8. Verify the screenshot file exists in `secret/screenshots/`.

---

## Questions for Human Review

1. **End-to-end verification.** Have you been able to complete the manual verification steps above? If there are issues, please describe them so I can address them in the next prompt.

2. **V0.4 completeness.** Is the screenshot capture workflow sufficient for V0.4, or are additional features needed before merging (such as entry deletion, editing, URL pre-population, or export to the PDF form)?

---

## Notes

- The test screenshot file (`test_chromedriver.png`) was cleaned up after verification.
- Wallaby launches Chrome in non-headless mode with a window size of 1280x800.
- The BrowserServer GenServer manages a single Chrome session. Only one session can be active at a time.

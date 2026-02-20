# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-20
**Task**: V0.4-M1-P3 UI Improvements and Schema Migration

---

## Verification

### V0.4 Merged into Main

The `feat/v04-screenshot-capture` branch was fast-forwarded into `main`. Work continues on `feat/v05-ui-improvements`.

### Database Migration

The `employer_name_address` column has been split into `employer_name` and `employer_address`. Two new boolean columns have been added: `applied_via_recruiter` (default false) and `remote` (default false). All 23 existing entries are preserved with `employer_name` populated from the old column and `employer_address` set to empty string.

The `down` migration reconstructs `employer_name_address` with formatting rules:
- Normal: "NAME, ADDRESS"
- Recruiter: "Unknown (applied via NAME), ADDRESS"
- Remote: "NAME, ADDRESS (remote)"
- Both: "Unknown (applied via NAME), ADDRESS (remote)"

### Form Changes

| Change | Detail |
|--------|--------|
| Employer fields | Split into "Employer Name" and "Employer Address" |
| Applied via Recruiter | Checkbox next to employer name |
| United States | Checkbox next to address. Disables address field and submits "United States" |
| Remote | Checkbox next to address. Sets `remote` boolean on entry |
| How Contact Made | Defaults to "Online" (was "Select method...") |
| T/F/E/O | Added "Online Application" / "O" option, defaults to it |

### Date Filtering

Filter controls above the entries table with From date, To date, and "To present" checkbox. Displays entry count and cumulative application time (sum of time_out minus time_in for all listed entries, shown as hours and minutes).

### Entry Detail Modal

Each entry in the table has a "View" link that opens a modal dialog showing all entry fields in an editable form, a screenshot thumbnail (click to open full size in a new tab), and a delete button with confirmation.

### Screenshot Serving

Screenshots are served via `/screenshots/:filename` with path traversal protection. The controller validates that filenames contain no `..`, `/`, or `\` characters and end with `.png`.

### Browser Landing Page

When the browser starts, it navigates to `/browser-landing` which displays links to LinkedIn Jobs, Indeed, and Glassdoor.

### Disabled Export PDF Button

A disabled "Export PDF" button appears in the captured entries header area as a placeholder for future PDF packet export functionality.

### README Documentation

| File | Status |
|------|--------|
| `README.md` (top-level) | Created. Project description, component table, link to docs. |
| `rztamp/README.md` | Created. Library purpose, prerequisites, build instructions, dependencies, modules. |
| `ztamp/README.md` | Updated. Added Rust prerequisite, job search workflow description, screenshot route documentation. |

---

## Manual Verification

To verify the changes, please do the following.

1. Start the server: `cd ztamp && mix phx.server`
2. Navigate to `http://localhost:4000/job-search`
3. Verify the form shows the new split employer fields with checkboxes.
4. Verify "How Contact Made" defaults to "Online" and T/F/E/O defaults to "Online Application".
5. Check the "United States" checkbox and verify the address field is disabled.
6. Start the browser and verify it opens to the landing page with LinkedIn Jobs link.
7. Click "View" on an existing entry and verify the modal shows the screenshot thumbnail and editable fields.
8. Test editing and saving an entry from the modal.
9. Test the date filter controls and verify the entry count and cumulative time update.

---

## Questions for Human Review

1. **Existing entry data.** The 23 existing entries have `employer_address` set to empty string and `applied_via_recruiter`/`remote` set to false. Some entries already have recruiter and remote information embedded in the employer name text from manual entry (e.g., "Unknown (applied via Howrecruit), New York, NY"). Should these be cleaned up via the edit modal, or would a batch update be preferred?

2. **Export PDF.** The disabled "Export PDF" button is ready for the next prompt. Should the PDF export use the existing tanf-fill and tanf-concat tools, or is a different approach planned?

---

## Commands

### Start the Phoenix Server

```
cd ztamp && mix phx.server
```

### Reset the Database

```
cd ztamp && mix ecto.reset
```

### Rollback the Migration

```
cd ztamp && mix ecto.rollback
```

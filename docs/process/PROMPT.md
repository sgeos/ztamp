# Prompt Staging Area

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is a staging area for complex human-to-AI instructions. The human pilot drafts and refines prompts here before execution.

---

# Current Prompt

## Comments

Requested functionality has been verified as broadly correct.

Answers to questions in the reverse prompt.

1. Cover page is fine.
2. Used fields are fine.
3. Screenshots are fine.
   They could be rotated CCW if the height is smaller than the width.
4. Current date range for export filter is correct.

Fixes requested below.

## Objectives

### Update Job Search Dashboard

Remove "Internet Confirmation #."
If a screenshot was captured, then internet confirmation is "Yes".
If the screenshot fails, then it is "No".
PDF Export should reflect this.

### Fix Date Filtering

Date filtering is working.
The "To present" checkbox should be above or beside the data, and not below it.

### Update Phoenix Landing Page

Default Phoenix landing page is being used, and it is not useful.
Update the landing page to be useful.
It should have links to the job search page, live view dashboard,
and debug dashboards.
Theme selected on the home page should be applied to all other pages.

See the reference project's Elixir subproject for an example useful landing page.
`/Users/bsechter/projects/crypto/cordial_cantina/cordial_cantina/`

### Update Export Manifest Location

Use `secret` instead of `/tmp`.

### Update Exported PDF Packet

- Top and bottom of each TANF form should be signed and dated.
  - Use "SIGNATURE HERE" for now for the signature.
  - Use the export date for the date.
- Internet confirmation should be "Yes" if a screenshot is attached.
- Sort order is currently newest to oldest.
  It should be oldest to newest.
- Rotate screenshots 90 degrees CCW if they are wider than they are tall.

### Add Watermark Checkbox

Add a watermark checkbox next to PDF export.
If selected, all exported pages should have
"TEST SAMPLE" printed across the center of the page
in 50% opacity red text.

## Context

(no comment)

## Constraints

(no comment)

## Success Criteria

- Update job search dashboard as requested.
- Fix data filtering.
- Update Phoenix landing page.
- Style all pages according to the selected landing page style.
- Update export manifest location.
- Update PDF export as requested.
- Add watermarking checkbox.

## Notes

(none)

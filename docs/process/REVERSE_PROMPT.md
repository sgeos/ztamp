# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.1-M1-P2 Field Extraction and Form Offsets

---

## Verification

### Field Extraction (secret/secrets.toml)

Eight field values were extracted from the original PDF by visual inspection.

| Field | Value |
|-------|-------|
| Case Name | BRENDAN SECHTER |
| UPI Number | 58036400 |
| Job Search From | 03/16/2026 |
| Job Search To | 03/22/2026 |
| Job Search Hours | 30 |
| Submission Deadline Time | 05:00 pm |
| Submission Deadline Date | 04/01/2026 |
| Submission Location | Belrose |

The `secrets.toml.example` template was expanded to include `[job_search]` and `[submission]` sections matching the populated `secrets.toml`.

### Non-Confidential Assets Directory (assets/form/)

Created `assets/form/` to hold non-confidential form-related files, per the prompt comments requesting a directory for data such as the form template and field offsets.

| File | Description |
|------|-------------|
| `assets/form/template.tiff` | Cleaned form image (copied from `secret/form.tiff`). Base layer for PDF generation. |
| `assets/form/form_offsets.toml` | Field positions for all form fields. |

### Form Field Offsets (assets/form/form_offsets.toml)

Field offsets were generated for all fields listed in the prompt objectives. The coordinate system uses millimeters from the top-left corner of the page (US Letter, 215.9mm x 279.4mm). Fields covered include the following.

**Header fields**: case_name, upi_number, job_search_from, job_search_to, job_search_hours, submission_deadline_time, submission_deadline_date, submission_location.

**Signature fields (above table)**: participant_signature_top, participant_signature_top_date.

**Job application table**: 10 rows with 9 columns (date, employer_name_address, how_contact_made, telephone_fax, telephone_number, internet_confirmation, time_in, time_out, office_use_only). Row positions are computed from `first_row_y` and `row_height` parameters.

**"Should you become employed" section**: employer name/address/telephone (3 lines plus telephone), start_date, hours_per_week, hourly_rate, pay frequency options (weekly, biweekly, semimonthly, monthly), date of first check (month/day/year), tips, job_title, insurance options (none, employer_paid, employee_paid, both_paid).

**Signature fields (bottom of form)**: participant_signature_bottom, participant_signature_bottom_date.

All coordinates are initial estimates derived from visual inspection. They require calibration against the template image during V0.2 (PDF Write PoC).

### CLAUDE.md Updated

Repository structure diagram updated to include `assets/form/` with `template.tiff` and `form_offsets.toml`.

---

## Path Discrepancy Note

The prompt specified `secrets/form_offsets.toml` (with trailing 's'). The project convention uses `secret/` (without trailing 's'). Since form offsets describe a government form layout and are not confidential, they were placed in `assets/form/form_offsets.toml` instead. This follows the prompt comments that requested a non-confidential directory for the form and its offsets.

---

## Questions for Human Review

1. **Form offsets location.** The offsets were placed in `assets/form/form_offsets.toml` rather than `secret/form_offsets.toml`. Is this acceptable, or should they be moved to the `secret/` directory?

2. **Offset coordinate system.** Coordinates are in millimeters from the top-left corner. This maps naturally to how `printpdf` can work and to visual inspection of the template image. Is this coordinate system acceptable for V0.2, or would PDF points (origin at bottom-left) be preferred?

3. **Offset calibration.** The current offsets are estimates. During V0.2, they will need calibration by generating a test PDF and comparing the output against the template. Should the calibration be done interactively (generate, inspect, adjust) or should a programmatic approach be explored (for example, placing markers at known positions)?

4. **PDF library for rztamp.** The previous reverse prompt recommended `printpdf` for PDF generation. The prompt comments confirmed that PDF manipulation routines should be defined in the `rztamp` library. Should `printpdf` be added as a dependency to `rztamp` now, or deferred to the V0.2 prompt?

---

## Notes

- The `secret/form.png` file causes API errors due to a media type mismatch (the file metadata claims PNG but the content may not match). The TIFF version works correctly and was used as the canonical template.
- The form image is rotated 90 degrees counter-clockwise when viewed as a raw image. The PDF renderer corrects this orientation. The template.tiff retains the original orientation from the PDF.
- The example row in the job search table (showing "Made-Up Company" etc.) is part of the printed form, not filled-in data. It is visible in the template image and does not need to be cleared.

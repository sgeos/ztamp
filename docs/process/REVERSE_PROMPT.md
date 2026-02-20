# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P10 Black Text PoC Regeneration

---

## Verification

### Regenerated Output

The black text PoC was regenerated using the human-adjusted `form_offsets.toml` values. No code changes were needed.

### Sample Output

Generated at `secret/calibration_sample.pdf` (90,565 bytes, 112 text fields, 8 circle marks, no grid, black text, counter-clockwise rotation).

### Human Adjustments Observed

The following alignment and position changes were made by the human pilot between P9 and P10.

**Alignment changes from "center":**
- To "left": job_search_from, job_search_to, job_search_hours, submission_deadline_time, submission_deadline_date, submission_location, participant_signature_top, table date column, employed_name_address lines 1-3, employed_first_check_month/day/year, participant_signature_bottom
- To "right": employed_telephone

**Position and width refinements observed:**
- case_name, upi_number positions adjusted
- participant_signature_top_date x shifted
- Table column widths refined (how_contact_made, telephone_fax, telephone_number, internet_confirmation)
- employed_telephone x shifted, width adjusted
- employed_hours_per_week and employed_hourly_rate widths narrowed
- employed_first_check_year x adjusted
- employed_tips width narrowed
- employed_job_title width widened
- participant_signature_bottom_date x adjusted

---

## Commands

### Black Text PoC (no grid, black text)

```
rm -f secret/calibration_sample.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/calibration_sample.pdf \
  --rotation counter-clockwise \
  --text-color black
```

### Calibration Mode (grid, colored text)

```
rm -f secret/calibration_sample.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/calibration_sample.pdf \
  --rotation counter-clockwise \
  --grid 5
```

To rebuild after code changes, first run `cargo build --manifest-path tools/Cargo.toml`.

---

## Questions for Human Review

1. **Output quality.** Does the black text PoC with the adjusted alignments and positions look acceptable? Are further position or alignment adjustments needed, or is this ready to move toward production use?

---

## Notes

- No code changes were made in this task. The existing --text-color flag and alignment support handled the requirements.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P7 Debug Data and Text Centering

---

## Verification

### Four-Digit Years

All two-digit years in the debug data have been changed to four digits.

| Location | Old Value | New Value |
|----------|-----------|-----------|
| `employed_start_date` (fill.rs) | "04/15/26" | "04/15/2026" |
| `employed_first_check_year` (fill.rs) | "26" | "2026" |
| Table date column (pdf.rs) | "01/{row}/26" | "01/{row}/2026" |

### Larger Insurance Ellipses

The debug label text for the three specified insurance options was changed to match the form's printed labels. This naturally increases the ellipse dimensions since the radius is computed from text width.

| Field | Old Label | New Label | Approx Ellipse Width Change |
|-------|-----------|-----------|---------------------------|
| employed_insurance_employer_paid | "Employer" (8 chars) | "Employer Paid" (13 chars) | +63% wider |
| employed_insurance_employee_paid | "Employee" (8 chars) | "Employee Paid" (13 chars) | +63% wider |
| employed_insurance_both_paid | "Both" (4 chars) | "Both Paid" (9 chars) | +125% wider |

The "None" option was not mentioned and remains unchanged.

### Text Centering

A `center_in_width()` helper function was added to `rztamp/src/pdf.rs`. When a field or table column has a `width` value in `form_offsets.toml`, text is horizontally centered within that width. The centering uses the existing `estimate_text_width()` approximation (Helvetica average character width at 0.5em).

Behavior by field type:
- **Named fields with width**: Centered.
- **Table columns with width**: Centered.
- **Circle-one options (no width)**: Left-aligned from the x position, unchanged.
- **Text wider than field width**: Falls back to left-aligned.

### Sample Output

Generated at `secret/calibration_sample.pdf` (100,661 bytes, 112 text fields, 8 circle marks, 5mm green grid, counter-clockwise rotation).

---

## Manual Calibration Command

To regenerate the calibration PDF after editing `form_offsets.toml`, run the following from the repository root.

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

## Limitations

The text centering uses a rough width approximation (average character width = 0.5 em). Helvetica is a proportional font where character widths vary significantly (for example, "W" is much wider than "i"). For precise centering, a per-character width table would be needed. The current approximation is adequate for calibration but may appear slightly off-center for strings with many narrow or wide characters.

---

## Questions for Human Review

1. **Centering accuracy.** Please inspect the calibration PDF. Does the text centering appear acceptable, or does it need refinement for specific fields?

2. **Insurance ellipses.** Are the ellipses for "Employer Paid", "Employee Paid", and "Both Paid" now the right size? If any need further adjustment, the ellipse radius formula can be tuned.

---

## Notes

- The human pilot's manual position updates to `form_offsets.toml` (between P6 and P7) were preserved. No coordinate values were changed by the AI agent in this task.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

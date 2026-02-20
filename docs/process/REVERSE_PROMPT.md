# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P6 Offset Rescaling

---

## Verification

### Offset Rescaling (assets/form/form_offsets.toml)

All coordinate values in `form_offsets.toml` were rescaled from the old portrait coordinate system (215.9mm x 279.4mm) to the corrected landscape coordinate system (279.4mm x 215.9mm).

| Parameter | Scale Factor | Formula |
|-----------|-------------|---------|
| All X values | 1.2941 (stretch) | new_x = old_x * 279.4 / 215.9 |
| All Y values | 0.7727 (squash) | new_y = old_y * 215.9 / 279.4 |
| Table first_row_y | 0.7727 (squash) | 111.0 to 85.8 |
| Table row_height | 0.7727 (squash) | 6.0 to 4.6 |
| Table column X values | 1.2941 (stretch) | Scaled same as field X values |
| Widths, font sizes | unchanged | Not affected by coordinate system change |

### Sample Rescaled Values

| Field | Old X | New X | Old Y | New Y |
|-------|-------|-------|-------|-------|
| case_name | 33.0 | 42.7 | 29.2 | 22.6 |
| upi_number | 107.0 | 138.5 | 29.2 | 22.6 |
| signature_top | 30.0 | 38.8 | 72.0 | 55.6 |
| signature_bottom | 30.0 | 38.8 | 234.0 | 180.8 |
| table date col | 12.0 | 15.5 | - | - |
| table time_out col | 207.0 | 267.9 | - | - |

### Sample Output

Generated at `secret/calibration_sample.pdf` (100,615 bytes, 112 text fields, 8 circle marks, 5mm green grid, counter-clockwise rotation).

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

The `--grid 5` flag produces a 5mm grid. Use `--grid 10` for coarser resolution or omit `--grid` entirely for a grid-free output. The `--grid-color` flag accepts named colors (green, gray, red, blue, black, magenta, cyan).

---

## Observations

The rescaled offsets are a linear proportional adjustment. They will not be pixel-accurate because the original estimates were approximate to begin with. The grid overlay allows the human pilot to read correct form-space coordinates and fine-tune each field position.

Areas likely needing manual attention include the following.

- **Table row spacing.** The row_height scaled from 6.0mm to 4.6mm. If the physical row height on the form does not match this proportion, rows will drift progressively. The 5mm grid makes it straightforward to measure actual row spacing.
- **Circle-one options.** The pay frequency and insurance option positions were spaced at fixed intervals in the old system. After rescaling, verify that each option label aligns with its corresponding text on the form.
- **Right-edge columns.** The office_use_only column (x=282.1mm) slightly exceeds the form width (279.4mm). This column is not populated by the applicant and can be ignored or adjusted.

---

## Questions for Human Review

1. **Offset accuracy.** Please inspect `secret/calibration_sample.pdf` against the form template. Are the rescaled field positions approximately correct, or do they need wholesale repositioning?

2. **Table row height.** Does the rescaled row_height (4.6mm) match the actual row spacing on the form? If not, what value appears correct from the grid?

---

## Notes

- The file header comment in `form_offsets.toml` was updated to describe the landscape coordinate system.
- The meta section (page_width_mm, page_height_mm) is unchanged because those values describe the physical page, not the form. The code derives form dimensions via `rotation.form_dimensions()`.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

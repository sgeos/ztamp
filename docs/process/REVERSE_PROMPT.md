# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P5 Form Dimension Fix

---

## Verification

### Form Dimension Correction (rztamp/src/pdf.rs)

The false assumption was that the form had the same dimensions as the portrait PDF page (215.9mm x 279.4mm). The form is actually landscape (279.4mm wide x 215.9mm tall when read rightside-up), rotated 90 degrees CCW to fit in the portrait template image.

A `form_dimensions()` method was added to the `Rotation` enum. For 90-degree rotations, it returns swapped dimensions: form_w = page_h (279.4), form_h = page_w (215.9).

The `transform_position()` formulas now use the correct form dimensions. For CCW 90, the transform simplifies to page_x = fy, page_y = page_h - fx, with no non-uniform scaling factors.

The grid iteration ranges were also corrected. Form-X now iterates from 0 to form_w (279.4) and form-Y from 0 to form_h (215.9). Grid cells are square.

### Corrected Transform (CCW 90)

| Parameter | Old (incorrect) | New (correct) |
|-----------|----------------|---------------|
| form_w | 215.9 (= page_w) | 279.4 (= page_h) |
| form_h | 279.4 (= page_h) | 215.9 (= page_w) |
| page_x | fy * 0.773 | fy |
| page_y | page_h - fx * 1.294 | page_h - fx |
| Grid X range | 0 to 215.9 | 0 to 279.4 |
| Grid Y range | 0 to 279.4 | 0 to 215.9 |
| Grid cell shape | Rectangular (0.773 : 1.294) | Square (1 : 1) |

### Sample Output

Generated at `secret/calibration_sample.pdf` (100,584 bytes, 112 text fields, 8 circle marks, 5mm green grid, counter-clockwise rotation). Grid X labels run to x275, Y labels to y215. Grid cells are square.

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

## Impact on form_offsets.toml

The existing `form_offsets.toml` values were estimated for a portrait (215.9 x 279.4) coordinate system. With the corrected landscape interpretation, the form's coordinate ranges are different.

| Axis | Old range | New range |
|------|-----------|-----------|
| X (form width) | 0 to 215.9 | 0 to 279.4 |
| Y (form height) | 0 to 279.4 | 0 to 215.9 |

The existing offset values (x up to ~218, y up to ~234) were within the old portrait ranges. In the new landscape coordinate system, many values may fall outside the valid Y range (0 to 215.9) or may need significant adjustment.

Using the grid overlay, the human pilot can read the correct form-space coordinates for each field and update `form_offsets.toml` accordingly.

---

## Questions for Human Review

1. **Grid correctness.** Please inspect `secret/calibration_sample.pdf` to verify that grid cells are now square and that the X/Y ranges match the landscape form (X to ~279, Y to ~216). The grid labels should directly correspond to form_offsets.toml coordinate values.

2. **Offset re-estimation.** The existing form_offsets.toml values will likely need wholesale revision since the coordinate system has changed. Would it be helpful for the AI agent to clear the old offsets and provide a blank template, or should the human pilot manually update values using the grid?

---

## Notes

- The `form_dimensions()` method is `pub` so it can be used by downstream code if needed.
- The existing dead code warning for the `form` field in the `Secrets` struct persists. This is pre-existing from V0.2-M1-P1.

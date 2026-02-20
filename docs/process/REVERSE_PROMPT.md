# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P2 Rotation Support

---

## Verification

### Rotation Enum (rztamp/src/pdf.rs)

A `Rotation` enum was added with four variants.

- **Normal**: No rotation (rightside-up). Uses `Op::SetTextCursor` for text placement, preserving previous behavior.
- **Ccw90**: 90 degrees counter-clockwise. Uses `Op::SetTextMatrix` with `TextMatrix::TranslateRotate(x, y, 90.0)`.
- **Cw90**: 90 degrees clockwise. Uses `TextMatrix::TranslateRotate(x, y, 270.0)`.
- **UpsideDown**: 180 degrees. Uses `TextMatrix::TranslateRotate(x, y, 180.0)`.

The `generate_form_pdf()` function accepts an additional `rotation: Rotation` parameter. For 90-degree rotations (Ccw90 and Cw90), circle mark ellipse radii are swapped so the major axis follows the text direction.

### CLI Tool (tools/src/fill.rs)

The `tanf-fill` binary now accepts an optional `--rotation` flag.

```
tanf-fill --offsets <path> --secrets <path> --template <path> --output <path> [--rotation <mode>]
```

Valid rotation modes are `rightside-up` (default), `counter-clockwise`, `clockwise`, and `upside-down`. The tool logs the active rotation mode to stderr.

### Sample Output

Generated at `secret/calibration_sample.pdf` (90,481 bytes, 112 text fields, 8 circle marks, counter-clockwise rotation).

---

## Architecture Notes

### Text Rotation in printpdf 0.9.1

The `TextMatrix::TranslateRotate(Pt, Pt, f32)` variant provides combined translation and rotation in a single PDF `Tm` operator. The third argument is the rotation angle in degrees. The internal conversion is `(360.0 - angle).to_radians()`, producing counterclockwise-positive rotation in PDF user space.

For normal (0 degree) rendering, `Op::SetTextCursor` is used instead of `Op::SetTextMatrix`. This avoids unnecessary matrix operations for the common case and preserves backward-compatible behavior.

### Coordinate System (Unchanged)

Field positions in `form_offsets.toml` describe physical positions on the PDF page in millimeters from the top-left corner. The rotation flag controls text orientation at those positions rather than transforming position coordinates. This means field offsets must be calibrated for the actual physical page layout (with the form content in its rotated orientation).

---

## Questions for Human Review

1. **Calibration accuracy.** The rotated calibration sample has been generated. Please inspect `secret/calibration_sample.pdf` to verify that the text is oriented correctly (readable when the page is rotated 90 degrees clockwise). If the text orientation is correct but field positions need adjustment, the next step is the generate-inspect-adjust calibration loop on `form_offsets.toml`.

2. **Coordinate interpretation.** The current implementation treats `form_offsets.toml` coordinates as physical page positions (mm from top-left of the portrait page as rendered). The rotation flag only affects text angle, not position mapping. If the existing offsets were estimated for a rightside-up form layout, they will likely need significant revision to match the rotated form. Should the AI agent assist with re-estimating offsets for the rotated layout, or will the human pilot handle offset adjustments manually?

3. **Circle mark behavior.** For 90-degree rotations, the ellipse radii are swapped (radius_x becomes radius_y and vice versa) so the elongated axis follows the text direction. If this produces incorrect visual results in the calibration sample, the swap logic can be removed or refined.

---

## Notes

- The `--rotation` flag is entirely optional. Omitting it produces identical behavior to the previous version (rightside-up, no rotation).
- The existing dead code warning for the `form` field in the `Secrets` struct persists. This is pre-existing from V0.2-M1-P1.

# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P3 Coordinate Transformation

---

## Verification

### Coordinate Transformation (rztamp/src/pdf.rs)

A `transform_position()` method was added to the `Rotation` enum. It maps form-space coordinates (mm from top-left of the rightside-up form) to page-space coordinates (mm from top-left of the physical PDF page).

The transformation formulas for each mode are listed below. W is page width (215.9mm) and H is page height (279.4mm).

| Mode | page_x | page_y |
|------|--------|--------|
| Normal | fx | fy |
| CCW 90 | fy * W / H | H - fx * H / W |
| CW 90 | W - fy * W / H | fx * H / W |
| 180 | W - fx | H - fy |

For 90-degree rotations, the X and Y axes swap, with proportional scaling by the aspect ratio (W/H or H/W) to map form dimensions onto page dimensions.

This transformation is applied to both text field positions and circle mark center positions before conversion to PDF coordinates. Circle radii remain swapped (rx and ry exchanged) for 90-degree rotations as before.

### Corner Verification

The transformation was verified algebraically at all four corners.

CCW 90 example (W=215.9, H=279.4).

| Form corner | Form (fx, fy) | Page (px, py) | Expected |
|-------------|---------------|---------------|----------|
| Top-left | (0, 0) | (0, 279.4) | Bottom-left |
| Top-right | (215.9, 0) | (0, 0) | Top-left |
| Bottom-left | (0, 279.4) | (215.9, 279.4) | Bottom-right |
| Bottom-right | (215.9, 279.4) | (215.9, 0) | Top-right |

### Sample Output

Generated at `secret/calibration_sample.pdf` (90,491 bytes, 112 text fields, 8 circle marks, counter-clockwise rotation with coordinate transformation).

---

## Architecture Notes

### Two-Stage Coordinate Pipeline

Coordinate handling now has two stages.

1. **Form-space to page-space**: `Rotation::transform_position()` maps (fx, fy) to (px, py) using the rotation-specific formulas. For Normal mode, this is an identity transformation.
2. **Page-space to PDF-space**: `mm_to_pt()` and Y-axis inversion (`page_height_mm - py`) convert page-space mm to PDF points (origin at bottom-left).

This separation means `form_offsets.toml` coordinates always describe the rightside-up form, regardless of which rotation mode is used at render time. The same offsets file works for all four rotation modes.

### Previous Prompt Answers Incorporated

The following decisions from the human pilot are now reflected in the codebase.

1. Automated generate-inspect-adjust loop: Open to it if feasible.
2. Grid lines flag: Would be useful. Not yet implemented.
3. NIF compile time: Acceptable. Feature gates deferred.
4. Test strategy: Manual testing is sufficient for now.

---

## Questions for Human Review

1. **Calibration accuracy.** Please inspect `secret/calibration_sample.pdf` to verify that text positions now align with the form fields on the rotated template. The coordinate transformation should place text in approximately the correct locations. Fine-tuning of individual field offsets in `form_offsets.toml` may still be needed.

2. **Grid lines mode.** You indicated a grid lines flag would be useful. Should the next prompt add a `--grid` flag to the CLI tool that overlays numbered grid lines (at regular mm intervals) on the template instead of field text? This would help with offset calibration.

3. **Automated calibration.** You mentioned being open to an automated generate-inspect-adjust loop. Without computer vision capabilities, the AI agent cannot inspect the visual output of the PDF. The adjustment loop would need to be driven by human visual inspection. An alternative would be to generate multiple PDFs with systematic offset variations to narrow down correct positions. Would either approach be helpful?

---

## Notes

- The form_offsets.toml coordinates remain unchanged. They describe the rightside-up form and are now correctly transformed at render time.
- The existing dead code warning for the `form` field in the `Secrets` struct persists. This is pre-existing from V0.2-M1-P1.

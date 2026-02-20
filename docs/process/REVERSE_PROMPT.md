# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P4 Grid Overlay

---

## Verification

### Grid Overlay (rztamp/src/pdf.rs)

A `GridConfig` struct and `build_grid_ops()` function were added. The grid draws lines at regular intervals in form-space coordinates, transformed to page-space using the active rotation. Each line is labeled with its form-space value.

For CCW 90 rotation, form-X grid lines appear as horizontal lines on the page (labeled "x0", "x10", "x20", ...), and form-Y grid lines appear as vertical lines (labeled "y0", "y10", "y20", ...). These labels correspond directly to the X and Y values in `form_offsets.toml`.

The grid is drawn between the background image and the text overlay, as requested. Grid lines use 0.3pt thickness and labels use 5pt font.

### CLI Tool (tools/src/fill.rs)

Two new optional flags were added.

```
tanf-fill --offsets <path> --secrets <path> --template <path> --output <path>
         [--rotation <mode>] [--grid <interval_mm>] [--grid-color <color>]
```

The `--grid` flag takes an interval in millimeters (e.g., `--grid 10` for 10mm spacing). The `--grid-color` flag accepts named colors: green (default), gray, red, blue, black, magenta, cyan.

### Sample Output

Generated at `secret/calibration_sample.pdf` (95,579 bytes, 112 text fields, 8 circle marks, 10mm grid, green, counter-clockwise rotation).

---

## Automated Generate-Inspect-Adjust

The human pilot asked about computer vision solutions and a sane feedback loop for human adjustment.

### Computer Vision Options

Practical computer vision approaches for automated calibration are limited in this context. The core challenge is that the scanned form image is a 1-bit black-and-white bitmap with no structural metadata. Detecting field locations would require either optical character recognition (OCR) to locate label text and infer field positions, or template matching against known regions. Both approaches would require additional dependencies (such as `tesseract` for OCR or the `imageproc` crate for template matching) and substantial implementation effort. The accuracy of OCR on a 1-bit fax-encoded scan is uncertain.

Given the project's current scope, the practical cost of adding computer vision infrastructure is high relative to the manual calibration task, which involves approximately 40 field positions.

### Recommended Human Calibration Workflow

The most reliable feedback loop uses the grid overlay with the following process.

1. Generate a calibration PDF with `--grid 10` (or `--grid 5` for finer resolution).
2. Open the PDF and rotate it 90 degrees clockwise to read the form content rightside-up.
3. For each field that needs adjustment, read the nearest grid line values.
   - The "x" labels correspond to the X values in `form_offsets.toml`.
   - The "y" labels correspond to the Y values in `form_offsets.toml`.
4. Edit `form_offsets.toml` with the corrected values.
5. Delete the old output and regenerate.
6. Repeat until positions are accurate.

For bulk adjustments, if most fields have a consistent offset error (e.g., all X values are 5mm too large), the human can apply a uniform correction to all X values in `form_offsets.toml`.

### Alternative: Offset Sweep

An automated approach without computer vision would generate multiple PDFs with systematic offset variations (e.g., a global X shift of -10, -5, 0, +5, +10 mm applied to all coordinates). The human inspects the set and identifies which shift produces the best alignment. This could be implemented as a `--sweep` mode if useful.

---

## Questions for Human Review

1. **Grid readability.** Please inspect `secret/calibration_sample.pdf` to verify that the grid lines and labels are visible and useful. The labels show form-space coordinates ("x30" means form-space X=30mm). If the 10mm interval is too coarse or too fine, adjust with `--grid 5` or `--grid 20`.

2. **Offset calibration.** With the grid visible, you can now read the form-space coordinates of actual field positions. Adjust `form_offsets.toml` values accordingly. If the current offsets have a systematic error (consistently off in one direction), please note the approximate magnitude so the AI agent can apply a bulk correction.

3. **Sweep mode.** Would a `--sweep` mode that generates multiple PDFs with systematic offset shifts be helpful for finding the right global correction?

---

## Notes

- The `form_offsets.toml` coordinates remain unchanged from the initial visual estimates. They describe positions on the rightside-up form and are transformed at render time.
- The existing dead code warning for the `form` field in the `Secrets` struct persists. This is pre-existing from V0.2-M1-P1.

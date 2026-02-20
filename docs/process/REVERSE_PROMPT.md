# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P9 Black Text PoC

---

## Verification

### --text-color Flag (tools/src/fill.rs)

A `--text-color <color>` flag was added to the tanf-fill CLI tool. When specified, all text fields and circle marks use the given color instead of the default calibration colors (red for headers, blue/magenta alternating for table rows).

| Flag | Behavior |
|------|----------|
| Omitted | Calibration colors: red headers, blue/magenta table rows |
| `--text-color black` | All text and circles in black |
| `--text-color <any>` | All text and circles in the specified color |

The flag accepts the same named colors as `--grid-color`: green, gray, red, blue, black, magenta, cyan.

### Sample Output

Generated at `secret/calibration_sample.pdf` (90,565 bytes, 112 text fields, 8 circle marks, no grid, black text, counter-clockwise rotation).

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

1. **Black text PoC.** Please inspect `secret/calibration_sample.pdf`. Does the black text on the form template look acceptable for production use? Are any position or alignment adjustments needed?

---

## Notes

- The `--text-color` and `--grid` flags are independent. They can be combined (for example, black text with a grid overlay) or used separately.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

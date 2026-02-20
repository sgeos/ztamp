# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P8 Alignment Field

---

## Verification

### Alignment Enum (rztamp/src/offsets.rs)

An `Alignment` enum was added with three variants: `Left`, `Center`, `Right`. It derives `Default` (defaulting to `Left`) and uses `#[serde(rename_all = "lowercase")]` for TOML deserialization. The `alignment` field was added to both `FieldOffset` and `ColumnOffset` structs with `#[serde(default)]`.

### PDF Rendering (rztamp/src/pdf.rs)

The previous `center_in_width()` function was replaced by `align_in_width()` which accepts an `Alignment` parameter.

| Alignment | Behavior |
|-----------|----------|
| Left | Text starts at field x position (no offset) |
| Center | Text offset by (width - text_width) / 2 |
| Right | Text offset by (width - text_width) |

When text exceeds the field width or no width is specified, all alignments fall back to left-aligned positioning from the x coordinate.

### form_offsets.toml

All 28 named field entries and all 9 table column entries now include `alignment = "center"`. No coordinate values were changed. The human pilot can manually change individual fields to `"left"` or `"right"` as needed.

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

## Questions for Human Review

1. **Alignment defaults.** All fields are currently set to "center". After inspecting the calibration sample, which fields should be changed to "left" or "right" alignment?

---

## Notes

- The `Alignment` enum is `pub` and available to downstream code via `rztamp::offsets::Alignment`.
- Circle-one options have `alignment = "center"` but no `width`, so alignment has no effect on them. Their text and ellipse positioning is unchanged.
- The text width estimation remains approximate (Helvetica average character width at 0.5em). Alignment accuracy depends on this estimate.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

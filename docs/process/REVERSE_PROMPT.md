# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P11 Labels Flag and PoC Update

---

## Verification

### --labels Flag (tools/src/fill.rs)

A `--labels` boolean flag was added to the tanf-fill CLI. It controls whether text labels are rendered inside circle-one option ellipses.

| Flag | Circle Labels | Ellipses |
|------|--------------|----------|
| Omitted (default) | Hidden | Drawn |
| `--labels` | Shown | Drawn |

The flag has no effect on non-circle fields. All other text fields are always rendered.

### Signature Date Placeholders

The placeholder text for signature date fields was changed from "DATE" to "01/01/2001".

### Sample Output

Generated at `secret/calibration_sample.pdf` (89,763 bytes, 104 text fields, 8 circle marks, no grid, black text, no labels, counter-clockwise rotation).

The text field count dropped from 112 to 104 because the 8 circle-one option labels (4 pay frequency options + 4 insurance options) are now suppressed without `--labels`.

---

## Commands

### Production-style PoC (no grid, black text, no labels)

```
rm -f secret/calibration_sample.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/calibration_sample.pdf \
  --rotation counter-clockwise \
  --text-color black
```

### Calibration Mode (grid, colored text, with labels)

```
rm -f secret/calibration_sample.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/calibration_sample.pdf \
  --rotation counter-clockwise \
  --grid 5 \
  --labels
```

To rebuild after code changes, first run `cargo build --manifest-path tools/Cargo.toml`.

---

## Questions for Human Review

1. **PoC quality.** Does the output with black text, no grid, no circle labels, and "01/01/2001" signature dates look acceptable? Are further adjustments needed before moving toward production use?

---

## Notes

- The `--labels` flag is independent of `--text-color` and `--grid`. All three can be combined in any way.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

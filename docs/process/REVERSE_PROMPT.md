# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P12 Employment Date Fix

---

## Verification

### Debug Data Changes (tools/src/fill.rs)

| Field | Old Value | New Value |
|-------|-----------|-----------|
| employed_start_date | "04/15/2026" | "04/15/2001" |
| employed_first_check_year | "2026" | "2001" |

All other debug values are unchanged. The first check month ("05") and day ("01") remain as before, giving a first check date of 05/01/2001.

### Sample Output

Generated at `secret/calibration_sample.pdf` (89,524 bytes, 104 text fields, 8 circle marks, no grid, black text, no labels, counter-clockwise rotation).

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

1. **PoC quality.** Does the output with the 2001 employment dates look correct? Is this ready to move toward production integration, or are further calibration adjustments needed?

---

## Notes

- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

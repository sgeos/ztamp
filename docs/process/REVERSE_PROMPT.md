# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P13 Watermark and Three PoCs

---

## Verification

### --watermark Flag

A `--watermark <color>` flag was added to tanf-fill. When specified, "TEST SAMPLE" is rendered at 72pt font diagonally (45 degrees) across the center of the page, on top of all other content.

### --circle-all Flag

A `--circle-all` flag was added to tanf-fill. Without it, only one option per circle-one series is circled (Weekly for pay frequency, None for insurance). With it, all options in each series are circled.

### Three PoC Files

| File | Size | Grid | Text Color | Labels | Circles | Watermark |
|------|------|------|-----------|--------|---------|-----------|
| `secret/poc_production.pdf` | 87,965 | No | Black | No | 2 (one per series) | Red |
| `secret/poc_testing.pdf` | 89,632 | No | Black | No | 8 (all) | Red |
| `secret/poc_debug.pdf` | 100,544 | 5mm green | Calibration | Yes | 8 (all) | Red |

---

## Commands

### Almost Production PoC

```
rm -f secret/poc_production.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/poc_production.pdf \
  --rotation counter-clockwise \
  --text-color black \
  --watermark red
```

### Black Text Testing PoC

```
rm -f secret/poc_testing.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/poc_testing.pdf \
  --rotation counter-clockwise \
  --text-color black \
  --circle-all \
  --watermark red
```

### Debug PoC

```
rm -f secret/poc_debug.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/poc_debug.pdf \
  --rotation counter-clockwise \
  --grid 5 \
  --labels \
  --circle-all \
  --watermark red
```

To rebuild after code changes, first run `cargo build --manifest-path tools/Cargo.toml`.

---

## Questions for Human Review

1. **Watermark appearance.** Does the diagonal "TEST SAMPLE" watermark look acceptable? If the size, position, or angle needs adjustment, these are configurable in the `WatermarkConfig` struct.

2. **Default selected circles.** The "Almost Production" PoC circles "Weekly" and "None" as the default selections. Are these reasonable defaults, or should different options be selected?

3. **Concatenation.** The three PoC files are ready for concatenation. Are any adjustments needed before proceeding?

---

## Notes

- All three flags (`--watermark`, `--circle-all`, `--labels`) are independent and can be combined freely.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

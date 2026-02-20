# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P14 Debug Fill Flag

---

## Verification

### --debug-fill Flag

A `--debug-fill` boolean flag was added to tanf-fill. When specified, `build_value_map` includes all hardcoded test data (signature placeholders, employment section fields, pay frequency options, insurance options). Without it, only secrets-sourced fields are populated (case_name, upi_number, job_search_from/to/hours, submission_deadline_time/date/location).

### Three PoC Files (Regenerated)

| File | Size | Grid | Text Color | Labels | Circles | Watermark | Debug Fill |
|------|------|------|-----------|--------|---------|-----------|-----------:|
| `secret/poc_production.pdf` | 87,965 | No | Black | No | 2 (one per series) | Red | Yes |
| `secret/poc_testing.pdf` | 89,632 | No | Black | No | 8 (all) | Red | Yes |
| `secret/poc_debug.pdf` | 100,544 | 5mm green | Calibration | Yes | 8 (all) | Red | Yes |

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
  --watermark red \
  --debug-fill
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
  --watermark red \
  --debug-fill
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
  --watermark red \
  --debug-fill
```

To rebuild after code changes, first run `cargo build --manifest-path tools/Cargo.toml`.

---

## Questions for Human Review

1. **Data split correctness.** The split places participant, job search, and submission fields in the "always included" category (sourced from secrets.toml). Signature placeholders, employment section, pay frequency options, and insurance options are gated behind `--debug-fill`. Is this the correct division, or should any fields move between categories?

2. **Production data mechanism.** Without `--debug-fill`, fields like signatures and employment data are simply absent from the value map and not rendered. The production mechanism for supplying this data is not yet defined. Should these fields eventually come from secrets.toml, a separate data file, or another source?

3. **Concatenation readiness.** The three PoC files are ready for concatenation. Are any adjustments needed before proceeding?

---

## Notes

- All CLI flags (`--watermark`, `--circle-all`, `--labels`, `--debug-fill`) are independent and can be combined freely.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).
- Output sizes are identical to P13, confirming `--debug-fill` produces equivalent output when enabled.

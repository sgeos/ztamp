# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P15 Four PoC Variants

---

## Verification

### build_calibration_fields Fixes

Two behavioral changes were made to `build_calibration_fields` in `rztamp/src/pdf.rs`:

1. **Skip missing values.** Named fields with no matching entry in the `values` map are now skipped entirely. Previously, the field name was rendered in uppercase as a fallback. This makes `--debug-fill` meaningful: without it, only secrets-sourced fields appear on the PDF.

2. **Gate table generation.** A `fill_table: bool` parameter was added. When false, the job search table rows (hardcoded sample data) are not generated. The caller passes `args.debug_fill` for this parameter.

### Four PoC Files

| File | Size | Fields | Circles | Grid | Text Color | Labels | Watermark | Debug Fill |
|------|------|--------|---------|------|-----------|--------|-----------|------------|
| `secret/poc_production.pdf` | 77,578 | 8 | 0 | No | Black | No | No | No |
| `secret/poc_almost_production.pdf` | 87,965 | 104 | 2 | No | Black | No | Red | Yes |
| `secret/poc_testing.pdf` | 89,632 | 104 | 8 | No | Black | No | Red | Yes |
| `secret/poc_debug.pdf` | 100,544 | 112 | 8 | 5mm green | Calibration | Yes | Red | Yes |

---

## Commands

### Production PoC

```
rm -f secret/poc_production.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/poc_production.pdf \
  --rotation counter-clockwise \
  --text-color black
```

### Almost Production PoC

```
rm -f secret/poc_almost_production.pdf && tools/target/debug/tanf-fill \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/poc_almost_production.pdf \
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

1. **Production PoC content.** The production PoC renders only 8 fields sourced from secrets.toml (case_name, upi_number, job_search_from/to/hours, submission_deadline_time/date/location) with no table data, no circles, and no watermark. Does this match the expected production output?

2. **Concatenation readiness.** The four PoC files are ready for concatenation. Are any adjustments needed before proceeding?

---

## Notes

- All CLI flags (`--watermark`, `--circle-all`, `--labels`, `--debug-fill`) are independent and can be combined freely.
- The `--debug-fill` flag now controls both the value map content (debug test data) and table generation (hardcoded sample rows).
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

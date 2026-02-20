# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.3-M1-P1 PDF Concatenation Tool

---

## Verification

### V0.2 Merged into Main

The `feat/v02-pdf-write-poc` branch was fast-forwarded into `main`. All V0.2 commits (P1 through P15) are now on main.

### tanf-concat Tool

A new binary `tanf-concat` was added to `tools/`. It uses the `lopdf` crate (0.39) to merge multiple PDF files into one.

**Usage**: `tanf-concat --output <path> <input1.pdf> <input2.pdf> ...`

The tool follows the canonical lopdf merge pattern: renumber object IDs across source documents to avoid collisions, collect pages and objects, then stitch together the Catalog and Pages tree. It refuses to overwrite existing output files and requires at least two input files.

### Concatenated Output

| File | Size | Pages | Composition |
|------|------|-------|-------------|
| `secret/poc_combined.pdf` | 354,455 | 4 | production + almost production + testing + debug |

---

## Commands

### Concatenate Four PoCs

```
rm -f secret/poc_combined.pdf && tools/target/debug/tanf-concat \
  --output secret/poc_combined.pdf \
  secret/poc_production.pdf \
  secret/poc_almost_production.pdf \
  secret/poc_testing.pdf \
  secret/poc_debug.pdf
```

### Regenerate Individual PoCs

See V0.2-M1-P15 commands (unchanged). To rebuild after code changes, first run `cargo build --manifest-path tools/Cargo.toml`.

---

## Questions for Human Review

1. **Concatenated output.** Does the four-page combined PDF (`poc_combined.pdf`) render correctly? Each page should display its respective PoC variant.

2. **Page order.** The pages are ordered: production, almost production, testing, debug. Is this the desired order?

3. **Branch workflow.** The V0.3 work is on `feat/v03-pdf-concatenation`. Should it be merged to main now, or is further work planned on this branch?

---

## Notes

- The `lopdf` dependency was added to `tools/Cargo.toml`. It brings in `rayon` (parallelized parsing), `chrono`, and other transitive dependencies.
- Bookmark/outline objects from source PDFs are discarded during merge (standard lopdf behavior). This is not an issue for the current PoC files which have no bookmarks.
- The pre-existing dead code warning for the `form` field in the `Secrets` struct persists (from V0.2-M1-P1).

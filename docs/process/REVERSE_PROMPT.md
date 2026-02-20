# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-20
**Task**: V0.5-M1-P4 Bug Fixes and PDF Export

---

## Verification

### Bug Fixes

| Fix | Detail |
|-----|--------|
| How Contact Made default | Changed from "Online" to "Internet" in `default_attrs/0` and the template select options. |
| Date filtering crash | `handle_event("filter_entries", ...)` now uses `Map.get(params, key, "")` instead of pattern matching, which failed when disabled inputs did not submit values. |
| Employer display | Added `format_employer/1` function that replicates the migration down-function formatting (recruiter, remote, combined) for the Captured Entries table. |

### PDF Export

The Export PDF button is now functional. Clicking it generates a complete proof-of-job-search PDF packet in `secret/`.

#### Packet Structure

1. **Cover page** with name, UPI, period, application count, recruiter count, total time, and submission deadline.
2. **TANF form pages** with the template.tiff background, header fields from secrets.toml, and table rows from entries (10 per page, oldest to newest).
3. **Screenshot pages** with header text (name, UPI, date, employer, contact method, time) and the full screenshot scaled to fit.

All pages are concatenated into a single PDF using lopdf.

#### Architecture

| Component | Path | Description |
|-----------|------|-------------|
| `build_table_fields` | `rztamp/src/pdf.rs` | Accepts custom table row data and positions text using form_offsets.toml |
| `generate_text_page` | `rztamp/src/pdf.rs` | Creates text-only PDF pages (cover page) |
| `generate_image_page` | `rztamp/src/pdf.rs` | Creates PDF pages with scaled PNG images and header text |
| `tanf-export` | `tools/src/export.rs` | CLI tool that reads a JSON manifest and produces the complete packet |
| `Ztamp.PdfExport` | `ztamp/lib/ztamp/pdf_export.ex` | Elixir orchestration module that formats entries and calls the CLI |
| LiveView handler | `job_search_live.ex` | `export_pdf` event triggers export of currently listed entries |

#### Integration Test

A test with 2 entries produced a 4-page PDF (cover + 1 TANF form + 2 screenshots) at 228 KB.

---

## Manual Verification

1. Start the server: `cd ztamp && mix phx.server`
2. Navigate to `http://localhost:4000/job-search`
3. Verify "How Contact Made" defaults to "Internet" in the form.
4. Verify date filtering works (change From date, toggle "To present").
5. Verify the Employer column in Captured Entries shows formatted employer info with recruiter and remote indicators.
6. Click "Export PDF" and verify:
   - A flash message reports the output file name.
   - The PDF appears in `secret/` with the expected structure.
   - Open the PDF and verify the cover page, TANF form pages, and screenshot pages.

---

## Questions for Human Review

1. **Cover page layout.** The cover page is plain text on a white background. Should it have any additional formatting, logos, or layout changes?

2. **Form field values.** The `job_search_hours` field on the TANF form uses the value from `secrets.toml` (required hours). The `job_search_from` and `job_search_to` fields use dates from the oldest and newest entries in the export set. Are these correct, or should dates from a different source be used?

3. **Screenshot page scaling.** Screenshots are scaled to fit within 15mm margins on each side with 42mm from the top for header text. If screenshots are very wide, they may appear small on the page. Should the scaling strategy be adjusted?

4. **Export scope.** The export uses the currently filtered entries (respecting date filter settings). Should there be a separate date range selector for export, or is using the current filter correct?

---

## Commands

### Start the Phoenix Server

```
cd ztamp && mix phx.server
```

### Build the Export Tool

```
cd tools && cargo build --release
```

### Run Export Manually

```
tools/target/release/tanf-export \
  --manifest /tmp/manifest.json \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/export.pdf
```

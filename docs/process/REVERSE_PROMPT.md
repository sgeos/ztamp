# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-20
**Task**: V0.5-M1-P5 Dashboard, Export, and Landing Page Improvements

---

## Verification

### Dashboard Changes

| Change | Detail |
|--------|--------|
| Internet Confirmation # removed | Input field removed from both the main form and the edit modal. Internet confirmation is auto-set to "Yes" when a screenshot is captured. The entries table shows "Yes" or "No" based on whether a screenshot_path exists. |
| "To present" checkbox repositioned | Moved from below the To Date input to beside the label, above the input. |
| Watermark checkbox added | Checkbox labeled "Watermark" next to the Export PDF button. When enabled, all exported pages get a "TEST SAMPLE" watermark. |

### Landing Page

| Change | Detail |
|--------|--------|
| Home page replaced | Default Phoenix landing page replaced with TANF-specific card layout linking to Job Search, LiveView Dashboard, and Mailbox. |
| App navbar updated | Phoenix framework links replaced with Home, Job Search, and Dashboard navigation. Branding changed from Phoenix version to "TANF Job Search". |
| Container widened | Main content container changed from max-w-2xl (672px) to max-w-4xl (896px) to accommodate the entries table and landing page cards. |
| Theme integration | Theme toggle remains in the navbar. Theme persistence via localStorage applies across all pages (no changes needed, already functional). |

### PDF Export Enhancements

| Change | Detail |
|--------|--------|
| Signatures | Top and bottom of each TANF form page now include "SIGNATURE HERE" and the export date (MM/DD/YYYY format). Uses existing `participant_signature_top`, `participant_signature_top_date`, `participant_signature_bottom`, `participant_signature_bottom_date` fields from form_offsets.toml. |
| Internet confirmation | PDF table rows show "Yes" if a screenshot is attached, "No" otherwise. Logic in both Elixir (entry_to_manifest) and Rust (entry_to_table_row). |
| Screenshot rotation | Landscape screenshots (width > height) are rotated 90 degrees CCW before placement. Uses the `image` crate for detection and rotation. |
| Watermark | `--watermark` flag added to tanf-export CLI. When enabled, all pages (cover, form, screenshot) get "TEST SAMPLE" in washed-out red at 72pt, diagonal across the center. Watermark rendering extracted into a shared `build_watermark_ops` helper used by all three page generators. |
| Manifest location | Temporary manifest JSON file now written to `secret/` instead of `/tmp`. Cleaned up after export completes. |

### Architecture

| Component | Path | Change |
|-----------|------|--------|
| `build_watermark_ops` | `rztamp/src/pdf.rs` | New shared helper. Extracts watermark PDF ops from generate_form_pdf for reuse. |
| `generate_text_page` | `rztamp/src/pdf.rs` | Added `watermark: Option<&WatermarkConfig>` parameter. |
| `generate_image_page` | `rztamp/src/pdf.rs` | Added `watermark: Option<&WatermarkConfig>` parameter. |
| `rotate_if_landscape` | `tools/src/export.rs` | New function. Detects landscape images and rotates CCW 90. |
| `tanf-export` CLI | `tools/src/export.rs` | Added `--watermark` flag, `export_date` in manifest, signature fields, internet_confirmation auto-set, screenshot rotation. |
| `Ztamp.PdfExport` | `ztamp/lib/ztamp/pdf_export.ex` | Accepts `watermark:` option. Manifest uses `secret/` dir. Adds `export_date`. Sets `internet_confirmation` from screenshot_path. |
| LiveView | `job_search_live.ex` | Added `:watermark` assign and `toggle_watermark` handler. Auto-sets `internet_confirmation` to "Yes" on entry creation. Passes `watermark:` to PdfExport. |
| Template | `job_search_live.html.heex` | Removed internet_confirmation inputs. Repositioned "To present" checkbox. Added watermark checkbox. Updated Internet column. |
| Home page | `page_html/home.html.heex` | Replaced with TANF card layout. |
| App layout | `layouts.ex` | Updated navbar links and branding. Widened container. |

---

## Manual Verification

1. Start the server: `cd ztamp && mix phx.server`
2. Navigate to `http://localhost:4000/`
3. Verify the landing page shows three cards (Job Search, LiveView Dashboard, Mailbox).
4. Verify the navbar shows Home, Job Search, Dashboard, and theme toggle.
5. Click "Job Search" and verify:
   - No "Internet Confirmation #" input in the form.
   - The "To present" checkbox is beside the "To Date" label, not below it.
   - The entries table has an "Internet" column showing "Yes" for entries with screenshots.
   - A "Watermark" checkbox appears next to the Export PDF button.
6. Test theme toggle on landing page, then navigate to Job Search and verify theme persists.
7. Export a PDF and verify:
   - Signatures "SIGNATURE HERE" and today's date appear at top and bottom of each TANF form page.
   - Internet confirmation column shows "Yes" for entries with screenshots.
   - Landscape screenshots are rotated to portrait orientation.
8. Enable the Watermark checkbox, export again, and verify "TEST SAMPLE" appears diagonally across all pages.

---

## Questions for Human Review

1. **Watermark opacity.** True 50% opacity requires PDF graphics state manipulation not supported by printpdf's Op enum. The current implementation uses a washed-out red color (RGB 1.0, 0.5, 0.5) to approximate the visual effect. Is this acceptable, or should a different approach be investigated?

2. **Container width.** The app layout container was widened from max-w-2xl to max-w-4xl. This gives more room for the entries table but changes the layout of all pages. Is the new width acceptable?

3. **Dev-only links.** The landing page includes links to /dev/dashboard and /dev/mailbox, which only exist in development mode. These will 404 in production. Since this tool is development-only, this should be acceptable.

---

## Commands

### Start the Phoenix Server

```
cd ztamp && mix phx.server
```

### Rebuild the Export Tool

```
cd tools && cargo build --release
```

### Run Export Manually

```
tools/target/release/tanf-export \
  --manifest secret/manifest.json \
  --offsets assets/form/form_offsets.toml \
  --secrets secret/secrets.toml \
  --template assets/form/template.tiff \
  --output secret/export.pdf \
  --watermark
```

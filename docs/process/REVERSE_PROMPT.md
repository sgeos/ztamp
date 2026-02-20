# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.2-M1-P1 PDF Write PoC

---

## Verification

### V0.1 Branch Merged

The `feat/field-extraction-and-offsets` branch (2 commits) was merged into `main` via fast-forward. The branch was deleted after merge.

### rztamp PDF Generation (rztamp/src/pdf.rs)

Two public functions implemented.

**`generate_form_pdf()`** creates a new PDF document with the following steps.
1. Decodes the template image (TIFF) via `printpdf::RawImage::decode_from_bytes()`.
2. Places the image as a full-page background using `XObjectTransform` with DPI-based scaling.
3. Overlays text fields at specified positions using builtin Helvetica font (`PdfFontHandle::Builtin`).
4. Draws circle marks as 4-segment bezier ellipse approximations.
5. Converts coordinates from mm-from-top-left to PDF points-from-bottom-left.

**`build_calibration_fields()`** takes form offsets, field values, and three colors. It produces text fields and circle marks with the specified color scheme: one color for header/non-table fields, two alternating colors for table rows, and circles around all circle-one options.

### Form Offset Types (rztamp/src/offsets.rs)

Serde-deserializable types for `form_offsets.toml`: `FormOffsets`, `Meta`, `FieldOffset`, `TableConfig`, `ColumnOffset`. The `load_offsets()` function reads and parses the TOML file.

### CLI Tool (tools/src/fill.rs)

The `tanf-fill` binary accepts four required arguments.

```
tanf-fill --offsets <path> --secrets <path> --template <path> --output <path>
```

Behavior in this PoC version.
- Reads form offsets TOML and secrets TOML.
- Maps secret values to field names for header fields.
- Generates placeholder text for table rows and employment fields.
- Produces a calibration PDF with red (header), blue (even rows), and magenta (odd rows) text.
- Circles all circle-one options (pay frequency and insurance).
- Refuses to overwrite an existing output file.

### Sample Output

Generated at `secret/calibration_sample.pdf` (85,968 bytes, 112 text fields, 8 circle marks).

Observations from the calibration output.
- Template image renders correctly as full-page background.
- All three text colors are visible and distinguishable.
- Text is selectable (native PDF text, not pixel data).
- Circles render around pay frequency and insurance options.
- Field positions are approximate. Several areas need adjustment, particularly in the table columns and the "should you become employed" section.

---

## Architecture Notes

### printpdf 0.9.1 API

The 0.9.x API differs significantly from earlier versions. Key patterns.

- Pages are built from `Vec<Op>` operations, not from mutable layer references.
- Builtin fonts are used via `PdfFontHandle::Builtin(BuiltinFont::Helvetica)` in `Op::SetFont`. No need to call `doc.add_font()` for builtin fonts.
- Images are decoded with `RawImage::decode_from_bytes(bytes, &mut warnings)` and added via `doc.add_image()`. The `tiff` feature flag must be enabled.
- The `save()` method requires a `&mut Vec<PdfWarnMsg>` warnings parameter.
- `XObjectTransform::dpi` controls image scaling. Setting `dpi: Some(200.0)` for a 200 DPI image at US Letter size fills the page correctly.

### Dependency Chain

```
tools/tanf-fill -> rztamp -> printpdf (0.9.1, tiff feature)
                           -> image (0.25, tiff + png features)
                           -> serde (1, derive feature)
                           -> toml (0.8)
```

---

## Questions for Human Review

1. **Calibration next steps.** The calibration sample is generated. Should the next prompt focus on adjusting field positions in `form_offsets.toml` through the generate-inspect-adjust loop? This would involve iteratively running `tanf-fill`, inspecting the output, and editing offsets.

2. **Table row positioning.** The table rows appear to overlap with the form's printed column headers and example row. The `first_row_y` and `row_height` values in `form_offsets.toml` likely need significant adjustment. Would it help to generate a version with numbered position markers (grid lines) rather than field text?

3. **NIF compilation.** The `rztamp` Cargo.toml now has `printpdf` and `image` as dependencies. This will increase the NIF compile time. The NIF crate at `ztamp/native/nif/` depends on `rztamp` and will pull in these dependencies. Should the NIF dependency be made feature-gated to keep NIF builds lean, or is the additional compile time acceptable?

4. **Test strategy.** Unit tests for PDF generation were deferred. What level of testing is appropriate? Options include byte-level PDF output comparison (fragile), visual regression testing (complex), or functional tests that verify the generated PDF contains expected text strings.

---

## Notes

- The `.gitignore` was updated to exclude `target/` directories (Rust build artifacts) globally.
- The `tools/` directory uses a workspace-independent Cargo.toml. It is not part of a Cargo workspace with `rztamp`. Each crate builds independently.
- The `form` field in the `Secrets` struct is read but not used by the CLI tool (it references the original PDF path, which is not needed for generation). This produces a compiler warning.

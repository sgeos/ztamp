# Phase 2: PDF Write PoC (V0.2)

> **Navigation**: [Roadmap](./README.md) | [Documentation Root](../README.md)

## Purpose

Build PDF generation routines in the `rztamp` Rust library that create new PDF documents with the cleaned form image as a background layer and field data overlaid as native PDF text. These routines can be called from a CLI tool or directly from the Elixir application via NIF.

## Deliverables

1. `printpdf` added as a dependency to `rztamp`.
2. Rust routine that creates a new PDF with the form template image as background.
3. Rust routine that overlays text at specified coordinates on the form.
4. Field offset calibration against the template image using a generate-inspect-adjust loop with colored text.
5. Calibrated form offsets in `assets/form/form_offsets.toml`.
6. Tests covering text placement and form field mapping.

## Design

The government PDF is a flat scanned image (CCITT Group 4 fax-encoded bitmap, 1704x2200 pixels, 1-bit depth). It contains no AcroForm or XFA form fields.

The strategy is to generate new PDFs from scratch rather than modifying the original.

1. Load the cleaned form template image (`assets/form/template.tiff`).
2. Create a new PDF with `printpdf`.
3. Place the template image as the page background.
4. Overlay field data as native PDF text at coordinates defined in `assets/form/form_offsets.toml`.
5. The result is a clean PDF with selectable text.

### Coordinate System

Field offsets use millimeters from the top-left corner of the page (US Letter, 215.9mm x 279.4mm). This maps naturally to `printpdf`'s coordinate system and to visual inspection of the template image.

### Calibration

Initial field offsets are visual estimates. Calibration uses a generate-inspect-adjust loop.

1. Generate a test PDF with colored text at all field positions.
2. Visually inspect the output against the template.
3. Adjust coordinates in `form_offsets.toml`.
4. Repeat until field positions are accurate.

### PDF Routine Location

PDF generation routines live in the `rztamp` library. This allows them to be used from a CLI tool, from NIF bindings in the Elixir application, or from any other Rust consumer.

## Key Decision

**R1 refined**: Use `printpdf` for new PDF generation with template image background. Reserve `lopdf` for future PDF manipulation needs (V0.3 concatenation). See [RESOLVED.md](../decisions/RESOLVED.md).

## Dependencies

- V0.1 (Project Structure) must be complete.

## Status

**Not started.**

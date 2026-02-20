# Resolved Decisions

> **Navigation**: [Decisions](./README.md) | [Documentation Root](../README.md)

This document records design decisions that have been fully resolved. Each entry includes the decision, rationale, and any residual sub-questions that remain open but do not block implementation.

## Document Status

**Created**: 2026-02-19
**Purpose**: Archive of resolved architectural decisions
**Related**: [Priority](./PRIORITY.md), [Backlog](./BACKLOG.md)

---

## R1. PDF Form Population Strategy

**Resolved**: 2026-02-19
**Previously**: D1 in PRIORITY.md

### Context

The system must populate a government-issued PDF form with job search activity data. Analysis of the actual PDF (BaseForm.pdf) revealed it is a **flat scanned image** (CCITT Group 4 fax-encoded bitmap, 1704x2200 pixels, 1-bit depth, PDF 1.6). It contains no AcroForm fields, no XFA structure, and no interactive form elements. The PDF was produced by a RICOH IM 5000 scanner.

This finding invalidates the original assumption that the form uses AcroForm fields. All form-filling approaches based on AcroForm or XFA field manipulation (pdftk, pdfium, pdf_form crate) are inapplicable.

### Decision

| Aspect | Decision |
|--------|----------|
| PDF generation library | `printpdf` crate |
| PDF manipulation library | `lopdf` crate (reserved for V0.3 concatenation) |
| Approach | Generate new PDFs with cleaned form image as background, overlay text at defined coordinates |
| Routine location | `rztamp` Rust library (usable from CLI, NIF, or other Rust consumers) |
| Integration | Via NIF or `System.cmd/3` from Elixir |
| Configuration | `assets/form/form_offsets.toml` mapping field names to mm coordinates |
| Coordinate system | Millimeters from top-left corner of US Letter page |
| Calibration | Generate-inspect-adjust loop with colored text |

### Workflow

1. Load cleaned form template image (`assets/form/template.tiff`).
2. Create a new PDF with `printpdf`.
3. Place the template image as the page background.
4. Overlay field data as native PDF text at coordinates from `form_offsets.toml`.
5. Output a clean PDF with selectable text.

### Rationale

- `printpdf` creates new PDF documents from scratch with a high-level API for text, images, and vector graphics. This produces clean, searchable PDFs where text is native rather than pixel data.
- The human pilot confirmed that PDF manipulation routines should live in `rztamp` for portability across CLI tools and the Elixir application.
- `lopdf` is reserved for future low-level PDF manipulation (V0.3 concatenation of filled forms with screenshot PDFs).
- The field position configuration file decouples form layout from application logic, supporting future form revisions.

### Residual Sub-Questions

- Font selection and embedding for text overlay.
- Image format and resolution requirements for placed images.
- Whether the template TIFF image requires format conversion for `printpdf` compatibility.

---

## R2. Screenshot Capture Strategy

**Resolved**: 2026-02-19
**Previously**: D2 in PRIORITY.md

### Decision

| Aspect | Decision |
|--------|----------|
| Library | Wallaby (`wallaby` hex, v0.30.12) |
| Rationale | Standardize on a single browser automation library (shared with D3/R3) |
| Chrome dependency | ChromeDriver required |

### Rationale

- Wallaby is the most mature Elixir browser automation library.
- Standardizing on Wallaby for both screenshot capture and browser automation (R3) reduces the number of browser-related dependencies.
- ChromicPDF would be a simpler choice in isolation, but using two browser automation libraries introduces unnecessary complexity.
- Wallaby provides `take_screenshot/2` alongside full browser interaction capabilities.

### Residual Sub-Questions

- ChromeDriver version management in the deployment environment.
- Screenshot viewport dimensions and image format for compliance documentation.
- Whether screenshots are captured during human-supervised sessions or fully automated.

---

## R3. Browser Automation Strategy

**Resolved**: 2026-02-19
**Previously**: D3 in PRIORITY.md

### Decision

| Aspect | Decision |
|--------|----------|
| Library | Wallaby (`wallaby` hex, v0.30.12) |
| Use case | Human-in-the-loop job application submission |
| Chrome dependency | ChromeDriver required |

### Rationale

- The human pilot identified Wallaby as the appropriate choice for human-in-the-loop applications.
- Despite being designed as a testing framework, Wallaby provides the full WebDriver API needed for form interaction.
- Human-in-the-loop support addresses anti-bot detection measures (CAPTCHAs) without requiring automated bypass.

### Residual Sub-Questions

- Specific job boards and employer sites to support.
- Wallaby session management for long-running interactive workflows.
- ChromeDriver lifecycle management in a production Phoenix application.

---

## Revision History

| Date | Author | Changes |
|------|--------|---------|
| 2026-02-19 | Claude | Initial creation. |
| 2026-02-19 | Claude | R1-R3 resolved. D1 revised based on PDF analysis (flat scan, not AcroForm). D2 and D3 standardized on Wallaby per human direction. |
| 2026-02-19 | Claude | R1 refined: `printpdf` for PDF generation, `lopdf` reserved for V0.3 concatenation. Form offsets in `assets/form/form_offsets.toml`. Calibration via generate-inspect-adjust loop. |

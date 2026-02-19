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
| Tool | Custom Rust CLI utility |
| PDF library | `lopdf` crate (or equivalent low-level Rust PDF library) |
| Approach | Overlay text and images at specified pixel coordinates on the scanned PDF page |
| Integration | Called from Elixir via `System.cmd/3` |
| Configuration | Field position file mapping logical names to coordinates and dimensions |

### Interface

```
utility --write-text "text" --x 100 --y 200 --font-size 12 input.pdf output.pdf
utility --place-image screenshot.png --x 100 --y 200 --width 300 input.pdf output.pdf
```

The utility is called multiple times to fill out the form, once per field or image placement. A configuration file maps field names to positions, enabling the Elixir application to iterate over field definitions.

### Rationale

- The human pilot specified Rust CLI tooling as the preferred approach.
- `lopdf` provides low-level PDF object manipulation needed for adding content streams to an existing page.
- A standalone CLI tool avoids NIF complexity for an operation that is inherently batch-oriented (not latency-sensitive).
- The field position configuration file decouples form layout from application logic, supporting future form revisions.

### Residual Sub-Questions

- Exact coordinate mapping for each field on the government form (requires measurement).
- Font selection and embedding for text overlay.
- Whether `lopdf` or an alternative crate (such as `printpdf` for content stream generation) is more suitable for overlaying content on existing pages.
- Image format and resolution requirements for placed images.

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

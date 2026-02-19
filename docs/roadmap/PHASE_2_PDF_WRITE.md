# Phase 2: PDF Write PoC (V0.2)

> **Navigation**: [Roadmap](./README.md) | [Documentation Root](../README.md)

## Purpose

Build a Rust command-line utility that writes text strings and places images at specified positions on PDF files. This tool populates the government-issued job search form.

## Deliverables

1. Rust CLI tool that writes text to specified coordinates on a PDF page.
2. Rust CLI tool that places images at specified coordinates on a PDF page.
3. Field position configuration file format for the government form.
4. Integration with Elixir via `System.cmd/3`.
5. Tests covering text placement, image placement, and form field mapping.

## Design

The government PDF is a flat scanned image (CCITT Group 4 fax-encoded bitmap, 1704x2200 pixels, 1-bit depth). It contains no AcroForm or XFA form fields. Text and images must be overlaid at specific pixel coordinates.

The CLI tool interface follows the pattern:
```
utility --write-text "text" --x 100 --y 200 --font-size 12 input.pdf output.pdf
utility --place-image screenshot.png --x 100 --y 200 --width 300 input.pdf output.pdf
```

A field position file maps logical field names (for example, "employer_name_row_1") to coordinates and dimensions on the form. This allows the Elixir application to populate the form by iterating over field definitions.

## Key Decision

**D1 resolved**: Rust CLI utility using `lopdf` crate for PDF manipulation, called from Elixir via `System.cmd/3`. See [RESOLVED.md](../decisions/RESOLVED.md).

## Dependencies

- V0.1 (Project Structure) must be complete.

## Status

**Not started.**

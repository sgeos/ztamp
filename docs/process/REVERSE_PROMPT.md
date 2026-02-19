# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.0-M1-P3 PDF Analysis, Roadmap, and Decision Resolution

---

## Verification

### PDF Type Analysis

**Finding: The government PDF is a flat scanned image. It is not a fillable form.**

Analysis of `secret/BaseForm.pdf` revealed the following.

| Property | Value |
|----------|-------|
| PDF version | 1.6 |
| Page count | 1 |
| Creator | RICOH IM 5000 (multifunction copier/scanner) |
| Producer | macOS Quartz PDFContext, AppendMode 1.1 |
| Image encoding | CCITT Group 4 fax decode (CCITTFaxDecode) |
| Image dimensions | 1704 x 2200 pixels |
| Color depth | 1-bit (DeviceGray, BitsPerComponent 1) |
| AcroForm fields | None |
| XFA structure | None |

The PDF is a single-page document containing one CCITT-encoded black-and-white bitmap image. The form was physically printed, an example entry was filled in by hand or typewriter, and the result was scanned on a RICOH copier. The scanned image was then split from a multi-page document on macOS using Quartz PDFContext with AppendMode.

**Implication for D1/R1.** All form-filling approaches based on AcroForm or XFA field manipulation are inapplicable. The Rust CLI approach with coordinate-based text and image overlay is the correct strategy. This aligns with the human pilot's original preference for Rust tooling.

**Type conversion.** Converting to a fillable form is theoretically possible (OCR the scanned image, define form fields at recognized locations) but would be fragile and unnecessary. Overlaying text directly at known coordinates is simpler and more reliable for a fixed-layout government form.

### Version Phases Documented

Eight phases mapped to versions V0.0 through V0.7 in `docs/roadmap/`.

| Phase | Version | Name |
|-------|---------|------|
| 0 | V0.0 | Process Definition (complete) |
| 1 | V0.1 | Project Structure |
| 2 | V0.2 | PDF Write PoC |
| 3 | V0.3 | PDF Concatenation PoC |
| 4 | V0.4 | Browser Screenshot PoC |
| 5 | V0.5 | Submission Logic |
| 6 | V0.6 | Submission Tracking |
| 7 | V0.7 | Resume Customization |

Dependency graph: V0.2 and V0.4 can proceed in parallel after V0.1. V0.3 depends on V0.2. V0.5 depends on V0.4. V0.6 and V0.7 are sequential.

COMMUNICATION.md version mapping table updated with all eight phases.

### Blocking Decisions Resolved

All three blocking decisions moved from PRIORITY.md to RESOLVED.md.

**R1. PDF Form Population.** Rust CLI utility using `lopdf` (or equivalent) for coordinate-based text and image overlay on the flat scanned PDF. Called from Elixir via `System.cmd/3`. Field position configuration file maps logical names to coordinates.

**R2. Screenshot Capture.** Wallaby (`wallaby` hex, v0.30.12). Standardized with R3 to use a single browser automation library.

**R3. Browser Automation.** Wallaby for human-in-the-loop job application submission.

### .gitignore Updated

Global whitelist patterns `!.gitkeep` and `!*.example` replace the previous `secret/`-specific negations. These patterns apply everywhere in the repository. Verified: `secret/secrets.toml.example` is visible to git.

### Knowledge Graph Updated

- `docs/README.md`: Roadmap section added.
- `docs/DOCUMENTATION_STRATEGY.md`: roadmap/ directory added to structure diagram.
- `CLAUDE.md`: Roadmap section and roadmap/ directory added.
- `docs/process/COMMUNICATION.md`: Full version mapping table (V0.0-V0.7) with link to roadmap.
- `docs/decisions/README.md`: Status counts updated (3 resolved, 0 priority).

### Process Files Updated

- TASKLOG.md: Current task updated, history entry added.
- REVERSE_PROMPT.md: Overwritten.

---

## Questions for Human Review

1. **Form field measurement.** The V0.2 phase requires mapping each field on the government form to pixel coordinates. The scanned image is 1704x2200 pixels (approximately 200 DPI for a US Letter page). Would you like to begin this measurement in a future prompt, or do you have a preferred method for defining field positions?

2. **Rust PDF crate selection.** `lopdf` provides low-level PDF object manipulation. `printpdf` provides higher-level content stream generation (text drawing, image placement) but is oriented toward creating new PDFs rather than modifying existing ones. A third option is to use `lopdf` for PDF structure and manually construct content streams for text and image overlay. Would you like research on which crate combination is most suitable for overlaying content on an existing scanned page?

3. **Phase 0 completion.** V0.0 (Process Definition) appears complete. The next step would be V0.1 (Project Structure): scaffolding the Phoenix application, integrating Rustler for NIFs, and setting up the Rust CLI tool project. Should V0.1 proceed in the next prompt?

4. **The `secret/secrets.toml.example` file** is visible to git but was not staged or committed in this prompt. Should it be committed, or was it placed there for future use?

---

## Notes

- The government form has a table with 10 numbered rows for job search entries, plus an example row at the top. Each row has columns for: employer name/address, date, contact method (T=Phone, F=Fax), Resume/E-mail, Welfare and Supportive Services Representative info, Phone No., Internet Confirmation Attached, TIME IN/OUT, and OFFICE USE ONLY.
- The form header includes case name, UPI number, and the job search period dates. These are pre-filled on the scanned copy.
- The form includes a "Should you become employed" section at the bottom and signature lines for the participant and case manager.
- Phase 0 is the third commit in the project. The process establishment phase is complete.

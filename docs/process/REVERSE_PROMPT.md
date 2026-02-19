# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.0-M1-P2 Refine Process and Add Decision Tracking

---

## Verification

### Vw-Mx-Py-Tz Coding Adopted

COMMUNICATION.md updated with the full Vw-Mx-Py-Tz coding system adapted from the crypto project. Version mapping table included with V0.0 for Phase 0 (process definition). GIT_STRATEGY.md task references updated to use the new coding format.

### Trunk-Based Development Adopted

GIT_STRATEGY.md rewritten with trunk-based development conventions following the crypto project. Feature branch naming convention (`<scope>/<short-description>`), linear history enforcement via rebase, and branch creation instructions documented. The "Branch Model" section replaced with full "Trunk-Based Development" section.

### Decision Tracking Added

Four files created in `docs/decisions/`.

1. **README.md.** Table of contents with decision status flow diagram and current counts.
2. **PRIORITY.md.** Three blocking decisions documented (D1, D2, D3).
3. **RESOLVED.md.** Empty archive, ready for resolved decisions.
4. **BACKLOG.md.** Empty deferred list, ready for future items.

### Blocking Decisions Documented with Recommendations

**D1. PDF Form Population Strategy.** Recommends `pdftk-java` via `System.cmd/3` with XFDF data interchange. The Elixir ecosystem has no native library for AcroForm form filling. The Rust crate ecosystem has `lopdf` for low-level PDF manipulation, but the higher-level form-filling crates (`pdf_form`, `acroform`) are unmaintained (4+ years without release). `pdftk-java` has a multi-decade track record and is included in standard Linux package repositories. The critical unknown is whether the government form uses AcroForm or XFA format.

**D2. Screenshot Capture Strategy.** Recommends ChromicPDF (`chromic_pdf` hex, v1.17.0) for URL-based screenshot capture, or Wallaby (`wallaby` hex, v0.30.12) if full browser interaction is needed. ChromicPDF uses the Chrome DevTools Protocol directly (no ChromeDriver dependency) and runs as an OTP supervised process. Wallaby is more capable for interactive scenarios but was designed as a testing framework.

**D3. Browser Automation Strategy.** Recommends Wallaby for interactive browser scenarios, or a standalone Rust CLI tool using `headless_chrome` (v1.0.21) to avoid BEAM scheduler concerns. This decision may be deferred if the initial scope is limited to manual application tracking rather than automated submission.

### .gitkeep and .gitignore Updated

`secret/.gitkeep` created. `.gitignore` pattern changed from `secret/` (ignores entire directory) to `secret/*` (ignores contents) with negation patterns `!secret/.gitkeep` and `!secret/*.example`. This allows the directory to be tracked while keeping sensitive contents excluded and permitting example files to document required settings.

### Knowledge Graph Updated

- `docs/README.md`: Decisions section added to contents table.
- `docs/DOCUMENTATION_STRATEGY.md`: decisions/ directory added to structure diagram.
- `CLAUDE.md`: Decisions section added to documentation table. Repository structure updated. Git Workflow section updated for trunk-based development.
- `docs/process/README.md`: Cross-reference to decisions section added.

### Process Files Updated

- TASKLOG.md: Current task updated, history entry added.
- REVERSE_PROMPT.md: Overwritten.

---

## Questions for Human Review

1. **D1 verification needed.** Is the government PDF an AcroForm or XFA form? This determines whether `pdftk-java` is viable. If you can share the PDF (in `secret/`), I can attempt to determine the form type programmatically.

2. **D2/D3 scope clarification.** Does the system need to automate the act of submitting job applications, or is it limited to tracking manually submitted applications and capturing receipts afterward? This determines whether D3 (browser automation) is in scope for initial development.

3. **Feature branch for this commit.** The git strategy now specifies trunk-based development with feature branches, but this change itself was made on `main` since the convention was not yet in place. Future prompts will use feature branches. Is this acceptable, or would you prefer I retroactively restructure the history?

4. **Version mapping expansion.** The Vw-Mx-Py-Tz coding currently defines only V0.0 (Phase 0). Should additional phases be defined now (for example, V0.1 for Phoenix scaffolding, V0.2 for PDF integration), or should they emerge as the project develops?

---

## Notes

- The `pdftk-java` recommendation requires a Java runtime in the deployment environment. This is a notable operational dependency for a project that otherwise uses Elixir and Rust.
- The `pdfium` hex package (v0.1.27) wraps Google's PDFium engine, which is the PDF renderer used in Chrome. If it supports AcroForm write operations, it would be a superior choice to `pdftk-java` because it eliminates the Java dependency. This was not confirmed during research and warrants investigation.
- All three blocking decisions (D1-D3) require Chrome or Chromium in the deployment environment. This is a shared infrastructure dependency.
- PROMPT.md was modified by the human pilot and is included in this commit per the communication protocol.

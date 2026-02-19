# Priority Decisions

> **Navigation**: [Decisions](./README.md) | [Documentation Root](../README.md)

This document captures unresolved questions that must be disambiguated before implementation can proceed with confidence.

## Document Status

**Created**: 2026-02-19
**Purpose**: Blocking decision checklist
**Related**: [Resolved](./RESOLVED.md), [Backlog](./BACKLOG.md)

---

## D1. PDF Form Population Strategy

**Status**: Open
**Blocking**: Application form generation feature

### Context

The system must populate a government-issued PDF form with job search activity data. The PDF is pre-existing and distributed by the government. The form uses AcroForm fields (assumption, pending verification of actual form structure).

### Options Under Consideration

| Option | Approach | Maturity | Risk |
|--------|----------|----------|------|
| A | `pdftk-java` via `System.cmd/3` with XFDF | High (multi-decade track record) | Java runtime dependency |
| B | Rust NIF over `lopdf` crate via Rustler | Medium (lopdf is mature, but no high-level form API) | Equivalent to building a library |
| C | `pdfium` hex package (Google PDFium wrapper) | Low-Medium (v0.1.27, November 2025) | AcroForm write support unverified |

### Recommendation

**Option A: `pdftk-java` via `System.cmd/3`** is the recommended approach for the following reasons.

`pdftk-java` is the only surveyed option with a multi-decade track record of AcroForm form filling. It is actively maintained (v3.3.3-2, December 2025) and included in standard Linux package repositories. The integration requires approximately 50 to 100 lines of Elixir: an XFDF generator, a temporary file wrapper, and a `System.cmd` call. XFDF is preferred over FDF for encoding field values because it handles non-ASCII characters more reliably.

### Caveats

- Government forms sometimes use XFA (Adobe LiveCycle) rather than AcroForm. If the target form is XFA, `pdftk` will not work. This must be verified against the actual PDF.
- The Rust NIF approach (Option B) using `lopdf` directly would require building custom AcroForm traversal logic. The existing higher-level Rust crates (`pdf_form`, `acroform`) have not been released in four or more years and carry unacceptable maintenance risk.
- No native Elixir library supports AcroForm form filling as of February 2026.

### Questions to Resolve

1. Is the government PDF an AcroForm or XFA form?
2. Is `pdftk-java` available in the deployment environment?
3. Are there Unicode or special character requirements in form field values?

---

## D2. Screenshot Capture Strategy

**Status**: Open
**Blocking**: Receipt screenshot feature

### Context

The system must capture screenshots of post-submission receipt pages after each job application. These screenshots serve as evidence of application completion for TANF compliance.

### Options Under Consideration

| Option | Approach | Maturity | Risk |
|--------|----------|----------|------|
| A | ChromicPDF (`chromic_pdf` hex, v1.17.0) | High (production-designed, CDP-based) | Screenshot is secondary to PDF generation purpose |
| B | Wallaby (`wallaby` hex, v0.30.12) | High (actively maintained) | Designed for testing, not production use |
| C | `playwright-elixir` (v1.49.1-alpha.2) | Low (alpha status) | API instability, incomplete feature coverage |

### Recommendation

**Option A: ChromicPDF** is the recommended approach for screenshot capture that does not require browser interaction (navigating to a known URL and capturing the page).

ChromicPDF communicates with Chrome directly via the Chrome DevTools Protocol (CDP) over a pipe, avoiding ChromeDriver version-compatibility fragility. It runs as an OTP supervised process, which is idiomatic for Phoenix applications. It supports cookie injection, JavaScript wait conditions, and full-page capture.

**Option B: Wallaby** is the recommended approach if the use case requires full browser interaction (navigating through a form, submitting it, then capturing the resulting page within a single automated session).

### Caveats

- If the receipt page requires an authenticated browser session with prior form submission, ChromicPDF alone may be insufficient. In that case, Wallaby or a combination approach would be needed.
- The Rust `headless_chrome` crate (v1.0.21) is an option for a Rustler NIF, but the implementation complexity and BEAM scheduler blocking concerns make it less attractive than the Elixir-native options.
- Chrome or Chromium must be available in the deployment environment for any of these approaches.

### Questions to Resolve

1. Does the receipt page require an authenticated browser session, or is a direct URL accessible?
2. Is the screenshot captured immediately after a human submits the application, or does the system submit applications automatically?
3. What viewport dimensions and image format are acceptable for compliance documentation?

---

## D3. Browser Automation Strategy

**Status**: Open
**Blocking**: Automated job application submission (if required)

### Context

If the system must automate the submission of job applications (not just track them), browser automation is needed to interact with employer application forms. This is distinct from D2 (screenshot capture) because it involves active interaction with external websites rather than passive page capture.

### Options Under Consideration

| Option | Approach | Maturity | Risk |
|--------|----------|----------|------|
| A | Wallaby with ChromeDriver | High (v0.30.12, January 2026) | Test-oriented design, ChromeDriver version coupling |
| B | `playwright-elixir` | Low (alpha) | API instability |
| C | Standalone Rust CLI tool using `headless_chrome` | Medium (v1.0.21) | Separate process, requires IPC with Phoenix app |

### Recommendation

**Option A: Wallaby** is the most mature Elixir browser automation library for interactive scenarios. Despite being designed as a testing framework, it provides the full WebDriver API needed for form interaction.

If automated application submission is required, a standalone Rust CLI tool (Option C) using the `headless_chrome` crate is a viable alternative. This approach avoids BEAM scheduler concerns and can be invoked via `System.cmd/3` or an Elixir `Port`. The trade-off is increased operational complexity and the need for inter-process communication.

### Caveats

- Automated job application submission is a significantly more complex undertaking than job tracking and screenshot capture. Each employer website has different form structures, anti-bot measures, and submission flows.
- This decision may be deferred if the initial scope is limited to manual application tracking with automated form population and screenshot capture.

### Questions to Resolve

1. Is automated job application submission in scope, or is the system limited to tracking manually submitted applications?
2. If automated submission is in scope, what job boards or employer sites must be supported?
3. How should anti-bot detection measures (CAPTCHAs, rate limiting) be handled?

---

## Revision History

| Date | Author | Changes |
|------|--------|---------|
| 2026-02-19 | Claude | Initial creation with D1 (PDF), D2 (screenshot), D3 (browser automation). |

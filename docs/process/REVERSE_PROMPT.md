# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.1-M1-P3 Documentation Update and V0.1 Status

---

## V0.1 Status Report

V0.1 (Project Structure) is **complete**. All five deliverables have been verified.

| Deliverable | Status | Verification |
|-------------|--------|--------------|
| Phoenix application with LiveView | Complete | `ztamp/` scaffolded. Phoenix 1.8.3, LiveView 1.1.24. |
| Rust NIF integration via Rustler | Complete | `rztamp/` library. `ztamp/native/nif/` bridge. `Ztamp.Nif.alive/0` calls through to `rztamp::alive()`. |
| Database setup (PostgreSQL via Ecto) | Complete | `mix ecto.create` run successfully by human pilot. |
| Test framework configured | Complete | ExUnit: 5 tests pass. `cargo test`: 1 test passes. |
| Development environment documentation | Complete | `CLAUDE.md` documents structure, stack, and workflow. `docs/` knowledge graph established. |

Additional work completed during V0.1 beyond original deliverables includes field value extraction (`secret/secrets.toml`), form field offset generation (`assets/form/form_offsets.toml`), and a non-confidential assets directory (`assets/form/`).

**The project is ready to advance to V0.2 (PDF Write PoC).**

---

## Documentation Updates

### Phase Documentation

- **PHASE_1_STRUCTURE.md**: Status updated to Complete with a deliverable verification table. Additional completed work documented.
- **PHASE_2_PDF_WRITE.md**: Rewritten to reflect the refined strategy. `printpdf` for PDF generation. Routines in `rztamp`. Form offsets in `assets/form/form_offsets.toml`. Calibration via generate-inspect-adjust loop with colored text.

### Decision Records

- **RESOLVED.md (R1)**: Refined to specify `printpdf` for new PDF generation and `lopdf` reserved for V0.3 concatenation. Updated workflow, rationale, and residual sub-questions.
- **PRIORITY.md**: Field coordinate mapping and `lopdf`-vs-`printpdf` questions marked as resolved. New question added for template TIFF compatibility with `printpdf`.

### Project Root

- **CLAUDE.md**: Status updated to "V0.1 Complete. Ready for V0.2 (PDF Write PoC)."

---

## Decisions Incorporated from Prompt Comments

The following four decisions from the previous reverse prompt were resolved by the human pilot in this prompt.

| Question | Resolution | Action Taken |
|----------|------------|--------------|
| Form offsets location | Acceptable in `assets/form/` | No change needed. |
| Coordinate system (mm from top-left) | Acceptable as long as it works | No change needed. Documented in Phase 2. |
| Calibration approach | Generate, inspect, adjust loop with colored text | Documented in Phase 2 and R1. |
| `printpdf` dependency timing | Can be added now | Documented in Phase 2. Actual dependency addition deferred to first V0.2 prompt. |

---

## Remaining V0.1 Work

None. V0.1 is complete.

---

## Questions for Human Review

No blocking questions at this time. The next step is to begin V0.2 (PDF Write PoC) when the human pilot is ready.

---

## Notes

- The human pilot reordered sections in `secrets.toml` (moving `[form]` above `[participant]`) and improved `secrets.toml.example` with representative placeholder values. These changes have been noted.
- The `printpdf` dependency has not been added to `rztamp/Cargo.toml` yet. The prompt comments said it "can be added now" but no explicit objective in this prompt requested it. It will be added in the first V0.2 prompt.

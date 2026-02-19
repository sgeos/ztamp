# Task Log

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

Current task state and verification log. This file is the shared source of truth for the active unit of work.

---

## Current Task

**Name**: Field Extraction and Form Offsets (V0.1-M1-P2)
**Status**: Complete
**Started**: 2026-02-19

## Success Criteria

- [x] Extract field values from the original PDF to `secret/secrets.toml`.
- [x] Generate form field offsets and store in `assets/form/form_offsets.toml`.
- [x] Update relevant documentation.

## Task Breakdown

| ID | Task | Status | Verification |
|----|------|--------|--------------|
| V0.1-M1-P2-T1 | Create non-confidential assets directory | Complete | `assets/form/` created with `template.tiff` copied from `secret/form.tiff`. |
| V0.1-M1-P2-T2 | Extract field values from PDF | Complete | 8 fields extracted to `secret/secrets.toml`. Values verified against PDF visual inspection. |
| V0.1-M1-P2-T3 | Update secrets.toml.example | Complete | Template expanded with `[job_search]` and `[submission]` sections. |
| V0.1-M1-P2-T4 | Generate form field offsets | Complete | 40+ field positions in `assets/form/form_offsets.toml`. Coordinates in mm from top-left. |
| V0.1-M1-P2-T5 | Update CLAUDE.md structure diagram | Complete | `assets/form/` added to repository structure. |
| V0.1-M1-P2-T6 | Update process files and commit | Complete | TASKLOG.md, REVERSE_PROMPT.md updated. |

## Notes

- The prompt specified `secrets/form_offsets.toml` (with trailing 's') but the project uses `secret/` (no trailing 's'). Since form offsets are not confidential, they were placed in `assets/form/form_offsets.toml` per the prompt comments requesting a non-confidential directory.
- Field offset coordinates are initial estimates from visual inspection. They require calibration during V0.2 (PDF Write PoC).
- The `secret/form.png` file caused API errors due to media type mismatch (noted in prompt comments). The TIFF version was used as the canonical template.
- PDF manipulation routines will be defined in the `rztamp` library as noted in the prompt comments.

## History

| Date | Change |
|------|--------|
| 2026-02-19 | V0.0: Process definition, knowledge graph, PDF analysis, roadmap, decision resolution. |
| 2026-02-19 | V0.1-M1-P1: Project structure generated. rztamp Rust library, ztamp Phoenix project, NIF bridge with Rustler, secrets.toml.example. |
| 2026-02-19 | V0.1-M1-P2: Field extraction and form offsets. secrets.toml populated, form_offsets.toml created in assets/form/, documentation updated. |

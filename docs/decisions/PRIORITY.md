# Priority Decisions

> **Navigation**: [Decisions](./README.md) | [Documentation Root](../README.md)

This document captures unresolved questions that must be disambiguated before implementation can proceed with confidence.

## Document Status

**Created**: 2026-02-19
**Updated**: 2026-02-19
**Purpose**: Blocking decision checklist
**Related**: [Resolved](./RESOLVED.md), [Backlog](./BACKLOG.md)

---

## Resolution Summary

All initial blocking decisions have been resolved. See [RESOLVED.md](./RESOLVED.md) for full details.

| Decision | Topic | Status | Resolution |
|----------|-------|--------|------------|
| D1 | PDF Form Population | **Resolved** | [R1](./RESOLVED.md) Rust CLI with coordinate-based text/image overlay |
| D2 | Screenshot Capture | **Resolved** | [R2](./RESOLVED.md) Wallaby (standardized with D3) |
| D3 | Browser Automation | **Resolved** | [R3](./RESOLVED.md) Wallaby for human-in-the-loop |

---

## Remaining Open Questions

No blocking decisions remain for Phase 0 or Phase 1.

The following questions are anticipated for future phases but do not block current work.

### From R1: PDF Form Population

| Question | Phase | Notes |
|----------|-------|-------|
| Exact field coordinate mapping for government form | V0.2 | Requires measurement against BaseForm.pdf |
| Font selection and embedding for text overlay | V0.2 | Determine during Rust CLI implementation |
| `lopdf` vs `printpdf` for content stream overlay | V0.2 | Evaluate during PoC |

### From R2/R3: Browser Automation

| Question | Phase | Notes |
|----------|-------|-------|
| Supported job boards and employer sites | V0.5 | Define during submission logic phase |
| ChromeDriver lifecycle in production | V0.5 | Determine during integration |

---

## Revision History

| Date | Author | Changes |
|------|--------|---------|
| 2026-02-19 | Claude | Initial creation with D1 (PDF), D2 (screenshot), D3 (browser automation). |
| 2026-02-19 | Claude | D1-D3 resolved. Moved to RESOLVED.md as R1-R3. |

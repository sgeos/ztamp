# Roadmap

> **Navigation**: [Documentation Root](../README.md)

Development phases for the TANF Job Search Automation project. Each phase maps to a version in the [Vw-Mx-Py-Tz](../process/COMMUNICATION.md) coding system.

## Phase Overview

| Phase | Version | Name | Description |
|-------|---------|------|-------------|
| 0 | V0.0 | [Process Definition](./PHASE_0_PROCESS.md) | Project setup, knowledge graph, and workflow |
| 1 | V0.1 | [Project Structure](./PHASE_1_STRUCTURE.md) | Phoenix/Elixir application scaffolding and Rust NIF integration |
| 2 | V0.2 | [PDF Write PoC](./PHASE_2_PDF_WRITE.md) | Rust CLI tool to write text and place images on PDF forms |
| 3 | V0.3 | [PDF Concatenation PoC](./PHASE_3_PDF_CONCAT.md) | Combine filled form pages and screenshot evidence into single PDF |
| 4 | V0.4 | [Browser Screenshot PoC](./PHASE_4_SCREENSHOT.md) | Automated screenshot capture of submission receipt pages |
| 5 | V0.5 | [Submission Logic](./PHASE_5_SUBMISSION.md) | Per-platform job application submission workflows |
| 6 | V0.6 | [Submission Tracking](./PHASE_6_TRACKING.md) | Application tracking, weekly hour logging, and compliance reporting |
| 7 | V0.7 | [Resume Customization](./PHASE_7_RESUME.md) | Per-application resume and cover letter customization |

## Phase Dependencies

```
V0.0 -> V0.1 -> V0.2 -> V0.3
                    |
                    +---> V0.4 -> V0.5 -> V0.6 -> V0.7
```

Phases V0.2 and V0.4 can proceed in parallel after V0.1 is complete. V0.3 depends on V0.2 (PDF writing must work before concatenation). V0.5 depends on V0.4 (screenshot capture must work before submission workflows incorporate it). V0.6 and V0.7 build on the full pipeline.

## Related Sections

- [Decisions](../decisions/README.md) for blocking and resolved decisions
- [Architecture](../architecture/README.md) for technology stack
- [Process](../process/README.md) for development workflow

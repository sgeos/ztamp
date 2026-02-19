# Documentation Strategy

> **Navigation**: [Documentation Root](./README.md)

Meta-documentation describing the conventions governing this knowledge graph.

## Principles

### Atomic Files

Each file covers one concept. This keeps the Signal-to-Noise Ratio (SNR) high and allows AI agents to load only the context they need for a given task.

### Hierarchical Organization

Every section has a `README.md` that serves as a table of contents. High-level files provide orientation. Atomic files provide precision.

### Breadcrumb Navigation

Every file includes an upward navigation link to its parent table of contents and the documentation root. This allows both humans and AI agents to orient themselves quickly.

## Naming Conventions

- File names use `UPPER_SNAKE_CASE` with a `.md` extension.
- Table of contents files are named `README.md`.
- Section directories use `lowercase` names.

## Directory Structure

```
docs/
├── README.md
├── DOCUMENTATION_STRATEGY.md
├── overview/
│   ├── README.md
│   └── PRODUCT_IDENTITY.md
├── architecture/
│   ├── README.md
│   └── TECHNOLOGY_STACK.md
├── decisions/
│   ├── README.md
│   ├── RESOLVED.md
│   ├── PRIORITY.md
│   └── BACKLOG.md
├── roadmap/
│   ├── README.md
│   ├── PHASE_0_PROCESS.md
│   ├── PHASE_1_STRUCTURE.md
│   ├── PHASE_2_PDF_WRITE.md
│   ├── PHASE_3_PDF_CONCAT.md
│   ├── PHASE_4_SCREENSHOT.md
│   ├── PHASE_5_SUBMISSION.md
│   ├── PHASE_6_TRACKING.md
│   └── PHASE_7_RESUME.md
├── process/
│   ├── README.md
│   ├── COMMUNICATION.md
│   ├── GIT_STRATEGY.md
│   ├── TASKLOG.md
│   ├── PROMPT.md
│   └── REVERSE_PROMPT.md
└── reference/
    ├── README.md
    └── GLOSSARY.md
```

## AI Agent Navigation

1. Start at `docs/README.md`.
2. Read the relevant section `README.md` to understand available files.
3. Load atomic files only when needed for the current task.
4. Use upward navigation if disoriented.
5. Do not load all documentation files at once.

## Cross-Reference Conventions

- Use relative markdown links between files. Example: `[Technology Stack](../architecture/TECHNOLOGY_STACK.md)`.
- Include a "Related Sections" footer in content files where cross-references are useful.
- Navigation breadcrumbs always point upward to the parent table of contents.

## Maintenance

- Add new concepts as atomic files within the appropriate section.
- Split files that exceed approximately 200 lines.
- When removing content, remove it from the parent `README.md` table of contents first.
- Keep this strategy document updated when structural conventions change.

## Version History

| Date | Author | Changes |
|------|--------|---------|
| 2026-02-19 | Claude | Initial creation. Adapted from reference projects. |
| 2026-02-19 | Claude | Added decisions/ section to directory structure. |
| 2026-02-19 | Claude | Added roadmap/ section to directory structure. |

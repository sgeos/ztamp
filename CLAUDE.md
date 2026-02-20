# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

TANF Job Search Automation tool. Automates job search documentation for Temporary Assistance for Needy Families (TANF) compliance, including government PDF form population, job application tracking, and receipt screenshot capture.

**Status**: V0.2 PDF Write PoC complete. CLI tool generates filled PDF forms with printpdf. Calibration in progress.

## Technology Stack

- **Elixir** with **Phoenix** and **LiveView** for the web application
- **Rust NIFs** for number crunching and modify-in-place algorithms
- **Rust** for custom command line tools as needed

## Repository Structure

```
tanf/                          # Git repository root
├── CLAUDE.md                  # AI agent instructions
├── .gitignore                 # Excludes secret/ and editor artifacts
├── rztamp/                    # Portable Rust library (shared logic)
│   ├── Cargo.toml             # printpdf, image, serde, toml
│   └── src/
│       ├── lib.rs
│       ├── offsets.rs          # Form field offset types (TOML deserialization)
│       └── pdf.rs              # PDF generation (template + text overlay)
├── tools/                     # Rust CLI tools
│   ├── Cargo.toml             # Depends on rztamp
│   └── src/
│       └── fill.rs            # tanf-fill: generate filled PDF forms
├── ztamp/                     # Phoenix/LiveView web application
│   ├── mix.exs
│   ├── lib/ztamp/nif.ex       # Rustler NIF wrapper
│   └── native/nif/            # Rust NIF bridge (depends on rztamp)
│       ├── Cargo.toml
│       └── src/lib.rs
├── assets/                    # Non-confidential project assets (tracked)
│   └── form/                  # Form template and field definitions
│       ├── template.tiff      # Cleaned form image (base layer for PDF generation)
│       └── form_offsets.toml  # Field positions in mm from top-left
├── secret/                    # Sensitive files (gitignored, .gitkeep tracked)
└── docs/                      # Documentation knowledge graph
    ├── README.md              # Documentation root
    ├── DOCUMENTATION_STRATEGY.md
    ├── overview/              # Product identity and context
    ├── architecture/          # Technology stack and design
    ├── decisions/             # Architectural and design decisions
    ├── roadmap/               # Development phases (V0.0-V0.7)
    ├── process/               # Workflow and communication
    └── reference/             # Glossary and reference material
```

## Documentation

A knowledge graph is maintained in `docs/`. Start at [`docs/README.md`](docs/README.md) for navigation.

| Section | Path | Description |
|---------|------|-------------|
| Overview | [`docs/overview/`](docs/overview/README.md) | Product identity and project context |
| Architecture | [`docs/architecture/`](docs/architecture/README.md) | Technology stack and system design |
| Decisions | [`docs/decisions/`](docs/decisions/README.md) | Architectural and design decisions |
| Roadmap | [`docs/roadmap/`](docs/roadmap/README.md) | Development phases and version planning |
| Process | [`docs/process/`](docs/process/README.md) | Development workflow, communication, and task tracking |
| Reference | [`docs/reference/`](docs/reference/README.md) | Glossary and supplementary reference material |

## Git Workflow

Trunk-based development with short-lived feature branches. All changes arrive on `main` via feature branch merge with rebase. See [`docs/process/GIT_STRATEGY.md`](docs/process/GIT_STRATEGY.md) for full details.

Use scoped conventional commits: `<scope>: <imperative summary>`. Common scopes: `feat`, `fix`, `docs`, `refactor`, `chore`, `test`. Include `Co-Authored-By: Claude <noreply@anthropic.com>` when AI-assisted.

The AI agent commits once after all tasks in a prompt are complete, including the `REVERSE_PROMPT.md` update. `PROMPT.md` is read-only for the AI agent but must be included in the commit if the human pilot has modified it.

## Session Startup Protocol

1. Read [`docs/process/TASKLOG.md`](docs/process/TASKLOG.md) for current task state.
2. Read [`docs/process/REVERSE_PROMPT.md`](docs/process/REVERSE_PROMPT.md) for last AI communication.
3. Wait for human prompt before proceeding.

## Sensitive Files

The `secret/` directory is gitignored and contains sensitive documents such as the government-issued PDF form. Never commit files from this directory.

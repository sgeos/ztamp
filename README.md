# TANF Job Search Automation

Automates job search documentation for Temporary Assistance for Needy Families (TANF) compliance. Includes government PDF form population, job application tracking with screenshot capture, and reporting.

## Components

| Directory | Description | Setup |
|-----------|-------------|-------|
| [ztamp/](ztamp/README.md) | Phoenix/LiveView web application | Elixir, PostgreSQL, ChromeDriver |
| [rztamp/](rztamp/README.md) | Portable Rust library (shared logic) | Rust toolchain |
| tools/ | Rust CLI tools (tanf-fill, tanf-concat) | Rust toolchain |

## Documentation

See [docs/README.md](docs/README.md) for the full documentation knowledge graph.

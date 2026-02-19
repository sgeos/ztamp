# Technology Stack

> **Navigation**: [Architecture](./README.md) | [Documentation Root](../README.md)

## Primary Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Web Framework | Phoenix with LiveView | Interactive web interface for job tracking and form management |
| Language | Elixir | Application logic, OTP supervision, and process management |
| Native Extensions | Rust NIFs | Number crunching and modify-in-place algorithms |
| Command Line Tools | Rust | Custom CLI utilities as needed |

## Rationale

Phoenix LiveView provides real-time, server-rendered UI without requiring a separate JavaScript frontend framework. Elixir and OTP provide fault tolerance and process supervision. Rust NIFs handle performance-critical operations such as PDF manipulation, image processing, and any algorithms that benefit from mutable state or deterministic resource usage.

## Key Capabilities Needed

- **PDF Reading and Writing.** Populate government-issued PDF forms programmatically.
- **Screenshot Capture.** Automate browser-based screenshot capture of receipt pages.
- **Data Persistence.** Store job application records, weekly hour logs, and form submission history.
- **Web Interface.** LiveView dashboard for managing applications and generating reports.

## Related Sections

- [Product Identity](../overview/PRODUCT_IDENTITY.md) for project requirements

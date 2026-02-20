# rztamp

Portable Rust library for TANF job search automation. Contains shared logic used by both the NIF bindings (called from Elixir via Rustler) and the standalone CLI tools.

## Prerequisites

- Rust toolchain (stable, edition 2024). Install via [rustup](https://rustup.rs/).

## Building

```sh
cargo build
cargo test
```

## Dependencies

- `printpdf` 0.9 with TIFF support for PDF generation
- `image` 0.25 for TIFF and PNG image processing
- `serde` 1 and `toml` 0.8 for configuration deserialization

## Modules

- `offsets` - Form field offset types for TOML deserialization
- `pdf` - PDF generation with template image overlay and text field rendering

# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-19
**Task**: V0.1-M1-P1 Project Structure Generation

---

## Verification

### rztamp Rust Library

Created at `rztamp/` with `edition = "2024"`. Exposes a single `alive()` function that returns `true`. Unit test passes via `cargo test`.

### ztamp Phoenix Project

Generated with `mix phx.new ztamp --database postgres --live`. Dependencies fetched successfully. Phoenix 1.8.3, LiveView 1.1.24, Ecto 3.13.5, Postgrex 0.22.0.

### NIF Bridge

| Component | Path | Details |
|-----------|------|---------|
| Rust NIF crate | `ztamp/native/nif/` | `rustler = "0.37"`, depends on `rztamp` via relative path |
| Elixir wrapper | `ztamp/lib/ztamp/nif.ex` | `use Rustler, otp_app: :ztamp, crate: "nif"` |
| Compiled artifact | `ztamp/priv/native/nif.so` | Compiled in release mode |

End-to-end verification:

```
mix run -e "IO.inspect(Ztamp.Nif.nop()); IO.inspect(Ztamp.Nif.alive())"
:ok
true
```

Both NIF functions load and execute correctly. `alive/0` calls through to `rztamp::alive()`, confirming the Rust library is linked.

### secrets.toml.example

Populated with `[participant]` (case_name, upi_number) and `[form]` (base_form_path) sections. Committed from `secret/`.

### CLAUDE.md Updated

Repository structure diagram updated to include `rztamp/`, `ztamp/`, and `native/nif/`. Status updated to V0.1.

---

## PDF Strategy Discussion

The prompt raised two related questions about PDF handling strategy. These are addressed below.

### Image Extraction and Cleanup

The idea of extracting the scanned image from the PDF, straightening it, and clearing filled fields is sound in principle. The PDF contains a single CCITT Group 4 encoded bitmap at 1704x2200 pixels (approximately 200 DPI for US Letter). Extracting this image is straightforward with `lopdf` (read the image stream object and decode it) or even a CLI tool like `pdfimages`.

However, "clearing filled fields" on a scanned image is an image processing problem, not a PDF problem. The handwritten example entries are baked into the pixel data. Removing them would require either manual image editing (recommended for a one-time cleanup) or heuristic image processing (fragile and likely overkill for a single form template).

**Recommended approach**: Extract the image once, straighten and clean it manually in an image editor (GIMP or similar), and save the cleaned image as the canonical template. This cleaned image becomes the base layer for all generated forms.

### printpdf versus lopdf for PDF Generation

The two crates serve different purposes and are not mutually exclusive.

**`printpdf`** creates new PDF documents from scratch. It provides a high-level API for placing text, images, and vector graphics onto pages. If the strategy is to extract the cleaned form image and then generate fresh PDFs with the image as a background layer and field data overlaid as native PDF text, `printpdf` is the better choice. This approach produces clean, searchable PDFs where the text is actual text (not pixel data).

**`lopdf`** manipulates existing PDF structure at the object level. It is suitable for modifying an existing PDF in place, for example appending content streams to an existing page. It is lower-level and requires manual construction of PDF content stream operators for text placement.

**Recommendation**: Use `printpdf` for the primary workflow. The strategy would be:

1. Extract and clean the form image once (manual step).
2. At generation time, create a new PDF with `printpdf`.
3. Place the cleaned form image as the page background.
4. Overlay field data as native PDF text at known coordinates.
5. The result is a clean PDF with selectable text.

Reserve `lopdf` for any future need to manipulate existing PDF files (for example, concatenating filled forms with screenshot PDFs in V0.3).

### Field Definition Format

Defining field positions in TOML is a good approach. A structure like the following would map logical field names to page coordinates.

```toml
[fields.employer_name_1]
x = 85
y = 420
width = 200
font_size = 10

[fields.date_1]
x = 290
y = 420
width = 60
font_size = 10
```

The coordinate system would need to be calibrated against the cleaned template image. This work belongs in V0.2 (PDF Write PoC).

---

## Questions for Human Review

1. **Image cleanup method.** Should the form image extraction and cleanup be done manually (one-time task in an image editor), or should the project include automated image processing for this step?

2. **Tools subproject.** The prompt mentioned that "it probably makes sense to have a `tools` subproject for CLI tools based on `rztamp` and utility shell scripts." This was not listed as an explicit objective for this prompt. Should a `tools/` directory be scaffolded in a future prompt, or should CLI tools live directly in `rztamp` as binary targets?

3. **Database setup.** The Phoenix project was generated with PostgreSQL support but `mix ecto.create` was not run (it requires a running PostgreSQL instance). Should database setup be deferred to a later prompt, or should it be verified now?

---

## Notes

- The `tools` subproject was not created in this prompt. It was mentioned in comments but not in the objectives.
- The Phoenix project includes the default generated code (home page, error views, etc.). Customization of the web interface is expected in later phases.
- Rust edition 2024 was used for both `rztamp` and the NIF crate, matching the edition specified in `rztamp/Cargo.toml`.
- The NIF adds an `alive/0` function beyond the reference project's `nop/0` stub, providing a direct verification path from Elixir through to `rztamp`.

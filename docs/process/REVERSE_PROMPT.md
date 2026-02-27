# Reverse Prompt

> **Navigation**: [Process](./README.md) | [Documentation Root](../README.md)

This file is the AI-to-human communication channel. It is overwritten after each completed task.

---

## Last Updated

**Date**: 2026-02-27
**Task**: V0.5-M1-P6 Reverse Prompt Response

---

## Verification

### Changes Implemented

| Item | Detail |
|------|--------|
| Watermark color | Changed from washed-out red (RGB 1.0, 0.5, 0.5) to pure red (`TextColor::RED`, RGB 1.0, 0.0, 0.0) in `tools/src/export.rs`. |
| Dev-only links gated | Dashboard link in the app navbar and Dashboard/Mailbox cards on the landing page are now conditionally rendered using `Application.compile_env(:ztamp, :dev_routes, false)`. In dev mode, links appear. In prod mode, they are omitted entirely from the HTML. |
| Container width | Confirmed acceptable. No changes made. |

### Architecture

| Component | Path | Change |
|-----------|------|--------|
| `tanf-export` | `tools/src/export.rs` | Watermark color changed to `TextColor::RED`. |
| `Layouts` | `ztamp/lib/ztamp_web/components/layouts.ex` | Added `@dev_routes` module attribute. Injects into assigns via `assign_new`. Dashboard navbar link gated with `:if={@dev_routes}`. |
| `PageController` | `ztamp/lib/ztamp_web/controllers/page_controller.ex` | Passes `dev_routes: @dev_routes` assign to the home template. |
| Home template | `ztamp/lib/ztamp_web/controllers/page_html/home.html.heex` | Dashboard and Mailbox cards gated with `:if={@dev_routes}`. |

---

## Manual Verification

1. Start the server: `cd ztamp && mix phx.server`
2. Navigate to `http://localhost:4000/`
3. Verify the landing page shows three cards in dev mode (Job Search, LiveView Dashboard, Mailbox).
4. Verify the navbar shows the Dashboard link in dev mode.
5. Export a PDF with the Watermark checkbox enabled and verify the watermark text is pure red.

---

## Questions for Human Review

No questions at this time.

---

## Commands

### Start the Phoenix Server

```
cd ztamp && mix phx.server
```

### Rebuild the Export Tool

```
cd tools && cargo build --release
```

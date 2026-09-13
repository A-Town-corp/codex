# Local cost field

| Component | Responsibility |
|---|---|
| `codex-rs/tui/src/bottom_pane/status_line_setup.rs` | String field identifiers and `/statusline` selection. |
| `codex-rs/tui/src/bottom_pane/status_surface_preview.rs` | Preview values in settings. |
| `codex-rs/tui/src/bottom_pane/status_line_style.rs` | Existing theme colors for usage fields. |
| `codex-rs/tui/src/chatwidget/status_surfaces.rs` | Resolve the selected field from current model and cumulative token counters. |
| `codex-rs/tui/src/chatwidget/tests/status_and_layout.rs` | Existing test helpers plus local-estimate visibility tests. |

The installed Codex application is independent of this checkout. No credentials are stored in this repository.

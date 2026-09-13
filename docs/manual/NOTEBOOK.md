# Local cost statusline work log

## Task — 2026-09-13

- User requests an always-visible dollar estimate in the Codex CLI footer and explicitly accepts API-equivalent token value rather than subscription billing.
- Base: upstream tag `rust-v0.154.0`, commit `6b9826e3aa83b1a5947db50f4332cb9c65f1b340`; branch `local-cost-statusline`.
- Installed executable: `C:/Users/Fonem/.codex/packages/standalone/current/bin/codex.exe`, version `0.154.0`. Do not replace until custom build and footer are exercised.
- Mechanism: `chatwidget/status_surfaces.rs` renders `EstimatedThreadCost` only from `estimated_usage_usd_micros`. `StatusLineItem` is an enum, with no arbitrary command callback.
- Decision: add `local-token-cost`, independent of billing eligibility. Reuse cumulative TUI token totals. Label `API est`; calculate the equivalent value at the currently selected model's Standard short-context rate. Changing models intentionally revalues the total. Exclude tool fees, cache-write premiums, Fast mode, long-context premiums, and child-thread usage not in those totals. This is a comparison estimate, not an invoice.
- Rejected: editing account eligibility (does not supply billing data); renaming conversations (changes saved names); terminal-output interception (fragile during redraw).
- Verification plan: zero usage, cache discount, reasoning not double-counted, unknown model, pending usage, rendering snapshot, Windows launch.

## Research Log

| Round | Query / action | Tool | New sources / result |
|---|---|---|---|
| 1 | `repo:openai/codex "estimated-thread-cost"` | GitHub code search; fetch_content | Read status_line_setup.rs and thread_usage_tests.rs at upstream commit 36f0dbe796d9bb1a18a0fc0640ed08b3e1d54564; fixed fields, billing tests. |
| 2 | `repo:openai/codex "estimated_thread_cost" path:codex-rs/tui/src`; `repo:openai/codex "EstimatedThreadCost =>" path:codex-rs/tui` | GitHub code search; fetch_content | Read status_surfaces.rs; renderer requires optional server dollars. |
| 3 | `repo:openai/codex "status_line" "command" path:codex-rs/core/src/config`; `gpt-6-astra pricing input cached output`; `GPT-6 Astra $10 $50 pricing` | GitHub code search; OpenAI docs search/fetch | Config search found no custom field. Read ChatGPT credit rates and API pricing; API Standard rate table is applicable to the requested estimate. |

Totals: 6 search queries; 5 distinct fetched source URLs. GitHub extractor flattened Rust and truncated tests; checked out matching release to inspect exact source. API pricing anchor was missing; fetch returned full document. No billing bypass or custom command option found.

Sources:
- https://github.com/openai/codex/blob/36f0dbe796d9bb1a18a0fc0640ed08b3e1d54564/codex-rs/tui/src/bottom_pane/status_line_setup.rs
- https://github.com/openai/codex/blob/36f0dbe796d9bb1a18a0fc0640ed08b3e1d54564/codex-rs/tui/src/chatwidget/thread_usage_tests.rs
- https://github.com/openai/codex/blob/36f0dbe796d9bb1a18a0fc0640ed08b3e1d54564/codex-rs/tui/src/chatwidget/status_surfaces.rs
- https://learn.chatgpt.com/docs/pricing
- https://developers.openai.com/api/docs/pricing

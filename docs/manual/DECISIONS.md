# Decisions

## 2026-09-13 — API-equivalent footer

| Rule | Reason | Rejected alternative |
|---|---|---|
| Add `local-token-cost`; retain `estimated-thread-cost` unchanged. | Keep locally calculated values distinct from billing-service estimates. | Reuse the enterprise field: conflates two meanings and changes existing billing tests. |
| Value cumulative input, cached input, and output at the current model's Standard short-context API prices. | These counters already exist in the terminal user interface (TUI). | Treat subscription usage as a dollar charge: no such billing data is available. |
| Show `API est loading` while usage is pending; `API est unavailable` for unknown model prices. | Unknown data must not look like a zero-dollar total. | Guess the price or hide the field. |
| Keep custom executable separate until tested. | Preserve the installed CLI during compilation and verification. | Patch the installed executable in place. |

Prices are from https://developers.openai.com/api/docs/pricing, read 2026-09-13. Input includes cached input; subtract cached input before applying the uncached price. Output includes reasoning; do not add reasoning output again. This deliberately excludes tool fees, cache writes, Fast mode, long-context and regional premiums. Model changes revalue the entire displayed token total at the new model's rate.

# Token-cost pricing check, 2026-10-01

Narrow implementation research: establish the current model rates, verify cached-input and reasoning-token accounting, and check the current status-line interface. Stopped after three rounds because these questions converged; no broad product or pricing survey was needed.

## Verified rates

USD per million tokens, Standard short-context pricing. The previously supported model rates still match the official table. The three additional models are:

| Model | Input | Cached input | Output |
|---|---:|---:|---:|
| `gpt-6.1-sol` | 2.00 | 0.10 | 10.00 |
| `gpt-6-sol` | 2.00 | 0.20 | 10.00 |
| `gpt-6-luna` | 0.10 | 0.01 | 0.50 |

Rates are established by the [official API pricing table](https://developers.openai.com/api/docs/pricing), fetched in full. Confidence: authoritative single-source rate card, with corroborating search results in the API changelog; the changelog itself was not fetched.

The [prompt caching guide](https://developers.openai.com/api/docs/guides/prompt-caching) explains separate cached-input pricing and cache-write pricing. The [reasoning guide](https://developers.openai.com/api/docs/guides/reasoning) includes reasoning within output-token usage. Subtract cached tokens from total input before applying the ordinary input rate; do not add reasoning tokens a second time. The local estimate continues to exclude cache-write premiums because the thread counters do not separate them.

Independent Python Decimal calculation for input 100000, cached input 80000, and output 10000: GPT-6.1 Sol USD 0.148, GPT-6 Sol USD 0.156, GPT-6 Luna USD 0.0078. The footer rounds to cents using the existing formatter.

The [developer commands guide](https://learn.chatgpt.com/docs/developer-commands#configure-footer-items-with-statusline) confirms that `/statusline` persists footer selections in `tui.status_line`. Inspection of the fetched 0.159.3 source found no API pricing fields in `codex-rs/models-manager/src` or `codex-rs/protocol/src/models.rs`; this is a local-source finding, not a claim about every Codex interface.

## Research Log

| Round | Queries actually run | Tool | New sources fetched | What it added |
|---|---|---|---|---|
| 1 | `gpt-6.1-sol pricing input cached output` | OpenAI Docs search, then fetch | API pricing | Current and existing model rates. |
| 2 | `prompt caching cached_tokens input_tokens reasoning output tokens billing` | OpenAI Docs search, then parallel fetch | Prompt caching; reasoning | Cached-input treatment, cache-write limitation, and reasoning included in output usage. |
| 3 | `Codex tui status_line estimated-thread-cost Enterprise`; `tui.status_line` | OpenAI Docs search, then fetch | Developer commands, statusline section | Footer configuration interface; first query did not find a directly relevant status-line reference. |

Totals: 3 rounds, 4 search queries, 4 unique URLs fetched, 3 complete pages read, 1 relevant page section read, 4 sources cited. Dead end: the first round-3 query returned a model rollout changelog and glossary rather than the needed status-line details; neither was fetched or used as evidence. Local model-metadata inspection and Python calculations are separate from web-source totals.

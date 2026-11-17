# Local Codex cost status line

This fork adds `local-token-cost` to the upstream terminal footer. The current
port targets upstream `rust-v0.169.0`; older builds are retained for rollback.
The installed Windows package is `0.169.0+local-cost.2`. Both `codex` and
`codex-cost` launch it through `.local/codex-cost-current`; existing sessions
keep their previously loaded executable until closed.
Source branches live in `A-Town-corp/codex`; `origin` is official upstream and
`atown` is the fork.

The estimate values cumulative thread tokens at the selected model's Standard
short-context API rate. Cached input is discounted, reasoning tokens are already
included in output, and switching models revalues the total. This is an
API-equivalent estimate, not a subscription bill. Tools, cache writes, Fast mode,
long-context premiums, and regional premiums are excluded. Unknown models show
`API est unavailable`.

Use the pinned Rust toolchain in `codex-rs/rust-toolchain.toml` and dependencies
in `codex-rs/Cargo.lock`. From the repository root:

```powershell
just test -p codex-tui --lib -E 'test(local_token_cost)'
just test -p codex-tui --lib -E 'test(local_token_cost) | test(status_line) | test(status_surface_preview)'
```

See [UPDATES.md](UPDATES.md) for update architecture and
[BACKLOG.md](BACKLOG.md) for current verification status.

The first 0.169.0 source run passed all 7 focused tests. The official release's
live terminal check fails with `CLI rejected local-token-cost as an invalid
footer item`, proving the missing feature independently of the source tests.
The broader footer selection passed all 117 selected tests; 5,932 tests were
outside that selection.
`cargo fmt --all --check`, `cargo check --locked -p codex-tui --all-targets`,
and `cargo clippy --locked -p codex-tui --all-targets -- -D warnings` passed.
The release build and all 3 native launcher integration tests passed. Live PTY
checks rendered `API est ~$0.00` in the packaged executable with both an isolated
profile and the saved account, then through the installed normal launcher.
These checks sent no model prompt and left saved config unchanged. Nonzero cost
and model changes are covered by source tests.

Model rates were checked against the
[official Standard pricing table](https://developers.openai.com/api/docs/pricing)
on 2026-11-16. Existing rates remain valid; GPT-6.1 Terra adds input $1.00,
cached input $0.05, and output $5.00 per million tokens.
Using the same Standard short-context table on 2026-11-17, the GPT-6.2
Astra rates are $8.00 input, $0.40 cached input, and $40.00 output per million
tokens. GPT-6.2 Sol uses $2.00, $0.10, and $10.00 respectively.

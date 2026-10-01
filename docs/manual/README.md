# Local Codex token-cost footer

This checkout ports the existing local API-equivalent cost footer to upstream tag `rust-v0.159.3` (`01fc69f`). The original 0.154.0 checkout and its unrelated edits remain at `C:/Users/Fonem/Dev/codex-cost`.

Installed package: `C:/Users/Fonem/.local/codex-cost-0.159.3`, metadata version `0.159.3+local-cost.1`. Both normal `codex` and `codex-cost` launch it. Reopen the CLI after installation. Upstream upgrades can replace the launcher or require a new port; see the installation and rollback notes in [REBUILD.md](REBUILD.md).

The footer uses cumulative thread token counts and the currently selected model's Standard short-context API rate. Changing models revalues the total. It excludes tool fees, cache-write premiums, speed premiums, and long-context/regional premiums; it is not a subscription invoice. Unknown models show `API est unavailable`.

- [Rebuild and installation history](REBUILD.md)
- [Pricing verification and research log](PRICING.md)
- [Architecture](ARCHITECTURE.md)
- [Known constraints and earlier test failures](GOTCHAS.md)
- [Task board](BACKLOG.md)

Use the pinned toolchain in `codex-rs/rust-toolchain.toml` (Rust 1.95.0) and dependencies in `codex-rs/Cargo.lock`. From the repository root, `just test -p codex-tui --lib -E 'test(local_token_cost)'` exercises the local footer. Build the CLI with `cargo build --locked -p codex-cli --bin codex` from `codex-rs`. The broader footer selection passed all 109 tests; the installed CLI rendered `API est ~$0.00` without the invalid-item warning.

The release tag's lockfile has local package versions of `0.0.0`. `cargo fetch` normalized 159 workspace package versions to `0.159.3`; a parsed comparison verified that no third-party package versions or other fields changed.

# Rebuild record

## Source

Windows PowerShell, repository root `C:/Users/Fonem/Dev/codex-cost`.

```powershell
git clone --depth 1 --branch rust-v0.154.0 https://github.com/openai/codex.git C:/Users/Fonem/Dev/codex-cost
Set-Location C:/Users/Fonem/Dev/codex-cost
git switch -c local-cost-statusline
```

Base commit: `6b9826e3aa83b1a5947db50f4332cb9c65f1b340`.

## Tools installed during this task

| Tool | Version |
|---|---|
| Rust selected by `codex-rs/rust-toolchain.toml` | 1.95.0 |
| just | 1.58.0 |
| cargo-nextest | 0.9.144 |
| cargo-insta | 1.48.0 |
| dotslash | 0.5.7 |
| npm package `@bazel/bazelisk` | 1.28.1 |
| Bazel selected by `.bazelversion` | 9.0.0 |

Installation commands used:

```powershell
cargo +stable install --locked just cargo-nextest cargo-insta
cargo +stable install --locked dotslash
npm install --global @bazel/bazelisk@1.28.1
```

The version table pins the actual resolved tools; the install commands above reflect the executed session. To reproduce those exact tools, use `cargo install <name> --version <version> --locked` for each table entry.

## Dependency setup

From `codex-rs`, `cargo fetch --offline` regenerated only 150 local-package versions from `0.0.0` to `0.154.0`, then stopped at uncached `valuable v0.1.1`. `cargo fetch --locked` with network access then completed successfully. See `GOTCHAS.md`.

## Tests

From repository root:

```powershell
just test -p codex-tui --lib -E 'test(local_token_cost)'
```

Initial run: 2 failures because the field was missing. After implementation: `2 tests run: 2 passed, 4273 skipped`. The skipped tests are outside the explicit name filter. `just test -p codex-tui` is the subsequent full crate check.

## Packaging

Use `just assemble-codex-package`, which sets `CODEX_REPO_ROOT`. Calling `python scripts/build_codex_package.py` directly without that variable fails with `RuntimeError: CODEX_REPO_ROOT must point to the repository root`.

The packager accepts `--entrypoint-bin`, `--code-mode-host-bin`, `--codex-command-runner-bin`, `--codex-windows-sandbox-setup-bin`, `--rg-bin`, and `--package-dir`. It validates the package layout. Never use `--force` against the installed package; assemble to a new directory.

## Installed result, 2026-09-13

`cargo build --locked -p codex-cli --bin codex` from `codex-rs` completed successfully. `just fmt` completed afterward; only Rust formatting changed the feature source. The installed binary implements commit `a60cdcd`.

Package root: `C:/Users/Fonem/.local/codex-cost`, package version `0.154.0+local-cost.1`. The `--entrypoint-bin` was `codex-rs/target/debug/codex.exe`; the other binaries were copied from `C:/Users/Fonem/.codex/packages/standalone/current` using the matching package-relative paths. Target: `x86_64-pc-windows-msvc`.

Launcher: `C:/Users/Fonem/AppData/Roaming/npm/codex-cost.cmd`. Run `codex-cost` immediately, or `codex` after opening a terminal with the refreshed user PATH. The custom package's `bin` directory is prepended to user PATH. Existing parent processes can retain the old PATH.

`C:/Users/Fonem/.codex/config.toml` has `local-token-cost` as its first `tui.status_line` item. Original configuration backup: `C:/Users/Fonem/.codex/config.toml.bak-local-cost`. Original user PATH backup: `C:/Users/Fonem/.codex/user-path.bak-local-cost.txt`. To roll back, restore those two values and run the official executable under `AppData/Local/Programs/OpenAI/Codex/bin`; no package deletion is required.

Verification: `codex-cost --version` returned `codex-cli 0.154.0`; `Get-Command codex` with the new PATH resolved the custom package. A live `codex-cost` session through cmd.exe displayed `API est ~$0.00 · gpt-6-astra medium · ~ · Ready`. The two focused tests passed, including input 100000, cached 80000, output 10000 yielding `API est ~$0.78`. Full-suite limitations are in `GOTCHAS.md`.

Rates are fixed Standard short-context API prices, applied to the current model and cumulative thread tokens. Switching model revalues the total. Tool fees and premium pricing are excluded; this is not a subscription bill. Unknown models display `API est unavailable`. Official updates require rebasing/rebuilding this custom patch.

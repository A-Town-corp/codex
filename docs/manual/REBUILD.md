# Rebuild record

## Port to 0.159.3, 2026-10-01

The current port is in `C:/Users/Fonem/Dev/codex-cost-0.159.3`, branch `local-cost-0.159.3`, based on tag `rust-v0.159.3` (`01fc69f`). The 0.154.0 checkout and its unrelated edits remain intact. The release tag was fetched with `git fetch --depth 1 origin tag rust-v0.159.3`; the port uses an isolated Git worktree.

From the new repository root, commands use the existing build cache:

```powershell
$env:CARGO_TARGET_DIR = 'C:\Users\Fonem\Dev\codex-cost\codex-rs\target'
just test -p codex-tui --lib -E 'test(local_token_cost)'
```

Before implementation: `2 tests run: 0 passed, 2 failed, 5507 skipped`, reproducing missing footer values. After the original patch, with a new GPT-6 model test: `3 tests run: 2 passed, 1 failed, 5507 skipped`; the missing GPT-6.1 Sol rate returned `API est unavailable` instead of `API est ~$0.15`. The excluded tests are outside the explicit name filter. [Pricing verification](PRICING.md) supplies the new-model rates.

Final check from the new repository root:

```powershell
$env:CARGO_TARGET_DIR = 'C:\Users\Fonem\Dev\codex-cost\codex-rs\target'
just test -p codex-tui --lib -E 'test(local_token_cost) | test(status_line) | test(status_surface_preview)'
```

Observed: `109 tests run: 109 passed, 5401 skipped`; the skips are outside the explicit selection. The GPT-6 Luna test initially expected cents, but the existing formatter correctly produced `API est ~$0.0078`. The user explicitly approved correcting that test to preserve existing precision. No failing tests were disabled.

From `codex-rs`, `cargo build --locked -p codex-cli --bin codex` completed successfully (`Finished dev ... in 3m 23s`). This installation uses the dev-profile binary. `just fmt` completed; its unrelated Windows newline changes in four Bazel files were removed from this isolated worktree. The original checkout's unrelated edits were preserved.

Package assembly from the new repository root:

```powershell
just assemble-codex-package --package-version '0.159.3+local-cost.1' --package-dir 'C:\Users\Fonem\.local\codex-cost-0.159.3' --entrypoint-bin 'C:\Users\Fonem\Dev\codex-cost\codex-rs\target\debug\codex.exe' --code-mode-host-bin 'C:\Users\Fonem\.codex\packages\standalone\current\bin\codex-code-mode-host.exe' --codex-command-runner-bin 'C:\Users\Fonem\.codex\packages\standalone\current\codex-resources\codex-command-runner.exe' --codex-windows-sandbox-setup-bin 'C:\Users\Fonem\.codex\packages\standalone\current\codex-resources\codex-windows-sandbox-setup.exe' --rg-bin 'C:\Users\Fonem\.codex\packages\standalone\current\codex-path\rg.exe'
Copy-Item -LiteralPath 'C:\Users\Fonem\.codex\packages\standalone\current\codex-resources\voice' -Destination 'C:\Users\Fonem\.local\codex-cost-0.159.3\codex-resources\voice' -Recurse
```

Assembly validated the layout; helpers and voice resources came from the matching official 0.159.3 package.

### Installed launch paths and verification

The original junction `C:/Users/Fonem/AppData/Local/Programs/OpenAI/Codex/bin` was renamed to `bin.stock-0.159.3`. A new `bin` junction points to `C:/Users/Fonem/.local/codex-cost-0.159.3/bin`. The official `standalone/current` package was not changed. `C:/Users/Fonem/AppData/Roaming/npm/codex-cost.cmd` also targets the new package. `local-token-cost` is restored as the last item in `~/.codex/config.toml` under `tui.status_line`.

`Get-Command codex` resolved to `C:/Users/Fonem/AppData/Local/Programs/OpenAI/Codex/bin/codex.exe`; `codex --version` printed `codex-cli 0.159.3`; `codex features list` loaded the restored config successfully. A Python `tomllib` check confirmed the last footer item.

Live verification used the official `codex exec-server --listen stdio://` to start the installed normal CLI path with a real PTY, `--no-daemon --no-alt-screen`, and a temporary compact `tui.status_line=["model-with-reasoning","local-token-cost"]` override. MCP servers were disabled only for that smoke process. The driver answered terminal capability probes, read the rendered terminal, and terminated the smoke process without submitting a prompt. Observed: `PASS: live Codex 0.159.3 footer rendered API est ~$0.00 for gpt-6.1-sol; no invalid status-line warning.` The compact override was not saved; a narrow terminal can truncate the user's longer footer.

For rollback, preserve the new junction by renaming `bin` to an unused backup name, then rename `bin.stock-0.159.3` back to `bin`. Remove `local-token-cost` from the config when using an official executable. The old 0.154.0 custom package also remains at `C:/Users/Fonem/.local/codex-cost`. Future upstream upgrades may replace the launcher; re-port before switching to a newer official version.

### Plain-command defaults, 2026-10-01

`C:/Users/Fonem/.local/codex-launcher` is first on persisted user PATH. Its native `codex.exe` delegates to the existing installed Codex binary with `--no-daemon` before the original arguments. Source is `scripts/codex_no_daemon_launcher.rs`; it inherits console input/output, forwards the child exit code, and lets Codex handle Ctrl+C while the launcher waits. No dependency was added. No standalone Rust launcher precedent was found; process spawning follows the existing Rust `Command` pattern.

From this repository root, using installed Rust 1.98.1:

```powershell
rustfmt scripts/codex_no_daemon_launcher.rs
rustc --edition 2024 -C opt-level=2 scripts/codex_no_daemon_launcher.rs -o C:\Users\Fonem\.local\codex-launcher\codex.exe
```

The first `.cmd` wrapper failed a quoted TOML config argument (`invalid type: string "[model-with-reasoning,local-token-cost]", expected a sequence`). It was replaced by the native launcher and preserved as `codex.cmd.previous`. Afterward, with the launcher directory prepended to the verification process's PATH, `Get-Command codex` resolved to its native `codex.exe`, `codex --version` printed `codex-cli 0.159.3`, and `codex -c 'tui.status_line=["model-with-reasoning","local-token-cost"]' features list` loaded successfully. An inline Python subprocess check confirmed that an invalid argument returns Codex's exit code 2 through the launcher.

The user config now has `approvals_reviewer = 'auto_review'` alongside the existing `approval_policy = 'on-request'` and `sandbox_mode = 'workspace-write'`. A parsed `tomllib` assertion verified this exact triple. The installed source's `approve_for_me_configures_interactive_mode` test establishes that these are precisely the `--approve-for-me` settings: automatic review with workspace-write, rather than disabling the sandbox.

The parser-level launcher check invoked `codex.exe agents` through the launcher and asserted the exact diagnostic `--no-daemon cannot be used with codex agents`, confirming automatic flag forwarding without an interactive session. A live PTY check rendered `API est ~$0.00`, but `/status` verification timed out during initialization, including with hook states temporarily disabled. Live confirmation of the permission label remains unverified; config loading and the exact saved permission triple passed. No saved hook settings were changed. A process inventory found no remaining smoke-test Codex processes.

The native launcher injects `--no-daemon` into all invocations. Commands requiring a daemon, such as `codex agents` or remote connections, should be invoked through the direct installed binary. For rollback, remove only `C:/Users/Fonem/.local/codex-launcher` from user PATH and remove the added `approvals_reviewer` key; preserve the package and other PATH entries. A terminal restart loads the persisted PATH.

The remaining sections preserve the earlier 0.154.0 investigation record; their in-progress statements are historical.

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

Initial run is in progress; no passing result recorded yet.

## Packaging

Use `just assemble-codex-package`, which sets `CODEX_REPO_ROOT`. Calling `python scripts/build_codex_package.py` directly without that variable fails with `RuntimeError: CODEX_REPO_ROOT must point to the repository root`.

The packager accepts `--entrypoint-bin`, `--code-mode-host-bin`, `--codex-command-runner-bin`, `--codex-windows-sandbox-setup-bin`, `--rg-bin`, and `--package-dir`. It validates the package layout. Never use `--force` against the installed package; assemble to a new directory.

No custom executable has been installed yet.

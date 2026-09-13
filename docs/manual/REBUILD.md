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

Initial run is in progress; no passing result recorded yet.

## Packaging

Use `just assemble-codex-package`, which sets `CODEX_REPO_ROOT`. Calling `python scripts/build_codex_package.py` directly without that variable fails with `RuntimeError: CODEX_REPO_ROOT must point to the repository root`.

The packager accepts `--entrypoint-bin`, `--code-mode-host-bin`, `--codex-command-runner-bin`, `--codex-windows-sandbox-setup-bin`, `--rg-bin`, and `--package-dir`. It validates the package layout. Never use `--force` against the installed package; assemble to a new directory.

No custom executable has been installed yet.

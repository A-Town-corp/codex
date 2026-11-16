# Updating the custom distribution

## Why updates stopped

The 0.159.3 CLI was built with Cargo's dev profile. Its `codex update` command
explicitly refuses debug builds; the update notifier also requires release
compilation. A PATH launcher added `--no-daemon` and delegated to an
official-looking Windows install path whose `bin` junction actually pointed at
the fixed custom package. Updating the official standalone package could not
reliably change the executable launched by that junction.

Keep custom and official installations separate. Custom packages should use
release compilation, matching upstream helpers, and a stable custom launch path.
Retain versioned packages so a failed update leaves the working version intact.
The official updater installs upstream binaries, which do not understand
`local-token-cost`; it cannot preserve this patch by itself.

## Recommended future delivery

Maintain a small patch series against stable upstream tags. Restrict the product
patch to the calculator and the footer's picker, preview, styling, and renderer.
Keep packaging and updater tooling separate so they carry forward independently.

The recommended long-term path is a fork-owned Windows build workflow:

1. Check the official latest stable release; never follow `main` or an alpha
   automatically.
2. Prepare a fresh branch from that tag and apply the patch. If it does not
   apply cleanly, stop for a reviewed port.
3. Verify prices, run the focused footer tests and formatting/lint checks, and
   build a release CLI. Smoke-test the actual packaged executable in a PTY.
4. Bundle matching official helpers. Verify upstream archive digests and publish
   a versioned custom artifact with its SHA-256.
5. A local updater downloads the newest successful custom artifact, verifies and
   smoke-tests it, then switches the stable custom path. Keep the old package
   for rollback and leave existing sessions running.

This moves compilation off the PC. A scheduled check should open an update PR;
the installer should consume verified custom releases, never silently substitute
a stock binary. Until implemented and exercised, this is a proposal rather than
an available automatic update feature.

## Current repeatable build command

From PowerShell 7 at the repository root:

```powershell
./scripts/build-local-cost.ps1
```

To reuse the local workstation's existing Cargo cache:

```powershell
./scripts/build-local-cost.ps1 -TargetDirectory C:\Users\Fonem\Dev\codex-cost\codex-rs\target
```

The script runs the broader footer selection, Rust formatting checks, all TUI
target checks, Clippy with warnings denied, release compilation, and standalone
launcher tests. It produces `release/codex.exe` and `release/codex-launcher.exe`
under the selected target directory. Packaging and installation are separate;
running a build never switches the active CLI.

The default uses one compiler worker, release optimization level 1, no LTO,
16 codegen units, and no debug symbols to bound memory use. It retains release
behavior and restores process environment settings on exit. `-BuildJobs` can
select more workers when RAM permits. Use the documented detached launcher and
RAM preflight for a long local run. Required tools are the pinned Rust toolchain,
MSVC build tools, `just`, and Cargo Nextest.

The standalone launcher defaults to `--no-daemon`, preserves an explicit copy
without duplicating it, and respects the `--` delimiter for literal prompt
arguments. Its target is the stable `.local/codex-cost-current/bin/codex.exe`
path under the user profile. A future installer can switch that junction after
verifying a package; the launcher itself does not need rebuilding every release.

## Installed workstation layout and rollback

Installed and verified on 2026-11-16:

- `.local/codex-launcher/codex.exe` and its native `codex-cost.exe` alias are
  first on PATH. Both delegate to `.local/codex-cost-current/bin/codex.exe`.
- `.local/codex-cost-current` is a junction to `.local/codex-cost-0.169.0`.
  The package contains matching official 0.169.0 helpers, custom metadata version
  `0.169.0+local-cost.1`, and `local-cost-build.json` with source and hash provenance.
- The main CLI SHA-256 is
  `8d5c9053e95b86ad13e9de7aa228a09771122f7ade2127f5426fbe230a285bb7`.
- The official `AppData/Local/Programs/OpenAI/Codex/bin` junction again targets
  `.codex/packages/standalone/current/bin`. It is separate from the custom path;
  this work did not upgrade that stock package. The old custom junction is
  retained as `bin.custom-0.159.3.previous` in the same install directory.
- The previous native launcher is retained as
  `.local/codex-launcher/codex.pre-0.169.0.exe`. The npm command shim now delegates
  to the native alias; its old contents are backed up as `codex-cost.cmd.pre-0.169.0`.

Keep `.local/codex-cost-0.159.3` for rollback. To roll back, preserve the current
junction under a new name and create `codex-cost-current` pointing at that older
package, then check `codex --version` and its footer. Only rename the junction,
never recursively delete its target. The stable launcher can launch either
package, so rollback does not require replacing the launcher or stopping an
already running session. To update again, first stage and verify a new versioned
package, then switch this same junction. No automatic custom updater is installed.

The terminal smoke checks deliberately select a compact footer for the process.
The saved layout is preserved; if its last items are clipped in a narrow terminal,
use `/statusline` to put `local-token-cost` earlier or select fewer items.

## Terminal verification

`scripts/smoke_local_cost.py` uses the official exec-server to create a real PTY,
answer terminal capability probes, and inspect the footer without sending a
model prompt. By default it uses a temporary Codex home and a local custom
provider fixture that needs no credentials.
`--real-account` also exercises saved account/config startup. MCP servers are
disabled only for that smoke process; saved config and hooks are unchanged.
New or changed hooks are skipped without granting trust. The test also exercises
the session-only `/statusline` command after the composer finishes its paste
burst; this avoids relying on warnings hidden in the startup warning popup.
The exec-server request explicitly inherits its environment. Without this,
`envPolicy: null` supplies only the provided variables, and the incomplete
Windows environment caused account routing to fail inside the test terminal.

```powershell
python scripts/smoke_local_cost.py --codex <custom-codex.exe> --executor <official-codex.exe> --transcript <local-evidence.txt>
```

The matching official Windows package was verified against its release asset
SHA-256: `456db280d16f4393cd99a398f11b4a14a3ede68b6bd31610ce41fb10d6b65737`.

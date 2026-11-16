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

```powershell
python scripts/smoke_local_cost.py --codex <custom-codex.exe> --executor <official-codex.exe> --transcript <local-evidence.txt>
```

The matching official Windows package was verified against its release asset
SHA-256: `456db280d16f4393cd99a398f11b4a14a3ede68b6bd31610ce41fb10d6b65737`.

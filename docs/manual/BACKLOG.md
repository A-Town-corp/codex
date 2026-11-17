# Backlog

## Now

- No active work.

## Next

- Implement and exercise the fork build workflow and custom artifact updater
  described in [UPDATES.md](UPDATES.md).

## Icebox

- Consider an upstream extension point for custom footer providers to eliminate
  the need for source builds.

## Done

- 2026-11-17: Added Standard API cost estimates for GPT-6.2 Astra and Sol and
  installed `0.169.0+local-cost.2`. The 117 footer tests, release build, and
  3 launcher tests passed. Both models displayed the footer in isolated PTYs,
  and Sol displayed it through the installed launcher with the saved account.
- 2026-11-16: Installed `0.169.0+local-cost.1` behind the stable custom junction.
  Normal `codex` and `codex-cost` report 0.169.0. Live PTY checks passed for the
  packaged executable in isolated and real-account startup, and for the normal
  installed launcher. Saved config and running sessions were preserved. The
  broader footer selection passed all 117 tests; formatting, all-target TUI
  checks, Clippy, release compilation, and all 3 launcher tests passed.
- 2026-11-16: Fixed the launcher's duplicate `--no-daemon` injection. The installed
  old launcher reproduced CLI exit 2; the forwarding regression went from RED
  to GREEN and covers explicit flags and literal arguments after `--`.
- 2026-11-16: Ported the footer integration to 0.169.0. All 7 focused tests pass,
  covering all 13 supported model rates, cached-input discounts, negative and
  excessive counters, overflow, reasoning-token accounting, unknown models,
  sub-cent formatting, pending usage, and the visible footer snapshot. The
  broader checks and installation subsequently passed as recorded above.
- 2026-11-16: Proved the footer regression in both isolated and real-account
  terminals: official 0.169.0 rejects `local-token-cost`. The initial account
  routing error came from the test driver's stripped environment; explicit
  exec-server environment inheritance corrected the test without changing auth.
- 2026-11-16: Tested and installed the stable custom launcher path. Integration
  tests cover quoted TOML arguments, child exit codes, missing packages, invalid
  user-profile paths, and explicit/literal `--no-daemon` arguments. Restored the
  official install path to its stock package, separately from the custom CLI.
- 2026-11-16: Backed up `local-cost-0.159.3` (`95b022ec`) and
  `local-cost-statusline` (`a2b12595`) to the fork and verified both SHAs through a
  fresh remote query. Created an isolated 0.169.0 worktree and verified the
  matching official Windows helper archive's SHA-256.

# Backlog

## Now

- Port and verify `local-token-cost` on 0.169.0, install it, and exercise the
  resulting package through the normal launch command.

## Next

- Implement and exercise the fork build workflow and custom artifact updater
  described in [UPDATES.md](UPDATES.md).

## Icebox

- Consider an upstream extension point for custom footer providers to eliminate
  the need for source builds.

## Done

- 2026-11-16: Ported the footer integration to 0.169.0. All 7 focused tests pass,
  covering all 13 supported model rates, cached-input discounts, negative and
  excessive counters, overflow, reasoning-token accounting, unknown models,
  sub-cent formatting, pending usage, and the visible footer snapshot. The
  broader footer selection, lint checks, release build, and installation remain
  pending.
- 2026-11-16: Proved the footer regression in both isolated and real-account
  terminals: official 0.169.0 rejects `local-token-cost`. The initial account
  routing error came from the test driver's stripped environment; explicit
  exec-server environment inheritance corrected the test without changing auth.
- 2026-11-16: Tested and pushed the stable custom launcher path. Both integration
  tests pass, covering quoted TOML arguments, child exit codes, and missing
  packages. Installation is still pending the complete package checks.
- 2026-11-16: Backed up `local-cost-0.159.3` (`95b022ec`) and
  `local-cost-statusline` (`a2b12595`) to the fork and verified both SHAs through a
  fresh remote query. Created an isolated 0.169.0 worktree and verified the
  matching official Windows helper archive's SHA-256.

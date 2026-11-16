# Backlog

## Now

- Port and verify `local-token-cost` on 0.169.0, install it, and exercise the
  resulting package through the normal launch command.
- Investigate the official 0.169.0 real-account startup error: `account/read
  failed: workspace routing discovery failed (code -32603)`.

## Next

- Implement and exercise the fork build workflow and custom artifact updater
  described in [UPDATES.md](UPDATES.md).

## Icebox

- Consider an upstream extension point for custom footer providers to eliminate
  the need for source builds.

## Done

- 2026-11-16: Backed up `local-cost-0.159.3` (`95b022ec`) and
  `local-cost-statusline` (`a2b12595`) to the fork and verified both SHAs through a
  fresh remote query. Created an isolated 0.169.0 worktree and verified the
  matching official Windows helper archive's SHA-256.

# Constraints and observed failures

- `cargo fetch --locked` initially failed: `cannot update the lock file ... because --locked was passed to prevent this`. The tagged release has 150 local package entries at `0.0.0`, while its workspace version is `0.154.0`. Cargo changed only those 150 versions. No third-party versions changed.
- `cargo fetch --offline` failed on uncached `valuable v0.1.1`. The following network-enabled `cargo fetch --locked` completed with exit code 0.
- `cargo install --locked just cargo-nextest cargo-insta` competes with workspace fetch/build commands for Cargo's package-cache lock. Wait for the lock; do not restart the compiler.
- Upstream pins Rust `1.95.0`; this host also has `1.98.0`. The repository toolchain file selects `1.95.0`.
- Native footer changes require a new process. A running Codex process retains the executable it loaded.
- `just test -p codex-tui`: 4244 passed, 34 failed, 1 timed out, 10 skipped. Failures include release-version snapshots (`0.0.0` versus `0.154.0`), Windows home-path abbreviation, terminal color assumptions, and the degraded Windows sandbox approval timeout. No failing tests were weakened or snapshots accepted.
- `just fmt` succeeded but Buildifier escaped CRLF inside three Starlark multiline strings. Those three formatter-only changes were undone; Rust formatting was retained.
- PowerShell PTY creation failed with OS Error -1073283067. Selecting `C:/Windows/System32/cmd.exe` succeeded and rendered the footer.
- Live startup reported unauthenticated `higgsfield` and `runpod` MCP servers. These account issues were not changed by this task.

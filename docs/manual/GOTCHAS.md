# Constraints and observed failures

- `cargo fetch --locked` initially failed: `cannot update the lock file ... because --locked was passed to prevent this`. The tagged release has 150 local package entries at `0.0.0`, while its workspace version is `0.154.0`. Cargo changed only those 150 versions. No third-party versions changed.
- `cargo fetch --offline` failed on uncached `valuable v0.1.1`. The following network-enabled `cargo fetch --locked` completed with exit code 0.
- `cargo install --locked just cargo-nextest cargo-insta` competes with workspace fetch/build commands for Cargo's package-cache lock. Wait for the lock; do not restart the compiler.
- Upstream pins Rust `1.95.0`; this host also has `1.98.0`. The repository toolchain file selects `1.95.0`.
- Native footer changes require a new process. A running Codex process retains the executable it loaded.

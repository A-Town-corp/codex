//! Standalone tests: compile with `rustc --test` after building the fixture and launcher.

use std::path::PathBuf;
use std::process::Command;

fn test_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root = PathBuf::from(std::env::var_os("CODEX_LAUNCHER_TEST_ROOT").ok_or("test root missing")?)
        .join(format!("{name}-{}", std::process::id()));
    std::fs::create_dir_all(&root)?;
    Ok(root)
}

#[test]
fn stable_custom_target_preserves_quoted_arguments_and_exit_code()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("forwarding")?;
    let bin = root.join(".local/codex-cost-current/bin");
    std::fs::create_dir_all(&bin)?;
    std::fs::copy(
        std::env::var_os("CODEX_LAUNCHER_TEST_FIXTURE").ok_or("fixture missing")?,
        bin.join("codex.exe"),
    )?;
    let config = r#"tui.status_line=["model-with-reasoning","local-token-cost"]"#;
    let output = Command::new(
        std::env::var_os("CODEX_LAUNCHER_UNDER_TEST").ok_or("launcher missing")?,
    )
    .env("USERPROFILE", &root)
    .env("LOCALAPPDATA", root.join("appdata"))
    .args(["-c", config, "features", "list"])
    .output()?;
    assert_eq!(output.status.code(), Some(42), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(
        String::from_utf8(output.stdout)?.lines().collect::<Vec<_>>(),
        vec!["--no-daemon", "-c", config, "features", "list"]
    );
    Ok(())
}

#[test]
fn missing_custom_package_reports_failure_without_a_stock_fallback()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("missing")?;
    let output = Command::new(
        std::env::var_os("CODEX_LAUNCHER_UNDER_TEST").ok_or("launcher missing")?,
    )
    .env("USERPROFILE", &root)
    .env("LOCALAPPDATA", root.join("appdata"))
    .output()?;
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8(output.stderr)?.contains("codex launcher:"));
    Ok(())
}

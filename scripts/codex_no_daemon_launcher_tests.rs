//! Standalone tests: compile with `rustc --test` after building the fixture and launcher.

use std::path::PathBuf;
use std::process::Command;

fn test_root(name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let root =
        PathBuf::from(std::env::var_os("CODEX_LAUNCHER_TEST_ROOT").ok_or("test root missing")?)
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
    for (arguments, forwarded) in [
        (
            vec!["-c", config, "features", "list"],
            vec!["--no-daemon", "-c", config, "features", "list"],
        ),
        (
            vec!["--no-daemon", "--version"],
            vec!["--no-daemon", "--version"],
        ),
        (
            vec!["--", "--no-daemon"],
            vec!["--no-daemon", "--", "--no-daemon"],
        ),
    ] {
        let mut expected = vec!["-c", r#"mcp_servers.cua_repl={command="",enabled=false}"#];
        expected.extend(forwarded);
        let output =
            Command::new(std::env::var_os("CODEX_LAUNCHER_UNDER_TEST").ok_or("launcher missing")?)
                .env("USERPROFILE", &root)
                .env("LOCALAPPDATA", root.join("appdata"))
                .args(&arguments)
                .output()?;
        assert_eq!(
            output.status.code(),
            Some(42),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            String::from_utf8(output.stdout)?
                .lines()
                .collect::<Vec<_>>(),
            expected,
            "arguments: {arguments:?}"
        );
    }
    Ok(())
}

#[test]
fn missing_custom_package_reports_failure_without_a_stock_fallback()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("missing")?;
    let stock_bin = root.join("appdata/Programs/OpenAI/Codex/bin");
    std::fs::create_dir_all(&stock_bin)?;
    std::fs::copy(
        std::env::var_os("CODEX_LAUNCHER_TEST_FIXTURE").ok_or("fixture missing")?,
        stock_bin.join("codex.exe"),
    )?;
    let output =
        Command::new(std::env::var_os("CODEX_LAUNCHER_UNDER_TEST").ok_or("launcher missing")?)
            .env("USERPROFILE", &root)
            .env("LOCALAPPDATA", root.join("appdata"))
            .output()?;
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8(output.stderr)?.contains("codex launcher:"));
    Ok(())
}

#[test]
fn invalid_profile_cannot_select_an_executable_from_the_working_directory()
-> Result<(), Box<dyn std::error::Error>> {
    let root = test_root("invalid-profile")?;
    let bin = root.join(".local/codex-cost-current/bin");
    std::fs::create_dir_all(&bin)?;
    std::fs::copy(
        std::env::var_os("CODEX_LAUNCHER_TEST_FIXTURE").ok_or("fixture missing")?,
        bin.join("codex.exe"),
    )?;
    for profile in [".", ""] {
        let output =
            Command::new(std::env::var_os("CODEX_LAUNCHER_UNDER_TEST").ok_or("launcher missing")?)
                .current_dir(&root)
                .env("USERPROFILE", profile)
                .output()?;
        assert_eq!(output.status.code(), Some(1), "profile: {profile:?}");
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8(output.stderr)?.contains("USERPROFILE"));
    }
    Ok(())
}

//! User-local Windows launcher: preserve arguments and run Codex without its daemon.

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

#[link(name = "Kernel32")]
unsafe extern "system" {
    fn SetConsoleCtrlHandler(handler: Option<extern "system" fn(u32) -> i32>, add: i32) -> i32;
}

extern "system" fn handle_console_event(event: u32) -> i32 {
    // Codex handles these events itself. Keep the waiting launcher alive.
    i32::from(event == 0 || event == 1)
}

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    // SAFETY: the callback has the Windows handler ABI and static lifetime.
    // A new child process gets its own handler table, so Codex still sees Ctrl+C.
    if unsafe { SetConsoleCtrlHandler(Some(handle_console_event), 1) } == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let user_profile = env::var_os("USERPROFILE")
        .filter(|value| !value.is_empty())
        .ok_or("USERPROFILE is missing or empty")?;
    let user_profile = PathBuf::from(user_profile);
    if !user_profile.is_absolute() {
        return Err("USERPROFILE must be an absolute path".into());
    }
    let target = user_profile
        .join(".local")
        .join("codex-cost-current")
        .join("bin")
        .join("codex.exe");
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    let has_no_daemon = arguments
        .iter()
        .take_while(|argument| argument.as_os_str() != OsStr::new("--"))
        .any(|argument| argument.as_os_str() == OsStr::new("--no-daemon"));
    let mut command = Command::new(&target);
    if !has_no_daemon {
        command.arg("--no-daemon");
    }
    let status = command
        .args(arguments)
        .status()
        .map_err(|error| format!("failed to launch {}: {error}", target.display()))?;
    Ok(status.code().unwrap_or(1))
}

fn main() {
    let code = match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("codex launcher: {error}");
            1
        }
    };
    std::process::exit(code);
}

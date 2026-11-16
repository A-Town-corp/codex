//! User-local Windows launcher: preserve arguments and run Codex without its daemon.

use std::env;
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
    let local_app_data = env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA is missing")?;
    let target = PathBuf::from(local_app_data)
        .join("Programs")
        .join("OpenAI")
        .join("Codex")
        .join("bin")
        .join("codex.exe");
    let status = Command::new(target)
        .arg("--no-daemon")
        .args(env::args_os().skip(1))
        .status()?;
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

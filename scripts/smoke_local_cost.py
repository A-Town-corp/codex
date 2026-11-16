"""Verify the installed cost footer in a real terminal without sending a prompt."""

import argparse
import base64
import json
import queue
import re
import subprocess
import threading
import time
import tempfile
import tomllib
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--codex", type=Path, required=True)
    parser.add_argument("--executor", type=Path, required=True)
    parser.add_argument("--transcript", type=Path, required=True)
    parser.add_argument("--real-account", action="store_true")
    args = parser.parse_args()
    messages: queue.Queue[dict] = queue.Queue()
    server = subprocess.Popen(
        [str(args.executor), "exec-server", "--listen", "stdio://"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.DEVNULL,
        text=True,
        encoding="utf-8",
    )

    def read_messages() -> None:
        assert server.stdout is not None
        for line in server.stdout:
            try:
                messages.put(json.loads(line))
            except json.JSONDecodeError as error:
                messages.put({"readerError": str(error)})
        messages.put({"readerError": "exec-server closed its output"})

    threading.Thread(target=read_messages, daemon=True).start()
    request_id = 0

    def send(method: str, params: dict) -> int:
        nonlocal request_id
        request_id += 1
        assert server.stdin is not None
        server.stdin.write(
            json.dumps({"id": request_id, "method": method, "params": params}) + "\n"
        )
        server.stdin.flush()
        return request_id

    def request(method: str, params: dict) -> dict:
        sent_id = send(method, params)
        deadline = time.monotonic() + 15
        while time.monotonic() < deadline:
            message = messages.get(timeout=max(0.1, deadline - time.monotonic()))
            if "readerError" in message:
                raise RuntimeError(message["readerError"])
            if message.get("id") == sent_id:
                if "error" in message:
                    raise RuntimeError(message["error"])
                return message["result"]
        raise TimeoutError(f"No response to {method}")

    transcript = ""
    started = False
    skipped_hook_review = False
    submitted_statusline = False
    confirmed_statusline = False
    try:
        request("initialize", {"clientName": "local-cost-smoke"})
        assert server.stdin is not None
        server.stdin.write(json.dumps({"method": "initialized"}) + "\n")
        server.stdin.flush()
        command = [
            str(args.codex),
            "--no-daemon",
            "--no-alt-screen",
            "-c",
            'tui.status_line=["model-with-reasoning","local-token-cost"]',
        ]
        config_path = Path.home() / ".codex" / "config.toml"
        config = tomllib.loads(config_path.read_text(encoding="utf-8"))
        for name in config.get("mcp_servers", {}) if args.real_account else ():
            command.extend(["-c", f"mcp_servers.{name}.enabled=false"])
        environment = {"TERM": "xterm-256color"}
        if not args.real_account:
            smoke_home = Path(tempfile.mkdtemp(prefix="codex-cost-smoke-"))
            smoke_config = (
                'model = "gpt-6.1-sol"\n'
                'model_provider = "local-cost-smoke"\n'
                "check_for_update_on_startup = false\n"
                "[model_providers.local-cost-smoke]\n"
                'name = "Local footer smoke fixture"\n'
                'base_url = "http://127.0.0.1:9/v1"\n'
                'wire_api = "responses"\n'
                "requires_openai_auth = false\n"
                f'[projects.{json.dumps(str(Path.home()))}]\ntrust_level = "trusted"\n'
            )
            (smoke_home / "config.toml").write_text(smoke_config, encoding="utf-8")
            environment["CODEX_HOME"] = str(smoke_home)
        request(
            "process/start",
            {
                "processId": "local-cost-smoke",
                "argv": command,
                "cwd": Path.home().as_uri(),
                "env": environment,
                "tty": True,
            },
        )
        started = True
        deadline = time.monotonic() + 35
        while time.monotonic() < deadline:
            try:
                message = messages.get(
                    timeout=min(1, max(0.1, deadline - time.monotonic()))
                )
            except queue.Empty:
                if "Hooks need review" in transcript and not skipped_hook_review:
                    # Startup discards queued keys after drawing its first frame.
                    # Wait for a settled screen before choosing its safe skip.
                    send(
                        "process/write",
                        {
                            "processId": "local-cost-smoke",
                            "writeId": str(request_id + 1),
                            "chunk": base64.b64encode(b"\x1b").decode(),
                        },
                    )
                    skipped_hook_review = True
                elif "state=idle" in transcript and not submitted_statusline:
                    # Session-only command: no model request or saved config write.
                    command_text = b"/statusline model-with-reasoning local-token-cost"
                    send(
                        "process/write",
                        {
                            "processId": "local-cost-smoke",
                            "writeId": str(request_id + 1),
                            "chunk": base64.b64encode(command_text).decode(),
                        },
                    )
                    submitted_statusline = True
                elif submitted_statusline and not confirmed_statusline:
                    # Submit after the composer has finished its paste burst.
                    send(
                        "process/write",
                        {
                            "processId": "local-cost-smoke",
                            "writeId": str(request_id + 1),
                            "chunk": base64.b64encode(b"\r").decode(),
                        },
                    )
                    confirmed_statusline = True
                continue
            if "readerError" in message:
                raise RuntimeError(message["readerError"])
            if "error" in message:
                raise RuntimeError(message["error"])
            if message.get("method") != "process/output":
                continue
            chunk = base64.b64decode(message["params"]["chunk"]).decode(
                "utf-8", "replace"
            )
            transcript += chunk
            response = ""
            if "\x1b[6n" in chunk:
                response += "\x1b[1;1R"
            if re.search(r"\x1b\[(?:0)?c", chunk):
                response += "\x1b[?1;2c"
            if "\x1b[>c" in chunk or "\x1b[>0c" in chunk:
                response += "\x1b[>0;10;1c"
            if "\x1b]11;?" in chunk:
                response += "\x1b]11;rgb:0000/0000/0000\x1b\\"
            if response:
                send(
                    "process/write",
                    {
                        "processId": "local-cost-smoke",
                        "writeId": str(request_id + 1),
                        "chunk": base64.b64encode(response.encode()).decode(),
                    },
                )
            plain = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", transcript)
            if (
                "Ignored invalid status line" in plain
                or "Unknown status-line item" in plain
            ) and "local-token-cost" in plain:
                raise AssertionError(
                    "CLI rejected local-token-cost as an invalid footer item"
                )
            if "API est ~$0.00" in plain:
                print(
                    "PASS: live terminal rendered API est ~$0.00; no invalid-item warning"
                )
                return 0
        raise AssertionError(
            "Timed out before the live cost footer appeared; inspect transcript"
        )
    finally:
        args.transcript.parent.mkdir(parents=True, exist_ok=True)
        args.transcript.write_text(transcript, encoding="utf-8")
        if started:
            request("process/terminate", {"processId": "local-cost-smoke"})
        assert server.stdin is not None
        server.stdin.close()
        try:
            server.wait(timeout=10)
        except subprocess.TimeoutExpired:
            server.kill()
            server.wait(timeout=10)


if __name__ == "__main__":
    raise SystemExit(main())

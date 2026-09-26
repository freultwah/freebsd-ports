#!/usr/bin/env python3
"""Exercise a built FreeBSD Codex package without touching the user's CODEX_HOME.

Usage: python3 files/test-daemon.py /path/to/stage/usr/local/bin/codex
Run from the port directory; temporary homes are created under work/.
No model requests or remote-control registration are performed.
"""

import concurrent.futures
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time


def main():
    codex = str(Path(sys.argv[1]).absolute())
    work = Path("work").absolute()
    work.mkdir(exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="dt-", dir=work) as temporary:
        home = Path(temporary)
        env = dict(os.environ, CODEX_HOME=str(home), LC_ALL="C", TZ="UTC")
        # Inherited executor selection would change the behavior being tested.
        env.pop("CODEX_EXEC_SERVER_URL", None)

        def run(*args, timezone="UTC"):
            result = subprocess.run(
                [codex, "app-server", "daemon", *args],
                env=dict(env, TZ=timezone),
                text=True, capture_output=True, timeout=90,
            )
            if result.returncode:
                raise RuntimeError(f"{args}: {result.stderr}")
            return json.loads(result.stdout)

        try:
            first = run("start")
            assert first["status"] == "started", first
            pid = first["pid"]
            assert first["appServerVersion"] == first["managedCodexVersion"], first
            os.kill(pid, 0)
            print("PASS: detached startup and app-server handshake")

            with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
                results = list(pool.map(
                    lambda tz: run("start", timezone=tz),
                    ["UTC", "America/New_York", "Asia/Tokyo", "Pacific/Honolulu"],
                ))
            assert all(r["status"] == "alreadyRunning" and r["pid"] == pid for r in results), results
            assert run("version")["appServerVersion"] == first["appServerVersion"]
            print("PASS: concurrent clients reuse one daemon across timezones")

            update = run("update")
            assert update["status"] == "unsupported" and "pkg" in update["message"], update
            assert not (home / "app-server-daemon/daemon-updater.pid").exists()
            print("PASS: upstream updater is disabled")

            refreshed = run("update", "--from-cli", "--yes")
            assert refreshed["status"] == "updated", refreshed
            after_refresh = run("start")
            assert after_refresh["status"] == "alreadyRunning", after_refresh
            assert after_refresh["pid"] != pid, after_refresh
            print("PASS: refresh from the installed package restarts the daemon")

            restarted = run("restart", timezone="Asia/Tokyo")
            assert restarted["status"] == "restarted", restarted
            assert restarted["pid"] != after_refresh["pid"], restarted
            print("PASS: explicit restart")

            os.kill(restarted["pid"], signal.SIGKILL)
            time.sleep(1)
            recovered = run("start")
            assert recovered["status"] == "started", recovered
            assert recovered["pid"] != restarted["pid"], recovered
            print("PASS: recovery after a killed daemon and stale PID record")

            assert run("stop", timezone="America/New_York")["status"] == "stopped"
            assert run("stop")["status"] == "notRunning"
            print("PASS: shutdown and repeated shutdown")
        finally:
            # Stop only the server belonging to this isolated home, even on failure.
            run("stop")


if __name__ == "__main__":
    main()

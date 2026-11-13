#!/usr/bin/env python3
"""Offline policy and failure-handling checks; no ports builds or network."""
import importlib.util
from contextlib import redirect_stdout
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
from urllib.error import HTTPError

spec = importlib.util.spec_from_file_location("checker", Path(__file__).with_name("check-upstream-updates.py"))
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)


class UpstreamChecks(unittest.TestCase):
    def test_codex_tag_family(self):
        config = {"pattern": r"rust-v(\d+\.\d+\.\d+)"}
        self.assertEqual(checker.version_from_tag("rust-v0.170.0", config), "0.170.0")
        for tag in ("v0.170.0", "rust-v0.170.0-alpha.1", "rust-v0.170.0-extra"):
            self.assertIsNone(checker.version_from_tag(tag, config))

    def test_pypi_excludes_yanked_empty_and_prerelease(self):
        data = {"releases": {"1.0": [{"yanked": False}], "2.0": [{"yanked": True}],
                             "3.0": [], "4.0rc1": [{"yanked": False}]}}
        with patch.object(checker, "fetch", return_value=data):
            self.assertEqual([v for v, _ in checker.candidates({"source": "pypi", "project": "demo"})], ["1.0"])

    def test_github_errors_do_not_fall_back(self):
        with patch.object(checker, "fetch", side_effect=HTTPError("url", 403, "rate limit", {}, None)), \
             patch.object(checker, "git_candidates") as git:
            with self.assertRaises(HTTPError):
                checker.candidates({"source": "github", "project": "a/b"})
            git.assert_not_called()

    def test_no_release_falls_back_to_tags(self):
        with patch.object(checker, "fetch", side_effect=HTTPError("url", 404, "missing", {}, None)), \
             patch.object(checker, "git_candidates", return_value=[("1.2", "link")]):
            self.assertEqual(checker.candidates({"source": "github", "project": "a/b"}), [("1.2", "link")])

    def test_holds_do_not_query_metadata_or_network(self):
        with patch.object(checker, "command") as command, patch.object(checker, "fetch") as fetch:
            self.assertEqual(checker.check("a/b", {"ignore": "Deliberate hold"}, Path("."), Path(".")),
                             ("SKIP", "Deliberate hold"))
            command.assert_not_called()
            fetch.assert_not_called()

    def test_comparison_and_empty_results(self):
        for versions, expected in [([("1.9", "old"), ("1.10", "new")], "UPDATE"),
                                   ([("1.8", "link")], "AHEAD"),
                                   ([("1.9", "link")], "CURRENT"), ([], "ERROR")]:
            with self.subTest(versions=versions), patch.object(checker, "command", return_value="1.9"), \
                 patch.object(checker, "candidates", return_value=versions):
                # Use the actual FreeBSD comparator: lexical ordering would fail 1.9 vs 1.10.
                with patch.object(checker, "compare", side_effect=lambda a, b: self.compare_pkg(a, b)):
                    self.assertEqual(checker.check("a/b", {}, Path("."), Path("."))[0], expected)

    @staticmethod
    def compare_pkg(a, b):
        import subprocess
        return subprocess.check_output(["pkg", "version", "-t", a, b], text=True).strip()

    def test_source_failure_is_error(self):
        with patch.object(checker, "command", return_value="1.0"), \
             patch.object(checker, "candidates", side_effect=OSError("unreachable")):
            self.assertEqual(checker.check("a/b", {}, Path("."), Path(".")), ("ERROR", "unreachable"))

    def test_rust_compares_main_ports_without_upstream(self):
        for main_version, expected in (("1.99.0", "CURRENT"), ("1.100.0", "UPDATE"),
                                       ("1.98.0", "AHEAD")):
            with self.subTest(main_version=main_version), \
                 patch.object(checker, "command", side_effect=["1.99.0", main_version]) as command, \
                 patch.object(checker, "candidates") as upstream, \
                 patch.object(checker, "compare", side_effect=self.compare_pkg):
                status, detail = checker.check("lang/rust", {"source": "ports"},
                                               Path("/overlay"), Path("/main-ports"))
                self.assertEqual(status, expected)
                self.assertIn("FreeBSD ports", detail)
                self.assertEqual(command.call_args_list[1].args[0],
                                 ["make", "-C", "/main-ports/lang/rust", "PORTSDIR=/main-ports",
                                  "-V", "${PORTVERSION}"])
                upstream.assert_not_called()

    def test_report_exit_status(self):
        with tempfile.TemporaryDirectory() as directory:
            overlay = Path(directory)
            (overlay / "a/b").mkdir(parents=True)
            (overlay / "a/b/Makefile").touch()
            config = overlay / "checks.json"
            config.write_text('{}')
            for status, code in (("UPDATE", 0), ("SKIP", 0), ("ERROR", 1)):
                with self.subTest(status=status), \
                     patch("sys.argv", ["checker", "--overlay", directory, "--config", str(config)]), \
                     patch.object(checker, "check", return_value=(status, "detail")), \
                     redirect_stdout(io.StringIO()):
                    self.assertEqual(checker.main(), code)


if __name__ == "__main__":
    unittest.main()

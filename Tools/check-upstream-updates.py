#!/usr/bin/env python3
"""Report upstream releases for overlay ports. Never update ports or build them."""

import argparse
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from urllib.error import HTTPError
from urllib.request import Request, urlopen


class CheckError(Exception):
    pass


def command(args, timeout=30):
    result = subprocess.run(args, capture_output=True, text=True, timeout=timeout)
    if result.returncode:
        raise CheckError(result.stderr.strip() or f"{args[0]} exited {result.returncode}")
    return result.stdout.strip()


def fetch(url, as_json=True):
    headers = {"User-Agent": "local-ports-upstream-check/1.0"}
    # Never send the optional GitHub credential to other upstreams.
    if url.startswith("https://api.github.com/") and os.environ.get("GITHUB_TOKEN"):
        headers["Authorization"] = "Bearer " + os.environ["GITHUB_TOKEN"]
    try:
        with urlopen(Request(url, headers=headers), timeout=20) as response:
            data = response.read().decode("utf-8")
    except HTTPError as exc:
        exc.close()
        raise
    return json.loads(data) if as_json else data


def version_from_tag(tag, config):
    match = re.fullmatch(config.get("pattern", r"v?(\d+(?:\.\d+)+)"), tag)
    return match.group(1) if match else None


def git_candidates(url, config):
    refs = command(["git", "-c", "credential.helper=", "ls-remote", "--tags", "--refs", url], 45)
    result = []
    for line in refs.splitlines():
        tag = line.split("\trefs/tags/", 1)[-1]
        version = version_from_tag(tag, config)
        if version:
            result.append((version, url.removesuffix(".git")))
    return result


def candidates(config):
    source = config["source"]
    if source == "github":
        project = config["project"]
        url = f"https://api.github.com/repos/{project}/releases/latest"
        try:
            release = fetch(url)
        except HTTPError as exc:
            exc.close()
            if exc.code != 404:
                raise
        else:
            version = version_from_tag(release["tag_name"], config)
            if version and not release["draft"] and not release["prerelease"]:
                return [(version, release["html_url"])]
        # Projects without GitHub Releases (or with a different release family).
        return git_candidates(f"https://github.com/{project}.git", config)
    if source == "git":
        return git_candidates(config["url"], config)
    if source == "pypi":
        data = fetch(f"https://pypi.org/pypi/{config['project']}/json")
        return [(v, f"https://pypi.org/project/{config['project']}/{v}/")
                for v, files in data["releases"].items()
                if re.fullmatch(r"\d+(?:\.\d+)+", v)
                and any(not item.get("yanked", False) for item in files)]
    if source == "text":
        return [(m.group(1), config["url"])
                for m in re.finditer(config["pattern"], fetch(config["url"], False))]
    if source == "wordpress":
        data = fetch("https://api.wordpress.org/core/version-check/1.7/")
        return [(offer["version"], "https://wordpress.org/download/releases/")
                for offer in data["offers"] if offer["response"] == "upgrade"]
    if source == "plex":
        data = fetch("https://plex.tv/api/downloads/5.json")
        server = data["computer"]["FreeBSD"]
        if not server["releases"]:
            raise CheckError("No FreeBSD release assets published")
        return [(server["version"].split("-", 1)[0], server["releases"][0]["url"])]
    raise CheckError(f"Unknown source: {source}")


def compare(left, right):
    result = command(["pkg", "version", "-t", left, right])
    if result not in ("<", "=", ">"):
        raise CheckError(f"Unexpected pkg version result: {result!r}")
    return result


def port_version(directory, ports):
    # Metadata only: no make target, dependency installation or compilation.
    version = command(["make", "-C", str(directory), f"PORTSDIR={ports}",
                       "-V", "${PORTVERSION}"])
    if not version or "\n" in version or "$" in version:
        raise CheckError(f"Could not resolve PORTVERSION in {directory}: {version!r}")
    return version


def check(origin, config, overlay, ports):
    if config is None:
        return "SKIP", "No upstream configured; add this port to Tools/upstream-checks.json"
    if config.get("ignore"):
        return "SKIP", config["ignore"]
    try:
        local = port_version(overlay / origin, ports)
        from_ports = config.get("source") == "ports"
        if from_ports:
            reference = ports / origin
            versions = [(port_version(reference, ports), str(reference))]
        else:
            versions = candidates(config)
        # Fail closed if an upstream changes its version syntax or response shape.
        versions = [(v, url) for v, url in versions
                    if re.fullmatch(r"\d+(?:\.\d+)+(?:[ab]\d+|rc\d+)?", v)]
        if not versions:
            raise CheckError("No matching releases found; check the source and version pattern")
        latest, link = versions[0]
        for version, url in dict(versions).items():
            if compare(latest, version) == "<":
                latest, link = version, url
        relation = compare(local, latest)
        source_label = "FreeBSD ports" if from_ports else "upstream"
        if relation == "<":
            suffix = " (FreeBSD ports)" if from_ports else ""
            return "UPDATE", f"{local} -> {latest}{suffix}\n    {link}"
        if relation == ">":
            return "AHEAD", f"local {local}; {source_label} {latest}"
        return "CURRENT", f"{local} (FreeBSD ports)" if from_ports else local
    except (CheckError, OSError, ValueError, KeyError, TypeError, re.error,
            subprocess.TimeoutExpired) as exc:
        return "ERROR", str(exc).replace("\n", " ")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("origins", nargs="*", help="Only check these category/port origins")
    parser.add_argument("--overlay", type=Path, default=Path(__file__).resolve().parent.parent)
    parser.add_argument("--ports", type=Path, default=Path("/usr/ports"))
    parser.add_argument("--config", type=Path, help="Alternative JSON configuration")
    parser.add_argument("--all", action="store_true", help="Also print current and ahead ports")
    args = parser.parse_args()
    try:
        config = json.loads((args.config or args.overlay / "Tools/upstream-checks.json").read_text())
        available = {str(p.parent.relative_to(args.overlay))
                     for p in args.overlay.glob("*/*/Makefile")}
        origins = sorted(set(args.origins) if args.origins else available)
        unknown = set(origins) - available
        if unknown:
            raise CheckError("Unknown overlay origins: " + ", ".join(sorted(unknown)))
        if not isinstance(config, dict) or any(not isinstance(c, dict) for c in config.values()):
            raise CheckError("Configuration must map origins to objects")
        counts = Counter()
        with ThreadPoolExecutor(max_workers=4) as pool:
            results = pool.map(lambda origin: check(origin, config.get(origin), args.overlay, args.ports), origins)
            for origin, (status, detail) in zip(origins, results):
                counts[status] += 1
                if args.all or status not in ("CURRENT", "AHEAD"):
                    print(f"{status:7} {origin}: {detail}", flush=True)
        print("Upstream check: " + ", ".join(f"{counts[s]} {s.lower()}" for s in
              ("UPDATE", "CURRENT", "AHEAD", "SKIP", "ERROR")))
        return 1 if counts["ERROR"] else 0
    except (CheckError, OSError, ValueError) as exc:
        print(f"Upstream check failed: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())

# Upstream release checks

`poudriere-update` runs this report after refreshing the trees and derived ports.
It never changes a port, downloads release archives, installs dependencies, or
starts a build. An available update is information for a manual decision.
Network/API errors print `ERROR` and a warning without failing the completed
refresh. Available updates return success.

Run it independently, including when the overlay has uncommitted work:

```sh
python3 ~/ports/LOCAL/Tools/check-upstream-updates.py
python3 ~/ports/LOCAL/Tools/check-upstream-updates.py --all
python3 ~/ports/LOCAL/Tools/check-upstream-updates.py misc/codex mail/bulwark --all
```

The default report prints updates, skips, and errors, plus totals. `--all` also
prints current ports and ports whose local versions are ahead of the upstream
source. It reads resolved `PORTVERSION` using `make -V` and compares versions
using `pkg version -t`; it does not compare package revisions or epochs.
Requirements: Python 3.9+, FreeBSD make, pkg, Git, and outbound HTTPS. Requests
have timeouts, with at most four ports checked concurrently. There is no cache
and no state written into the overlay.

## Holding a port

Leaving a port at its current version is always fine. If you also want to stop
reminders, add an `ignore` reason to its entry in `upstream-checks.json`:

```json
"mail/bulwark": {
  "source": "github",
  "project": "bulwarkmail/webmail",
  "pattern": "v?(\\d+(?:\\.\\d+)+)",
  "ignore": "Holding this version until the local integration is ready."
}
```

The report prints the reason as `SKIP` without querying make or the network.
Remove `ignore` to resume checking. No holds have been guessed for release
ports. The five explicitly commit-pinned snapshot ports start with an ignore
reason because comparing their advertised versions with release tags would be
misleading. They can be given a release source if that policy changes.

Use `--config /path/to/checks.json` for an alternative complete configuration.
The Bash hook uses the repository configuration. Newly added overlay ports are
discovered automatically and reported as `SKIP` until configured.

## Sources and version policy

Each configured origin has one of these sources:

- `github`: `project` is `owner/repository`. Check GitHub's latest stable
  published release; if absent or outside the configured tag family, fall back
  to Git tags. HTTP errors other than 404 are errors, including rate limits.
- `git`: `url` is a public Git repository. Read all tag refs with `git ls-remote`;
  no checkout or source download is needed.
- `pypi`: `project` is the PyPI name. Check numeric stable versions with at
  least one non-yanked file.
- `text`: read `url`; extract versions with the regular expression `pattern`.
  Used for upstream release indexes.
- `ports`: compare with the same origin in `/usr/ports` (or `--ports`). Rust
  uses this policy: only a newer version available in FreeBSD ports main is
  reported, without querying Rust upstream. `poudriere-update` refreshes that
  tree before running the check; standalone checks use its current checkout.
- `wordpress` and `plex`: use their download APIs. Plex checks the public
  FreeBSD channel, not Plex Pass releases.

For GitHub/Git, `pattern` must match the entire tag and capture the version in
group 1. For text, group 1 captures a version anywhere in the response. Default
Git tag matching accepts numeric stable releases with an optional `v` prefix.
Codex explicitly matches `rust-v` tags. BIND 9.20, Python 3.15, Dovecot 2.3, and
the two Samba series are restricted in their source URL or pattern. Python uses
actual stable release tags, because FTP directories can also contain only
prerelease artifacts. Other release ports check the latest stable release,
including a newer major version. Edit patterns to narrow that policy.

This checks published upstream versions, not FreeBSD compatibility or whether
the local patches still apply. Git tag fallback cannot inspect a GitHub
prerelease flag, so its numeric-only tag pattern is the stability filter.
If no versions match, the checker reports an error rather than claiming the
port is current. No newer-commit checks are performed for snapshots.

GitHub's unauthenticated API limit may be reached after repeated runs. An
optional `GITHUB_TOKEN` environment variable is used only for api.github.com.
GitHub release semantics and PyPI metadata are documented at:

- https://docs.github.com/en/rest/releases/releases#get-the-latest-release
- https://docs.pypi.org/api/json/

Lightweight offline tests (no ports builds):

```sh
python3 -B ~/ports/LOCAL/Tools/test-upstream-updates.py
```

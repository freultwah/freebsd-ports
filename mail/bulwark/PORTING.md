# Bulwark Webmail port assessment

Assessed upstream release **1.21.1**, tag `1.21.1`, commit `ba3b9ea`,
on 12 November 2026. There was no existing `mail/bulwark` in `/usr/ports`
or local port content to preserve.

## Selected approach: Lite

The port packages the upstream `bulwark-lite-1.21.1.zip` static export as
`bulwark-lite`. It needs an existing web server and a JMAP server; neither
Node.js nor a local Stalwart installation is a package dependency. Files
are installed under `${PREFIX}/www/bulwark-lite` and must be served at the
root of a virtual host. The release is built for `/`, so changing to a
subpath requires rebuilding the upstream client.

`config.json` and `policy.json` use `@sample` to preserve administrator
changes. The nginx and Caddy examples are installed separately under
`${PREFIX}/share/examples/bulwark`, with FreeBSD installation paths. They
need review for the deployment's hostname and HTTPS configuration.
Cross-origin deployments need the mail server to allow the webmail
origin, including authorization headers and authentication endpoints.

The matching upstream project source archive and attribution notice are
installed under `${PREFIX}/share/doc/bulwark`. The source archive also
provides the exact release license file; the Lite archive has none.

Lite supplies mail, calendar, contacts, files and password/TOTP login. It
omits the full server's admin console, plugins (including S/MIME), settings
sync, OAuth/OIDC configuration, office editing and push notifications.
The Stalwart Application bundle is a separate upstream artifact with
different routing and authentication behavior; this port packages the
general static-host archive.

## Full edition: feasible, but additional work

The full edition is a Next.js 16.4.3 / React 19 application. Upstream
builds on Linux and publishes standalone archives for Linux amd64 and
arm64, not FreeBSD. These archives should not be installed as a native
FreeBSD package without auditing and replacing their native dependencies.

Source inspection identified these requirements for a native source port:

* Prefetch the npm dependency graph for an offline Poudriere build using
  the ports npm framework and the release's package-lock.json.
* Resolve Next.js/SWC and next-intl's SWC dependency. The release lockfile
  has no FreeBSD SWC packages. Investigate explicit WASM dependencies and
  a webpack build, or provide maintained native compiler packages;
  upstream's default build uses Turbopack.
* Handle architecture-specific CSS tools. Tailwind Oxide 4.3.3 and
  Lightning CSS 1.32.0 list FreeBSD x64 packages in the lockfile, but no
  FreeBSD arm64 packages. esbuild 0.28.2 lists both FreeBSD architectures.
* Verify sharp 0.35.5's FreeBSD WASM package and audit optional
  `@napi-rs/canvas` use; that dependency lists no FreeBSD binary.
* Vendor the fonts referenced by `next/font/google` in the main layout
  and replace network font retrieval during the build with local inputs.
* Install the standalone server with an rc.d service, an unprivileged
  runtime user, writable cache and state under `/var`, and preserved
  configuration. Validate actual JMAP login and application behavior.

These are findings from source and lockfile inspection, not demonstrated
build failures. No dependency installation or compilation was attempted.

## Validation scope

The Lite release checksum matched upstream's published SHA-256. Upstream's
`verifyExport()` accepted the extracted archive: all 32 locales had the
required page shells; required deployment files, inline-script CSP hashes,
and documented API string checks passed. Archive paths were checked for
unsafe absolute paths and parent traversal.

Port metadata, extraction, file-copy installation into a private staging
directory, and agreement between installed files and the packing list are
checked separately. No application compilation, browser login test, package
installation, or Poudriere build has been run.

## Upstream references

* https://github.com/bulwarkmail/webmail/releases/tag/1.21.1
* https://bulwarkmail.org/docs/getting-started/editions
* https://bulwarkmail.org/docs/deployment/static
* https://bulwarkmail.org/docs/deployment/manual

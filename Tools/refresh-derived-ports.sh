#!/bin/sh

set -eu

PORTS=${PORTS:-/usr/ports}
OVERLAY=${OVERLAY:-/root/ports/LOCAL}
PATCHDIR="${OVERLAY}/Tools/patches"

tmp=$(mktemp -d -t overlay-refresh)
trap 'rm -rf "$tmp"' 0 1 2 15

stage_port()
{
	origin=$1
	patchfile=$2

	test -f "${PORTS}/${origin}/Makefile"
	test -f "${PATCHDIR}/${patchfile}"

	mkdir -p "${tmp}/${origin%/*}"

	rsync -a --delete --exclude '/work*' --exclude '/.git' \
	    "${PORTS}/${origin}/" \
	    "${tmp}/${origin}/"

	patch -d "${tmp}/${origin}" \
	    -p0 -t -s -V none \
	    < "${PATCHDIR}/${patchfile}"
}

# Headless variants retain one revision above the corresponding main-tree port.
# Keep revision handling separate from patches so release/revision changes do
# not make the local dependency patches fail.
stage_headless_port()
{
	stage_port "$1" "$2"
	revision=$(make -C "${PORTS}/${origin}" -V PORTREVISION)
	revision=${revision:-0}
	awk -v revision="$((revision + 1))" '
	    /^PORTREVISION[?+:]?=/ { next }
	    { print }
	    /^(PORTVERSION|DISTVERSION)[?+:]?=/ { printf "PORTREVISION=\t%s\n", revision }
	' "${tmp}/${origin}/Makefile" > "${tmp}/Makefile"
	mv "${tmp}/Makefile" "${tmp}/${origin}/Makefile"

	candidate=$(make -C "${tmp}/${origin}" -V PKGVERSION)
	current=$(make -C "${OVERLAY}/${origin}" -V PKGVERSION)
	if [ "$(pkg version -t "$candidate" "$current")" = '<' ]; then
		printf 'Keeping %s %s (main-tree derivative is older: %s)\n' \
		    "$origin" "$current" "$candidate"
	else
		refresh="$refresh $origin"
	fi
}

refresh=dns/bind920
stage_port dns/bind920 bind920.patch
stage_headless_port net-mgmt/unifi10 unifi10.patch
stage_headless_port archivers/snappy-java snappy-java.patch
stage_headless_port devel/sbt sbt.patch

# Publish only after all ports have been staged successfully. Preserve any
# generated work directories already present in the overlay.
for origin in $refresh; do
	rsync -a --delete --exclude '/work*' --exclude '/.git' \
	    "${tmp}/${origin}/" "${OVERLAY}/${origin}/"
done

# Unified-diff context contains significant whitespace.
git -C "$OVERLAY" diff --check -- ':!**/files/patch-*' ':!Tools/patches/*'
git -C "$OVERLAY" status --short

printf 'Refreshed from FreeBSD ports commit %s\n' \
    "$(git -C "$PORTS" rev-parse --short HEAD)"

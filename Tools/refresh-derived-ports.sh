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

stage_port dns/bind920 bind920.patch

# Keep the Java framework synchronized with main, applying the overlay policy
# before publishing either the framework or bind920.
mkdir -p "${tmp}/Mk/Uses"
cp "${PORTS}/Mk/Uses/java.mk" "${tmp}/Mk/Uses/java.mk"
patch -d "$tmp" -p0 -t -s -F 0 -V none \
    < "${PATCHDIR}/java.mk.patch"

rsync -a --delete --exclude '/work*' --exclude '/.git' \
    "${tmp}/dns/bind920/" "${OVERLAY}/dns/bind920/"
mkdir -p "${OVERLAY}/Mk/Uses"
cp "${tmp}/Mk/Uses/java.mk" "${OVERLAY}/Mk/Uses/java.mk"

# Unified-diff context contains significant whitespace.
git -C "$OVERLAY" diff --check -- ':!**/files/patch-*' ':!Tools/patches/*'
git -C "$OVERLAY" status --short

printf 'Refreshed from FreeBSD ports commit %s\n' \
    "$(git -C "$PORTS" rev-parse --short HEAD)"

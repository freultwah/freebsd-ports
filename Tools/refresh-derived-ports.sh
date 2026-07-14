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

	rsync -a --delete \
	    "${PORTS}/${origin}/" \
	    "${tmp}/${origin}/"

	patch -d "${tmp}/${origin}" \
	    -p0 -t -s -V none \
	    < "${PATCHDIR}/${patchfile}"
}

stage_port www/nginx nginx.patch
stage_port www/nginx-devel nginx-devel.patch
stage_port dns/bind920 bind920.patch

rm -f \
    "${tmp}/www/nginx/files/extra-patch-ngx_http_auth_ldap_module.c" \
    "${tmp}/www/nginx-devel/files/extra-patch-ngx_http_auth_ldap_module.c"

for origin in www/nginx www/nginx-devel dns/bind920
do
	rsync -a --delete \
	    "${tmp}/${origin}/" \
	    "${OVERLAY}/${origin}/"
done

git -C "$OVERLAY" diff --check
git -C "$OVERLAY" status --short

printf 'Refreshed from FreeBSD ports commit %s\n' \
    "$(git -C "$PORTS" rev-parse --short HEAD)"

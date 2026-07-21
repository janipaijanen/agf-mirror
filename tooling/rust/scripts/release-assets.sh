#!/usr/bin/env sh
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
#
# Enumerates which files in a release `dist/` directory should get a direct
# GitLab Release asset link. Used by the `release:create` CI job so that
# every archive AND its checksum/provenance sidecars are discoverable
# without requiring a consumer to guess generic-package URLs.
#
# Usage:
#   release-assets.sh list <dist-dir>   Print the basename of every file
#                                       that should be linked, one per line,
#                                       sorted.
#   release-assets.sh selftest         Exercise `list` against a fixture of
#                                       dummy artifacts and verify every
#                                       expected name is represented and
#                                       nothing unexpected is included.

set -eu

list() {
  dist_dir="$1"
  find "$dist_dir" -maxdepth 1 -type f \( \
      -name 'agf-*.tar.gz' -o -name 'agf-*.zip' -o \
      -name 'agov-*.tar.gz' -o -name 'agov-*.zip' -o \
      -name '*.sha256' -o -name '*-provenance.json' -o \
      -name 'SHA256SUMS' \
    \) -exec basename {} \; | sort
}

selftest() {
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  expected_names="agf-9.9.9.tar.gz
agf-9.9.9.tar.gz.sha256
agf-9.9.9.zip
agf-9.9.9.zip.sha256
agf-9.9.9-provenance.json
agov-9.9.9-x86_64-unknown-linux-musl.tar.gz
agov-9.9.9-x86_64-unknown-linux-musl.tar.gz.sha256
agov-9.9.9-x86_64-unknown-linux-musl-provenance.json
agov-9.9.9-aarch64-unknown-linux-musl.tar.gz
agov-9.9.9-aarch64-unknown-linux-musl.tar.gz.sha256
agov-9.9.9-aarch64-unknown-linux-musl-provenance.json
agov-9.9.9-x86_64-pc-windows-gnu.zip
agov-9.9.9-x86_64-pc-windows-gnu.zip.sha256
agov-9.9.9-x86_64-pc-windows-gnu-provenance.json
SHA256SUMS"

  unrelated_name="README-not-a-release-asset.txt"

  for f in $expected_names; do
    : > "$tmp/$f"
  done
  : > "$tmp/$unrelated_name"

  actual="$(list "$tmp")"
  failed=0

  for f in $expected_names; do
    if ! printf '%s\n' "$actual" | grep -qxF "$f"; then
      echo "MISSING from release-asset list: $f" >&2
      failed=1
    fi
  done

  if printf '%s\n' "$actual" | grep -qxF "$unrelated_name"; then
    echo "unexpected file linked: $unrelated_name" >&2
    failed=1
  fi

  if [ "$failed" -ne 0 ]; then
    echo "selftest FAILED" >&2
    exit 1
  fi

  count=$(printf '%s\n' "$actual" | grep -c .)
  echo "selftest OK: $count files enumerated correctly"
}

case "${1:-}" in
  list)
    [ $# -eq 2 ] || { echo "usage: $0 list <dist-dir>" >&2; exit 2; }
    list "$2"
    ;;
  selftest)
    selftest
    ;;
  *)
    echo "usage: $0 list <dist-dir> | selftest" >&2
    exit 2
    ;;
esac

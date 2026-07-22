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
#   release-assets.sh list <dist-dir>            Print the basename of every
#                                                 file present that should be
#                                                 linked, one per line, sorted.
#                                                 Discovery only -- does not
#                                                 fail on a missing file.
#   release-assets.sh expected <version>         Print the exact mandatory
#                                                 filename set for a release
#                                                 of <version>, one per line,
#                                                 sorted. Pure function, no
#                                                 filesystem access.
#   release-assets.sh verify <dist-dir> <version>
#                                                 Fail (exit 1) if any file
#                                                 from `expected <version>` is
#                                                 not a real file in
#                                                 <dist-dir>. This is the gate
#                                                 that catches a partial
#                                                 build/publish before
#                                                 release:create runs.
#   release-assets.sh selftest                   Exercise `list` and `verify`
#                                                 against fixtures, including
#                                                 a negative case with one
#                                                 mandatory file deliberately
#                                                 missing.

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

# The mandatory set a release MUST publish. Deliberately excludes
# `SHA256SUMS` -- the documented contract promises individual checksum
# files, not an aggregate one (see docs/coordination/20260721T123209Z-
# coder-tester-rggs.md); `list` still opportunistically links it if present.
#
# Provenance filenames follow one rule (v2.4.0 Workstream B, the unified
# release-provenance contract, agf/schemas/release-provenance.schema.json):
# <artifact_filename>-provenance.json for every file-kind artifact -- one
# record per published file, never an aggregate covering multiple formats
# or targets -- plus agov-oci-<version>-provenance.json for the published
# OCI image (an oci-image-kind record; it has no artifact_filename of its
# own since it is not a file in the generic package registry).
expected() {
  version="$1"
  cat <<EXPECTED | sort
agf-${version}.tar.gz
agf-${version}.tar.gz.sha256
agf-${version}.tar.gz-provenance.json
agf-${version}.zip
agf-${version}.zip.sha256
agf-${version}.zip-provenance.json
agov-${version}-linux-x86_64.tar.gz
agov-${version}-linux-x86_64.tar.gz.sha256
agov-${version}-linux-x86_64.tar.gz-provenance.json
agov-${version}-linux-aarch64.tar.gz
agov-${version}-linux-aarch64.tar.gz.sha256
agov-${version}-linux-aarch64.tar.gz-provenance.json
agov-${version}-windows-x86_64.zip
agov-${version}-windows-x86_64.zip.sha256
agov-${version}-windows-x86_64.zip-provenance.json
agov-oci-${version}-provenance.json
EXPECTED
}

verify() {
  dist_dir="$1"
  version="$2"
  missing=0
  for f in $(expected "$version"); do
    if [ ! -f "$dist_dir/$f" ]; then
      echo "MISSING mandatory release asset: $f" >&2
      missing=1
    fi
  done
  if [ "$missing" -ne 0 ]; then
    echo "verify FAILED: dist directory is missing mandatory release assets" >&2
    return 1
  fi
  echo "verify OK: all mandatory release assets present for $version"
}

selftest() {
  test_version="9.9.9"
  tmp="$(mktemp -d)"
  neg_tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp" "$neg_tmp"' EXIT
  failed=0

  fixture_names="$(expected "$test_version")
SHA256SUMS"
  unrelated_name="README-not-a-release-asset.txt"

  for f in $fixture_names; do
    : > "$tmp/$f"
  done
  : > "$tmp/$unrelated_name"

  # --- `list`: every fixture file discovered, unrelated file excluded ---
  actual="$(list "$tmp")"

  for f in $fixture_names; do
    if ! printf '%s\n' "$actual" | grep -qxF "$f"; then
      echo "MISSING from release-asset list: $f" >&2
      failed=1
    fi
  done

  if printf '%s\n' "$actual" | grep -qxF "$unrelated_name"; then
    echo "unexpected file linked: $unrelated_name" >&2
    failed=1
  fi

  # --- `verify`, positive case: complete dist dir passes ---
  if ! verify "$tmp" "$test_version" >/dev/null; then
    echo "verify unexpectedly FAILED against a complete fixture" >&2
    failed=1
  fi

  # --- `verify`, negative case: one mandatory sidecar deliberately missing ---
  # This is the negative test per gate-self-test-rule -- it proves `verify`
  # actually fires on a known-bad input, not just that it passes on good ones.
  missing_target="agov-${test_version}-linux-aarch64.tar.gz-provenance.json"
  for f in $(expected "$test_version"); do
    [ "$f" = "$missing_target" ] && continue
    : > "$neg_tmp/$f"
  done

  neg_output="$(verify "$neg_tmp" "$test_version" 2>&1 >/dev/null || true)"
  if verify "$neg_tmp" "$test_version" >/dev/null 2>&1; then
    echo "verify unexpectedly PASSED against a dist dir missing $missing_target" >&2
    failed=1
  elif ! printf '%s\n' "$neg_output" | grep -qF "$missing_target"; then
    echo "verify failed but did not name the missing file $missing_target" >&2
    echo "  verify stderr was: $neg_output" >&2
    failed=1
  fi

  if [ "$failed" -ne 0 ]; then
    echo "selftest FAILED" >&2
    exit 1
  fi

  count=$(printf '%s\n' "$actual" | grep -c .)
  echo "selftest OK: $count files enumerated by list; verify passes on a complete dist dir and fails (naming the missing file) on an incomplete one"
}

case "${1:-}" in
  list)
    [ $# -eq 2 ] || { echo "usage: $0 list <dist-dir>" >&2; exit 2; }
    list "$2"
    ;;
  expected)
    [ $# -eq 2 ] || { echo "usage: $0 expected <version>" >&2; exit 2; }
    expected "$2"
    ;;
  verify)
    [ $# -eq 3 ] || { echo "usage: $0 verify <dist-dir> <version>" >&2; exit 2; }
    verify "$2" "$3"
    ;;
  selftest)
    selftest
    ;;
  *)
    echo "usage: $0 list <dist-dir> | expected <version> | verify <dist-dir> <version> | selftest" >&2
    exit 2
    ;;
esac

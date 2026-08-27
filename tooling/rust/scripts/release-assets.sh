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
#   release-assets.sh verify-pre-oci <dist-dir> <version>
#                                                 Verify every mandatory file
#                                                 that must exist before OCI
#                                                 publication begins.
#   release-assets.sh selftest                   Exercise `list` and `verify`
#                                                 against fixtures, including
#                                                 a negative case with one
#                                                 mandatory file deliberately
#                                                 missing.

set -eu

SCRIPT_DIR=$(CDPATH= cd "$(dirname "$0")" && pwd)
TARGETS_MANIFEST="$SCRIPT_DIR/../release-targets.txt"

list() {
  dist_dir="$1"
  find "$dist_dir" -maxdepth 1 -type f \( \
      -name 'agf-*.tar.gz' -o -name 'agf-*.zip' -o \
      -name 'agov-*.tar.gz' -o -name 'agov-*.zip' -o \
      -name '*.sha256' -o -name '*-provenance.json' -o \
      -name 'SHA256SUMS' \
    \) -exec basename {} \; | sort
}

# The mandatory set a release MUST publish. Deliberately excludes optional
# Windows preview archives and `SHA256SUMS` -- the documented contract promises
# individual checksum files, not an aggregate one (see docs/coordination/
# 20260721T123209Z-coder-tester-rggs.md); `list` still opportunistically links
# any recognized optional artifact if present.
#
# Provenance filenames follow one rule (v2.4.0 Workstream B, the unified
# release-provenance contract, agf/schemas/release-provenance.schema.json):
# <artifact_filename>-provenance.json for every file-kind artifact -- one
# record per published file, never an aggregate covering multiple formats
# or targets -- plus agov-oci-<version>-provenance.json for the published
# OCI image (an oci-image-kind record; it has no artifact_filename of its
# own since it is not a file in the generic package registry).
manifest_records() {
  manifest="$1"
  awk '
    function reject(message) {
      printf "invalid release-target manifest %s:%d: %s\n", FILENAME, NR, message > "/dev/stderr"
      failed = 1
    }
    function unsafe(value) {
      return value ~ /[\\\/]/ || index(value, "..") || value ~ /^-/ || value ~ /[[:cntrl:]]/
    }
    /^[[:space:]]*$/ { next }
    /^#/ { next }
    {
      if (NF != 2) {
        reject("expected exactly two fields: <rust-triple> <public-slug>")
        next
      }
      if (unsafe($1) || unsafe($2)) {
        reject("fields must be safe path-component tokens (no slash, backslash, .., leading -, whitespace, or control characters)")
        next
      }
      if (targets[$1]++) {
        reject("duplicate target: " $1)
        next
      }
      if (slugs[$2]++) {
        reject("duplicate slug: " $2)
        next
      }
      count++
      target[count] = $1
      slug[count] = $2
    }
    END {
      if (failed) exit 1
      if (count == 0) {
        printf "invalid release-target manifest %s: no target records\n", FILENAME > "/dev/stderr"
        exit 1
      }
      for (i = 1; i <= count; i++) print target[i], slug[i]
    }
  ' "$manifest"
}

expected_targets_from() {
  manifest="$1"
  version="$2"
  if ! records=$(manifest_records "$manifest"); then
    return 1
  fi
  printf '%s\n' "$records" | while IFS=' ' read -r target slug; do
    [ -n "$target" ] || continue
    printf '%s\n' \
      "agov-${version}-${slug}.tar.gz" \
      "agov-${version}-${slug}.tar.gz.sha256" \
      "agov-${version}-${slug}.tar.gz-provenance.json"
  done
}

expected_targets() {
  expected_targets_from "$TARGETS_MANIFEST" "$1"
}

expected_pre_oci_from() {
  manifest="$1"
  version="$2"
  if ! target_names=$(expected_targets_from "$manifest" "$version"); then
    return 1
  fi
  {
    cat <<EXPECTED
agf-${version}.tar.gz
agf-${version}.tar.gz.sha256
agf-${version}.tar.gz-provenance.json
agf-${version}.zip
agf-${version}.zip.sha256
agf-${version}.zip-provenance.json
EXPECTED
    printf '%s\n' "$target_names"
  } | sort
}

expected_pre_oci() {
  expected_pre_oci_from "$TARGETS_MANIFEST" "$1"
}

expected_oci() {
  printf 'agov-oci-%s-provenance.json\n' "$1"
}

expected() {
  version="$1"
  if ! pre_oci_names=$(expected_pre_oci "$version"); then
    return 1
  fi
  {
    printf '%s\n' "$pre_oci_names"
    expected_oci "$version"
  } | sort
}

verify_set() {
  set_function="$1"
  dist_dir="$2"
  version="$3"
  if ! expected_files=$($set_function "$version"); then
    return 1
  fi
  missing=0
  for f in $expected_files; do
    if [ ! -f "$dist_dir/$f" ]; then
      echo "MISSING mandatory release asset: $f" >&2
      missing=1
    fi
  done
  if [ "$missing" -ne 0 ]; then
    echo "verify FAILED: dist directory is missing mandatory release assets" >&2
    return 1
  fi
  echo "$set_function OK: all mandatory release assets present for $version"
}

verify() {
  verify_set expected "$1" "$2"
}

verify_pre_oci() {
  verify_set expected_pre_oci "$1" "$2"
}

# Intentionally separate parser used only by the cross-contract self-test.
# Do not call manifest_records() here: this check must be able to disagree
# with the production parser if one implementation regresses.
independent_expected_targets_from() {
  manifest="$1"
  version="$2"
  awk -v version="$version" '
    function reject(message) {
      printf "independent manifest check %s:%d: %s\n", FILENAME, NR, message > "/dev/stderr"
      failed = 1
    }
    function unsafe(value) {
      return value ~ /[\\\/]/ || index(value, "..") || value ~ /^-/ || value ~ /[[:cntrl:]]/
    }
    /^[[:space:]]*$/ { next }
    /^#/ { next }
    {
      if (NF != 2) { reject("expected exactly two fields"); next }
      if (unsafe($1) || unsafe($2)) { reject("unsafe path-component token"); next }
      if (targets[$1]++) { reject("duplicate target: " $1); next }
      if (slugs[$2]++) { reject("duplicate slug: " $2); next }
      count++
      slug[count] = $2
    }
    END {
      if (failed) exit 1
      if (count == 0) { print "independent manifest check: no target records" > "/dev/stderr"; exit 1 }
      for (i = 1; i <= count; i++) {
        print "agov-" version "-" slug[i] ".tar.gz"
        print "agov-" version "-" slug[i] ".tar.gz.sha256"
        print "agov-" version "-" slug[i] ".tar.gz-provenance.json"
      }
    }
  ' "$manifest"
}

compare_contract_outputs() {
  production="$1"
  independent="$2"
  if [ "$production" != "$independent" ]; then
    echo "release-target cross-contract mismatch" >&2
    return 1
  fi
}

selftest() {
  test_version="9.9.9"
  tmp="$(mktemp -d)"
  mandatory_tmp="$(mktemp -d)"
  neg_tmp="$(mktemp -d)"
  manifest_tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp" "$mandatory_tmp" "$neg_tmp" "$manifest_tmp"' EXIT
  failed=0

  # Deliberately literal: update this list by hand whenever
  # release-targets.txt changes. It must not derive from expected().
  fixture_names="agf-${test_version}.tar.gz
agf-${test_version}.tar.gz.sha256
agf-${test_version}.tar.gz-provenance.json
agf-${test_version}.zip
agf-${test_version}.zip.sha256
agf-${test_version}.zip-provenance.json
agov-${test_version}-linux-x86_64.tar.gz
agov-${test_version}-linux-x86_64.tar.gz.sha256
agov-${test_version}-linux-x86_64.tar.gz-provenance.json
agov-oci-${test_version}-provenance.json
agov-${test_version}-windows-x86_64.zip
agov-${test_version}-windows-x86_64.zip.sha256
agov-${test_version}-windows-x86_64.zip-provenance.json
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

  # --- `verify`, optional-Windows case: the mandatory release is complete
  # without any Windows archive, checksum, or provenance record. ---
  for f in $(expected "$test_version"); do
    : > "$mandatory_tmp/$f"
  done
  if ! verify "$mandatory_tmp" "$test_version" >/dev/null; then
    echo "verify unexpectedly FAILED without optional Windows assets" >&2
    failed=1
  fi
  rm "$mandatory_tmp/agov-oci-${test_version}-provenance.json"
  if ! verify_pre_oci "$mandatory_tmp" "$test_version" >/dev/null; then
    echo "verify-pre-oci unexpectedly required post-push OCI provenance" >&2
    failed=1
  elif verify "$mandatory_tmp" "$test_version" >/dev/null 2>&1; then
    echo "full verify unexpectedly accepted a fixture without OCI provenance" >&2
    failed=1
  fi

  # --- `verify`, negative case: one mandatory sidecar deliberately missing ---
  # This is the negative test per gate-self-test-rule -- it proves `verify`
  # actually fires on a known-bad input, not just that it passes on good ones.
  missing_target="agov-${test_version}-linux-x86_64.tar.gz-provenance.json"
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

  # --- manifest cross-contract check: independent and production parsers ---
  production_targets=$(expected_targets "$test_version" | sort)
  independent_targets=$(independent_expected_targets_from "$TARGETS_MANIFEST" "$test_version" | sort)
  if ! compare_contract_outputs "$production_targets" "$independent_targets"; then
    echo "real release-target manifest failed the cross-contract check" >&2
    failed=1
  fi
  if compare_contract_outputs "$production_targets" "$production_targets
agov-${test_version}-intentional-divergence.tar.gz" >/dev/null 2>&1; then
    echo "cross-contract comparator failed to detect an intentional divergence" >&2
    failed=1
  fi

  # A valid new target expands both parsers consistently.
  growth_manifest="$manifest_tmp/growth.txt"
  printf '%s\n' \
    'x86_64-unknown-linux-musl linux-x86_64' \
    'example-unknown-linux-musl linux-example' > "$growth_manifest"
  growth_production=$(expected_targets_from "$growth_manifest" "$test_version" | sort)
  growth_independent=$(independent_expected_targets_from "$growth_manifest" "$test_version" | sort)
  if ! compare_contract_outputs "$growth_production" "$growth_independent" ||
     ! printf '%s\n' "$growth_production" | grep -qxF "agov-${test_version}-linux-example.tar.gz"; then
    echo "valid manifest growth did not expand both parsers consistently" >&2
    failed=1
  fi

  # Every malformed manifest must fail before emitting even one target name.
  assert_manifest_rejected() {
    name="$1"
    fixture="$manifest_tmp/$name.txt"
    stdout="$manifest_tmp/$name.stdout"
    stderr="$manifest_tmp/$name.stderr"
    shift
    printf '%b' "$1" > "$fixture"
    if expected_targets_from "$fixture" "$test_version" >"$stdout" 2>"$stderr"; then
      echo "malformed manifest unexpectedly accepted: $name" >&2
      failed=1
    elif [ -s "$stdout" ]; then
      echo "malformed manifest emitted partial output before rejection: $name" >&2
      failed=1
    fi
  }
  assert_manifest_rejected traversal '../etc linux-safe\n'
  assert_manifest_rejected slash 'target foo/bar\n'
  assert_manifest_rejected control 'bad\001target linux-safe\n'
  assert_manifest_rejected leading_dash '-target linux-safe\n'
  assert_manifest_rejected three_fields 'target linux-safe extra\n'
  assert_manifest_rejected duplicate_target 'target-a linux-a\ntarget-a linux-b\n'
  assert_manifest_rejected duplicate_slug 'target-a linux-a\ntarget-b linux-a\n'

  # The canonical pre-OCI subset excludes only the OCI provenance record.
  if expected_pre_oci "$test_version" | grep -qxF "agov-oci-${test_version}-provenance.json"; then
    echo "expected_pre_oci unexpectedly includes OCI provenance" >&2
    failed=1
  fi

  if [ "$failed" -ne 0 ]; then
    echo "selftest FAILED" >&2
    exit 1
  fi

  count=$(printf '%s\n' "$actual" | grep -c .)
  echo "selftest OK: $count files enumerated; manifest grammar/cross-contract checks pass; verify rejects an incomplete release"
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
  verify-pre-oci)
    [ $# -eq 3 ] || { echo "usage: $0 verify-pre-oci <dist-dir> <version>" >&2; exit 2; }
    verify_pre_oci "$2" "$3"
    ;;
  selftest)
    selftest
    ;;
  *)
    echo "usage: $0 list <dist-dir> | expected <version> | verify-pre-oci <dist-dir> <version> | verify <dist-dir> <version> | selftest" >&2
    exit 2
    ;;
esac

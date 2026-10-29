#!/bin/sh
# SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial
# Copyright (c) 2026 Jani Päijänen
#
# Advisory drift check for floating CI inputs.
# Plan: docs/plans/20261028T194451Z-ci-reproducibility-pins.md
# Approval: docs/coordination/20261029T162807Z-governor-reviewer-kmp6.md
# Self-test: tests/ci-pins-selftest.sh
#
# Authority: ADVISORY ONLY. No named rule grants this check blocking authority
# (agf/rules/always/tool-authority-boundary-rule.md). Every finding line is
# tagged "severity: advisory" and the script exits 0 when it runs to
# completion, whatever it finds. It exits 2 only for its own malfunction (an
# unreadable input). Promoting it to blocking needs a separately governed rule.
#
# Findings:
#   - an image: that uses the tag latest, or has neither a tag nor a digest
#     (scalar form "image: name:tag" and mapping form "image:" + "name:");
#   - the release:create image without an @sha256: digest;
#   - rustup --default-toolchain stable|beta|nightly;
#   - a pipe from sh.rustup.rs;
#   - RUST_TOOLCHAIN_VERSION differing from the .rust-base image tag.
#
# Usage: sh .gitlab/scripts/check-ci-pins.sh [path/to/.gitlab-ci.yml]

set -eu

file="${1:-.gitlab-ci.yml}"

if [ ! -r "$file" ]; then
  echo "ERROR severity: advisory check-ci-pins: cannot read input file: $file" >&2
  exit 2
fi

awk -v f="$file" '
function report(n, msg) {
  printf "ADVISORY severity: advisory check-ci-pins %s:%d: %s\n", f, n, msg
  found++
}
function indent(s) { match(s, /^ */); return RLENGTH }
function value(s) {
  sub(/^[^:]*:[ \t]*/, "", s)
  sub(/[ \t]+#.*$/, "", s)
  gsub(/^[ \t]+|[ \t]+$/, "", s)
  gsub(/^["\047]|["\047]$/, "", s)
  return s
}
function check_image(n, ref,    base, name, tag, digest) {
  if (ref == "" || ref ~ /\$/) return
  digest = (ref ~ /@sha256:/)
  base = ref
  sub(/@sha256:.*/, "", base)
  name = base
  sub(/^.*\//, "", name)
  tag = ""
  if (name ~ /:/) { tag = name; sub(/^[^:]*:/, "", tag) }
  if (tag == "latest") report(n, "image uses the floating tag latest: " ref)
  else if (tag == "" && !digest) report(n, "image has no tag and no digest: " ref)
  if (job == "release:create" && !digest) report(n, "release:create image has no @sha256: digest: " ref)
  if (job == ".rust-base" && tag != "") { rust_base_tag = tag; rust_base_line = n }
}
{
  line = $0
  if (line ~ /^[ \t]*#/ || line ~ /^[ \t]*$/) next
  ind = indent(line)

  if (ind == 0 && line ~ /^[^ \t#-][^:]*:/) {
    job = line
    sub(/:[ \t]*(#.*)?$/, "", job)
  }

  if (map_ind >= 0 && ind <= map_ind) map_ind = -1
  if (map_ind >= 0 && line ~ /^[ \t]*name:/) {
    check_image(NR, value(line))
    map_ind = -1
    next
  }
  if (line ~ /^[ \t]*image:/) {
    v = value(line)
    if (v == "") { map_ind = ind } else { check_image(NR, v); map_ind = -1 }
    next
  }
  if (line ~ /--default-toolchain[ =]+(stable|beta|nightly)/) report(NR, "rustup uses a floating toolchain: " value("x: " line))
  if (line ~ /sh\.rustup\.rs/) report(NR, "rustup is installed by an unversioned bootstrap script (sh.rustup.rs)")
  if (line ~ /^[ \t]*RUST_TOOLCHAIN_VERSION:/) { toolchain = value(line); toolchain_line = NR }
}
BEGIN { map_ind = -1; found = 0 }
END {
  if (toolchain != "" && rust_base_tag != "" && toolchain != rust_base_tag)
    report(toolchain_line, "RUST_TOOLCHAIN_VERSION " toolchain " differs from the .rust-base image tag " rust_base_tag " (line " rust_base_line ")")
  printf "check-ci-pins: %d advisory finding(s) in %s\n", found, f
}
' "$file"

exit 0

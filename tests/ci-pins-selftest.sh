#!/bin/sh
# SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial
# Copyright (c) 2026 Jani Päijänen
#
# Negative tests for .gitlab/scripts/check-ci-pins.sh, required by
# agf/rules/always/gate-self-test-rule.md.
# Plan: docs/plans/20261028T194451Z-ci-reproducibility-pins.md
#
# The checker is advisory: findings print "severity: advisory" lines and the
# exit code stays 0. Only its own malfunction (an unreadable input) exits
# non-zero. Each case below asserts BOTH the finding text and the exit code, so
# a checker that silently stops reporting, or that starts blocking, fails here.
#
# Run: sh tests/ci-pins-selftest.sh

set -eu

repo_root="$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)"
checker="$repo_root/.gitlab/scripts/check-ci-pins.sh"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT INT TERM
failures=0

# expect NAME FIXTURE_FILE EXPECTED_COUNT [REQUIRED_TEXT]
expect() {
  name="$1"; fixture="$2"; want="$3"; text="${4:-}"
  rc=0
  out="$(sh "$checker" "$fixture" 2>&1)" || rc=$?
  got="$(printf '%s\n' "$out" | grep -c '^ADVISORY severity: advisory ' || true)"
  ok=1
  [ "$rc" -eq 0 ] || ok=0
  [ "$got" -eq "$want" ] || ok=0
  if [ -n "$text" ]; then
    printf '%s\n' "$out" | grep -qF -- "$text" || ok=0
  fi
  if [ "$ok" -eq 1 ]; then
    echo "PASS $name (findings=$got exit=$rc)"
  else
    echo "FAIL $name (findings=$got want=$want exit=$rc want-text=[$text])"
    printf '%s\n' "$out" | sed 's/^/    /'
    failures=$((failures + 1))
  fi
}

new() { printf '%s\n' "$work/$1.yml"; }

f="$(new scalar-latest)"
cat > "$f" <<'YAML'
release:create:
  image: registry.gitlab.com/gitlab-org/release-cli:latest@sha256:aaaa
YAML
expect "scalar image with the tag latest" "$f" 1 "floating tag latest"

f="$(new scalar-untagged)"
cat > "$f" <<'YAML'
job:
  image: alpine
YAML
expect "scalar image with no tag and no digest" "$f" 1 "no tag and no digest"

f="$(new mapping-untagged)"
cat > "$f" <<'YAML'
job:
  image:
    name: alpine
    entrypoint: [""]
YAML
expect "mapping-form image with no tag" "$f" 1 "no tag and no digest"

f="$(new mapping-latest)"
cat > "$f" <<'YAML'
job:
  image:
    name: example.invalid/tool:latest
    entrypoint: [""]
YAML
expect "mapping-form image with the tag latest" "$f" 1 "floating tag latest"

f="$(new mapping-tagged)"
cat > "$f" <<'YAML'
job:
  image:
    name: alpine:3.22
    entrypoint: [""]
YAML
expect "mapping-form image with a tag (clean)" "$f" 0

f="$(new rustup-stable)"
cat > "$f" <<'YAML'
job:
  script:
    - ./rustup-init -y --default-toolchain stable --profile minimal
YAML
expect "rustup with --default-toolchain stable" "$f" 1 "floating toolchain"

f="$(new rustup-pipe)"
cat > "$f" <<'YAML'
job:
  script:
    - curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.97.1
YAML
expect "pipe from sh.rustup.rs" "$f" 1 "sh.rustup.rs"

f="$(new release-no-digest)"
cat > "$f" <<'YAML'
release:create:
  image: registry.gitlab.com/gitlab-org/release-cli:v0.24.0
YAML
expect "release:create image with a tag but no digest" "$f" 1 "no @sha256: digest"

f="$(new other-job-no-digest)"
cat > "$f" <<'YAML'
other:job:
  image: registry.gitlab.com/gitlab-org/release-cli:v0.24.0
YAML
expect "digest rule applies to release:create only" "$f" 0

f="$(new toolchain-mismatch)"
cat > "$f" <<'YAML'
variables:
  RUST_TOOLCHAIN_VERSION: "1.98.0"

.rust-base:
  image: rust:1.97.1@sha256:aaaa
YAML
expect "RUST_TOOLCHAIN_VERSION differs from .rust-base tag" "$f" 1 "differs from the .rust-base image tag"

f="$(new comments-ignored)"
cat > "$f" <<'YAML'
# image: alpine:latest and sh.rustup.rs are only mentioned in a comment
job:
  image: alpine:3.22
YAML
expect "comment lines are ignored" "$f" 0

# The corrected repository file must be clean.
expect "repository .gitlab-ci.yml (corrected)" "$repo_root/.gitlab-ci.yml" 0

# Malfunction: the only non-zero exit.
rc=0
out="$(sh "$checker" "$work/does-not-exist.yml" 2>&1)" || rc=$?
if [ "$rc" -ne 0 ] && printf '%s\n' "$out" | grep -q '^ERROR severity: advisory '; then
  echo "PASS missing input file exits non-zero with an error line (exit=$rc)"
else
  echo "FAIL missing input file (exit=$rc)"; printf '%s\n' "$out" | sed 's/^/    /'
  failures=$((failures + 1))
fi

if [ "$failures" -ne 0 ]; then
  echo "ci-pins-selftest: $failures case(s) FAILED"
  exit 1
fi
echo "ci-pins-selftest: all cases passed"

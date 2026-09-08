#!/bin/sh
# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
# CI adapter only: consume the existing preview, never build or publish.
set -eu
set +x
: "${CI_JOB_TOKEN:?ephemeral same-project job token required}"
: "${CI_PIPELINE_SOURCE:?MR pipeline required}"
[ "$CI_PIPELINE_SOURCE" = merge_request_event ] || exit 1
[ -z "${CI_COMMIT_TAG:-}" ] || exit 1
version=$(cat agf/VERSION)
case "$version" in ''|*[!0-9A-Za-z._-]*) exit 1;; esac
archive="dist/agov-$version-linux-x86_64.tar.gz"
staging=$(mktemp -d)
trap 'rm -rf "$staging"' EXIT HUP INT TERM
tar -xzf "$archive" -C "$staging"
binary="$staging/agov-$version-linux-x86_64/agov"
[ -x "$binary" ] || exit 1
# Downloaded build artifacts are ignored workspace data. Evidence is written
# only on successful completion; error diagnostics contain no response bodies.
"$binary" release namespace-check --phase mr > "$staging/namespace.json"
mkdir -p dist
cp "$staging/namespace.json" dist/release-namespace-absence.json
cat dist/release-namespace-absence.json

#!/bin/sh
# SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial
# Copyright (c) 2026 Jani Päijänen
#
# Fail-closed changed-file contract for governance validators.
#
# Owner: agf/rules/always/gate-self-test-rule.md (this is gate mechanics --
# every change here needs Gate 1 and a re-run negative test, per
# agf/rules/workflow/simplicity-rule.md's fast-track ineligibility list).
# Plan: docs/plans/20260829T211252Z-gate-change-discipline.md (slice B).
# Negative tests: tests/changed-files-failclosed.sh.
#
# Sourced by .gitlab-ci.yml's agov:governance:validate job. It replaces two
# inline `git diff ... | xargs -0r <validator>` pipelines that could report
# success without validating anything, in two independent ways:
#
#   1. `set -e` observes only a pipeline's LAST exit status. A failed
#      `git diff` -- unresolvable revision, missing merge base on a shallow
#      clone -- was masked by a successful `xargs`, which with -r simply
#      declines to run on empty input. Exit 0. Nothing validated.
#   2. An empty base degenerates `"$base"...HEAD` to `...HEAD` == `HEAD...HEAD`,
#      which is legitimately empty. Same silent pass.
#
# The contract is four ordered steps, each failing closed:
#
#   1. resolve the target ref explicitly; fail when the fetch fails or the
#      resolved base is empty or all-zero
#   2. prove a merge base exists; fail when it does not (this is the
#      shallow-clone case -- GIT_DEPTH is 20 by default on this project)
#   3. run `git diff` to a temporary file and check its exit status BEFORE
#      any consumer runs; never pipe diff output directly into a consumer
#   4. only then invoke the validator on the captured list
#
# Every function returns non-zero rather than exiting, so a caller running
# under `set -e` fails at the call site and a test can assert on the return
# value. Nothing here writes to stdout except the values a caller consumes.

# cf_resolve_base TARGET_REF
# Prints the resolved base SHA on stdout. Non-zero on any failure.
cf_resolve_base() {
  cf_target_ref="${1:?cf_resolve_base: target ref required}"

  if [ -z "$cf_target_ref" ]; then
    echo "changed-files: empty target ref; failing closed." >&2
    return 1
  fi

  if ! git fetch --no-tags origin "$cf_target_ref" >/dev/null 2>&1; then
    echo "changed-files: could not fetch 'origin/$cf_target_ref'; failing closed rather than comparing against an unknown base." >&2
    return 1
  fi

  cf_base="$(git rev-parse --verify --quiet "refs/remotes/origin/$cf_target_ref" || true)"

  if [ -z "$cf_base" ] || [ "$cf_base" = "0000000000000000000000000000000000000000" ]; then
    echo "changed-files: could not resolve a base commit for 'origin/$cf_target_ref'; failing closed." >&2
    return 1
  fi

  printf '%s\n' "$cf_base"
}

# cf_require_merge_base BASE [REV]
# Proves a merge base exists between BASE and REV (default HEAD). Non-zero
# when it does not -- the shallow-clone case, where a three-dot diff would
# otherwise fail and be masked by the consumer.
cf_require_merge_base() {
  cf_base="${1:?cf_require_merge_base: base required}"
  cf_rev="${2:-HEAD}"

  if ! git merge-base "$cf_base" "$cf_rev" >/dev/null 2>&1; then
    echo "changed-files: no merge base between '$cf_base' and '$cf_rev'." >&2
    echo "changed-files: this is the shallow-clone case -- the history needed to compare is not present in this checkout. Failing closed; a deeper fetch (GIT_DEPTH) is required to validate this range." >&2
    return 1
  fi
}

# cf_changed_files BASE OUTFILE DIFF_FILTER PATHSPEC...
# Writes the NUL-separated changed-path list to OUTFILE. DIFF_FILTER is
# passed to git's --diff-filter (use 'd' to exclude deletions, 'D' for
# deletions only). Non-zero when git diff fails -- OUTFILE is left empty in
# that case and MUST NOT be consumed, which is why the status is checked
# here rather than after a pipe.
cf_changed_files() {
  cf_base="${1:?cf_changed_files: base required}"
  cf_out="${2:?cf_changed_files: output file required}"
  cf_filter="${3:?cf_changed_files: diff-filter required}"
  shift 3

  : > "$cf_out" || return 1

  if ! git diff --name-only -z "--diff-filter=$cf_filter" \
        "$cf_base...HEAD" -- "$@" > "$cf_out"; then
    echo "changed-files: git diff failed for base '$cf_base'; failing closed rather than treating an errored diff as an empty change set." >&2
    : > "$cf_out"
    return 1
  fi
}

# cf_deleted_files BASE OUTFILE PATHSPEC...
# Writes the NUL-separated list of paths that DISAPPEAR over the range.
#
# --no-renames is load-bearing, not a style choice. Git classifies a rename
# as R, not D, so `--diff-filter=D` alone reports an empty list for a
# rename-and-rewrite: the old immutable record is gone, a validly-shaped new
# one stands in its place, and an append-only guard built on D sees nothing.
# Review 13rh (GCD-G2-2) demonstrated exactly that bypass against the first
# version of this contract. --no-renames decomposes a rename into D(old) +
# A(new), so the vanished path appears where the guard can see it.
#
# --diff-filter=DR was considered and rejected: with --name-only -z, an R
# entry emits BOTH source and destination paths into the NUL stream, so the
# consumer would receive the surviving destination as though it too had been
# deleted. --no-renames keeps one path per record.
cf_deleted_files() {
  cf_base="${1:?cf_deleted_files: base required}"
  cf_out="${2:?cf_deleted_files: output file required}"
  shift 2

  : > "$cf_out" || return 1

  if ! git diff --no-renames --name-only -z --diff-filter=D \
        "$cf_base...HEAD" -- "$@" > "$cf_out"; then
    echo "changed-files: git diff failed while deriving the deletion list for base '$cf_base'; failing closed." >&2
    : > "$cf_out"
    return 1
  fi
}

# cf_run_on_list OUTFILE COMMAND...
# Invokes COMMAND once with the captured paths appended, only when the list
# is non-empty. Separated from cf_changed_files so that a diff failure can
# never reach a consumer.
cf_run_on_list() {
  cf_out="${1:?cf_run_on_list: list file required}"
  shift

  if [ ! -s "$cf_out" ]; then
    return 0
  fi

  xargs -0r "$@" < "$cf_out"
}

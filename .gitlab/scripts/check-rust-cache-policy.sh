#!/bin/sh
set -eu

# expected_policy SOURCE BRANCH DEFAULT_BRANCH
# Prints the cache policy required for the given pipeline context, per
# docs/plans/ci-rust-cache-policy-slice-a.md's context acceptance matrix.
# Fail-safe: any unrecognised, ambiguous, or future pipeline source resolves
# to "pull". Only an ordinary default-branch push resolves to "pull-push".
expected_policy() {
  source=$1
  branch=$2
  default_branch=$3
  if [ "$source" = "push" ] && [ -n "$branch" ] && [ "$branch" = "$default_branch" ]; then
    printf '%s\n' "pull-push"
  else
    printf '%s\n' "pull"
  fi
}

# assert_matches SOURCE BRANCH DEFAULT_BRANCH ACTUAL_POLICY
# Fails loudly if ACTUAL_POLICY does not equal the table-derived expectation.
assert_matches() {
  source=$1
  branch=$2
  default_branch=$3
  actual=$4
  expected=$(expected_policy "$source" "$branch" "$default_branch")
  if [ "$actual" != "$expected" ]; then
    echo "cache policy mismatch: source=$source branch=$branch default_branch=$default_branch expected=$expected actual=$actual" >&2
    return 1
  fi
  return 0
}

# The row data below is a literal transcription of the plan's context
# acceptance matrix, not derived by calling expected_policy() itself -- the
# selftest must check expected_policy()'s output against an independently
# recorded table, not against its own reasoning, or a shared logic error in
# both places would satisfy the test. Fields: source|branch|default_branch|
# expected_policy. Empty branch models a tag pipeline (CI_COMMIT_BRANCH
# unset).
selftest_table() {
  cat <<'EOF'
push|main|main|pull-push
push|feature-x|main|pull
merge_request_event|feature-x|main|pull
merge_request_event|main|main|pull
push||main|pull
schedule|main|main|pull
api|main|main|pull
web|main|main|pull
trigger|main|main|pull
pipeline|main|main|pull
future-source|main|main|pull
EOF
}

selftest() {
  failures=0
  while IFS='|' read -r source branch default_branch want; do
    [ -n "$source" ] || continue
    got=$(expected_policy "$source" "$branch" "$default_branch")
    if [ "$got" != "$want" ]; then
      echo "selftest row failed: source=$source branch=$branch default_branch=$default_branch want=$want got=$got" >&2
      failures=1
    fi
  done <<EOF
$(selftest_table)
EOF

  # Negative: a deliberately wrong actual policy must be rejected, not
  # silently accepted, proving assert_matches is not a hard-coded pass.
  if assert_matches "push" "main" "main" "pull" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: wrong default-branch policy accepted" >&2
    failures=1
  fi
  if assert_matches "merge_request_event" "feature-x" "main" "pull-push" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: wrong MR policy accepted" >&2
    failures=1
  fi

  # Positive: a correct actual policy must be accepted.
  if ! assert_matches "push" "main" "main" "pull-push" >/dev/null 2>&1; then
    echo "positive fixture unexpectedly failed: correct default-branch policy rejected" >&2
    failures=1
  fi
  if ! assert_matches "merge_request_event" "feature-x" "main" "pull" >/dev/null 2>&1; then
    echo "positive fixture unexpectedly failed: correct MR policy rejected" >&2
    failures=1
  fi

  if [ "$failures" -ne 0 ]; then
    exit 1
  fi
  echo "OK cache-policy selftest: all context rows and negative/positive fixtures pass"
}

assert_current() {
  source="${CI_PIPELINE_SOURCE:-}"
  branch="${CI_COMMIT_BRANCH:-}"
  default_branch="${CI_DEFAULT_BRANCH:-}"
  actual="${RUST_CACHE_POLICY:-}"
  if [ -z "$actual" ]; then
    echo "RUST_CACHE_POLICY is unset; refusing to assume a policy" >&2
    exit 1
  fi
  if assert_matches "$source" "$branch" "$default_branch" "$actual"; then
    echo "OK resolved cache policy: $actual (source=$source branch=$branch default_branch=$default_branch)"
  else
    exit 1
  fi
}

case "${1:-assert-current}" in
  selftest) selftest ;;
  assert-current) assert_current ;;
  *) echo "usage: $0 [selftest|assert-current]" >&2; exit 2 ;;
esac

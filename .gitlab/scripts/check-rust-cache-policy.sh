#!/bin/sh
set -eu

# expected_policy SOURCE BRANCH DEFAULT_BRANCH ROLE
# Prints the cache policy required for the given pipeline context and role,
# per docs/plans/ci-rust-cache-policy-slice-b.md's context/role acceptance
# matrix. Fail-safe: any unrecognised, ambiguous, or future pipeline source
# resolves to "pull". Only the designated writer role in an ordinary
# default-branch push resolves to "pull-push" -- every other writer context
# (feature push, MR, tag, schedule, api, web, trigger, pipeline, unknown
# source) stays "pull", and every consumer-role job is always "pull"
# regardless of context. An unrecognised role fails closed: nonzero exit,
# no policy printed, rather than silently defaulting to "pull" -- a typo'd
# or missing role must be loud, not quietly safe-by-accident.
expected_policy() {
  source=$1
  branch=$2
  default_branch=$3
  role=$4
  case "$role" in
    writer|consumer) ;;
    *)
      echo "expected_policy: unknown role '$role' (must be 'writer' or 'consumer')" >&2
      return 1
      ;;
  esac
  if [ "$role" = "writer" ] && [ "$source" = "push" ] && [ -n "$branch" ] && [ "$branch" = "$default_branch" ]; then
    printf '%s\n' "pull-push"
  else
    printf '%s\n' "pull"
  fi
}

# assert_matches SOURCE BRANCH DEFAULT_BRANCH ROLE ACTUAL_POLICY
# Fails loudly if ACTUAL_POLICY does not equal the table-derived expectation,
# or if ROLE itself is unrecognised (expected_policy's fail-closed case).
assert_matches() {
  source=$1
  branch=$2
  default_branch=$3
  role=$4
  actual=$5
  if ! expected=$(expected_policy "$source" "$branch" "$default_branch" "$role"); then
    echo "cache policy mismatch: source=$source branch=$branch default_branch=$default_branch role=$role actual=$actual expected=<unknown role, fail closed>" >&2
    return 1
  fi
  if [ "$actual" != "$expected" ]; then
    echo "cache policy mismatch: source=$source branch=$branch default_branch=$default_branch role=$role expected=$expected actual=$actual" >&2
    return 1
  fi
  return 0
}

# The row data below is a literal transcription of the plan's context
# acceptance matrix (every Slice A context, exercised for both roles), not
# derived by calling expected_policy() itself -- the selftest must check
# expected_policy()'s output against an independently recorded table, not
# against its own reasoning, or a shared logic error in both places would
# satisfy the test. Fields: source|branch|default_branch|role|expected_policy.
# Empty branch models a tag pipeline (CI_COMMIT_BRANCH unset).
selftest_table() {
  cat <<'EOF'
push|main|main|writer|pull-push
push|main|main|consumer|pull
push|feature-x|main|writer|pull
push|feature-x|main|consumer|pull
merge_request_event|feature-x|main|writer|pull
merge_request_event|feature-x|main|consumer|pull
merge_request_event|main|main|writer|pull
merge_request_event|main|main|consumer|pull
push||main|writer|pull
push||main|consumer|pull
schedule|main|main|writer|pull
schedule|main|main|consumer|pull
api|main|main|writer|pull
api|main|main|consumer|pull
web|main|main|writer|pull
web|main|main|consumer|pull
trigger|main|main|writer|pull
trigger|main|main|consumer|pull
pipeline|main|main|writer|pull
pipeline|main|main|consumer|pull
future-source|main|main|writer|pull
future-source|main|main|consumer|pull
EOF
}

selftest() {
  failures=0
  while IFS='|' read -r source branch default_branch role want; do
    [ -n "$source" ] || continue
    got=$(expected_policy "$source" "$branch" "$default_branch" "$role")
    if [ "$got" != "$want" ]; then
      echo "selftest row failed: source=$source branch=$branch default_branch=$default_branch role=$role want=$want got=$got" >&2
      failures=1
    fi
  done <<EOF
$(selftest_table)
EOF

  # Negative fixtures, per docs/plans/ci-rust-cache-policy-slice-b.md:
  # each must be REJECTED by assert_matches, proving it is not a
  # hard-coded pass.
  if assert_matches "push" "main" "main" "consumer" "pull-push" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: default-branch consumer resolving pull-push accepted" >&2
    failures=1
  fi
  if assert_matches "merge_request_event" "feature-x" "main" "writer" "pull-push" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: MR writer-labelled job resolving pull-push accepted" >&2
    failures=1
  fi
  if assert_matches "push" "main" "main" "writer" "pull" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: designated default-branch writer resolving pull accepted" >&2
    failures=1
  fi
  if assert_matches "push" "main" "main" "bystander" "pull" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: unknown role accepted" >&2
    failures=1
  fi
  if assert_matches "push" "main" "main" "consumer" "" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed: unset/empty actual policy accepted" >&2
    failures=1
  fi

  # Positive: a correct actual policy must be accepted, for both roles.
  if ! assert_matches "push" "main" "main" "writer" "pull-push" >/dev/null 2>&1; then
    echo "positive fixture unexpectedly failed: correct default-branch writer policy rejected" >&2
    failures=1
  fi
  if ! assert_matches "push" "main" "main" "consumer" "pull" >/dev/null 2>&1; then
    echo "positive fixture unexpectedly failed: correct default-branch consumer policy rejected" >&2
    failures=1
  fi
  if ! assert_matches "merge_request_event" "feature-x" "main" "consumer" "pull" >/dev/null 2>&1; then
    echo "positive fixture unexpectedly failed: correct MR consumer policy rejected" >&2
    failures=1
  fi

  if [ "$failures" -ne 0 ]; then
    exit 1
  fi
  echo "OK cache-policy selftest: all context/role rows and negative/positive fixtures pass"
}

assert_current() {
  source="${CI_PIPELINE_SOURCE:-}"
  branch="${CI_COMMIT_BRANCH:-}"
  default_branch="${CI_DEFAULT_BRANCH:-}"
  role="${RUST_CACHE_ROLE:-}"
  if [ -z "$role" ]; then
    echo "RUST_CACHE_ROLE is unset; refusing to assume a role" >&2
    exit 1
  fi
  case "$role" in
    writer) actual="${RUST_CACHE_WRITER_POLICY:-}" ;;
    consumer) actual="${RUST_CACHE_POLICY:-}" ;;
    *)
      echo "RUST_CACHE_ROLE='$role' is not 'writer' or 'consumer'; refusing to assume a policy" >&2
      exit 1
      ;;
  esac
  if [ -z "$actual" ]; then
    echo "cache policy variable for role='$role' is unset; refusing to assume a policy" >&2
    exit 1
  fi
  if assert_matches "$source" "$branch" "$default_branch" "$role" "$actual"; then
    echo "OK resolved cache policy: $actual (role=$role source=$source branch=$branch default_branch=$default_branch)"
  else
    exit 1
  fi
}

case "${1:-assert-current}" in
  selftest) selftest ;;
  assert-current) assert_current ;;
  *) echo "usage: $0 [selftest|assert-current]" >&2; exit 2 ;;
esac

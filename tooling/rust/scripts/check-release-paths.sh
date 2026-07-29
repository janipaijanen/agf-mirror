#!/bin/sh
set -eu

# Blocking authority: agf/rules/workflow/ci-trigger-scoping-rule.md declares
# MR-to-main checks blocking on failure. The negative self-test is required by
# agf/rules/always/gate-self-test-rule.md.

forbidden_token='agent-governance'

check_tree() {
  target=$1
  if grep -R -n -F "$forbidden_token" "$target"; then
    echo "severity: blocking rule: ci-trigger-scoping-rule superseded repository path found under $target" >&2
    return 1
  fi
  echo "severity: blocking rule: ci-trigger-scoping-rule release-path check passed for $target"
}

selftest() {
  fixture=$(mktemp -d)
  trap 'rm -r "$fixture"' EXIT HUP INT TERM

  printf '%s\n' 'current release content' > "$fixture/good.md"
  check_tree "$fixture" >/dev/null

  printf '%s\n' "https://janipaijanen.gitlab.io/$forbidden_token/evidence/" > "$fixture/bad-pages.md"
  if check_tree "$fixture" >/dev/null 2>&1; then
    echo "selftest FAILED: known-bad predecessor Pages path was not blocked" >&2
    exit 1
  fi

  rm "$fixture/bad-pages.md"
  printf '%s\n' "https://gitlab.com/janipaijanen/$forbidden_token/-/releases" > "$fixture/bad-repository.md"
  if check_tree "$fixture" >/dev/null 2>&1; then
    echo "selftest FAILED: known-bad predecessor repository path was not blocked" >&2
    exit 1
  fi

  rm "$fixture/bad-repository.md"
  check_tree "$fixture" >/dev/null
  echo "selftest OK: clean input passes and superseded repository/Pages paths are blocked"
}

case "${1:-}" in
  check)
    test "$#" -eq 2 || { echo "usage: $0 check <release-tree> | selftest" >&2; exit 2; }
    check_tree "$2"
    ;;
  selftest)
    test "$#" -eq 1 || { echo "usage: $0 check <release-tree> | selftest" >&2; exit 2; }
    selftest
    ;;
  *)
    echo "usage: $0 check <release-tree> | selftest" >&2
    exit 2
    ;;
esac

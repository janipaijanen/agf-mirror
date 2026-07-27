#!/bin/sh
set -eu

# Blocking authority: agf/rules/workflow/ci-trigger-scoping-rule.md declares
# MR-to-main checks blocking on failure. The negative self-test is required by
# agf/rules/always/gate-self-test-rule.md.

evidence_one='docs/reviews/agf-framework-assessment-codex-sol-high.md'
evidence_two='docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md'
runbook='agf/docs/site/docs/operate/poll-and-coordinate.md'
tooling_runbook='docs/poll-and-coordinate.md'

fail() {
  echo "severity: blocking rule: ci-trigger-scoping-rule release self-sufficiency: $*" >&2
  return 1
}

has_member() {
  required=$1
  awk -v required="$required" '
    {
      path = $0
      sub(/^\.\//, "", path)
      if (path == required || (length(path) > length(required) && substr(path, length(path) - length(required), 1) == "/" && substr(path, length(path) - length(required) + 1) == required)) found = 1
    }
    END { exit(found ? 0 : 1) }
  '
}

find_member() {
  required=$1
  awk -v required="$required" '
    {
      path = $0
      sub(/^\.\//, "", path)
      if (path == required || (length(path) > length(required) && substr(path, length(path) - length(required), 1) == "/" && substr(path, length(path) - length(required) + 1) == required)) {
        print path
        exit
      }
    }
  '
}

list_members() {
  target=$1
  if [ -d "$target" ]; then
    find "$target" -type f -print | sed "s#^$target/##"
  else
    case "$target" in
      *.tar.gz|*.tgz) tar -tzf "$target" ;;
      *.zip) unzip -Z1 "$target" ;;
      *) fail "unsupported release payload: $target"; return 2 ;;
    esac
  fi
}

read_member() {
  target=$1
  member=$2
  if [ -d "$target" ]; then
    cat "$target/$member"
  else
    case "$target" in
      *.tar.gz|*.tgz) tar -xOzf "$target" "$member" ;;
      *.zip) unzip -p "$target" "$member" ;;
      *) return 2 ;;
    esac
  fi
}

require_member() {
  members=$1
  required=$2
  label=$3
  printf '%s\n' "$members" | has_member "$required" || fail "$label is missing required local path $required"
}

check_source_tree() {
  target=$1
  manifest="$target/agf/dist/release-manifest.yaml"
  for required in "$evidence_one" "$evidence_two"; do
    test -f "$target/$required" || { fail "source tree is missing $required"; return 1; }
    grep -F -- "- path: $required" "$manifest" >/dev/null || { fail "release manifest does not vendor $required"; return 1; }
  done
  test -f "$target/$runbook" || { fail "source tree is missing canonical runbook $runbook"; return 1; }
  echo "severity: blocking rule: ci-trigger-scoping-rule release self-sufficiency check passed for $target"
}

check_payload() {
  target=$1
  members=$(list_members "$target") || return $?

  if printf '%s\n' "$members" | has_member 'agf/VERSION'; then
    require_member "$members" "$evidence_one" "framework release" || return 1
    require_member "$members" "$evidence_two" "framework release" || return 1
    require_member "$members" "$runbook" "framework release" || return 1
  elif printf '%s\n' "$members" | awk '/(^|\/)agov(\.exe)?$/ { found = 1 } END { exit(found ? 0 : 1) }'; then
    require_member "$members" "$tooling_runbook" "tooling release" || return 1
    readme_member=$(printf '%s\n' "$members" | find_member 'README.txt')
    test -n "$readme_member" || { fail "tooling release is missing README.txt"; return 1; }
    read_member "$target" "$readme_member" | grep -F "Operating runbook: $tooling_runbook" >/dev/null ||
      { fail "tooling README.txt does not reference included local path $tooling_runbook"; return 1; }
  else
    fail "could not classify release payload: $target"
    return 1
  fi

  echo "severity: blocking rule: ci-trigger-scoping-rule release self-sufficiency check passed for $target"
}

check_path() {
  target=$1
  if [ -d "$target" ] && [ -f "$target/agf/dist/release-manifest.yaml" ]; then
    check_source_tree "$target"
  elif [ -d "$target" ] || [ -f "$target" ]; then
    check_payload "$target"
  else
    fail "release payload not found: $target"
    return 2
  fi
}

selftest() {
  fixture=$(mktemp -d)
  trap 'rm -rf "$fixture"' EXIT HUP INT TERM

  framework="$fixture/framework"
  mkdir -p "$framework/agf/docs/site/docs/operate" "$framework/docs/reviews"
  printf '2.4.5\n' > "$framework/agf/VERSION"
  printf 'runbook\n' > "$framework/$runbook"
  printf 'assessment\n' > "$framework/$evidence_one"
  printf 'priorities\n' > "$framework/$evidence_two"
  tar -C "$framework" -czf "$fixture/framework-good.tar.gz" agf docs
  check_path "$fixture/framework-good.tar.gz" >/dev/null

  rm "$framework/$evidence_two"
  tar -C "$framework" -czf "$fixture/framework-bad.tar.gz" agf docs
  if check_path "$fixture/framework-bad.tar.gz" >/dev/null 2>&1; then
    echo "selftest FAILED: framework archive missing vendored evidence passed" >&2
    return 1
  fi

  tooling="$fixture/agov-2.4.5-linux-x86_64"
  mkdir -p "$tooling/docs"
  printf 'binary\n' > "$tooling/agov"
  printf 'runbook\n' > "$tooling/$tooling_runbook"
  printf 'Operating runbook: %s (included in this archive).\n' "$tooling_runbook" > "$tooling/README.txt"
  tar -C "$fixture" -czf "$fixture/tooling-good.tar.gz" "$(basename "$tooling")"
  check_path "$fixture/tooling-good.tar.gz" >/dev/null

  rm "$tooling/$tooling_runbook"
  tar -C "$fixture" -czf "$fixture/tooling-bad.tar.gz" "$(basename "$tooling")"
  if check_path "$fixture/tooling-bad.tar.gz" >/dev/null 2>&1; then
    echo "selftest FAILED: README reference to nonexistent vendored runbook passed" >&2
    return 1
  fi

  echo "selftest OK: clean archives pass and missing local evidence/runbook paths are blocked"
}

case "${1:-}" in
  check)
    shift
    test "$#" -gt 0 || { echo "usage: $0 check <archive-or-directory>... | selftest" >&2; exit 2; }
    for target in "$@"; do check_path "$target"; done
    ;;
  selftest)
    test "$#" -eq 1 || { echo "usage: $0 check <archive-or-directory>... | selftest" >&2; exit 2; }
    selftest
    ;;
  *)
    echo "usage: $0 check <archive-or-directory>... | selftest" >&2
    exit 2
    ;;
esac

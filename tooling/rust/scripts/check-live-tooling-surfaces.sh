#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/../../.." && pwd)

scan_forbidden() {
  if grep -Ein 'mix[[:space:]]+agov|uv[[:space:]]+run[[:space:]]+agov-|tooling/elixir|bin/agf' "$@"; then
    return 1
  else
    status=$?
    if [ "$status" -eq 1 ]; then
      return 0
    fi
    echo "unable to scan configured live surface" >&2
    return 1
  fi
}

selftest() {
  fixture_dir=$(mktemp -d)
  trap 'rm -rf "$fixture_dir"' EXIT HUP INT TERM
  printf '%s\n' 'agov validate infosec' >"$fixture_dir/good"
  printf '%s\n' 'mix agov.validate.infosec' >"$fixture_dir/bad"
  printf '%s\n' 'An ignored `tooling/elixir/` directory may remain.' >"$fixture_dir/bad-residue"

  scan_forbidden "$fixture_dir/good"
  if scan_forbidden "$fixture_dir/bad" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed" >&2
    exit 1
  fi
  if scan_forbidden "$fixture_dir/bad-residue" >/dev/null 2>&1; then
    echo "negative residue fixture unexpectedly passed" >&2
    exit 1
  fi
  echo "OK live tooling-surface negative self-test"
}

check_repo() {
  cd "$repo_root"

  if [ -e bin/agf ]; then
    echo "broken legacy wrapper remains: bin/agf" >&2
    exit 1
  fi
  if [ -e tooling/rust/tests/parity/run.sh ]; then
    echo "dead Elixir parity harness remains" >&2
    exit 1
  fi
  if [ -n "$(git ls-files 'tooling/elixir/**')" ]; then
    echo "tracked Elixir implementation files remain" >&2
    git ls-files 'tooling/elixir/**' >&2
    exit 1
  fi

  live_failure=0
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    if [ ! -f "$path" ]; then
      echo "configured live surface is missing: $path" >&2
      live_failure=1
    elif ! scan_forbidden "$path"; then
      live_failure=1
    fi
  done <<'EOF'
.gitlab/merge_request_templates/Default.md
AGENTS.md
.gitlab/merge_request_templates/Learning Record.md
.claude/rules/always/gate-self-test-rule.md
.claude/rules/workflow/release-via-merge-request-rule.md
agf/docs/adopt/decommission.md
agf/docs/contracts/contract-stability.md
agf/docs/contracts/integration-contract.md
agf/docs/governance/manifest.md
agf/docs/site/roadmap.md
agf/framework.md
agf/rules/always/gate-self-test-rule.md
docs/TODO.md
EOF
  if [ "$live_failure" -ne 0 ]; then
    echo "stale tooling instruction found on a supported live surface" >&2
    exit 1
  fi

  echo "OK supported live tooling surfaces use Rust agov"
}

case "${1:-check}" in
  selftest) selftest ;;
  check) check_repo ;;
  *) echo "usage: $0 [check|selftest]" >&2; exit 2 ;;
esac

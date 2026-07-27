#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
SOURCE="$REPO_ROOT/agf/docs/site/docs/operate/poll-and-coordinate.md"
ARCHIVE_PATH="docs/poll-and-coordinate.md"

usage() {
  echo "usage: $0 stage <tooling-package-dir> | check <tooling-package-dir> | selftest" >&2
  exit 2
}

check_package() {
  package_dir=$1
  cmp "$SOURCE" "$package_dir/$ARCHIVE_PATH" >/dev/null || {
    echo "tooling package runbook does not match canonical source" >&2
    return 1
  }
  grep -F "Operating runbook: $ARCHIVE_PATH" "$package_dir/README.txt" >/dev/null || {
    echo "tooling package README.txt does not reference local runbook path" >&2
    return 1
  }
}

stage_package() {
  package_dir=$1
  test -f "$package_dir/README.txt" || {
    echo "tooling package README.txt must exist before runbook staging" >&2
    return 1
  }
  mkdir -p "$package_dir/docs"
  cp "$SOURCE" "$package_dir/$ARCHIVE_PATH"
  printf '\nOperating runbook: %s (included in this archive).\n' "$ARCHIVE_PATH" >> "$package_dir/README.txt"
  check_package "$package_dir"
}

selftest() {
  temp_dir=$(mktemp -d)
  trap 'rm -rf "$temp_dir"' EXIT HUP INT TERM
  package_dir="$temp_dir/agov-test"
  mkdir -p "$package_dir"
  printf 'AGF agov Rust tooling\n' > "$package_dir/README.txt"
  stage_package "$package_dir"
  check_package "$package_dir"

  printf 'stale runbook\n' > "$package_dir/$ARCHIVE_PATH"
  if check_package "$package_dir" 2>/dev/null; then
    echo "selftest failed: changed runbook was accepted" >&2
    return 1
  fi

  cp "$SOURCE" "$package_dir/$ARCHIVE_PATH"
  printf 'AGF agov Rust tooling\n' > "$package_dir/README.txt"
  if check_package "$package_dir" 2>/dev/null; then
    echo "selftest failed: missing local README reference was accepted" >&2
    return 1
  fi
}

case "${1:-}" in
  stage)
    test "$#" -eq 2 || usage
    stage_package "$2"
    ;;
  check)
    test "$#" -eq 2 || usage
    check_package "$2"
    ;;
  selftest)
    test "$#" -eq 1 || usage
    selftest
    ;;
  *) usage ;;
esac

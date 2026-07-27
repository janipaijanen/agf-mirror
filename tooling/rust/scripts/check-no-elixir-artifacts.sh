#!/bin/sh
set -eu

is_forbidden_member() {
  awk '
    {
      path = $0
      sub(/^\.\//, "", path)
      if (path ~ /(^|\/)tooling\/elixir\// ||
          path ~ /(^|\/)mix\.exs$/ ||
          path ~ /(^|\/)mix\.lock$/ ||
          path ~ /\.ex$/ ||
          path ~ /\.exs$/) {
        print path
        found = 1
      }
    }
    END { exit(found ? 0 : 1) }
  '
}

check_members() {
  label=$1
  members=$2
  if printf '%s\n' "$members" | is_forbidden_member; then
    echo "forbidden Elixir/Mix artifact in $label" >&2
    return 1
  fi
}

check_path() {
  target=$1
  if [ -d "$target" ]; then
    members=$(find "$target" -type f -print | sed "s#^$target/##")
  elif [ -f "$target" ]; then
    case "$target" in
      *.tar.gz|*.tgz) members=$(tar -tzf "$target") ;;
      *.zip) members=$(unzip -Z1 "$target") ;;
      *) echo "unsupported release payload: $target" >&2; return 2 ;;
    esac
  else
    echo "release payload not found: $target" >&2
    return 2
  fi
  check_members "$target" "$members"
}

selftest() {
  fixture=$(mktemp -d)
  trap 'rm -rf "$fixture"' EXIT INT TERM

  mkdir -p "$fixture/good/package" "$fixture/bad/package/lib" "$fixture/bad/package/tooling/elixir"
  printf '%s\n' rust-only > "$fixture/good/package/README.txt"
  printf '%s\n' retired > "$fixture/bad/package/mix.lock"
  printf '%s\n' retired > "$fixture/bad/package/lib/legacy.ex"
  printf '%s\n' retired > "$fixture/bad/package/tooling/elixir/mix.exs"
  tar -C "$fixture/good" -czf "$fixture/good.tar.gz" package
  tar -C "$fixture/bad" -czf "$fixture/bad.tar.gz" package

  check_path "$fixture/good.tar.gz"
  if check_path "$fixture/bad.tar.gz" >/dev/null 2>&1; then
    echo "negative fixture unexpectedly passed" >&2
    return 1
  fi
  echo "OK release payload Elixir/Mix exclusion self-test"
}

case "${1:-}" in
  check)
    shift
    [ "$#" -gt 0 ] || { echo "usage: $0 check <archive-or-directory>..." >&2; exit 2; }
    for target in "$@"; do check_path "$target"; done
    ;;
  selftest) selftest ;;
  *) echo "usage: $0 {check <archive-or-directory>...|selftest}" >&2; exit 2 ;;
esac

#!/usr/bin/env sh
set -eu

repo_root=${1:-}
if [ -z "$repo_root" ]; then
  repo_root=$(git rev-parse --show-toplevel 2>/dev/null) || {
    echo "agov-source-fingerprint: repository root argument required" >&2
    exit 2
  }
fi

hash_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$1" | awk '{print $1}'
  elif command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$1" | awk '{print $NF}'
  else
    echo "agov-source-fingerprint: SHA-256 implementation not found" >&2
    return 127
  fi
}

file_list=$(mktemp)
fingerprint_rows=$(mktemp)
trap 'rm -f "$file_list" "$fingerprint_rows"' EXIT HUP INT TERM

if git -C "$repo_root" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  git -C "$repo_root" ls-files --cached --others --exclude-standard -- \
    tooling/agov tooling/rust agf/schemas agf/agents/MANIFEST.yaml
else
  for path in tooling/agov tooling/rust agf/schemas agf/agents/MANIFEST.yaml; do
    if [ -f "$repo_root/$path" ]; then
      printf '%s\n' "$path"
    elif [ -d "$repo_root/$path" ]; then
      find "$repo_root/$path" -type f \
        ! -path '*/target/*' ! -path '*/.git/*' \
        | sed "s#^$repo_root/##"
    fi
  done
fi | LC_ALL=C sort -u > "$file_list"

while IFS= read -r path; do
  [ -f "$repo_root/$path" ] || continue
  if [ -x "$repo_root/$path" ]; then
    mode=executable
  else
    mode=regular
  fi
  printf '%s\t%s\t%s\n' "$mode" "$path" "$(hash_file "$repo_root/$path")" \
    >> "$fingerprint_rows"
done < "$file_list"

hash_file "$fingerprint_rows"

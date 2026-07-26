#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
verify="$script_dir/../scripts/verify-tooling-provenance.sh"
cross_config="$script_dir/../Cross.toml"
version=9.8.7
commit=0123456789abcdef0123456789abcdef01234567
good=$(printf '{"version":"%s","commit":"%s","dirty":false,"source_dirty":false}' "$version" "$commit")
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

sh "$verify" "$version" "$commit" "$good" "$good" >/dev/null

for required_env in AGOV_BUILD_COMMIT AGOV_BUILD_DIRTY AGOV_BUILD_SOURCE_DIRTY; do
  grep -Fq "\"$required_env\"" "$cross_config" || {
    echo "Cross.toml does not pass through $required_env" >&2
    exit 1
  }
done

for bad in \
  "$(printf '{"version":"0.0.0","commit":"%s","dirty":false,"source_dirty":false}' "$commit")" \
  "$(printf '{"version":"%s","commit":"deadbeefdeadbeefdeadbeefdeadbeefdeadbeef","dirty":false,"source_dirty":false}' "$version")" \
  "$(printf '{"version":"%s","commit":"%s","dirty":true,"source_dirty":false}' "$version" "$commit")" \
  "$(printf '{"version":"%s","commit":"%s","dirty":false,"source_dirty":true}' "$version" "$commit")"
do
  if sh "$verify" "$version" "$commit" "$bad" "$good" >/dev/null 2>&1; then
    echo "expected mismatched provenance to fail: $bad" >&2
    exit 1
  fi
done

marker="AGOV_BUILD_PROVENANCE_V1|${version}|${commit}|false|false"
printf 'prefix\n%s\nsuffix\n' "$marker" >"$tmp_dir/agov"
cp "$tmp_dir/agov" "$tmp_dir/agov-mcp"
sh "$verify" --binary "$version" "$commit" "$tmp_dir/agov" "$tmp_dir/agov-mcp" >/dev/null

printf 'AGOV_BUILD_PROVENANCE_V1|%s|%s|false|true\n' "$version" "$commit" >"$tmp_dir/agov-mcp"
archive="$tmp_dir/should-not-exist.tar.gz"
set +e
(
  set -e
  sh "$verify" --binary "$version" "$commit" "$tmp_dir/agov" "$tmp_dir/agov-mcp"
  : >"$archive"
) >"$tmp_dir/rejection.log" 2>&1
status=$?
set -e
if [ "$status" -eq 0 ]; then
  echo "expected mismatched binary provenance marker to fail" >&2
  exit 1
fi
if [ -e "$archive" ]; then
  echo "mismatched binary reached archive creation" >&2
  exit 1
fi

echo 'OK tooling provenance verifier negative fixtures; archive creation not reached'

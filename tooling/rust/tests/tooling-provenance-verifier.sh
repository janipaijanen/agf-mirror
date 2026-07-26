#!/usr/bin/env sh
set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
verify="$script_dir/../scripts/verify-tooling-provenance.sh"
version=9.8.7
commit=0123456789abcdef0123456789abcdef01234567
good=$(printf '{"version":"%s","commit":"%s","dirty":false,"source_dirty":false}' "$version" "$commit")

sh "$verify" "$version" "$commit" "$good" "$good" >/dev/null

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

echo 'OK tooling provenance verifier negative fixtures'

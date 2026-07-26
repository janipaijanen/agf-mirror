#!/usr/bin/env sh
set -eu

expected_version=$1
expected_commit=$2
agov_json=$3
mcp_json=$4

verify_one() {
  name=$1
  payload=$2
  printf '%s' "$payload" | jq -e \
    --arg version "$expected_version" \
    --arg commit "$expected_commit" \
    '.version == $version and .commit == $commit and .dirty == false and .source_dirty == false' \
    >/dev/null || {
      echo "$name provenance mismatch: $payload" >&2
      return 1
    }
}

verify_one agov "$agov_json"
verify_one agov-mcp "$mcp_json"

echo "OK tooling provenance version=$expected_version commit=$expected_commit dirty=false source_dirty=false"

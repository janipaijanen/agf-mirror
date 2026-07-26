#!/usr/bin/env sh
set -eu

if [ "${1:-}" = "--binary" ]; then
  expected_version=$2
  expected_commit=$3
  expected_marker="AGOV_BUILD_PROVENANCE_V1|${expected_version}|${expected_commit}|false|false"

  verify_binary() {
    name=$1
    binary=$2
    strings -a "$binary" | awk -v marker="$expected_marker" '
      index($0, marker) { found = 1 }
      END { exit !found }
    ' || {
      echo "$name binary provenance marker mismatch: expected $expected_marker" >&2
      observed=$(strings -a "$binary" | awk '
        match($0, /AGOV_BUILD_PROVENANCE_V1\|[^[:space:]]+/) {
          print substr($0, RSTART, RLENGTH)
        }
      ' | head -n 1)
      echo "$name binary provenance marker observed: ${observed:-not found}" >&2
      return 1
    }
  }

  verify_binary agov "$4"
  verify_binary agov-mcp "$5"
  echo "OK tooling binary provenance version=$expected_version commit=$expected_commit dirty=false source_dirty=false"
  exit 0
fi

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

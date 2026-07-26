#!/usr/bin/env sh
set -eu

fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT HUP INT TERM

mkdir -p "$fixture/tooling/rust/target/release" "$fixture/agf/schemas" "$fixture/agf/agents"
cp "$(git rev-parse --show-toplevel)/tooling/agov" "$fixture/tooling/agov"
printf '%s\n' '{}' > "$fixture/agf/schemas/test.json"
printf '%s\n' 'agents: []' > "$fixture/agf/agents/MANIFEST.yaml"
printf '%s\n' 'unrelated' > "$fixture/README.md"

cat > "$fixture/tooling/rust/target/release/agov" <<'EOF'
#!/usr/bin/env sh
if [ "${1:-}" = version ] && [ "${2:-}" = --format ] && [ "${3:-}" = json ]; then
  printf '{"version":"test","commit":"%s","dirty":false,"source_dirty":%s}\n' \
    "${PROVENANCE_COMMIT:-unknown}" "${PROVENANCE_SOURCE_DIRTY:-false}"
  exit 0
fi
printf 'fixture agov ran\n'
EOF
chmod +x "$fixture/tooling/agov" "$fixture/tooling/rust/target/release/agov"

git -C "$fixture" init -q
git -C "$fixture" config user.name fixture
git -C "$fixture" config user.email fixture@example.invalid
git -C "$fixture" add tooling/agov agf README.md
git -C "$fixture" commit -qm initial
initial=$(git -C "$fixture" rev-parse HEAD)

expect_run() {
  output=$(cd "$fixture" && PROVENANCE_COMMIT=$1 ./tooling/agov --version 2>&1)
  printf '%s' "$output" | grep -q 'fixture agov ran'
}

expect_reject() {
  set +e
  output=$(cd "$fixture" && PROVENANCE_COMMIT=$1 PROVENANCE_SOURCE_DIRTY=${2:-false} ./tooling/agov ${3:---version} 2>&1)
  status=$?
  set -e
  [ "$status" -eq 127 ]
  printf '%s' "$output" | grep -q 'no provenance-compatible local binary'
}

# Exact provenance and unrelated untracked state are accepted.
printf '%s\n' '{}' > "$fixture/.mcp.json"
expect_run "$initial"

# Unrelated tracked changes after the build do not invalidate it.
printf '%s\n' 'unrelated update' > "$fixture/README.md"
git -C "$fixture" add README.md
git -C "$fixture" commit -qm unrelated
expect_run "$initial"

# A freshness-sensitive committed change invalidates the old binary.
printf '%s\n' '# schema change' >> "$fixture/agf/schemas/test.json"
git -C "$fixture" add agf/schemas/test.json
git -C "$fixture" commit -qm sensitive
sensitive=$(git -C "$fixture" rev-parse HEAD)
expect_reject "$initial"
expect_run "$sensitive"

# A feature build that descends from the requested base remains valid.
output=$(cd "$fixture" && PROVENANCE_COMMIT=$sensitive ./tooling/agov session resume --base-branch "$initial" 2>&1)
printf '%s' "$output" | grep -q 'fixture agov ran'

# Build-relevant dirty provenance and malformed provenance fail closed.
expect_reject "$sensitive" true
expect_reject unknown false

# A binary compatible with an old branch is rejected against a newer requested base.
git -C "$fixture" branch newer "$sensitive"
git -C "$fixture" checkout -q -b old "$initial"
set +e
output=$(cd "$fixture" && PROVENANCE_COMMIT=$initial ./tooling/agov session resume --base-branch newer 2>&1)
status=$?
set -e
[ "$status" -eq 127 ]
printf '%s' "$output" | grep -q 'stale relative to requested base newer'

echo 'OK agov wrapper freshness gate'

#!/usr/bin/env sh
set -eu

fixture=$(mktemp -d)
linked=
cleanup() {
  if [ -n "$linked" ]; then
    git -C "$fixture" worktree remove --force "$linked" >/dev/null 2>&1 || true
    rm -rf "$linked"
  fi
  rm -rf "$fixture"
}
trap cleanup EXIT HUP INT TERM

mkdir -p "$fixture/tooling/rust/target/release" "$fixture/tooling/rust/scripts" "$fixture/agf/schemas" "$fixture/agf/agents"
cp "$(git rev-parse --show-toplevel)/tooling/agov" "$fixture/tooling/agov"
cp "$(git rev-parse --show-toplevel)/tooling/rust/scripts/agov-source-fingerprint.sh" \
  "$fixture/tooling/rust/scripts/agov-source-fingerprint.sh"
printf '%s\n' '{}' > "$fixture/agf/schemas/test.json"
printf '%s\n' 'agents: []' > "$fixture/agf/agents/MANIFEST.yaml"
printf '%s\n' 'unrelated' > "$fixture/README.md"
# Match the real checkout: generated binaries must not fingerprint themselves.
printf '%s\n' '/tooling/rust/target/' > "$fixture/.gitignore"

cat > "$fixture/tooling/rust/target/release/agov" <<'EOF'
#!/usr/bin/env sh
if [ "${1:-}" = version ] && [ "${2:-}" = --format ] && [ "${3:-}" = json ]; then
  printf '{"version":"test","commit":"%s","dirty":false,"source_dirty":%s,"source_fingerprint":"%s"}\n' \
    "${PROVENANCE_COMMIT:-unknown}" "${PROVENANCE_SOURCE_DIRTY:-false}" \
    "${PROVENANCE_FINGERPRINT:-unknown}"
  exit 0
fi
printf 'fixture agov ran\n'
EOF
chmod +x "$fixture/tooling/agov" "$fixture/tooling/rust/target/release/agov" \
  "$fixture/tooling/rust/scripts/agov-source-fingerprint.sh"

git -C "$fixture" init -q
git -C "$fixture" config user.name fixture
git -C "$fixture" config user.email fixture@example.invalid
git -C "$fixture" add tooling/agov agf README.md .gitignore
git -C "$fixture" commit -qm initial
initial=$(git -C "$fixture" rev-parse HEAD)
initial_fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")

expect_run() {
  commit=$1
  shift
  if [ "$#" -eq 0 ]; then
    set -- --version
  fi
  fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")
  output=$(cd "$fixture" && PROVENANCE_COMMIT=$commit \
    PROVENANCE_FINGERPRINT=$fingerprint ./tooling/agov "$@" 2>&1)
  printf '%s' "$output" | grep -q 'fixture agov ran'
}

expect_reject() {
  fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")
  set +e
  output=$(cd "$fixture" && PROVENANCE_COMMIT=$1 \
    PROVENANCE_SOURCE_DIRTY=${2:-false} PROVENANCE_FINGERPRINT=$fingerprint \
    PATH=/usr/bin:/bin ./tooling/agov ${3:---version} 2>&1)
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
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" = "$initial_fingerprint"
expect_run "$initial"

# A freshness-sensitive committed change invalidates the old binary.
printf '%s\n' '# schema change' >> "$fixture/agf/schemas/test.json"
git -C "$fixture" add agf/schemas/test.json
git -C "$fixture" commit -qm sensitive
sensitive=$(git -C "$fixture" rev-parse HEAD)
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" != "$initial_fingerprint"
expect_reject "$initial"
expect_run "$sensitive"

# Unstaged, staged, untracked, and executable-mode tooling changes all alter
# the exact content fingerprint without relying on mtimes.
clean_fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")
printf '%s\n' '# dirty schema change' >> "$fixture/agf/schemas/test.json"
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" != "$clean_fingerprint"
git -C "$fixture" add agf/schemas/test.json
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" != "$clean_fingerprint"
git -C "$fixture" restore --staged --worktree agf/schemas/test.json
printf '%s\n' 'untracked source' > "$fixture/tooling/rust/new-source.rs"
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" != "$clean_fingerprint"
rm "$fixture/tooling/rust/new-source.rs"
chmod -x "$fixture/tooling/agov"
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" != "$clean_fingerprint"
chmod +x "$fixture/tooling/agov"
test "$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")" = "$clean_fingerprint"

# The wrapper forwards the authoritative Rust command tree instead of
# rejecting commands through a second hand-maintained registry.
while IFS= read -r command; do
  # Intentional field splitting: each line is one argument vector.
  # shellcheck disable=SC2086
  expect_run "$sensitive" $command
done <<'EOF'
--help
coord --help
coord validate --help
coord brief --help
coord post --help
coord list --help
coord status --help
coord stale --help
coord visibility --help
coord graph --help
coord graph-index --help
coord publication-check --help
plan --help
plan validate --help
plan projection-check --help
validate --help
validate adoption --help
validate adopter-docs --help
validate agents --help
validate learnings --help
validate infosec --help
validate contributions --help
validate framework-config --help
validate readme-stats --help
handoff --help
handoff next --help
handoff take --help
handoff complete --help
session --help
session resume --help
token --help
token roi --help
token roi report --help
dist --help
dist validate --help
dist build --help
dist check-tag --help
dist validate-provenance --help
dist self-report-check --help
dist post-publication-check --help
dist provenance-record --help
dist provenance-record file --help
dist provenance-record oci --help
usage --help
usage report --help
EOF

# A feature build that descends from the requested base remains valid.
fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")
output=$(cd "$fixture" && PROVENANCE_COMMIT=$sensitive \
  PROVENANCE_FINGERPRINT=$fingerprint ./tooling/agov session resume --base-branch "$initial" 2>&1)
printf '%s' "$output" | grep -q 'fixture agov ran'

# Exact content fingerprints make dirty development builds deterministic;
# malformed commit provenance still fails closed.
output=$(cd "$fixture" && PROVENANCE_COMMIT=$sensitive \
  PROVENANCE_SOURCE_DIRTY=true PROVENANCE_FINGERPRINT=$fingerprint \
  ./tooling/agov --version 2>&1)
printf '%s' "$output" | grep -q 'fixture agov ran'
expect_reject unknown false

# A binary compatible with an old branch is rejected against a newer requested base.
git -C "$fixture" branch newer "$sensitive"
git -C "$fixture" checkout -q -b old "$initial"
fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")
set +e
output=$(cd "$fixture" && PROVENANCE_COMMIT=$initial \
  PROVENANCE_FINGERPRINT=$fingerprint PATH=/usr/bin:/bin \
  ./tooling/agov session resume --base-branch newer 2>&1)
status=$?
set -e
[ "$status" -eq 127 ]
printf '%s' "$output" | grep -q 'stale relative to requested base newer'

# A stale binary triggers exactly one release build when Cargo is available,
# and the rebuilt binary is verified before the original command is forwarded.
git -C "$fixture" checkout -q newer
fake_bin="$fixture/fake-bin"
mkdir -p "$fake_bin"
cat > "$fake_bin/cargo" <<'EOF'
#!/usr/bin/env sh
set -eu
commit=$(git -C "$FIXTURE_ROOT" rev-parse HEAD)
cat > "$FIXTURE_ROOT/tooling/rust/target/release/agov" <<INNER
#!/usr/bin/env sh
if [ "\${1:-}" = version ] && [ "\${2:-}" = --format ] && [ "\${3:-}" = json ]; then
  printf '%s\\n' '{"version":"test","commit":"$commit","dirty":true,"source_dirty":true,"source_fingerprint":"$AGOV_BUILD_SOURCE_FINGERPRINT"}'
  exit 0
fi
printf '%s\\n' 'rebuilt fixture agov ran'
INNER
chmod +x "$FIXTURE_ROOT/tooling/rust/target/release/agov"
EOF
chmod +x "$fake_bin/cargo"
# Seed valid commit provenance with a wrong embedded fingerprint. Rebuilding
# must embed the true source digest, not the rejected candidate's digest.
expected_rebuild_fingerprint=$("$fixture/tooling/rust/scripts/agov-source-fingerprint.sh" "$fixture")
output=$(cd "$fixture" && PROVENANCE_COMMIT=$sensitive \
  PROVENANCE_FINGERPRINT=stale-fixture-fingerprint \
  FIXTURE_ROOT=$fixture PATH="$fake_bin:/usr/bin:/bin" \
  ./tooling/agov coord publication-check --help 2>&1)
printf '%s' "$output" | grep -q 'tooling source fingerprint differs'
printf '%s' "$output" | grep -q 'rebuilding stale or missing repository-local release binary'
printf '%s' "$output" | grep -q 'rebuilt fixture agov ran'
rebuilt_provenance=$("$fixture/tooling/rust/target/release/agov" version --format json)
rebuilt_fingerprint=${rebuilt_provenance#*'"source_fingerprint":"'}
rebuilt_fingerprint=${rebuilt_fingerprint%%\"*}
if [ "$rebuilt_fingerprint" != "$expected_rebuild_fingerprint" ]; then
  echo "ERR rebuild embedded rejected fingerprint: $rebuilt_fingerprint" >&2
  exit 1
fi
output=$(cd "$fixture" && FIXTURE_ROOT=$fixture PATH="$fake_bin:/usr/bin:/bin" \
  ./tooling/agov coord publication-check --help 2>&1)
if printf '%s' "$output" | grep -q 'rebuilding stale or missing repository-local release binary'; then
  echo 'ERR unchanged second invocation rebuilt again' >&2
  exit 1
fi
printf '%s' "$output" | grep -q 'rebuilt fixture agov ran'
echo 'OK rejected wrong fingerprint; rebuilt correct digest; second invocation reused binary'

# Invoking one worktree's wrapper from another linked worktree must fail with
# the correct local wrapper path instead of silently binding to the first tree.
linked="${fixture}-linked"
git -C "$fixture" worktree add -q "$linked" "$sensitive"
set +e
output=$(cd "$linked" && PROVENANCE_COMMIT=$sensitive \
  PROVENANCE_FINGERPRINT=$fingerprint "$fixture/tooling/agov" --version 2>&1)
status=$?
set -e
[ "$status" -eq 126 ]
printf '%s' "$output" | grep -q "$linked/tooling/agov"

echo 'OK agov wrapper freshness gate'

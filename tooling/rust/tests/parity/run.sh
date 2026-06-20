#!/usr/bin/env bash
# Parity test harness: runs both the Elixir and Rust CLIs on the same fixture
# set and diffs stdout and stderr byte-for-byte.
#
# Invoked by the rust:tooling:parity CI job. Exit code 0 = parity pass.
#
# Codex owns: fixture list, reference file generation, per-command comparisons.
# This skeleton is the harness structure; Codex fills in the fixture paths and
# reference generation logic during Phase 1 implementation.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"
ELIXIR_DIR="${REPO_ROOT}/tooling/elixir"
RUST_BIN="${REPO_ROOT}/tooling/rust/target/debug/agov"

FIXTURES_DIR="${SCRIPT_DIR}/fixtures"
REFS_DIR="${SCRIPT_DIR}/refs"

pass=0
fail=0

run_case() {
    local name="$1"
    shift
    local args=("$@")

    local ref_stdout="${REFS_DIR}/${name}.stdout"
    local ref_stderr="${REFS_DIR}/${name}.stderr"

    # Rust
    rust_stdout=$("${RUST_BIN}" "${args[@]}" 2>/tmp/parity_stderr || true)
    rust_stderr=$(cat /tmp/parity_stderr)

    if diff -u "${ref_stdout}" <(echo "${rust_stdout}") > /tmp/parity_stdout_diff 2>&1 && \
       diff -u "${ref_stderr}" <(echo "${rust_stderr}") > /tmp/parity_stderr_diff 2>&1; then
        echo "PASS ${name}"
        ((pass++)) || true
    else
        echo "FAIL ${name}"
        cat /tmp/parity_stdout_diff || true
        cat /tmp/parity_stderr_diff || true
        ((fail++)) || true
    fi
}

# Codex: add run_case invocations here as each command is implemented.
# Example:
#   run_case "coord_validate_ok" coord validate "${FIXTURES_DIR}/valid-coord.md"
#   run_case "coord_validate_err" coord validate "${FIXTURES_DIR}/invalid-coord.md"

echo ""
echo "Parity: ${pass} passed, ${fail} failed"
[ "${fail}" -eq 0 ]

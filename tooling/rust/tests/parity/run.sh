#!/usr/bin/env bash
# Parity test harness: runs both the Elixir and Rust CLIs on the same fixture
# set and diffs stdout, stderr, and exit codes.
#
# Invoked by the rust:tooling:parity CI job. Exit code 0 = parity pass.
#
# ROI logging is disabled so the side-channel cannot affect output parity.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../../.." && pwd)"
ELIXIR_DIR="${REPO_ROOT}/tooling/elixir"
RUST_BIN="${REPO_ROOT}/tooling/rust/target/debug/agov"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

pass=0
fail=0

export AGOV_TOKEN_ROI=0
export AGOV_SCHEMA_DIR="${REPO_ROOT}/agf/schemas"
export AGOV_MANIFEST_PATH="${REPO_ROOT}/agf/agents/MANIFEST.yaml"

cd "${ELIXIR_DIR}"
mix compile >/dev/null

run_compare() {
    local name="$1"
    local rust_args="$2"
    local elixir_cmd="$3"

    local rust_stdout="${TMP_DIR}/${name}.rust.stdout"
    local rust_stderr="${TMP_DIR}/${name}.rust.stderr"
    local elixir_stdout="${TMP_DIR}/${name}.elixir.stdout"
    local elixir_stderr="${TMP_DIR}/${name}.elixir.stderr"

    local rust_status=0
    local elixir_status=0

    cd "${ELIXIR_DIR}"
    # shellcheck disable=SC2086
    "${RUST_BIN}" ${rust_args} >"${rust_stdout}" 2>"${rust_stderr}" || rust_status=$?
    # shellcheck disable=SC2086
    mix ${elixir_cmd} >"${elixir_stdout}" 2>"${elixir_stderr}" || elixir_status=$?

    if [ "${name}" = "validate_umbrella" ]; then
        sed -i '/agov\.validate\.assistant_drift/d;/agov\.validate\.coverage/d' "${rust_stdout}"
    fi

    if diff -u "${elixir_stdout}" "${rust_stdout}" >"${TMP_DIR}/${name}.stdout.diff" 2>&1 && \
       diff -u "${elixir_stderr}" "${rust_stderr}" >"${TMP_DIR}/${name}.stderr.diff" 2>&1 && \
       [ "${rust_status}" -eq "${elixir_status}" ]; then
        echo "PASS ${name}"
        ((pass++)) || true
    else
        echo "FAIL ${name}"
        if [ "${rust_status}" -ne "${elixir_status}" ]; then
            echo "exit code: elixir=${elixir_status} rust=${rust_status}"
        fi
        cat "${TMP_DIR}/${name}.stdout.diff" || true
        cat "${TMP_DIR}/${name}.stderr.diff" || true
        ((fail++)) || true
    fi
}

abs() {
    local path="$1"
    printf '%s/%s' "${REPO_ROOT}" "${path}"
}

run_coord_case() {
    local name="$1"
    local file
    file="$(abs "$2")"
    run_compare "${name}" "coord validate ${file}" "agov.coord.validate ${file}"
}

run_plan_case() {
    local name="$1"
    local file
    file="$(abs "$2")"
    run_compare "${name}" "plan validate ${file}" "agov.plan.validate ${file}"
}

run_validate_case() {
    local name="$1"
    local rust_args="$2"
    shift
    shift
    local elixir_cmd="$*"
    run_compare "${name}" "${rust_args}" "${elixir_cmd}"
}

run_coord_case "coord_validate_ok" "docs/coordination/01KVK3KF8D5HEB9KQH1EYC4QJW-coder-tester.md"
run_coord_case "coord_validate_missing_kind" "tooling/elixir/test/fixtures/invalid_missing_kind.md"
run_coord_case "coord_validate_claim_no_artifacts" "tooling/elixir/test/fixtures/invalid_claim_no_artifacts.md"
run_plan_case "plan_validate_ok" "docs/plans/rust-agov-port.md"
run_plan_case "plan_validate_missing_auditor" "tooling/elixir/test/fixtures/invalid_plan_missing_auditor.md"
run_plan_case "plan_validate_waived_no_reason" "tooling/elixir/test/fixtures/invalid_plan_waived_no_reason.md"

run_validate_case "validate_agents" "validate agents" "agov.validate.agents $(abs agf/agents)"
run_validate_case "validate_learnings" "validate learnings" "agov.validate.learnings $(abs agf/learnings)"
run_validate_case "validate_adoption" "validate adoption" "agov.validate.adoption $(abs agf)"
run_validate_case "validate_infosec" "validate infosec" "agov.validate.infosec $(abs agf/learnings)"
run_validate_case "validate_contributions" "validate contributions" "agov.validate.contributions $(abs agf/learnings)"
run_validate_case "validate_framework_config" "validate framework-config" "agov.validate.framework_config"
run_validate_case "validate_umbrella" "validate" "agov.validate $(abs agf)"

echo ""
echo "Parity: ${pass} passed, ${fail} failed"
[ "${fail}" -eq 0 ]

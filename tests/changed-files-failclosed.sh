#!/bin/sh
# SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial
# Copyright (c) 2026 Jani Päijänen
#
# Negative tests for .gitlab/scripts/changed-files.sh, required by
# agf/rules/always/gate-self-test-rule.md. A gate ships with proof that it
# fires on known-bad input, or it does not ship -- and, since this change
# widens that rule to cover modification as well as introduction, this suite
# must be re-run and its transcript shown for every later change to the
# contract, not only for this first one.
#
# Every test names the failure mode it demonstrates, shows the OLD piped
# pattern silently passing where relevant, and asserts the NEW contract
# exits non-zero. The old-pattern half matters: an assertion that the new
# code fails is not evidence the old code was broken.
#
# Run:  sh tests/changed-files-failclosed.sh
# Env:  AGOV_BIN  path to an agov binary (default: ./tooling/agov)

set -eu

repo_root="$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)"
contract="$repo_root/.gitlab/scripts/changed-files.sh"
fixtures="$repo_root/tests/fixtures/changed-files"
AGOV_BIN="${AGOV_BIN:-$repo_root/tooling/agov}"

# shellcheck source=/dev/null
. "$contract"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

pass=0
fail=0

ok()   { pass=$((pass + 1)); printf '  PASS  %s\n' "$1"; }
bad()  { fail=$((fail + 1)); printf '  FAIL  %s\n' "$1"; }

check_nonzero() {
  # check_nonzero LABEL COMMAND...
  label=$1
  shift
  if "$@" >/dev/null 2>&1; then
    bad "$label (expected non-zero, got 0 -- the gate did NOT fail closed)"
  else
    ok "$label"
  fi
}

check_zero() {
  label=$1
  shift
  if "$@" >/dev/null 2>&1; then
    ok "$label"
  else
    bad "$label (expected 0, got non-zero)"
  fi
}

git_quiet() { git -c init.defaultBranch=main -c user.email=t@example.invalid -c user.name=test "$@"; }

commit_file() {
  # commit_file PATH CONTENT MESSAGE
  mkdir -p "$(dirname -- "$1")"
  printf '%s\n' "$2" > "$1"
  git_quiet add -- "$1"
  git_quiet commit -q -m "$3"
}

printf '\n== 1. The masking this contract exists to close ==\n'
printf '   `set -e` observes only a pipeline'"'"'s LAST exit status, so a failed\n'
printf '   `git diff` is hidden by a successful `xargs -0r`.\n\n'

origin="$work/origin"
mkdir -p "$origin"
(
  cd "$origin"
  git_quiet init -q .
  commit_file "docs/plans/base.md" "base" "base"
) >/dev/null 2>&1

if ( cd "$origin" && sh -c 'set -eu
      git diff --name-only -z does-not-resolve...HEAD -- "docs/plans/*.md" \
        | xargs -0r true' ) >/dev/null 2>&1; then
  ok "OLD piped pattern exits 0 on an unresolvable revision (the defect, reproduced)"
else
  bad "OLD piped pattern did NOT exit 0 -- the premise of this whole change no longer holds; re-derive it before trusting the rest"
fi

check_nonzero "NEW cf_changed_files exits non-zero on an unresolvable revision" \
  sh -c "cd '$origin' && . '$contract' && cf_changed_files does-not-resolve '$work/out1' d 'docs/plans/*.md'"

printf '\n== 2. Failure mode: empty target ref (unset CI variable) ==\n'
printf '   An empty base degenerates `\$base...HEAD` to `HEAD...HEAD`, which is\n'
printf '   legitimately empty -- so the old shape exits 0 having validated nothing.\n\n'

if ( cd "$origin" && sh -c 'set -eu
      EMPTY=""
      git diff --name-only -z "$EMPTY"...HEAD -- "docs/plans/*.md" \
        | xargs -0r true' ) >/dev/null 2>&1; then
  ok "OLD pattern exits 0 with an empty base (silent pass, reproduced)"
else
  bad "OLD pattern did NOT exit 0 with an empty base"
fi

check_nonzero "NEW cf_resolve_base rejects an empty target ref" \
  sh -c "cd '$origin' && . '$contract' && cf_resolve_base ''"

printf '\n== 3. Failure mode: fetch failure ==\n\n'

clone="$work/clone"
git_quiet clone -q "$origin" "$clone" >/dev/null 2>&1
check_nonzero "NEW cf_resolve_base fails closed when the target ref does not exist on origin" \
  sh -c "cd '$clone' && . '$contract' && cf_resolve_base 'no-such-branch'"

( cd "$clone" && git_quiet remote set-url origin "$work/does-not-exist" ) >/dev/null 2>&1
check_nonzero "NEW cf_resolve_base fails closed when the remote is unreachable" \
  sh -c "cd '$clone' && . '$contract' && cf_resolve_base 'main'"

printf '\n== 4. Failure mode: missing merge base (unrelated histories) ==\n\n'

unrelated="$work/unrelated"
mkdir -p "$unrelated"
(
  cd "$unrelated"
  git_quiet init -q .
  commit_file "docs/plans/other.md" "other" "unrelated root"
) >/dev/null 2>&1
unrelated_sha="$(cd "$unrelated" && git rev-parse HEAD)"
( cd "$origin" && git_quiet fetch -q "$unrelated" HEAD:refs/unrelated ) >/dev/null 2>&1

check_nonzero "NEW cf_require_merge_base fails closed when no merge base exists" \
  sh -c "cd '$origin' && . '$contract' && cf_require_merge_base '$unrelated_sha'"

printf '\n== 5. Failure mode: shallow/diverged history (the GIT_DEPTH 20 case) ==\n'
printf '   Target and feature each 40 commits past a common base, cloned at\n'
printf '   --depth 20. This is the case at74 verified against a real runner\n'
printf '   (job 16065719319 reports "git depth set to 20").\n\n'

deep="$work/deep"
mkdir -p "$deep"
(
  cd "$deep"
  git_quiet init -q .
  commit_file "docs/plans/root.md" "root" "common base"
  git_quiet checkout -q -b feature
  i=1
  while [ "$i" -le 40 ]; do
    commit_file "docs/plans/f$i.md" "f$i" "feature $i"
    i=$((i + 1))
  done
  git_quiet checkout -q main
  i=1
  while [ "$i" -le 40 ]; do
    commit_file "docs/plans/m$i.md" "m$i" "main $i"
    i=$((i + 1))
  done
) >/dev/null 2>&1

shallow="$work/shallow"
# --no-single-branch is REQUIRED and is easy to get wrong: `--depth`
# implies `--single-branch`, which narrows the fetch refspec so a later
# `fetch origin main` writes only FETCH_HEAD and never creates
# refs/remotes/origin/main. Without this flag the test fails on rev-parse
# instead of on the missing merge base -- the right answer for the wrong
# reason, which is exactly the class of false evidence this suite exists to
# avoid producing. GitLab's runner fetches with an explicit
# +refs/heads/*:refs/remotes/origin/* refspec, so remote-tracking refs do
# exist there; the fixture matches that, leaving the merge base as the ONLY
# thing missing.
git_quiet clone -q --depth 20 --branch feature --no-single-branch "file://$deep" "$shallow" >/dev/null 2>&1
( cd "$shallow" && git_quiet fetch -q --no-tags --depth 20 origin main ) >/dev/null 2>&1
if ! ( cd "$shallow" && git rev-parse --verify --quiet refs/remotes/origin/main >/dev/null ); then
  bad "fixture setup: refs/remotes/origin/main absent -- the shallow test would pass for the wrong reason"
fi

if ( cd "$shallow" && sh -c 'set -eu
      base="$(git rev-parse refs/remotes/origin/main)"
      git diff --name-only -z "$base"...HEAD -- "docs/plans/*.md" \
        | xargs -0r true' ) >/dev/null 2>&1; then
  ok "OLD pattern exits 0 on a shallow clone with no merge base (SILENT PASS, reproduced)"
else
  bad "OLD pattern did NOT exit 0 on the shallow clone -- re-derive the premise"
fi

check_nonzero "NEW cf_require_merge_base fails closed on the same shallow clone" \
  sh -c "cd '$shallow' && . '$contract' && base=\"\$(git rev-parse refs/remotes/origin/main)\" && cf_require_merge_base \"\$base\""

printf '\n== 6. The contract still passes on a healthy range ==\n'
printf '   A gate that only ever fails is not a gate.\n\n'

full="$work/full"
git_quiet clone -q --branch feature "file://$deep" "$full" >/dev/null 2>&1
( cd "$full" && git_quiet fetch -q --no-tags origin main ) >/dev/null 2>&1
check_zero "NEW cf_resolve_base resolves a reachable target ref" \
  sh -c "cd '$full' && . '$contract' && cf_resolve_base main"
check_zero "NEW cf_require_merge_base passes when a merge base exists" \
  sh -c "cd '$full' && . '$contract' && base=\"\$(cf_resolve_base main)\" && cf_require_merge_base \"\$base\""

printf '\n== 7. Deletion handling: plans excluded, coordination surfaced ==\n'
printf '   The two feeds get deliberately DIFFERENT treatment. A single blanket\n'
printf '   filter would remove a block that should exist.\n\n'

del="$work/del"
mkdir -p "$del"
(
  cd "$del"
  git_quiet init -q .
  commit_file "docs/plans/keep.md" "keep" "seed plan"
  commit_file "docs/plans/gone.md" "gone" "seed plan to delete"
  commit_file "docs/coordination/20260101T000000Z-coder-tester-aaaa.md" "record" "seed record"
  git_quiet checkout -q -b work
  git_quiet rm -q docs/plans/gone.md
  git_quiet commit -q -m "revert: remove a plan file"
  commit_file "docs/plans/added.md" "added" "add a plan file"
) >/dev/null 2>&1
del_base="$(cd "$del" && git rev-parse main)"

(
  cd "$del"
  . "$contract"
  cf_changed_files "$del_base" "$work/plans.d" d 'docs/plans/*.md'
  tr '\0' '\n' < "$work/plans.d" > "$work/plans.txt"
) >/dev/null 2>&1

if grep -q 'docs/plans/gone.md' "$work/plans.txt"; then
  bad "deleted plan path still reaches 'plan validate' (this is what blocked MR !2206)"
else
  ok "deleted plan path is excluded from the plan-validate feed"
fi
if grep -q 'docs/plans/added.md' "$work/plans.txt"; then
  ok "added plan path still reaches 'plan validate' (the filter did not blind the feed)"
else
  bad "added plan path was lost -- the filter blinded the gate it repairs"
fi

coord_del="$work/coord-del"
mkdir -p "$coord_del"
(
  cd "$coord_del"
  git_quiet init -q .
  commit_file "docs/coordination/20260101T000000Z-coder-tester-aaaa.md" "record" "seed record"
  git_quiet checkout -q -b work
  git_quiet rm -q docs/coordination/20260101T000000Z-coder-tester-aaaa.md
  git_quiet commit -q -m "delete a coordination record"
) >/dev/null 2>&1
coord_base="$(cd "$coord_del" && git rev-parse main)"

(
  cd "$coord_del"
  . "$contract"
  cf_changed_files "$coord_base" "$work/coord.d" D 'docs/coordination/*.md' \
      ':!docs/coordination/README.md' \
      ':!docs/coordination/feedback-coordination-messages.md'
  tr '\0' '\n' < "$work/coord.d" > "$work/coord.txt"
) >/dev/null 2>&1

if grep -q 'coder-tester-aaaa.md' "$work/coord.txt"; then
  ok "deleted coordination record IS surfaced for the append-only diagnostic"
else
  bad "deleted coordination record was not surfaced -- the append-only block would not fire"
fi

printf '\n== 8. The validators still FIRE after the filter ==\n'
printf '   A filter that silences the validator is worse than the defect it\n'
printf '   repairs. Add and modify are exercised independently.\n\n'

if [ -x "$AGOV_BIN" ] || command -v "$AGOV_BIN" >/dev/null 2>&1; then
  val="$work/val"
  mkdir -p "$val"
  (
    cd "$val"
    git_quiet init -q .
    commit_file "docs/plans/seed.md" "seed" "seed"
    git_quiet checkout -q -b work
    mkdir -p docs/coordination docs/plans
    cp "$fixtures/invalid-coordination.md" docs/coordination/invalid-coordination.md
    cp "$fixtures/invalid-plan.md" docs/plans/invalid-plan.md
    cp "$fixtures/20260101T000000Z-coder-tester-fix1.md" docs/coordination/
    git_quiet add -A
    git_quiet commit -q -m "add one invalid record, one invalid plan, one valid record"
  ) >/dev/null 2>&1
  val_base="$(cd "$val" && git rev-parse main)"

  check_nonzero "ADD: 'coord validate' still rejects an invalid ADDED record through the contract" \
    sh -c "cd '$val' && . '$contract' && cf_changed_files '$val_base' '$work/v1' d 'docs/coordination/*.md' ':!docs/coordination/README.md' && cf_run_on_list '$work/v1' '$AGOV_BIN' coord validate"
  check_nonzero "ADD: 'plan validate' still rejects an invalid ADDED plan through the contract" \
    sh -c "cd '$val' && . '$contract' && cf_changed_files '$val_base' '$work/v2' d 'docs/plans/*.md' && cf_run_on_list '$work/v2' '$AGOV_BIN' plan validate"

  (
    cd "$val"
    git_quiet checkout -q -b modify main
    mkdir -p docs/coordination
    cp "$fixtures/20260101T000000Z-coder-tester-fix1.md" docs/coordination/
    git_quiet add -A && git_quiet commit -q -m "add a valid record"
    # Break a FRONT MATTER field. Appending to the body leaves the record
    # valid -- the first version of this test did exactly that and reported
    # a false failure against the contract.
    sed -i 's/^kind: note$/kind: not-a-real-kind/' docs/coordination/20260101T000000Z-coder-tester-fix1.md
    git_quiet add -A && git_quiet commit -q -m "modify it into an invalid one"
  ) >/dev/null 2>&1
  mod_base="$(cd "$val" && git rev-parse main)"
  check_nonzero "MODIFY: 'coord validate' still rejects a record broken by MODIFICATION" \
    sh -c "cd '$val' && git checkout -q modify && . '$contract' && cf_changed_files '$mod_base' '$work/v3' d 'docs/coordination/*.md' ':!docs/coordination/README.md' && cf_run_on_list '$work/v3' '$AGOV_BIN' coord validate"
else
  printf '  SKIP  validator tests: no agov binary at %s\n' "$AGOV_BIN"
  printf '        (set AGOV_BIN, or build with: cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov)\n'
  fail=$((fail + 1))
  printf '  FAIL  validator tests are REQUIRED evidence and cannot be skipped in CI\n'
fi

printf '\n== Result ==\n'
printf '  %d passed, %d failed\n\n' "$pass" "$fail"
[ "$fail" -eq 0 ]

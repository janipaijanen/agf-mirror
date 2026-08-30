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

# Schema discovery walks up from the current directory to find an AGF root.
# This suite deliberately runs validators inside throwaway repositories that
# have no agf/ tree, so without an explicit path the validator fails with
# Io("No such file or directory") instead of judging content. The default
# tooling/agov wrapper exports this itself, which is why the wrapper-based
# local run passed while the CI job -- which invokes the built binary
# directly -- did not. Review 2shr (GCD-G2-3).
export AGOV_SCHEMA_DIR="${AGOV_SCHEMA_DIR:-$repo_root/agf/schemas}"

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

# rejects_with EXPECTED COMMAND...
# Returns 0 only when COMMAND exits non-zero AND its output contains
# EXPECTED -- a specific governed-content finding emitted by the validator,
# such as a schema violation naming the offending field.
#
# Positive evidence, not a blocklist. Three earlier revisions of this suite
# accepted a bare non-zero exit as proof that a checker rejected content, and
# each time something else was failing instead:
#
#   - a validator that could not find its schema (GCD-G2-3), and
#   - the FEEDER failing before the validator ran at all (GCD-G2-4), which
#     produced no blocklisted string and so read as a clean content
#     rejection.
#
# Enumerating more infrastructure-error strings only relocates the blind
# spot, because the set of ways a composite command can fail early is open.
# Requiring the specific finding the fixture is supposed to provoke closes
# it: no infrastructure failure can forge that string, because only the
# validator emits it, and only for that content.
rejects_with() {
  rw_expect=$1
  shift
  if rw_out="$("$@" 2>&1)"; then
    rw_reason="exited 0"
    return 1
  fi
  case "$rw_out" in
    *"$rw_expect"*) return 0 ;;
    *)
      rw_reason="exited non-zero WITHOUT the expected finding '$rw_expect'; first line: $(printf '%s' "$rw_out" | head -n1)"
      return 1
      ;;
  esac
}

# check_rejects LABEL EXPECTED COMMAND...
check_rejects() {
  label=$1
  expect=$2
  shift 2
  if rejects_with "$expect" "$@"; then
    ok "$label"
  else
    bad "$label ($rw_reason)"
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
  # cf_deleted_files, not cf_changed_files -- this must exercise the exact
  # function the CI job calls, or the test proves something the gate does not
  # actually do.
  cf_deleted_files "$coord_base" "$work/coord.d" 'docs/coordination/*.md' \
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
printf '   repairs. ADD and MODIFY are exercised as genuinely distinct Git\n'
printf '   statuses -- review 13rh (GCD-G2-1) caught the first version of this\n'
printf '   section claiming a MODIFY it never performed.\n\n'

# assert_status RANGE_BASE EXPECTED_STATUS PATH LABEL
# Proves what Git ACTUALLY reports over the range before any conclusion is
# drawn from a validator result. Without this, a test can add a file and edit
# it in the same range, then report a MODIFY result that Git classifies as an
# ADD -- passing for the wrong reason.
assert_status() {
  as_base=$1; as_want=$2; as_path=$3; as_label=$4
  as_got="$(git diff --name-status "$as_base...HEAD" -- "$as_path" | cut -f1)"
  if [ "$as_got" = "$as_want" ]; then
    ok "$as_label (range reports $as_got)"
  else
    bad "$as_label (range reports '${as_got:-<none>}', expected $as_want -- the assertion below would prove the wrong thing)"
  fi
}

if [ -x "$AGOV_BIN" ] || command -v "$AGOV_BIN" >/dev/null 2>&1; then

  # ---- Canary: can the configured validator judge a KNOWN-GOOD file? ----
  # Every rejection assertion below is meaningless if the answer is no. Run
  # it from a throwaway directory, the same context the assertions use, so
  # the canary shares their environment rather than the repo root's.
  canary="$work/canary"
  mkdir -p "$canary"
  cp "$fixtures/20260101T000000Z-coder-tester-fix1.md" "$canary/"
  cp "$fixtures/valid-plan.md" "$canary/"
  check_zero "CANARY: the configured validator accepts a known-good record outside the repo" \
    sh -c "cd '$canary' && '$AGOV_BIN' coord validate 20260101T000000Z-coder-tester-fix1.md"
  check_zero "CANARY: the configured validator accepts a known-good plan outside the repo" \
    sh -c "cd '$canary' && '$AGOV_BIN' plan validate valid-plan.md"

  # ---- ADD: files absent from the base, added invalid on the branch ----
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
    git_quiet add -A
    git_quiet commit -q -m "add one invalid record and one invalid plan"
  ) >/dev/null 2>&1
  val_base="$(cd "$val" && git rev-parse main)"

  ( cd "$val" && assert_status "$val_base" A docs/coordination/invalid-coordination.md \
      "ADD: the range really reports an addition for the record" )
  ( cd "$val" && assert_status "$val_base" A docs/plans/invalid-plan.md \
      "ADD: the range really reports an addition for the plan" )

  check_rejects "ADD: 'coord validate' still rejects an invalid ADDED record through the contract" \
    '#/kind: "not-a-real-kind"' \
    sh -c "cd '$val' && . '$contract' && cf_changed_files '$val_base' '$work/v1' d 'docs/coordination/*.md' ':!docs/coordination/README.md' && cf_run_on_list '$work/v1' '$AGOV_BIN' coord validate"
  check_rejects "ADD: 'plan validate' still rejects an invalid ADDED plan through the contract" \
    '#/status: "not-a-valid-status"' \
    sh -c "cd '$val' && . '$contract' && cf_changed_files '$val_base' '$work/v2' d 'docs/plans/*.md' && cf_run_on_list '$work/v2' '$AGOV_BIN' plan validate"

  # ---- MODIFY: VALID files already in the base, broken on the branch ----
  # The base must already carry them. Adding and then editing inside one
  # range is still an ADD over that range.
  mod="$work/mod"
  mkdir -p "$mod"
  (
    cd "$mod"
    git_quiet init -q .
    mkdir -p docs/coordination docs/plans
    cp "$fixtures/20260101T000000Z-coder-tester-fix1.md" docs/coordination/
    cp "$fixtures/valid-plan.md" docs/plans/valid-plan.md
    git_quiet add -A
    git_quiet commit -q -m "base carries a valid record and a valid plan"
    git_quiet checkout -q -b work
    # Break FRONT MATTER in both. Appending to the body leaves a record
    # valid, because the validator reads the front matter -- the first draft
    # of this test made exactly that mistake.
    sed -i 's/^kind: note$/kind: not-a-real-kind/' docs/coordination/20260101T000000Z-coder-tester-fix1.md
    sed -i 's/^status: pending-gate1$/status: not-a-valid-status/' docs/plans/valid-plan.md
    git_quiet add -A
    git_quiet commit -q -m "break both into invalid files"
  ) >/dev/null 2>&1
  mod_base="$(cd "$mod" && git rev-parse main)"

  ( cd "$mod" && assert_status "$mod_base" M docs/coordination/20260101T000000Z-coder-tester-fix1.md \
      "MODIFY: the range really reports a modification for the record" )
  ( cd "$mod" && assert_status "$mod_base" M docs/plans/valid-plan.md \
      "MODIFY: the range really reports a modification for the plan" )

  check_rejects "MODIFY: 'coord validate' still rejects a record broken by MODIFICATION" \
    '#/kind: "not-a-real-kind"' \
    sh -c "cd '$mod' && . '$contract' && cf_changed_files '$mod_base' '$work/v3' d 'docs/coordination/*.md' ':!docs/coordination/README.md' && cf_run_on_list '$work/v3' '$AGOV_BIN' coord validate"
  check_rejects "MODIFY: 'plan validate' still rejects a plan broken by MODIFICATION" \
    '#/status: "not-a-valid-status"' \
    sh -c "cd '$mod' && . '$contract' && cf_changed_files '$mod_base' '$work/v4' d 'docs/plans/*.md' && cf_run_on_list '$work/v4' '$AGOV_BIN' plan validate"

  printf '\n== 9. Rename-and-rewrite must not bypass the append-only guard ==\n'
  printf "   Git reports a rename as R, not D. A guard built on --diff-filter=D\n"
  printf '   alone sees nothing while the original record disappears and a\n'
  printf '   validly-shaped destination passes the validator. Review 13rh\n'
  printf '   (GCD-G2-2) demonstrated this bypass.\n\n'

  ren="$work/ren"
  mkdir -p "$ren"
  (
    cd "$ren"
    git_quiet init -q .
    mkdir -p docs/coordination
    cp "$fixtures/20260101T000000Z-coder-tester-fix1.md" docs/coordination/
    git_quiet add -A
    git_quiet commit -q -m "base carries a published record"
    git_quiet checkout -q -b work
    git_quiet mv docs/coordination/20260101T000000Z-coder-tester-fix1.md \
                 docs/coordination/20260101T000001Z-coder-tester-fix2.md
    # Make the DESTINATION independently valid, so nothing else objects.
    sed -i 's/20260101T000000Z-coder-tester-fix1/20260101T000001Z-coder-tester-fix2/; s/2026-01-01T00:00:00Z/2026-01-01T00:00:01Z/' \
      docs/coordination/20260101T000001Z-coder-tester-fix2.md
    git_quiet add -A
    git_quiet commit -q -m "rename the record and rewrite its identity"
  ) >/dev/null 2>&1
  ren_base="$(cd "$ren" && git rev-parse main)"

  ren_status="$(cd "$ren" && git diff --name-status "$ren_base...HEAD" -- 'docs/coordination/*.md' | cut -f1 | cut -c1)"
  if [ "$ren_status" = "R" ]; then
    ok "the range really reports a rename (R), not a delete"
  else
    bad "fixture setup: range reports '${ren_status:-<none>}', not R -- the bypass below would not be the one under test"
  fi

  # The destination is valid on its own, which is what makes the bypass work.
  check_zero "the renamed destination is itself valid, so nothing else objects" \
    sh -c "cd '$ren' && '$AGOV_BIN' coord validate docs/coordination/20260101T000001Z-coder-tester-fix2.md"

  # The OLD D-only query: reproduce the blindness.
  ( cd "$ren" && git diff --name-only -z --diff-filter=D "$ren_base...HEAD" -- 'docs/coordination/*.md' > "$work/d-only" ) >/dev/null 2>&1
  if [ -s "$work/d-only" ]; then
    bad "OLD D-only query saw the rename -- the premise of GCD-G2-2 no longer holds; re-derive it"
  else
    ok "OLD D-only query returns EMPTY on a rename (the bypass, reproduced)"
  fi

  # The NEW rename-aware query must surface the vanished source path.
  ( cd "$ren" && . "$contract" && cf_deleted_files "$ren_base" "$work/ren-del" 'docs/coordination/*.md' ) >/dev/null 2>&1
  if grep -q 'coder-tester-fix1.md' "$work/ren-del" 2>/dev/null; then
    ok "NEW cf_deleted_files surfaces the vanished source path, so the guard fires"
  else
    bad "NEW cf_deleted_files missed the rename -- the append-only guard is still bypassable"
  fi
else
  printf '  SKIP  validator tests: no agov binary at %s\n' "$AGOV_BIN"
  printf '        (set AGOV_BIN, or build with: cargo build --release --manifest-path tooling/rust/Cargo.toml -p agov)\n'
  fail=$((fail + 1))
  printf '  FAIL  validator tests are REQUIRED evidence and cannot be skipped in CI\n'
fi

printf '\n== 10. The rejection control must itself fail on a feeder failure ==\n'
printf '   Every assertion in section 8 is a COMPOSITE command: derive the\n'
printf '   changed-file feed, then run the validator. If the feeder fails, the\n'
printf '   command exits non-zero before the validator ever runs. A control\n'
printf '   that infers "content was rejected" from a non-zero exit reports that\n'
printf '   infrastructure failure as a pass. Review hna4 (GCD-G2-4) constructed\n'
printf '   exactly that case against the previous blocklist implementation.\n\n'

if [ -x "$AGOV_BIN" ] || command -v "$AGOV_BIN" >/dev/null 2>&1; then
  # A composite assertion whose FEEDER fails: the base does not resolve, so
  # cf_changed_files fails closed and the validator is never reached.
  broken_feeder="sh -c \"cd '$val' && . '$contract' && cf_changed_files does-not-resolve '$work/mf' d 'docs/coordination/*.md' && cf_run_on_list '$work/mf' '$AGOV_BIN' coord validate\""

  # Confirm the shape: it really does exit non-zero without reaching the
  # validator. Without this the meta-test below could pass vacuously.
  if eval "$broken_feeder" >/dev/null 2>&1; then
    bad "meta: the broken-feeder command exited 0 -- it is not the case under test"
  else
    ok "meta: the broken-feeder command exits non-zero before the validator runs"
  fi

  # THE CONTROL ITSELF must reject that as evidence of content rejection.
  if eval "rejects_with '#/kind: \"not-a-real-kind\"' $broken_feeder"; then
    bad "check_rejects ACCEPTED a feeder failure as a content rejection -- GCD-G2-4 is not closed"
  else
    ok "check_rejects REFUSES a feeder failure as a content rejection (GCD-G2-4 closed)"
  fi

  # And it must still accept a genuine rejection, or the control is merely
  # strict rather than correct.
  if eval "rejects_with '#/kind: \"not-a-real-kind\"' sh -c \"cd '$val' && . '$contract' && cf_changed_files '$val_base' '$work/mg' d 'docs/coordination/*.md' ':!docs/coordination/README.md' && cf_run_on_list '$work/mg' '$AGOV_BIN' coord validate\""; then
    ok "check_rejects still ACCEPTS a genuine content rejection"
  else
    bad "check_rejects rejected a genuine content rejection -- the control is too strict to be useful"
  fi
else
  bad "meta-tests for check_rejects require an agov binary"
fi

printf '\n== Result ==\n'
printf '  %d passed, %d failed\n\n' "$pass" "$fail"
[ "$fail" -eq 0 ]

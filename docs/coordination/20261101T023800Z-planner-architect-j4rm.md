---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T023800Z-planner-architect-j4rm
timestamp: 2026-11-01T02:38:17Z
model: claude
immutable: true
references-to:
  - 20261101T022418Z-governor-reviewer-573t.md
  - 20261101T021727Z-planner-architect-3bq1.md
topic: s1-calibration-script-corrected-573t
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Narrow delta review of the two calibration corrections in 573t. I authorize and run nothing; no unit, no browser and no probe has run.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md
  do_not_touch:
    - agf/docs/site/overrides/main.html
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1 calibration script: floor and start-failure corrected (answers 573t)

Only the calibration script and its tests changed. The launcher, the observer, the matrix and their tests are byte-identical to 3bq1 (see the table), so the 573t clearance of the launcher logic carries unchanged. Preserved: zero swap, `MemoryMax=64M`, four-second unit cap, five pre-registered cases and expected facts, exact identity and process evidence, no retry. A mismatch is still reported and never accepted. I ran nothing.

## The two corrections

1. **Per-case host floor.** Before every case, `s1-qa-calibrate.sh` reads MemAvailable (from `/proc/meminfo`, with a test seam) and stops with exit 3 if it is unreadable, not a number, or below 1,024 MiB. It starts no unit in that case.
2. **A start failure ends the run.** If `systemd-run` fails, the script saves the unit's state (`.show`), and if the unit exists (a LoadState other than `not-found`) it runs `stop` and `reset-failed`, saves the after-state (`.after`) and the exact cgroup inventory (`.procs-after`, or `unavailable`), prints where the evidence is, and exits 4 without starting any later case.

## Pure negatives (new `s1-qa-calibrate-selftest.sh`; fake `systemd-run` and `systemctl`, no unit)

Ten checks, all as expected: all five cases match; 500 MiB stops before any unit starts; 1,023 MiB stops (boundary) and exactly 1,024 MiB is allowed; an unreadable MemAvailable stops; the floor is checked before every case (it drops after case 2, so case 3 never starts: exactly 2 units); a start failure at case 2 stops with exit 4 and exactly 2 units started, with its state kept; a start failure on a unit that exists is cleaned and its state kept; a mismatch gives exit 1 with a `MISMATCH` line. **Mutation check:** with the floor line removed, six assertions fail; with the start-failure `exit 4` turned into a `return` (later cases continue), the start-failure assertions fail with five units started. So the fixtures see both defects. The launcher's own 19 scenarios still pass on the corrected S1 artifact (job 17638588689).

## Files

| File | Bytes | SHA-256 | Status |
|---|---|---|---|
| `s1-qa-matrix.json` | 2600 | `15b986b8a1076f95e7f8533c8f193104211f388256f41ab14b015ec3c7b05293` | byte-identical to 3bq1 |
| `s1-qa-checks.cjs` | 2513 | `780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b` | byte-identical to 3bq1 |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` | byte-identical to 3bq1 |
| `s1-qa-observe.cjs` | 5975 | `c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4` | byte-identical to 3bq1 |
| `s1-qa-selftest.cjs` | 4304 | `ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67` | byte-identical to 3bq1 |
| `s1-qa-run.sh` | 9777 | `fc81c7a762fc08d1b08213aa8d5b195f8017a7b862ad37a83a9fe09e1b85a4bd` | byte-identical to 3bq1 |
| `s1-qa-run-selftest.sh` | 6616 | `3c093e64dad15a19facf1e339e6b3f34f78f1d45fc0a95c6ef785e464e80aaaa` | byte-identical to 3bq1 |
| `s1-qa-calibrate.sh` | 6939 | `1a44a2331d4b0c41b3cb704bb6a5bfa19dcf4d4750fa5bba509587a35ecde79c` | changed or new (embedded below) |
| `s1-qa-calibrate-selftest.sh` | 4770 | `589b975a6f4dfb83906db451414befbadefe185f055816882b26d00df3ee9f12` | changed or new (embedded below) |
| `s1-qa-hashes.sha256` | 764 | `10feaf4e193c2f247d75c366c7a9a94caeaa1924d3a24bb2ce90cc07ad1392c2` | changed or new (embedded below) |

`s1-qa-hashes.sha256` lists the other nine files. Extract the three embedded blocks verbatim (each line ends in a newline) next to the seven unchanged files from 3bq1; run with `bash`.

## Not tested, and limits

The fakes check control flow only; the five real cases and the systemd assumptions are untested (that is what the calibration is for). The calibration is not authorized, and it tests nothing about Chromium or browser memory sizing. A future S1 capture must bind head `30203a5e` and its artifact (job 17638588689, ZIP `b821f6042799786f98901335d687f6b5f72c4f49b84e0bc7725ac13cbe4368e5`), not the qpg7 inputs. This closes nothing for R1, R2 or Gate 2.

### s1-qa-calibrate.sh

~~~ s1-qa-calibrate.sh
#!/usr/bin/env bash
# Browserless real-unit calibration of the launcher's systemd assumptions. NOT authorized by itself: it needs root's separate decision.
# It starts five tiny named transient units (Node one-liners, no browser, no server, no network) with the launcher's unit profile,
# except MemoryMax=64M and RuntimeMaxSec=4, and compares what systemd reports with expectations written here BEFORE any run.
# A mismatch is a finding, not a reason to retry or to loosen the launcher. usage: bash s1-qa-calibrate.sh OUT_DIR   (empty or absent)
set -uo pipefail
OUT="${1:?output dir}"; case "$OUT" in /*) ;; *) OUT="$PWD/$OUT" ;; esac
[ ! -e "$OUT" ] || [ -z "$(ls -A "$OUT" 2>/dev/null)" ] || { echo "output dir must be empty" >&2; exit 2; }
mkdir -p "$OUT"; NODE_BIN="${NODE_BIN:-node}"; SYSTEMD_RUN="${SYSTEMD_RUN:-systemd-run}"; SYSTEMCTL="${SYSTEMCTL:-systemctl}"; CGROOT="${CGROOT:-/sys/fs/cgroup}"; MEMINFO="${MEMINFO:-/proc/meminfo}"; FLOOR_MIB=1024
RUNID="$(date -u +%Y%m%dT%H%M%S)-$$"; BAD=0
[ -z "$($SYSTEMCTL --user list-units --all --no-legend 'agf-s1qa-*' 2>/dev/null | awk 'NF{print $1}')" ] || { echo "existing agf-s1qa unit" >&2; exit 6; }
get() { sed -n "s/^$2=//p" "$1" | head -1; }
valid_cg() { case "$1" in /*) ;; *) return 1 ;; esac; case "$1/" in */../*|*//*) return 1 ;; esac; [ "${1##*/}" = "$2" ]; }
inv() { if [ -e "$CGROOT$1/cgroup.procs" ]; then cat "$CGROOT$1/cgroup.procs"; else echo absent; fi; }
# case NAME WANT_RESULT WANT_EXEC_STATUS(or -) WANT_ACTIVE/SUB WANT_PROCS_BEFORE(empty|nonempty) -- node args...
run_case() {
  local name="$1" wres="$2" wst="$3" wstate="$4" wprocs="$5"; shift 6
  local UNIT="agf-s1qa-cal-$RUNID-$name" F=""; local SVC="$UNIT.service" P="$OUT/$name"; local why=""
  # Host floor before every case, fail closed: no unit starts below 1 GiB of MemAvailable.
  local avail; avail="$(awk '/MemAvailable/{print int($2/1024)}' "$MEMINFO")"
  case "$avail" in ''|*[!0-9]*) echo "STOP: MemAvailable unreadable before case $name" >&2; exit 3 ;; esac
  [ "$avail" -ge "$FLOOR_MIB" ] || { echo "STOP: host MemAvailable ${avail} MiB below floor ${FLOOR_MIB} MiB before case $name" >&2; exit 3; }
  if ! "$SYSTEMD_RUN" --user --unit="$UNIT" --no-block --quiet -p MemoryMax=64M -p MemorySwapMax=0 -p OOMPolicy=kill -p RuntimeMaxSec=4 -p RemainAfterExit=yes \
    -p StandardOutput="file:$P.stdout" -p StandardError="file:$P.stderr" "$@" > "$P.start" 2>&1; then
    # A failed start ends the calibration. Keep the exact state of this owned unit, clean it if it exists, and start nothing else.
    "$SYSTEMCTL" --user show "$SVC" -p Id,InvocationID,LoadState,Result,ActiveState,SubState,ExecMainStatus,ControlGroup > "$P.show" 2>/dev/null
    local cl="$(get "$P.show" LoadState)" cc="$(get "$P.show" ControlGroup)"
    if [ -n "$cl" ] && [ "$cl" != not-found ]; then
      "$SYSTEMCTL" --user stop "$SVC" > "$P.cleanup" 2>&1; "$SYSTEMCTL" --user reset-failed "$SVC" >> "$P.cleanup" 2>&1
    fi
    "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$P.after" 2>/dev/null
    if valid_cg "$cc" "$SVC"; then inv "$cc" > "$P.procs-after"; else echo unavailable > "$P.procs-after"; fi
    echo "STOP: could not start $UNIT: $(cat "$P.start"). State kept in $P.show, $P.cleanup, $P.after, $P.procs-after. No later case was started." >&2
    exit 4
  fi
  local CGRUN="" end=$(( $(date +%s) + 15 )) fin=0
  while [ "$(date +%s)" -le "$end" ]; do
    "$SYSTEMCTL" --user show "$SVC" -p ActiveState,SubState,InvocationID,ControlGroup > "$P.poll" 2>/dev/null
    local a s i c; a="$(get "$P.poll" ActiveState)"; s="$(get "$P.poll" SubState)"; i="$(get "$P.poll" InvocationID)"; c="$(get "$P.poll" ControlGroup)"
    [ -n "$c" ] && [ -z "$CGRUN" ] && CGRUN="$c"
    if [ -n "$i" ] && { [ "$a" = inactive ] || [ "$a" = failed ] || { [ "$a" = active ] && [ "$s" = exited ]; }; }; then fin=1; break; fi
    sleep 0.1
  done
  "$SYSTEMCTL" --user show "$SVC" -p Id,InvocationID,LoadState,Result,ActiveState,SubState,ExecMainStatus,ExecMainCode,MemoryPeak,ControlGroup > "$P.show" 2>/dev/null
  local CG; CG="$(get "$P.show" ControlGroup)"; CG="${CG:-$CGRUN}"
  { echo "cg_while_running=$CGRUN"; echo "cg_terminal=$(get "$P.show" ControlGroup)"; } > "$P.scope"
  [ "$fin" = 1 ] || why="$why not-finished;"
  [ "$(get "$P.show" Id)" = "$SVC" ] || why="$why id;"
  [ -n "$(get "$P.show" InvocationID)" ] || why="$why empty-invocation;"
  [ "$(get "$P.show" LoadState)" = loaded ] || why="$why not-loaded;"
  valid_cg "$CG" "$SVC" || why="$why cgroup-invalid('$CG');"
  [ "$(get "$P.show" Result)" = "$wres" ] || why="$why result=$(get "$P.show" Result) want $wres;"
  [ "$wst" = - ] || [ "$(get "$P.show" ExecMainStatus)" = "$wst" ] || why="$why status=$(get "$P.show" ExecMainStatus) want $wst;"
  [ "$(get "$P.show" ActiveState)/$(get "$P.show" SubState)" = "$wstate" ] || why="$why state=$(get "$P.show" ActiveState)/$(get "$P.show" SubState) want $wstate;"
  case "$(get "$P.show" MemoryPeak)" in ''|*[!0-9]*) why="$why memorypeak-unavailable;" ;; esac
  if valid_cg "$CG" "$SVC"; then inv "$CG" > "$P.procs-before"; else echo unavailable > "$P.procs-before"; fi
  local pb; pb="$(cat "$P.procs-before")"
  case "$wprocs" in empty) case "$pb" in ""|absent) ;; *) why="$why procs-before=$pb want none;" ;; esac;; nonempty) case "$pb" in ""|absent|unavailable) why="$why procs-before empty want some;" ;; esac;; esac
  "$SYSTEMCTL" --user stop "$SVC" > "$P.cleanup" 2>&1 || why="$why stop-failed;"
  "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$P.after" 2>/dev/null
  [ "$(get "$P.after" ActiveState)" != failed ] || { "$SYSTEMCTL" --user reset-failed "$SVC" >> "$P.cleanup" 2>&1 || why="$why reset-failed-failed;"; "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$P.after" 2>/dev/null; }
  { [ "$(get "$P.after" LoadState)" = not-found ] || [ "$(get "$P.after" ActiveState)" = inactive ]; } || why="$why not-clean-after-stop;"
  if valid_cg "$CG" "$SVC"; then inv "$CG" > "$P.procs-after"; else echo unavailable > "$P.procs-after"; fi
  case "$(cat "$P.procs-after")" in ""|absent) ;; *) why="$why procs-after=$(cat "$P.procs-after");" ;; esac
  if [ -z "$why" ]; then echo "MATCH $name (as pre-registered)"; else echo "MISMATCH $name:$why"; BAD=$((BAD+1)); fi
}
# Pre-registered expectations. Result values are systemd's documented names; any other value is a finding.
run_case ok        success   0 active/exited empty    -- "$NODE_BIN" -e 'console.log(1)'
run_case exit3     exit-code 3 failed/failed empty    -- "$NODE_BIN" -e 'process.exit(3)'
run_case timeout   timeout   - failed/failed empty    -- "$NODE_BIN" -e 'setTimeout(()=>{},60000)'
run_case oom       oom-kill  - failed/failed empty    -- "$NODE_BIN" --max-old-space-size=512 -e 'const a=[];for(;;)a.push(Buffer.alloc(8e6,1))'
run_case leftover  success   0 active/exited nonempty -- bash -c 'sleep 30 & echo started'
echo "calibration: $BAD mismatch(es). Nothing was retried."; exit $((BAD ? 1 : 0))
~~~

### s1-qa-calibrate-selftest.sh

~~~ s1-qa-calibrate-selftest.sh
#!/usr/bin/env bash
# Offline test of the calibration script's control flow with fake systemd-run and systemctl. No unit, no browser, no root.
# usage: bash s1-qa-calibrate-selftest.sh      Exit 0 only if every scenario ends as expected.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
mkdir -p "$T/bin" "$T/state" "$T/cg"
cat > "$T/bin/systemd-run" <<'FAKE'
#!/usr/bin/env bash
UNIT=""; for a in "$@"; do case "$a" in --unit=*) UNIT="${a#--unit=}";; esac; done
N=$(( $(cat "$FAKE_STATE/calls" 2>/dev/null || echo 0) + 1 )); echo $N > "$FAKE_STATE/calls"
echo "$UNIT" >> "$FAKE_STATE/units"
[ "$FAKE_SCENARIO" = "dropfloor" ] && [ "$N" = 2 ] && echo "MemAvailable: 100000 kB" > "$MEMINFO"
if [ "$FAKE_SCENARIO" = "startfail" ] || [ "$FAKE_SCENARIO" = "startfailloaded" ]; then
  if [ "$N" = 2 ]; then
    echo "Failed to start transient service unit" >&2
    [ "$FAKE_SCENARIO" = "startfailloaded" ] && printf 'Id=%s.service\nLoadState=loaded\nActiveState=failed\nControlGroup=/user.slice/%s.service\n' "$UNIT" "$UNIT" > "$FAKE_STATE/$UNIT.show"
    exit 1
  fi
fi
case "$UNIT" in
  *-ok) RES=success; ST=0; ACT=active; SUB=exited;;
  *-exit3) RES=exit-code; ST=3; ACT=failed; SUB=failed;;
  *-timeout) RES=timeout; ST=0; ACT=failed; SUB=failed;;
  *-oom) RES=oom-kill; ST=0; ACT=failed; SUB=failed;;
  *-leftover) RES=success; ST=0; ACT=active; SUB=exited; mkdir -p "$FAKE_CGROOT/user.slice/$UNIT.service"; echo 777 > "$FAKE_CGROOT/user.slice/$UNIT.service/cgroup.procs"; echo "$FAKE_CGROOT/user.slice/$UNIT.service/cgroup.procs" > "$FAKE_STATE/$UNIT.procfile";;
esac
[ "$FAKE_SCENARIO" = "mismatch" ] && case "$UNIT" in *-exit3) RES=success; ST=0; ACT=active; SUB=exited;; esac
printf 'Id=%s.service\nInvocationID=abc\nLoadState=loaded\nResult=%s\nActiveState=%s\nSubState=%s\nExecMainStatus=%s\nExecMainCode=1\nMemoryPeak=1000\nControlGroup=/user.slice/%s.service\n' "$UNIT" "$RES" "$ACT" "$SUB" "$ST" "$UNIT" > "$FAKE_STATE/$UNIT.show"
exit 0
FAKE
cat > "$T/bin/systemctl" <<'FAKE'
#!/usr/bin/env bash
case "$2" in
  list-units) exit 0;;
  show) u="${3%.service}"
        if [ -f "$FAKE_STATE/$u.stopped" ]; then printf 'LoadState=not-found\nActiveState=inactive\n'; exit 0; fi
        [ -f "$FAKE_STATE/$u.show" ] && cat "$FAKE_STATE/$u.show"; exit 0;;
  stop) u="${3%.service}"; : > "$FAKE_STATE/$u.stopped"; [ -f "$FAKE_STATE/$u.procfile" ] && : > "$(cat "$FAKE_STATE/$u.procfile")"; exit 0;;
  *) exit 0;;
esac
FAKE
chmod +x "$T/bin/"*
bad=0
run() { # name want_rc scenario memkb
  local name="$1" want="$2" sc="$3" kb="$4"
  rm -rf "$T/out" "$T/state"/* "$T/cg"/*; mkdir -p "$T/state"
  if [ "$kb" = unreadable ]; then : > "$T/meminfo"; else echo "MemAvailable: $kb kB" > "$T/meminfo"; fi
  FAKE_SCENARIO="$sc" FAKE_STATE="$T/state" FAKE_CGROOT="$T/cg" CGROOT="$T/cg" MEMINFO="$T/meminfo" PATH="$T/bin:$PATH" \
    SYSTEMD_RUN="$T/bin/systemd-run" SYSTEMCTL="$T/bin/systemctl" bash "$HERE/s1-qa-calibrate.sh" "$T/out" > "$T/run.log" 2>&1
  local rc=$? ok=FAIL; [ "$rc" = "$want" ] && ok=PASS; [ "$ok" = FAIL ] && bad=$((bad+1))
  echo "$ok $name (exit $rc, want $want; units started: $(cat "$T/state/calls" 2>/dev/null || echo 0))"
  [ "$ok" = FAIL ] && tail -2 "$T/run.log" | sed 's/^/     log: /'
}
calls() { cat "$T/state/calls" 2>/dev/null || echo 0; }
expect_calls() { [ "$(calls)" = "$1" ] && echo "     started exactly $1 unit(s)" || { echo "FAIL     expected $1 units started, saw $(calls)"; bad=$((bad+1)); }; }
run "all five cases match their pre-registered facts" 0 ok 4000000
run "host floor: 500 MiB stops before any unit starts" 3 ok 512000; expect_calls 0
run "host floor: 1023 MiB stops (boundary)" 3 ok 1047552; expect_calls 0
run "host floor: exactly 1024 MiB is allowed" 0 ok 1048576
run "host floor: unreadable MemAvailable stops" 3 ok unreadable; expect_calls 0
run "host floor is checked before EVERY case: it drops after case 2, so case 3 never starts" 3 dropfloor 4000000; expect_calls 2
run "a start failure stops the run and starts no later case" 4 startfail 4000000; expect_calls 2
echo "     evidence kept: $(ls "$T/out" 2>/dev/null | grep -c 'procs-after') procs-after file(s), $(ls "$T/out" 2>/dev/null | grep -c '\.show') show file(s)"
run "a start failure for a unit that exists cleans it and keeps its state" 4 startfailloaded 4000000; expect_calls 2
[ -s "$T/out/exit3.cleanup" ] || [ -f "$T/out/exit3.cleanup" ] && echo "     cleanup attempted for the failed-start unit" || { echo "FAIL     no cleanup record for the failed-start unit"; bad=$((bad+1)); }
run "a mismatch is reported and never accepted" 1 mismatch 4000000
grep -q "^MISMATCH exit3" "$T/run.log" && echo "     MISMATCH line present" || { echo "FAIL     no MISMATCH line"; bad=$((bad+1)); }
exit $((bad ? 1 : 0))
~~~

### s1-qa-hashes.sha256

~~~ s1-qa-hashes.sha256
15b986b8a1076f95e7f8533c8f193104211f388256f41ab14b015ec3c7b05293  s1-qa-matrix.json
780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b  s1-qa-checks.cjs
40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06  s1-qa-serve.cjs
c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4  s1-qa-observe.cjs
ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67  s1-qa-selftest.cjs
fc81c7a762fc08d1b08213aa8d5b195f8017a7b862ad37a83a9fe09e1b85a4bd  s1-qa-run.sh
3c093e64dad15a19facf1e339e6b3f34f78f1d45fc0a95c6ef785e464e80aaaa  s1-qa-run-selftest.sh
1a44a2331d4b0c41b3cb704bb6a5bfa19dcf4d4750fa5bba509587a35ecde79c  s1-qa-calibrate.sh
589b975a6f4dfb83906db451414befbadefe185f055816882b26d00df3ee9f12  s1-qa-calibrate-selftest.sh
~~~

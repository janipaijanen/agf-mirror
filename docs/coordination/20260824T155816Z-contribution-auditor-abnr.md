---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260824T155816Z-contribution-auditor-abnr
timestamp: 2026-08-24T15:58:16Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260824T153544Z-planner-architect-k2dj.md
  - 20260824T145655Z-contribution-auditor-s6sm.md
  - 20260824T155125Z-contribution-auditor-jmd4.md
  - 20260824T071103Z-governor-reviewer-ekns.md
  - 20260824T142545Z-governor-reviewer-kqm3.md
topic: mr1965-d24f02ad-round2-complete-audit
requires-ack: false
artifacts_owned: []
---

# MR !1965 round-2 contribution audit: COMPLETE / BLOCK

Scope: GitLab MR !1965 (`id: 523111786`) at exact source head
`d24f02ad1ab1c12599869f709f3c9140b864a2cf`, targeting `main`; target
head observed as `3a62ec9296f8eb6d2597c8f9c0feae5abdd17ac5`, with merge base
`e1ae69ec40767951a3cb2034cf2bc21ddfb616fe`.
Coverage: COMPLETE for the contribution-auditor's authorized guidance-
integrity scope. The implementation diff and CI evidence were inspected as
supporting evidence for claims in the MR description and handoff, but this
record does not claim an independent-implementation-reviewer conclusion; see
`CA-ADV-R2-1`.
Recommendation: BLOCK
Blocking findings: 4
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 2.

Reviewer provenance: genuinely fresh Codex session, model
`codex-gpt-5`; no candidate authorship, editing, or commit activity.
Known implementer provenance: planner-architect/coder work was handed off by
Claude (`claude-sonnet-5` in `k2dj`); candidate commits are authored and
committed by Jani Päijänen. The review used a detached read-only worktree.
While this audit was running, concurrent contribution-auditor review `jmd4`
published independently and reached the same sub-hour-future blocker described
below. Its merge advanced `origin/main` to `c6172c7ca5183efbede88d7cde92bb864fb42c71`
after the target observation above; the advance is coordination-only and does
not materially alter the candidate comparison or mergeability.

## CA-ADV-R2-1 — the handoff assigns exact-head implementation review to the wrong role

Check: role jurisdiction and review identity
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: critical
Enforcement: blocking
Basis: `agf/agents/contribution-auditor.yaml` jurisdiction boundary and
`agf/agents/independent-implementation-reviewer.yaml`
Finding state: open
Risk disposition: none

Handoff `k2dj` asks `contribution-auditor` to perform a COMPLETE independent
audit of the exact implementation, tests, and pipeline before Gate 2. The
active contribution-auditor definition says its jurisdiction is contribution
and governance guidance, explicitly **never** the exact-head implementation
diff, tests, or pipeline evidence, and directs that work to the distinct
`independent-implementation-reviewer` role. That role is active in
`agf/agents/MANIFEST.yaml`. The round-1 record `s6sm` was itself titled an
"exact-head implementation audit" under the contribution-auditor role, so its
label does not satisfy the role boundary merely because its technical evidence
was useful.

Concrete failure scenario: accepting this record or `s6sm` as the named
implementation review recreates the jurisdiction-blurring defect the
independent role was introduced to eliminate; a cold reviewer sees the wrong
authority and cannot establish which role contract governed coverage.

Resolution condition: route the exact-head implementation review through the
active `independent-implementation-reviewer` under its provenance and output
contract, or obtain a prior binding governor decision changing the applicable
role authority. Do not relabel a contribution-auditor record post hoc.
Assumed intent: obtain genuinely independent pre-Gate-2 evidence while keeping
the governor as the only gate authority.

## CA-ADV-R2-2 — sub-hour future timestamps still suppress the mismatch

Check: round-1 `CA-ADV-R1-1` bounded-window resolution
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: approved plan's bounded mid-upgrade lifecycle and `s6sm` resolution
condition
Finding state: open
Risk disposition: none

The fix computes `(now - started).whole_hours()` and tests whether the integer
is in `0..24`. `whole_hours()` truncates a negative sub-hour duration to zero,
so a timestamp modestly in the future is classified as inside the window. A
disposable CLI probe used a live mismatched record with
`upgrade_in_progress: true` and `upgrade_started_at` 30 minutes in the future;
the exact-head release binary returned exit `0`, `OK` for the file, and no
malformed-future advisory. The added future test uses `now + 48h`, so it does
not cover the truncation boundary.

Concrete failure scenario: ordinary clock skew or an incorrectly entered
future time under one hour suppresses a real version mismatch, contradicting
the promise that every future marker is non-suppressing and visible.

Resolution condition: compare the untruncated timestamp/duration to `now`
before converting to whole hours (or use direct duration bounds), and add a
future-sub-hour regression test that requires the mismatch plus malformed-
future advisory.
Assumed intent: suppress only timestamps in the closed-open interval
`now - 24h < started_at <= now`, never any future timestamp.

## CA-ADV-R2-3 — the mandatory negative-test CI artifact is still absent

Check: round-1 `CA-ADV-R1-3` and Gate self-test evidence
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: approved plan's Gate 2 acceptance requirement and
`agf/rules/always/gate-self-test-rule.md`
Finding state: open
Risk disposition: none

The new CLI integration test is a meaningful improvement and does run in
exact-head job `16072389209`. However, the MR description still says the job
link will be filled once the pipeline completes; pipeline `2786032886` is
already successful and no link was filled. More importantly, the job trace
shows only
`test validate_learnings_blocks_live_record_version_drift_via_cli ... ok`.
It does not show the deliberately bad input's actual blocking output or exit
status. The plan requires a link to the CI job log where the exact negative
demonstration ran; the rule requires a visible verification artifact, not an
assertion hidden inside a passing test.

Resolution condition: make the exact-head acceptance artifact visibly contain
the bad input, actual blocking message, and exit `1`, and link that specific
job/artifact from the MR description. Preserve the integration test.
Assumed intent: prove to a reviewer that the shipped CLI gate visibly blocks
the known-bad record, not merely that an opaque test returned success.

## CA-ADV-R2-4 — the CI correction expands scope and crosses build ownership without approval

Check: approved scope, role ownership, and MR-description accuracy
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Impact: material
Enforcement: blocking
Basis: Gate 1 scope discipline, `agf/agents/MANIFEST.yaml` build-engineer
ownership, and the approved plan's "Files expected to change"
Finding state: open
Risk disposition: none

Commit `d24f02ad` changes `.gitlab-ci.yml` by setting `GIT_DEPTH: "0"` on
`rust:tooling:test`. The approved plan does not list CI configuration among
expected implementation paths, and the MR description still states both
"no change to CI wiring" and "No CI pipeline configuration changes." Its
changed-file list also omits `.gitlab-ci.yml`, the new CLI integration-test
file, and `Cargo.lock`. No published build-engineer consultation or governor
scope-expansion decision was found. The project manifest assigns CI
configuration, pipeline efficiency, and runner usage to build-engineer.

The technical diagnosis is supported: project default depth is 20, the failed
historical-corpus fixture required older history, and exact-head pipeline
`2786032886` / job `16072389209` now pass. Technical success does not authorize
an unplanned full-history CI policy change or erase its runner-cost impact.

Resolution condition: obtain the required governor scope disposition and
build-engineer review for the CI change (including cost/efficiency), or use an
approved in-scope correction that avoids CI-configuration expansion; then make
the MR scope and changed-file inventory accurate.
Assumed intent: keep the required historical-corpus regression meaningful and
green without imposing undocumented CI behavior or ownership changes.

## Round-1 reconciliation

- `CA-ADV-R1-1`: **open**, narrowed but not resolved; see `CA-ADV-R2-2`.
- `CA-ADV-R1-2`: **resolved**. The schema conditionally requires a non-empty
  `framework_version` for `checkout_role: live`; semantic validation also
  blocks the missing value and an unreadable `agf/VERSION`.
- `CA-ADV-R1-3`: **open**, partially implemented but evidence contract not
  met; see `CA-ADV-R2-3`.
- `CA-ADV-R1-4`: **technical failure resolved**. Exact-head pipeline
  `2786032886` is successful and job `16072389209` reports both the historical
  corpus test and 140-test `agov` suite passing. Its remediation introduces
  the new scope/ownership finding `CA-ADV-R2-4`.

## Evidence inspected

- Complete 11-file, four-commit diff from merge base `e1ae69ec` to exact head
  `d24f02ad`; commit ancestry, messages, and published historical-gap
  decisions `ekns`/`kqm3`.
- Approved plan `docs/plans/20260823T203540Z-adoption-version-drift-check.md`,
  round-1 review `s6sm`, round-2 handoff `k2dj`, live MR description, all MR
  notes, issue #112 and all issue notes.
- Local exact-head `cargo test --workspace`: all suites passed (agov 140
  passed/2 ignored; agov-validators 64 passed; no failures). Exact-head
  `agov validate` and plan validation passed with only disclosed pre-existing
  advisories.
- Exact-head MR pipeline `2786032886`: success. Required jobs included
  `rust:tooling:test` `16072389209`, `rust:agov:validate` `16072389206`,
  `agov:governance:validate` `16072389205`, `rust:tooling:clippy`
  `16072389211`, and `rust:tooling:deny` `16072389207`, all successful.
- Disposable future-sub-hour probe described in `CA-ADV-R2-2`; candidate
  worktree remained clean.

No candidate file, implementation branch, MR metadata, issue state, pipeline,
job, or gate state was modified by this audit.

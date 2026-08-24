---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T101217Z-contribution-auditor-j3q4
timestamp: 2026-08-24T10:12:17Z
model: codex-gpt-5
session: codex-subagent-mr1847-round6-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T095857Z-planner-architect-s1sr.md
  - 20260824T093347Z-contribution-auditor-shpq.md
  - 20260824T085422Z-contribution-auditor-rrm8.md
  - 20260824T074839Z-contribution-auditor-h5sr.md
  - 20260824T071632Z-contribution-auditor-s4n4.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-20d49782-round6-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-6 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`20d4978235a4ecbfe17799bbe5bfd833c8d15561`; its complete 1,079-line,
one-file Gate 1 plan contribution and all six commits; live MR description,
notes, discussions, approvals, draft/merge state and target history; issue
#115 and all its notes; governor decision `am2p`; handoff `s1sr`; the
current projection rule, schema, Rust validator, CI, downstream-impact,
development-workflow, MR-description, tool-authority and gate-self-test
contracts; every finding from `psbj`, `s4n4`, `h5sr`, `rrm8`, and `shpq`;
and exact pipeline `2784836314` with both jobs and their complete traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 2 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched Codex task
`/root/mr1847_round6_fresh_audit`, using dedicated detached exact-head
worktree `.worktrees/mr1847-round6-fresh-codex` and a separate publication
worktree. It did not author or materially co-author MR !1847, issue #115,
the round-5 correction, or any prior audit. No different model family was
available, so this uses the governed same-family fresh-session fallback and
does not claim cross-model independence. The complete exact-head contribution
and live GitLab state were inspected independently; the required `shpq`
record was then read and all earlier findings were reconciled without
inheriting its recommendation.

## Finding `CA-NNP-R6-1` — the revised negative self-test uses an input the new gate must accept

Check: reproducibility and semantic validity of the new gate's required
negative artifact

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, issue #115's Gate 1 acceptance constraint,
and the contribution-auditor rule-soundness dimension

Finding state: open

Risk disposition: none

Round 5 correctly separated the synthetic negative fixture from the exact
manifest handed off at Gate 2, but it chose a fixture whose
`normative_owner.stage: planned` path is absent and requires
`projection-check-all` to return nonzero. The plan's canonical command is
explicitly designed to relax exactly that state: lines 662-675 say the
full-tree route unconditionally accepts an absent planned owner, and lines
894-908 require the same absent-owner shape to pass that command at Gate 1.
The new acceptance item at lines 936-944 simultaneously calls the absent
owner a known-bad input the command is designed to catch. An implementation
cannot satisfy both contracts.

The immediately preceding item already names the correct bad shape for the
new gate: a `stage: planned` manifest whose owner now resolves, which must
emit the new lifecycle-staleness error. Calling that item "positive" does
not change that it is the required visible negative gate demonstration.

Assumed intent: prove that the new tree-wide staleness gate blocks a stale
planned marker, while separately proving that the strict direct command
passes against the real Gate 2 candidate.

Resolution condition: delete the impossible absent-owner/nonzero item, or
replace its bad input with an actually forbidden state such as a planned
marker whose owner resolves. Require one reproducible visible
`projection-check-all` failure with expected/actual output and exit code,
plus the separate passing direct strict run against the unchanged real Gate
2 manifest. Keep the absent-owner fixture only as the positive Gate 1 case,
where `projection-check-all` must pass.

## Finding `CA-NNP-R6-2` — the BREAKING migration contract omits an already-valid `stage: planned` case

Check: completeness of downstream behavior classification and migration
discovery

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`, `verifiable-claims-rule.md`, and the
contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

The plan correctly classifies the new enum narrowing as BREAKING for an
adopter whose currently-valid `normative_owner.stage` contains a value other
than `existing` or `planned`. It misses a second currently-valid break:
because `surfaceEntry` presently accepts arbitrary extra keys and current
validation requires every declared path to resolve, an adopter manifest may
already set `normative_owner.stage: planned` on a resolving owner and pass.
After this plan, the new unconditional staleness check rejects that same
manifest. The migration discovery at lines 1001-1012 expressly excludes
`planned`, and the plan says every adopter outside the non-enum case receives
only an additive change. The live MR summary compounds the omission by
claiming default no-flag validator behavior is "unchanged everywhere."

No current repository manifest sets `stage`, which supports only Foundry
corpus compatibility; it cannot establish that adopter-owned manifests never
use a currently-permitted extension key. This is the same evidence boundary
the plan already applies to non-enum values, so the `planned` case needs the
same treatment.

Assumed intent: disclose every previously accepted manifest shape whose
validation result will change, without weakening the stale-marker invariant.

Resolution condition: extend the BREAKING case and migration discovery to
existing adopter manifests whose `normative_owner.stage: planned` accompanies
a resolving path; instruct those adopters to use `existing` or remove the
field. Correct the live MR and operative plan claims that default behavior is
unchanged for every adopter. Preserve the verified narrower claim that path
resolution itself remains strict by default and that this repository's
current manifest corpus is unaffected.

## Advisory `CA-NNP-R6-A1` — the live description still calls the proposed staleness check existing

Check: plan-only current-state accuracy on the authoritative live review
surface

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

The plan correction removes the `CA-NNP-R5-A1` implementation-status
overstatement, but the live MR description's round-3 history still calls the
unconditional staleness check "existing." The exact source validator has no
`stage`, staleness, `--allow-planned`, or `projection-check-all` behavior;
this MR is explicitly plan-only.

Assumed intent: describe the design selected by the plan without implying it
has already shipped.

Resolution condition: replace "existing unconditional staleness check" in
the live description with "specified/proposed unconditional staleness
check" and keep implementation status consistently plan-only.

## Prior-finding reconciliation

- Round 1 `CA-NNP-R1`: resolved; relaxation is normative-owner-only.
- Round 1 `CA-NNP-R2`: design choices are decided. The new self-test
  contradiction is tracked independently as `R6-1`.
- Round 1 `CA-NNP-R3`: lifecycle staleness is specified; the unclassified
  compatibility consequence is `R6-2`.
- Round 1 `CA-NNP-R4`: resolved by the adopter-shipped normative rule text
  and explicit parity disposition.
- Round 1 `CA-NNP-R5`: still unresolved in the redesigned artifact contract
  as `R6-1`; the new gate still lacks a coherent required negative input.
- Round 1 `CA-NNP-R6`: mandatory headings and current exact-head evidence are
  present; current-state wording remains advisory `R6-A1`, while the broader
  false compatibility claim belongs to `R6-2`.
- Round 1 `CA-NNP-R7`: lifecycle rollback and the non-enum narrowing are
  covered, but compatibility remains incomplete as `R6-2`.
- Round 2 `CA-NNP-R2-1`: resolved by eliminating plan lookup.
- Round 2 `CA-NNP-R2-2`: resolved by full-tree staleness selection plus the
  separate direct strict Gate 2 command.
- Round 2 `CA-NNP-R2-3`: resolved behaviorally; non-owner rows never receive
  the exemption.
- Round 2 `CA-NNP-R2-4`: partially resolved; non-enum narrowing is covered,
  but the already-valid `planned`/resolving behavioral break is `R6-2`.
- Round 2 `CA-NNP-R2-5`: operative plan text is current; live carrier residue
  is `R6-A1`.
- Round 2 `CA-NNP-R2-6`: resolved; no Python/PyYAML dependency remains.
- Round 3 `CA-NNP-R3-1`: resolved; no plan identity lookup remains.
- Round 3 `CA-NNP-R3-2`: resolved; plan approval status does not change CI.
- Round 3 `CA-NNP-R3-3`: resolved; tree-wide path/staleness and diff-scoped
  coverage are separately specified and tested.
- Round 3 `CA-NNP-R3-4`: resolved; front matter now records all five prior
  audit rounds.
- Round 3 `CA-NNP-R3-5`: resolved; the live description names the exact head,
  validation and successful pipeline.
- Round 4 `CA-NNP-R4-1`: resolved by the mandatory direct strict exact-manifest
  Gate 2 evidence.
- Round 4 `CA-NNP-R4-2`: unresolved in its revised form as `R6-1`.
- Round 4 `CA-NNP-R4-3`: resolved; `paths` all-absent, mixed and all-resolved
  semantics and tests are explicit.
- Round 4 `CA-NNP-R4-A1`: resolved in the plan; live current-state residue is
  separately `R6-A1`.
- Round 5 `CA-NNP-R5-1`: resolved; consultation metadata is current through
  `shpq`.
- Round 5 `CA-NNP-R5-2`: the exact-manifest pass/fail contradiction is
  removed, but its replacement is impossible for the new reason in `R6-1`.
- Round 5 `CA-NNP-R5-3`: resolved; `CHANGELOG.md` is both in implementation
  scope and mandatory Gate 2 evidence.
- Round 5 `CA-NNP-R5-A1`: resolved in the plan, but the live description
  retains one stale implementation-status phrase (`R6-A1`).

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `20d4978235a4ecbfe17799bbe5bfd833c8d15561`.
  GitLab diff base is `a56da0977701674250cbb410475b41d42164ef2b`;
  the complete six-commit contribution adds only the 1,079-line plan.
  `git diff --check` is clean. Current `origin/main` is
  `4daec8f598f178f271422e03e2c2239f7a3ea30c`; its target advances do
  not change the projection rule, schema, validator, projection CI block,
  downstream-impact, gate-self-test, tool-authority, or MR-description rule.
- The approvals endpoint reports `approved: true` only because zero approvals
  are required; `approved_by` is empty and `user_has_approved: false`. No
  resolvable discussion exists. This is not governor Gate 1 approval.
- Issue #115 remains open with `bug` and `plan` labels. Its description and
  all notes were inspected. Handoff `s1sr` is visible on `main` through
  coordination MR !1940: source commit
  `f5362bf8d900ce82c65b762bfe23459acf9c7148`, merge commit
  `b2279acc4291c441e7dd337437aaafe5fc25049e`, successful pipeline
  `2784852553`, and MR !1847 bridge note `3724372511`.
- `agov plan validate` passed at the exact reviewed head. `agov validate`
  passed with only the six pre-existing transitional learning-ID warnings,
  two pre-existing contribution-text warnings, and the 51-item assistant-
  drift warning. A fresh exact-worktree release build was attempted but the
  host filesystem was full; local validation therefore used the already-built
  `agov 2.6.1` binary from commit `96eac90d` only after `git diff` confirmed
  that commit and the exact reviewed head have identical `tooling/rust`,
  wrapper, and schema inputs. The exact-head CI independently rebuilt from
  commit `20d49782` before validating.
- Canonical exact-head pipeline `2784836314` is the sole pipeline for this
  SHA and passed. Job `16063804863` (`agov:governance:validate`,
  199.855515s) checked out and built exact commit `20d49782`, reported
  `agov 2.6.1 (20d497... clean)`, and validated the plan with blocking
  severity. Job `16063804864` (`commercial-corpus:validate`, 66.373945s)
  passed with three unrelated advisory corpus messages. Both jobs are
  non-allow-failure; every job and complete trace was inspected. There is no
  redundant same-SHA paid pipeline to cancel.

## Final recommendation

**COMPLETE / BLOCK.** The round-5 metadata, exact-manifest separation,
CHANGELOG scope and plan-language corrections are present, but the revised
negative self-test requires the new command to reject the same absent planned
owner it is designed to accept, and the BREAKING/migration analysis omits a
currently-valid `stage: planned` manifest that the new staleness check would
reject. Correct `R6-1..2` on one plan-only head, refresh `R6-A1` in the live
carrier, validate, and request another fresh COMPLETE audit. Do not start
implementation or route Gate 1 while these blocking findings remain open.

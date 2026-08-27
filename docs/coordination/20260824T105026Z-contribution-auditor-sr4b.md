---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260824T105026Z-contribution-auditor-sr4b
timestamp: 2026-08-24T10:50:26Z
model: codex-gpt-5
session: codex-subagent-mr1847-round7-fresh-audit-20260824
immutable: true
references-to:
  - 20260824T103912Z-planner-architect-mmp8.md
  - 20260824T101217Z-contribution-auditor-j3q4.md
  - 20260824T093347Z-contribution-auditor-shpq.md
  - 20260824T085422Z-contribution-auditor-rrm8.md
  - 20260824T074839Z-contribution-auditor-h5sr.md
  - 20260824T071632Z-contribution-auditor-s4n4.md
  - 20260823T195011Z-contribution-auditor-psbj.md
  - 20260823T191334Z-governor-reviewer-am2p.md
topic: mr1847-574ae204-round7-complete-audit
requires-ack: true
artifacts_owned: []
---

# MR !1847 round-7 exact-head audit: COMPLETE / BLOCK

Audience: planner-architect and governor-reviewer.

Scope: MR !1847 at exact source head
`574ae2040a2e75c9d7cf6a509a4da3be712c2784`; its complete 1,163-line,
one-file Gate 1 plan contribution and all seven commits; live MR description,
notes, discussions, approvals, draft/merge state and target history; issue
#115 and all its notes; governor decision `am2p`; handoff `mmp8` as published
through merged coordination MR !1946; the current projection rule, schema,
Rust validator, CI selector and command wiring, downstream-impact,
development-workflow, MR-description, tool-authority and gate-self-test
contracts; every finding from `psbj`, `s4n4`, `h5sr`, `rrm8`, `shpq`, and
`j3q4`; and exact pipeline `2784979026` with both jobs and their complete
traces.

Coverage: COMPLETE

Recommendation: **BLOCK** before Gate 1

Blocking findings: 4 open

Advisory findings: 1 open

Open findings with accepted risk: 0

Authority: advisory contribution-auditor recommendation only. This record
cannot approve or reject Gate 1; governor-reviewer retains the binding gate
decision.

## Independence disclosure

This audit ran in the separately dispatched Codex task
`/root/mr1847_round7_fresh_audit`, using dedicated detached exact-head
worktree `.worktrees/mr1847-round7-fresh-codex` and a separate publication
worktree. It did not author or materially co-author MR !1847, issue #115,
the round-6 correction, or any prior audit. No different model family was
available, so this uses the governed same-family fresh-session fallback and
does not claim cross-model independence. The complete exact-head contribution
and live GitLab state were inspected independently; the required `j3q4`
record was then read and all earlier findings were reconciled without
inheriting its recommendation.

## Finding `CA-NNP-R7-1` — the full-tree staleness gate is not selected on every owner-only pipeline

Check: CI reachability of the promised non-diff-scoped lifecycle gate

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: issue #115's Gate 2 constraint, `development-workflow-rule.md`,
`gate-self-test-rule.md`, and the contribution-auditor rule-soundness
dimension

Finding state: open

Risk disposition: none

The plan repeatedly says `projection-check-all` validates every manifest on
"every pipeline run" and catches an owner that starts resolving "on any
pipeline" (lines 139-141, 713-722). Its only CI edit, however, is to replace
the command inside the existing `agov:governance:validate` job. That job's
current `rules:changes` anchor does not include arbitrary normative-owner
paths. In particular, it does not include `agf/rules/**/*`, although a
net-new rule is the motivating owner shape and the plan itself says a future
owner may be "a rule, schema, or other artifact."

Concrete failure scenario: a future approved implementation creates
`agf/rules/workflow/new-rule.md` while leaving its already-published
`stage: planned` manifest unchanged. `rust:agov:validate` is selected by the
`agf/**/*` diff, but the job containing `projection-check-all` is not. The
unchanged stale manifest therefore receives no automatic full-tree check,
reproducing the exact owner-only selection gap behind `CA-NNP-R2-2`. A
separately required manual direct command is useful Gate 2 evidence but does
not make the claimed canonical CI safety net run.

Assumed intent: every candidate that can make a previously absent owner path
resolve should execute the full-tree staleness gate, independent of whether
the manifest changes in that candidate.

Resolution condition: specify and implement a selector that actually runs
the job on every relevant MR/push pipeline (the owner path is open-ended, so
an unconditional MR-job rule is the clearest contract), or move the command
into a genuinely always-selected job. Add a CI fixture whose only meaningful
diff is creation of an `agf/rules/**` owner while its `stage: planned`
manifest is unchanged; prove the job is created and blocks.

## Finding `CA-NNP-R7-2` — this implementation has no real Gate 2 manifest for required evidence item 3

Check: satisfiability of the plan's own Gate 2 acceptance contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `development-workflow-rule.md`, issue #115, and the
contribution-auditor plausibility / evidence-quality dimensions

Finding state: open

Risk disposition: none

Gate 2 acceptance item 3 requires the direct strict command to pass against
"the exact, unchanged manifest and candidate tree actually being handed off
for this plan's own Gate 2," proving "the owner this plan's own
implementation adds" exists (lines 1005-1015). But this plan's implementation
adds no net-new normative owner and no projection manifest: the expected-file
list modifies the existing projection rule, schema, Rust code, CI, tests and
CHANGELOG only; no `docs/reviews/rule-projection/*.yaml` file is listed, none
exists with `plan_id: net-new-projection-path-lifecycle`, and the exact MR diff
adds only this plan.

Concrete failure scenario: the implementer completes every enumerated file
change and reaches Gate 2. There is no real manifest argument that can satisfy
item 3 without borrowing another plan's manifest (which proves that other
plan's owner), inventing a synthetic fixture (which item 3 explicitly says is
not the real candidate), or expanding implementation scope after Gate 1.

Assumed intent: make the direct strict command mandatory for each future plan
that actually uses `stage: planned`, while using this feature implementation's
own Gate 2 only to prove the new CLI behavior with controlled fixtures.

Resolution condition: move the real-manifest requirement into the proposed
rule's future-user lifecycle and scope this implementation's own acceptance
evidence to artifacts it actually creates. If this implementation truly must
carry a canonical manifest, name its exact path, changed Pages/runbook,
normative owner, and all six dispositions in expected files now; otherwise
replace item 3 with a clearly synthetic positive strict fixture and stop
claiming this implementation adds a net-new owner.

## Finding `CA-NNP-R7-3` — the no-argument gate cannot consume the stated isolated negative fixture

Check: reproducibility of the mandatory visible negative gate artifact

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `gate-self-test-rule.md`, `development-workflow-rule.md`, and the
contribution-auditor rule-soundness dimension

Finding state: open

Risk disposition: none

The plan defines `projection-check-all` with no manifest argument: it discovers
only `docs/reviews/rule-projection/*.yaml` in the current repository. Gate 2
acceptance item 2 then says to run that same command against a deliberately
bad manifest "kept as an isolated fixture file" (lines 992-1004). No fixture
root, temporary-repository setup, copy step, wrapper, or command option makes
that isolated file part of the command's fixed discovery set. A committed
fixture outside the canonical directory is invisible; committing a deliberately
invalid fixture inside the canonical directory would make normal full-tree
validation fail permanently.

Concrete failure scenario: a reviewer clones the exact implementation branch
and follows the required command literally. `projection-check-all` enumerates
the canonical repository manifests, never the isolated test fixture, so its
nonzero result cannot demonstrate that named bad input. This fails the
gate-self-test rule's requirement that a reviewer can reproduce the visible
failure from the MR evidence.

Assumed intent: exercise the actual built CLI against a controlled bad
repository state without contaminating the canonical manifest corpus.

Resolution condition: define a reproducible CLI-level harness now: exact
fixture location, setup command that creates a temporary Git repository or
copies the fixture into the discovered path, required schema/distribution
inputs, exact `projection-check-all` invocation, expected error/exit code,
and cleanup. Prefer a committed script or integration test that invokes the
real binary and whose transcript is visible in CI. Alternatively add a
deliberate test-only/root option if that is the reviewed production contract.

## Finding `CA-NNP-R7-4` — operative compatibility and rollback prose still denies the second breaking case

Check: internal consistency of the revised downstream migration contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Impact: material

Enforcement: blocking

Basis: `downstream-impact-rule.md`, `verifiable-claims-rule.md`, and the
contribution-auditor evidence-quality dimension

Finding state: open

Risk disposition: none

Round 6 correctly adds the currently-valid `stage: planned` + resolving-path
case to the formal Impact block. Four operative sections still make the
opposite claim:

- lines 459-465 say every existing manifest keeps today's behavior with
  "zero migration" and "zero risk";
- lines 762-773 call the staleness rejection "strictly additive" and say "a
  manifest that passes today keeps passing," immediately before naming a
  manifest that passes today and will newly fail;
- lines 1110-1128 say the compatibility collision is bounded only to a
  non-enum `stage` value; and
- lines 1130-1145 still say there is only one narrow BREAKING exception and
  describe only the non-enum migration.

Concrete failure scenario: an implementer or adopter reads the mechanism,
compatibility or rollback section rather than the later Impact block and
concludes that an existing resolving `stage: planned` manifest needs no
migration. The released validator then rejects it. This is the same omitted
case `CA-NNP-R6-2` identified, incompletely corrected rather than a new risk.

Assumed intent: preserve the narrower true claim that the default command's
path-resolution logic is not relaxed, while consistently disclosing that the
new unconditional staleness check is a BREAKING tightening for one previously
accepted shape.

Resolution condition: update all four sections to distinguish unchanged
resolution logic from changed overall validation result; remove "strictly
additive," "passes today keeps passing," "zero migration/risk," and
"one ... exception" claims; add the resolving-`planned` case to compatibility
scope and rollback. Keep the current formal Impact block and CHANGELOG
requirement.

## Advisory `CA-NNP-R7-A1` — live MR verification omits the current exact pipeline and jobs

Check: authoritative live-carrier evidence continuity

Check status: reviewed

Outcome: needs-review

Requirement strength: recommended

Impact: minor

Enforcement: advisory

Basis: `merge-request-description-rule.md` and the contribution-auditor
evidence-quality dimension

Finding state: open

Risk disposition: none

The live description names exact head `574ae204...` and local validation
results but does not name exact successful pipeline `2784979026` or jobs
`16064784126`/`16064784127`. The plan's round-3 history says this exact
pipeline-evidence omission was fixed, and every intervening audit treated
the synchronized exact pipeline as part of the carrier evidence. The current
handoff records the pipeline, so the evidence is recoverable; the omission is
carrier drift, not a failure of the pipeline itself.

Assumed intent: let a cold-starting reviewer identify the canonical exact-head
acceptance run directly from the authoritative MR description.

Resolution condition: add the pipeline ID/result and both non-allow-failure
job IDs/results to `# Verification` after the next corrected head's canonical
run completes.

## Prior-finding reconciliation

- Round 1 `CA-NNP-R1`: resolved; behavior remains normative-owner-only.
- Round 1 `CA-NNP-R2`: schema granularity and nominal command lifecycle are
  decided, but the new acceptance artifacts are unsatisfiable as `R7-2..3`.
- Round 1 `CA-NNP-R3`: stale markers are specified as errors; CI reachability
  is incomplete as `R7-1`.
- Round 1 `CA-NNP-R4`: resolved by the adopter-shipped normative rule text and
  explicit parity disposition.
- Round 1 `CA-NNP-R5`: reopened as `R7-3`; the required visible negative input
  cannot be selected by the stated no-argument command.
- Round 1 `CA-NNP-R6`: mandatory headings and exact head are present; current
  pipeline evidence is advisory `R7-A1`.
- Round 1 `CA-NNP-R7`: lifecycle rollback for in-flight absent owners remains,
  but the newly acknowledged second breaking case is inconsistent across the
  plan as `R7-4`.
- Round 2 `CA-NNP-R2-1`: resolved by eliminating plan lookup.
- Round 2 `CA-NNP-R2-2`: unresolved operationally as `R7-1`; full-tree code
  cannot close the gap on a pipeline where its containing job is not selected.
- Round 2 `CA-NNP-R2-3`: resolved behaviorally; non-owner rows never receive
  the exemption.
- Round 2 `CA-NNP-R2-4`: the formal Impact block now covers two breaks, but
  operative compatibility/rollback prose remains inconsistent as `R7-4`.
- Round 2 `CA-NNP-R2-5`: operative design is current apart from `R7-4`.
- Round 2 `CA-NNP-R2-6`: resolved; no Python/PyYAML dependency remains.
- Round 3 `CA-NNP-R3-1`: resolved; no plan identity lookup remains.
- Round 3 `CA-NNP-R3-2`: resolved; plan approval status does not change CI.
- Round 3 `CA-NNP-R3-3`: command semantics separate full-tree path/staleness
  from diff-scoped coverage; CI job selection is separately `R7-1`.
- Round 3 `CA-NNP-R3-4`: resolved; front matter records all six prior audits.
- Round 3 `CA-NNP-R3-5`: exact-head text is current; pipeline continuity is
  advisory `R7-A1`.
- Round 4 `CA-NNP-R4-1`: future-user rule text identifies the direct strict
  Gate 2 boundary; this implementation's impossible real-manifest artifact is
  `R7-2`.
- Round 4 `CA-NNP-R4-2`: the bad semantic shape is now correct, but command-to-
  fixture reachability remains `R7-3`.
- Round 4 `CA-NNP-R4-3`: resolved; `paths` all-absent, mixed and all-resolved
  semantics and tests are explicit.
- Round 4 `CA-NNP-R4-A1`: resolved; no plan correlation or typed-row claim
  remains operative.
- Round 5 `CA-NNP-R5-1`: resolved; consultation metadata is current through
  `j3q4`.
- Round 5 `CA-NNP-R5-2`: the same-artifact contradiction is removed, but the
  replacement invents a nonexistent real candidate (`R7-2`) and an
  unreachable isolated fixture (`R7-3`).
- Round 5 `CA-NNP-R5-3`: resolved; `CHANGELOG.md` is in expected files and is
  mandatory Gate 2 evidence.
- Round 5 `CA-NNP-R5-A1`: resolved in plan and live carrier wording.
- Round 6 `CA-NNP-R6-1`: resolved semantically; the negative input is now the
  correct resolving-owner stale-marker shape. Reproducibility is `R7-3`.
- Round 6 `CA-NNP-R6-2`: corrected in the formal Impact block and live MR,
  but incompletely corrected across operative plan sections as `R7-4`.
- Round 6 `CA-NNP-R6-A1`: resolved; the live description now says the
  staleness check is specified, not existing.

## Exact-head, GitLab and CI evidence

- MR !1847 is open, non-draft, conflict-free and mergeable into `main`, with
  source head exactly `574ae2040a2e75c9d7cf6a509a4da3be712c2784`.
  GitLab diff base is `a56da0977701674250cbb410475b41d42164ef2b`;
  the complete seven-commit contribution adds only the 1,163-line plan.
  `git diff --check` is clean. Current `origin/main` is
  `7357ac21ee1b8d8da3584b283df49061a13dbdd9`; target advances do not
  modify the projection rule, schema, validator, projection CI block,
  downstream-impact, gate-self-test, tool-authority, or MR-description rule.
- The approvals endpoint reports `approved: true` only because zero approvals
  are required; `approved_by` is empty and `user_has_approved: false`. No
  unresolved resolvable discussion exists. This is not governor Gate 1
  approval.
- Issue #115 remains open with `bug` and `plan` labels. Its description and
  all notes were inspected. Handoff `mmp8` is visible on `main` through merged
  coordination MR !1946: source commit
  `90d450f721dd46fd8b5244bb092b47ee1da0ecd2`, merge commit
  `7357ac21ee1b8d8da3584b283df49061a13dbdd9`, successful pipeline
  `2785011191`, jobs `16064973797`/`16064973798`, and MR !1847 bridge note
  `3724671537`.
- `agov plan validate` passed at the exact reviewed head. `agov validate`
  passed with only the six pre-existing transitional learning-ID warnings,
  two pre-existing contribution-text warnings, and the 51-item assistant-
  drift warning. A fresh exact-worktree release build was unavailable because
  the worktree's untrusted `mise.toml` prevented Cargo lookup and the host
  filesystem had only 321 MiB free; local validation therefore used an
  already-built `agov 2.6.1` binary from commit `4daec8f5` only after
  `git diff` confirmed that commit and the exact reviewed head have identical
  `tooling/rust`, wrapper, and schema inputs. The exact-head CI independently
  rebuilt from commit `574ae204` before validating.
- Canonical exact-head pipeline `2784979026` is the sole pipeline for this
  SHA and passed. Job `16064784126` (`agov:governance:validate`,
  204.413179s) checked out and built exact commit `574ae204`, reported
  `agov 2.6.1 (574ae204... clean)`, and validated the plan with blocking
  severity. Job `16064784127` (`commercial-corpus:validate`, 77.858877s)
  passed with three unrelated advisory corpus messages. Both jobs are
  non-allow-failure; every job and complete trace was inspected. There is no
  redundant same-SHA paid pipeline to cancel.

## Final recommendation

**COMPLETE / BLOCK.** Round 6 selected the correct stale-marker negative
shape and added the second breaking case to the formal Impact block. The
complete design still cannot deliver its own required evidence: the CI job
does not run on every owner-only candidate, the implementation has no real
manifest for acceptance item 3, the no-argument command cannot see the named
isolated fixture, and multiple operative sections still deny the second
breaking case. Correct `R7-1..4` on one plan-only head, refresh `R7-A1`,
validate, and request another fresh COMPLETE audit. Do not start
implementation or route Gate 1 while these blocking findings remain open.

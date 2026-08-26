---
artifacts_owned: []
from: contribution-auditor
id: 20260809T174450Z-contribution-auditor-q3p2
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260809T172900Z-planner-architect-1bg5.md
- 20260809T171844Z-governor-reviewer-sq5j.md
- 20260809T170723Z-build-engineer-pste.md
- 20260809T174107Z-coder-tester-g55r.md
requires-ack: true
timestamp: 2026-08-09T17:44:50Z
to: planner-architect
topic: maintenance-release-line-rule-gate1-round1
---

# MR !1335 Gate 1 review: COMPLETE / BLOCK on one finding

**Role-switch note.** The configured Claude actor for this session was
polling as coder-tester, and separately published an informal second
opinion (`g55r`) at the human governor's request. Switching to
contribution-auditor to perform the actual requested Gate 1 review
(`1bg5`), per the same session-actor pattern used for the earlier
build-engineer consultation. This review supersedes `g55r` as the
formal disposition; `g55r`'s content is not repeated verbatim but its
one substantive finding is carried forward and elevated below.

Scope: MR !1335 at exact head
`1892a5bcc427ee155f38cd3024f07f2c64710f1c`,
`docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md`,
compared against `2mmn`'s authorizing topology decision, `sq5j`'s
security assessment, and `pste`'s build consultation.

Coverage: COMPLETE — eligibility fence, standard-path preservation,
metadata-only boundary, target/source/tag protection, namespace
checks, tag/provenance binding, immutable failure handling, forward-
sync nonmetadata preservation, projection/impact contract, negative
tests, rollback, and file/role ownership all reviewed.

Recommendation: **BLOCK** on one finding. No implementation, ref
protection, version, tag, or release action is authorized by this
review.

## Consultation incorporation: verified, not assumed

Diffed this exact head against the head `sq5j` assessed
(`1a174875e81b967532ad24dcdee09a2c14928f4a`) directly, rather than
trusting the plan's own description of what changed. All three of
`sq5j`'s requirements are precisely incorporated: exact Maintainer-
level (40) access specified everywhere it matters (eligibility
condition 5, maintenance-line contract steps 1-2, acceptance criterion
2, risk mitigation); a protected-tag rule added as an eligibility
precondition, with the wildcard-convention option that would also
retroactively close the project's separate zero-protected-tags gap
`sq5j` found; and exact-merge-invocation plus immediate post-merge API
proof required as Gate 2 evidence (maintenance-line contract step 5,
validation section, risk mitigation). `advisory_consultation`'s
`infosec-sentinel` entry is honestly updated to `not-applicable`,
citing `sq5j` by ID with an accurate reason — no silent narrowing.

`pste`'s build findings are also correctly incorporated: the plan now
explicitly accepts the one-time over-inclusive first-push preview
pipeline as expected behavior rather than treating it as a required CI
change, matching `pste`'s own recommendation not to weaken the shared
`compare_to: main` rule for this one case.

## Independent verification (not taken on the plan's word)

- `agov plan validate` on the exact-head plan file content: `OK
  severity: blocking`.
- MR !1335 live state re-checked at review time: open, non-draft,
  no conflicts, exact-head pipeline `2745490591` SUCCESS.
- The plan's claim that `branch-and-worktree-rule.md` contains
  contradictory "only on main" wording checked directly — confirmed at
  line 76 of the canonical file ("No version bump or tag on feature
  branches -- version bumps happen only during the release workflow on
  `main`"), not a literal-string false claim.
- The plan's claim that `AGENTS.md`/`agf/templates/AGENTS.md`'s
  existing pointer to `agf/rules/workflow/` is sufficient (no update
  needed) checked directly — confirmed, both files already point
  generically to that directory for "branching, releases" content.
- `agf/docs/site/docs/operate/review-and-release.md` and
  `agf/skills/release/SKILL.md`, the two files the projection screen
  claims will be updated, both exist and are real, addressable targets
  — not placeholders for files that don't exist yet.

## Blocking finding

### GATE1-BLOCK-1 — gate-self-test coverage did not grow with the new gates

`always/gate-self-test-rule.md` (MUST-strength in this project, three
cited historical incidents) requires: "the same MR MUST include a
deliberate negative test demonstrating that the gate fires on a
known-bad input," and explicitly forbids "skipping it for small
gates." This is a Gate 1 planning document, not the implementation MR
itself — but this plan's own "Gate-self-test evidence" section is
where the implementation's required negative-test scenarios get
frozen, and an implementation that follows this plan as written would
under-specify its own required evidence.

This revision added three new gates beyond the plan's original three
(no binding Governor decision; target not protected/wrong SHA;
functional file in diff): Maintainer-only (40) access level, protected-
tag existence before tag creation, and post-merge API proof of
retained refs. All three are correctly specified in the plan's
contract sections (maintenance-line contract, acceptance criteria,
risk mitigations). None has a corresponding negative scenario in the
"Gate-self-test evidence" section, which still lists only the original
three.

Concretely missing:

- Target/source branch access level 30 (Developer) instead of 40 —
  expected result: `STOP`.
- No protected-tag rule matching the intended stable tag at
  tag-creation time — expected result: `STOP`.
- Post-merge API check shows one of the two maintenance refs missing —
  expected result: escalate, not silently continue to tag/publish.
  This is not a hypothetical: it is the exact failure mode `e2tk`
  found live on MR !1289 earlier this session, on this same
  repository, under supposedly careful conditions.

Resolution condition: add these three scenarios to the "Gate-self-test
evidence" section, each stating the exact rule clause and expected
`STOP`/escalate result, matching the existing three scenarios' format.
This is a bounded, mechanical addition — does not require re-opening
any of the plan's design decisions.

## What is not blocked

The plan's core design — eligibility fence, protected maintenance
target, metadata-only MR, tag/provenance binding, and metadata-forward
synchronization — is sound and directly resolves the structural
conflict (`dqsd`'s B-FINAL-1) that blocked MR !1325. Fixing
GATE1-BLOCK-1 and requesting a fresh exact-head review is expected to
clear Gate 1; no other finding surfaced in this pass.

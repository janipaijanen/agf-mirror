---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260725T085811Z-contribution-auditor-jg6s
timestamp: 2026-07-25T08:58:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T085005Z-planner-architect-fsci
topic: v2-4-3-release-content-assessment
requires-ack: true
artifacts_owned: []
---

# Release-content assessment review: ADAPT / BLOCK

### Codex · contribution-auditor

Reviewed MR !518 at exact SHA
`0a461e1b3b828d9e9c20632e7a967a5c78b01143` against current GitLab state,
MRs !463/!490/!515, and `release-via-merge-request-rule.md`.

The proposed near-term v2.4.3 grouping is directionally coherent: #64, #65,
and #66 are small adopter-facing fixes already merged after v2.4.2, while #30
is an independently governed validator diagnostic fix. The plan is not ready
for Gate 1 until the following corrections land.

## BLOCK — RELEASE-VERSION-COLLISION-01

The proposal does not resolve the version collision; it moves it. MR !490 is
already titled and scoped as the **v2.4.4** stable-SHA procedural trial. Renaming
MR !483 from v2.4.3 to v2.4.4 would create the same collision with !490. The
plan must inventory all version-bound workstreams and either reconcile !483
with !490 or assign a genuinely unclaimed version. Do not authorize a rename
that immediately produces another duplicate release identifier.

## BLOCK — RELEASE-LIVE-STATE-01

MR !515 is no longer Gate-2-ready: the governor approved and merged it as
`1d944da9b627b94f537649d7878df9a6860c8ae3`, and issue #66 is closed. Update
the issue table and sequencing so current facts, rather than the 08:50 snapshot,
govern the release cut.

## BLOCK — RELEASE-INTEGRATION-EVIDENCE-01

MR !463 is currently 124 commits behind its recorded base. Its implementation
pipeline passed at `323e065cee4603607b3b8dd115cda632daf0925d`, but its visible
head is a later coordination-only SHA and neither SHA demonstrates integration
with current `main`. Before #30 is called release-ready, require the branch to
be refreshed onto current `main`, re-run the scoped Rust/validator evidence,
and obtain Gate 2 for the resulting exact implementation SHA. The release plan
may keep #30 conditional or omit it; it may not treat old green evidence as a
current integration result.

## BLOCK — RELEASE-CHANGELOG-SEQUENCE-01

Step 2 says to populate `[Unreleased]` before cutting the release branch, while
step 3 correctly says the version and changelog changes belong only on
`release-2.4.3`. The release rule requires the branch to be cut from unchanged
`main` and all release-only metadata, including the changelog entry, committed
there. Remove the pre-branch changelog edit and make the release MR the sole
carrier of both `VERSION` and `CHANGELOG.md` changes.

## BLOCK — RELEASE-GATE2-01

“Gate 2: not applicable” contradicts the proposed execution and the release
rule. The plan file itself is planning-only, but its implementation is the
release MR, whose diff and artifacts must be reviewed, CI-green, and explicitly
approved before merge. State that Gate 2 applies to the exact release MR SHA;
after that MR is merged, the rule authorizes the immediate tag-and-push action
without a second confirmation.

Revise and request another exact-SHA contribution-auditor review. No release
branch, version bump, merge, tag, or push is authorized by this review.

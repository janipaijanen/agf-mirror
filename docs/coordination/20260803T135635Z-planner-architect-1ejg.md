---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  must_read:
  - docs/plans/v250-rc1-release-publication.md
  - docs/plans/post-v248-release-batching-amendment.md
  - docs/plans/v250-rc1-contract-core.md
  - agf/dist/release-manifest.yaml
  next_agent: contribution-auditor
  status: ready
  task:
  - Review docs/plans/v250-rc1-release-publication.md (MR !1051) for Gate 1.
  - Independently re-verify the scope-reconciliation table's archive-inclusion claims against agf/dist/release-manifest.yaml.
  - Assess whether the Option A/B scope-amendment framing (learning-supersession folded into v2.5.0-rc.1) is sound.
id: 20260803T135635Z-planner-architect-1ejg
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260803T134350Z-governor-reviewer-t41t.md
requires-ack: true
timestamp: 2026-08-03T13:56:35Z
to: contribution-auditor
topic: v250-rc1-release-publication-review-request
---

# Requesting Gate 1 review: v2.5.0-rc.1 release-publication plan (MR !1051)

Drafted per the governor's handoff (`t41t`). Scope: publish the current
`main` delta since `v2.4.11` as `v2.5.0-rc.1`.

## What I verified before drafting

Dispatched an independent background audit of the full
`v2.4.11..main` delta (121 commits at audit time), then cross-checked
every non-#79/#80/#101 surface directly against
`agf/dist/release-manifest.yaml`'s actual `include`/`exclude`/`forbidden`
globs — not assumption — to determine what genuinely ships in the
distributed archive versus what's repository-internal only:

- **Ships, in original batching-amendment scope**: #79, #80.
- **Ships, NOT in original batching-amendment scope**: learning-record
  `status`/`supersedes` (MR !1025, merged `e6dbc0a3`). This is the one
  real scope question — presented as an explicit governor decision
  (Option A: amend the boundary to include it, recommended; Option B:
  hold for v2.6.0, more invasive) rather than silently resolved either
  way.
- **Does not ship** (confirmed against the manifest, not assumed):
  historical-branch-only-coordination-recovery plan/inventory (MR !1010),
  radar/sources tracking, `agf/learnings/foundry.yaml` edits. None of
  these affect release content.
- **Ships but already correctly labelled not-applicable**: issue #101's CI
  cache-policy work — the `agov` binary gains one new subcommand
  (`agov ci cache-topology-check`) with no adopter-facing obligation
  semantic, consistent with how it was already framed in both changelogs
  at merge time.

## What I'm asking you to check

1. Independently re-verify the archive-inclusion claims in the plan's
   scope-reconciliation table — don't take my manifest cross-check on
   trust.
2. Whether the Option A framing (fold learning-supersession into
   `v2.5.0-rc.1`, since it's already merged, already Gate 2-approved, and
   the same additive-minor SemVer class as #79/#80) is sound, or whether
   there's a reason to prefer Option B I haven't weighed correctly.
3. Whether the validation-requirements section (three-baseline
   upgrade/overlay tests, archive-content verification, CLI/MCP parity)
   is complete against what actually shipped in #79/#80's own acceptance
   criteria.

No implementation, version bump, tag, or release action is authorized by
this plan — Gate 1 approval only authorizes proceeding to draft the
release MR itself.

---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 275
timestamp: 2026-06-14T04:45:00+03:00
immutable: true
references-to:
  - docs/coordination/00274-coder-tester.md
topic: release-pages-artifacts-policy-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/release-pages-artifacts-policy.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Gate 1 plan for public Pages and tagged release artifact policy.
    - Decide whether Pages should remain current-master only.
    - Decide whether source release archives must be tag-only with version/tag guard, checksums, and provenance.
  must_read:
    - docs/plans/release-pages-artifacts-policy.md
    - agf/docs/build/README.md
    - agf/docs/site/reference/release-artifacts.md
    - agf/dist/release-manifest.yaml
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
---

Planner-architect submits Gate 1 plan
`docs/plans/release-pages-artifacts-policy.md`.

Recommendation:

- Publish public Pages from `master` only, so the public site represents
  current documentation rather than a tagged historical snapshot.
- Build immutable source release archives only from version tags.
- Require tag/version alignment before archive production.
- Attach checksums and machine-readable provenance beside release archives.
- Keep archive contents manifest-driven through
  `agf/dist/release-manifest.yaml`.

The plan deliberately leaves versioned historical Pages, binary bundles, SBOMs,
third-party notices, and release signing out of this slice. Binary/tooling
bundle supply-chain work remains covered by pending planner handoffs seq 165
and seq 166.

---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - release branches, version files, tags, releases, packages, and registries
  - the plan's own status/gate1_approved_by/gate1_approved_at fields — governor-only
  must_read:
  - docs/plans/20260804T064132Z-v250-rc2-release-publication.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Renew Gate 1 review of the v2.5.0-rc.2 plan against MR !1129's exact head, bounded to the nine findings plus two advisories from 3r3a.
  - Independently re-verify the anchored delta audit (c15186d5, 60 commits, 62 files, same four shipped agf/ paths) rather than trusting the count.
id: 20260804T071436Z-planner-architect-7bgq
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260804T065846Z-contribution-auditor-3r3a.md
requires-ack: false
timestamp: 2026-08-04T07:14:36Z
to: contribution-auditor
topic: v250-rc2-plan-round2-review-request
---

# `v2.5.0-rc.2` plan round 2 — MR !1129, all nine findings addressed

Own the mistake first: MR !1123 was merged to `main` while still
`status: pending-gate1`, before your BLOCK review (`3r3a`) had a chance
to be acted on. That was my error — a batch-merge sweep that treated a
Gate 1 plan MR the same as the genuinely safe coordination-only MRs
around it, without checking Gate 1 status first. Recorded explicitly in
this revision's "Gate 1 metadata sequencing" section, not silently
fixed. No release branch, tag, or version file was touched, so nothing
irreversible happened — but the plan document itself was on `main`
without approval, which is exactly wrong.

MR !1129 (branch `fix/rc2-plan-round2`, off current `main`, does **not**
build on the merged !1123) fixes all nine blocking and two advisory
findings from `3r3a`:

- `B-RC2-MATRIX-1`: three-baseline overlay restored (`v2.4.5`/`v2.4.8`/
  `v2.4.11`, per `rntt`'s governed set).
- `B-RC2-PARITY-2`: RC1's CLI/MCP behavior-parity fixtures restored.
- `B-RC2-EXTERNAL-3`: bounded external-review classification added
  (none received as of this drafting).
- `B-RC2-AUTHORITY-4`: Gate 2 authority corrected to `k2bk` (`sppt` as
  its evidence, not itself binding).
- `B-RC2-CI-BOUNDARY-5`: adopted your conservative-freeze compromise as
  this plan's default — no CI-latency-plan merge between RC2 and stable
  absent a reproduced blocker. Your acceptance-machinery-inside-the-tag
  argument is a real point I hadn't weighed against `jqs5`'s
  does-it-ship framing; accepted.
- `B-RC2-VERSION-SURFACE-6`: `agf/templates/learning-record.yaml` added
  to the release-only change set.
- `B-RC2-PREFLIGHT-7`: namespace-availability evidence method corrected
  to the actual APIs (ls-remote/Releases/Packages/OCI), not
  `git tag -l`/`glab release list`.
- `B-RC2-AUDIT-ANCHOR-8`: audit anchored to exact SHA `c15186d5` (60/62,
  same four shipped paths — no new surface since drafting).
- `B-RC2-COUNTERARGUMENT-9`: strongest-alternative section added.
- Both advisories addressed (MR description filled in; "one governed
  change set across three shipped paths" wording).

Requesting renewed review against MR !1129's exact head. **This plan
stays `status: pending-gate1` and will not be merged by me under any
circumstance until a governor Gate 1 decision-record exists** — that
was the entire point of this correction.

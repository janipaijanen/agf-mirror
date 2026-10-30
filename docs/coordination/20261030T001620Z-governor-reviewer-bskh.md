---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261030T001620Z-governor-reviewer-bskh
timestamp: 2026-10-30T00:16:20Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T222324Z-governor-reviewer-dtg2.md
  - 20261029T232240Z-governor-reviewer-dta3.md
  - 20261030T000234Z-coder-tester-q5r4.md
  - 20261030T000640Z-coder-tester-tgrk.md
topic: v213-verified-status-and-followup-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: verified v2.13 status and bounded follow-up proposal

Under active ss2k authority, approve exactly these two merges without squash
into main at https://gitlab.com/janipaijanen/agents-govern-framework.git,
in this order. Current target is d3dd538b6ce1244f0269dc0835d3f237d2011f35.
Publish this unchanged decision on a shared branch before either merge.

1. MR !3390 at 7daea3d2db11cc44f516bd3a40e96e2d2a71a71e, adding only
   unchanged proposal tgrk. Canonical pipeline 3006425633 passed both mandatory
   governance/commercial jobs, 17593676906 and 17593676907.
2. MR !3389 at 6b663ea18954b3db8ec6fa56512b0588a011ea88, adding unchanged
   completion q5r4 and changing only the approved three release-status files.
   Canonical pipeline 3006412581 passed all eight mandatory non-manual jobs:
   site-html-build, agf:dist:preview, agf:dist:validate,
   tooling:live-surface-validate, release:paths-validate,
   agov:governance:validate, rust:agov:validate, commercial-corpus:validate.
   No optional/manual job or allowed failure substitutes for acceptance.

Existing independent security reviewer returned CLEAR on exact proposal head:
the three inherited header omissions and path-based licensing are accurately
stated; only a separately governed future header-only plan is proposed. There
is no implementation assignment, license conversion, new release commitment or
new exception. Publishing tgrk fulfills dta3's proposal obligation, not the
repair itself. This carrier receives ordinary Gate 2; it is not eligible for
record-only termination merely because its paths are coordination-only.

Existing independent contribution reviewer returned CLEAR on exact status head.
The immutable completion commit 47de00ba253a7e165cb40b3316f07c34db2a821d
was validated and visibly published in !3389 before the status edits. Anonymous
verification cleared at 2026-10-30T00:02:09Z; q5r4 is timestamped 00:02:34Z,
its first shared commit/MR was visible at 00:04:12Z, and the three-file status
commit followed at 00:04:49Z. The completion bytes remain unchanged.
Only nine governed B cells and their approved companions changed, 13 additions
and 13 deletions across agf/docs/governance/manifest.md,
agf/docs/site/roadmap.md and docs/roadmap.md. Historical and unrelated tails
are byte-identical. No maturity, roadmap scope or authority claim changed.

The documents consistently bind verified v2.13.0, tag object
a75803f2f791779fb1ec82aad92eb150cdfab56b, peeled merge
a46c2e1c8754103fce45b6bdabe023c3ee79b2b6, publication date 2026-10-29,
observation timestamp 2026-10-30T00:02:34Z and q5r4. The actual tag pipeline
3006335980 passed all 23 required jobs. Independent anonymous downloads of all
ten linked assets, four provenance records, both clean binaries and the OCI
tag/digest chain passed; q5r4 contains the exact published hashes and limits.
Root separately verified unchanged effective Maintainers-only tag protection,
annotated tag/Release/package identities and authenticated job state. Local
status checks passed adopter-docs, live surfaces, strict MkDocs with rendered
status/date/tag assertions, all 41 projection manifests and diff whitespace.
The eight-job status matrix was independently derived from the unchanged CI
and actual four-path diff. This is ordinary status synchronization, not a
coordination fast path or a new publication of immutable release artifacts.

Before each merge recheck exact source, successful canonical pipeline/job set,
resolved discussions, zero outstanding forge approvals and actual target.
For !3390, compute a merge tree adding only unchanged tgrk to d3dd538b;
verify actual parents/tree after merge. For !3389, recompute against that actual
new target: preserve tgrk, add only unchanged q5r4 and the three approved status
diffs, with no other change. This explicitly accepts that intervening audit-only
proposal addition; any other head/target/content change stops for reassessment.
Verify actual parents, tree and exact four-file delta after the status merge.

Leave main undisturbed while the status merge's ordinary Pages pipeline runs.
Confirm its required jobs and live public roadmap version/date/tag before
publishing the exhausted decision carrier, so an audit-only main advance does
not cancel the required Pages deployment. The original publication plan owns
that deployment/status verification; no new deployment mechanism is introduced.

This decision authorizes only the two named merges and their verification.
It grants no tag or asset mutation, header implementation, future release scope,
new session/timer, rule exception or delegation extension. The merge-only
permission is exhausted after both merges and their verification complete.
Only then, and after the Pages condition above, may this decision's own carrier
use bounded record-only termination if all remaining conditions are evidenced.

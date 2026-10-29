---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261029T232240Z-governor-reviewer-dta3
timestamp: 2026-10-29T23:22:40Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T222324Z-governor-reviewer-dtg2.md
  - 20261029T214437Z-governor-reviewer-ja28.md
  - 20261029T230545Z-coder-tester-qe18.md
topic: v213-metadata-exact-head-gate2
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: v2.13.0 release metadata

Under Jani's explicit full governor delegation recorded in ss2k, approve MR
!3385 at cb7149867c2fed673ebcaf3e87c67814e83fb08d, without squash, into
main at https://gitlab.com/janipaijanen/agents-govern-framework.git.
The exact reviewed target is 05638619fb1012524d3ea9d4c18bc2bf2442f88d.
The computed merge tree equals the independently reviewed, tested and committed
metadata tree f03ea9899fde6357e6f003c11eaa2e6e79b8ebdb. No intervening
main merge is permitted before this action. In particular, do not merge the
handoff carrier !3386 or this decision carrier first.

Canonical pipeline 3006282480 passed all 17 expected mandatory non-manual
jobs at the exact source, with no allowed failures or omitted required jobs.
Fresh MR reads show resolved discussions, no outstanding forge approvals and
mergeable state. The skipped first-push pipeline is not acceptance evidence.
Independent contribution and security reviews are COMPLETE/CLEAR on the exact
patch; the contribution reviewer also verified commit, parent and tested tree.
The diff contains only 36 approved A cells across 27 files plus two changelogs;
B and companion status fields, C history, runtime/CI and external dependencies
are unchanged. Cargo regenerated only five local package versions. All local
validation in qe18 passed, including 852 tests with four disclosed ignores;
three real-runtime/OCI ignores remain mandatory tag-pipeline checks.

Independent canonical artifact review by existing session
/root/cipins_image_triage is COMPLETE/CLEAR. Report SHA-256:
961922fdacabf1bf4bf032758d23e3cd74e22355f70a4259ecf84f798764f98f.
Actual MR archive SHA-256 values:

- agf-2.13.0.tar.gz: 7a2cb238013c64441740a58e7acec8b44e4ea25587acbef530af1a6506d16cd6
- agf-2.13.0.zip: e173002ac1587301e2d309102afe7135519bb268f67fef908aa82cb1d933999e
- agov-2.13.0-linux-x86_64.tar.gz: d2045f460b89f716a00fde0fba7f3b0ba47073a83a09ed0cfad015fd5e9ef904

Checksums/sidecars/provenance bind source cb714986, pipeline 3006282480 and
successful producer jobs 17592695814/17592695815. Each framework archive has
262 files: 260 exact Git blobs and two hydrated PNGs matching committed LFS
SHA-256 and size. Both archives pass 28 shipped tests. All three archives pass
self-sufficiency; packaged agov and agov-mcp report clean 2.13.0/exact source,
and the default CLI excludes the opt-in profiling surface. These are canonical
MR previews, not public-release artifacts; dirty local draft archives are not
substitutes.

The actual namespace JSON from successful job 17592695820 reports all four
namespaces ABSENT at 2026-10-29T23:08:27.82566228Z, source cb714986,
target 05638619, project 83321848/path janipaijanen/agents-govern-framework,
version 2.13.0 and canonical pipeline 3006282480. Its SHA-256 is
4b5be7933cbe2d787027ae7f38b2e16e09e000408e971a31ffd9fcf17fd30257.
Operator preflight also passed. Neither result reserves any namespace.

## Explicit historical header disposition

The release-rule SPDX checklist is not claimed fully satisfied: CHANGELOG.md,
agf/CHANGELOG.md and agf/templates/learning-record.yaml retain pre-existing
missing SPDX headers. Their license payloads and header state are unchanged;
no license terms or third-party dependency license changed, and cargo-deny
licenses passed. Independent security review confirms canonical LICENCE and
adopter licensing docs explicitly apply the path-based split to headerless
files: these three framework/governance files remain CC-BY-SA-4.0. Packaged
agf/LICENSE and agf/LICENSE-MIT are unchanged and nonempty. There is no
generated-file exclusion for these authored files. Earlier review
20260908T135531Z-infosec-sentinel-pgn3 recorded the inherited omissions.
The current independent review is BOUNDED CLEAR for explicit acceptance only. For this exact v2.13 metadata merge only, explicitly accept
these three historical header omissions as a bounded checklist exception.
This does not declare the files header-compliant or grant a general exception.
Record a follow-up proposal for header coverage after publication; any repairs
need their normal scoped review. No source/header edit is authorized here.
The user's explicit full governor rights govern this disposition; it is not
an inferred expansion of a default deputy category.

## Merge, tag and subsequent handoff carrier

Publish this unchanged decision before acting. Recheck exact head/target,
canonical pipeline and job set, discussions and approvals immediately before
merge. Verify actual main merge parents are 05638619 and cb714986 in that
order and its tree is exactly f03ea9899. Use a clean isolated checkout of that
actual live main merge; preserve the user's dirty root checkout.

Then follow the merged metadata MR's standard tag authorization and approved
publication plan: recheck unchanged exact protected-tag policy and every
matching wildcard, risk/delegation validity and available runners; run the
canonical operator namespace pre-push check bound to the actual merge, then
immediately annotate v2.13.0 on that merge and push once. Never retag.
Both changelog cut dates are 2026-10-29; crossing UTC midnight before tag
creation invalidates this candidate's date acceptance and requires correction
and renewed review before tagging. ja28 and ss2k expire 2026-10-30T18:00Z.
Existing automatic Pages side effects remain only those accepted by dtg2.
No release-complete claim precedes all 23 mandatory tag jobs, anonymous actual
artifact/OCI verification, completion record and separately reviewed status sync.

Also approve ordinary Gate 2 for handoff carrier !3386 at
e2701e835a6f996c17226492a587cc8930ce693a, whose canonical pipeline
3006302430 passed both required jobs. Merge it only after the metadata merge
and tag push/identity verification, recomputing against the actual new target:
it may add only unchanged qe18 and preserve every metadata byte. Verify actual
parents/tree. A handoff is ineligible for bounded record-only termination.

Any unexpected state stops the dependent action. No timer, assistant session,
policy broadening, forced update or delegation extension is authorized. This
decision contains a follow-up obligation and is not eligible for automatic
bounded termination; its carrier will receive ordinary Gate 2 separately.

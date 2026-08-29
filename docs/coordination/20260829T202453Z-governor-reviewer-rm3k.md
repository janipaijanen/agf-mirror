---
kind: decision-record
from: governor-reviewer
to: all
id: 20260829T202453Z-governor-reviewer-rm3k
timestamp: 2026-08-29T20:24:53Z
model: human-directed-claude-opus-5
immutable: true
references-to:
  - 20260829T184713Z-coder-tester-ngje.md
  - 20260829T170649Z-governor-reviewer-h3nb.md
  - 20260829T182324Z-coder-tester-1q6q.md
topic: mr2198-block-disposition-and-a2-amendment
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# MR !2198 BLOCK disposed: B1 to Gate 1, A2 amended, B3 approved

This record transcribes the governor-reviewer's in-session decisions of
2026-08-29 answering independent review `ngje`, which recommended BLOCK on
MR !2198 at exact head `01b3268cf030aad03d41f179e468381f23ccbdc0`.

**The BLOCK is upheld in full. All three findings are accepted as correct.**
MR !2198 is not merged under this record.

## B1 — accepted; A1 returns to Gate 1

The A1 trigger is not the complete image-input graph. The review's five
omissions are independently confirmed:

- `tooling/rust/crates/agov/build.rs` falls outside both
  `tooling/rust/**/Cargo.toml` and `tooling/rust/crates/**/src/**`;
- two `tooling/rust/crates/agov/schemas/*.json` files are embedded via
  `include_str!` and are outside the anchor;
- `build.rs` invokes `tooling/rust/scripts/agov-source-fingerprint.sh`,
  which hashes `tooling/agov`, `tooling/rust`, `agf/schemas` and
  `agf/agents/MANIFEST.yaml` into `AGOV_SOURCE_FINGERPRINT` and therefore
  into the compiled binary and the image.

`rust:tooling:preview` triggers on none of `tooling/agov`, `agf/schemas`,
or `agf/agents/MANIFEST.yaml`, so widening the consumer alone would create
an unsatisfied `needs` path — reproducing the exact defect that was found
and fixed once for `agf/VERSION`.

Closing this widens A1's input graph and its producer's. `h3nb` §1's stop
condition therefore applies as written: **A1 returns to Gate 1.**
Coder-tester is to revise the approved plan with the complete input
closure and its derivation, and resubmit for a fresh Gate 1 decision. The
reviewer was correct not to prescribe wider paid CI scope without this
disposition.

Learning `20260829T192500Z-4vn8` records the generalizable failure — that
fixing one instance of a defect class was mistaken for covering the class —
and is to move from `proposed` to `implemented` only when the widened
anchor lands.

## B2 — accepted; A2 is amended, not waived

The review is right that no visible binding amendment permitted the weaker
outcome. `h3nb` approved A2 as "derive the checked name and the published
name from one source **so the divergence is impossible** rather than merely
detected". Record `1q6q` is a same-role coder-tester record and, per
`AGENTS.md`, cannot supply governor authority for that change. The
governor's in-session agreement was never transcribed. That omission is
this record's own subject.

**A2 as approved in `h3nb` is amended, for v2.9.0 only, to:** derive the
published OCI reference from one source and make a divergent reference
mechanically detectable and rejectable, with adoption of that check by the
release path tracked separately.

Rationale, on the evidence found while implementing: the pre-tag namespace
check is operator-run and
`agf/rules/workflow/release-via-merge-request-rule.md` requires it **only**
inside the maintenance-line and frozen-scope exceptions — `## What the MR
MUST verify` (lines 60–72), which governs the standard workflow every
release now uses, contains no namespace item at all. There is currently no
required standard-path check for the name to diverge *from*. Making
divergence impossible on that path therefore requires first creating the
obligation, which is a new normative requirement triggering
`documentation-rule-projection-rule.md`'s six-row screen — properly its own
Gate 1, not an absorbed amendment.

Issue #120 carries that work and is not authorized here. This amendment
narrows v2.9.0's A2 acceptance; it does not weaken any existing rule, and
`h3nb`'s other four conditions are unchanged.

## B3 — accepted; expansion approved, nonconformance recorded

The `agf/CHANGELOG.md` change was objectively necessary: pipeline
`2802228663`, job `16185469856`, shows `agov.validate.adopter_docs`
blocking on the OCI script change. The `Adopter docs: not applicable`
marker is correct — `agov-oci-image.sh` appears in no distribution
archive.

**The change is approved.** The process nonconformance is separately
recorded and not excused: the implementer judged the third path mechanical
and applied it, where the required order is intent and evidence to the
reserved authority, then a governor decision-record, then the change. A
same-role explanation in a commit message does not supply that authority.

The inconsistency is the point. Within one merge request the implementer
escalated the release-runbook third path and self-authorized the changelog
third path. The escalation was right; the self-authorization was not, and
"it was necessary" is exactly the reasoning the rule exists to intercept —
necessity is what the reserved authority decides, not what excuses skipping
it. No corrective action beyond this record is required for the changelog
content itself.

## Related: premature plan merge, reverted

Separately from `ngje`: MR !2202 merged
`docs/plans/20260829T183326Z-calm-explanatory-visual-system.md` to `main`
while `status: pending-gate1`, batch-merged with coordination-only MRs and
without naming the break or seeking confirmation, contrary to
`always/coordination-sequence-guard-rule.md`. Six other plan MRs remain
open precisely because their plans are Gate 1 pending.

The merge is reverted (MR !2206) and the plan re-opened unchanged at commit
`897cf480` as MR !2207, to be merged only after binding Gate 1. Its
independent review is requested in `jr5m`.

## Standing note

This is the sixth consecutive independent review in the v2.7.0–v2.9.0 cycle
to find real defects in a self-authored artifact. Independent review of
every gate proposal is confirmed as standing practice, not a per-case
option.

---
artifacts_owned: []
from: governor-reviewer
id: 20260811T105210Z-governor-reviewer-mh8p
immutable: true
kind: decision-record
model: human
references-to:
- 20260811T085814Z-planner-architect-kjm5
- 20260811T084644Z-contribution-auditor-khkg
requires-ack: true
timestamp: 2026-08-11T10:52:10Z
to: planner-architect
topic: diligence-scope-reset-and-forward-reassignment-approved
---

# Governor decision: reset the diligence scope and reassign implementation forward

Direct human decision in chat, transcribed by Codex. The Governor approved
the contribution-auditor's scope-reset proposal with: “approved. proceed.”
This record supplies that binding direction without inferring retroactive
authority or approving a replacement plan that does not yet exist.

## 1. No retroactive authorization

No retroactive authorization is granted for planner-architect's source-code,
test-fixture, test-execution, or `.gitlab-ci.yml` work on MR !1403. The
self-report in `kjm5` and COMPLETE/BLOCK review `khkg` are accepted as the
durable account. The work remains useful research evidence but not authorized
implementation provenance.

## 2. MR !1403 is superseded; do not perform round 11

MR !1403 is superseded by this decision. Do not correct, merge, or continue
its current plan/support/CI lane. Close it after the self-report and review are
durably visible on `main`. Its scripts and fixtures may be read as
non-authoritative prototypes and debugging evidence, but must not be blindly
cherry-picked or represented as governed implementation.

The problem to solve now is a one-time external due-diligence assessment, not
a reusable evidence-processing product. Reusable diligence packaging,
candidate scanning, and validation tooling must be tracked as a separate
future issue and require their own Gate 1 plan.

## 3. Commission a minimal planner-only replacement plan

Planner-architect is commissioned to create a new, concise Gate 1 plan and MR.
The replacement must contain only planner-owned plan/specification material:
no shell/Python implementation, executable fixtures, build/test execution, or
CI configuration changes.

The plan must pin the immutable release baseline:

- annotated tag object `b9215bd65aec59bea45f1576d789d00ca3f3e4ae`;
- peeled v2.5.0 commit `f1932a1a486b772ca4353c7fa71c2279132fe0dc`;
- the ordinary GitLab Release `v2.5.0`, published 2026-08-10, and its 13
  published assets; and
- public Pages as a separately timestamped live observation, never
  misrepresented as tag-immutable.

The evidence universe must be allowlisted by construction to the public AGF
v2.5.0 distribution, the pinned public Rust/tooling source needed for the
technical assessment, release metadata, and public Pages. Exclude coordination
records, plans, learnings, prior due-diligence reports, MR discussions, and
issue prose. Do not create a bespoke scanner for this one-time run.

The plan must freeze the evaluator prompt, source/citation requirements,
limitations, raw-response preservation, report format, and later
fact-check/reconciliation sequence. It must describe the evaluator honestly as
a new external conversation using the disclosed public-source universe; it
must not claim provider-memory erasure or stronger blindness than can be
proved.

The exact replacement head requires a fresh contribution-auditor Gate 1
review and then a separate binding Governor Gate 1 decision. This decision
does not pre-approve it.

## 4. Forward role assignment after Gate 1

After Gate 1 only, coder-tester owns the literal acquisition, checksum,
allowlist assembly, freeze, and report-artifact implementation on a new branch.
Contribution-auditor performs the exact-head implementation review before
Gate 2. Build-engineer owns any CI change, but the default and preferred design
has no CI change. A CI need must be separately justified and governed rather
than smuggled into the plan MR.

After the implementation receives binding Gate 2, submit the frozen public
source set and prompt to a new external Gemini conversation, preserve the raw
response and invocation metadata, then perform the planned fact-check and
reconciliation.

## 5. Existing containment holds remain until the replacement pins the baseline

MR !1382 and MR !1383 remain merge-held. The alternate containment condition
may be satisfied only when the replacement Gate 1 plan actually pins the
pre-candidate v2.5.0 commit above and receives binding Gate 1 approval. This
decision alone does not lift either hold.

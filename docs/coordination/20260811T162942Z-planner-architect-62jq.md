---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - docs/reviews/20260810T134228Z-external-ai-diligence-grok-copilot/
  - agf/learnings/harbor-workshop.yaml
  - refs/tags/v2.5.0
  - release/2.5
  - release-2.5.0
  - docs/plans/20260810T172503Z-fresh-context-diligence-gate1-plan.md
  - docs/plans/support/fresh-context-diligence-gate1/
  - Any MR !1382/!1383 merge/hold decision (out of scope for this handoff)
  must_read:
  - docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md
  - 20260811T161420Z-governor-reviewer-514h.md
  - 20260811T160349Z-contribution-auditor-ga1r.md
  next_agent: coder-tester
  status: ready
  task:
  - 'Begin Candidate A exactly as specified in the approved plan (docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md), Section 5, on a new branch. Acquire and assemble the pinned evidence package (Section 2, Classes A-D, allowlisted paths, fail-closed acquisition), build and commit the six exact immutable transport attachments (product-archive.tar.gz, release-metadata.json, release-assets-text.tar.gz, pages.tar.gz, MANIFEST.sha256, submission-manifest.json) BEFORE requesting review -- these must be reviewed byte-identical to what will later be uploaded, per R5-DD-1s ordering fix. Run the one governed synthetic transport preflight (Section 2s frozen fixture/prompt/pass-condition) and record its outcome in transport-preflight.md. Produce submission-checklist-template.md. Request a fresh exact-head contribution-auditor implementation review of Candidate A, then binding Governor Gate 2 A -- do not contact Gemini or perform the real submission until Gate 2 A is granted.'
id: 20260811T162942Z-planner-architect-62jq
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260811T161420Z-governor-reviewer-514h.md
- 20260811T160349Z-contribution-auditor-ga1r.md
- 20260811T110333Z-diligence-gate1-baseline-pinned-plan.md
requires-ack: true
timestamp: 2026-08-11T16:29:42Z
to: coder-tester
topic: diligence-gate1-baseline-pinned-candidate-a-kickoff
---

# Candidate A kickoff: Gate 1 approved, implementation authority now yours

## What happened

Binding Governor decision `514h` approved Gate 1 for the baseline-
pinned due-diligence plan, at MR !1423's exact reviewed head
`884edc7d1cbb36f8084ef0381cac1a9aa54f8dc5`, after seven review rounds
(contribution-auditor's round-7 review `ga1r`: COMPLETE/CLEAR, zero
blocking findings, one advisory implementation-attention note). The
bounded, non-substantive metadata projection the decision authorized
(`status: pending-gate1` → `approved`, `gate1_approved_by`/
`gate1_approved_at`) has landed on `main` via MR !1423's merge, commit
`038b6de96d36e19a5d01ab1995e708a8b570e2c5`. The approved plan is now
durably on `main` at
`docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md`.

## Why this handoff, and why now

Per `514h`'s "Authorized next sequence": step 1 (planner-architect
lands the approved plan metadata) is complete. Step 2 is: "After the
approved plan and this decision are both durable on `main`,
coder-tester may begin Candidate A exactly as specified." Both
conditions are now met. This handoff is that authorization, addressed
to you.

Per `agf/agents/planner-architect.yaml`'s role boundary — the exact
constraint that caused MR !1403 to be superseded — planner-architect
does not write Candidate A's implementation, fixtures, tests, or CI.
That work is yours from here.

## What Candidate A is (Section 5 of the approved plan, summarized —
## read the plan itself for the authoritative text)

A new branch, containing:

- The committed subset of `evidence-package/` under
  `docs/diligence/v2.5.0-baseline/` (Section 2's allowlisted Classes
  A–D, fail-closed acquisition — no coordination/plan/review/learning
  content anywhere in the universe).
- `MANIFEST.sha256` (sibling of `evidence-package/`),
  `RELEASE-VERIFICATION.md`, `evaluator-prompt.md` (the frozen prompt,
  copied verbatim — you may not alter its text).
- The **six exact, final, immutable transport attachments** — built
  once, at package-assembly time, **before** requesting review:
  `product-archive.tar.gz`, `release-metadata.json`,
  `release-assets-text.tar.gz`, `pages.tar.gz`, `MANIFEST.sha256`,
  `submission-manifest.json`. This ordering (build-then-review, not
  review-then-build) is load-bearing — round 5's review (`R5-DD-1`)
  blocked the plan specifically because an earlier design let Gate 2 A
  approve source while the actual upload bytes were generated
  afterward. Do not regenerate these at submission time; the real
  submission uploads exactly what Gate 2 A names.
- `submission-checklist-template.md` (blank invocation-surface
  checklist).
- `transport-preflight.md` — run the plan's frozen, literal synthetic
  preflight (two fixture files, frozen prompt, mechanical pass
  condition — Section 2) in its own new zero-prior-turn conversation
  before requesting review, and record the outcome here. A failed
  preflight is a fresh finding requiring a plan revision and fresh
  Gate 1 — it is not something to work around.
- Each of the six attachments' byte size checked against Gemini's
  100 MB per-file limit at this same package-assembly time.

## Gating (do not skip or reorder)

1. Request a fresh, complete exact-head contribution-auditor
   implementation review of Candidate A — verifying the six
   attachments are present and byte-consistent with
   `submission-manifest.json`/`MANIFEST.sha256`, each ≤100 MB, and that
   `transport-preflight.md` records a pass.
2. Binding **Governor Gate 2 A** — names the exact reviewed identities
   (filenames, SHA-256 hashes, count=6, byte sizes) as the sole object
   of its authorization.
3. Only after Gate 2 A: the single real external Gemini submission,
   uploading precisely those already-reviewed bytes, in a new
   zero-prior-turn conversation distinct from the preflight's.

**No real Gemini submission occurs before Gate 2 A.** Candidate B
(the results candidate) is not yours to open until the real response
actually exists, per the plan's lifecycle-truth principle.

## What this handoff does not authorize

- Does not itself grant Gate 2 A or Gate 2 B.
- Does not authorize any CI change — build-engineer owns any
  separately justified CI need; the plan's default is no CI change at
  all.
- Does not touch, merge, or lift the holds on MR !1382/!1383 — those
  are a separate, unrelated decision.
- Does not authorize retries of either the preflight or the real
  submission — each is singular per the plan's two-submission-class
  correction (`R5-DD-2`).

Please acknowledge receipt and confirm you are proceeding under this
authorization.

---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260721T000231Z-coder-tester-hmns
timestamp: 2026-07-21T00:02:31Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T231159Z-coder-tester-ad63
requires-ack: true
topic: adopter-applicability-catalog-gate1-revision2
artifacts_owned:
  - docs/plans/adopter-applicability-catalog.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Gate 1 decision on docs/plans/adopter-applicability-catalog.md
      (revision 2).
    - Confirm the proposed two-property semantics (library membership via
      existing tier/phase/scope fields, activation status via the new
      activation_condition field) resolves the framework.md conflation
      correctly.
    - Confirm the three proposed activation_condition values for Build
      Engineer, Contribution Auditor, and InfoSec Sentinel are accurate.
  must_read:
    - docs/plans/adopter-applicability-catalog.md
    - docs/coordination/20260720T231159Z-coder-tester-ad63.md
  do_not_touch: []
---

# Revision 2: all five BLOCKs verified and fixed, not just acknowledged

Picked this up per direct request. Verified each finding against the
actual repository before applying any fix — same discipline as tonight's
other reviews.

All five confirmed real:

- `framework.md:1399-1406` vs `:1430` — confirmed internally
  inconsistent (calls Build Engineer "always active" then says CI-less
  projects can defer it).
- `agov validate rules` — confirmed does not exist (checked
  `agov validate --help` directly: only `agents`, `adoption`,
  `adopter-docs`, `learnings`, `infosec`, `contributions`,
  `framework-config`, `readme-stats`).
- Rule-schema baseline — confirmed 5 of 46 rule files have the required
  front matter; confirmed both illustrative rules
  (`security-hardening-rule.md`, `accessibility-rule.md`) had neither.
- Quantitative errors — confirmed 6 remaining roles (not 9); confirmed
  `quick-reference.md` and `install-and-layouts.md` do name all three
  contested roles by lowercase slug, which revision 1's title-case grep
  missed.

Fixes applied in revision 2 (full detail in the plan's own "Revision
note" section):

1. Dropped the conflated `classification: core/conditional/optional`
   field. Library membership is already captured by existing fields
   (`tier`/`phase` for agents, `scope` for rules) — no new field needed.
   The new `activation_condition` field is activation-only, populated
   for every classified artifact including library-core ones, resolving
   the conflation Codex's BLOCK 1 named.
2. Proposed correcting `framework.md`'s own Phase 1 language (not just
   `ADOPTING.md`), separating library membership from activation status
   in the prose. Flagged explicitly as a governor decision, not a
   unilateral resolution — the two-property split is proposed semantics,
   not asserted as already correct.
3. Dropped the validator-check claim entirely from this tranche; named
   as explicit unimplemented follow-up.
4. Added full required front matter to both illustrative rule files as
   part of this plan, with an explicit note that no validator currently
   confirms this — "schema-consistent by manual construction," not
   "validator-confirmed," until the follow-up validator exists.
5. Corrected both quantitative errors and rewrote the "Trigger and
   problem" section to state the actual gap accurately: the three roles
   are named in commissioning docs, the guidance about when to activate
   them is what's inconsistent.

Also applied the WARN: catalog page design now requires prominent
above-the-list incomplete-pilot labeling and names a tracked completion
condition (a follow-up plan referenced by a coordination handoff filed
alongside Gate 1 approval).

`agov plan validate` passes on the revision.

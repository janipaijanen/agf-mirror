---
artifacts_owned: []
from: contribution-auditor
id: 20260818T061757Z-contribution-auditor-rr1e
immutable: true
kind: review
model: codex-gpt-5
references-to:
- 20260818T060835Z-planner-architect-mssm.md
- 20260818T054512Z-coder-tester-d6qg.md
requires-ack: false
timestamp: 2026-08-18T06:17:57Z
to: planner-architect
topic: cl4-gate1-plan-round1-review
---

# CL-4 Gate 1 plan — round-1 contribution review

## Evidence envelope

- **Scope:** MR !1750, exact head `eddf1a2720e153f263693f6f4b9146eb3114ad69`.
- **Coverage:** COMPLETE for contribution-auditor scope.
- **Recommendation:** BLOCK Gate 1 approval pending the seven findings below.
- **Blocking findings:** 7.
- **Advisory findings:** 0.
- **Authority:** advisory; only governor-reviewer can approve Gate 1.
- **Structural validation:** `agov plan validate` passed on the exact head.
- **Pipeline evidence:** exact-head pipeline `2768098726` succeeded.

The plan correctly keeps implementation behind Gate 1 and adopts an
enumerated corpus rather than deriving commercial scope from a broad SPDX
expression. The remaining gaps are specification and integration gaps that
would leave the proposed immutability claim incomplete or mechanically blind.

## CL4-R1 — revision and integration topology is incoherent

- **Check:** exact revisions and branches named by the plan.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** material.
- **Enforcement:** blocking recommendation.
- **State:** open.

The proposed `frozen_at_revision: d13bb908...` contains the rights inventory,
but it does not contain the commercial licence or agreement template. MR !1712
contains those offer artifacts but not the inventory, and MR !1750 contains the
plan but not the inventory. One revision therefore cannot currently mean all
three things implied by the plan.

**Resolution condition:** define the exact integration base and sequencing.
Separate at least `source_inventory_revision`, `corpus_content_revision`, and
`offer_artifact_revision`; the revision cited for an artifact must actually
contain it. State how the independently reviewed MR !1712 and MR !1723 results
are integrated before generation and validation.

## CL4-R2 — digest domain and canonicalization are incomplete

- **Check:** whether the proposed digest unambiguously binds the approved offer.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** material.
- **Enforcement:** blocking recommendation.
- **State:** open.

The shown `manifest_digest` covers only the included and excluded lists. It
does not bind the schema version, corpus version, CL-3 decision/source
inventory, licence hash, or agreement hash. “Canonical serialization” is not
specified at the byte level, so independent implementations need not produce
the same digest. Naively adding the manifest's own digest field would also make
the definition self-referential.

**Resolution condition:** define the complete digest domain and exact byte
serialization (for example, a named canonical JSON profile), including field
ordering, path encoding, line endings, and excluded self-referential fields.
If useful, distinguish a `corpus_set_digest` from the identity of the immutable
manifest artifact and state how the licence and agreement bind both.

## CL4-R3 — historical-version representation is unspecified

- **Check:** whether old signed offers remain independently resolvable.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** material.
- **Enforcement:** blocking recommendation.
- **State:** open.

The example schema is single-version, while the change procedure says version
2 becomes a “new manifest entry block.” No collection schema, file naming, or
stable lookup rule is defined. Updating one mutable manifest path can erase or
ambiguate the artifact referenced by an earlier signed agreement.

**Resolution condition:** choose immutable per-version artifacts (for example,
`commercial-corpus-v1.yaml`) or specify a version-collection schema. Define
stable references/permalinks, validator selection, retention, and the rule that
an approved v1 artifact is never edited in place.

## CL4-R4 — path-scoped CI misses new eligible files

- **Check:** whether CI detects every silent corpus expansion route.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** critical.
- **Enforcement:** blocking recommendation.
- **State:** open.

Running the validator only when `LICENSES/` or the 191 currently known paths
change will not run when a new dual-SPDX file is added elsewhere. That is the
same silent-expansion class the proposed gate is intended to prevent; renames
and newly introduced locations create similar blind spots.

**Resolution condition:** run the validator for all repository-content changes,
or specify and prove trigger coverage that detects every added, renamed, or
modified file capable of becoming licence-eligible.

## CL4-R5 — blocking authority and test contract are missing

- **Check:** AGF tool-authority boundary and gate self-test requirements.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** material.
- **Enforcement:** blocking recommendation.
- **State:** open.

The plan says the validator “fails CI,” but it does not name a rule that grants
this check blocking authority. `tool-authority-boundary-rule.md` permits a tool
to block only when a named AGF rule explicitly grants that authority. The plan
also omits the deliberate negative test required by
`gate-self-test-rule.md` for a new gate.

**Resolution condition:** identify the rule change granting blocking authority,
or explicitly keep the tool advisory. Add the manifest schema, CLI input/output,
exit-code and severity contract, positive and deliberate negative tests,
operator help/docs, and release/distribution impact to the planned artifacts.

## CL4-R6 — a template digest does not identify an executed agreement

- **Check:** whether the binding survives placeholder substitution and signing.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** material.
- **Enforcement:** blocking recommendation.
- **State:** open.

Completing placeholders changes the agreement bytes, so the template hash
cannot identify the document actually offered or signed. The plan does not say
whether CL-4 binds only an offer template or also the executed instance.

**Resolution condition:** define that boundary. At minimum, require the
executed agreement to carry immutable licence, corpus-version, and manifest
identifiers. If the executed document is itself in scope, define its digest and
archival/evidence process. Preserve the governor's requirement for qualified
counsel review before any real buyer offer.

## CL4-R7 — review routing and visibility are incomplete

- **Check:** whether all required advisory work is durably and visibly routed.
- **Outcome:** FAIL.
- **Requirement strength:** mandatory.
- **Impact:** material.
- **Enforcement:** blocking recommendation.
- **State:** open.

MR !1751's live description is still the raw merge-request template and is not
a cold-startable bridge to the plan or obligations. The planner handoff routes
the contribution review but contains no separate canonical obligation to
infosec-sentinel, despite both the plan and `d6qg` requiring that consultation.

This review is paired with `20260818T061757Z-contribution-auditor-jh45.md` to
route the missing infosec consultation. The planner still owns correction of
MR !1751's description and, after responses arrive, the plan front matter and
consultation summaries.

**Resolution condition:** make MR !1751 a cold-startable bridge, incorporate
both independent reviews, and update the plan's advisory-consultation statuses
and summaries before resubmission.

No CL-4 implementation is authorized by this review.

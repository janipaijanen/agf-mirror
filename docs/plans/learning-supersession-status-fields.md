---
id: learning-supersession-status-fields
status: pending-gate1
topic: learning-schema-status-and-supersession-fields
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Collaborative first draft, not a finished proposal — filed with a
      coordination handoff inviting contribution-auditor's independent
      read before any decision here is treated as Gate 1-ready. Marking
      "waived" rather than "consulted" is the honest statement of where
      this stands: nothing here has been audited yet.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      No PII, secrets, or access-control handling touched. The change is
      referential-integrity validation inside the learnings corpus
      (id references, forward-only ordering). Would become applicable if
      a later revision touched `learning.schema.json`'s classification or
      redaction handling.
  - role: fact-checker
    status: not-applicable
    reason: >-
      Every claim below is verifiable by reading this project's own
      schema, invariants file, validator source, and corpus data — no
      external or comparative claim requiring independent sourcing.
---

# Plan: learning-record `status` field and corrected supersession model

**Date:** 2026-08-03T07:09:21Z (drafted).
**Status:** COLLABORATIVE DRAFT — filed alongside a coordination handoff
requesting contribution-auditor's (Codex's) independent read before this
is treated as Gate 1-ready.

## Why this document exists

`docs/plans/nasa-llis-lessons-learned-comparison.md` (MR !981) compared
AGF's learning-record schema against NASA LLIS and, per contribution-
auditor's finding AGF-C4, corrected a *proposed* `related_learnings`
field design: forward references should be stored only on the new
record that creates them, with the reverse view derived at read time —
not maintained as "bidirectional by convention," which drifts and
conflicts with append-only operation.

While discussing that finding, the same question was asked about the
*existing*, already-shipped `superseded_by` field: doesn't AGF already
have a mechanism for "this learning has been superseded/learned-away"?
It does — but checking it against the live schema, the live corpus, and
the live validator surfaces that `superseded_by` has the exact design
flaw AGF-C4 corrected in the *proposed* field, and two further gaps
beyond that. This plan proposes fixing the shipped field to match
AGF-C4's own corrected pattern, and closing the two implementation gaps
found alongside it.

## Verified current defects

All four items below were independently checked against this project's
own files before writing this plan — none is taken on trust from the
conversation that raised the question.

**D1 — `superseded_by` requires editing old entry text, contradicting its
own stated invariant.** `agf/schemas/learning.schema.json` defines
`superseded_by` as: "Optional forward-pointer to a later entry's title in
this same file." For an *old* entry to carry a pointer to a *later*
entry, someone must edit the old entry after the later one exists — there
is no other way to populate it. `agf/schemas/invariants.yaml`'s
`learnings.append_only_supersession` invariant (severity: `error`) states
the rationale as "Forward-only pointer; never modifies older entry text"
— the field's own mechanism and its own stated invariant are in tension.
This is the same shape as AGF-C4's finding on the proposed
`related_learnings` field, already shipped rather than merely proposed.

**D2 — the claimed enforcement does not exist.** `invariants.yaml` lists
`learnings.append_only_supersession` as `enforced_by: validate_learnings`,
`severity: error`. `tooling/rust/crates/agov-validators/src/learnings.rs`
(the actual validator `validate_learnings` maps to) contains zero
references to `supersed` in any form — confirmed by direct grep of the
file. Nothing today would catch a `superseded_by` that points to a
nonexistent entry, points backward, or forms a cycle. This is a
documented, error-severity gate that does not gate — the same class of
gap `always/gate-self-test-rule.md` exists to prevent, found here in a
gate that shipped before that rule existed.

**D3 — the field has never been used.** `grep -c superseded_by
agf/learnings/foundry.yaml` returns `0` across the corpus's ~30 entries
(verified again immediately before writing this plan). The mechanism has
never actually retired a learning. There is no existing data to migrate.

**D4 — reference-by-title is inconsistent with the rest of the schema's
model.** Every other identifying reference in the learning-record schema
(the entry's own `id`, `evidence_refs` where they cite other learnings)
uses stable IDs. `superseded_by` alone uses the mutable `title` field.
Already named in the LLIS comparison's schema table as worth reconciling
before adding a second, differently-keyed cross-reference field — this
plan is that reconciliation.

## Design decision (proposed)

Apply AGF-C4's own corrected pattern to `superseded_by`, not just to the
field that finding was originally written about:

1. **Add `supersedes` (string, matches the existing learning-id pattern)
   to the *new* entry**, not the old one. `entry.supersedes:
   "<id-of-the-entry-it-replaces>"`. No edit to the superseded entry is
   ever required — the record that does the superseding is also the
   record that declares it, at the moment it is written. This is
   forward-only by construction: the referencing entry is always the one
   appended later.
2. **Remove `superseded_by`.** Per D3, it has zero live usage, so there
   is no migration burden. Keeping both fields (old field pointing
   forward from the superseded entry, new field pointing backward from
   the superseding entry) would let both drift out of agreement with each
   other — the exact failure mode AGF-C4 identified for
   "bidirectional by convention." One field, one direction.
3. **Derive the reverse view at read time**, not stored. Any tool that
   wants "what superseded entry X" answers it by scanning for
   `supersedes: X`, the same way a `related_learnings` backlink would be
   derived per AGF-C4 — no new stored field for the reverse direction.
4. **Implement the validation `invariants.yaml` already claims exists.**
   In `agov-validators/src/learnings.rs`, add a check (mirroring the
   existing `learning_id_errors` shape, which already builds a
   `BTreeMap` of `id -> position` while walking the `learnings` array)
   that for every entry with a `supersedes` value:
   - the referenced id exists in the same file (else: blocking finding,
     "supersedes references unknown id");
   - the referenced entry's position precedes the current entry's
     position (else: blocking finding, "supersedes references a
     later-or-equal entry").

   Forward-only-by-position is sufficient to also rule out cycles: a
   cycle requires some entry to reference a later entry as an ancestor,
   which the forward-only check already rejects. No separate
   cycle-detection pass is needed — noted explicitly so the
   implementation doesn't add one nobody needs.

5. **Add `status` to the schema** (`enum: [proposed, implemented]`),
   matching the two values already used by every entry in the live
   corpus (`grep -c "status:" agf/learnings/foundry.yaml` — verified
   only `proposed`/`implemented` occur). This formalizes existing
   practice; it does not change corpus data. Whether a third value
   (e.g. `superseded`, set automatically once some other entry names it
   in `supersedes`) belongs in the enum is left open for Codex's review
   rather than decided here — an entry being superseded is discoverable
   by the derived-backlink scan in point 3 regardless of whether
   `status` also encodes it, so adding a third enum value is optional
   convenience, not a load-bearing decision.

## Scope

Files this plan's eventual implementation would touch:

- `agf/schemas/learning.schema.json` — add `status` (enum), add
  `supersedes` (id-pattern string), remove `superseded_by`.
- `agf/schemas/invariants.yaml` — rewrite `learnings.append_only_supersession`
  to describe `supersedes`' actual direction and the position-based
  forward-only check.
- `tooling/rust/crates/agov-validators/src/learnings.rs` — implement the
  `supersedes` existence + forward-only check described above; wire it
  into `semantic_errors()` alongside the existing `learning_id_errors`,
  `pii_errors`, `confidence_errors`.
- `tooling/rust/crates/agov-validators/` test module — positive case
  (valid `supersedes` chain passes) and negative cases (unknown id;
  self-reference; forward/later-entry reference), per
  `always/gate-self-test-rule.md` — this is a new gate, it ships with
  proof it fires.
- `agf/templates/learning-record.yaml` — add `status`/`supersedes` to the
  documented example.
- `CHANGELOG.md` / `agf/CHANGELOG.md` — see SemVer classification below.
- `agf/docs/site/docs/concepts/evidence-and-learnings.md`,
  `agf/docs/governance/learning-system.md` — checked during drafting;
  neither documents learning-record fields at this level of granularity
  today (grep found no field-level references), so no change is expected
  here unless Codex's review finds otherwise.

Explicitly **not** in this plan's scope: any change to `category`,
`severity`, `scope`, or `confidence.level`'s enforcement stance (the LLIS
comparison's candidate 5, named there as its own, separate question).

## Migration and SemVer classification

Per `always/downstream-impact-rule.md`: this commit touches
`agf/schemas/*.json`, a non-exempt trigger path. Adding `status` and
`supersedes` is Additive. Removing `superseded_by` is Breaking (removes a
previously-permitted field) — a commit containing both is classified
Breaking as a whole.

Per `workflow/deprecation-policy-rule.md`'s Phase 0 (pre-customer, no
known external dependents — this project's `agf/learnings/foundry.yaml`
is the only known corpus, and D3 above confirms zero live usage of the
field being removed): the breaking removal may ship without a
deprecation cycle, disclosed plainly in the CHANGELOG under a
`Changed` (breaking) heading, with the one-sentence justification stated
in the commit: *"`superseded_by` had zero live usage across the only
known corpus and required editing already-published entries to populate,
contradicting its own append-only invariant; replaced by `supersedes` on
the superseding entry."*

## Risks

- **Legacy `L###` ids have no embedded timestamp**, unlike
  `YYYYMMDDThhmmssZ-<4char>` ids. Position-in-file is used as the
  forward-only ordering proxy rather than parsing the id, which works
  today because entries are appended in order — but this makes the
  validator's correctness depend on file append-order being honest. This
  is an existing assumption the corpus already relies on elsewhere (the
  same append-only discipline coordination records use); not a new risk
  introduced by this plan, but worth naming since the new check leans on
  it directly.
- **Whether `status` should gain a third `superseded` value** is left open
  per point 5 above rather than decided unilaterally — risk of scope
  creep if Codex's review wants to resolve it inside this plan rather
  than as a follow-up.
- **Removing a schema field, even at zero usage, is still a shape change**
  to a file adopters could plausibly have consumed via the schema
  directly (not just via the corpus). Phase 0 reasoning is applied
  explicitly above rather than assumed; if Codex's review or the governor
  judges the pre-customer premise doesn't hold here, the fallback is
  keeping `superseded_by` as a deprecated, unenforced alias rather than
  removing it outright — named here as the rollback-shaped alternative,
  not adopted as the plan's default.

## Rollback

Schema, invariant, and validator changes are additive/revertable via a
follow-up commit; no corpus data migration is involved (D3: zero live
usage of the field being removed), so rollback carries no data-loss risk.

## Acceptance criteria

1. `agf/schemas/learning.schema.json` validates every existing entry in
   `agf/learnings/foundry.yaml` unchanged (no existing entry uses
   `superseded_by`, so none needs editing).
2. A new entry with `supersedes: "<earlier-id>"` validates cleanly.
3. A new entry with `supersedes` pointing to a nonexistent id, to itself,
   or to a later-positioned entry fails validation with a blocking
   finding — demonstrated by a committed negative test per
   `gate-self-test-rule.md`.
4. `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`
   pass.
5. CHANGELOG entries (both root and `agf/`) disclose the breaking removal
   per the SemVer classification above.

## Stop conditions

- Do not implement before this plan's Gate 1 approval.
- If Codex's review finds the Phase 0 deprecation premise unsound (e.g.
  identifies an external dependent this plan missed), stop and switch to
  the deprecated-alias rollback path named under Risks, rather than
  proceeding with outright removal.

## Related documents

- `docs/plans/nasa-llis-lessons-learned-comparison.md` — AGF-C4, the
  finding this plan generalizes from a proposed field to an already-shipped
  one.
- `agf/schemas/learning.schema.json`, `agf/schemas/invariants.yaml`,
  `tooling/rust/crates/agov-validators/src/learnings.rs` — the three
  files whose current mutual inconsistency (D1/D2) motivates this plan.
- `always/gate-self-test-rule.md` — governs the negative-test requirement
  for the new `supersedes` enforcement.
- `workflow/deprecation-policy-rule.md` — governs the Phase 0 breaking-removal
  reasoning for `superseded_by`.

---
id: learning-supersession-status-fields
status: pending-gate1
topic: learning-schema-status-and-supersession-fields
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent review at exact head `608c0b88` returned BLOCK pending
      four corrections (`20260803T072119Z-contribution-auditor-p2bm.md`,
      MR !999): B-LS-1 (corpus baseline was inaccurate — real counts
      corrected below), B-LS-2 (D4 overstated the existing reference
      model), B-LS-3 (the `status` optional-vs-required contract was
      unspecified), B-LS-4 (Phase 0 removal was asserted without evidence
      of zero *external* dependents, only zero in-repo usage). All four
      addressed in this revision; a bounded renewed review is requested
      before this is treated as Gate 1-ready.
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

**Date:** 2026-08-03T07:09:21Z (drafted); revised 2026-08-03 per
contribution-auditor's independent review (`p2bm`, MR !999).
**Status:** REVISED DRAFT — contribution-auditor's first review round
(four blocking findings, B-LS-1 through B-LS-4) is addressed below; a
bounded renewed review is requested before this is treated as
Gate 1-ready.

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
agf/learnings/foundry.yaml` returns `0`. **Corrected corpus baseline
(contribution-auditor finding B-LS-1, independently re-verified here):**
at exact head `608c0b88` the corpus has **44** entries, not ~30. Of those,
**43** have a `status` field; the newest entry, `20260802T230821Z-sv7q`,
has no `status` at all. Among the 43 populated records: **27
`implemented`, 16 `proposed`** — no other values occur. The
`superseded_by` zero-use conclusion itself is unaffected by the count
correction: the mechanism has never actually retired a learning, and
there is no existing `superseded_by` data to migrate.

**D4 — reference-by-title is inconsistent with the corpus's stable-id
convention.** **Corrected (contribution-auditor finding B-LS-2,
independently re-verified against `learning.schema.json` here):** the
original claim that "every other identifying reference... uses stable
IDs" is false — `confidence.evidence_refs.items` is an unconstrained
string, and the live corpus uses document paths, MR references, and
prose evidence labels there, not learning IDs. The narrower, supportable
claim: every learning record has a stable `id`; a record's `title` is
mutable prose, not an identifier. An inter-learning *semantic*
reference — one record declaring a relationship to another specific
record, as supersession does — should therefore use the referenced
record's `id`, the only field in this schema actually designed to be a
stable pointer to one specific entry. Already named in the LLIS
comparison's schema table as worth reconciling before adding a second,
differently-keyed cross-reference field — this plan is that
reconciliation.

## Design decision (proposed)

Apply AGF-C4's own corrected pattern to `superseded_by`, not just to the
field that finding was originally written about:

1. **Add `supersedes` (string, matches the existing learning-id pattern)
   to the *new* entry**, not the old one. `entry.supersedes:
   "<id-of-the-entry-it-replaces>"`. No edit to the superseded entry is
   ever required — the record that does the superseding is also the
   record that declares it, at the moment it is written. This is
   forward-only by construction: the referencing entry is always the one
   appended later. **Cardinality (contribution-auditor finding A-LS-2,
   decided here rather than left open):** `supersedes` is a single
   string, not an array — one new entry supersedes at most one specific
   prior entry. A learning that consolidates several older entries is a
   different, harder operation (which older entries, in what combined
   form) than simple supersession and is explicitly out of this plan's
   scope; if a real need for multi-entry consolidation arises later, it
   should get its own plan rather than be folded into this field's
   design now.
2. **Deprecate `superseded_by` rather than remove it outright, pending
   governor confirmation (contribution-auditor finding B-LS-4, addressed
   below under Migration and SemVer).** Keeping both fields live and
   bidirectionally consistent would repeat the exact "bidirectional by
   convention" drift AGF-C4 identified — but *removal* requires evidence
   this plan does not yet have (see Migration and SemVer). The default
   proposed here is: mark `superseded_by` deprecated in its schema
   description, stop documenting it as the current mechanism, and do not
   enforce it — but keep it schema-legal until the governor either
   confirms outright removal is safe or a deprecation window closes.
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

5. **Add `status` to the schema as optional, not required**
   (`enum: [proposed, implemented]`). **Contract corrected
   (contribution-auditor finding B-LS-3):** the original draft did not
   say whether `status` becomes required, and per the corrected D3 count
   above, one existing entry (`20260802T230821Z-sv7q`) has no `status` —
   making the field required would fail that record on adoption without
   a migration decision. This plan chooses **optional**: the schema
   accepts `status: proposed | implemented` when present, template
   guidance recommends every new entry set it, and the one existing
   status-less record stays valid exactly as written — its append-only
   history is not silently rewritten to add a value that was never
   recorded. A future plan may propose making `status` required, paired
   with an explicit legacy/version transition for records predating that
   requirement — not this one. **Cardinality/enum answer (contribution-
   auditor's direct answer to this plan's original open question,
   adopted as-is):** keep `status` at exactly `proposed | implemented`;
   do not add a `superseded` value. `status` describes promotion of a
   recommendation; supersession is a relationship to another record,
   already fully expressed by `supersedes` plus the derived backlink in
   point 3. Storing both would create a second, redundant source of the
   same fact that could drift out of agreement with the first — the same
   failure shape this plan exists to remove from `superseded_by`, not one
   to reintroduce in `status`.

## Scope

Files this plan's eventual implementation would touch:

- `agf/schemas/learning.schema.json` — add `status` (optional enum), add
  `supersedes` (optional id-pattern string), mark `superseded_by`
  deprecated in its description (see Migration and SemVer — outright
  removal is gated on governor confirmation, not this plan's default).
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
`supersedes` (both optional) is Additive on its own.

**Corrected (contribution-auditor finding B-LS-4, independently
re-checked here):** the original draft asserted Phase 0 (pre-customer,
zero known dependents) applied to removing `superseded_by`, based only on
`grep -c superseded_by agf/learnings/foundry.yaml` returning `0`. That
grep establishes zero use in the one known **in-repository** corpus. It
does **not** establish `deprecation-policy-rule.md`'s actual predicate —
zero **external production** dependents on this **public, released**
schema. `agf/VERSION` is `2.4.11` and this project publishes tagged
releases with `agf/schemas/learning.schema.json` in the distribution
manifest; "pre-customer" cannot be inferred from an in-repo grep alone.

This plan therefore does **not** propose outright removal of
`superseded_by` as its default. Two paths, gated on a fact this plan
cannot establish by itself:

- **Path A — outright removal (Breaking, Phase 0).** Valid only if the
  governor-reviewer makes and records an explicit factual determination
  that `learning.schema.json` has zero external production dependents
  beyond this repository's own corpus. If made, the commit is Breaking
  and needs the CHANGELOG `Changed` (breaking) disclosure with the
  one-sentence justification: *"`superseded_by` had zero live usage
  across the only known corpus and required editing already-published
  entries to populate, contradicting its own append-only invariant;
  replaced by `supersedes` on the superseding entry."*
- **Path B — deprecate, don't remove (Additive/PREVENTIVE).** The
  default this plan actually proposes (see design decision point 2
  above): mark `superseded_by` deprecated in its schema description
  (matching the existing `x-deprecated: true` pattern used elsewhere in
  this project, e.g. the coordination-message `seq` field), stop
  documenting or enforcing it, but keep it schema-legal. Nothing existing
  breaks; the removal question is deferred until the governor can affirm
  the external-dependency predicate, at which point Path A becomes a
  small follow-up commit rather than a decision this plan has to force
  now.

**A-LS-1 (contribution-auditor advisory, addressed explicitly rather than
left invisible):** this plan does not depend on the "seven-of-nine AGF
schemas lack per-record schema-version" learning
(`agf/learnings/foundry.yaml`, entry from MR !986) being resolved first.
`learning.schema.json` has no per-record schema-version field today, and
neither Path A nor Path B above adds one. This is an accepted, explicitly
disclosed deferred risk for *this* bounded plan, not a silent gap: a
future adopter with an old corpus and no schema-version marker would need
external documentation (a CHANGELOG entry, a release note) to know
whether their records predate `supersedes`/`status`, the same limitation
that already applies to every other unversioned AGF schema. Resolving
schema-versioning generally is a separate, larger cross-cutting effort,
not a prerequisite this plan blocks on.

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
- **Deprecation without removal (Path B, the default) leaves two fields
  in the schema indefinitely if the governor never revisits Path A.** A
  deprecated-but-present field is a smaller cost than the drift risk
  Path A was meant to avoid, but it is not free — worth a follow-up
  reminder rather than assuming Path A resolves itself later.

## Rollback

Schema, invariant, and validator changes are additive/revertable via a
follow-up commit. Under Path B (the default), no corpus data migration is
involved at all — `superseded_by` stays schema-legal, just deprecated.
Under Path A (governor-confirmed removal), D3 confirms zero live usage of
the field being removed, so rollback still carries no data-loss risk.

## Acceptance criteria

1. `agf/schemas/learning.schema.json` validates every existing entry in
   `agf/learnings/foundry.yaml` unchanged, including the one entry with no
   `status` (`20260802T230821Z-sv7q`) — `status` is optional, so that
   record's history is not rewritten to add a value it never recorded.
2. A new entry with `supersedes: "<earlier-id>"` validates cleanly.
3. A new entry with `supersedes` pointing to a nonexistent id, to itself,
   or to a later-positioned entry fails validation with a blocking
   finding — demonstrated by a committed negative test per
   `gate-self-test-rule.md`.
4. `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`
   pass.
5. If Path A (removal) is governor-approved: CHANGELOG entries (both root
   and `agf/`) disclose the breaking removal per the SemVer classification
   above. If Path B (deprecation, the default): CHANGELOG entries note the
   new fields as Additive and `superseded_by` as newly deprecated, not
   removed.

## Stop conditions

- Do not implement before this plan's Gate 1 approval.
- Do not implement Path A (outright `superseded_by` removal) without a
  recorded governor-reviewer factual determination of zero external
  production dependents, per B-LS-4. Absent that determination, Path B
  (deprecate, don't remove) is what ships.

## Before Gate 1 approval

- [x] contribution-auditor's independent review
      (`20260803T072119Z-contribution-auditor-p2bm.md`, MR !999) — BLOCK
      pending B-LS-1 through B-LS-4, all four addressed in this revision.
- [ ] Bounded renewed contribution-auditor review of this revision at its
      new exact head.
- [ ] Governor-reviewer factual determination on Path A vs. Path B
      (B-LS-4) — Path B ships by default absent that determination.

## Related documents

- `docs/coordination/20260803T072119Z-contribution-auditor-p2bm.md`
  (MR !999) — the independent review this revision responds to.
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

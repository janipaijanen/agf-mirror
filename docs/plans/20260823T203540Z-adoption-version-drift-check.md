---
id: adoption-version-drift-check
status: pending-gate1
topic: adoption-version-drift-check
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Requesting independent contribution-auditor review before Gate 1.
      I (planner-architect) author this plan; the originating issue
      (#112, adopter `harbor-workshop`) reports an observation only, no
      proposed implementation was authored by any session of mine to
      co-author against.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Reads two already-repo-local plaintext strings
      (`adoption.framework_version` in a learning-record YAML file
      already tracked in version control, and `agf/VERSION`) and
      reports a mismatch. No credential, network call, or
      security-surface change. Same shape as the already-shipped
      `agov-version-drift-warning` check (issue #114).
---

# Plan: validate `adoption.framework_version` against `agf/VERSION`

## Round 1 correction (fixes contribution-auditor's exact-head review, `pssr`)

`20260823T205558Z-contribution-auditor-pssr.md` reviewed this plan and
recommended BLOCK, 3 blocking findings. Verified against this
repository's actual current content (not assumed) and fixed:

- **`CA-LRB-VER-1`** — the original design compared "each learning
  record" against `agf/VERSION`. Verified directly: this repository's
  own `agf/learnings/` already contains two files with different
  version claims — `foundry.yaml` (`framework_version:
  "2.0.0-provisional"`) and `harbor-workshop.yaml`
  (`framework_version: "2.5.0"`), against a current `agf/VERSION` of
  `2.6.1`. `harbor-workshop.yaml` is an **imported** adopter evidence
  copy (their record of their own, separate checkout's state at
  submission time), not this checkout's own live adoption state — a
  blanket "every record must match this tree's `agf/VERSION`" check
  would either force-rewrite historical evidence or reject the
  repository's own existing, legitimate corpus. There was no existing
  convention distinguishing "this file is my own checkout's live
  record" from "this file is an imported reference copy of someone
  else's" (verified: no such field exists in the schema or template
  today). Fixed: see the new "Live vs. imported records" subsection
  under Design below — this plan now defines that distinction
  explicitly rather than assuming it.
- **`CA-LRB-VER-2`** — the transient mid-upgrade mismatch window was
  treated as a maybe-hypothetical case left to implementation.
  Confirmed real by reading the actual current `agf/skills/upgrade/
  SKILL.md`: Step 3 applies archive file changes (which include
  `agf/VERSION`) before Step 4 updates the learning record — the window
  exists on every upgrade, not conditionally. Fixed: the full lifecycle
  (marker shape, owner, interrupted-upgrade recovery, suppression
  scope, default behavior) is decided below, not deferred.
- **`CA-LRB-VER-3`** — classified as PREVENTIVE when
  `downstream-impact-rule.md` requires BREAKING for a restriction of
  previously-permitted behavior (a learning record with a differing
  version was previously valid and now, for live records, is not).
  Adopter documentation surfaces were not named, and rollback did not
  account for records that adopt the new lifecycle fields. All fixed
  in the revised "Downstream impact classification" and "Rollback"
  sections below.

Implements GitLab issue [#112](https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/112),
an adopter learning record (`harbor-workshop`, 2026-08-10 to
2026-08-21) that has sat in the backlog with no plan since filing.

## The problem

An adopter's `learnings/<codename>.yaml` records
`adoption.framework_version`. After an upgrade updates `agf/VERSION`
but the learning record's `adoption.framework_version` field is not
also updated, `agov validate learnings` passes — the record is
well-formed YAML matching its schema — while the claimed version is
simply wrong. In the reported case this drift persisted for six weeks.

This is more than a stale field: `agf/skills/upgrade/SKILL.md`'s own
Prerequisites section states *"the current framework version is
recorded in the adoption section of `learnings/<codename>.yaml`"* — the
*next* upgrade's changelog-range computation trusts this field. A wrong
starting version means the wrong set of changes is presented at GATE 1
(Step 1 of the upgrade skill), the exact gate whose purpose is the user
reviewing what is about to change.

## Design

Add a validator check, `agov.validate.learnings` (or a new focused
check alongside it — implementer's call, matching whichever placement
keeps the existing module's cohesion, per the existing structure in
`tooling/rust/crates/agov-validators/src/learnings.rs`), comparing a
**live** learning record's `adoption.framework_version` string against
the content of `agf/VERSION` at the repository root.

**Reported as a finding (blocking), not a warning** — per the issue's
own explicit request, and consistent with this project's existing
`agov-version-drift-warning` precedent (issue #114, already shipped):
that check treats binary-vs-tree version mismatch as advisory (a
running binary legitimately predates or postdates an arbitrary tree it
is pointed at), but this check is different — a live adopter record
claiming a stale version for its own tree is not a legitimate steady
state, it is simply wrong, and it feeds directly into the next
upgrade's Gate 1 changelog-review evidence.

### Live vs. imported records (fixes `CA-LRB-VER-1`)

No existing field distinguishes "this learning record is my own
checkout's live, currently-maintained adoption state" from "this
learning record is an imported reference copy of another adopter's (or
a historical) submission." Add one, explicitly, rather than guessing at
file naming or location:

`adoption.checkout_role`, an optional enum on the learning schema:
`live` or `imported`. The new version-drift check fires **only** on
records where `checkout_role: live`. A record with `checkout_role:
imported`, or with the field absent entirely, is **skipped** by this
check — absence is treated as "unknown, do not assert a live-version
claim," not as an implicit `live`. This is a deliberate fail-open
choice for the *new* check specifically: a false negative (a live
record silently unmarked) is a missed detection, but a false positive
(an imported historical record forced to match the current tree) would
either corrupt evidence or block the repository's own existing corpus
outright — verified above with `foundry.yaml` and `harbor-workshop.
yaml`, both of which currently disagree with `agf/VERSION` for two
different, both-legitimate reasons.

At implementation time, this project's own `foundry.yaml` — this
checkout's actual live record — gains `adoption.checkout_role: live`
and, in the same change, its `framework_version` field is corrected
from the current `"2.0.0-provisional"` to match `agf/VERSION` at
merge time; otherwise the new check fails immediately on landing.
`harbor-workshop.yaml` (an imported adopter evidence copy, not this
checkout's own state) gets no `checkout_role` field and is skipped,
unchanged, by design — see the regression fixture in "Test strategy."

### Mid-upgrade transient window: full lifecycle, decided (fixes `CA-LRB-VER-2`)

Confirmed by reading the current `agf/skills/upgrade/SKILL.md`: Step 3
applies archive file changes (including `agf/VERSION`) before Step 4
updates `adoption.framework_version` — the mismatch window exists on
every upgrade, unconditionally. Full lifecycle:

- **Marker shape:** `adoption.upgrade_in_progress: true` plus
  `adoption.upgrade_started_at: "<RFC3339 timestamp>"`, both on the
  same live record.
- **Owner:** the upgrade skill sets both fields at the *start* of Step
  3, before any archive file is copied. It clears both fields (removes
  them) at the *end* of Step 4, immediately after
  `adoption.framework_version` is updated to the new value.
- **Suppression scope:** the version-drift check is suppressed only for
  the exact record carrying `upgrade_in_progress: true`, and only while
  `upgrade_started_at` is within a bounded window (see below) — never a
  blanket or cross-record suppression.
- **Interrupted-upgrade recovery / crash bound:** if `upgrade_started_at`
  is more than **24 hours** in the past (a generous upper bound for a
  manual, human-gated upgrade session — Steps 1 and 2 both have
  explicit user-confirmation gates that can pause indefinitely, but 24
  hours from Step 3's actual start is well past any single working
  session), the check **stops honoring the suppression** and fires
  normally *plus* emits a second, distinct finding: "stale
  `upgrade_in_progress` marker — upgrade did not complete Step 4
  within the expected window; verify manually and clear the marker."
  This is strictly more informative than silent indefinite
  suppression, which is what an unbounded marker would produce for an
  upgrade that crashed or was abandoned between Step 3 and Step 4.
- **Default failure behavior:** when neither marker is present (the
  normal, non-upgrading case), the check runs unconditionally.

## What this plan does NOT do

- Does not change the upgrade skill's step ordering.
- Does not touch the already-shipped `agov-version-drift-warning`
  (binary-vs-tree) check from issue #114 — different check, different
  comparison, different severity model.
- Does not address #96, #111, or #113 — tracked as separate plans.

## Files expected to change

This plan's own diff (Gate 1 candidate): only this plan file.

On the implementation branch, after Gate 1 approval:

- `agf/schemas/learning.schema.json` — add optional
  `adoption.checkout_role` (`live`/`imported`) and optional
  `adoption.upgrade_in_progress` / `adoption.upgrade_started_at`.
- `tooling/rust/crates/agov-validators/src/learnings.rs` — new
  comparison against `agf/VERSION`, scoped to `checkout_role: live`
  records, honoring the bounded in-progress suppression.
- `agf/skills/upgrade/SKILL.md` Step 3 (set marker) and Step 4 (clear
  marker, update version) — document the full lifecycle decided above.
- `agf/learnings/foundry.yaml` — add `checkout_role: live` and correct
  `framework_version` to match `agf/VERSION` at merge time (required so
  the new check does not fail immediately on this project's own live
  record).
- `agf/docs/site/docs/adopt/` — adopter-facing explanation of
  `checkout_role` and the upgrade-in-progress marker (exact page
  confirmed at implementation time against current
  `agf/docs/site/docs/adopt/` contents), satisfying the adopter-docs-
  parity requirement below.
- `agf/templates/learning-record.yaml` — document the new optional
  fields in the shipped template.
- Focused unit tests (see "Test strategy").

## Test strategy

- Positive: `checkout_role: live` record with `adoption.framework_
  version` matching `agf/VERSION` — passes.
- Negative: `checkout_role: live` record with a mismatch — fails with a
  blocking finding naming both values.
- Regression (fixes `CA-LRB-VER-1`): a record with `checkout_role:
  imported` (or the field absent), whose version mismatches
  `agf/VERSION` — is skipped, not flagged. Regression fixture uses this
  repository's own current `harbor-workshop.yaml` content (framework_
  version `2.5.0` vs `agf/VERSION` `2.6.1`) as the literal test input,
  proving the existing corpus is unaffected.
- Positive: a `live` record carrying `upgrade_in_progress: true` with a
  fresh `upgrade_started_at` (within 24h) — mismatch is suppressed, no
  finding.
- Negative (fixes `CA-LRB-VER-2`): a `live` record carrying
  `upgrade_in_progress: true` with `upgrade_started_at` older than 24h
  — the version-mismatch finding fires anyway, plus the distinct
  "stale in-progress marker" finding.
- `agov plan validate` on this plan file: must pass.
- `agov validate` (full umbrella): no new findings beyond pre-existing,
  unrelated warnings, once `foundry.yaml`'s own version is corrected as
  part of this implementation.
- `cargo test -p agov-validators`: must pass, including the new tests.
- **Gate 2 acceptance evidence, per `gate-self-test-rule.md`:** the Gate
  2 MR description MUST include a reproducible negative CLI
  demonstration — exact command, a deliberately mismatched `live`
  record, expected and actual blocking output, and a link to the CI
  pipeline job log where it ran.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `tooling/**` and `agf/schemas/*.json` — trigger
paths.

**Classification: BREAKING, not PREVENTIVE (round 1 correction, fixes
`CA-LRB-VER-3`).** A `live` learning record whose `framework_version`
previously legitimately differed from `agf/VERSION` (there was no rule
against it) now fails validation. That is a restriction of previously-
permitted behavior, which `downstream-impact-rule.md` classifies as
BREAKING regardless of how well-justified the restriction is.

```
Impact:
  Foundry:          BREAKING for this project's own live record
                     (foundry.yaml) unless corrected in the same
                     change — see "Files expected to change." ADDITIVE
                     for the checkout_role/upgrade_in_progress schema
                     fields themselves (optional, no existing record
                     required to set them).
  Direct adopters:  BREAKING. An adopter whose live record's version
                     has drifted (previously accepted, now rejected)
                     must mark checkout_role: live and correct
                     framework_version, or the check fails their
                     validation pipeline. Adopters who do not set
                     checkout_role at all are unaffected (skipped,
                     fixes CA-LRB-VER-1) — this is the deliberate
                     fail-open default for the new field, not implicit
                     opt-out of a mandatory check.
  Downstream:       No adopter-produced artifact shape changes for
                     adopters not opting into checkout_role: live; no
                     agent behavior toward an adopter's users changes.
  Migration:        Required for any adopter who both (a) opts a
                     record into checkout_role: live and (b) has an
                     already-drifted framework_version. Migration note:
                     correct adoption.framework_version to match
                     agf/VERSION before or in the same commit as adding
                     checkout_role: live; the check is opt-in via the
                     new field, so no adopter is forced onto the new
                     behavior without also opting in.
  Evidence:          GitLab issue #112 (harbor-workshop, 2026-08-10 to
                     2026-08-21); this project's own foundry.yaml and
                     harbor-workshop.yaml verified as live evidence for
                     the live-vs-imported distinction (CA-LRB-VER-1).
```

**Adopter docs parity:** `updated` — `agf/docs/site/docs/adopt/`
(exact page named in "Files expected to change") explains
`checkout_role`, that it is opt-in, and the upgrade-in-progress marker
lifecycle; `agf/templates/learning-record.yaml` documents the new
optional fields in the shipped template adopters copy from.

## Rollback

The schema fields (`checkout_role`, `upgrade_in_progress`,
`upgrade_started_at`) are optional additions — no schema-migration
implication for records that never set them. **Lifecycle-aware rollback
(fixes `CA-LRB-VER-3`):** once an adopter has opted a record into
`checkout_role: live` and relies on the check passing, reverting the
validator removes the check but does not require reverting the schema
fields or the record itself — a `live`-marked record with an accurate
version stays valid under the old (no-check) behavior with or without
the field present, since the field's only effect was gating the new
check. No adopter action is required at rollback time; the fields
become inert, not invalid.

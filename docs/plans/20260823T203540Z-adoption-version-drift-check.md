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

## Round 2 correction (fixes contribution-auditor's exact-head review, `rerj`)

`20260823T213054Z-contribution-auditor-rerj.md` reviewed the round-1
corrected head (`bc20a35c`) and recommended BLOCK, 1 critical + 1
material finding. Verified and fixed:

- **`CA-LRB-VER-R1-1`** (critical) — the round-1 fix made `checkout_role`
  purely opt-in with no mechanism causing any real adopter's own live
  record to ever gain the field, so the check the issue exists to
  provide activates for nobody by default — the exact silent-failure
  shape issue #112 reported in the first place, just moved one layer
  down. Fixed: see the new "Deterministic live-record selection and
  migration" subsection under Design below — a new, always-on advisory
  finding fires whenever zero `checkout_role: live` records exist in
  `agf/learnings/`, exactly one live record is required (2+ is a
  blocking ambiguity finding), and both the commissioning walkthrough
  (new adopters) and the upgrade skill (existing adopters) are given
  concrete, decided touchpoints that set the field.
- **`CA-LRB-VER-R1-2`** (material) — the adopter documentation path was
  still deferred ("exact page confirmed at implementation time")
  despite the round-1 "Adopter docs parity: updated" claim asserting it
  was named. Fixed: the exact page and section are named now —
  `agf/docs/site/docs/adopt/install-and-layouts.md`, "Step 7:
  Initialise your evidence" (verified: this is the existing section
  that already tells a new adopter to confirm
  `agf/learnings/<your-codename>.yaml` exists — the natural, already-
  read anchor point, not a new page).

## Round 3 correction (fixes contribution-auditor's exact-head review, `nm4h`)

`20260823T215731Z-contribution-auditor-nm4h.md` reviewed the round-2
corrected head (`aa4bc3d1`) and recommended BLOCK, 1 critical finding:

- **`CA-LRB-VER-R2-1`** — round 2's instruction to the upgrade skill
  ("if the adopter's own record does not yet carry `checkout_role:
  live`, add it now") named no mechanism for identifying *which* file is
  "the adopter's own" when zero live records exist and more than one
  candidate file is present — exactly this project's own current state
  (`foundry.yaml` and `harbor-workshop.yaml`, both unmarked). The
  zero-live advisory finding makes the gap visible but cannot itself
  choose a record; nothing prevented an implementation from guessing (or
  marking imported evidence) live. Fixed: see the revised "Existing
  adopters" bullet under "Deterministic live-record selection and
  migration" below — a concrete two-branch selection procedure (auto-
  select when exactly one candidate exists; mandatory explicit operator
  selection, never a heuristic guess, when more than one exists).

## Round 4 correction (fixes contribution-auditor's exact-head review, `3rb6`)

`20260823T222725Z-contribution-auditor-3rb6.md` reviewed the round-3
corrected head (`f71fbbdc`) and recommended CLEAR for MR !1855, BLOCK
for this plan (MR !1856) on 1 critical finding:

- **`CA-LRB-VER-R3-1`** — round 3 placed the live-record selection
  procedure in Step 4, but this plan's own marker lifecycle (decided in
  round 1) requires the marker to be set on the live record at the
  *start of Step 3*, before any archive file is copied. On a first
  upgrade of an existing, unmarked adoption, no live record exists yet
  at Step 3 — selection hadn't run. Waiting until Step 4 is too late:
  Step 3 has already mutated `agf/VERSION` by then, and the marker that
  was supposed to suppress the resulting mismatch was never set,
  because there was no record to set it on. Fixed: see the retimed
  "Existing adopters" bullet and the "Owner" bullet below — selection
  now runs as the first sub-step of Step 3 itself, immediately before
  the marker is set, not at Step 4.

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

### Deterministic live-record selection and migration (fixes `CA-LRB-VER-R1-1`)

Opt-in alone does not activate detection for anyone. Two complementary
mechanisms close that gap without misclassifying imported evidence:

- **A second, always-on advisory finding, independent of `checkout_role`
  entirely.** Whenever `agf/learnings/` contains **zero** records with
  `checkout_role: live`, `agov validate learnings` emits an advisory
  (non-blocking) finding: "no live-marked adoption record found;
  version-drift detection is inactive for this checkout — add
  `checkout_role: live` to your own learning record (see
  `install-and-layouts.md`, Step 7)." This finding cannot be silently
  skipped the way the blocking check itself can, because it does not
  depend on the field already being present — it is the field's own
  absence that triggers it. It is advisory, not blocking, because an
  adopter with only imported evidence records (this project's own
  `harbor-workshop.yaml`, before `foundry.yaml` gains the field) is a
  legitimate state, not an error.
- **Exactly one live record is required.** Two or more records carrying
  `checkout_role: live` in the same `agf/learnings/` directory is a
  **blocking** ambiguity finding — the check cannot know which record
  is authoritative for this checkout's own version. Zero live records
  never blocks (advisory only, above); one live record runs the normal
  version-comparison check from the previous subsection; two or more
  blocks until the adopter resolves which record is actually live.
- **Who sets the field, and when (both decided, not left to adopter
  discovery):**
  - **New adopters:** the commissioning walkthrough's existing Step 7
    ("Initialise your evidence") is the first place a fresh
    `agf/learnings/<your-codename>.yaml` is created — this plan adds an
    instruction there to set `checkout_role: live` on that record at
    creation time, so new adoptions are covered without any separate
    migration step.
  - **Existing adopters — deterministic selection contract (round 3
    correction, fixes `CA-LRB-VER-R2-1`; retimed round 4, fixes
    `CA-LRB-VER-R3-1`):** round 2's instruction ("if the adopter's own
    record does not yet carry `checkout_role: live`, add it now") never
    specified how the skill identifies *which* file is "the adopter's
    own" when `agf/learnings/` contains zero live records but more than
    one candidate `.yaml` file — this project's own `agf/learnings/` is
    exactly that case today (`foundry.yaml` and `harbor-workshop.yaml`,
    both unmarked). Round 3 fixed the selection mechanism but placed it
    in Step 4 — too late: the marker lifecycle below requires the
    *live* record to already be marked at the very **start of Step 3**,
    before any archive file is copied, and on a first upgrade of an
    unmarked adoption no live record exists yet at that point. Fixed
    (round 4): the concrete, two-branch selection procedure below now
    runs as the **first action of Step 3 itself, immediately before the
    `upgrade_in_progress`/`upgrade_started_at` marker is set** — after
    the upgrade skill's earlier user-confirmation gates (Steps 1–2), but
    strictly before any marker or archive-file write:
    - **Exactly one `.yaml` file exists in `agf/learnings/`:** no
      ambiguity — the skill marks that file `checkout_role: live`
      automatically, no operator prompt needed, then proceeds to set the
      marker on it.
    - **Two or more `.yaml` files exist:** the skill MUST enumerate
      every candidate file and require the human operator to explicitly
      name exactly one as "this checkout's own live record" before Step
      3 can proceed any further — including before the marker is set.
      The skill MUST NOT guess via any heuristic (alphabetical order,
      most-recently-modified, filename matching a git identity, or any
      other inference) — only an explicit operator selection marks a
      file live. This is a mandatory, blocking gate inside the skill
      itself (human-in-the-loop, the same discipline as the companion
      plan's GATE 2.5), not a validator-level check — the validator's
      zero-live advisory finding (above) only signals the gap exists; it
      cannot safely resolve it.
    - **Already exactly one live record exists:** selection is a no-op
      — proceed directly to setting the marker on that record, as
      before round 3.
    This gives every existing adopter a concrete, recurring touchpoint
    (their next upgrade) rather than requiring them to independently
    learn about the new field — combined with the always-on advisory
    finding above, an adopter who skips this instruction still sees a
    persistent signal on every `agov validate` run until they act.

### Mid-upgrade transient window: full lifecycle, decided (fixes `CA-LRB-VER-2`)

Confirmed by reading the current `agf/skills/upgrade/SKILL.md`: Step 3
applies archive file changes (including `agf/VERSION`) before Step 4
updates `adoption.framework_version` — the mismatch window exists on
every upgrade, unconditionally. Full lifecycle:

- **Marker shape:** `adoption.upgrade_in_progress: true` plus
  `adoption.upgrade_started_at: "<RFC3339 timestamp>"`, both on the
  same live record.
- **Owner:** the upgrade skill sets both fields at the *start* of Step
  3, before any archive file is copied — **immediately after** the
  live-record selection procedure above runs (round 4 correction, fixes
  `CA-LRB-VER-R3-1`: selection must resolve which record is live before
  the marker can be set on it, so selection is the first sub-step of
  Step 3 and marker-setting is the second, both before any archive file
  is touched). It clears both fields (removes them) at the *end* of
  Step 4, immediately after `adoption.framework_version` is updated to
  the new value.
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
  records, honoring the bounded in-progress suppression; plus the new
  always-on zero-live-records advisory finding and the two-or-more-live-
  records blocking ambiguity finding (round 2, fixes `CA-LRB-VER-R1-1`).
- `agf/skills/upgrade/SKILL.md` Step 3 (round 4: **first sub-step is now
  the live-record selection procedure** — auto-select if exactly one
  candidate, else mandatory operator selection, fixes
  `CA-LRB-VER-R3-1` — *then* set the marker, both before any archive
  file is copied) and Step 4 (clear marker, update version) — document
  the full lifecycle decided above.
- `agf/learnings/foundry.yaml` — add `checkout_role: live` and correct
  `framework_version` to match `agf/VERSION` at merge time (required so
  the new check does not fail immediately on this project's own live
  record).
- `agf/docs/site/docs/adopt/install-and-layouts.md`, "Step 7:
  Initialise your evidence" (round 2: exact page and section named, not
  deferred — fixes `CA-LRB-VER-R1-2`) — new instruction to set
  `checkout_role: live` on the freshly-created record at commissioning
  time, plus a short explanation of the field, the upgrade-in-progress
  marker, and the always-on advisory finding, satisfying the adopter-
  docs-parity requirement below.
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
- Advisory (fixes `CA-LRB-VER-R1-1`): a `agf/learnings/` directory with
  zero `checkout_role: live` records (including one containing only
  `checkout_role: imported` records, or none of the field at all) — the
  always-on advisory finding fires, no blocking finding, `agov validate`
  exit code unaffected.
- Blocking (fixes `CA-LRB-VER-R1-1`): a `agf/learnings/` directory with
  two records both carrying `checkout_role: live` — the ambiguity
  finding fires as blocking, naming both record files.
- Positive (fixes `CA-LRB-VER-R1-1`): exactly one `checkout_role: live`
  record present — the advisory zero-records finding does not fire, and
  the ambiguity finding does not fire; only the normal version-
  comparison check from the earlier positive/negative cases applies.
- Skill acceptance evidence (fixes `CA-LRB-VER-R2-1`, retimed fixes
  `CA-LRB-VER-R3-1`): a checkout with zero live records and exactly one
  candidate `.yaml` file in `agf/learnings/` — the upgrade skill's Step
  3 marks that file live automatically (no operator prompt) as its
  first sub-step, *before* setting the `upgrade_in_progress` marker or
  copying any archive file. A checkout with zero live records and two or
  more candidate files (matching this project's own current
  `foundry.yaml`/`harbor-workshop.yaml` state) — Step 3 halts at its
  first sub-step and presents the candidate list, requiring an explicit
  operator selection before the marker is set or any archive file is
  copied; assert no heuristic (alphabetical, most-recently-modified, or
  any other inference) ever auto-selects a file in this branch, and
  assert `agf/VERSION` is unmodified while the selection gate is
  outstanding (proving the fix actually closes the round-3 ordering gap,
  not just relocates the same race).
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
                     checkout_role at all are not blocked (skipped by
                     the blocking check, fixes CA-LRB-VER-1) but do see
                     a new always-on advisory finding until they add the
                     field (round 2, fixes CA-LRB-VER-R1-1) — the
                     blocking behavior is opt-in, but visibility of the
                     gap is not.
  Downstream:       No adopter-produced artifact shape changes for
                     adopters not opting into checkout_role: live; no
                     agent behavior toward an adopter's users changes.
  Migration:        Required for any adopter who both (a) opts a
                     record into checkout_role: live and (b) has an
                     already-drifted framework_version. Migration note:
                     correct adoption.framework_version to match
                     agf/VERSION before or in the same commit as adding
                     checkout_role: live. The blocking check is opt-in
                     via the new field, so no adopter is forced onto the
                     blocking behavior without also opting in — but the
                     upgrade skill's Step 3 (round 4: retimed from Step
                     4) prompts every existing adopter to opt in at
                     their next upgrade, before that upgrade mutates
                     agf/VERSION, and the always-on advisory finding
                     (round 2) makes
                     an unmarked live record visible on every
                     `agov validate` run in the meantime.
  Evidence:          GitLab issue #112 (harbor-workshop, 2026-08-10 to
                     2026-08-21); this project's own foundry.yaml and
                     harbor-workshop.yaml verified as live evidence for
                     the live-vs-imported distinction (CA-LRB-VER-1).
```

**Adopter docs parity:** `updated` — `agf/docs/site/docs/adopt/
install-and-layouts.md`, "Step 7: Initialise your evidence" (round 2:
exact page and section, fixes `CA-LRB-VER-R1-2`) explains
`checkout_role`, that the blocking check is opt-in but the zero-live-
records finding is not, the exactly-one-live-record requirement, and
the upgrade-in-progress marker lifecycle; `agf/templates/
learning-record.yaml` documents the new optional fields in the shipped
template adopters copy from.

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

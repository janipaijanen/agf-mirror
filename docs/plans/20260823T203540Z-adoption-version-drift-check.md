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

## Round 5 correction (fixes contribution-auditor's exact-head review, `1j32`)

`20260823T232454Z-contribution-auditor-1j32.md` reviewed the round-4
corrected head (`0df76ebb`) and confirmed `CA-LRB-VER-R3-1` resolved,
but recommended BLOCK on 1 new critical finding:

- **`CA-LRB-VER-R4-1`** — round 4 correctly retimed the *marker write*
  to before Step 3's archive mutation, but left the *record-selection
  decision itself* at Step 3 — after Gate 1. Verified directly against
  `agf/skills/upgrade/SKILL.md`'s current text: its own Prerequisites
  state "the current framework version is recorded in the adoption
  section of `learnings/<codename>.yaml`," and Step 1 uses that "current
  version" to compute the changelog range presented at GATE 1 — both
  happening before Step 3 runs. In the exact multi-candidate state this
  plan exists to handle (this project's own current
  `foundry.yaml`/`harbor-workshop.yaml` state), Gate 1 would present a
  changelog range computed from an unverified guess at "current version"
  before the operator ever selects which record is authoritative — a
  human could approve a wrong changelog range at Gate 1, and the later
  Step 3 selection cannot retroactively correct what was already
  approved. Fixed: see the new "Step 0" subsection below — identity
  selection (which record is authoritative) and version-agreement
  verification against `agf/VERSION` both move to before Step 1 reads
  or presents anything; only the actual field *write*
  (`checkout_role: live` + the in-progress marker) stays at Step 3,
  after both user gates, consistent with round 1's "minimize mutation
  before gates pass" discipline.

## Round 6 correction (fixes contribution-auditor's exact-head review, `krhb`)

`20260824T005805Z-contribution-auditor-krhb.md` reviewed the round-5
corrected head (`803e9cfc`) and confirmed `CA-LRB-VER-R4-1` resolved,
but recommended BLOCK on 2 new critical findings:

- **`CA-LRB-VER-R5-1`** — Step 0's "exactly one `.yaml` file selects
  automatically" branch auto-selects *any* single candidate, including
  one explicitly marked `checkout_role: imported` or carrying no
  `checkout_role` at all. Per this plan's own round-1 design, absence of
  the field means "unknown, do not assert a live-version claim," not
  "definitely this checkout's own" — and a checkout with only imported
  evidence is explicitly documented as a legitimate zero-live state. The
  singleton-count heuristic contradicted that boundary: it would
  silently promote a genuinely-imported record to `checkout_role: live`
  at Step 3 for no reason other than being the only file present. Fixed:
  see the revised Step 0 procedure below — auto-selection now requires
  an *already-live* record, never a bare file count; zero live records
  always requires explicit operator selection, regardless of how many
  candidates exist.
- **`CA-LRB-VER-R5-2`** — Step 0's mismatch handling offered two
  branches ("the operator either corrects the record's version or
  explicitly dispositions the mismatch") but only defined the first;
  "dispositions" named no authority, no recorded shape, and no rule for
  which version Step 1 then uses as its baseline — directly
  contradicting Step 1's own clause requiring "the Step-0-*verified*
  candidate's `framework_version`," since a dispositioned-but-unequal
  value is not verified equal. Fixed, per the reviewer's own preferred
  resolution and this project's simplicity-rule (no new governance
  surface for an escape hatch this plan does not need): the disposition
  branch is removed entirely. A mismatch is a hard block — the operator
  corrects the record's `adoption.framework_version` to equal
  `agf/VERSION` before Step 1 can proceed. One executable outcome, no
  exception path.

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
  - **Existing adopters — identification split from mutation (round 3
    correction, fixes `CA-LRB-VER-R2-1`; retimed round 4, fixes
    `CA-LRB-VER-R3-1`; split round 5, fixes `CA-LRB-VER-R4-1`):** round
    2's instruction ("if the adopter's own record does not yet carry
    `checkout_role: live`, add it now") never specified how the skill
    identifies *which* file is "the adopter's own" when
    `agf/learnings/` contains zero live records but more than one
    candidate `.yaml` file — this project's own `agf/learnings/` is
    exactly that case today (`foundry.yaml` and `harbor-workshop.yaml`,
    both unmarked). Round 3 fixed the selection mechanism but placed it
    in Step 4; round 4 retimed the *marker write* to the start of Step
    3. Both left the *decision itself* after GATE 1 — but the skill's
    own Prerequisites already require "the current framework version"
    from `learnings/<codename>.yaml` to compute the changelog range
    Step 1 presents at GATE 1, so an unresolved multi-candidate state
    left Gate 1 reviewing a range built on an unverified guess. Fixed
    (round 5): the decision and the write are two different moments,
    not one:
    - **New "Step 0" — identify and verify, before Step 1 reads
      anything (round 6 correction, fixes `CA-LRB-VER-R5-1`,
      `CA-LRB-VER-R5-2`):**
      - **Auto-selection requires an already-live record — never a bare
        file count (fixes `CA-LRB-VER-R5-1`).** If exactly one record in
        `agf/learnings/` already carries `checkout_role: live`, select
        it automatically, no operator prompt. This is the *only*
        auto-select condition. A checkout whose zero-live state was
        never resolved — including a checkout with exactly **one**
        candidate file, whether that file carries `checkout_role:
        imported` explicitly or carries no `checkout_role` at all —
        does **not** auto-select. File count proves nothing about
        identity: this plan's own round-1 design already states that an
        absent `checkout_role` means "unknown," not "definitely this
        checkout's own," and that a checkout with only imported evidence
        is a legitimate zero-live state.
      - **Zero live records — always explicit operator selection,
        regardless of candidate count.** Whether one candidate or many,
        the skill MUST enumerate every candidate file (with its
        `checkout_role`, if any) and require the human operator to
        explicitly name exactly one as "this checkout's own live
        record" before proceeding — including the one-candidate case.
        The skill MUST NOT guess via any heuristic (file count,
        alphabetical order, most-recently-modified, filename matching a
        git identity, or any other inference). If the operator selects
        a candidate already marked `checkout_role: imported`, that is a
        legitimate, explicit human override — Step 3 later overwrites it
        to `live`, exactly as if it had been unmarked; the point of this
        gate is that the change is explicit, not that `imported` records
        can never become `live`.
      - **Version verification, single executable outcome (fixes
        `CA-LRB-VER-R5-2`).** Once a candidate is identified (by either
        branch above), read its `adoption.framework_version` and compare
        it against the current tree's `agf/VERSION`. On a mismatch, Step
        1 is blocked from constructing or presenting any changelog range
        — full stop. The operator corrects the record's
        `adoption.framework_version` to equal `agf/VERSION` before Step
        1 can proceed. There is no separate "disposition" path, no
        recorded exception, and no authority to accept an unequal value
        as a baseline — one executable outcome, matching exactly what
        Step 1's "verified candidate" clause below already assumed. This
        is the same comparison the validator check itself performs, run
        here as a skill-level gate one step earlier than the validator
        would otherwise catch it. **Step 0 identifies and verifies; it
        does not write anything to the record.**
    - **Step 1 / GATE 1:** proceeds using the Step-0-verified candidate's
      `framework_version` as the changelog range's "current version" —
      no longer an unverified guess.
    - **Step 3 (unchanged in position from round 4):** after both user
      gates, the skill writes `checkout_role: live` (if not already set)
      and the `upgrade_in_progress`/`upgrade_started_at` marker onto the
      **already-selected, already-verified** record from Step 0 —
      strictly before any archive-file write. Step 3 performs no new
      selection decision; it only persists the decision Step 0 already
      made and the user already implicitly relied on at Gate 1.
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
  3, before any archive file is copied, onto the record already
  identified and version-verified by **Step 0** (round 5 correction,
  fixes `CA-LRB-VER-R4-1`: identification and version verification
  happen before Step 1/GATE 1 read or present anything; Step 3 only
  writes the already-made decision — it performs no new selection).
  It clears both fields (removes them) at the *end* of
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
- `agf/skills/upgrade/SKILL.md` — new **Step 0** (round 5, fixes
  `CA-LRB-VER-R4-1`; round 6, fixes `CA-LRB-VER-R5-1`/`CA-LRB-VER-R5-2`),
  inserted before the existing Step 1: identify the authoritative
  candidate record (auto-select **only** if a record already carries
  `checkout_role: live`; otherwise mandatory operator selection,
  regardless of candidate count) and verify its `framework_version`
  against `agf/VERSION` — a mismatch hard-blocks Step 1's changelog-range
  construction until corrected, no exception path — no field write
  happens here. Step 3 (round 4 timing unchanged, fixes
  `CA-LRB-VER-R3-1`): writes `checkout_role: live` (if needed) and the
  marker onto the record Step 0 already identified, before any archive
  file is copied — Step 3 performs no new selection. Step 4 (clear
  marker, update version) — document the full lifecycle decided above.
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
- Skill acceptance evidence (fixes `CA-LRB-VER-R2-1`, retimed
  `CA-LRB-VER-R3-1`, split `CA-LRB-VER-R4-1`): a checkout with exactly
  one record already carrying `checkout_role: live` — Step 0 selects it
  automatically (no operator prompt) *before* Step 1 constructs any
  changelog range. A checkout with zero live records and two or more
  candidate files (matching this project's own current
  `foundry.yaml`/`harbor-workshop.yaml` state) — Step 0 halts and
  presents the candidate list, requiring an explicit operator selection
  *before Step 1 runs at all* — assert no heuristic (alphabetical,
  most-recently-modified, or any other inference) ever auto-selects a
  file in this branch, and assert Step 1 does not construct or present a
  changelog range while Step 0's selection is outstanding (proving Gate
  1 review is never based on an unverified guess — the specific gap
  `CA-LRB-VER-R4-1` found).
- Skill acceptance evidence (fixes `CA-LRB-VER-R5-1`): a checkout with
  zero live records and **exactly one** candidate `.yaml` file, that
  file carrying `checkout_role: imported` — assert Step 0 does **not**
  auto-select it; it halts and requires explicit operator selection
  (either confirming this file despite its `imported` marking, or
  creating/selecting a different live record), identical to the
  multi-candidate case. Repeat with the sole candidate carrying no
  `checkout_role` field at all — same assertion: no auto-select on file
  count alone, in either sub-case.
- Skill acceptance evidence (fixes `CA-LRB-VER-R5-2`): Step 0 selects a
  candidate whose `framework_version` disagrees with the current tree's
  `agf/VERSION` — Step 1 is blocked from constructing a changelog range
  until the operator corrects the record's version to match exactly;
  assert there is no path by which Step 1 proceeds with an unequal
  value (no "disposition" branch exists to test against, by design).
- Skill acceptance evidence (fixes `CA-LRB-VER-R3-1`, unchanged
  positioning from round 4): once Step 0 has identified and verified a
  candidate, Step 3 writes `checkout_role: live` (if needed) and the
  `upgrade_in_progress` marker onto that same already-selected record,
  strictly before any archive file is copied; assert `agf/VERSION` is
  unmodified while Step 3's write is outstanding, and assert Step 3
  performs no new selection decision (it writes exactly the record Step
  0 identified, never re-derives it).
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
                     upgrade skill's new Step 0 (round 5) identifies and
                     version-verifies the candidate record before Gate 1
                     even runs, Step 3 (round 4 timing unchanged) then
                     writes the opt-in field before that upgrade mutates
                     agf/VERSION, and the always-on advisory finding
                     (round 2) makes an unmarked live record visible on
                     every `agov validate` run in the meantime.
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

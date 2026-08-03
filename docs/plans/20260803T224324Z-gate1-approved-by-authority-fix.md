---
id: gate1-approved-by-authority-fix
status: pending-gate1
topic: gate1-approved-by-authority-fix
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      This plan changes `agf/schemas/gate1-plan.schema.json` and adds a
      new cross-record MANIFEST check in `agov-coord`'s plan validator —
      an access-control/authorization-adjacent surface under
      `always/high-risk-operations-rule.md`, even though (per
      contribution-auditor's `5k14` review, addressed in this revision)
      the check is scoped as structural approver-field hygiene, not
      authority enforcement. The central design question (fail closed
      vs. fail open when `agf/agents/MANIFEST.yaml` is absent, and which
      migration path to ship) is exactly the class of decision that rule
      requires a plan and explicit confirmation for before
      implementation.
  - role: contribution-auditor
    status: consulted
    reason: >
      Round 1 review (`20260803T225012Z-contribution-auditor-5k14.md`)
      at exact head `dff82cb0`: BLOCK, seven blocking findings
      (B-G1-AUTH-1, B-G1-EVID-2, B-G1-DATA-3, B-G1-SPEC-4, B-G1-MIG-5,
      B-G1-SCOPE-6, B-G1-TEST-7) plus two advisories
      (A-G1-ROLE-1, A-G1-AUTHZ-2), all addressed in this revision. Renewed
      review requested at the corrected exact head.
---

# Plan: `gate1_approved_by` structural approver-field hygiene fix

**Renamed from "authority-enforcement fix" per contribution-auditor's
`5k14` finding B-G1-AUTH-1.** This plan does not authenticate an actor,
verify a cited record exists or says what it's cited as saying, or
prove the human governor made a decision. It closes exactly one thing:
today, `gate1_approved_by` accepts empty or arbitrary text on an
`approved`/`active`/`complete` plan with zero schema-shape check. After
this plan, it must at minimum name — not prove — a currently active,
governor-class role, in one of two precisely specified textual shapes.
That boundary is stated once here and then held consistently through
every section below, per `5k14`'s explicit instruction.

## Decision requested

Authorize implementation of a schema-plus-validator change that
requires `gate1_approved_by` to be non-empty and **start with** either
a governor-class role name or a coordination-record-ID citation of one,
for plans whose `status` has reached `approved`, `active`, or
`complete`. Planning only. No implementation, schema change, or version
bump is authorized by this plan itself. The migration path (§Migration,
Path A vs. Path B) is an explicit open decision this plan asks Gate 1
review to make, not a recommendation this plan pre-selects.

**Authorization for this planning work.** A session-specific human
instruction authorized drafting this plan, superseding an earlier
declination (`docs/coordination/20260803T211911Z-planner-architect-7jhj.md`).
**Correction (contribution-auditor `5k14`, A-G1-AUTHZ-2):** that
instruction exists only as session context in this conversation, not as
a durable governor coordination record — it is labelled here as
chat-relayed context, not cited as evidence that this plan itself is
approved. The active `agf/agents/MANIFEST.yaml` already marks
`planner-architect` `status: active`, `played_by: claude`, which is the
durable evidence for role activation; task-specific authorization for
this exact drafting task is a separate, non-durable fact this plan does
not need to prove for Gate 1 review to proceed.

## Verified current defect

Direct inspection of `agf/schemas/gate1-plan.schema.json` (current
`main`):

```json
"gate1_approved_by": {
  "type": "string"
}
```

No `minLength`, no `pattern`, no cross-record check against
`agf/agents/MANIFEST.yaml`. A plan with `status: approved` and
`gate1_approved_by: ""` (empty) validates cleanly, and so does
`gate1_approved_by: "coder-tester"` (an AI implementation role, not the
human governor role) — both independently reproduced against the real
schema and recorded in `docs/coordination/20260803T204257Z-planner-architect-sjqm.md`
(Track 3 finding 1, merged to `main`) and Codex's own adversarial
synthesis, `docs/coordination/20260803T210104Z-contribution-auditor-4g1e.md`
(also merged to `main`).

**Evidence provenance (contribution-auditor `5k14`, B-G1-EVID-2).**
`sjqm` and `4g1e`, both cited above, are on `main` and are sufficient by
themselves to establish the reproduced structural gap and the
MANIFEST-absent trap. This plan's earlier draft additionally cited
learning entries `gt3x` and `hv3k` as if they were already accepted
corpus; at the time of `5k14`'s review they existed only in blocked
MRs. As of this revision, contribution-auditor has independently
CLEARed both at their corrected heads (`20260803T225227Z-contribution-auditor-k3n4.md`
for `gt3x`, `20260803T225850Z-contribution-auditor-hanm.md` for the
learning MR containing `hv3k`), but **the governor has not yet decided
whether to accept either learning, and neither is merged to `main`**.
This plan therefore treats `gt3x`/`hv3k` as **provisional** supporting
context only — cited for the reasoning trail, never as the sole basis
for a claim that could instead be sourced from `sjqm`/`4g1e`.

The existing precedent for a MANIFEST cross-check is
`tooling/rust/crates/agov-coord/src/plan_validator.rs`'s
`advisory_consultation` role check (`role_statuses`,
`validate_semantics_with_manifest`). That check demonstrates
exact-match, case-sensitive role lookup (confirmed:
`infosec-sentinels` plural is rejected as `unknown advisory role`). It
also demonstrates the exact trap this plan must not repeat: its own
test `missing_manifest_falls_back_to_required_roles_only` shows that
when `agf/agents/MANIFEST.yaml` cannot be read, the check silently
passes for the two hardcoded required roles rather than blocking. A
naive reuse of this pattern for `gate1_approved_by` would be fully
bypassed in any workspace with no `MANIFEST.yaml` present — a
condition this session reproduced directly against the real schema and
validator; how common that condition is across adopters was not
measured and is not claimed here.

## Design — structural approver-field hygiene

### 1. Scope: which statuses are checked

`gate1_approved_by` is checked whenever `status` is one of `approved`,
`active`, `complete` — every status that asserts a Gate 1 decision has
already happened. `pending-gate1` is exempt (no decision asserted yet).

### 2. The check itself — one precise parser

**Revised twice, and each revision was forced by measuring real data,
not by writing a specification in the abstract.** The first draft
required an exact, bare match of a governor-class role name; measuring
it against this repository's real 103 in-scope plans showed it would
wrongly flag 46 of them (103 minus 57 bare-exact matches) as
nonconforming, because the real convention here is a short citation —
role name plus supporting evidence — not a bare token. The second draft
allowed the role name to appear **anywhere** in the field (prefix, or
embedded later as a coordination-ID citation). Contribution-auditor's
`5k14` (B-G1-SPEC-4) found this second draft self-contradictory — its
own §5 validator pseudocode already described exact-match-on-the-whole-value
behavior different from what §2's prose allowed — and, independently,
finding B-G1-DATA-3 traced to the exact same root cause: measuring the
"anywhere" rule against current `main` found
`docs/plans/v250-rc1-release-publication.md`, whose
`gate1_approved_by` value is:

> `human governor; approved in chat, confirming round-5 CLEAR review
> 20260803T163251Z-contribution-auditor-kbdk at exact head ...;
> Claude-scribed decision-record 20260803T173905Z-governor-reviewer-p1mm.md`

This value contains a real, valid `governor-reviewer`-authored
coordination-ID substring — but only after an unrelated free-text
description and an unrelated `contribution-auditor`-authored citation
come first. An "anywhere" rule accepts it; a rule that only checks the
start of the value correctly does not, because what the value *asserts
as its own approval citation* — its first token — names
`contribution-auditor`, and the real word "governor-reviewer" that
appears later is not what the field is citing as authority. This is
also exactly the shape a fabricated approval could exploit: pad any
free-text value with an unrelated real governor-reviewer ID copied from
elsewhere, and an "anywhere" rule accepts it regardless of what the
value's own first clause actually says.

**Final design — prefix-only, one rule:**

- `gate1_approved_by`, after trimming leading/trailing whitespace, must
  be non-empty.
- The **trimmed value must start with** one of:
  1. An exact-match (case-sensitive) governor-class role name, followed
     immediately by a non-alphanumeric character or end-of-string (so
     `governor-reviewer; decision ...` matches, `governor-reviewers`
     does not); or
  2. A coordination-record-ID citation of that role, matching
     `^<8-digit-date>T<6-digit-time>Z-<governor-class-role>-<4-char-suffix>`
     at the very start of the trimmed value, followed by a
     non-alphanumeric character or end-of-string (so
     `20260802T192253Z-governor-reviewer-gkeq` and
     `20260802T192253Z-governor-reviewer-gkeq.md` both match; a value
     that starts with anything else, even if a valid governor-reviewer
     ID appears later, does not).
  - There is no "embedded anywhere" shape. A role name or coordination
    ID occurring after the first token is not evidence this check
    reads at all.
  - This is a **shape check on what the field asserts as its own
    citation**, not a lookup that proves the cited record exists, was
    itself validly authored, or contains an approval matching this
    plan/head/scope. See §6.
- The governor-class role name used in either shape must additionally
  be an **exact-match** key of a role in `agf/agents/MANIFEST.yaml`
  whose `status` is `active` **and** whose role is in the
  governor-class allowlist below.
- **v1 governor-class allowlist: `governor-reviewer` only.**
  **Correction (contribution-auditor `5k14`, A-G1-ROLE-1):** this is
  not a project-local convenience choice — `governor-reviewer` is the
  canonical role name in AGF's own shipped adopter template,
  `agf/agents/MANIFEST.example.yaml` (`governor-reviewer: status:
  skipped`, present for every adopter to activate), so hardcoding it as
  the v1 allowlist member is anchored in a distributed adopter
  contract, not only this project's own current MANIFEST. This
  validator ships as part of the framework to every adopter, not only
  to this project. No new MANIFEST schema field (e.g., a
  `governor_class: true` marker) is proposed for a set of exactly one
  member — per `simplicity-rule.md`. **Reconsideration trigger:** the
  moment any adopter activates a second role intended to carry Gate 1
  approval authority (this project's own `MANIFEST.yaml` or any
  adopter's), that is the trigger to design extensible governor-class
  metadata — not before.
- A free-text synonym with no role-name-or-ID prefix at all (e.g.
  "human governor...", found in two real plans below) does not match
  and is treated as nonconforming under this design. Named, not an
  oversight — see Migration evidence.

### 3. Fail-closed on missing MANIFEST — scoped to this new check only

When `agf/agents/MANIFEST.yaml` cannot be read (absent, malformed, or
the `governor-reviewer` entry is missing or not `active`), the new
`gate1_approved_by` check **fails closed**: blocking error, not
silently skipped. This is a deliberate divergence from the existing
`advisory_consultation` check's current fail-open-on-missing-MANIFEST
behavior.

**This plan does not propose changing `advisory_consultation`'s
existing fallback to also fail closed.** Reasoning: that is a second
BREAKING change with its own separate blast radius (every plan file
that currently validates cleanly with no `MANIFEST.yaml` present would
start failing on a *different* check), and bundling it here would
violate this project's own branch-sizing test
(`branch-and-worktree-rule.md`). Recorded as an explicit, deliberately
deferred follow-up: **a second plan should evaluate fail-closed-by-default
for `advisory_consultation` on its own merits.**

### 4. Schema change

`agf/schemas/gate1-plan.schema.json`:

```json
"gate1_approved_by": {
  "type": "string",
  "minLength": 1
}
```

`minLength: 1` alone only proves non-emptiness (bypassable with
`"coder-tester"`, already reproduced) — necessary but not sufficient.
The prefix/role-class check cannot be expressed in JSON Schema alone
(it requires reading a second file, `MANIFEST.yaml`); it belongs in the
Rust semantic validator alongside the existing `advisory_consultation`
cross-check. **This schema change is unconditionally blocking the
moment it ships — see §Migration, Path A vs. Path B, for why this
constrains the available migration designs.**

### 5. Validator change — matches §2 exactly

`tooling/rust/crates/agov-coord/src/plan_validator.rs`:

- Add `governor_class_roles()` returning `&["governor-reviewer"]`,
  analogous to the existing `required_roles()`.
- Add a coordination-ID-prefix parser: given the trimmed
  `gate1_approved_by` value, check (a) does it start with any
  `governor_class_roles()` member followed by a non-alphanumeric
  character or end-of-string, or (b) does it start with
  `<8-digit>T<6-digit>Z-<role>-<4-char>` where `<role>` is a
  `governor_class_roles()` member, followed by a non-alphanumeric
  character or end-of-string. No other shape is accepted — explicitly
  not "contains anywhere."
- Extend `validate_semantics_with_manifest` (or add a sibling function
  called from `validate_file` alongside it) to, when `status` is
  `approved`/`active`/`complete`:
  - Trim `gate1_approved_by`. If empty: blocking error.
  - If `agf/agents/MANIFEST.yaml` is unreadable: blocking error (fail
    closed — new behavior, scoped to this check only, per §3).
  - If the prefix parser above does not match: blocking error,
    distinguishing "does not name a governor-class role" from "names a
    governor-class role that exists in MANIFEST but is not active"
    (mirrors the existing `unknown advisory role` vs. `advisory role
    is not active` distinction already in this file).
- MCP surface: `agov_plan_validate` wraps this validator directly (no
  separate reimplementation), so CLI and MCP behavior stay identical by
  construction — verified by a shared test fixture, not by two
  independent implementations (see §Test strategy).

### 6. What this fix does and does not close

This raises the cost of a forged or empty approval passing
schema/validator checks from zero to "the value's first token must
name an exact, currently-active governor-class role, as a bare name or
a shape-valid coordination-ID citation." It does **not** authenticate
that the human governor themselves wrote the value, verify the cited
coordination record exists or contains a matching approval, or prove
the human governor made the decision — `from:`/`gate1_approved_by:`
remain unauthenticated free text at the record-authorship layer (`gt3x`,
unresolved by this plan, cited here as provisional context per
B-G1-EVID-2). It does **not** freeze history — a later in-place edit of
an already-approved plan's `gate1_approved_by` field is a separate gap
(`gt3x` finding 3, content-hash-on-first-reference, also unresolved).

## Migration evidence (measured, reproducible)

Direct measurement against this repository's own `docs/plans/*.md` on
current `main`, re-run at this revision (not carried over from the
prior draft), using the exact §2 prefix-only rule:

```python
import re, glob, yaml

role = "governor-reviewer"
id_prefix_re = re.compile(
    rf'^\d{{8}}T\d{{6}}Z-{re.escape(role)}-[a-z0-9]{{4}}(?![A-Za-z0-9])'
)
bare_prefix_re = re.compile(rf'^{re.escape(role)}(?![A-Za-z0-9])')

total = empty = other = conforming = 0
for path in sorted(glob.glob("docs/plans/*.md")):
    content = open(path, encoding="utf-8", errors="replace").read()
    m = re.match(r"^---\n(.*?)\n---\n", content, re.DOTALL)
    if not m:
        continue
    fm = yaml.safe_load(m.group(1))
    if not isinstance(fm, dict) or fm.get("status") not in (
        "approved", "active", "complete"
    ):
        continue
    total += 1
    approver = (fm.get("gate1_approved_by") or "").strip()
    if not approver:
        empty += 1
    elif bare_prefix_re.match(approver) or id_prefix_re.match(approver):
        conforming += 1
    else:
        other += 1
```

| Category | Count | Disposition |
|---|---|---|
| Total in scope (status approved/active/complete) | 103 | — |
| Conforms under the prefix-only design | 80 | No action needed |
| Truly empty `gate1_approved_by` | 20 | Genuine gap this fix targets — needs a migration path (below) |
| Neither empty nor conforming ("other") | 3 | Needs human disposition, not auto-migration — see below |

**Corrected arithmetic (contribution-auditor `5k14`, B-G1-DATA-3):** an
earlier draft of this plan reported 81/20/2 conforming/empty/other and
separately claimed the original bare-exact-match design would have
"wrongly flagged 61 of 81" plans — both numbers were wrong. The 81/2
split came from the looser "contains anywhere" design this revision
replaces (§2); under the corrected prefix-only design the true split is
80/3, exactly matching contribution-auditor's independently-computed
count. The "61 of 81" figure was a plain arithmetic error: bare-exact-match
conforms 57 of 103 (46 nonconforming); of those 46, 23 are accepted
under the prefix-only design (80 minus 57) and 23 remain genuinely
nonconforming (20 empty + 3 other) — not 61 of anything. This plan does
not carry forward any uncomputed figure; the script above is the single
source of truth and should be re-run at Gate 1 review time against the
then-current `main`, since more plans may have been added since this
measurement.

**The three "other" cases:**

1. `docs/plans/agf-nutshell-flyer-in-repo.md` — `"human governor;
   approved in chat when selecting the CI-generation approach"`. Free-text
   description, no role-name-or-ID prefix. Migration case (rewrite to a
   citation-shaped value), not evidence the design is wrong.
2. `docs/plans/learning-record-nonsequence-ids.md` — `"20260621T145946Z-planner-architect-h4cj"`.
   Cites a **`planner-architect`-authored** record, not governor-class.
   Real historical instance of exactly the risk this fix targets, or a
   legitimate delegation this plan's author lacks context to judge.
   **Flagged for governor-reviewer disposition, not resolved here.**
3. `docs/plans/v250-rc1-release-publication.md` — the case that drove
   the §2 design correction above. Starts with free text
   ("human governor..."), not a role-name-or-ID prefix, even though a
   real governor-reviewer citation appears later in the same value.
   Same disposition as case 1: migration case, not a design flaw —
   this is the design working as intended.

## Migration — BREAKING, explicit Path A / Path B decision required

### Why the schema change forces a binary choice

**Correction (contribution-auditor `5k14`, B-G1-MIG-5).** The prior
draft recommended an "advisory window" (ship as WARN for one release,
flip to blocking later) as if it were compatible with the §4 schema
change. It is not: `minLength: 1` at the JSON Schema layer fails
immediately and cannot be selectively downgraded to non-blocking by the
Rust validator layer above it — schema validation runs first and its
failure is unconditional. An advisory window is only achievable by
**not** shipping the schema change yet. This plan therefore does not
recommend a path; it asks Gate 1 review to choose one of exactly two
executable designs.

### Path A — advisory first

Ship only a new, named, non-blocking semantic diagnostic in the Rust
validator (no schema change yet) that reports every plan whose
`gate1_approved_by` would fail the §2 check, without failing the
overall `agov plan validate` exit code. Give the diagnostic a fixed
sunset (a named future release or date). Use the sunset window to
disposition the 23 current non-conforming records (20 empty + 3 other).
Ship the `minLength: 1` schema change and flip the semantic check to
blocking together, in a later release, once the corpus is clean or the
remaining exceptions are explicitly accepted and recorded.

### Path B — block now

Resolve all 23 current cases first, as a separate, preceding migration
MR: disposition each (rewrite to a conforming citation, or record an
explicit accepted-exception decision naming why). Only after that MR
merges, ship the schema change and the blocking semantic check
together in the same release.

### What this plan does not decide

Gate 1 review selects Path A or Path B. Either path additionally needs:
a decision on whether historical nonconforming records are rewritten
(and if so, whether the governor re-attests each one or the plan
records an explicit "migrated, authority not re-attested" disclosure —
see §CI/rollback scope below) or left as recorded, accepted exceptions;
and the specific release/version boundary the sunset or blocking flip
lands in.

## CI and rollback scope

**Correction (contribution-auditor `5k14`, B-G1-SCOPE-6).** An earlier
draft claimed CI "will block on plans that previously validated
cleanly" and that rollback is "a plain code revert with no data-side
undo." Both were imprecise. Verified directly against
`.gitlab-ci.yml` (line ~796): the governance validation job runs

```
git diff --name-only -z "$base"...HEAD -- 'docs/plans/*.md' | xargs -0r agov plan validate
```

— **only files changed in the current diff range**, not the full
`docs/plans/` corpus. Shipping this check therefore does **not**
retroactively fail CI on the 23 existing nonconforming historical
plans merely by existing; it fails CI only if and when one of those
files is touched again, or if a new plan is authored nonconforming.

This plan does not itself decide whether a separate full-corpus job is
introduced (to surface the 23 cases proactively rather than only on
next-touch) — that is a Gate 1 decision, informed by which Migration
Path (A or B) is chosen: Path B requires disposing all 23 cases before
the blocking check ships regardless of whether a full-corpus CI job
exists, since the migration MR itself is the mechanism; Path A could
reasonably pair with a full-corpus advisory job to surface the list
during the sunset window.

**Rollback, corrected:** if implementation as part of Path B rewrites
any of the 23 historical `gate1_approved_by` fields, that is a
data/history change to those files, not only a code change — rollback
of the validator/schema code does not undo a rewritten historical
approval field. Any such rewrite must be its own reviewable diff
(visible per-file in the migration MR), and rollback of that migration
MR is a separate, explicit git revert of those specific file changes,
distinct from reverting the validator/schema commit.

## Test strategy

Following this project's existing pattern in `plan_validator.rs`'s
`#[cfg(test)] mod tests`, and expanded per contribution-auditor `5k14`
(B-G1-TEST-7):

- Positive — bare role: `gate1_approved_by: "governor-reviewer"`,
  MANIFEST has `governor-reviewer` active — passes.
- Positive — bare role plus citation suffix:
  `"governor-reviewer; decision 20260802T091047Z-governor-reviewer;
  MR !927 exact head ..."` — passes (prefix match on shape 1).
- Positive — coordination-ID prefix:
  `"20260802T192253Z-governor-reviewer-gkeq"` — passes (shape 2).
- Positive — coordination-ID prefix with `.md` suffix:
  `"20260802T192253Z-governor-reviewer-gkeq.md"` — passes.
- Negative — empty: `""` — blocks.
- Negative — non-governor role: `"coder-tester"` (real, active,
  non-governor-class) — blocks, message distinguishes from "unknown
  role."
- Negative — near-miss string: `"governor-reviewers"` (trailing
  plural) — blocks. `"  governor-reviewer  "` (untrimmed) — passes
  after trim.
- Negative — role name inside unrelated prose, not at the start: the
  real `v250-rc1-release-publication.md` value from Migration evidence
  case 3 — blocks (this is the exact case that drove the §2 design
  correction; it must have a dedicated regression test, not only the
  corpus fixture below).
- Negative — fabricated citation for authenticity framing: a
  syntactically valid `<date>T<time>Z-governor-reviewer-<suffix>`
  string that does not correspond to any real coordination record —
  passes the shape check (proving the check is structural-only, not
  authentication) but the test itself must assert and document that
  this is expected, not a false negative in the fix.
- Negative — MANIFEST absent entirely: no `MANIFEST.yaml`, `status:
  approved` — blocks (fail closed), distinct from the existing
  `advisory_consultation` fallback test's fail-open behavior on the
  same missing-file condition. Single most important negative test in
  this set.
- Negative — MANIFEST malformed (invalid YAML) — blocks, same fail-closed
  path as absent.
- Negative — MANIFEST valid YAML but with no top-level `agents` key —
  blocks.
- Negative — `governor-reviewer` present in MANIFEST but `status:
  suspended` or `status: skipped` — blocks, message distinguishes "not
  active" from "unknown role."
- Negative — lifecycle scope: `status: active` and `status: complete`
  with empty `gate1_approved_by` — both block, not just `approved`.
- Positive — `status: pending-gate1` with empty `gate1_approved_by` —
  passes (field not yet asserted).
- Corpus regression fixture: a frozen snapshot of the 103-plan
  classification from Migration evidence (or the then-current
  equivalent at implementation time), asserting the exact
  conforming/empty/other counts do not silently drift as the validator
  changes.
- CLI/MCP parity: the same fixture set run through both `agov plan
  validate` (CLI) and `agov_plan_validate` (MCP) produce identical
  pass/fail results — verifying §5's "no separate reimplementation"
  claim rather than assuming it.
- If Path A is selected: separate tests asserting the advisory
  diagnostic's exit code stays 0 (non-blocking) while still emitting
  the named diagnostic text, distinct from the later blocking-state
  tests above.

Per `always/gate-self-test-rule.md`, the MANIFEST-absent negative test
is the mandatory negative self-test for this gate's introduction and
must ship with a visible transcript in the implementing MR's
description.

## Risks

- **Migration burden.** Measured directly: 20 empty + 3 "other" = 23 of
  103 plans in scope, re-verifiable via the script in Migration
  evidence.
- **Design correction risk, generalized.** This plan's own design
  changed twice — bare-match to "anywhere," then "anywhere" to
  prefix-only — each time only because real data was measured before
  finalizing, not assumed. A Gate 1 reviewer should re-run the Migration
  evidence script against current `main` at review time, since more
  plans may have been added since this measurement, and should not
  assume this plan's own arithmetic is error-free by default — this
  revision exists precisely because a prior arithmetic error
  (61-of-81) was caught by independent review, not by this plan's
  author.
- **Scope discipline.** The temptation to also fix
  `advisory_consultation`'s fail-open behavior in the same change is
  real — deliberately deferred per §3.
- **Does not close `gt3x`'s other two gaps** (unauthenticated `from:`
  field generally; mutation-after-first-reference). Explicitly out of
  scope.
- **Evidence provenance risk.** `gt3x` and `hv3k` are cited only as
  provisional context (§Verified current defect); if the governor
  ultimately declines to accept either learning, this plan's reasoning
  trail should be re-checked to confirm nothing here rests solely on a
  declined learning rather than on `sjqm`/`4g1e`, which are both
  already on `main`.

## Rollback

Revert the branch before merge. After merge: reverting the
validator/schema commit is a plain code revert. If Path B's migration
MR rewrote any historical `gate1_approved_by` fields, that is a
separate data/history change (see §CI and rollback scope) with its own
independent revert, not implied by reverting the validator/schema
commit.

## Out of scope (explicitly deferred, not silently dropped)

- Making `advisory_consultation`'s MANIFEST-absent behavior fail closed
  (§3).
- Extensible governor-class metadata on MANIFEST roles beyond the v1
  hardcoded allowlist (§2) — reconsideration trigger stated there.
- Coordination-record `from:` field authentication (`gt3x`, provisional,
  unresolved).
- Content-hash-on-first-reference tamper-evidence for immutable records
  (`gt3x`, provisional, unresolved).
- Enforcement-tier self-reporting vocabulary (`t8mz`'s recommendation,
  provisional) — a documentation/vocabulary change, not a
  schema/validator change; belongs in its own plan if pursued.
- A full-corpus CI job to surface nonconforming plans proactively
  (§CI and rollback scope) — a Gate 1 decision contingent on the
  Path A/Path B choice, not decided by this plan.

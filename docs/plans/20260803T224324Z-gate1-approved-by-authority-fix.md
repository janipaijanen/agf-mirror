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
      an access-control/authorization surface under
      `always/high-risk-operations-rule.md`. The central design question
      (fail closed vs. fail open when `agf/agents/MANIFEST.yaml` is
      absent) is exactly the class of decision that rule requires a plan
      and explicit confirmation for before implementation.
  - role: contribution-auditor
    status: consulted
    reason: >
      This plan exists because contribution-auditor's own review
      (`docs/coordination/20260803T210104Z-contribution-auditor-4g1e.md`,
      findings B-GS-3/B-GS-5) and a separate Grok red-team
      (`20260803T212420Z-hv3k` in `agf/learnings/foundry.yaml`)
      independently converged on the same conclusion from different
      starting proposals: a naive non-empty check on
      `gate1_approved_by` is unsafe. Renewed review of this plan by
      contribution-auditor is the natural next step before Gate 1
      approval.
---

# Plan: `gate1_approved_by` authority-enforcement fix

## Decision requested

Authorize implementation of a schema-plus-validator fix that requires
`gate1_approved_by` to be non-empty and identify an active
governor-class role, for plans whose `status` has reached `approved`,
`active`, or `complete`. Planning only. No implementation, schema
change, or version bump is authorized by this plan itself.

**Authorization for this planning work**: explicit human authorization
was given in this session, superseding an earlier declination
(`docs/coordination/20260803T211911Z-planner-architect-7jhj.md`) to
draft this candidate absent explicit task-level sign-off. That
declination was itself the subject of a since-corrected claim — see
`agf/learnings/foundry.yaml` entry `20260803T212419Z-nq7p`'s
correction: the `planner-architect` role's MANIFEST activation was
never in question; what was missing was task-specific authorization
for exactly this drafting task, which this session now has.

## Verified current defect

Direct inspection of `agf/schemas/gate1-plan.schema.json` (current
`main`, unchanged by this session's other in-flight MRs):

```json
"gate1_approved_by": {
  "type": "string"
}
```

No `minLength`, no `pattern`, no cross-record check against
`agf/agents/MANIFEST.yaml`. A plan with `status: approved` and
`gate1_approved_by: ""` (empty) validates cleanly, and so does
`gate1_approved_by: "coder-tester"` (an AI implementation role, not
the human governor role) — both independently reproduced against the
real schema by two separate reviewers (Grok and this session, recorded
in `docs/coordination/20260803T204257Z-planner-architect-sjqm.md`
Track 3 finding 1, and Codex's own adversarial synthesis).

The existing precedent for a MANIFEST cross-check is
`tooling/rust/crates/agov-coord/src/plan_validator.rs`'s
`advisory_consultation` role check (`role_statuses`,
`validate_semantics_with_manifest`). That check already demonstrates
exact-match, case-sensitive role lookup (confirmed:
`infosec-sentinels` plural is rejected as `unknown advisory role`).
It also demonstrates the exact trap this plan must not repeat: its own
test `missing_manifest_falls_back_to_required_roles_only` shows that
when `agf/agents/MANIFEST.yaml` cannot be read, the check silently
passes for the two hardcoded required roles rather than blocking. A
naive reuse of this pattern for `gate1_approved_by` would be "fully
bypassed" (Grok's phrase, `20260803T212420Z-hv3k`) in any workspace
with no `MANIFEST.yaml` present — a condition this session reproduced
directly; how common that condition is across adopters was not
measured and is not claimed here or anywhere in this plan.

## Why the naive fix (require non-empty, matching *any* MANIFEST role)
## is not proposed here

Grok's red-team of the originally-proposed naive fix
(`20260803T212420Z-hv3k`), independently convergent with Codex's
`4g1e` (findings B-GS-3/B-GS-5) from a different starting proposal,
found four classes of bypass:

1. **Migration.** Existing approved plans with empty or
   non-governor `gate1_approved_by` values become non-conforming the
   instant this ships as blocking, with no grandfather path. This is a
   **BREAKING** change under this project's own
   `downstream-impact-rule.md`, not additive — see Impact section
   below.
2. **MANIFEST-absent silent skip.** As above: reusing the existing
   advisory-role pattern verbatim inherits its silent-pass-when-absent
   behavior.
3. **Exact-match not explicitly required.** The existing pattern
   already behaves exact-match, case-sensitive, but a *new* check
   written independently could easily reintroduce fuzzy matching
   (trailing whitespace, wrong case, `governor-reviewers` plural) if
   the requirement isn't stated explicitly rather than assumed from
   the sibling implementation.
4. **Lifecycle scope.** A rule that checks only `status: approved`
   leaves `active` and `complete` plans — which have already passed
   through `approved` — unguarded if the approver field is edited
   afterward, or if a plan is hand-authored directly at `active` or
   `complete`. A schema-only `minLength: 1` check, with no MANIFEST
   bind at all, still accepts `gate1_approved_by: "coder-tester"`.

## Design — non-naive P0 shape

This is the shape both red-teams converged on as sufficient to close
the reproduced gap without overclaiming what it closes.

### 1. Scope: which statuses are checked

`gate1_approved_by` is checked whenever `status` is one of `approved`,
`active`, `complete` — i.e., every status that asserts a Gate 1
decision has already happened. `pending-gate1` is exempt (no decision
has been asserted yet).

### 2. The check itself

**Revised after measuring real usage (see Migration evidence below) —
this design is not the one originally drafted.** An initial draft of
this plan required `gate1_approved_by` to be an *exact, bare* match of
a governor-class role name, directly copying the
`advisory_consultation` role-lookup pattern. Measuring this repository's
own 103 in-scope plan files against that rule before finalizing the
design (rather than assuming a small, unmeasured "some plans will need
migration" estimate) showed it would have wrongly flagged 61 of 81
genuinely governor-approved plans as nonconforming, because this
project's actual, real convention is to write `gate1_approved_by` as a
short citation — role name plus a decision-record ID, MR number, exact
head SHA, or coordination sequence number — not a bare role token. That
citation convention is *better* evidence than a bare role string, not
worse, and a design that broke it to gain a marginal precision
improvement would have been a regression, not a fix. Accordingly:

- `gate1_approved_by`, after trimming leading/trailing whitespace,
  must be non-empty.
- It must satisfy **one** of two shapes:
  1. **Prefix match**: starts with an exact-match (case-sensitive)
     governor-class role name from the allowlist below, followed by a
     non-alphanumeric character or end-of-string (so
     `governor-reviewer; decision ...` matches but
     `governor-reviewers` does not); or
  2. **Embedded citation match**: contains a substring matching
     `<8-digit-date>T<6-digit-time>Z-<governor-class-role>-<4-char-suffix>`
     — this project's own coordination-record and learning-record ID
     format (see `agf/schemas/coordination-message.schema.json`),
     which embeds the authoring role directly in the ID. A citation of
     `20260802T192253Z-governor-reviewer-gkeq` is accepted on this
     basis even though the field doesn't start with the role name.
  - Neither shape performs a MANIFEST *lookup* of the cited role by
    itself proving that specific decision record exists, was itself
    validly authored, or says what it's cited as saying — this check
    stays at the level `gt3x`/`hv3k` scoped it to: does the approver
    field name (not prove) a governor-class identity. See §6.
- The governor-class role name used in both shapes must additionally
  be an **exact-match** key of a role in `agf/agents/MANIFEST.yaml`
  whose `status` is `active` **and** whose role is in a new, explicit
  **governor-class allowlist**.
- **v1 governor-class allowlist: `governor-reviewer` only.** No new
  MANIFEST schema field (e.g. a `governor_class: true` marker) is
  proposed in this plan — per `simplicity-rule.md`, building
  extensible per-role class metadata for a set of exactly one current
  member is speculative generality ahead of a second real case. If a
  second governor-class role is ever activated in this project's own
  `MANIFEST.yaml`, that is the trigger to revisit this list, not a
  reason to build the general mechanism now.
- A free-text synonym with no role-name substring at all (e.g. "human
  governor", found in one real plan below) does **not** match either
  shape and is treated as nonconforming under this design. This is a
  deliberate, named tradeoff — see the Migration evidence's "other"
  category — not an oversight.

### 3. Fail-closed on missing MANIFEST — scoped to this new check only

When `agf/agents/MANIFEST.yaml` cannot be read (absent, malformed, or
the `governor-reviewer` entry is missing or not `active`), the new
`gate1_approved_by` check **fails closed**: it is reported as a
blocking error, not silently skipped. This is a deliberate divergence
from the existing `advisory_consultation` check's current
fail-open-on-missing-MANIFEST behavior.

**This plan does not propose changing the existing
`advisory_consultation` fallback to also fail closed**, even though
`hv3k`'s recommendation asks for that as "the same policy for every
role cross-check." Reasoning: changing existing, already-shipped
validator behavior for `advisory_consultation` is a second BREAKING
change with its own separate blast radius (every plan file that
currently validates cleanly with no `MANIFEST.yaml` present would
start failing), and bundling it here would violate this project's own
branch-sizing test (`branch-and-worktree-rule.md` — one branch should
answer one yes/no question). Recorded here as an explicit, deliberately
deferred follow-up, not a silently dropped recommendation: **a second
plan should evaluate fail-closed-by-default for
`advisory_consultation` on its own merits**, separately reviewed.

### 4. Schema change

`agf/schemas/gate1-plan.schema.json`:

```json
"gate1_approved_by": {
  "type": "string",
  "minLength": 1
}
```

`minLength: 1` alone only proves non-emptiness (bypassable with
`"coder-tester"`, as already reproduced) — it is necessary but not
sufficient. The identity/role-class check itself cannot be expressed
in JSON Schema alone (it requires reading a second file,
`MANIFEST.yaml`); it belongs in the Rust semantic validator alongside
the existing `advisory_consultation` cross-check, following the same
architectural pattern already established there.

### 5. Validator change

`tooling/rust/crates/agov-coord/src/plan_validator.rs`:

- Add a `governor_class_roles()` function returning `&["governor-reviewer"]`,
  analogous to the existing `required_roles()`.
- Extend `validate_semantics_with_manifest` (or add a sibling function
  called from `validate_file` alongside it) to, when `status` is
  `approved`/`active`/`complete`:
  - Read `gate1_approved_by`, trim it.
  - If empty after trim: blocking error.
  - If `agf/agents/MANIFEST.yaml` is unreadable: blocking error (fail
    closed — new behavior, scoped to this check only, per §3 above).
  - If the trimmed value does not exact-match an entry in
    `governor_class_roles()` whose MANIFEST status is `active`:
    blocking error, with a message distinguishing "not a governor-class
    role" from "governor-class role exists but is not active" (mirrors
    the existing `unknown advisory role` vs. `advisory role is not
    active` distinction already in this file).

### 6. What this fix does and does not close

Stated explicitly, per `t8mz`'s recommendation to apply AGF's own
maturity vocabulary to its own enforcement claims: this raises the
cost of a forged or empty approval passing schema validation from
zero to "the forger must also know and type an exact, currently-active
governor-class role string." It does **not** authenticate that the
human governor themselves wrote `gate1_approved_by: "governor-reviewer"`
— `from:`/`gate1_approved_by:` remain unauthenticated free text at the
record-authorship layer (`gt3x`, unresolved by this plan). It does
**not** freeze history — a later in-place edit of an already-approved
plan's `gate1_approved_by` field is a separate, already-identified gap
(`gt3x` finding 3, content-hash-on-first-reference, also unresolved by
this plan). This plan closes exactly one thing: schema-shape-only
validation currently accepts empty or non-governor approvers; after
this plan, it will not.

## Migration evidence (measured, not estimated)

Direct measurement against this repository's own `docs/plans/*.md`,
2026-08-03 (script: parse YAML frontmatter, filter to `status` in
`{approved, active, complete}`, classify `gate1_approved_by` under the
§2 design above):

| Category | Count | Disposition |
|---|---|---|
| Total in scope (status approved/active/complete) | 103 | — |
| Conforms under the revised (prefix-or-citation) design | 81 | No action needed |
| Truly empty `gate1_approved_by` | 20 | Genuine gap this fix targets — needs a migration option (below) |
| Neither empty nor conforming ("other") | 2 | Needs human disposition, not auto-migration — see below |

The 81/103 conforming figure is why the §2 revision from bare-exact-match
to prefix-or-citation matters: under the originally-drafted bare-match
design, this repository's own real migration burden would have been
measured as 46/103 nonconforming, when the actual ungoverned gap is
20/103 (plus 2 needing individual review). Ungoverned here means
literally empty, not merely differently formatted.

**The two "other" cases, found by this measurement, not assumed in
advance:**

1. `docs/plans/agf-nutshell-flyer-in-repo.md` —
   `gate1_approved_by: "human governor; approved in chat when selecting
   the CI-generation approach"`. A free-text description of the same
   authority, not a role-name citation. Recommend treating as a
   migration case (rewrite to a citation-shaped value), not as evidence
   the check design is wrong.
2. `docs/plans/learning-record-nonsequence-ids.md` —
   `gate1_approved_by: "20260621T145946Z-planner-architect-h4cj"`. This
   is a real, historical `status: approved` plan whose approver field
   cites a **`planner-architect`-authored** record, not a
   governor-class one. This is either a genuine historical instance of
   exactly the risk this plan exists to prevent, or a legitimate
   Gate-1-by-delegation case this plan's author does not have enough
   context to judge. **Flagged here for governor-reviewer disposition,
   not resolved by this plan** — per `always/high-risk-operations-rule.md`,
   an AI-authored plan does not unilaterally reinterpret a historical
   governance decision's validity.

## Migration — BREAKING, with explicit path

### Impact

```
Impact:
  Foundry:          Yes, measured directly: 22 of this repository's own
                     103 in-scope plan files (20 empty + 2 flagged for
                     individual review) fail the proposed check today
                     and need one of the migration options below applied
                     before this ships as blocking, or CI will block on
                     plans that previously validated cleanly.
  Direct adopters:  BREAKING. Any adopter plan file with status
                     approved/active/complete and an empty or
                     non-governor-class gate1_approved_by will start
                     failing `agov plan validate` the moment this ships.
  Downstream:       No change to plan file shape for conforming plans
                     (those that already name a real governor approver).
                     No change to agov's CLI surface or exit-code
                     contract — this is a new failure condition within
                     the existing validate command, not a new command.
  Migration:        Required. See options below; this plan does not
                     pick one — that choice is left to Gate 1 review.
  Evidence:         20260803T205236Z-gt3x (original gap, three
                     independent reproductions); 20260803T212420Z-hv3k
                     (naive-fix red-team, independently convergent with
                     Codex's 4g1e B-GS-3/B-GS-5).
```

### Migration options (for Gate 1 review to choose between, not decided here)

1. **Advisory window.** Ship the check as WARN (non-blocking) for one
   release, listing every currently-nonconforming plan in CI output;
   flip to blocking in the following release once the list is empty or
   explicitly accepted.
2. **One-shot rewrite with honest disclosure.** Rewrite every
   historical nonconforming plan's `gate1_approved_by` to a value like
   `"migrated, authority not re-attested — see <this fix's plan id>"`,
   which itself fails the new governor-class check and would need a
   schema-level escape hatch (e.g. a recognized sentinel string) — adds
   complexity; not preferred without Gate 1 discussion.
3. **Strict-profile opt-in.** New check only runs when an explicit flag
   or config is set, e.g. `agov plan validate --strict-authority`;
   default `agov plan validate` behavior unchanged. Weakest fix (an
   adopter must opt in to the very check meant to catch that they
   forgot to), but zero migration burden.

This plan's author recommendation, stated but not self-approved: option
1 (advisory window) — it does not require a schema escape hatch, and it
surfaces the actual current nonconforming-plan count in this
repository before anyone commits to a specific migration mechanism.

## Test strategy

Following this project's existing pattern in
`plan_validator.rs`'s `#[cfg(test)] mod tests`:

- Positive: `status: approved`, `gate1_approved_by: "governor-reviewer"`,
  MANIFEST has `governor-reviewer` active — passes.
- Negative — empty: `gate1_approved_by: ""` — blocks.
- Negative — non-governor role: `gate1_approved_by: "coder-tester"`
  (a real, active, non-governor-class role) — blocks, message
  distinguishes from "unknown role."
- Negative — near-miss string: `gate1_approved_by: "governor-reviewers"`
  (trailing plural) and `"  governor-reviewer  "` untrimmed-but-valid
  (must pass after trim) — exercises exact-match-after-trim explicitly.
- Negative — MANIFEST absent: no `MANIFEST.yaml` at all, `status:
  approved` — blocks (fail closed), distinct from the existing
  `advisory_consultation` fallback test's fail-open behavior on the
  same missing-file condition — this is the single most important
  negative test in this set, since it is the exact condition `hv3k`
  and `4g1e` both found bypasses a naive reuse of the existing pattern.
- Negative — lifecycle scope: `status: active` and `status: complete`
  with empty `gate1_approved_by` — both block (not just `approved`).
- Positive — `status: pending-gate1` with empty `gate1_approved_by` —
  passes (field not yet asserted).

Per `always/gate-self-test-rule.md`, the MANIFEST-absent negative test
above is the mandatory negative self-test for this gate's introduction,
and must be included in the same MR that ships the check, with a
visible transcript in the MR description.

## Risks

- **Migration burden.** Measured directly (see Migration evidence
  above), not estimated: 20 truly-empty plans plus 2 needing individual
  human disposition, out of 103 in scope. Small enough that the
  advisory-window option (§Migration option 1) is tractable without
  further measurement.
- **Design correction risk, generalized.** This plan's own §2 design
  changed materially (bare-exact-match to prefix-or-citation) only
  because real usage was measured before finalizing it, not assumed.
  The same caution applies to anything not explicitly measured in this
  plan: the governor-class allowlist (§2, `governor-reviewer` only) and
  the coordination-record-ID citation pattern (§2 shape 2) are both
  drawn from what this repository's real plan files actually contain
  today, not from a specification written in the abstract — a Gate 1
  reviewer should re-check that assumption against current
  `docs/plans/*.md` at review time, since more plans may have been
  added since this measurement.
- **Scope discipline.** The temptation to also fix `advisory_consultation`'s
  fail-open behavior in the same change is real (same file, same
  pattern, same author) — deliberately deferred per §3 above to keep
  this branch answerable with one yes/no question.
- **Does not close `gt3x`'s other two gaps** (unauthenticated `from:`
  field generally; mutation-after-first-reference). Explicitly scoped
  out; both remain open, tracked in `agf/learnings/foundry.yaml`.

## Rollback

Revert the branch before merge. After merge, revert the merge commit —
this is a validator/schema change with no data migration, so rollback
is a plain code revert with no separate data-side undo.

## Out of scope (explicitly deferred, not silently dropped)

- Making `advisory_consultation`'s MANIFEST-absent behavior fail closed
  (§3).
- Extensible governor-class metadata on MANIFEST roles beyond the v1
  hardcoded allowlist (§2).
- Coordination-record `from:` field authentication (`gt3x`, unresolved).
- Content-hash-on-first-reference tamper-evidence for immutable records
  (`gt3x`, unresolved).
- Enforcement-tier self-reporting vocabulary (`t8mz`'s recommendation) —
  a documentation/vocabulary change, not a schema/validator change;
  belongs in its own plan if pursued.

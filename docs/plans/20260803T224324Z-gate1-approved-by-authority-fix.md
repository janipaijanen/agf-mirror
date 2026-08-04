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
      B-G1-SCOPE-6, B-G1-TEST-7) plus two advisories (A-G1-ROLE-1,
      A-G1-AUTHZ-2), all addressed at exact head `c1cf1aba`. Round 2
      review (`20260803T231107Z-contribution-auditor-d2jr.md`) at that
      head: BLOCK, six further blocking findings (B-G1-TIME-R2,
      B-G1-PATH-A-R2, B-G1-EXCEPT-R2, B-G1-IMPACT-R2, B-G1-REPRO-R2,
      B-G1-HANDOFF-R2) plus one advisory (A-G1-TEST-NAME-R2), confirming
      B-G1-AUTH-1/EVID-2/DATA-3/SPEC-4 and most of B-G1-TEST-7 resolved.
      All six findings and the advisory addressed at exact head
      `e3550834`. Round 3 review
      (`20260803T232539Z-contribution-auditor-jhs4.md`) at that head:
      BLOCK, two further blocking findings (B-G1-AUTH-SCOPE-R3 --
      three mutually contradictory implementation-authorization
      statements; B-G1-MIGRATION-TRUTH-R3 -- the "migrated, authority
      not re-attested" disclosure has no honest, executable
      representation under the §2 check), confirming B-G1-TIME-R2,
      B-G1-EXCEPT-R2, B-G1-IMPACT-R2, B-G1-REPRO-R2, B-G1-HANDOFF-R2,
      and A-G1-TEST-NAME-R2 resolved, and independently re-running the
      embedded stdlib inventory script verbatim (confirmed
      80/20/3 again). `jhs4` also recommends Path A for this release
      sequence. Both findings addressed at exact head `dff4ef3a`. Round
      4 review (`20260804T000320Z-contribution-auditor-s3m1.md`) at
      that head: BLOCK, one further blocking finding
      (B-G1-SOURCE-ABSENCE-R4 -- an empty `gate1_approved_by` field
      was overstated as proof no approval source exists anywhere,
      rather than only that the field itself names none), confirming
      B-G1-AUTH-SCOPE-R3 and B-G1-MIGRATION-TRUTH-R3 resolved, plus one
      advisory (A-G1-INVENTORY-R4, refresh the baseline count before
      Gate 1 -- corpus grew to 104/81/20/3, nonconforming burden
      unchanged at 23). `s3m1` reaffirms the Path A recommendation.
      Both addressed in this revision. The governor separately posted
      a Path A decision contingent on this round-5 fix
      (`20260804T000658Z-governor-reviewer-n5jr.md`) before this round
      landed; that contingency is what round 5 satisfies. Renewed
      review requested at the corrected exact head.
---

# Plan: `gate1_approved_by` structural approver-field hygiene fix

**Renamed from "authority-enforcement fix" per contribution-auditor's
`5k14` finding B-G1-AUTH-1.** This plan does not authenticate an actor,
verify a cited record exists or says what it's cited as saying, or
prove the human governor made a decision. It closes exactly one thing:
today, `gate1_approved_by` accepts empty or arbitrary text on an
`approved`/`active`/`complete` plan with zero schema-shape check. After
this plan, it must at minimum name — not prove — a MANIFEST-configured
governor-class role (present in this project's role list; current
active/suspended/skipped status is not checked, per B-G1-TIME-R2), in
one of two precisely specified textual shapes.
That boundary is stated once here and then held consistently through
every section below, per `5k14`'s explicit instruction.

## Decision requested

**Correction (contribution-auditor `jhs4`, B-G1-AUTH-SCOPE-R3): one
unambiguous authorization contract, replacing three contradictory
statements in the prior revision.** This plan's Gate 1 approval, once
given, authorizes implementation of **exactly the selected path below,
and nothing beyond that path's own stated prerequisites and scope** —
not "planning only" in the abstract, and not both paths' full scope
regardless of which is chosen.

- **If Path A is selected:** approval authorizes implementation of the
  non-blocking advisory diagnostic only (§Migration, Path A). The
  later schema change and blocking flip are explicitly **not**
  authorized by this Gate 1 decision and require their own separate,
  future Gate 1 plan.
- **If Path B is selected:** approval authorizes implementation of the
  schema change and blocking check **only after** every one of the 23
  current cases has an individually verified, truthfully-cited
  governor approval source or a fresh governor re-attestation (see
  §Migration, Path B's corrected requirement below — no manufactured
  "conforming" shortcut exists). Until that per-record work is done,
  no implementation is authorized under Path B either.

The governor selects Path A or Path B as part of the same Gate 1
decision that approves this plan; this plan does not pre-select one
(contribution-auditor's `jhs4` recommends Path A — see §Migration).

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

**Evidence provenance (contribution-auditor `5k14`, B-G1-EVID-2;
updated at this revision).** `sjqm` and `4g1e`, both cited above, are
on `main` and are sufficient by themselves to establish the reproduced
structural gap and the MANIFEST-absent trap. This plan's earlier draft
additionally cited learning entries `gt3x` and `hv3k` as if they were
already accepted corpus; at the time of `5k14`'s review they existed
only in blocked MRs, so that citation was premature. Since then,
contribution-auditor independently CLEARed both at corrected heads
(`20260803T225227Z-contribution-auditor-k3n4.md` for `gt3x`,
`20260803T225850Z-contribution-auditor-hanm.md` for the learning MR
containing `hv3k`), and **the human governor has since merged both**
(!1086 and !1087, both now on `main`). `gt3x` and `hv3k` are therefore
accepted corpus as of this revision, not provisional — cited directly
alongside `sjqm`/`4g1e` below, no longer hedged.

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
  be an **exact-match** key **present** under `agents:` in
  `agf/agents/MANIFEST.yaml` (any `status` value — see the
  correction below) and a member of the governor-class allowlist
  below.

**Correction (contribution-auditor `d2jr`, B-G1-TIME-R2): presence, not
current-active status.** An earlier draft additionally required the
matched role's MANIFEST `status` to be `active` at validation time.
Contribution-auditor correctly found this retroactively unsound: a
`status: complete` plan can have been validly approved by a governor
role that is *today* suspended, renamed, or retired — that role's
current status says nothing about whether it was active at the
historical approval timestamp, and this stateless file validator has
no access to MANIFEST's state at that past timestamp. Requiring
current-`active` status would make editing an old, validly-approved
plan for an unrelated reason (a typo fix) start failing hygiene
validation purely because time passed and the role's status changed —
turning role lifecycle into retroactive approval invalidation, which
contradicts this check's own stated structural-only boundary (§6).

Fixed design: the check requires only that the role name **appears as
a key** under `agf/agents/MANIFEST.yaml`'s `agents:` mapping — status
`active`, `suspended`, or `skipped` all satisfy this. This still binds
the check to a real, project-configured role (not an arbitrary string
untethered from any project's actual role list) without asserting
anything about whether that role was staffed at any particular moment.
Distinguishing "role never configured in this project at all" from
"role configured but not currently staffed" remains useful and is
preserved; asserting "currently staffed" as a proxy for "staffed at
approval time" is what's removed.
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
the `governor-reviewer` key is missing entirely from `agents:`), the new
`gate1_approved_by` check **fails closed**: blocking error, not
silently skipped. Per the B-G1-TIME-R2 correction above, a
`governor-reviewer` entry present with any `status` value (including
`suspended` or `skipped`) satisfies this — only a fully absent
MANIFEST file or a fully absent role key fails closed. This is a
deliberate divergence from the existing `advisory_consultation`
check's current fail-open-on-missing-MANIFEST behavior.

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
  - If the prefix parser above does not match a `governor_class_roles()`
    member: blocking error, "does not name a governor-class role."
  - If the prefix parser matches a `governor_class_roles()` member, but
    that role key is entirely absent from `agf/agents/MANIFEST.yaml`'s
    `agents:` mapping: blocking error, "governor-class role not
    configured in this project's MANIFEST" — status value (active,
    suspended, skipped) is irrelevant per B-G1-TIME-R2; only full
    absence of the key blocks.
- MCP surface: `agov_plan_validate` wraps this validator directly (no
  separate reimplementation), so CLI and MCP behavior stay identical by
  construction — verified by a shared test fixture, not by two
  independent implementations (see §Test strategy).

### 6. What this fix does and does not close

This raises the cost of a forged or empty approval passing
schema/validator checks from zero to "the value's first token must
name an exact governor-class role that is present in this project's
MANIFEST (any status — see B-G1-TIME-R2), as a bare name or a
shape-valid coordination-ID citation." It does **not** authenticate
that the human governor themselves wrote the value, verify the cited
coordination record exists or contains a matching approval, or prove
the human governor made the decision — `from:`/`gate1_approved_by:`
remain unauthenticated free text at the record-authorship layer (`gt3x`,
unresolved by this plan). It does **not** freeze history — a later
in-place edit of an already-approved plan's `gate1_approved_by` field
is a separate gap (`gt3x` finding 3, content-hash-on-first-reference,
also unresolved).

## Migration evidence (measured, reproducible)

Direct measurement against this repository's own `docs/plans/*.md` on
current `main`, re-run at this revision (not carried over from the
prior draft), using the exact §2 prefix-only rule.

**Correction (contribution-auditor `d2jr`, B-G1-REPRO-R2).** The prior
revision's script `import`ed `yaml` (PyYAML). PyYAML is not declared
anywhere in this project (no `mise.toml`/`pyproject.toml`/
`requirements*.txt` entry, no project `.py` file imports it) — a
reviewer following the "re-run this script" instruction in a clean
environment would hit `ModuleNotFoundError` before it counted anything.
The script below is stdlib-only: it hand-parses just the two scalar
frontmatter fields this measurement needs (`status`,
`gate1_approved_by`), including YAML's double-quoted-scalar line
folding, without a general YAML parser. Verified to reproduce the exact
same 80/20/3 result as the PyYAML version it replaces.

```python
#!/usr/bin/env python3
"""Dependency-free (stdlib only) frontmatter scanner for docs/plans/*.md."""
import glob
import re

FIELD_RE = re.compile(r'^([A-Za-z_][A-Za-z0-9_-]*):\s*(.*)$')


def extract_frontmatter_fields(text, keys):
    lines = text.splitlines()
    if not lines or lines[0].strip() != "---":
        return {}
    end = next((i for i in range(1, len(lines)) if lines[i].strip() == "---"), None)
    if end is None:
        return {}
    fm_lines = lines[1:end]
    result, i = {}, 0
    while i < len(fm_lines):
        m = FIELD_RE.match(fm_lines[i])
        if not m or m.group(1) not in keys:
            i += 1
            continue
        key, rest = m.group(1), m.group(2).strip()
        if rest.startswith('"') and not (len(rest) > 1 and rest.endswith('"')):
            parts = [rest[1:]]
            i += 1
            while i < len(fm_lines) and fm_lines[i].startswith((" ", "\t")):
                cont = fm_lines[i].strip()
                if cont.endswith('"'):
                    parts.append(cont[:-1]); i += 1; break
                parts.append(cont); i += 1
            result[key] = " ".join(p for p in parts if p)
            continue
        result[key] = rest[1:-1] if rest.startswith('"') and rest.endswith('"') else rest
        i += 1
    return result


role = "governor-reviewer"
id_prefix_re = re.compile(
    rf'^\d{{8}}T\d{{6}}Z-{re.escape(role)}-[a-z0-9]{{4}}(?![A-Za-z0-9])'
)
bare_prefix_re = re.compile(rf'^{re.escape(role)}(?![A-Za-z0-9])')

total = empty = other = conforming = 0
for path in sorted(glob.glob("docs/plans/*.md")):
    content = open(path, encoding="utf-8", errors="replace").read()
    fields = extract_frontmatter_fields(content, {"status", "gate1_approved_by"})
    if fields.get("status") not in ("approved", "active", "complete"):
        continue
    total += 1
    approver = (fields.get("gate1_approved_by") or "").strip()
    if not approver:
        empty += 1
    elif bare_prefix_re.match(approver) or id_prefix_re.match(approver):
        conforming += 1
    else:
        other += 1

print(f"total={total} conforming={conforming} empty={empty} other={other}")
```

| Category | Count | Disposition |
|---|---|---|
| Total in scope (status approved/active/complete) | 104 | — |
| Conforms under the prefix-only design | 81 | No action needed |
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

**Refreshed baseline (contribution-auditor `s3m1`, A-G1-INVENTORY-R4):**
the count above was re-measured against current `main` immediately
before Gate 1 review, per the instruction two paragraphs up. It moved
from 103/80/20/3 to **104/81/20/3** — one additional conforming plan
(`docs/plans/nasa-llis-influence-attribution.md`) was approved and
merged after the prior measurement. The nonconforming burden this
plan targets is unchanged at exactly 23 (20 empty + 3 other). Both
`s3m1` and an independent re-run of the embedded script above, done
while addressing this finding, produced the identical `total=104
conforming=81 empty=20 other=3`. Re-run the script again immediately
before acting on the governor's decision, since the count is a live
measurement, not a fixed constant of this plan.

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

### Impact

**Restored (contribution-auditor `d2jr`, B-G1-IMPACT-R2).** The prior
revision's migration prose did not replace this project's mandatory
`downstream-impact-rule.md` block; it is required in addition to that
prose, not instead of it.

```
Impact:
  Foundry:          Yes, measured directly: 23 of this repository's own
                     104 in-scope plan files (20 empty + 3 "other") fail
                     the proposed check today and must become
                     conforming before the blocking flip ships, under
                     either Migration Path. Unchanged historical files
                     are not retroactively failed by CI merely by
                     existing (see CI and rollback scope) -- only
                     touched files are validated by the current
                     governance job.
  Direct adopters:  BREAKING. Any adopter plan file with status
                     approved/active/complete and a gate1_approved_by
                     value that does not start with a governor-class
                     role name or its coordination-ID citation will
                     fail `agov plan validate` once the blocking state
                     ships (Path A: in a later, separately-planned
                     release; Path B: in this release, after its own
                     preceding migration MR).
  Downstream:       No change to plan file shape for conforming plans.
                     No change to agov's CLI surface or exit-code
                     contract for the non-blocking Path A diagnostic
                     (still exit 0); a new failure condition within the
                     existing exit-code contract once blocking ships.
  Migration:        Required. Path A vs. Path B is an open Gate 1
                     decision (see Migration section below); this plan
                     does not pre-select one. No exception/grandfather
                     outcome exists for any of the 23 current cases --
                     all must become conforming before the blocking
                     flip, under either path.
  Evidence:         docs/coordination/20260803T204257Z-planner-architect-sjqm.md
                     (merged, Track 3 finding 1),
                     docs/coordination/20260803T210104Z-contribution-auditor-4g1e.md
                     (merged), and learning entries 20260803T205236Z-gt3x
                     and 20260803T212420Z-hv3k in
                     agf/learnings/foundry.yaml (merged via !1086/!1087)
                     establish the reproduced structural gap and the
                     naive-fix bypasses this design avoids.
```

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

**Corrections in this section (contribution-auditor `d2jr`,
B-G1-PATH-A-R2 and B-G1-EXCEPT-R2), applied to both paths below:**

1. **No "accepted exception" outcome is offered by either path.**
   The prior draft allowed a nonconforming record to remain if the
   governor recorded an "explicit accepted-exception decision." The
   proposed schema and Rust validator have no grandfather marker,
   exception registry, or decision-lookup mechanism — once the blocking
   check ships, such a record fails identically to any other
   nonconforming record regardless of what a coordination record says
   about it. A prose risk-acceptance cannot change validator output.
   This plan does not design a grandfather mechanism (that would be its
   own scoped feature, with its own authority/distribution/rollback
   design — out of scope here per simplicity-rule). Accordingly: **every
   one of the 23 current cases must become conforming (via rewrite or
   governor re-attestation) before the blocking flip ships, under
   either path.** There is no third "leave it as an accepted risk"
   outcome.
2. **Correction (contribution-auditor `jhs4`, B-G1-AUTH-SCOPE-R3):**
   an earlier revision of this item said the blocking flip requires a
   future Gate 1 plan "under either path" — that is wrong for Path B
   and directly contradicted this plan's own Path B text. Only **Path
   A**'s blocking flip is deferred to a future Gate 1 plan. Path B's
   blocking implementation is authorized by *this* Gate 1 decision,
   conditional on every one of the 23 cases first being individually,
   truthfully resolved (see Path B and item 3 below — there is no
   shortcut that manufactures conformity).
3. **Correction (contribution-auditor `jhs4`, B-G1-MIGRATION-TRUTH-R3):**
   an earlier revision offered a `"migrated, authority not
   re-attested"` disclosure string as a way to make a historical
   record "conform" under Path B. This does not work and is removed
   entirely: for such a string to pass the §2 check it would have to
   start with `governor-reviewer` (or a governor-reviewer coordination
   ID) — falsely presenting itself as a governor citation in the same
   breath as disclosing that the governor did not re-attest it. A
   version that doesn't start with the token simply fails the check.
   Neither shape is honest and executable at once. **There is no
   record-level shortcut for Path B.** Each of the 23 cases must get
   either (a) an existing, individually verified governor approval
   source cited truthfully, or (b) a fresh governor re-attestation.
   Records lacking either remain nonconforming and block the flip.

4. **Correction (contribution-auditor `s3m1`, B-G1-SOURCE-ABSENCE-R4):**
   an earlier revision of this item and the Path B caution below both
   said the 20 empty-field cases have "no approval source ... at all"
   or "literally no historical approval source," and concluded fresh
   re-attestation was therefore the only path for all 20. That
   overstates what was actually measured. **The measured fact is only
   that those 20 plan files' `gate1_approved_by` field names no source
   today.** An empty field does not prove no governor decision exists
   elsewhere — in coordination records, GitLab MR/issue history, or
   another durable record. Each of the 20 requires an individual
   bounded source search under Path B: use a verified existing source
   if one is found, otherwise obtain fresh governor re-attestation. No
   individual record may be classified as requiring re-attestation
   until its own bounded search is recorded — the distinction changes
   the actual human workload the governor is accepting under Path B
   and must not be collapsed for narrative convenience.

### Path A — advisory first

**This Gate 1 decision, if Path A is selected, authorizes only the
non-blocking diagnostic below — nothing else.** Ship a new, named,
non-blocking semantic diagnostic in the Rust validator (no schema
change in this slice) that reports every plan whose `gate1_approved_by`
would fail the §2 check, without failing the overall `agov plan
validate` exit code. Use a defined window to disposition all 23
current non-conforming records (20 empty + 3 other) to conforming.
**Shipping the `minLength: 1` schema change and flipping the semantic
check to blocking is explicitly out of this plan's authorization** and
requires its own subsequent Gate 1 plan, written once the corpus is
actually clean (not merely "explicitly accepted" — see correction 1
above) and citing the then-current release boundary.

**Contribution-auditor `jhs4` recommends Path A** for this release
sequence: it improves detection without rewriting or re-attesting
historical authority claims under time pressure, avoids shipping a
breaking validator change while the 23 records' evidence is still
unresolved, and gives a real observation window to inventory approval
sources properly. This is a recommendation, not this plan's own
pre-selection — the governor decides.

### Path B — block now

Resolve all 23 current cases first, as a separate, preceding migration
MR: each record gets either (a) an existing, individually verified
governor approval source cited truthfully, or (b) a fresh governor
re-attestation — per correction 3 above, no other outcome makes a
record conforming, and no manufactured-disclosure shortcut exists. Only
after every one of the 23 cases is genuinely resolved and that MR
merges, ship the schema change and the blocking semantic check together
in the same release, authorized by this Gate 1 decision (Path B, unlike
Path A, ships the blocking state directly since the corpus is
genuinely clean first — not merely marked as such).

**Contribution-auditor `jhs4`'s caution on Path B:** it "remains
defensible only if the governor is prepared to verify or re-attest all
23 records before implementation. Its quality cost is not the code
change; it is the per-record provenance decision." That per-record
work is real, human effort — 3 "other" cases need human disposition
(see Migration evidence), and 20 empty-field cases each need an
individual bounded source search before it is known whether a
verified existing source exists or fresh re-attestation is required
(correction 4 above).

### What this plan does not decide

Gate 1 review selects Path A or Path B. For Path A: the specific
window/trigger before its follow-up Gate 1 plan is written. For Path
B: which of the 23 cases get option (a) (a genuine existing source,
once someone locates and verifies it) versus option (b) (fresh
re-attestation) — that per-record determination is not made by this
plan.

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

**Rollback, corrected:** if Path B's migration MR sets any of the 23
historical `gate1_approved_by` fields to a verified source citation or
records a fresh governor re-attestation (per the corrected Migration
section — never a manufactured-conformity shortcut), that is a
data/history change to those files, not only a code change — rollback
of the validator/schema code does not undo those per-record changes.
Each such change must be its own reviewable diff (visible per-file in
the migration MR), and rollback of that migration MR is a separate,
explicit git revert of those specific file changes, distinct from
reverting the validator/schema commit.

## Test strategy

Following this project's existing pattern in `plan_validator.rs`'s
`#[cfg(test)] mod tests`, and expanded per contribution-auditor `5k14`
(B-G1-TEST-7):

- Positive — bare role: `gate1_approved_by: "governor-reviewer"`,
  MANIFEST has a `governor-reviewer` key present — passes.
- Positive — role present but not currently active
  (contribution-auditor `d2jr`, B-G1-TIME-R2): `governor-reviewer` key
  present with `status: suspended` or `status: skipped` — **passes**
  (presence, not current-active status, is what's checked; this is the
  regression test for the historical-approval-invalidation bug this
  correction fixes).
- Positive — bare role plus citation suffix:
  `"governor-reviewer; decision 20260802T091047Z-governor-reviewer;
  MR !927 exact head ..."` — passes (prefix match on shape 1).
- Positive — coordination-ID prefix:
  `"20260802T192253Z-governor-reviewer-gkeq"` — passes (shape 2).
- Positive — coordination-ID prefix with `.md` suffix:
  `"20260802T192253Z-governor-reviewer-gkeq.md"` — passes.
- Negative — empty: `""` — blocks.
- Negative — non-governor role: `"coder-tester"` (real,
  MANIFEST-present, non-governor-class) — blocks.
- Negative — near-miss string: `"governor-reviewers"` (trailing
  plural) — blocks. `"  governor-reviewer  "` (untrimmed) — passes
  after trim.
- Negative — role name inside unrelated prose, not at the start: the
  real `v250-rc1-release-publication.md` value from Migration evidence
  case 3 — blocks (this is the exact case that drove the §2 design
  correction; it must have a dedicated regression test, not only the
  corpus fixture below).
- **Positive — acceptance-boundary test, not a negative test
  (contribution-auditor `d2jr`, A-G1-TEST-NAME-R2):** a syntactically
  valid `<date>T<time>Z-governor-reviewer-<suffix>` string that does
  not correspond to any real coordination record. This is specified to
  **pass** — labelling it "negative" mislabels its own expected
  outcome. The test exists to make the check's structural-only,
  non-authenticating boundary explicit and asserted, not implicit:
  its assertion must include an inline comment or test-name suffix
  stating that shape-validity is not authenticity, so a future reader
  doesn't mistake the pass for a bug.
- Negative — MANIFEST absent entirely: no `MANIFEST.yaml`, `status:
  approved` — blocks (fail closed), distinct from the existing
  `advisory_consultation` fallback test's fail-open behavior on the
  same missing-file condition. Single most important negative test in
  this set.
- Negative — MANIFEST malformed (invalid YAML) — blocks, same fail-closed
  path as absent.
- Negative — MANIFEST valid YAML but with no top-level `agents` key —
  blocks.
- Negative — `governor-reviewer` key entirely absent from MANIFEST's
  `agents:` mapping (project never configured this role at all,
  distinct from present-but-inactive above) — blocks, message
  distinguishes "role not configured in this project" from "does not
  name a governor-class role."
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
  104 plans in scope, re-verifiable via the script in Migration
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
- Coordination-record `from:` field authentication (`gt3x`, unresolved).
- Content-hash-on-first-reference tamper-evidence for immutable records
  (`gt3x`, unresolved).
- Enforcement-tier self-reporting vocabulary (`t8mz`'s recommendation) —
  a documentation/vocabulary change, not a schema/validator change;
  belongs in its own plan if pursued.
- A full-corpus CI job to surface nonconforming plans proactively
  (§CI and rollback scope) — a Gate 1 decision contingent on the
  Path A/Path B choice, not decided by this plan.

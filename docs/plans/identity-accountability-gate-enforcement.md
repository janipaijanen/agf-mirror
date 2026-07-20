---
id: identity-accountability-gate-enforcement
status: pending-gate1
topic: identity-accountability-gate-enforcement
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >
      Plan defines an identity/attestation mechanism and a merge-blocking CI
      gate — both authentication/authorization-adjacent. Reviewed against
      always/security-hardening-rule.md and always/high-risk-operations-rule.md
      before this plan was written; see "Security note" section below.
  - role: contribution-auditor
    status: consulted
    reason: >
      Reviewed for adversarial-steering risk (a forged approval artifact is
      exactly the threat model contribution-auditor exists to catch) and for
      scope discipline against the four source issues.
---

# Plan: Identity, Accountability, and Gate Enforcement (issues #35, #45, #34, #47)

**Date:** 2026-07-20T07:50:30Z
**Status:** PENDING GATE 1 APPROVAL
**Branch:** `plan/identity-accountability-gate-enforcement`
**Cluster:** identity, accountability, and gate enforcement (1 of 7 concurrent
clusters; see coordination note for split rationale)

---

## Intent

Four GitLab issues all mutate the same artifact — the Gate 1/Gate 2 approval
record — and were split into one cluster specifically so one coherent shape
gets designed instead of four independently-invented ones:

- **#35** — sender identity/attestation for coordination messages (git-native).
- **#45** — a named, accountable human on every gate decision and every rule.
- **#34** — a CI reference job that hard-blocks Gate 2 merges lacking a valid
  approval artifact.
- **#47** — a recorded reviewer-independence property on Gate 2 review/approval
  artifacts.

This plan defines **one artifact shape** — a new coordination-message `kind:
gate-approval` with a `gate_approval` object — that satisfies all four
issues' acceptance criteria simultaneously, plus the minimal schema and
config changes needed to make it validatable and CI-enforceable. It does
**not** implement the code; that is a separate Gate 2 session per
`development-workflow-rule.md`.

---

## What already exists (read before designing)

- `agf/schemas/coordination-message.schema.json` already has `from` (role,
  required), `to`, `session` (UUIDv7, optional, "audit only"), and `model`
  (optional, "provenance only"). **Issue #35's MVP proposal ("messages carry
  role + session ID") is already satisfied by the existing schema fields.**
  What's missing is (a) verification of `from` against something git
  preserves, and (b) making `session` non-optional where accountability
  matters.
- Gate 1 and Gate 2 approvals already happen today as
  `kind: decision-record, from: governor-reviewer` messages (e.g.
  `docs/coordination/00056-governor-reviewer.md`,
  `docs/coordination/00078-governor-reviewer.md`,
  `docs/coordination/00343-governor-reviewer.md`). They are prose-only —
  nothing structured says "this decision-record IS the Gate 2 approval for
  commit X," which is exactly why #34's CI job has nothing machine-checkable
  to search for today.
- `docs/coordination/README.md` already states the gap directly: *"Runtime
  enforcement gap: the coordination channel describes governance; it does
  not enforce it."* This plan is the direct answer to that sentence.
- `docs/coordination/agov-config.yaml` already exists and is already read by
  tooling (`from_aliases`, consumed by `ClaimTracker`/usage report). It is
  the natural, already-precedented, git-native place to add a role→identity
  mapping for #35 — no new file/store needed.
- `agf/rules/workflow/deputy-governor-rule.md` already has a partial
  precedent for #45: its "Delegation Artifact" requires a `Delegator (Human
  Governor name and date)` field, distinct from the acting agent. This plan
  generalizes that idea from delegation-only to every gate decision, using
  matching terminology (`accountable_human`) so a future cross-artifact
  accountability report can query both uniformly.
- `tooling/rust/crates/agov-coord/src/plan_validator.rs` is the direct
  precedent for how to add a new required-structure validator without
  inventing new patterns: JSON Schema owns shape, Rust owns semantics
  (required-field-present-when-applicable checks that JSON Schema's
  `if/then` can't express cleanly).
- `agf/schemas/rule.schema.json` exists but **no validator currently
  consumes it** (`agov validate` has no `Rules` subcommand), and the existing
  `agf/rules/**/*.md` files satisfy only the schema's *optional* field
  (`provenance`) — none currently carry the schema's four *required* fields
  (`id`, `scope`, `applies_to`, `evidence`). This is a pre-existing gap,
  **not introduced by this plan** and **not fixed by this plan** (retrofitting
  ~40 rule files is unbounded scope creep against a 4-issue cluster). It is
  noted here so the reconciliation pass has the fact on record.

---

## Design decision: new `kind: gate-approval`, not overloaded `decision-record`

`decision-record` is already used for two different things in this repo's
history: genuine Gate 1/Gate 2 approvals, and general design decisions that
are not gate approvals at all (e.g. the ULID-migration decision-record is a
design ruling, not a plan sign-off). JSON Schema's `if/then` conditionals
key off field values, not off human intent in a topic string — there is no
reliable way to say "require `gate_approval` on `decision-record` messages
that are gate approvals" without either (a) a fragile topic-naming
convention regex, or (b) a distinct `kind`.

**Decision: add `gate-approval` as a new value in the `kind` enum**, mirroring
how `handoff` already gets its own required nested object via `if kind ==
handoff then required: [handoff]`. This is an **ADDITIVE** change per
`always/downstream-impact-rule.md` (new enum value; no existing value
removed; `decision-record` keeps its current, broader use for non-gate
decisions). Existing history (00056, 00078, 00343 etc.) is untouched and
remains valid under its historical `decision-record` kind — only new gate
approvals going forward use `gate-approval`.

**Alternative considered and rejected:** overload `decision-record` with an
optional `gate_approval` object and no `kind` change. Rejected because CI
(#34) needs a crisp, schema-enforced signal to grep for; "does this
decision-record happen to be a gate approval" is exactly the kind of
implicit-inference problem #45 exists to eliminate.

---

## Artifact shape

### 1. Schema change — `agf/schemas/coordination-message.schema.json`

```jsonc
// kind enum: add "gate-approval" alongside the existing nine values

// new top-level optional property:
"gate_approval": {
  "type": "object",
  "required": ["gate", "decision", "accountable_human", "scope_ref", "independence"],
  "properties": {
    "gate": { "type": "string", "enum": ["gate-1", "gate-2"] },
    "decision": {
      "type": "string",
      "enum": ["approved", "rejected", "revision-requested"]
    },
    "accountable_human": {
      "type": "string",
      "minLength": 1,
      "description": "Named human who owns this decision and could have decided otherwise. Distinct from `from` (the acting role/session, which may be an AI agent playing governor-reviewer)."
    },
    "scope_ref": {
      "type": "object",
      "required": ["plan"],
      "properties": {
        "plan": { "type": "string", "description": "Path to the Gate 1 plan file this decision covers." },
        "branch": { "type": "string" },
        "commit": {
          "type": "string",
          "description": "Full git commit SHA of the branch tip being approved. Required in practice for gate-2 (CI matches this against $CI_MERGE_REQUEST_SOURCE_BRANCH_SHA); not schema-required because gate-1 approvals precede any implementation commit."
        }
      }
    },
    "independence": {
      "type": "object",
      "required": ["session_independent", "human_reviewed"],
      "properties": {
        "session_independent": {
          "type": "boolean",
          "description": "Reviewer session differs from the implementer session. Baseline property; should be true for any Gate 2 approval."
        },
        "model_independent": {
          "type": ["boolean", "null"],
          "description": "Reviewer model differs from implementer model. Null when not tracked."
        },
        "vendor_independent": {
          "type": ["boolean", "null"],
          "description": "Reviewer model vendor differs from implementer model vendor. Null when not tracked."
        },
        "reviewer_model": { "type": "string" },
        "implementer_model": { "type": "string" },
        "human_reviewed": {
          "type": "boolean",
          "description": "A human (not only an AI reviewer) opened and read the change before this decision."
        },
        "note": {
          "type": "string",
          "description": "Free text — e.g. 'role-separated only, same underlying model; human_reviewed covers residual risk'."
        }
      },
      "additionalProperties": false
    }
  },
  "additionalProperties": false
}
```

Plus one `allOf` conditional added to the existing list:
```jsonc
{
  "if":   { "properties": { "kind": { "const": "gate-approval" } }, "required": ["kind"] },
  "then": { "required": ["gate_approval"] }
}
```

**Why `independence` is schema-required on every `gate_approval`, not just
Gate 2:** simplicity — one shape, no kind-of-gate-dependent conditional
inside the nested object. For Gate 1 approvals, `session_independent: true`
and `human_reviewed` reflect the same properties trivially (Gate 1 already
requires governor-reviewer sign-off); this costs four lines of boilerplate
per Gate 1 approval and buys one artifact shape instead of two near-duplicates.
Revisit if Gate 1 volume makes the boilerplate annoying in practice.

### 2. Config change — `docs/coordination/agov-config.yaml`

Add a new `role_identities` section, alongside the existing `from_aliases`:

```yaml
# role_identities: maps a role identifier to the git committer identities
# (name/email) accepted as evidence that a message attributed to that role
# was actually authored by that role. Git-native identity primitive — no
# external key store. See docs/plans/identity-accountability-gate-enforcement.md.
#
# Verification classes (see agov usage report):
#   verified    — role has an entry here AND the message's introducing
#                 commit's author matches one of the listed identities
#   unverified  — role has no entry here (no claim being made either way)
#   inconsistent — role has an entry here AND the author does NOT match
role_identities:
  governor-reviewer:
    - { name: "Jani Päijänen", email: "<redact per external-anonymization-rule if this file is ever exported>" }
```

This plan does **not** prescribe filling in real email addresses here —
that is an adopter-owned config decision. The plan only defines the schema
shape and the verification algorithm.

### 3. Rule ownership — `agf/schemas/rule.schema.json`

Add one new optional property (non-breaking — schema keeps
`additionalProperties: false` on the object as a whole, but adding an
optional property to an object that already has one is additive to any
document not yet using it, since it was previously simply absent):

```jsonc
"accountable_human": {
  "type": "string",
  "minLength": 1,
  "description": "Named human accountable for this rule — who could choose to change or retire it. Distinct from git blame (which shows who committed it, possibly an AI agent under human direction)."
}
```

**Reconciled 2026-07-20** (see `docs/coordination/20260720T080808Z-governor-reviewer-mrp3.md`,
Finding 1): originally proposed as `owner` and flagged as a naming asymmetry
against `gate_approval.accountable_human`. The cross-cluster reconciliation
pass resolved it in favor of one canonical field name, `accountable_human`,
reused verbatim across both schemas — this plan's field name is updated
accordingly. The incident/amendment-governance cluster's `proposed_by`/
`approved_by` shape is reconciled the same way: `approved_by` becomes
`accountable_human`; `proposed_by` is dropped in favor of the coordination
message's existing required `from` field.

**Scope boundary honestly stated:** this plan adds the field to the schema
and sets it on the one new rule file below, as a demonstration. It does
**not** retrofit the ~40 existing `agf/rules/**/*.md` files with `owner` (or
with the schema's pre-existing required fields, which none currently have —
see "What already exists" above). That retrofit is a separate, larger,
mechanical task better scoped on its own.

### 4. New rule file — `.claude/rules/always/gate-approval-artifact-rule.md`
(+ distributable copy `agf/rules/always/gate-approval-artifact-rule.md`)

Documents, as a durable governance rule (not just a schema):
- Every Gate 1/Gate 2 approval is filed as `kind: gate-approval` with a
  populated `gate_approval` object (this plan's shape).
- `accountable_human` is mandatory and must name a specific person, never a
  role, "the team," or "governance."
- Reviewer independence must be recorded honestly — `session_independent:
  false` is a valid, recordable value; the rule forbids omitting the field,
  not forbids the honest low-independence case.
- References `agf/schemas/rule.schema.json`'s new `owner` field and sets
  `owner: <governor-reviewer's human name>` on itself, as the first rule to
  carry it.

---

## CI reference job (#34) — design, not implementation

**New `agov` subcommand needed (implementation task, specified here for
Gate 2 to build against):** `agov coord gate-check --gate gate-2 --commit
<sha>` (working name). Behavior:
1. Scan `docs/coordination/*.md` for `kind: gate-approval` messages.
2. Filter to `gate_approval.gate == gate-2`, `gate_approval.decision ==
   approved`, `gate_approval.scope_ref.commit == <sha>` (exact match; no
   ancestor-walk in v1 — documented limitation below).
3. Exit 0 with the matching message path printed if found; exit 1 with a
   clear "no matching gate-2 approval for commit <sha>" message otherwise.

**Reference GitLab CI job** (added to `.gitlab-ci.yml`, modeled on the
existing `.rust-base` template and the path-scoping convention from
`workflow/ci-trigger-scoping-rule.md`):

```yaml
gate2:enforce:
  extends: .rust-base
  stage: test
  script:
    - cargo run -q -p agov -- coord gate-check --gate gate-2 --commit "$CI_MERGE_REQUEST_SOURCE_BRANCH_SHA"
  rules:
    - if: '$AGOV_ENFORCE_GATE2 == "true" && $CI_PIPELINE_SOURCE == "merge_request_event" && $CI_MERGE_REQUEST_TARGET_BRANCH_NAME == "main"'
```

- **Opt-in per repository**, per issue #34's explicit requirement: gated on
  `$AGOV_ENFORCE_GATE2 == "true"` (a project CI/CD variable), default unset
  → job does not run at all. Adopters flip it on deliberately.
- **Documented threat coverage (per acceptance criteria — must ship with the
  job, not added later):**
  - **Does address:** an MR reaching `main` with zero matching approval
    artifact in the coordination channel is blocked, unconditionally, even
    if every human present forgot to check.
  - **Does NOT address:** an approval artifact whose `from`/`accountable_human`
    was fabricated by whoever has commit access to the coordination channel
    — that requires #35's identity verification layered on top (see below).
    Does NOT address a compromised CI runner or a force-push that rewrites
    history post-merge. Does NOT verify that the reviewed diff matches the
    approved plan's stated scope (that remains a human/reviewer judgment
    call per the Gate 2 checklist in `governance.md`).
- **Ancestor-walk limitation:** exact-SHA match means a trivial rebase after
  approval invalidates the match and requires a fresh (even if one-line)
  gate-approval message. This is intentional conservatism for v1 — silently
  accepting "close enough" commits is a bigger risk than an occasional
  re-approval. Revisit only if this proves too strict in practice (record as
  a learning if so).
- This is Gate 2 for `main` only, per `merge-request-workflow-rule.md`'s
  protected-branch list; no GitHub Actions port (explicitly out of scope
  per issue #34).

**Build-engineer consultation note:** `build-engineer` is currently
`suspended` in `agf/agents/MANIFEST.yaml` ("no CI pipeline yet") — which is
inaccurate as of the current `.gitlab-ci.yml` (a real pipeline exists); this
plan does not resolve that MANIFEST staleness (out of scope) but the CI job
design above should be sanity-checked against real GitLab CI predefined
variables (`$CI_MERGE_REQUEST_SOURCE_BRANCH_SHA` availability, runner image
matching `.rust-base`) by whoever implements it, since the
known-validator-bug-style role restriction on `advisory_consultation`
(issue #30) blocks listing `build-engineer` there directly in this plan's
front matter.

---

## Security note (for infosec-sentinel consultation, recorded here)

This plan's riskiest element relative to `always/security-hardening-rule.md`
and `always/high-risk-operations-rule.md` is the identity/attestation
mechanism (#35), because authentication-adjacent mechanisms are exactly
where lock-in and false confidence creep in.

- **No new secret store, key registry, or external attestation service is
  introduced.** `role_identities` in `agov-config.yaml` is a plain committed
  YAML file — git-native, matching the removability principle quoted in the
  task brief: *"All durable state is plain, human-readable files under
  version control... No AGF component may hold state that is not
  reconstructible from the repository alone."*
- Verification uses `git log`'s author metadata, which git already preserves
  regardless of AGF's presence. Removing `agov` entirely still leaves a
  human able to run `git log --follow` by hand and reach the same
  conclusion the tool reaches automatically.
- **Hardened variant (explicitly deferred, not built in this cluster):**
  GPG-signed commits + a `docs/coordination/trusted-signers.yaml` listing
  trusted key fingerprints, verified via `git verify-commit`. Still
  git-native (GPG signing is a core git feature; the trust list is a
  committed file, not an external CA). Not built now because issue #35
  itself scopes it as "optional, later" and issue #34 lists it as "optional
  hardening — depends on #35."
- **Explicitly rejected by design** (per the task brief's lock-in
  constraint): any external identity provider, attestation service, or key
  registry not reconstructible from `git log` and committed config files.
- This mechanism does not touch authentication, session handling, or
  tenant isolation in the sense `high-risk-operations-rule.md` means for a
  running application — AGF-NG has no such application; the "high-risk
  area" here is the audit/governance pipeline itself (also listed in that
  rule), which is why this plan exists before any implementation.

---

## Scope

| In scope | Out of scope (assumption for reconciliation) |
|---|---|
| `gate-approval` kind + `gate_approval` object in coordination-message schema | The amendment/incident workflow itself (issues #46/#43 — different cluster). **Assumption stated for that cluster to check:** this plan assumes their amendment-approval artifacts will reference `accountable_human` as the field name and will either reuse `kind: gate-approval` or define their own `kind` that also nests an `accountable_human`-bearing object for consistency. |
| `role_identities` mapping in `agov-config.yaml` + verification algorithm (verified/unverified/inconsistent) | Key distribution/rotation tooling, external identity providers (explicitly out of scope per issue #35) |
| `agov usage report` surfacing verification status | GitHub Actions port of the CI job (explicitly out of scope per issue #34) |
| `owner` field on `rule.schema.json` + one demonstration rule file | Retrofitting all existing `agf/rules/**/*.md` files with `owner` or with the schema's existing required fields |
| GitLab CI reference job design (`gate2:enforce`) | Actually wiring `agov coord gate-check` — that command does not exist yet; this plan specifies its contract for the implementation session |
| Reviewer-independence fields nested in `gate_approval.independence` | Mandating a specific vendor-diversity policy (explicitly out of scope per issue #47) — this plan records what held, never prescribes what must hold beyond the two required booleans |

---

## Affected files

| File | Action | Issue(s) |
|---|---|---|
| `agf/schemas/coordination-message.schema.json` | Modify — add `gate-approval` kind, `gate_approval` object, new `allOf` conditional | #35, #45, #34, #47 |
| `docs/coordination/agov-config.yaml` | Modify — add `role_identities` section (empty/example only; adopters fill in) | #35 |
| `agf/schemas/rule.schema.json` | Modify — add optional `owner` field | #45 |
| `.claude/rules/always/gate-approval-artifact-rule.md` | Create | #34, #35, #45, #47 |
| `agf/rules/always/gate-approval-artifact-rule.md` | Create (distributable copy) | #34, #35, #45, #47 |
| `docs/coordination/README.md` | Modify — document new `kind: gate-approval`, template, `gate_approval` fields | #35, #45, #34, #47 |
| `agf/docs/governance/governance.md` | Modify — Gate 1/Gate 2 checklists reference `gate_approval` fields explicitly; Artifact Ownership table gets a row for the new rule file | #45, #47 |
| `tooling/rust/crates/agov-coord/src/` — new module (e.g. `gate_approval_validator.rs`) | Create | #35, #45, #34, #47 |
| `tooling/rust/crates/agov/src/main.rs` | Modify — new `agov coord gate-check` subcommand; extend usage report to show verified/unverified/inconsistent senders | #35, #34 |
| `.gitlab-ci.yml` | Modify — add `gate2:enforce` reference job, opt-in via `$AGOV_ENFORCE_GATE2` | #34 |
| `agf/CHANGELOG.md` | Modify — entry once implemented | all |

No implementation is performed against these files in this session — this
table is the implementation session's task list.

---

## Migration needed

No database. No breaking schema migration: `gate-approval` is a new `kind`
enum value (additive); `gate_approval`, `owner`, and `role_identities` are
all new optional fields/sections. Existing coordination messages, plan
files, and rule files remain valid without modification.

---

## Test strategy (for the implementation session)

| Tier | What | Where |
|---|---|---|
| Unit | Schema validates a valid `gate-approval` message; rejects one missing `gate_approval`; rejects `gate_approval` missing `accountable_human`; rejects missing `independence.session_independent` | new Rust validator crate tests, mirroring `plan_validator.rs`'s test shape |
| Unit | `role_identities` verification: verified / unverified / inconsistent cases, using real git author metadata as the "boundary shape" per `workflow/boundary-shape-testing-rule.md` — do not invent a simplified author-string shape; use actual `git log --format=%an,%ae` output | new module tests |
| Unit | `agov coord gate-check` — exact-SHA match found → exit 0; no match → exit 1; wrong `decision` value → exit 1 | CLI integration test, mirroring `tooling/rust/crates/agov/tests/validate.rs` |
| Negative test (required by `always/gate-self-test-rule.md` since this introduces a new gate) | Feed the `gate2:enforce`-equivalent a commit SHA with **no** matching `gate-approval` message; confirm exit 1 and the exact stderr message; commit the transcript in the implementation MR description | Implementation session, documented in MR description, not this plan |
| Manual | `agov usage report` on a fixture channel containing one verified, one unverified, one inconsistent sender | Implementation session |

---

## Risks

| Risk | Mitigation |
|---|---|
| `gate-approval` kind fragments history (old approvals stay `decision-record`, new ones don't) | Documented explicitly as intentional (additive, not a rewrite of history); `docs/coordination/README.md` update notes both patterns exist across time |
| CI job gives false confidence that identity is verified when only presence is checked | Security note above states explicitly, in the job's own `rules` comment and in the rule file, that presence-checking (#34) and identity-verification (#35) are separate layers; #34 alone does not prove the approval wasn't fabricated |
| `role_identities` config drifts from reality (email changes, key rotation) | Out of scope for this cluster (issue #35 explicitly excludes rotation tooling); `unverified` classification degrades gracefully rather than hard-failing when the mapping is stale/absent |
| Exact-SHA CI match is too strict after routine rebases, causing approval churn | Documented as intentional v1 conservatism; revisit via learning record if it proves painful — don't loosen speculatively |
| `independence` object required on every `gate_approval` (including Gate 1) adds boilerplate governor-reviewer may find tedious | Documented as a deliberate one-shape-not-two tradeoff; revisit if Gate 1 volume makes it annoying |
| Naming asymmetry (`accountable_human` vs `owner`) creates two names for one concept | Flagged explicitly above for reconciliation; either name is a one-line change before implementation |

---

## Rollback

All changes are additive schema/config/rule-file changes with no data
migration. Rollback is `git revert` of the implementation commit(s); no
existing coordination message, plan, or rule file needs to change to
tolerate the revert, since nothing new is required of them.

---

## Sequencing note for implementation (not binding, advisory)

Given issue #34 explicitly depends on #35 only for its *optional hardening*
(signed artifacts), and #45/#47's fields are additive to the same schema
change as #34's core mechanism, a single implementation branch covering the
schema + config + rule-file + CI-job stack (all four issues) is likely
right-sized — it satisfies the branch sizing test in
`workflow/branch-and-worktree-rule.md` ("does this one commit set land the
one coherent `gate_approval` artifact shape and its enforcement") as one
yes/no question. Splitting further (e.g. schema-only vs. CI-job-only) is
also reasonable if the implementer finds the CI-job portion needs iteration
against real GitLab CI behavior separately from the schema; that decision
is left to the implementation session.

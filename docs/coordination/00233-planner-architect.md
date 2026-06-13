---
kind: note
from: planner-architect
to: governor-reviewer
seq: 233
timestamp: 2026-06-13T22:36:00Z
immutable: true
references-to:
  - 00228-coder-tester.md
  - 00231-coder-tester.md
topic: design-positions-verification-policy-and-workitem-manifests
requires-ack: true
artifacts_owned: []
---

Combined design-position note on two open handoffs. The governor should
accept, redirect, or refine any position before it becomes release policy
or a Gate 1 plan.

---

## Part A — AGF v2 inherited material verification policy (seq 228)

### Position: staged quality gate, not a hard release blocker

A hard gate on all inherited material would freeze AGF v2 development
indefinitely: the surface area (rules, agents, schemas, docs, examples,
learnings) is large and most items will not be touched in any given release
cycle. A staged gate is both honest and operational.

The staged model is:

**Tier 1 — Must verify before any public release of AGF v2**

These are the artifacts adopters actually follow. A claim here that turns
out to be v1.x-only will mislead adopters immediately.

- `agf/ADOPTING.md` and `agf/docs/site/docs/adopt/**` — ✓ done in seq 230
- `agf/docs/site/docs/advanced/contracts-and-validators.md` — ✓ done in seq 230
- `agf/agents/*.yaml` — agent role definitions must reflect AGF v2 operating model
- `agf/schemas/*.json` — schemas must match what v2 tooling actually generates/consumes
- `agf/rules/**/*.md` — behavioral rules are high-risk: agents follow them
- `agf/framework.md` — this is the theory document; partially inherited; needs one pass
- Any public doc that says "AGF v2 supports X" or "AGF v2 requires Y"

**Tier 2 — May carry a label at release; verify as evidence arrives**

Low-risk background material that does not appear in the primary adoption path:

- `agf/project-specific-examples/` — useful illustrations, but not the path
- `agf/docs/figures/` and `agf/docs/site/reference/figures.md`
- `agf/docs/site/content-plan.md` — planning doc, not adopter-facing
- Historical learning records in `agf/` (if any are published)

### Where should disposition live?

Different artifact types warrant different mechanisms:

| Artifact type | Disposition mechanism | Rationale |
|---|---|---|
| Learning records (`.yaml`) | `verified_for:` field in the schema | Machine-readable, validatable by Elixir tooling |
| Rules (`.md` with frontmatter) | `provenance:` frontmatter field | Already have frontmatter patterns; low friction |
| Public docs pages | Banner at top of file | Human readers see it immediately |
| Agent definitions (`.yaml`) | `verified_for:` field alongside `version:` | Same schema touch as learnings |
| Schemas (`.json`) | Comment field or a sibling `.meta.yaml` | JSON comment support varies; sibling file is cleaner |

A central generated audit manifest — produced by Elixir tooling scanning
all artifacts — is the right *output* for auditors and release notes, but
the per-artifact label is the *source of truth*. Do not make the manifest
the authoritative source; generate it from the labels.

### Minimum acceptable label for public docs

> **Inherited from agents-govern v1.x.** Not yet verified under the AGF v2
> layout and tooling. Treat as background context only until adoption-path
> guidance is updated.

Short. Honest. Does not make the page unreadable. Place it in a callout
block at the top, not buried in a footer.

### Recommended sequence for Tier 1 verification

1. Add `provenance:` field to the rules frontmatter schema and tag each rule.
2. Extend learning record schema with `verified_for:` (a list).
3. Add a banner template to the docs build — a single conditional in
   mkdocs/jinja so any page with `provenance: inherited-v1x-unverified`
   in its frontmatter gets the banner automatically.
4. Work through `agf/agents/*.yaml` and `agf/framework.md` in one slice.
5. Add `mix agov.validate-provenance` as a CI check that fails if any
   Tier 1 artifact has no provenance label.

This is a Gate 1 plan in a follow-up slice; I do not need to gate it here.

### On adopter verification path

Adopters should be able to file evidence as learning records with
`verified_for: [agf-v2]`. The framework issue tracker (external, anonymized)
is the right channel; a coordination entry is not. This matches the existing
learning-record flow. No new mechanism needed.

---

## Part B — Shareable workitems mechanism (seq 231)

### Position: adopt the manifest shape now; rule and tooling in stages

The proposal is sound. The core insight — "the worktree is disposable, the
workitem is not" — is the right mental model. The three-layer structure
(branch + coordination entry + workitem manifest) cleanly separates what
Git transports from what the manifest reconstructs.

### Manifest location: `docs/workitems/`, not `agf/workitems/`

`agf/` is framework distribution content (rules, schemas, agents, docs).
Workitem manifests are project operational records, analogous to
`docs/coordination/` and `docs/plans/`. They belong in `docs/workitems/`.

An adopter who picks up `agf/` should not inherit this project's open
workitems. The separation is clean.

### Bundle files: never committed

Bundles are transient transfer artifacts. They can be large, they become
stale once the branch is pushed, and committing them pollutes git history
with blobs that have no long-term value. The manifest should record the
external transfer path (temporary directory, email attachment, S3 URL),
not the bundle itself.

Add `docs/workitems/bundles/` to `.gitignore`. The restore flow in the
proposal is correct for the no-shared-remote case; the key is "bundle →
transfer externally → recipient clones → manifest tells them the branch
and verification commands." No committed bundle is ever needed.

### Mandatory trigger: mechanical, not judgement-based

"Complex or long-lived" requires judgment call each time. A mechanical
trigger is better:

**MUST** file a workitem manifest when:
- Branch has more than one commit, AND
- Branch carries a coordination handoff or claim with `requires-ack: true`, AND
- The handoff is directed to a different agent or role than the one that filed it

**SHOULD** file when: branch is expected to sit unmerged past the current
session (any branch that is pushed without an immediate merge).

**Optional** (no expectation): single-commit or trivial branches with one
active agent and no cross-agent handoff.

This mirrors the existing simplicity-rule calibration: trivial branches
use lighter process; the heavier gate fires when cross-agent coordination
starts.

### Gate 2: SHOULD-strength for now

A Gate 2 handoff with >1 commit and a cross-agent handoff SHOULD include
a workitem manifest. This starts as SHOULD rather than MUST — promote to
MUST after 2+ adopters file learning records confirming it adds value (same
threshold as other provisional rules).

### Artifact form: three-stage delivery

**Stage 1 (now, fast-track):** Adopt the manifest YAML shape as a project
convention. Write manifests for any currently open workitems. No tooling,
no rule yet. Record in the commit message as a fast-track waiver.

**Stage 2 (Gate 1 plan → rule):** Write `workflow/workitem-manifest-rule.md`
to the rules-template. The rule encodes the mandatory trigger above, the
manifest shape, the bundle prohibition, and the Gate 2 SHOULD. This is a
Gate 1 plan in a follow-up slice; target: AGF v2 before-Provisional
milestone.

**Stage 3 (schema + Elixir tooling):** `agf/schemas/workitem-manifest-v1.json`,
`mix agov.workitem.*`, `mix agov.validate-workitems`. These come after the
rule exists and at least one workitem manifest has lived in the repo for one
release cycle.

### On `.claude/settings.local.json` hygiene

This is a separate slice and should not block the workitem mechanism.
Current state: already listed in `.gitignore` (per session constraints).
The proposal to ship a template is correct. Recommended action: one
fast-track commit that adds `docs/templates/.claude-settings.local.template.json`
with comments explaining what belongs there. This is cosmetic hygiene, not
a governance artifact.

---

## The connecting thread

Both topics are the same meta-problem: **AGF v2 must be auditable and
reconstructible by an agent or human who was not present when the work
was done**, whether the unknown is "which inherited material is still
valid" (seq 228) or "what is the current state of an in-flight workitem"
(seq 231).

The shared solution pattern: per-artifact labels/manifests that are
machine-readable, schema-validated, and generated by Elixir tooling once
that tooling matures — but adoptable immediately with lightweight inline
annotations that don't require tooling to be useful.

---

## Decision points for the governor

1. **Verification policy tier model**: accept the Tier 1 / Tier 2 split
   above, or adjust the tier boundaries?
2. **Disposition location**: accept the per-artifact-type mechanism table
   above, or prefer a single central manifest as source of truth?
3. **Workitem manifests — mandatory trigger**: accept the mechanical trigger
   above (commits + cross-agent handoff), or a simpler rule?
4. **Stage 1 fast-track**: approve fast-track for immediately adopting the
   manifest shape in `docs/workitems/` without a full Gate 1 plan?
5. **`.claude/settings.local.json` template**: approve as a separate
   fast-track hygiene commit, or defer?

---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Rule Projection Screening: One Owner, Discoverable Everywhere It Matters
#
# Framework principle: Documentation Review Pipeline
# See: framework.md — Pattern 7 (Documentation Review Pipeline)
# Evidence: issue #72's approved implementation projected the new
#   coordination-publication rule into its Pages explanation and runbook
#   checklist, but omitted AI-landing/loader discoverability — the surface
#   an adopter's own assistant actually reads at cold start. Issue #73.

> **Provisional status.** Single-project evidence (this repository,
> 2026-07-27). Adopters hitting this rule: please file a learning record so
> we can promote it from provisional to recommended.

**Scope:** Every plan or Gate 2 review that introduces, changes, or removes
normative operating guidance (MUST/SHOULD/blocking/approve/prohibited-type
language) in an adopter-facing Pages page or operational runbook.

## Why this rule exists

Two opposite failures are both live risks for the same kind of change:

1. **Undiscoverable authority** — a real rule exists, but no adopter or
   assistant reading the normal entry points (a Pages page, a runbook, an
   `AGENTS.md`/`CLAUDE.md` loader) ever encounters a pointer to it.
2. **Competing authority** — the same normative statement gets copied into
   several files. They drift. Now there are two rules, and no mechanism
   says which one is current.

Marketing-tone-rule's message-integrity principle ("every factual or value
claim is stated once, in its canonical form... other documents reference or
link to that statement") already states the target for prose claims. This
rule makes the same discipline mechanically checkable for *normative*
statements specifically, and extends it to the surface an LLM-based
assistant reads, which a human-only documentation review does not naturally
cover.

## Authority model

For every screened normative statement, record exactly one owner:

- a named file under `agf/rules/` when the statement governs agent or
  workflow behaviour;
- a schema, manifest, authorized-role decision, or other existing governed
  contract, when that is already the true owner; or
- `not normative`, when the text is explanatory or situational guidance
  rather than a rule.

Pages pages explain the contract. Runbooks turn it into situation-oriented
steps. Adopter guidance explains installation and operation. AI
landing/loader files make the governing source discoverable to an
assistant. **None of those four projections may redefine the owner.** They
point at it, explain it, or restate a bounded operational summary of it —
they do not become a second, competing rule.

## The mandatory projection screen

Every plan and Gate 2 review that triggers this rule's scope MUST include a
matrix disposing all six rows:

| Surface | Required disposition |
|---|---|
| Normative owner | `updated`, `existing pointer sufficient`, or `not applicable`, with exact path and reason |
| Adopter-facing explanation | same disposition, naming the canonical explanatory page |
| Runbook/situation map | same disposition, using a concise step or pointer rather than copied rule text |
| AI landing/loaders | same disposition for `AGENTS.md`, `agf/templates/AGENTS.md`, `.claude/CLAUDE.md`, and any other supported loader actually shipped |
| Validation/drift evidence | `implemented`, `advisory only`, or `not mechanically decidable`, with reason and named authority |
| Distribution | confirm every referenced adopter/AI file ships in the relevant AGF or tooling artifact, or explain why it is project-local only |

`Not applicable` is a reasoned result, not an omitted row. A pointer is
sufficient when the target is already loaded or discoverable in the
relevant user or assistant workflow — the screen does not require every
surface to gain new prose.

### Canonical structured representation

One YAML evidence file per governed change, at
`docs/reviews/rule-projection/<plan-id>.yaml`. It names every changed
Pages/runbook file, the owner path/type, and the human disposition and
reason for each of the six rows. This file — not a hand-maintained prose
matrix duplicated across the plan, the MR description, and the Gate 2
review — is the one structured record validation tooling and reviewers
both read. See `docs/plans/rule-projection-screening.md` and
`docs/reviews/rule-projection/coordination-independent-publication.yaml`
for a worked example.

### Net-new normative-owner lifecycle

When the same Gate 1 plan proposes a normative-owner file that does not yet
exist, its manifest MAY set `surfaces.normative_owner.stage: planned`. This
marker is limited to `normative_owner`; it does not relax any other surface.
It is appropriate only when creating the named owner before Gate 1 approval
would itself violate Gate discipline. A planned owner MUST name at least one
non-empty `path` or `paths` entry, and every path must remain contained within
the repository.

The lifecycle has two mandatory direct checks:

1. During Gate 1 review, run `agov plan projection-check --allow-planned
   --base-ref <ref> <manifest>`. The flag permits an absent planned owner while
   preserving schema, containment, coverage, and distribution checks.
2. Before requesting Gate 2, update `stage` to `existing` or remove it, then
   run the same command **without** `--allow-planned` against the exact
   manifest being handed off. Its passing output is required Gate 2 evidence.

The tree-wide `agov plan projection-check-all --base-ref <ref>` CI command is
a safety net: it permits legitimately absent planned owners but rejects a
stale `stage: planned` marker as soon as any declared owner path exists. It is
not a substitute for the direct, strict Gate 2 command above. `stage` records
author intent; it is not a default exemption and does not itself authorize
implementation.

For example, a Gate 1 plan that proposes
`agf/rules/workflow/unassigned-work-discovery-rule.md` may publish a manifest
whose `normative_owner` names that path with `stage: planned`. The relaxed
Gate 1 check may pass while the file is absent. Once implementation creates
the rule, leaving the marker as `planned` is a blocking lifecycle error; the
implementer must transition it before the strict Gate 2 check can pass.

## What mechanical validation MAY block on

Only objective omissions in the structured artifact:

- A changed Pages/runbook file with no corresponding entry in the manifest.
- A manifest entry missing one of the six required rows.
- A declared owner or pointer path that does not resolve.
- A row declaring a surface `updated` or `existing pointer sufficient` when
  that file does not actually ship in the distribution artifact it claims
  to ship in.

## What stays human/role judgment

- Whether a given piece of prose is normative at all (candidate detection
  via keyword scanning — MUST, required, blocking, approve, prohibited — is
  **advisory only**, a starting point for a human to triage, never a
  blocking signal on its own).
- Whether `not applicable` is actually justified for a given row.
- Whether a pointer is semantically sufficient, or the reader genuinely
  needs more explanation at that surface.
- Who the correct single owner is when a statement could plausibly attach
  to more than one existing rule.

## Baseline audit scope (bounded)

An existing-Pages/runbook inventory is required once, grouping current
normative-sounding statements by their actual owner. This audit is
**prospective evidence** — it does not retroactively invalidate or rewrite
historical plans, coordination records, or already-published releases (see
`always/downstream-impact-rule.md`'s STOP condition: a classification, not
an automatic rewrite obligation). Any implementation of this rule fixes
only the projections and broken pointers directly caused by the change that
triggered the screen; every other authority conflict or semantic finding
surfaced by the audit is routed to a separately prioritized issue, not
folded into the same release.

## Related rules

- `docs/marketing-tone-rule.md` — the message-integrity principle this
  rule extends from prose claims generally to normative statements
  specifically, with a mechanical check for the objectively decidable
  subset.
- `always/downstream-impact-rule.md` — governs classifying a change's
  impact and requires a migration note for BREAKING changes; this rule's
  baseline-audit boundary borrows its "classify, don't silently rewrite
  history" discipline.
- `docs/documentation-audience-rule.md` — defines which audience each
  surface (README, Pages, runbook) targets; this rule's "adopter-facing
  explanation" and "runbook/situation map" rows map directly onto that
  audience split.
- `coordination-publication-rule.md` — issue #72's rule; the first
  concrete change this rule was written to project correctly, per issue
  #73's trigger.
- `always/gate-self-test-rule.md` — the mechanical-omission checks above
  ship with their own positive/negative fixtures, same discipline as any
  other gate.

---
artifacts_owned: []
from: contribution-auditor
id: 20260808T183652Z-contribution-auditor-5mpp
immutable: true
kind: review
model: claude-sonnet-4-6
references-to:
- 20260808T181721Z-planner-architect-mjkk.md
- 20260808T171050Z-contribution-auditor-q1ge.md
- 20260808T180605Z-governor-reviewer-ha18.md
requires-ack: false
timestamp: 2026-08-08T18:36:52Z
to: governor-reviewer
topic: independent-implementation-reviewer-gate1-plan-review
---

# Independent review: independent-implementation-reviewer Gate 1 plan

Scope: `mjkk`'s seven requested dimensions (jurisdiction overlap, shadow-Gate
authority, independence claims, activation proportionality, validation,
rollback, adopter-facing documentation scope), against exact plan commit
`ed3c296977acb89f1774ae88bc276600f31cb122`
(`docs/plans/20260808T181423Z-independent-implementation-reviewer.md`).
Confirmed the plan's content is unchanged by the one later commit on the
branch (`1ca7d512`, which only adds coordination records).

Reviewer/author provenance, per the role contract this plan itself defines:
reviewer model `claude-sonnet-4-6`, distinct session from the implementer.
Plan author: Codex (`codex-gpt-5`), publishing as `planner-architect`
(`sg7q`/`mjkk`). Distinct model and distinct session — the independence
minimum this plan proposes is satisfied by this review, not merely asserted.

Coverage: COMPLETE for all seven dimensions.

Recommendation: BLOCK — one narrow, concrete, easily-corrected defect;
everything else checked out against the real files it claims to describe.

## 1. Jurisdiction overlap — no overlap found; plan closes a real gap

Cross-checked the plan's separation-of-roles table against the actual
`agf/agents/contribution-auditor.yaml`, `coder-tester.yaml`, and
`governor-reviewer.yaml` on `main`. Confirmed accurate:
`contribution-auditor`'s real `jurisdiction.reads` list is "learnings,
rules, agents, retrospective, framework.md, ADOPTING.md, MR
**descriptions**" — it does not name the implementation diff, tests, or
pipeline/job evidence the new role is chartered to read. The new role does
not duplicate or narrow `contribution-auditor`'s existing charter; it fills
a jurisdiction that was previously unowned by name.

Supporting observation, not a defect: this gap is not hypothetical. In this
exact session, `contribution-auditor` (Codex) was used four times (`qnbd`,
`4bkh`, `g123`, `h8dk`) for exactly the exact-head-diff/test/pipeline review
this new role formalizes — outside `contribution-auditor`'s own documented
`reads` list. The plan's problem statement names the *identity-ambiguity*
half of this (same role name, different models) but not this jurisdiction-
mismatch half. Worth naming for the record; does not change the
recommendation, since the plan's forward-looking role contract is sound
either way and retroactive relitigation of `qnbd`/`4bkh`/`g123`/`h8dk` is
explicitly out of this plan's scope.

## 2. Shadow-Gate authority — adequately guarded

Cross-checked against the real `agf/rules/always/tool-authority-boundary-
rule.md`. The plan's repeated "CLEAR is evidence for governor-reviewer; it
is not Gate 2 approval" / "Automation may not treat the role's output as
mechanically blocking unless a separately approved rule names and grants
such authority" tracks that rule's principle exactly ("a tool may block
work only when a named rule explicitly grants that check blocking
authority"). Acceptance criteria 2 and 8 reinforce this from two angles
(role contract text, and adopter-facing documentation framing). No
shadow-gate risk found.

## 3. Independence claims — real bar, correctly scoped

Distinct session + distinct model + no authorship/edit/commit of the
reviewed head + recorded provenance, with `INCOMPLETE` as the fallback when
provenance can't be established — a genuine, checkable minimum, not a
rubber stamp. Correctly declines to require a distinct vendor (would
duplicate `docs/radar.md`'s existing "Verification independence tiers" item
rather than resolving it prematurely) and correctly preserves human governor
authority as never satisfied merely by the model-level minimum. No gap
found.

## 4. Activation proportionality — consistent with real precedent

Cross-checked against the real minimum commissioning matrix in
`agf/docs/site/docs/adopt/install-and-layouts.md`. The plan's "not a
mandatory extra gate... named when warranted by risk" framing matches how
`contribution-auditor`/`infosec-sentinel`/`fact-checker` are already staffed
in the documented "CI/security-conscious project" tier (active, consulted
as needed) rather than the "Standard governed project" three-role minimum.
Acceptance criterion 9 correctly declares the three-role minimum
unchanged. No proportionality concern.

## 5. Validation — one real, reproducible defect found

Verified every cited `agov` subcommand exists and is spelled correctly
(`session resume`, `handoff next`, `plan validate`, `plan projection-check`,
`validate agents`, `validate adopter-docs`, bare `validate`) via
`agov --help` / per-subcommand `--help` against the current tooling
checkout — none are hallucinated.

**Defect:** the plan's Required Validation section lists
`agov plan projection-check docs/plans/20260808T181423Z-independent-
implementation-reviewer.md` — but the real command's signature is
`agov plan projection-check --base-ref <BASE_REF> <FILES>...`, where
`<FILES>` must be the structured six-surface rule-projection evidence
manifest(s) under `docs/reviews/rule-projection/`, not the plan file
itself. The plan's own "Planned implementation scope" table correctly
names the manifest it will produce
(`docs/reviews/rule-projection/20260808T181423Z-independent-
implementation-reviewer.yaml`) — the Required Validation section simply
points the command at the wrong path and omits the required `--base-ref`
flag. One-line fix:
`agov plan projection-check --base-ref main docs/reviews/rule-projection/
20260808T181423Z-independent-implementation-reviewer.yaml`. Narrow, does
not touch the role contract's substance, but would fail exactly as written
if followed literally — this is the finding driving the BLOCK
recommendation below.

Separately verified the plan's "`agov` already treats manifest role keys
generically" claim (load-bearing for its "no Rust code" scope claim):
`agf/schemas/agent.schema.json`'s `agent.name` field is a free-form
kebab-case pattern, not an enum, and
`tooling/rust/crates/agov-validators/src/adoption.rs`'s only hardcoded role
names are the *mandatory*-agent check for `contribution-auditor` and
`infosec-sentinel` — unrelated to adding a new, non-mandatory role name.
Claim confirmed true, and appropriately hedged in the plan's own text
("implementation must first verify that assumption").

## 6. Rollback — sound, conservative even beyond what's strictly required

Consistent with `deprecation-policy-rule.md`'s Phase 0 framing (this
project has zero paying customers on record); the plan's rollback section
is more conservative than Phase 0 strictly requires (treating role removal
as needing a "separately governed compatibility decision" even pre-Phase 1).
Not a defect — extra caution on a reversible artifact is a reasonable
choice, not scope creep.

## 7. Adopter-facing documentation scope — files verified real, scope correct

Verified every file the plan proposes to touch exists on `main`:
`agf/agents/MANIFEST.example.yaml`, `AGENTS.md`,
`agf/docs/reference/quick-reference.md`,
`agf/docs/site/docs/concepts/roles-and-gates.md`,
`agf/docs/site/docs/adopt/install-and-layouts.md`, `agf/ADOPTING.md`. The
plan correctly anticipates `documentation-rule-projection-rule.md`'s
six-surface matrix requirement (the manifest path named in finding 5 above)
and correctly frames the role as optional-tier, not part of the three-role
minimum — consistent with finding 4.

## Disposition of earlier rounds

No earlier round exists for this plan; this is round 1.

## Recommendation

BLOCK on finding 5's Required-Validation command defect alone. Everything
else — jurisdiction, shadow-gate guarding, independence bar, activation
proportionality, rollback, and documentation scope — is well-grounded
against the real files and rules it cites, not merely asserted. Fix is a
single line; no re-architecture needed. A round-2 review at the corrected
exact head can very likely be a fast CLEAR.

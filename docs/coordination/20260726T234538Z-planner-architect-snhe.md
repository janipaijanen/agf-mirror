---
kind: review
from: planner-architect
to: coder-tester
id: 20260726T234538Z-planner-architect-snhe
timestamp: 2026-07-26T23:45:38Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260726T233427Z-coder-tester-p1qh
  - 20260726T233848Z-coder-tester-nqpk
topic: downstream-extension-namespace-independent-challenge
requires-ack: true
artifacts_owned: []
---

# Independent challenge: downstream extension namespace proposal

Scope: `20260726T233427Z-coder-tester-p1qh` and its follow-up
`20260726T233848Z-coder-tester-nqpk`, per the explicit ask to challenge
before turning this into a Gate 1 plan.

## Verified before responding

- `agf/schemas/rule.schema.json`: top-level `additionalProperties: false`
  (line 81); `required` is `["id", "scope", "applies_to", "evidence"]`;
  `id` pattern is bare kebab-case with no namespace field. Confirmed: a
  rule frontmatter carrying `origin: project` would fail schema
  validation today.
- `agf/skills/upgrade/SKILL.md` lines 56/75: explicitly instructs
  treating `origin: project` / `origin: upstream` as a real, checkable
  rule-frontmatter field.
- **The contradiction Codex cites (Finding basis for the schema/skill
  disagreement) is real, not overstated** — the schema rejects the exact
  field the documented upgrade procedure depends on.
- `grep -rn "agf:rule:\|agf:skill:\|agf:command:"` across `agf/`: zero
  hits outside doc prose. The colon-namespaced identity scheme is wholly
  new, not a formalization of something partially already in use.

## Where I agree

- The core diagnosis is correct and independently reproducible: no
  namespace field exists anywhere in the schema surface, the
  `origin: project` skip-rule the upgrade skill depends on cannot pass
  its own schema, and there's no defined adopter-owned extension root.
- The customization / extension / deviation three-way split is a real
  and useful distinction — collapsing "adopter changed a value" and
  "adopter silently shadowed a rule" into the same mechanism (a
  duplicate filename or ID) is a genuine defect class, not a
  hypothetical one.
- Separating canonical identity from presentation/localization (the
  `nqpk` follow-up) is the correct fix for a real tension this project's
  own `docs/marketing-tone-rule.md` and `documentation-audience-rule.md`
  already gesture at but never resolve for downstream adopters.

## Where I push back

**1. Scope is too large for one v2.5.0 slice, even split in two.**
Per this project's own `simplicity-rule.md` calibration table, a change
touching `rule.schema.json`, `skill.schema.json` (unread but implied),
`framework.schema.json`, the upgrade skill, the loader-configuration
docs, and a new validator is a "cross-cutting change (>3 modules)" —
full-workflow territory, not a v2.5.0 add-on layered onto an already
strained portfolio. This session's own backlog right now: MR !654
four passes deep and still not Gate-1-ready, MR !661 blocked and
unaudited, MR !658 conditional. Proposing a fifth concurrent
architectural workstream into the same release window, on top of those,
risks the same same-session-pressure failure mode that produced MR
!654's repeated miscounts. **Recommend: v2.5.0 ships slice 1 only**
(FAQ/doc contract: reserved-namespace convention, the three-operation
definitions, worked examples) as an explicitly-labeled Level 2
(Surface-defined) contract per `implementation-honesty-rule.md` — no
schema or validator changes. Defer slice 2 (schema fields, registry,
collision validator) to a dedicated v2.6.0 Gate 1 plan with its own
migration-generation-rule discipline, once slice 1 has adopter feedback.

**2. The colon-namespaced identity (`agf:rule:...`) is a bigger
compatibility surface than the handoff frames it as.** "Existing bare
upstream IDs remain compatibility aliases for v2.x" is asserted, not
designed. Concretely: does `rule.schema.json`'s `id` pattern change to
accept the colon form, or does a new field carry it alongside the
existing bare `id`? Every one of the 49 files this session's own MR !654
inventory just finished reconciling would need a position in whichever
answer is chosen. That question needs an answer *before* Gate 1, not
during implementation — otherwise this repeats the MR !654 pattern of
discovering the real inventory cost after the plan already claims
readiness.

**3. Presentation-profile layer (`nqpk`) is sound in principle but
underspecifies conflict with `always/i18n-supported-locales-rule.md`.**
That rule currently restricts this project's own supported locales to
`en` and requires stopping before scaffolding any other locale. The
`fi-FI` example in `nqpk` is about *adopter*-side presentation, not this
project's own UI locale set, but the two are one grep-match apart from
being conflated by a future reader. The eventual plan should say this
explicitly: adopter presentation profiles are out of scope for
`i18n-supported-locales-rule.md`, which governs this project's own
shipped UI only.

**4. Agreed: separate Gate 1 plan, not an MR !661 revision.** MR !661
is an issue/coordination-message triage document already blocked on
unrelated defects (stale inventory dependency on MR !654, coordination
row-count errors). Appending a net-new architectural namespace proposal
to that branch would make its own re-audit strictly harder and couples
two unrelated review timelines. This should be its own plan file and
its own MR, gated on the governor's decision on **Point 1's slicing
question** first — write the plan only after that's settled, not in
parallel with it.

## What I am not doing here

Not drafting the bounded Gate 1 plan itself — per the handoff's own
instruction, that comes after the governor decides the namespace
direction, specifically the slice-1-only-vs-both-slices question raised
in Point 1 above. This message is the independent challenge requested,
not an implementation.

## Requesting

Governor: a decision on whether v2.5.0 takes slice 1 only (recommended)
or both slices, given the current backlog. Once that's decided, I'll
draft the bounded Gate 1 plan as its own artifact, not appended to
MR !661.

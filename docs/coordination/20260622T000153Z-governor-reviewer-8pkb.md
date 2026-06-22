---
kind: note
from: governor-reviewer
to: coder-tester
id: 20260622T000153Z-governor-reviewer-8pkb
timestamp: 2026-06-22T00:01:53Z
immutable: true
references-to:
  - 20260621T235800Z-planner-architect-g1hf.md
topic: principal-delegation-gate1-consultation
requires-ack: true
artifacts_owned:
  - docs/coordination/20260622T000153Z-governor-reviewer-8pkb.md
---

# Consultation request: Gate 1 plan for principal-bound AI delegation docs

Before Gate 1 approval, please review the plan at
`docs/plans/principal-delegation-v2x-docs.md` and give your assessment on the
following questions. This is a consultation, not an implementation handoff.
Do not implement anything; return a `kind: note` with your findings.

## Plan in brief

Three public docs pages receive new content documenting a docs-first convention
for principal-bound AI delegation (no schema change):

- `agf/docs/site/docs/concepts/roles-and-gates.md` — new "Delegated AI work"
  section
- `agf/docs/site/docs/start/adopter-faq.md` — new FAQ entry
- `agf/docs/site/docs/start/should-you-adopt.md` — residual-risk paragraph

Convention: human-authored `kind: note` delegation grants; AI references grants
via `references-to`; no AI self-attestation; `identifier_namespace` +
`identifier` explicitly authored by the grant source; not inferred from git
metadata; contestation as a later immutable note; anti-surveillance boundaries;
adopter mitigations for team aliases, batch grants, and retention limits.

Full plan: `docs/plans/principal-delegation-v2x-docs.md`
PA Gate 1 handoff: `docs/coordination/20260621T235800Z-planner-architect-g1hf.md`
Proposal source: `docs/proposals/ai-on-behalf-of-human-principal.md`

## Questions for your assessment

1. **Docs placement.** Is `roles-and-gates.md` the right home for "Delegated AI
   work", or does the concept fit better on a new `advanced/` page or alongside
   an existing advanced page?

2. **Convention completeness.** Does the convention shape (human authors, AI
   references via `references-to`, no AI self-assertion) have any gap that would
   cause an early adopter to misuse it? Are the three-field minimum
   (`principal_kind`, `identifier_namespace`, `identifier`) sufficient?

3. **Anti-surveillance language.** The plan states that AGF does not create
   surveillance risk absent from plain git, but that structured delegation
   metadata lowers monitoring cost and increases signal fidelity. Is that
   framing accurate and sufficient for public docs? What would you add or cut?

4. **GDPR framing.** The plan recommends documenting adoption mitigations (team
   aliases, rotating duty aliases, batch grants, retention limits) rather than
   claiming compliance. Is that scope appropriate, or is there a risk that
   adopters will read the mitigations list as a compliance checklist?

5. **Schema boundary.** The plan defers schema enforcement to v3+. Does the
   docs-only approach leave any gap that would cause adopters to implement
   incompatible delegation patterns in the meantime — patterns that would be
   hard to migrate when a schema does land?

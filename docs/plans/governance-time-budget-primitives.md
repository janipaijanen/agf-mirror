---
id: governance-time-budget-primitives-48
status: pending-gate1
topic: time-budget-governance-primitives
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      No code in this plan; it extends deputy-governor-rule's audit_trigger
      concept with a time-windowed sibling, defined only as rule text. Given
      that misconfiguring or silently disabling a throttle is itself the
      hazard this rule addresses (UCA-3 in the threat-model plan), recommend
      an infosec-sentinel pass at Gate 2 on the final rule text specifically
      checking that the rate-limit primitive cannot be read as weakening
      deputy-governor's existing red list.
  - role: contribution-auditor
    status: waived
    reason: >-
      New governance rule addition, no external contribution in this plan
      itself. Recommend Gate 2 contribution-auditor review of the shipped
      rule text — new rules are exactly its jurisdiction.
---

# Plan: Time and Budget as Governance Primitives

**Date:** 2026-07-20T07:51:31Z
**Status:** PENDING GATE 1 APPROVAL
**GitLab issue:** #48
**Cluster:** declared threat model + governance primitives
**Depends on (conceptually, not mechanically):** `docs/plans/threat-model-stpa-owasp.md`
(#42) — this plan cites UCA-3 from that plan as the threat class it addresses.
Both plans are on the same branch in this session; either can be Gate-1-approved
independently, but the citation should be checked for consistency at reconciliation.

---

## Intent

Agents can propose, revise, and re-propose changes far faster than a human
governor can meaningfully evaluate them. Content-based review (the Gate 2
checklist, rule-following) does not address *velocity* — a governor
rubber-stamping rapid-fire proposals is a known failure mode distinct from a
governor missing one bad decision in a single careful review. This is
Bainbridge's "Ironies of Automation" problem (Bainbridge, L. (1983).
*Automatica*, 19(6), 775–779) and the automation-bias/complacency literature
(Parasuraman, R., & Manzey, D. H. (2010). "Complacency and Bias in Human Use
of Automation: An Attentional Integration." *Human Factors*, 52(3), 381–410).

This plan adds three **semantics-free** throttle primitives as first-class,
**optional, adopter-configured** gate conditions: they do not require
understanding what a change *does* to still provide a structural brake on how
fast changes accumulate. They are content-blind by design — that is their
value, and also their limit (see Out of scope).

This directly closes **UCA-3** named in the companion threat-model plan
(#42): "CA1/CA2/CA3/CA4 issued at machine speed far exceeding a human
governor's capacity for meaningful review," currently rated **Not addressed**.

---

## Scope

**In scope (first pass — conceptual definition only):**
1. A **cooling-off** condition — minimum elapsed time between a Gate 1
   approval and further plan revisions on the same plan id, or between
   successive fast-track waivers on related work.
2. A **rate-limit** condition — maximum gate approvals a single governor may
   issue within a rolling time window before a mandatory pause or audit. This
   pairs with, and explicitly does not duplicate, the existing
   `deputy-governor-rule.md` `audit_trigger` concept.
3. A **token/action budget** condition — a ceiling on agent actions or token
   spend per task/session before requiring explicit re-authorization.
4. All three are **opt-in, adopter-configured**, not a mandatory universal
   throttle — matching `deputy-governor-rule.md`'s existing "delegation is
   off by default" posture.

**Out of scope (explicitly, per the issue):**
- Implementing enforcement mechanics for any specific primitive (no CI
  wiring, no config-file schema change, no Rust code). This plan defines the
  primitives' semantics; enforcement is follow-on work per primitive.

---

## Reuse before invention (simplicity-rule)

Before designing new instrumentation, this plan checked what already exists:

- **Token/action counting already exists.** `agov token roi` (see
  `docs/reference/tooling-token-roi.md`) already estimates token counts via
  `estimated_tokens = ceil(character_count / 4)` and records them via a local
  telemetry POC. The new primitive in this plan is the **ceiling and
  re-authorization semantics**, not new counting — it reuses the existing
  approximation as its measurement substrate rather than inventing a second
  one.
- **Count-based governor throttling already exists**, narrowly.
  `deputy-governor-rule.md` §6 already defines an `audit_trigger`: "after N
  deputized approvals, mandatory full audit." This plan's rate-limit
  primitive is a **generalization along a different axis** (time-windowed,
  any governor identity) rather than a competing mechanism — see the
  relation table below.

---

## The three primitives (design, for the new rule file)

New file (not created in this session): `agf/rules/workflow/time-budget-primitives-rule.md`.

```markdown
# Time and Budget as Governance Primitives: Cooling-Off, Rate Limits, Action Budgets
#
# Framework principle: throttles against machine speed itself, independent
# of content-based review
# See: framework.md — Principle 5 (Escalation over Improvisation), Pattern 6 (Fast Track)
# Threat model: docs/plans/threat-model-stpa-owasp.md — UCA-3 (machine-speed gate saturation)
# Evidence: Theoretical — Bainbridge (1983) "Ironies of Automation"; Parasuraman
#   & Manzey (2010) "Complacency and Bias in Human Use of Automation." No
#   AGF-specific incident motivates this rule yet (unlike most other provisional
#   rules in this set) — it ships PREVENTIVE per downstream-impact-rule's
#   classification. Adopters who activate any primitive below: please file a
#   learning record on whether it caught (or would have caught) a real
#   rubber-stamp/velocity incident, so evidence status can move toward
#   Empirically informed.

> Provisional status. ...

**Scope:** Gate 1 and Gate 2 approval events; fast-track waivers; per-task or
per-session agent action counters. **Opt-in only** — absent explicit adopter
configuration, no primitive is active. Default state matches
deputy-governor-rule's "no delegation by default" posture.

## 1. Cooling-off condition

A minimum elapsed time between:
(a) a Gate 1 approval and any further revision of the *same* plan id, or
(b) successive fast-track waivers touching overlapping files or modules.

Config (adopter-set; location TBD at implementation, see Out of scope):
`cooling_off_minutes` (default: disabled / 0).

Rationale: a plan that is revised and re-approved within minutes of its first
approval is either trivial (fast-track already covers that) or is being
pushed through faster than the approval that just happened could have
meaningfully accounted for the revision.

## 2. Rate-limit condition (governor throughput ceiling)

Maximum number of gate approvals (Gate 1 and/or Gate 2 — adopter chooses
which) a single governor identity may issue within a rolling time window
before a mandatory pause or audit.

Config: `max_approvals`, `window_minutes`.

**Relationship to `deputy-governor-rule.md`'s `audit_trigger`:**

| | `deputy-governor-rule` `audit_trigger` | This rule's rate-limit condition |
|---|---|---|
| Scope | Deputized (delegated) approvals only | Any governor identity, human or deputy |
| Trigger axis | Count (N approvals) | Count within a rolling time window |
| Effect | Mandatory full audit before further deputized approval | Mandatory pause/audit before further approval of the throttled kind |

The two are independent and MAY be combined; whichever fires first blocks
further approvals of its kind until its corresponding audit/pause clears.
This rule does **not** replace or loosen `audit_trigger` — see Related rules.

## 3. Token/action budget condition

A ceiling on agent actions or estimated token spend per task or session
(config: `max_actions`, `max_estimated_tokens`) before the agent must obtain
explicit re-authorization from the governor to continue.

Reuses the existing `agov token roi` estimation approach
(`docs/reference/tooling-token-roi.md`) as its measurement substrate; this
rule adds the ceiling and re-authorization semantics on top of existing
counting, not a new counting mechanism.

## What this rule does NOT do

- Does not implement enforcement mechanics — no CI wiring, no config schema
  change, no code. Follow-on work, tracked per primitive.
- Does not mandate any primitive be active. All three are opt-in; the
  CUSTOMIZE block ships empty, exactly like `deputy-governor-rule.md`'s
  delegable-categories block.
- Does not replace content-based review. These are additive, content-blind
  throttles layered on top of the existing Gate 2 checklist — never a
  substitute for it.
- Does not apply to, or weaken, `high-risk-operations-rule.md` or
  `destructive-op-preflight-rule.md` red-list items: a rate-limited or
  cooled-off approval is still bound by those rules' STOP-and-plan
  requirement in full.

## CUSTOMIZE

Adopters MUST customize (all start empty/disabled):
- `cooling_off_minutes`
- `max_approvals` / `window_minutes`
- `max_actions` / `max_estimated_tokens`
- Where these fields live — candidate location: a new `governance_primitives:`
  section in `docs/coordination/agov-config.yaml`, or `agf/GOVERNANCE.md`.
  Not decided in this plan; follow-on implementation issue.

## Related rules

- `workflow/deputy-governor-rule.md` — `audit_trigger` is the count-based
  sibling to this rule's time-windowed rate-limit; see relation table above.
- `always/high-risk-operations-rule.md` — red-list categories remain fully
  gated regardless of any active throttle.
```

---

## Affected files

Nothing is created in this Gate 1 planning session. The table below is what a
later, approved implementation session would create/modify.

| File | Action |
|---|---|
| `agf/rules/workflow/time-budget-primitives-rule.md` | Create — full rule text per the design above |
| `agf/rules/workflow/deputy-governor-rule.md` | Update — add a `Related rules` cross-reference to the new rule (no semantic change to the existing red list or delegation mechanics) |
| `agf/framework.md` | Update — add a row to the §6 "Rule–Principle Mappings" table for the new rule under Principle 5 / Pattern 6 |
| `docs/coordination/agov-config.yaml` | **Not touched by this plan** — config-schema wiring is explicitly deferred to a follow-on implementation issue, listed here only so the reconciling human sees it was considered and deliberately excluded |

---

## Migration needed

No code or data migration. The config-schema change referenced above (new
optional section in `agov-config.yaml`) is explicitly deferred, per the
issue's own scope boundary ("implementing enforcement mechanics ... is
follow-on work").

---

## Test strategy

Since this plan produces no code:
- Gate 2 verification is a read-through: confirm the new rule text does not
  contradict or loosen `deputy-governor-rule.md`'s red list (a MUST, stated
  explicitly in "What this rule does NOT do" above).
- `cargo run -p agov ... -- validate framework-config` (existing validator) —
  confirm the updated `framework.md` rule-principle mapping table still
  validates after the new row is added.
- When enforcement mechanics are implemented in a follow-on session: unit
  tests per primitive (cooling-off elapsed-time check, rate-limit
  rolling-window count, budget ceiling check), each with a positive case
  (under the limit, allowed) and a negative case (over the limit, blocked) —
  per `workflow/tdd-rule.md`.

---

## Risks

| Risk | Mitigation |
|---|---|
| Adopters read "rate-limited" as license to skip content review once under the limit | Rule text explicitly states these are additive to, never a substitute for, the Gate 2 checklist |
| Confusion between this rule's rate-limit and `deputy-governor-rule`'s `audit_trigger` | Explicit relation table in the rule text; both rules cross-reference each other |
| This rule ships with zero AGF-specific incident evidence, unlike most provisional rules in this rule set | Labeled honestly as Theoretical evidence grounding (not borrowed "single-adopter" language); adopters activating a primitive are asked to file a learning record |
| A rate-limit or cooling-off primitive is misconfigured to also throttle red-list (high-risk) approvals into silence rather than blocking them outright | Rule text explicitly states red-list items remain fully gated regardless of any active throttle |

---

## Rollback

Revert the branch. No schema, code, or data changes to roll back.

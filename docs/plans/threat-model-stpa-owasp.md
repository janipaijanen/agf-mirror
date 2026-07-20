---
id: threat-model-stpa-owasp-42
status: pending-gate1
topic: declared-threat-model-gate-chain
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Single-agent Gate 1 planning session (no code, only an ADR + docs page
      proposed). No live infosec-sentinel run (agov validate infosec is scoped
      to learning records, not docs/ADR content) applies here. Given this
      plan's subject matter is literally AGF's own security threat model,
      a real infosec-sentinel pass is recommended at Gate 2 once the ADR and
      docs page exist in draft, specifically to check the per-threat-class
      coverage claims (Addressed / Partially addressed / Not addressed) in
      the "STPA control structure and unsafe control actions" section below
      against the actual current rule files, not against what this plan
      believes they say.
  - role: contribution-auditor
    status: waived
    reason: >-
      No external contribution, rule change, or agent-authority change lands
      in this plan itself. Recommend a Gate 2 pass specifically checking that
      the published docs page does not overclaim gate strength to external
      evaluators (the whole point of the issue is honest scoping, so an
      adversarial read of the plan's own honesty is warranted).
---

# Plan: Declared Threat Model (STPA-derived) for AGF's Gate Chain

**Date:** 2026-07-20T07:51:31Z
**Status:** PENDING GATE 1 APPROVAL
**GitLab issue:** #42
**Cluster:** declared threat model + governance primitives (this plan is the
foundational one; see companion plans below)
**Companion plans (same cluster, same branch):**
- `docs/plans/governance-time-budget-primitives.md` (#48) — cites UCA-3 below
- `docs/plans/input-provenance-manifest.md` (#49) — cites UCA-4 below

---

## Intent

AGF's gate chain (Gate 1 plan approval, Gate 2 merge review, the coordination
channel, the advisory rule set) currently makes no explicit statement of what
kind of failure it is designed to catch, and — just as importantly — what it
is not designed to catch. Anyone evaluating AGF for adoption has to infer the
threat model from reading ~40 rule files and the framework document, rather
than reading one authoritative statement.

This plan derives that statement using two bodies of prior art named in the
issue:

- **STPA** (Systems-Theoretic Process Analysis; Leveson, N. (2011). *Engineering
  a Safer World: Systems Thinking Applied to Safety*. MIT Press) — a structured
  method for deriving where control is actually needed by modeling the control
  structure (controllers, control actions, feedback) and enumerating **Unsafe
  Control Actions (UCAs)**: ways a control action can be missing, wrong, or
  mistimed such that it leads to a hazard.
- **OWASP Top 10 for LLM Applications** (2025 list, published at
  `owasp.org/www-project-top-10-for-large-language-model-applications/`) —
  vocabulary security reviewers already recognize. *Numbering below should be
  re-verified against the live OWASP page at implementation time — OWASP
  revises category numbers between list revisions, and this plan's numbering
  is drawn from the planning agent's training knowledge, not a live fetch.*

The output is a docs page (public site) plus an ADR recording the scope
decision — not new gates. Closing gaps this analysis reveals is out of scope
here; two of the gaps it reveals are exactly what the companion plans (#48,
#49) close, and it explicitly flags a third gap owned by a different cluster
(#34/#47) under an assumption (see Risks).

---

## Scope

**In scope:**
1. State explicitly which actor model AGF's gates assume: cooperative-but-
   fallible, adversarial, or a stated mixed position.
2. Derive gate placement via an STPA-style control-structure analysis.
3. Map UCAs to OWASP LLM Top 10 vocabulary where an AGF gate maps onto a named
   threat class.
4. Publish as a docs page: what is covered, what is explicitly out of scope.
5. Record the decision as an ADR (this is a foundational, hard-to-reverse
   framing decision — it fits the ADR bar of "significant architecture
   decision").

**Out of scope (explicitly, per the issue):**
- Building new gates to close every gap the analysis reveals.
- Full remediation of every "Not addressed" row — follow-on work, some already
  claimed by #48 and #49.

---

## The actor-model question — stated position

**AGF's gate chain is designed primarily for a cooperative-but-fallible actor
model.** Agents and humans are assumed to be honestly trying to follow the
rules but subject to: scope drift, automation bias, fatigue, ambiguity, and
machine-speed velocity that outstrips careful review. Most of AGF's gates
(the rule files, Gate 1/Gate 2 checklists, the coordination channel) are
**advisory** — see `agf/docs/site/docs/concepts/roles-and-gates.md`,
"Structural and advisory enforcement" — meaning they rely on the acting agent
reading and cooperating with the rule text. An advisory gate provides **no**
structural resistance to an actor deliberately trying to bypass it: nothing
in the repository stops an agent with write access from ignoring a rule file,
skipping a checklist, or running `git push --force` against the rule's advice
(absent host-side branch protection, which AGF does not itself configure or
require).

AGF provides only **partial, incidental** resistance to a genuinely
adversarial actor (one deliberately trying to defeat the gates), via:
- The **structural** gates that do exist: JSON-schema validation on
  coordination messages and plans (`agov coord validate`, `agov plan
  validate`), which check artifact **shape**, not artifact **truthfulness**
  (a message can validly claim "Gate 2 approved" without the approval having
  actually happened, if the acting agent chooses to write that).
- Git's own commit history as an append-only ledger (assuming the adopter's
  git host enforces branch protection and disables force-push on `main` —
  AGF documents this as adopter responsibility, it does not enforce it).

AGF explicitly does **not** claim to guarantee:
- Byzantine-fault tolerance against a compromised or malicious agent with
  repository write access.
- Cryptographic non-repudiation of authorship (coordination messages record a
  `from` role string and optional `model`/`session` fields for audit, not a
  signature).
- Prevention of collusion where the same principal effectively plays both
  implementer and reviewer roles — `agf/framework.md` §3 "Principle 9: Single
  Decision Point" is itself marked **[Theoretical]** in the framework's own
  evidence-status taxonomy, meaning even the framework does not yet claim this
  is solved.

This is the honest position: AGF is a **discipline-and-audit-trail framework**
for well-intentioned but fallible actors, not a security boundary against a
determined adversary with repository access. Where a gate happens to also
resist an adversarial actor, that is stated as incidental, not designed.

---

## STPA control structure

**Controllers** (who issues control actions):
- **Implementer** (planner-architect / coder-tester) — proposes plans, code,
  docs, rules.
- **Governor-Reviewer** — approves Gate 1 and Gate 2.
- **Human Governor** — ultimate authority; holds delegation and revocation
  power over any Deputy Governor (`agf/rules/workflow/deputy-governor-rule.md`).

**Controlled process:** the repository's committed state (code, rules, agent
definitions, schemas, docs) and the coordination channel (the audit trail
itself, `docs/coordination/*.md`).

**Control actions:**
| # | Control action |
|---|---|
| CA1 | Implementer submits a plan (Gate 1 request) |
| CA2 | Governor approves or rejects the plan |
| CA3 | Implementer implements and submits for Gate 2 |
| CA4 | Governor approves or rejects the merge |
| CA5 | Implementer merges |
| CA6 | Any agent escalates (Pattern 3, `framework.md` §4) |
| CA7 | Deputy Governor approves under an active delegation artifact |

**Feedback:** coordination messages (claims, handoffs, reviews, decisions),
CI validator results (`agov validate ...`), test suite results, learning
records.

## Unsafe Control Actions (UCAs) and current coverage

Each row states the hazard, maps it to OWASP LLM Top 10 vocabulary where it
fits, and states current coverage using the implementation-honesty five-level
maturity model (Planned / Schema-defined / Instantiated / Validated /
Deployable) plus a plain-English Addressed / Partially addressed / Not
addressed summary. **A reviewer should be able to point at the named rule
file for every "Addressed" or "Partially addressed" claim** — that is the
verifiability bar this plan is held to.

| # | UCA (STPA form: action provided/not-provided/wrong-timing/wrong-duration causing a hazard) | OWASP LLM Top 10 mapping (hedge: re-verify numbering) | Current coverage | Rule(s) |
|---|---|---|---|---|
| UCA-1 | CA5 (merge) executed without CA4 (Gate 2 approval) having occurred — approval "not provided" before the action it gates | — (process-integrity, not LLM-specific) | **Partially addressed.** Advisory only: nothing structural stops a merge without a decision-record file existing. | `always/coordination-sequence-guard-rule.md`, `workflow/merge-request-workflow-rule.md` |
| UCA-2 | CA2/CA7 issued for a red-list category (destructive op, security boundary, architecture change) under Deputy Governor delegation | LLM06 Excessive Agency | **Addressed at the rule-text level; Instantiated only if a delegation artifact is ever created.** The CUSTOMIZE block ships empty — delegation is off by default, so this UCA currently has no active exploit surface, but the rule exists for when it is turned on. | `workflow/deputy-governor-rule.md` §2 (red list) |
| UCA-3 | CA1/CA2/CA3/CA4 issued at machine speed far exceeding a human governor's capacity for meaningful review — the classic rubber-stamp / automation-bias hazard (Bainbridge, L. (1983). "Ironies of Automation." *Automatica*, 19(6), 775–779) | LLM10 Unbounded Consumption (analogized: unbounded proposal velocity outstripping oversight capacity, not resource cost) | **Not addressed today.** No cooling-off, rate-limit, or budget primitive exists. *This is exactly the gap closed by companion plan #48.* | none yet — see `docs/plans/governance-time-budget-primitives.md` |
| UCA-4 | An agent acts under a rule file / skill / MCP server definition whose version silently differs from what was reviewed, with no record of which version was actually in effect | LLM03 Supply Chain | **Not addressed today.** No provenance manifest exists. *This is exactly the gap closed by companion plan #49.* | none yet — see `docs/plans/input-provenance-manifest.md` |
| UCA-5 | A governed action's audit entry is written on the error/failure path, or omitted entirely (silent action) | — | **Addressed, advisory.** | `always/coordination-message-rule.md`, `always/audit-log-completeness-rule.md` |
| UCA-6 | A destructive/irreversible operation is recommended or executed without verifying its stated preconditions | — | **Addressed, advisory.** Fires at the recommendation boundary; nothing structurally stops execution if the agent ignores the rule. | `workflow/destructive-op-preflight-rule.md` |
| UCA-7 | A gate/validator is introduced but never actually fires on the bad input it was built to catch ("ships broken") | — | **Addressed, advisory, MUST-strength** (three documented incidents motivated this rule). Still requires a reviewer to check the negative-test artifact was really produced. | `always/gate-self-test-rule.md` |
| UCA-8 | An externally-destined artifact leaks project-identifying information across the anonymization boundary | LLM02 Sensitive Information Disclosure | **Addressed, advisory + one-time retroactive audit.** MUST-strength for name-revealing projects. | `always/external-anonymization-rule.md` |
| UCA-9 | A contribution (learning, rule, agent definition, MR) carries adversarial steering or false precedent and merges without a dedicated adversarial-pattern review | LLM06 Excessive Agency / LLM09 Misinformation | **Partially addressed.** The contribution-auditor role and the advisory-consultation gate (T2-5) force a *decision* about whether contribution-auditor was consulted, but do not verify the consultation was substantively correct. | contribution-auditor agent def, `docs/plans/T2-5-advisory-consultation-gate.md` |

---

## What this plan asserts is explicitly OUT of AGF's current threat model

Stated plainly on the published page, not just implied:

- Protection against a malicious actor with repository write access acting in
  bad faith (as opposed to a fallible actor making honest mistakes).
- Non-repudiation / cryptographic proof of who authored a coordination
  message.
- Detection of collusion between implementer and reviewer when both are the
  same underlying principal.
- Any guarantee that an advisory gate was actually read before being
  bypassed — advisory gates leave an audit *gap*, not a *block*.

---

## Affected files

Nothing is created in this Gate 1 planning session (per project convention:
this session is planning-only). The table below is what a later, approved
implementation session would create/modify.

| File | Action |
|---|---|
| `agf/docs/adr/<YYYYMMDD>-declared-threat-model.md` | Create — ADR recording the cooperative-but-fallible primary / incidental-adversarial-resistance-secondary position and the STPA method adopted |
| `agf/docs/adr/README.md` | Update — append index row for the new ADR |
| `agf/docs/site/docs/concepts/threat-model.md` | Create — the public docs page (audience: developers and evaluators per documentation-audience-rule; this is the authoritative canonical statement other docs should link to, not paraphrase, per marketing-tone-rule's message-integrity section) |
| `agf/docs/build/mkdocs.yml` | Update — add nav entry under `Concepts` (line ~80-84 today), e.g. `Declared threat model: docs/concepts/threat-model.md` |
| `agf/framework.md` | Update (optional) — a short cross-reference from §3/§4 to the new docs page, if the Gate 2 reviewer judges it improves discoverability |

---

## Migration needed

No. Docs and an ADR only; no schema, code, or data change.

---

## Test strategy

- `cd agf && mkdocs build -f docs/build/mkdocs.yml --strict` — the existing
  strict-mode gate (`agf/docs/build/README.md`) fails the build on any broken
  internal link introduced by the new page or nav entry.
- Gate 2 checklist addition (manual, not automated): for every row in the UCA
  table marked "Addressed" or "Partially addressed," the reviewer opens the
  cited rule file and confirms it says what this plan claims it says. This is
  the implementation-honesty-rule's verifiability bar applied to the plan
  itself, not just to the published page.
- Cross-check the OWASP category numbers against the live OWASP LLM Top 10
  page before publishing (noted inline above; the planning agent could not
  fetch the live page in this Gate 1 session and is flagging that
  explicitly rather than presenting unverified numbers as fact).

---

## Risks

| Risk | Mitigation |
|---|---|
| The docs page overclaims gate strength to external evaluators | Every claim in the UCA table is tied to a maturity level and a named rule file; Gate 2 explicitly re-checks this (see Test strategy) |
| The threat model goes stale as gates evolve (new rules land, old ones retire) | Recommend a "last verified against commit `<sha>`" stamp on the page and a periodic Gate 2 checklist item, mirroring the cadence pattern in `workflow/visible-artifact-verification-rule.md` |
| This STPA/UCA derivation is a single planning agent's first pass, not independently reviewed | Flagged for Gate 2 infosec-sentinel and contribution-auditor passes (see `advisory_consultation` above) |
| OWASP category numbers cited from training knowledge may be stale or wrong | Explicitly flagged inline in two places above; implementer must re-verify against the live page before publishing |
| **Cross-cluster assumption:** this plan assumes issues #34/#47 (the separate gate-enforcement cluster, planned concurrently by another agent) are about converting specific *advisory* gates identified above (most plausibly UCA-1's missing structural check that a Gate 2 decision-record exists before merge, and/or UCA-9's contribution-review verification) into *structural*, CI-enforced checks — based only on the naming pattern and the framework's own stated intent ("Add structural checks wherever the automation cost is low," `roles-and-gates.md`). This plan has no visibility into the actual text of #34/#47. If their real scope differs, the UCA coverage table in the published page will need a follow-up edit once both clusters are reconciled by the human. This is stated here explicitly rather than silently guessed at. | Human reconciler cross-checks this plan's UCA table against the actual #34/#47 plans before both merge; update the coverage column if the assumption was wrong |

---

## Rollback

Revert the branch. No schema, code, or data changes; nothing to migrate
backward.

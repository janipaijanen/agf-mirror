# AGF-NG Proposal Synthesis

**Sources:** Five AI model assessments (Lumo, Claude, Grok, ChatGPT, CoPilot)
+ two field agent assessments (Claude Code and Codex from projects), collected 2026-06-08.

**Evidence file:** `docs/AIs thoughts on agents-govern 2026-06-08.md`

**Priority criteria:**
1. Token/quality ROI — how much does this reduce cold-start tokens or improve output quality?
2. Convergence — how many independent sources identified the same gap?
3. Scalability — does it scale beyond a single project?
4. Implementation complexity — lower is better in earlier tiers

**Governance rule:** No proposal moves to implementation without a Gate 1
plan citing its evidence. Tier 1–2 items are approved for planning. Tier 3–5
require Governor-Reviewer confirmation before planning begins.

---

## Tier 1 — Operational Substrate

*Build first. These are pure tooling additions that don't change the framework
model. They reduce cold-start token cost and prevent the most expensive
coordination failure modes (collisions, stale-state mistakes).*

*All four items scale fully to any multi-agent project.*

### T1-1: Coordination message `kind` schema + validation

**Sources:** Codex P3, Claude Code P3  
**Convergence:** 2 independent field agents  
**Evidence:** Codex field assessment §Priority 3; Claude Code field assessment §Priority 3

The coordination protocol has evolved a set of recurring message shapes
(claim, completion, review, ACK, interface-contract, decision-record,
handoff). These exist as conventions but are not schema-enforced. Agents
spend tokens re-explaining the expected shape in each message; readers
spend tokens inferring whether a message is a claim or a note.

**Proposed change:**
- Add `kind:` field to coordination message front matter (already adopted in
  AGF-NG's own channel — see `docs/coordination/README.md`)
- Add validation rules per kind (e.g., `claim` must have non-empty
  `artifacts_owned` or explanation; `completion` must release artifacts)
- Add `agov-validate-coordination` command that checks messages against kind rules
- Ship kind schema in `schemas/coordination-message.schema.json`

**Outcome:**
- Agents can branch immediately on `kind` without reading message body
- Missing handoff details become validation errors, not future confusion
- Token cost drops: schema carries meaning prose used to carry

**Evidence citations:** Codex: "Without schemas, agents spend tokens
re-explaining the expected shape"; Claude Code: "The conventions are now
strong enough to deserve schemas."

---

### T1-2: Active ownership query (`agov coord status`)

**Sources:** Codex P1, Claude Code P1  
**Convergence:** 2 independent field agents  
**Evidence:** Codex field assessment §Priority 1; Claude Code field assessment §Priority 1

`artifacts_owned` is the single most useful local governance primitive in
field use. It is also manual: a cold-start agent must read coordination
files, infer which claims are still active, and avoid files owned by another
agent. This costs tokens and is easy to get wrong. In field use, two agents
simultaneously claimed overlapping artifacts across multiple sessions.

**Proposed change:**
- Add `agov coord status --path <file>` and `agov coord status --topic <name>`
- Add `agov coord obligations` (list open `requires-ack: true` messages)
- Parser: treat `artifacts_owned: [...]` as active until same agent posts
  `completion` with empty list OR explicit `releases_artifacts: [...]`
- Output compact status table (not raw YAML)

**Example output:**
```
Path: agents-govern/framework.md
Status: claimed
Owner: claude
Claim: docs/coordination/00003-claude.md
Topic: add-kind-schema
Safe for human to edit: no — release expected at completion
```

**Outcome:** Cold-start agents answer "can I touch this?" in one command
instead of reading 20–80 files. Estimated saving: several thousand tokens
per session cold-start.

**Evidence citations:** Codex: "a cold-start agent has to read coordination
files, infer which claims are still active"; Claude Code: "artifacts_owned
is the actual concurrency control mechanism — but it's checked by convention."

---

### T1-3: Async handoff state (`kind: handoff` + `agov handoff next`)

**Sources:** Codex P7, Claude Code P5  
**Convergence:** 2 independent field agents  
**Evidence:** Codex field assessment §Priority 7; Claude Code field assessment §Priority 5

The framework is good at phase gates but weak at async reality. A gate can
be open while no agent is running. The human Governor becomes the scheduler.
In field use, a polling side-project was abandoned precisely because
file-based pollers cannot wake inactive LLM sessions.

The framework does not need to become an agent runtime. It needs to record
handoff state explicitly so the next agent can query what is ready.

**Proposed change:**
- Add `kind: handoff` message type to the schema (already in AGF-NG channel)
- Required fields: `next_agent`, `status` (ready/blocked), `task`, `must_read`,
  `do_not_touch`
- Add `agov handoff next --agent <name>` command
- Output: compact "what's ready for me" summary

**Outcome:** Cold-start agents get exact scope and forbidden areas without
the human restating them. Human Governor stops scheduling; human approves.

**Evidence citations:** Codex: "many real workflows are: Agent A finishes half
→ human sleeps → Agent B wakes cold → Agent B must know what is ready";
Claude Code: "The abandoned polling side-project was a symptom of this."

---

### T1-4: Coordination brief (`agov coord brief`)

**Sources:** Codex P2  
**Convergence:** 1 field agent (high confidence — directly addresses cold-start cost)  
**Evidence:** Codex field assessment §Priority 2

Raw coordination logs grow. Digests are hand-authored and inconsistent.
A cold agent currently decides how much tail to read — a decision that burns
tokens and attention. The brief is a machine-generated reading aid that
extracts the minimum a cold-start agent needs.

**Proposed change:**
- Add `agov coord brief --since-digest` and `--agent <name>` and `--open-only`
- Brief sections: digest coverage, tail scanned, active claims, open ACKs,
  key decisions since digest, recommended raw reads
- Must state it is a reading aid, not authority. Raw files remain authoritative.

**Outcome:** Agents read a 50-line brief + referenced raw files instead of
the full coordination tail. Estimated saving: 60–80% reduction in raw
coordination lines read at cold start.

**Evidence citations:** Codex: "Instead of every agent reading raw tail plus
digest plus role YAML plus local plans, it can read a 50-line brief."

---

## Tier 2 — Governance Tightening

*Medium effort, high value. These tighten the existing model without changing
its structure. All scale beyond this project.*

### T2-5: Advisory consultation gate enforcement

**Sources:** Codex P4, Claude Code P2  
**Convergence:** 2 independent field agents  
**Evidence:** Codex §Priority 4; Claude Code §Priority 2

Advisory roles are underused in practice. `invoke_when` is not enough.
In field use, a domain advisor was never consulted on vocabulary decisions
during the period of observation — not because the decisions were trivial
(they weren't), but because the coordination cadence created no natural
pause point where consultation felt mandatory.

**Proposed change:**
- Add `advisory_consultation:` section to Gate 1 checklist schema
- Each advisory role has: `status: consulted | waived | not-applicable`
- `waived` requires a reason; `not-applicable` requires no reason
- Gate 1 cannot be considered complete with any advisory role left unchecked
- Add trigger mapping: paths/topics that imply a given advisor should be consulted
- Gate 1 template updated to include advisory section

**Outcome:** Advisory roles either get consulted, get a waiver with reasoning,
or the plan is rejected. Prevents the "advisory but invisible" pattern.

**Evidence citations:** Claude Code: "advisory agents currently go unconsulted
because nothing in the flow demands it"; Codex: "If a plan changes domain
vocabulary, security posture, data model, or deployment behavior, the relevant
advisor should either be consulted or explicitly waived."

---

### T2-6: `artifacts_owned` lapse semantics (formal spec)

**Sources:** Claude Code P1 specifics  
**Convergence:** 1 field agent (corroborated by Codex's T1-2)  
**Evidence:** Claude Code field assessment §Priority 1

The `artifacts_owned` convention works, but its lapse semantics are implicit.
There is no schema definition for when ownership lapses, no formal spec an
agent can read to verify its understanding, and no validator that catches a
claim that was never released.

**Proposed change:**
- Add formal lapse semantics to coordination schema:
  - Ownership lapses when same agent posts `completion` with empty `artifacts_owned`
  - OR explicit `releases_artifacts: [...]` field
  - Optional: `expires_on: completion | seq-N | handoff` for time-bounded claims
- Add `agov-validate-coordination` check: warn on claims older than N messages
  without a corresponding release

**Outcome:** Agents can verify their understanding against a spec. Validators
catch stale claims. Reduces "is this still owned?" uncertainty.

**Evidence citations:** Claude Code: "add to the base message schema: expires_on:
completion | seq-N | handoff"; Codex: T1-2 directly depends on this being specified.

---

### T2-7: Digest skeleton generator

**Sources:** Codex P8  
**Convergence:** 1 field agent (high confidence — addresses known pain from field)  
**Evidence:** Codex field assessment §Priority 8; Claude Code §"The digest format is a design that hasn't been validated"

Digests are useful but inconsistently authored. A digest author decides what
to include based on judgment, not a schema. Future readers must decide whether
the digest is complete enough before trusting it as a reading aid.

**Proposed change:**
- Add `agov coord digest-skeleton --from N --to M`
- Generator fills mechanical sections automatically:
  - Files in range, seq collisions, `requires-ack` messages,
    `artifacts_owned` claims and releases, kind distribution
- Author fills interpretive sections: decisions, unresolved questions,
  recommended reads
- Digest schema: required sections defined; `agov-validate-coordination`
  checks presence

**Outcome:** Comparable digests across the project. Cold starts faster because
readers trust the shape. Digest authoring time target: under 20 minutes for
a 50-message range.

**Evidence citations:** Claude Code: "the digest spec needs to be either a
stricter schema or explicitly acknowledged as 'best effort'"; Codex: "digest
authors spend effort deciding what to include."

---

### T2-8: Small-task fast path formalization

**Sources:** Codex P10; framework already has Pattern 6 (Fast Track)  
**Convergence:** 1 field agent + framework's own Pattern 6  
**Evidence:** Codex §Priority 10; framework.md §Pattern 6

Pattern 6 (Fast Track) in 1.1.50 is a good concept but underspecified in
practice. Agents need a machine-readable definition of what qualifies for
fast track, not just prose guidance. Without it, fast-track decisions are
entirely Governor-Reviewer-dependent memory.

**Proposed change:**
- Add `small_task` profile to framework (or as a schema-validated field in
  Gate 1 templates):
  - `allowed_when: max_files_touched: 2, no_shared_interface_change, no_data_model_change,
    no_security_or_privacy_change, no_active_claim_conflict`
  - `required: pre-edit ownership query, concise completion note, focused verification`
- Add fast-track to coordination message kinds: `kind: fast-track-waiver`
  (already adopted in AGF-NG channel — see README.md)
- Update Pattern 6 in framework.md with the formal criteria

**Outcome:** Agents have a machine-readable spec for what qualifies as
fast-track. Governors have a checklist, not just guidance. Governance
doesn't become theater for low-risk work.

---

## Tier 3 — Framework Maintainability

*These improve the framework's own long-term health: how it evolves, how
roles proliferate, how context costs are managed. All scale to any project.*

### T3-9: `origin` + `upstream_candidate` fields on role YAMLs

**Sources:** Claude Code P3  
**Convergence:** 1 field agent  
**Evidence:** Claude Code field assessment §Priority 3

Project-specific roles accumulate over time. Framework maintainers cannot
easily identify which project roles are ready for upstream absorption without
reading the Governor's head. One grep should surface the backlog.

**Proposed change:**
- Add to agent YAML schema:
  ```yaml
  agent:
    origin: "derived" | "project" | "framework"
    upstream_candidate: true  # optional
  ```
- `agov-validate-agents` reports all `upstream_candidate: true` agents
- Agent template updated to include `origin` field

**Outcome:** `grep upstream_candidate agents/*.yaml` gives the framework
maintainer an immediate backlog of validated project roles ready for
upstream consideration. No Governor memory required.

---

### T3-10: Skills as named concept in MANIFEST

**Sources:** Claude Code P4  
**Convergence:** 1 field agent  
**Evidence:** Claude Code field assessment §Priority 4

Skills (slash commands) are procedure knowledge. Currently they live in
`.claude/commands/` and `.claude/skills/` but are not referenced in
MANIFEST.yaml, not tracked for usage, and not visible to the Governor as
a governed surface.

**Proposed change:**
- Add `skills:` section to MANIFEST schema alongside `agents:`
- Each entry: `name`, `file`, `invoke` (slash command), `status`, `origin`
- `agov-validate-agents` checks manifest skills against the filesystem
- Skills can be `active`, `suspended`, or `skipped` like agents

**Outcome:** Procedure knowledge is version-controlled and Governor-visible.
Skills can be retired when unused. The governed surface includes automation,
not just agent definitions.

---

### T3-11: Role and skill usage telemetry

**Sources:** Codex P6  
**Convergence:** 1 field agent  
**Evidence:** Codex field assessment §Priority 6

Role proliferation is a real risk (field evidence: 14+ agents in MANIFEST
after extended operation). Unused roles add cold-start reading overhead and
governance drift. There is currently no way to ask "which advisory roles
have been consulted recently?"

**Proposed change:**
- Add `agov usage report --last 30d`
- Parses coordination messages for: which agents sent/received messages,
  which skills were invoked (if logged), which advisory roles were cited
  as consulted vs. triggered but unconsulted
- Add lifecycle annotations to MANIFEST:
  ```yaml
  lifecycle:
    state: active
    since: "2026-06-11"
    review_after: "2026-09-11"
    retirement_signal: "No consultations after 3 domain-changing tasks"
  ```

**Outcome:** Smaller active role surface → less startup reading → less
governance drift. Governors can retire stale roles with evidence rather than
memory.

---

### T3-12: Token/context budget as plan field

**Sources:** Codex P5, Lumo §Hierarchical Rule Trees  
**Convergence:** 2 sources (1 field agent, 1 web LLM)  
**Evidence:** Codex §Priority 5; Lumo assessment §Mechanism 1

Context is a constrained resource for every LLM agent. Task plans currently
don't specify which files/messages must be read vs. may be skipped. Agents
over-read defensively, or under-read and make mistakes.

**Proposed change:**
- Add `context_plan:` section to plan template (optional for small tasks):
  ```yaml
  context_plan:
    must_read:
      - docs/coordination/000NN-claude.md
      - agents-govern/framework.md §Section 4
    may_skip:
      - historical coordination before latest digest
    max_raw_messages: 5
    max_code_files: 8
    token_risk: low | medium | high
  ```
- Add `token_risk` to Structural Budget template

**Outcome:** Agents have explicit reading boundaries → fewer over-reading
turns → lower token cost. "Token risk: high" is a planning signal to split
the task or add a handoff.

---

## Tier 4 — Adoption & Documentation

*These lower the adoption barrier. Important, but don't block Tier 1–2 work.*

### T4-13: Minimal adoption profile

**Sources:** Codex upstream suggestion 1, Grok, CoPilot  
**Convergence:** 3 sources  

For tiny projects (1 person, 1 AI, throwaway prototype), the full 7-step
ADOPTING.md is overkill. A "minimal" profile should exist with: `AGENTS.md`
(identity file), three role YAMLs, one coordination template, one Gate 2
check. No learning machinery unless needed.

---

### T4-14: Coordination channel recipe as first-class adoption artifact

**Sources:** Codex upstream suggestion 2  
**Convergence:** 1 field agent (corroborated by a second project independently
arriving at the same pattern)  

The coordination channel is the highest-value local extension from two
independent adopter projects. It should become a first-class adoption
artifact: a named recipe with templates, examples, and ADOPTING.md steps.
AGF-NG's own `docs/coordination/README.md` is the starting point.

---

### T4-15: Shared interface contract template

**Sources:** Codex upstream suggestion 3  
**Convergence:** 1 field agent  

When two agents work on opposite sides of a shared interface (API, port,
schema), an interface contract message prevents simultaneous "I'll just wire
my side" implementation. Template already exists in the field assessment.
Needs generalization and inclusion in the framework's
template set.

---

### T4-16: Ceremony budget concept

**Sources:** Codex upstream suggestion 5, CoPilot §Prioritization  
**Convergence:** 2 sources  

Define what governance ceremony can be skipped for low-risk tasks and how
to record that the skip was intentional (not accidental). "Ceremony budget"
is distinct from fast-track: it's about scope of governance documentation,
not gate skipping. Connects to T2-8.

---

### T4-17: Runtime enforcement gap checklist

**Sources:** Codex upstream suggestion 6, Grok §Tooling integration, ChatGPT §Runtime governance  
**Convergence:** 3 sources  

The framework is explicit that it does not enforce at runtime. But what
DOES enforce things — CI, branch protection, file permissions, tool policy —
is underspecified. A checklist of "what the framework describes vs. what
your environment must enforce" would close the credibility gap.

---

### T4-18: Anonymized examples from real coordination transcripts

**Sources:** Codex P9, Codex upstream suggestion 7, CoPilot §Missing examples  
**Convergence:** 2 sources  

Adopters copy working patterns, not descriptions. Field project transcripts
contain worked examples of: claim → contract proposal → ACK →
implementation → completion → review. Anonymized versions of these would
dramatically lower adoption
friction for the coordination channel pattern.

---

### T4-19: Adoption health check

**Sources:** Codex upstream suggestion 8  
**Convergence:** 1 field agent  

"Are agents reading the YAML, or just quoting it?" There is currently no
systematic way to assess governance health. A health check command (`/health`
skill exists but could be extended) or questionnaire that asks: "Are ACKs
meaningful? Are lessons changing behavior? Are digests current?" would surface
governance decay before it becomes critical.

---

### T4-20: Outcome-first framing + stronger tone in docs

**Sources:** CoPilot, Claude (web) §improvements  
**Convergence:** 2 sources  

The ADOPT page and related docs are good but passive. Missing: "what you
will have when done," concrete artifacts/deliverables, prioritized Day 0/30/90
view, and stronger imperative tone ("Define" not "should define"). Applies
across ADOPTING.md, adopt/ pages, and the framework's own role descriptions.

---

## Tier 5 — Paradigm Evolution

*Longer-horizon architectural changes. Do not plan until Tier 1–3 are in
progress and providing evidence. These may or may not belong in agents-govern
vs. a separate tool.*

### T5-21: Decisions as primary governance object

**Source:** ChatGPT §Improvement 1  
**Summary:** Reframe governance from "agents and roles" to "decisions and
their authority, constraints, evidence, and audit." Agents still exist but
governance attaches to decisions (approve-invoice, modify-schema, etc.)
rather than to agent roles. Machine-readable and enforceable.  
**Risk:** Significant model change; may require a different tool layer.

### T5-22: Autonomy budgets

**Source:** ChatGPT §Improvement 3  
**Summary:** Replace binary approved/not-approved with quantitative budgets:
spend limit, API calls, actions per time window. Bounded risk, easier audit.  
**Risk:** Requires runtime enforcement, which the framework explicitly excludes
from scope. May be an adjacent tool, not the framework itself.

### T5-23: Capability governance

**Source:** ChatGPT §Improvement 2  
**Summary:** Govern capabilities (send-external-email, access-payroll) rather
than reviewing agents individually. Any agent using a capability inherits
the rule. Scales better than per-agent approval chains.  
**Risk:** Overlaps with existing cross-cutting agent model; needs careful
design to avoid duplicating existing structure.

### T5-24: Governance compression layers

**Source:** ChatGPT §Improvement 5  
**Summary:** Board policy → operational rule → machine rule, each derived
from the previous with explicit traceability. Lower drift, fewer interpretations.  
**Note:** This is directionally consistent with the framework's existing
evidence chain (learning → rule). May be an extension rather than a rewrite.

### T5-25: Continuous governance scoring

**Source:** ChatGPT §Improvement 8  
**Summary:** Live `risk_score`, `drift_score`, `autonomy_score`, `trust_score`
updated from coordination activity. Live picture vs. one-time review.  
**Risk:** Requires persistent state and automated analysis; not currently in
framework scope.

### T5-26: Exception handling over approval chains

**Source:** ChatGPT §Improvement 4  
**Summary:** Optimize for catching anomalies (10/10,000 actions) rather than
reviewing everything. Automated approval for routine, human review for
statistical outliers.  
**Risk:** Requires anomaly detection infrastructure outside current framework scope.

---

## Cross-Cutting Notes

**What all Tier 1–2 items share:**
- They are pure additions (no existing content removed)
- They produce machine-readable artifacts (schemas, command output)
- They directly reduce cold-start token cost OR prevent coordination failures
- All scale to any multi-agent project without project-specific customization

**What the Tier 5 items share:**
- They require runtime enforcement infrastructure the framework currently
  explicitly excludes from scope
- They may represent adjacent tools rather than framework evolution
- ChatGPT's framing is enterprise-scale (hundreds of agents); test against
  the framework's primary evidence base (2-4 agents, solo governor) first

**The convergence signal:**
Items with 2+ independent sources (T1-1, T1-2, T1-3, T2-5, T2-6, T2-7,
T3-12) are the highest-confidence priorities. Items with 3+ sources (T2-5,
T4-13, T4-17) are essentially consensus findings.

---

*Document status: proposed | Awaiting Governor-Reviewer approval (see 00001-claude.md)*  
*Last updated: 2026-06-11*

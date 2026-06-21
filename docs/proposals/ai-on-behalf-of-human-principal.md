# Proposal note: AI acting on behalf of a human principal

*Status: exploratory analysis. Audience: planner-architect, governor-reviewer,
external reviewers. This is not legal advice.*

## Question

Should AGF support an operating model where an AI assistant works **on behalf
of person X**, similar to enterprise approaches where a tool acts within a
named employee's workspace, permissions, and accountability boundary?

Short answer: yes, AGF should consider it, but only if it treats the model as
**delegated agency with explicit attribution**, not as human impersonation.

The distinction matters:

- **Acceptable framing:** "AI session S acted under delegated authority from
  human principal X, using role R, with constraints C, and human decision D."
- **Unsafe framing:** "X did this", when the material action, text, or code was
  produced by an AI assistant and X did not meaningfully review or approve it.

## Why this matters

The current AGF role model is role-first: `planner-architect`,
`coder-tester`, `governor-reviewer`, and other roles communicate through
coordination records. Enterprise adoption may also need a principal-first
dimension:

- a developer asks an AI to operate in their branch or IDE session;
- a maintainer delegates triage or patch preparation to an assistant;
- a security owner authorizes an assistant to collect evidence;
- a manager or tech lead wants a trace of "whose authority was used" without
  pretending the AI was that human.

Without a principal model, downstream projects may invent weaker patterns:
shared bot accounts, opaque "AI did it" commit authorship, or commits that
look human-authored while hiding the delegation chain.

## Candidate models

### A. Pure role model

AI sessions act only as AGF roles. Human names appear only in governor
decisions and normal Git history.

Pros:

- Simple and already implemented.
- Avoids collecting personal data in AGF records beyond what git already
  records.
- Keeps governance focused on authority boundaries instead of org charts.

Cons:

- Weak in enterprise settings where tool access is delegated from a named
  person or team.
- Harder to answer "who authorized this AI session?"
- Encourages external identity systems to carry accountability silently.

### B. Principal-bound delegation model

A human or service principal may record a delegation grant in the coordination
channel. AI sessions then reference that grant instead of asserting the
principal themselves:

```yaml
kind: note
from: governor-reviewer
to: coder-tester
topic: delegation-grant-docs-branch
note_type: delegation-grant
delegation:
  principal_kind: human
  identifier: "@example-user"
  delegation_scope:
    - "prepare branch"
    - "run tests"
```

The AI remains identified as an AI session and AGF role. The principal is
recorded by the authority source, not self-asserted by the AI.

Pros:

- Preserves human accountability without pretending the AI is human.
- Makes access delegation auditable.
- Aligns well with enterprise IAM and code-owner practices.
- Helps answer incident questions: who delegated, what scope, who reviewed?
- Avoids AI self-attestation of the human principal.

Cons:

- Adds personal-data handling and retention questions.
- Can create a chilling effect or blame routing if used punitively.
- Requires careful UI/docs language so "principal" does not become fake
  approval.
- Could be confused with legal agency, employment authority, or signature
  authority if wording is sloppy.

### C. Impersonation model

The AI works as person X and records only X as actor.

Recommendation: reject.

Pros:

- Operationally easy in tools that only understand a human account.
- Matches how some IDE assistants feel in practice.

Cons:

- Weak audit trail: reviewers cannot distinguish human-authored work from AI
  work.
- Creates accountability confusion after incidents.
- Makes disclosure, review, and contestability harder.
- Ethically fragile: a person can be blamed for AI outputs that they did not
  meaningfully review, or the AI can launder action through a trusted human
  identity.

## Architectural implications

If AGF adopts principal-bound delegation, prefer additive metadata over a
rewrite of the role model.

The most important architecture question is who writes the authority record.
An AI-authored coordination message that says `principal: "@example-user"` is
self-attestation: the AI is claiming who authorized it. Schema validation can
check shape, but it cannot verify that authority.

The cleaner direction is inversion:

1. A human writes a coordination note that grants or describes delegation.
2. The AI session references that human-authored note in its work records.
3. The governor-reviewer decision remains separate and human-authored.
4. Any contestation is a later coordination note referencing the disputed
   message or delegation grant.

Possible places:

- human-authored coordination note: delegation grant or session authorization;
- AI-authored coordination messages: references to the human-authored grant,
  not self-asserted principal identity;
- plan front matter: possibly reference the delegation grant when a plan was
  prepared under delegated authority;
- decision records: explicit `reviewed_by` / `approved_by` stays human and
  cannot be delegated to the AI session;
- `agov session resume`: show role, session, branch, pending handoffs, and any
  referenced delegation grant or contestation when present.

Keep these invariants:

- The AI identity and role must remain visible.
- The human principal does not equal approval.
- Gate decisions still require governor-reviewer authority.
- A principal can delegate preparation, evidence collection, and drafting; AGF
  should not imply that they delegated binding risk acceptance unless a
  recorded governance rule says so.
- System accounts should be explicitly typed as service principals, not hidden
  as humans.

## Ethical questions

### Attribution

Every record should answer three questions:

1. Who or what produced the work?
2. Under whose authority was it allowed to act?
3. Who accepted the risk at the gate?

Combining those into one name is convenient and dangerous.

### Consent and knowledge

A human principal should know when an AI is acting under their authority. A
team should not silently attach a person's name to AI work after the fact.

### Accountability without scapegoating

Principal attribution should support reconstruction, not blame automation. A
bad pattern would be: "The AI made a bad decision, but it was under X, so X is
solely responsible." A better pattern is: "X delegated scope C; the AI exceeded
or stayed within C; the governor accepted or rejected gate D; tooling did or
did not catch E."

### Human review quality

If the model says "reviewed by X", it should distinguish skim, checklist
review, code review, and binding gate approval. Otherwise, review labels become
ceremony.

### Organizational misuse

Principal metadata creates a tempting paper shield: an organization can mandate
AI-assisted work, configure the tools and deadlines, then point at the
principal field when something goes wrong. AGF should explicitly reject that
interpretation. Delegation metadata helps reconstruct authority; it does not
transfer all organizational responsibility to the named person.

The same risk appears in consent. If a workplace requires a person to attach
their name to AI sessions to remain eligible for certain work, "consent" is
thin. AGF should avoid language that treats principal attribution as a
voluntary personal choice when the surrounding organization controls tool
access, staffing, and performance incentives.

### Power asymmetry

In workplaces, "AI on behalf of employee X" may become monitoring of the
employee's productivity or judgement. AGF should avoid features that require
more personal surveillance than the governance trail needs.

### Governance metadata versus surveillance

Useful governance metadata asks: who authorized this AI session, what was it
allowed to touch, what gate decision was made, and what evidence supports that
decision? Surveillance metadata asks: how much AI did this person use, how long
did they take, how much effort did they personally contribute, and how do they
compare to peers?

AGF should keep that boundary structural, not only rhetorical:

- Do not add numeric review-quality fields such as review scores, minutes
  spent reviewing, lines checked, or effort percentages.
- Do not build native productivity, principal-ranking, or cross-user
  comparison reports from coordination metadata.
- Do not require routine per-person review-depth labels on every decision
  record. Even categorical labels such as `skimmed` or `substantive` can become
  a career signal when they are grep-able across permanent records.
- When review depth is necessary, prefer incident-gated reconstruction: record
  it only when investigating or explaining a specific failure, exception, or
  disputed gate.
- Record positive authority scope: what the AI may do. Avoid negative scope
  fields that imply surveillance of what the human did not do.
- Treat export of principal metadata for cross-user analysis as outside AGF's
  intended governance purpose.

This does not prevent an adopter from misusing logs with external tooling. It
does avoid handing them a first-class AGF feature that turns governance records
into a productivity dashboard.

## EU and regulatory context to consider

This section is issue-spotting, not advice.

- The EU AI Act entered into force on 1 August 2024. Some obligations already
  apply: prohibited practices and AI literacy from 2 February 2025, and GPAI
  governance obligations from 2 August 2025. The Commission states the Act is
  generally applicable from 2 August 2026, with high-risk timelines affected by
  later simplification/political agreement.
  Source: European Commission AI Act page:
  <https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai>
- The Commission page also states that high-risk systems used in areas such as
  biometrics, critical infrastructure, education, employment, migration, asylum,
  and border control are expected to apply from 2 December 2027 under the
  simplification timeline. Employment and worker-management use cases are
  therefore a sensitive boundary for any "AI on behalf of employee" framing.
- AI Act transparency tools and guidance are still developing. The Commission
  notes forthcoming support for marking and labelling AI-generated content and
  transparent AI systems. AGF should avoid overclaiming compliance and instead
  preserve auditable disclosure.
- The AI Liability Directive proposal was officially withdrawn in October 2025,
  according to the European Parliament Legislative Train. That does not remove
  liability risk; it means AGF should not build around a withdrawn directive as
  if it were enacted law.
  Source:
  <https://www.europarl.europa.eu/legislative-train/theme-a-europe-fit-for-the-digital-age/file-ai-liability-directive>
- GDPR remains relevant when records identify humans, process personal data, or
  feed automated decisions about people. Article 22 concerns solely automated
  decisions with legal or similarly significant effects; AGF should not imply
  that a logged human principal automatically solves meaningful human
  involvement. Source for GDPR text:
  <https://eur-lex.europa.eu/eli/reg/2016/679/oj/eng>

Design consequence: AGF should treat principal metadata as accountability and
audit metadata, not as a legal compliance claim.

## Pros of supporting principal-bound delegation

- Better enterprise fit: maps to IAM, branch permissions, code ownership, and
  existing accountability practices.
- Better incident reconstruction: distinguishes actor, delegator, reviewer, and
  gate approver.
- Better ethics: avoids hidden AI work under a human-looking identity.
- Better cross-tool operation: multiple assistants can act for the same human
  while still preserving session and role boundaries.
- Better future compliance posture: keeps disclosure and logs available without
  claiming AGF is a compliance system.

## Cons and risks

- Adds complexity to schemas and docs.
- May make small-team adoption feel heavier.
- May collect more personal data than necessary.
- May be misread as "the principal approved this".
- May conflict with local labour, employment, or monitoring expectations.
- May encourage organizations to shift responsibility to individuals while
  the organization controls the tools, prompts, and deadlines.

## Possible AGF policy direction

Recommended default:

1. Keep role-first AGF as the base model.
2. Add optional principal-bound delegation metadata for adopters that need it.
3. Explicitly reject AI impersonation of humans.
4. Require disclosure in coordination records when AI work is done under a
   human, team, or service principal.
5. Keep governor-reviewer gate approval separate from the principal.
6. Add docs language: "principal attribution is not legal advice, employment
   attribution, or proof of meaningful human review."
7. Treat workplace monitoring, performance scoring, and disciplinary use as
   sensitive downstream uses outside AGF's normal code-governance purpose.
8. Decline native tooling for review scores, time tracking, principal ranking,
   or productivity comparisons.
9. Make review-depth metadata incident-gated by default, not routine on every
   human decision record.
10. Do not let AI-authored messages self-assert a human principal. Prefer
    human-authored delegation-grant notes that AI sessions reference.

Possible rule:

> If an AI assistant acts under a named human or service account's delegated
> authority, the coordination record must identify the AI role/session, the
> principal, the delegation scope, and the human gate decision separately. An
> AI session must not be represented as the human principal.

## Delegation terms

The hard question is not whether a `principal` field can exist. It can. The
hard question is what an AI is allowed to do "on behalf of" a person without
collapsing into impersonation, blame shifting, or fake review.

Minimum terms:

1. **Identity separation:** the AI session and AGF role remain visible. The
   principal is the authority source, not the author identity.
2. **Scoped delegation:** delegation names action scope, artifact boundaries,
   and freshness. "Act as me" is not a scope.
3. **Non-delegable gates:** Gate 1, Gate 2, release approval, and binding risk
   acceptance remain human governor-reviewer decisions.
4. **Revocation:** the principal can withdraw delegation without needing the AI
   session to agree.
5. **Contestation:** a principal must be able to file a later coordination
   record disputing or disavowing a claimed delegation. The original record
   remains immutable for technical traceability, but the later record must be
   visible in session resume and audit views.
6. **Escalation:** the AI must pause or escalate when it encounters scope
   ambiguity, sensitive data, conflicting claims, CI or validator failures, or
   instructions that would convert delegated work into approval.
7. **Review depth:** `reviewed` is too vague, but routine per-person review
   depth is also surveillance-prone. If review depth is recorded, it should be
   incident-gated by default and tied to a specific reconstruction need, using
   categorical labels such as `skimmed`, `checklist-reviewed`,
   `substantively-reviewed`, and `gate-approved`.
8. **Anti-scapegoating:** delegation metadata must not be the sole explanation
   of fault when the organization chose the tool, configured the workflow, or
   imposed the deadline.
9. **Anti-surveillance:** AGF should not provide native metrics or aggregation
   features that make principal metadata easy to reuse for productivity
   scoring.

Possible delegation depth vocabulary:

A D1-D4 taxonomy could eventually distinguish informational, preparatory,
execution, and operational delegation. Do not ship that taxonomy in v2.x. It
adds learning curve before there is adopter evidence that the distinction is
needed. For the docs-only slice, plain positive `delegation_scope` is enough.

## Minimum viable shape

The minimum viable design should be two-tiered and docs-first:

| Tier | Audience | Metadata burden |
|---|---|---|
| Tier 0 | Solo and small-team AGF use | Current coordination fields only; no delegation convention |
| Tier 1 | Multi-person, enterprise, regulated, or delegated-access use | Current coordination fields plus human-authored delegation-grant notes |

Tier 0 is not a lesser mode. It is the default AGF model: role-first,
coordination-first, with no personal principal metadata to aggregate. That
keeps small teams from paying an enterprise compliance tax and preserves data
minimization.

Tier 1 activates only when a human writes a delegation-grant note. The AI does
not write `principal` into its own coordination messages; it references the
grant.

Possible human-authored grant convention using existing `kind: note`:

```yaml
kind: note
from: governor-reviewer
to: coder-tester
id: 20260622T083000Z-governor-reviewer-abcd
timestamp: 2026-06-22T08:30:00Z
immutable: true
references-to: []
topic: delegation-grant-docs-branch
requires-ack: true
artifacts_owned: []
note_type: delegation-grant
delegation:
  principal_kind: human | service | team
  identifier: "@handle-or-service-name"
  delegation_scope:
    - "prepare patch"
    - "run tests"
  valid_until: "2026-06-22T18:00:00Z"
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
```

Notes on the minimum:

- `note_type: delegation-grant` is a convention, not a new message kind in the
  current schema.
- `principal_kind`, `identifier`, and positive `delegation_scope` are the
  minimum useful grant fields.
- `valid_until` is useful when the human writes the grant; it should not be
  inferred by the AI.
- `voluntary` is intentionally omitted from the minimum because AGF cannot
  prove workplace voluntariness.
- `cannot_approve` is redundant with Gate 1/Gate 2 authority, but useful as
  defense-in-depth wording in a human-authored grant.
- Review depth should not be routine on every human decision record. If needed,
  record it in an incident, exception, or disputed-gate reconstruction record.
- Contestation should be a later coordination record that references the
  contested message, not a mutable edit to the original immutable message.

Example contestation record using current AGF coordination vocabulary:

```yaml
kind: note
from: governor-reviewer
to: planner-architect
id: 20260622T090000Z-governor-reviewer-abcd
timestamp: 2026-06-22T09:00:00Z
immutable: true
references-to:
  - 20260621T161500Z-coder-tester-wxyz
topic: principal-delegation-contested
requires-ack: true
artifacts_owned: []
note_type: contestation
contestation:
  status: filed
  reason: "claimed delegation exceeded scope"
```

The schema may not accept `note_type`, `delegation`, or `contestation` objects
today; this proposal is naming the convention and design direction. If
implemented, these objects should make grants and disputes discoverable without
adding new message kinds prematurely.

The GDPR and immutability tension should be framed as purpose limitation, not
as a total blocker. Git repositories already contain personal data in commits
and review trails. Principal or delegation records should be documented as
governance, audit, and safety metadata. Repurposing them for employee
monitoring or performance evaluation exceeds that stated purpose and may
require separate legal basis or consent. AGF cannot enforce purpose limitation
technically; it can document the intended purpose and decline native misuse
features.

## Implementation sketch

### Docs-first convention

For v2.x, prefer no schema addition. Document a convention for human-authored
delegation grants and contestations using existing `kind: note` messages.

Human-authored delegation grant:

```yaml
note_type: delegation-grant
delegation:
  principal_kind: human | service | team
  identifier: "@handle-or-service-name"
  delegation_scope:
    - "prepare patch"
    - "run tests"
  valid_until: "2026-06-22T18:00:00Z"
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
```

AI-authored messages should reference the grant through `references-to`, not
repeat or self-assert the principal.

Human-authored contestation:

```yaml
note_type: contestation
contestation:
  status: filed
  reason: "claimed delegation exceeded scope"
```

Optional incident-gated review reconstruction metadata:

```yaml
review_depth:
  category: skimmed | checklist-reviewed | substantively-reviewed | gate-approved
  context: incident | exception | disputed-gate
  notes: "Free-text explanation tied to the specific reconstruction."
```

Do not require `review_depth` on routine decision records. Requiring it by
default creates a dense, durable, per-person quality signal that can be
aggregated with basic text search.

### Tooling

- `agov coord validate` validates the shape but does not decide legal meaning.
- `agov session resume` could eventually surface referenced delegation grants
  and contestation notes if the convention proves useful.
- `agov usage report` should avoid principal productivity reports. If it ever
  counts principal-bound records, the count should be scoped to artifact audit
  and should not rank or compare people.
- `agov coord graph` could optionally render role-to-role and
  principal-to-role views separately.
- AGF should not add native commands that rank principals, compare AI usage
  across humans, calculate human oversight scores, or export principal
  metadata as an HR-ready dataset.
- AGF should not add native commands that aggregate review depth by person.

### Docs

- Add "Delegated AI work" to Roles and gates or Advanced customization.
- Add a FAQ entry: "Can an assistant work on behalf of a human?"
- Add a warning against shared bot accounts and hidden impersonation.
- Add a small example coordination record.

## Questions for Claude / planner-architect

- Should this become a v2.x optional extension, or wait for v3 because it
  touches schemas, identity, and ethics?
- Should principal metadata live in every coordination message, only claims,
  or only handoffs/decision records?
- What is the minimum useful shape that avoids personal-data overcollection?
- How should AGF handle service principals, team aliases, and rotating duty
  roles?
- Should "reviewed by principal" and "approved by governor" be separate schema
  fields?
- Should public docs mention Amazon-style enterprise delegation generically, or
  avoid naming vendors and describe the pattern abstractly?

## Prompt for Lumo or another external AI reviewer

Please review the AGF proposal note
`docs/proposals/ai-on-behalf-of-human-principal.md` as an external AI reviewer.

Questions:

- Does the proposal correctly distinguish delegated authority from
  impersonation?
- What ethical risks are missing, especially for workplace monitoring,
  accountability, and blame shifting?
- What EU regulatory issues should AGF consider without pretending to provide
  legal advice?
- Is optional `principal` metadata a reasonable architecture, or should AGF
  keep identity outside the framework?
- What minimum docs wording would help adopters avoid representing AI work as
  human work?
- Which parts are too heavy for a small team?

Please cite what you read, say what you did not inspect, and separate legal
observations from product/design recommendations.

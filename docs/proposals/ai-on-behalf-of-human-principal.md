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

Every AI session may optionally record a human or service principal:

```yaml
principal:
  type: human
  handle: "@example-user"
  authority_source: "GitLab Maintainer on project X"
  delegation_scope: "prepare branch and tests; no merge approval"
  reviewed_by: "@example-user"
```

The AI remains identified as an AI session and AGF role. The principal is the
person or accountable service identity under whose authorization it worked.

Pros:

- Preserves human accountability without pretending the AI is human.
- Makes access delegation auditable.
- Aligns well with enterprise IAM and code-owner practices.
- Helps answer incident questions: who delegated, what scope, who reviewed?

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

Possible places:

- coordination message front matter: optional `principal:` object;
- plan front matter: optional `principal` or `delegation` section for the AI
  session preparing the plan;
- claim messages: principal-bound claims for branch or artifact ownership;
- decision records: explicit `reviewed_by` / `approved_by` stays human and
  cannot be delegated to the AI session;
- `agov session resume`: show role, session, branch, pending handoffs, and
  principal/delegation scope when present.

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
- Prefer categorical review labels over quantitative scores, and document that
  categories are audit context, not performance ratings.
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
7. **Review depth:** `reviewed` is too vague. Records should distinguish at
   least `unreviewed`, `skimmed`, `checklist-reviewed`, `substantively-reviewed`,
   and `gate-approved`.
8. **Anti-scapegoating:** delegation metadata must not be the sole explanation
   of fault when the organization chose the tool, configured the workflow, or
   imposed the deadline.
9. **Anti-surveillance:** AGF should not provide native metrics or aggregation
   features that make principal metadata easy to reuse for productivity
   scoring.

Possible delegation depth vocabulary:

| Level | Name | AI may do | Human still must do |
|---|---|---|---|
| D1 | Informational | Read, search, summarize, answer questions | All edits, gates, and approvals |
| D2 | Preparatory | Draft plans, patches, tests, and evidence | Review drafts and approve gates |
| D3 | Execution | Commit to a branch, open MRs, respond within scope | Approve gates, merge, release |
| D4 | Operational | Trigger CI and limited non-production operations | Approve gates, production changes, security decisions |

AGF should be conservative: D2 is the safest default for most teams. D3 and D4
should require explicit scope and stronger coordination evidence. "After the
fact" supervision is acceptable only for low-risk reconstruction; it cannot
replace gate approval before merge.

Example:

```yaml
principal:
  kind: human
  identifier: "@example-user"
  authority_source: "explicit session instruction"
  delegation_depth: D2
  delegation_scope:
    actions: ["prepare patch", "run tests", "draft review notes"]
    artifacts: ["agf/docs/**", "docs/proposals/**"]
    valid_until: "2026-06-22T18:00:00Z"
  supervision: checkpoint
  voluntary: true
  contested_by_principal: false
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
  review_status: unreviewed
```

This shape is intentionally heavier than the minimum schema. Small teams may
only need `kind`, `identifier`, `delegation_scope`, and `cannot_approve`, but
the docs should name the richer terms so adopters understand what they are
choosing not to record.

## Minimum viable shape

The minimum viable design should be two-tiered:

| Tier | Audience | Metadata burden |
|---|---|---|
| Tier 0 | Solo and small-team AGF use | Current coordination fields only; no `principal` block |
| Tier 1 | Multi-person, enterprise, regulated, or delegated-access use | Current coordination fields plus an optional `principal` block |

Tier 0 is not a lesser mode. It is the default AGF model: role-first,
coordination-first, with no personal principal metadata to aggregate. That
keeps small teams from paying an enterprise compliance tax and preserves data
minimization.

Tier 1 activates only when a record names a principal. If `principal` is
present, the minimum useful block is:

```yaml
principal:
  kind: human | service | team
  identifier: "@handle-or-service-name"
  delegation_scope:
    - "prepare patch"
    - "run tests"
  valid_until: "2026-06-22T18:00:00Z"
  voluntary: true | false | unknown
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
```

Notes on the minimum:

- `kind`, `identifier`, and positive `delegation_scope` are required if the
  `principal` block exists.
- `valid_until` is recommended but not mandatory; it avoids indefinite
  delegation without forcing every small record to carry expiry metadata.
- `voluntary` is optional and defaults semantically to `unknown`; AGF cannot
  prove workplace voluntariness.
- `review_status` should live on human review or decision records, not on every
  AI-authored coordination message.
- Contestation should be a later coordination record that references the
  contested message, not a mutable edit to the original immutable message.

Example contestation record using current AGF coordination vocabulary:

```yaml
kind: review
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
contestation:
  status: filed
  reason: "claimed delegation exceeded scope"
```

The schema may not accept this exact `contestation` object today; this proposal
is naming the design direction. If implemented, the object should mark the
record as disputed for audit reconstruction without deleting history or
deciding liability.

## Implementation sketch

### Schema addition

Optional `principal` object on coordination messages:

```yaml
principal:
  kind: human | service | team | none
  identifier: "@handle-or-service-name"
  authority_source: "GitLab role, ticket, policy, or explicit instruction"
  delegation_depth: D1 | D2 | D3 | D4
  delegation_scope:
    - "prepare patch"
    - "run tests"
    - "draft review notes"
  valid_until: "2026-06-22T18:00:00Z"
  supervision: realtime | checkpoint | after-the-fact
  voluntary: true | false | unknown
  contested_by_principal: true | false
  cannot_approve:
    - "Gate 1"
    - "Gate 2"
    - "release tag"
  review_status: unreviewed | skimmed | checklist-reviewed | substantively-reviewed | gate-approved
```

### Tooling

- `agov coord validate` validates the shape but does not decide legal meaning.
- `agov session resume` surfaces the active principal/delegation scope when
  present, and highlights later contestation or disavowal records that
  reference the same delegation.
- `agov usage report` should avoid principal productivity reports. If it ever
  counts principal-bound records, the count should be scoped to artifact audit
  and should not rank or compare people.
- `agov coord graph` could optionally render role-to-role and
  principal-to-role views separately.
- AGF should not add native commands that rank principals, compare AI usage
  across humans, calculate human oversight scores, or export principal
  metadata as an HR-ready dataset.

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

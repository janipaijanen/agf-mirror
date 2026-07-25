---
id: split-person-gates-adopter-faq
status: pending-gate1
topic: split-person-gates-adopter-faq
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "the governor supplied one bounded FAQ question; implementation adds a clarification consistent with the existing one-governor-at-a-time and rotation guidance, without changing role authority or gate contracts"
  - role: infosec-sentinel
    status: not-applicable
    reason: "scope is public explanatory prose about authorized human gate ownership; no identity system, credential, permission, schema, tooling enforcement, or sensitive information is introduced"
---

# Plan: explain split-person Gate 1 and Gate 2 operation

## Trigger and gap

Issue #66 records a direct adopter question: what happens if different people
perform Gate 1 and Gate 2?

The Adopter FAQ currently recommends one governor at a time and honestly says
that full multi-governor operation is not yet validated. The roles-and-gates
page also permits a named primary, backup, or rotation. Neither page gives a
direct answer for the narrower sequential pattern: one authorized human
approves the plan, then another authorized human reviews the implementation.

## User outcome

A team can use two people without inventing parallel authority. The reader
understands that the `governor-reviewer` role is the authority boundary, that
different people may hold it at successive gates, and that each change still
has only one final decision-maker at a time.

## Frozen scope

Add one FAQ entry to `agf/docs/site/docs/start/adopter-faq.md`, near “Who is the
governor?” or the mixed-team questions:

1. Answer **yes**, provided the arrangement is explicit, authorized, and
   sequential.
2. State that Person A may own Gate 1 and Person B Gate 2; this can add an
   independent implementation review, but AGF does not require two people.
3. Give the minimum safe checklist:
   - record who owns each gate and the applicable repository/change/time scope;
   - make the Gate 1 decision, exact plan, conditions, and accepted risks
     visible to the Gate 2 reviewer;
   - require Gate 2 to compare the diff against that exact plan and evidence;
   - return scope changes to Gate 1 rather than letting Gate 2 silently rewrite
     the plan;
   - name one primary governor/project authority to resolve disagreement;
   - prevent either approver from approving implementation they personally
     performed while acting as coder-tester.
4. Preserve the evidence boundary: AGF v2's validated minimum is one governor
   at a time. Split-person gates are a team operating pattern/pilot, not a
   universally validated organizational design. Encourage a learning record
   when adopters try it.
5. Link to the canonical roles-and-gates guidance rather than duplicating its
   rotation, delegation, and conflict-of-interest detail.

## Claim and authority boundary

This is documentation clarification only. It does not split the shipped
`governor-reviewer` role, create a second decision role, change schemas, grant
AI approval authority, or enable concurrent/multi-decider gates. The framework
anti-pattern remains: a change must not wait on or accept two simultaneous
final approvals with unclear precedence.

Fact-checker reviews the exact future prose before Gate 2 against:

- `agf/docs/site/docs/concepts/roles-and-gates.md`;
- `agf/agents/governor-reviewer.yaml`;
- `agf/rules/workflow/deputy-governor-rule.md`; and
- the FAQ's existing “one governor at a time” and multi-team evidence limits.

## Acceptance evidence

- FAQ directly answers the question in its first sentence.
- Wording distinguishes sequential role rotation from simultaneous governors.
- The checklist preserves exact-plan traceability, conflict-of-interest
  separation, and one escalation owner.
- No statement presents split-person gates as externally validated or required.
- Fact-checker records ADOPT, ADAPT, or REJECT for the exact implementation
  text; every ADAPT/REJECT finding is resolved before Gate 2.
- Strict MkDocs, link checks, `agov validate adopter-docs`, umbrella
  `agov validate`, rendered narrow/desktop inspection, and `git diff --check`
  pass.

## Explicit exclusions

- No change to role definitions, manifests, rules, schemas, validators,
  delegation artifacts, or coordination fields.
- No design for multiple simultaneous governors, quorum, four-eyes approval,
  committee voting, or cross-repository authority.
- No claim that different people are always safer or more efficient.
- No change to the pending `agov` day-one FAQ plan or framework-map figure plan.
- No modification of the frozen v2.4.2 release.

## Gate requests

- **Gate 1:** governor-reviewer approves or blocks this clarification plan.
- **Gate 2:** governor-reviewer reviews the exact FAQ SHA after fact-checking and
  validation evidence are complete.

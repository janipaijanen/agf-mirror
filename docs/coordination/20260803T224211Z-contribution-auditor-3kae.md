---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260803T224211Z-contribution-auditor-3kae
timestamp: 2026-08-03T22:42:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T214947Z-contribution-auditor-qqj4
  - 20260803T223946Z-contribution-auditor-dsqg
topic: rc1-grok-learnings-renewed-review
requires-ack: true
artifacts_owned: []
---

# Renewed review of !1087: two corrections remain internally inconsistent

Scope: MR !1087 exact head
`23ef9e56a3dc30459a7f20b672201ac0c2e94aea`, including its dependency
commit `d63238de40ca6ceb5e3699051ec501f931ec4a15` from !1086.
Coverage: COMPLETE for the three corrected learning entries and the new
Goodhart entry.
Recommendation: BLOCK.
Blocking findings: 3 (one inherited dependency, two delta findings)
Advisory findings: 1
Open findings with accepted risk: 0
Authority: contribution-auditor recommendation only; the governor decides.

`B-LRN-1` is resolved: `nq7p` now correctly distinguishes standing role
activation, task-scope judgment, and binding governor authority.

`B-LRN-3` is resolved: the Goodhart claim is now a separate proposed learning
with its own evidence boundary, and the unpublished six-lens assertion was
removed.

The prevalence portion of `B-LRN-4` is resolved: `hv3k` and `t8mz` now bound
their observations to this project and the tested sandbox conditions.

## Blocking findings

### B-1087-DEP-1 — the branch inherits blocked !1086

!1087 is stacked on `d63238de`, which remains blocked by `dsqg` on the
unchanged CI-enforcement overclaim and unresolved `hv3k` dependency. Therefore
this exact head cannot merge even where its own delta is acceptable.

Resolution: correct !1086 first, then rebase the learning delta on the accepted
`gt3x` head. Re-run learning and infosec validation at the rebased exact head.

### B-LRN-2-R2 — `nq7p` still calls Grok's agreement independent

The opening sentence still says the finding was “independently confirmed by
Grok when the point was put to it.” The later correction correctly says the
opposite: Grok agreed after the specific point was supplied through the relay;
there was no blinded or independent protocol. Both statements cannot stand in
one record.

Resolution: replace the opening with the later precise claim: Grok agreed when
the point was put to it, but that agreement is relayed, unauthenticated, and
not independent corroboration. Keep the direct repository reproductions as
evidence for claim correctness, not speaker identity.

### B-LRN-4-R2 — `t8mz` assigns this project a CI gate-enforcement tier it does not run

The corrected text says `CI-blocking-if-configured` is what this project's
hosted pipeline provides, citing `sjqm` finding 4. But that finding now says the
pipeline does **not** require or authenticate Gate 1/Gate 2 decisions; it blocks
merge on pipeline success and validates record shape. Running structural
validation in CI changes the technical delivery context, not the authority
assurance of “Gate 1 approved.” Calling it the next gate-enforcement tier
reintroduces the conflation corrected in `B-EVAL-1`.

Resolution: describe the observed project configuration as “structural checks
executed in merge-blocking CI; gate authority still procedurally asserted.” If
an enforcement-tier taxonomy is retained, define tiers by the assurance they
actually add (for example approval-record presence, role-string constraint,
authenticated actor, protected immutable provenance), not merely by whether
the same shape check ran locally or in CI.

## Advisory

### A-GOODHART-R2 — evidence is candid but not durable

The split Goodhart entry is appropriately `status: proposed`, `severity:
minor`, `single-adopter`, and explicit that it is one session rather than an
independent reproduction. Its claimed observed conversational pattern still
has no published primary transcript. Retain that limitation; do not promote
the entry or turn it into a normative rule without independently inspectable
evidence or recurrence.

## Renewal boundary

Renewed review may be bounded to the two changed paragraphs above, the rebased
dependency identity, and validation output. The corrected task-scope and split
Goodhart sections need no further rewrite unless they change.

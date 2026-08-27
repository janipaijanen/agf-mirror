---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260802T222149Z-contribution-auditor-b72h
timestamp: 2026-08-02T22:21:49Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260802T204834Z-contribution-auditor-ah7j.md
  - 20260802T214224Z-contribution-auditor-jjqt.md
  - 20260802T215208Z-contribution-auditor-ph5j.md
  - 20260802T220154Z-governor-reviewer-sejb.md
  - 20260802T221709Z-coder-tester-kehq.md
topic: v250-b2-regulatory-renewed-review
requires-ack: true
artifacts_owned: []
---

# Renewed contribution audit: B2 closure and regulatory gap analysis

Scope:

- MR !948 exact head `935a449b673ad6935d493d765394319eb6b79e5a`;
- MR !975 exact head `b56b5b1b46019e862f4c92df0bb6e753e279e42f`;
- MR !967 exact head `bdfc4b02b42804348107b2f781430e70fa603807`;
- governor decision record `20260802T220154Z-governor-reviewer-sejb.md` at
  MR !974 exact head `a8f5235c3fb0ccf17a78f909d77cecad49030fb4`.

Coverage: COMPLETE for the bounded renewed-review scope in handoff
`20260802T221709Z-coder-tester-kehq.md`: B2/B-DOC-5, the companion plan
amendment, and B-REG-1 through B-REG-6. Previously resolved B1, B3, B4,
B-DOC-1 through B-DOC-4, and B-DOC-6 were not reopened.

Recommendation:

- CLEAR B2/B-DOC-5 on MR !948 and the companion plan amendment on MR !975;
- BLOCK MR !967 Gate 1 solely on renewed finding B-REG-1-R2;
- B-REG-2 through B-REG-6 are resolved at the reviewed head.

Blocking findings: 1

Advisory findings: 0

Open findings with accepted risk: 0

Authority: recommendation only; governor-reviewer decides Gate 1, Gate 2,
merge order, and merge disposition.

Evidence: all reviewed exact-head pipelines succeeded: MR !948 pipeline
`2725584508`, MR !975 pipeline `2725586405`, MR !967 pipeline `2725598117`,
and MR !974 pipeline `2725581804`. Each reviewed range passes
`git diff --check`. The implementation, both operator-facing documentation
pages, the plan amendment, and the governor decision record all use the exact
cutoff `2026-08-02T22:01:54Z`.

## B2 and B-DOC-5 — resolved

Check: the implementation and documentation reproduce the governed cutoff

Check status: reviewed

Outcome: pass

Requirement strength: mandatory

Basis: governor decision record
`20260802T220154Z-governor-reviewer-sejb.md`, verifiable-claims rule, and
implementation-honesty rule

Finding state: resolved

Risk disposition: none

MR !948 sets the default cutoff to the decision record's own timestamp,
`2026-08-02T22:01:54Z`. Both
`agf/docs/site/docs/concepts/coordination-protocol.md` and
`docs/coordination/README.md` state that same boundary and provenance. MR !975
amends decision (c) in `docs/plans/v250-rc1-contract-core.md` to the same
contract and accurately explains why the former merge-timestamp wording was
not implementable deterministically.

The governor should land the decision record and plan amendment before MR !948,
or use an equivalent controlled merge sequence, so the implementation never
lands on `main` ahead of its governed contract. This is a sequencing condition,
not a defect in the reviewed artifacts.

## B-REG-1-R2 — capability and mapping maturity remain coupled

Check: each maturity axis follows the canonical maturity vocabulary and rates
only its own unit of assessment

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: B-REG-1-R2

Impact: material

Enforcement: blocking

Basis: `implementation-honesty-rule` and
`agf/docs/site/reference/maturity-vocabulary.md`

Finding state: open

Risk disposition: none

The rewrite adds separate Capability and Mapping columns, but its local legend
changes the meaning of canonical Level 4 to include a capability that is
"shipped and tested" merely because no requirement-specific mapping exists.
The canonical vocabulary instead distinguishes Level 4 Validated work in a
branch or checkout from Level 5 Deployable work in the supported distribution.
Absence of a requirement mapping belongs exclusively in the Mapping axis and
must not lower the Capability score.

That coupling is then applied in the table. Several shipped generic governance
artifacts receive Capability 4 because their regulation-specific mapping is
absent. AI-10 assigns the existing provenance/checksum capability Capability 3
because SLSA alignment is absent, and AI-11 assigns a shipped MCP allowlist
Capability 3 because it is narrower than Agent-BOM. In each case, the missing
or narrow crosswalk describes Mapping maturity, not the lifecycle maturity of
the underlying capability.

Resolution condition: use the canonical five-level ladder unchanged on both
axes. Rate each named generic capability solely by its own lifecycle (including
Level 5 when it is tested, documented, and present in a supported
distribution), and rate the requirement-specific adapter or crosswalk solely
on the Mapping axis. Do not change one score because of the other. Where one
row bundles capabilities at different lifecycle levels, split the components
or report their individual levels rather than inventing a blended number.

## B-REG-2 through B-REG-6 — resolved

Check status: reviewed

Outcome: pass

Requirement strength: mandatory

Finding state: resolved

Risk disposition: none

The corrected head now:

- labels the Article 26 relationship as a structural analogy and does not
  claim AGF development records are adopter runtime logs or retention controls;
- describes `security-hardening-rule.md` as shipped generic normative guidance,
  not a mechanically enforced CRA policy;
- narrows corpus scope, declares incomplete primary verification, includes the
  Finland AI Act supervision item, and names unmerged MR !962 dependencies;
- supplies an explicit evidence-status field for every row, all Theoretical;
- distinguishes the active WIMSE working group from the AIMS presentation and
  proposal, and distinguishes the pending threat-model plan from a deployed
  threat model.

## Required next sequence

1. Planner-architect corrects B-REG-1-R2 on MR !967 without marking the plan
   approved, then publishes a new exact head and requests bounded renewed
   contribution audit.
2. Governor-reviewer controls the MR !974, MR !975, and MR !948 merge sequence
   and makes the binding Gate 2 decision.
3. Governor-reviewer makes no Gate 1 approval for MR !967 until B-REG-1-R2 and
   the separately required fact-checker and infosec dispositions are clear.

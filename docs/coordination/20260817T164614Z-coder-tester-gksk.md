---
kind: note
from: coder-tester
to: planner-architect
id: 20260817T164614Z-coder-tester-gksk
timestamp: 2026-08-17T16:46:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260817T163300Z-coder-tester-kmm2
  - 20260817T150743Z-planner-architect-spgp
topic: dd-action-plan-disposition-framework
requires-ack: true
artifacts_owned: []
---

# Apply explicit disposition and version tests to the DD recommendation

The governor has clarified that the eventual joint recommendation must not be
a catalogue of agreeable improvements. Your critical response to `kmm2` must
try to disprove or narrow each proposal and then assign one explicit
disposition:

- **discard** — factually unsupported, immaterial, duplicated, or outside AGF's
  product responsibility; state the evidence or boundary that justifies
  removal;
- **park** — valid but not a current release commitment; state the trigger for
  reconsideration and the owner of that trigger;
- **accept** — include in a named release or non-release workstream, with a
  rationale and an observable completion condition.

Do not use `accept` without a version/workstream destination, or `park` without
a re-entry condition. Where evidence is insufficient, classify the uncertainty
instead of converting it into a remediation commitment.

## Proposed transparency and tracking model to challenge

I propose GitLab Issues as the public tracking surface, not as gate authority:

1. One umbrella **Due-diligence response programme** Issue holds the approved
   outcome, disposition matrix, release mapping, and links to governed plans.
2. One child Issue exists per independently deliverable outcome, provisionally:
   licensing and inbound rights; security policy and dependency assessment;
   SBOM/signing/provenance; version-aware history classification; minimal
   adopter profile and pilot evidence; maintainer continuity; and
   privacy/retention boundaries.
3. Milestones group accepted engineering outcomes by release or phase.
4. A Gate 1 plan is created only when an accepted non-trivial outcome is
   activated. Issues expose work; coordination records and governor decisions
   authorize it. Do not create an Issue per file edit.

Challenge this structure directly. In particular, say whether an umbrella
Issue would improve traceability or create a second, stale roadmap; whether
any provisional child should be merged, split, discarded, or owned outside
GitLab; and whether opening Issues before governor acceptance would falsely
signal commitment. My current position is to open none until the joint
recommendation is reconciled and the governor accepts its scope.

## Proposed action-plan contents to challenge

The later action plan, if requested and authorized, should be decision-oriented
rather than a maximum-depth remediation dossier. I propose these sections:

1. **Executive decision** — adoption posture and current operating boundary.
2. **Finding disposition matrix** — source, verified fact, evidence strength,
   severity, discard/park/accept decision, rationale, owner, target
   version/workstream, and tracking link.
3. **Prioritized workstreams** — outcome, rationale, non-goals, accountable
   owner, dependencies, gate, release/phase, required evidence, success
   measures, residual risk, and rollback/exit condition.
4. **Version strategy** — patch only restores an existing promised contract;
   minor adds adopter-visible capability or obligation compatibly; major is
   reserved for incompatible semantics. Legal, organizational, or research
   work gets no artificial product version unless it changes shipped
   artifacts or supported contracts.
5. **Measurable success factors** — evidence such as licence consistency,
   monitored security intake, dependency disposition, reproducible SBOM and
   provenance verification, cold-start task completion, pilot governor effort,
   decision reconstruction, continued-use/abandonment data, and named
   continuity ownership. Select only measures that test accepted findings.
6. **Risk register** — trigger, likelihood/impact, mitigation, owner,
   verification, residual risk, and acceptance authority.
7. **Decision and evidence gates** — what must be true before pilot expansion,
   production reconsideration, or release activation.
8. **Transparency status** — Issue/MR/plan/coordination links and current state,
   with one canonical source for each kind of truth.

Please try to discard, park, or accept these sections too. Identify anything
that is bureaucracy without decision value, anything missing from an
enterprise/adopter perspective, and any success factor that is not realistically
measurable by this project.

## Output needed

Return one self-contained critical response that:

- reconciles `kmm2` and this supplement rather than merely agreeing;
- gives every material proposal a discard/park/accept disposition;
- maps each accepted item to a named version or non-release workstream;
- distinguishes engineering, governor, legal/organizational, and adopter
  ownership;
- states falsification criteria for the controlled-pilot recommendation; and
- identifies unresolved disagreements that require the governor, instead of
  manufacturing consensus.

This is analysis and recommendation work only. It authorizes no Issue creation,
roadmap mutation, implementation, release, or gate decision.

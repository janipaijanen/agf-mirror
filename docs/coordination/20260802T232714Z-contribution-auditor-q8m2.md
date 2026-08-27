---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260802T232714Z-contribution-auditor-q8m2
timestamp: 2026-08-02T23:27:14Z
immutable: true
references-to:
  - 20260802T232357Z-coder-tester-r6h5.md
topic: gitlab-project-page-description-review
requires-ack: false
artifacts_owned: []
---

# Review evidence envelope

Scope: MR !989 draft GitLab project description and topic additions at exact head `59507f5b708e562fe1f283415f4ee17952e932fa`
Coverage: COMPLETE
Recommendation: BLOCK description as drafted; CLEAR topic additions
Blocking findings: 2
Advisory findings: 1
Open findings with accepted risk: 0
Authority: advisory collaboration only; no live project setting is authorized by this record

Check: live-state inventory
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-PAGE-1
Impact: material
Enforcement: blocking
Basis: factual completeness of the before/after proposal
Finding state: open
Risk disposition: none
Resolution condition: include the existing headline in the current-state inventory before presenting or applying a replacement

The live description is not only the paragraph quoted in the handoff. It begins
with `AI Assisted coding with human accountability.` followed by that paragraph.

Check: canonical message integrity and product naming
Check status: reviewed
Outcome: fail
Requirement strength: mandatory
Finding: B-PAGE-2
Impact: material
Enforcement: blocking
Basis: `agf/rules/docs/marketing-tone-rule.md` message-integrity rule
Finding state: open
Risk disposition: none
Resolution condition: use a verbatim canonical descriptor, or link instead of synthesizing a new summary

Most proposed phrases are traceable, but the parenthetical `file-based
governance (named roles, two human-decided gates, a durable decision log)` and
the sentence assembled around it are a new synthesis, not a verbatim canonical
descriptor. The cited rule permits linking or a verbatim attributed quotation,
not a fresh paraphrase. `AGF` also makes a newcomer decode an acronym, while the
canonical H1 and descriptor use `agents-govern`.

Recommended replacement, copied verbatim from the established homepage/README
language (apart from joining its lines):

> AI coding with human accountability. AI assistants can produce changes you
> cannot reconstruct later. agents-govern makes AI-assisted software work
> auditable without adding a hosted service or runtime.

Check: proposed topics `rust` and `model-context-protocol`
Check status: reviewed
Outcome: pass
Requirement strength: recommended
Finding state: resolved
Risk disposition: none
Resolution condition: met

Both topics are accurate and absent from the current nine topics. Rust is the
supported tooling implementation language; `agov-mcp` is a documented local
stdio Model Context Protocol server. Adding both is CLEAR.

Advisory A-PAGE-1: leaving the avatar unset is a sound separation of concerns.
An avatar should wait for an approved design asset and explicit live-setting
authorization.

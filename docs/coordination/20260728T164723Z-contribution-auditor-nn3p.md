---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260728T164723Z-contribution-auditor-nn3p
timestamp: 2026-07-28T16:47:23Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T161947Z-contribution-auditor-pmjr
  - 20260728T163234Z-contribution-auditor-tqh2
topic: whole-system-product-assessment
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - synthesize a phased AGF and AGOV roadmap from the current product and portfolio evidence
    - obtain independent role-specific consultations before requesting a replacement Gate 1 roadmap
    - reconcile release-contract and portfolio-hygiene defects before assigning new feature work
  must_read:
    - GitLab issue 57 and blocked MR 661
    - GitLab issues 67 and 75 through 82
    - agf/docs/contracts/contract-stability.md
    - agf/docs/site/reference/tooling-conformance.md
    - agf/CHANGELOG.md and CHANGELOG.md
    - agf/agents/MANIFEST.yaml
  do_not_touch:
    - version files, tags, release branches, or product implementation before Gate 1
    - external scientific or promotional repositories before issue 77 is approved
    - .claude/settings.local.json
---

# Whole-system assessment: AGF and AGOV after v2.4.5

## Current standing

AGF is a credible deployable governance framework for one repository with one
accountable human governor. AGOV is a substantial packaged reference tool, not
just a validator prototype. Release v2.4.5 publishes framework archives,
Linux x86_64 and arm64 tooling, Windows x86_64 tooling, an OCI image, checksums,
and per-artifact provenance. Its tag pipeline passed all 17 jobs.

The released surface includes roles, two human gates, plans, append-only
coordination, learning records, rules, schemas, templates, assistant loaders,
release provenance, a broad Rust CLI, and a local MCP adapter. Documentation is
strongly audience-led across evaluators, adopters, operators, AI assistants,
reviewers, and contributors. The honest non-goals are also clear: no runtime,
hosted orchestrator, compliance certification, or enterprise federation.

The product is nevertheless ahead of its operational control plane. The next
release should not be another feature accumulation pass.

## Evidence requiring disposition

- GitLab has 82 work items, all of type Issue: 37 open and 45 closed. Only one
  open issue has a milestone. The Development board has no lists or workflow
  labels. Issues #32 and #37 describe commands/MCP capability already shipped,
  while remaining open.
- There are 46 open MRs, including 31 coordination-only MRs; five conflict and
  22 are unchecked. Completed review/release records remain mixed with active
  plans and implementations. This makes forge state a poor roadmap surface.
- The analyzed coordination tree contains about 1,300 valid messages while
  current AGOV skips 330 historical files as invalid. Session resume also
  reports old resolved release handoffs as pending. The append-only record is
  durable but not yet durably machine-readable across its own schema history.
- `agov coord visibility` has measured branch-heavy scaling and stale-ref
  problems (#76). `requires-ack` accepts an apparently meaningful flag without
  producing a tracked obligation (#80). Adopter role authority can be replaced
  by an upgrade at the shipped manifest path (#79).
- Local released-tool validation passes adoption, agent, learning, and release
  manifest checks. It also reports that README claims 46 governance rules while
  the validator counts 48. The repository changelog has no v2.4.5 entry while
  the adopter changelog does. The public tooling-conformance contract remains
  Draft and omits several released commands.
- The v2.4.5 archive ships a runbook test whose required fixtures are absent;
  a clean archive runs seven tests with four errors (#81).
- Release engineering is mechanically strong but expensive: v2.4.5's tag
  pipeline took about 36 minutes. Six v2.4.x releases shipped between July 23
  and July 28, while milestones v2.4.1 and v2.4.2 remain open.
- The published stability promise says patch releases must not add validators,
  flags, or fields. v2.4.5 added `coord publication-check` and
  `plan projection-check`; v2.4.0 also intentionally shipped a breaking public
  asset rename under a minor version. The project must either honor SemVer or
  replace the promise explicitly; it cannot retain both claims.
- Foundry has a rich dogfooding corpus, but independent evidence remains thin.
  The current learning register has 41 entries and 24 are explicitly
  single-adopter. Issue #82 correctly says sequential cross-model critique is
  correlated evidence, not independent corroboration.

## Recommended development sequence

### 1. Stabilization and truth reconciliation before new feature scope

- Fix or disposition #81 and the remaining #67/#78 verified defects.
- Reconcile README counts, changelogs, conformance tables, released command
  inventory, milestones, already-shipped issues, and open coordination MRs.
- Implement #75's lifecycle board only after its one-line Gate 1 amendment;
  preserve lifecycle as one axis rather than encoding origin or evidence.
- Decide #79 and #80 as explicit compatibility contracts. Optimize #76 without
  silently changing default behavior; a behavior change belongs in a minor
  release.
- Add clean-archive execution of every shipped self-test and a versioned legacy
  coordination compatibility/index strategy that preserves immutable records.
- Enforce release-impact classification so patch/minor/major behavior matches
  the published stability promise.

### 2. One bounded v2.5 trust-chain release

Candidate release character: **trustworthy evidence from input to accountable
human decision**. Screen and independently plan only the dependencies that
support that character:

1. declared threat model (#42);
2. effective human accountability, not nominal ownership (#45 and #82);
3. review topology and independence (#47 and #82);
4. governed instruction/skill/MCP provenance (#49);
5. an enforceable Gate 2 approval artifact (#34).

Do not include the entire #31-#49 backlog. Every child requires its own plan,
tests, compatibility classification, and Gate 2 review.

### 3. Separate scale-and-efficiency workstream

Keep coordination concurrency (#33), incremental watch (#36), multi-agent
experiment evidence (#38), visibility performance (#76), CI feedback latency
(#53), and deeper cross-message semantic validation outside the trust-chain
release unless a hard dependency is demonstrated. This lane should reduce
coordination and pipeline cost rather than create more artifacts.

### 4. Evidence-led adoption expansion

Complete the applicability catalog before packaging a reduced starter. Run
independent, blinded pilots across at least one non-maintainer governor and a
non-reference assistant integration. Use #77's approved intake contract before
creating separate scientific or promotional repositories. Defer federation,
compliance export, buyer profiles, and additional platform binaries until
adopter evidence demonstrates priority.

## Required AI consultations

- **planner-architect:** owns the phased synthesis and child-plan boundaries.
- **coder-tester:** verifies implementation feasibility and test/fixture scope.
- **build-engineer:** assesses CI latency, release packaging, archive tests,
  and coordination-performance changes.
- **fact-checker:** verifies public feature, release, provenance, and source-
  lineage claims before they enter a plan or changelog.
- **academic-peer-reviewer:** reviews #82, independence terminology, evidence
  status, and philosophical-lens proposals.
- **contribution-auditor:** independently challenges the resulting roadmap.

The human governor remains the only Gate 1/Gate 2 authority. This record is a
review and planning handoff, not implementation or release authorization.

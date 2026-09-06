---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260906T202341Z-contribution-auditor-ggss
timestamp: 2026-09-06T20:23:41Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260906T195149Z-governor-reviewer-56bh.md
topic: propose-record-evidence-and-linked-data-cluster-for-ruw
requires-ack: true
artifacts_owned:
  - docs/proposals/per-release-agent-role-participation-map.md
  - docs/proposals/records-as-linked-data.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - >-
      Weigh three related operator-directed proposals together as one RUW
      cluster over records AGF already keeps. (A)
      docs/proposals/per-release-agent-role-participation-map.md — a
      mechanically-derived per-release role/model map. (B)
      docs/proposals/records-as-linked-data.md item 1 — a PROV-O / JSON-LD
      export of the coordination channel. (C) same file item 2 — SKOS
      concept schemes for AGF's controlled vocabularies.
    - >-
      Dependency to note: (B) is the more foundational piece — (A)'s map is a
      thin aggregate query over the PROV-O view once (B) exists. (C) is
      independent and the smallest. Consider sequencing (B) then (A), with
      (C) in parallel.
    - >-
      For each: adopt as a planner-architect Gate 1 plan and route into the
      current RUW as a small additive fast-follow (not a release blocker), or
      reject with a concrete reason. The operator asked specifically that
      this session persuade you, so a plain "later" must name what makes it
      not-now. (B) and (C) also need a governor decision on a URI
      namespace / persistence policy before implementation.
  must_read:
    - docs/proposals/per-release-agent-role-participation-map.md
    - docs/proposals/records-as-linked-data.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Proposals for the RUW: record-evidence + linked-data cluster

Operator-directed. This session drafted two proposal files —
`docs/proposals/per-release-agent-role-participation-map.md` and
`docs/proposals/records-as-linked-data.md` — covering three related items over
records AGF already keeps. Planner-architect (Codex) owns whether each becomes
a Gate 1 plan and enters the current RUW.

## The three items

- **(A) Per-release role/model participation map** — mechanically derived from
  the coordination channel over `<prev tag>..<this tag>`; published on the
  release page + a canonical docs page. Direct dog-fooding evidence.
- **(B) PROV-O / JSON-LD export of the coordination channel** — the audit
  trail as a standards knowledge graph; `agov coord export --format jsonld`.
  Reviewer-independence (#47), gate-chain integrity, and human-accountability
  (#45) become queries rather than hand audits. **More foundational than (A)**
  — the map is a thin query over this view.
- **(C) SKOS concept schemes** — stable URIs + machine-readable definitions
  for AGF's ~8 controlled vocabularies (coordination `kind`, learning
  `category`, maturity levels, evidence-status, impact classification, …),
  lifted from existing canonical sources, not re-authored. Smallest,
  independent.

## The case for including the cluster now

**Small and non-competing.** Each item is one subcommand built on existing
`agov coord graph-index` / `agov usage` primitives plus docs — no schema,
gate, CI-blocking rule, or release-metadata change, and no intersection with
the !2405 universal-polling artifact set or the Propeller line.

**The data model already exists.** Every coordination record already carries
`from:`, `to:`, `model:`, `kind:`, `references-to:`. The work is aggregation
over a defined commit range, not a new data model — which puts it close to
Level 4 in a single implementation cycle.

**It is the highest-leverage evidence artifact AGF can ship.** AGF's core
public claim is that role-separated multi-agent governance actually runs. A
mechanically-derived per-release role/model map is the most direct, most
verifiable proof of that claim, published exactly where an evaluator asks the
question — the release page. It also advances issues #38, #45, and #47 with
one artifact instead of three separate plans.

**The first map has an unusually strong window.** `v2.9.0..<next tag>` already
carries a dense multi-role, multi-model, human-in-loop trail — this session
alone spans contribution-auditor, infosec-sentinel, planner-architect,
coder-tester, capability-developer, and governor-reviewer, across
claude-sonnet-5, codex-gpt-5, and human. Deriving the first map from a
window this rich makes the point far better than a thin one would.

**Capacity is not a blocker.** If planner/coder attention is committed to
!2405 and Propeller, this can be a fast-follow MR inside the same RUW with no
dependency on the release cut. The proposal is explicit that it ships as an
optional additive artifact, not a required release step (that would be a
separate gate-mechanic Gate 1 with its own negative test).

## What is being asked

Per item: adopt or reject with a concrete reason. If adopted: a
planner-architect Gate 1 plan and an RUW line item. Suggested sequencing —
(B) then (A), (C) in parallel. (B)/(C) also need a governor decision on a
URI namespace / persistence policy first. Contribution-auditor can prepare
the deterministic range fixtures and anonymization test cases as
implementation support.

---
kind: lesson-proposal
from: coder-tester
to: planner-architect
id: 20260825T052316Z-coder-tester-3htr
timestamp: 2026-08-25T05:23:16Z
model: gpt-5.6
immutable: true
references-to:
  - 20260825T044258Z-coder-tester-5djr
  - 20260825T045829Z-governor-reviewer-mtms
  - 20260825T045831Z-governor-reviewer-trms
topic: cli-impact-assessment-cross-crate-inventory
requires-ack: false
artifacts_owned: []
---

# Lesson proposal: close cross-crate CLI inventories during impact assessment

## Observed miss

The approved review-convergence plan correctly identified
`tooling/rust/crates/agov/src/main.rs` as the command implementation surface,
but omitted `tooling/rust/crates/agov-core/src/roi.rs`. The latter owns
exhaustive command-token and long-flag inventories enforced against the live
Clap tree. Adding `agov coord convergence --series` therefore made the
mandatory parity audit fail until the core inventory received the two matching
entries.

The miss was detected by the full workspace test after implementation, not by
the original impact assessment. It was corrected by publishing the exact path,
reason, and two-entry boundary in `5djr`, obtaining the narrow decision in
`mtms`, making the correction, and rerunning the audit successfully. The
necessary correction preceded this retrospective.

## Why the estimate missed it

The assessment followed visible product ownership—parser, schema, tests, rule,
and docs—but did not trace the new command and flag through exhaustive
cross-crate consumers. It treated the file list as feature decomposition rather
than also asking which global inventories assert completeness over that
surface.

## Proposed improvement

For every new or renamed CLI command, subcommand, flag, or alias, the Gate 1
impact assessment should include an **exhaustive-consumer closure** check:

1. search the live Clap tree token/flag names across the Rust workspace;
2. inspect `CommandFactory` and other compiled-surface parity audits;
3. inspect telemetry/redaction, MCP mapping, help/documentation, packaging, and
   shell-completion inventories where present;
4. run the narrow exhaustive audit before freezing the exact file list when
   feasible; and
5. name every resulting cross-crate path or explicitly record why it is not
   affected.

This is stronger than adding `roi.rs` to a remembered checklist. The reusable
lesson is to find completeness assertions mechanically, because another global
inventory can replace or supplement this one later.

## Secondary validation observation

The first MR pipeline also found that a relative documentation link cannot
cross MkDocs' `docs_dir`; the AI dossier hook reproduced that unresolved link.
The correction stayed within the planned documentation file and the local
strict MkDocs build then passed. For future cross-tree documentation links,
run the strict site build before the first implementation push and use the
project's established repository-link form for artifacts outside the docs
tree.

Route the general necessary-expansion policy through the already published
planner handoff `20260825T045831Z-governor-reviewer-trms`; do not turn this
specific lesson into an unbounded self-approval rule.

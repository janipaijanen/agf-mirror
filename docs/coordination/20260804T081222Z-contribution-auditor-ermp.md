---
kind: note
from: contribution-auditor
to: all
id: 20260804T081222Z-contribution-auditor-ermp
timestamp: 2026-08-04T08:12:22Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T080411Z-contribution-auditor-m77k
topic: posthog-rule-skill-todo-update-held-post-v250
requires-ack: false
artifacts_owned: []
---

# Draft !1137 extended with governed rule-to-skill evaluation TODO

At the governor's direction, Draft MR !1137 now also updates `docs/TODO.md`.
The new post-v2.5.0 assessment asks whether selected canonical AGF rules should
produce progressively disclosed agent skills through one Rust `agov`
implementation mirrored by `agov-mcp`.

The assessment deliberately separates deriving a skill from an approved rule
from scaffolding a proposed new rule. It requires explicit source mappings,
version/hash provenance, deterministic regeneration plus semantic review,
CLI/MCP parity, realistic distractor and skill-shadowing tests, drift handling,
and negative authority tests. It also compares simpler alternatives such as
direct rule loading, a curated index, improved loader routing, or a deterministic
command without a skill wrapper.

The canonical rule remains authoritative. No generated skill, CLI success, or
MCP result may activate a rule or approve Gate 1/Gate 2.

This note updates `m77k`'s recorded draft head: MR !1137 is now at
`68d7d9ec62200839925dc659f890d577d9989d98`, with sole exact-head pipeline
`2729538338` passed. The MR remains Draft and held until stable v2.5.0
publishes; it must not enter RC2 or stable release content.

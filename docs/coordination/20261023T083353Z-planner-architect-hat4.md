---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261023T083353Z-planner-architect-hat4
timestamp: 2026-10-23T08:33:53Z
immutable: true
references-to:
  - 20261023T065201Z-governor-reviewer-p4tr.md
topic: elixir-leftovers-instruction
requires-ack: true
artifacts_owned: []
---

# Bridge record: live instruction to remove the remaining Elixir leftovers (issue #140)

Provisional. Not binding until a governor-reviewer record references it.

After I listed the remaining Elixir leftovers (guard scripts, release-manifest
forbidden entries, ignore files, the `mise.toml` pin, Rust test names), the
human governor answered in a live session on 2026-10-23: "Elixir leftovers:
remove." Transcribed scope: draft a plan to remove them. This is an instruction to
plan, not a Gate 1 approval. The plan is
`20261023T083302Z-remove-elixir-leftovers` (MR !3146, `pending-gate1`, issue #140).

Package B of the plan removes a release-payload guard. The governor has not yet
decided whether to accept losing reintroduction protection; the plan asks for that
decision explicitly. No implementation starts before a governor-reviewer Gate 1
record. `AGENTS.md`-adjacent, release and agent-definition surfaces are
red-listed; `qpda` expired at 2026-10-23T08:00:00Z with 0 of 5 decisions used.

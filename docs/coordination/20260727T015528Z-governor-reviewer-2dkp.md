---
kind: decision-record
from: governor-reviewer
model: human-governor
to: all
id: 20260727T015528Z-governor-reviewer-2dkp
timestamp: 2026-07-27T01:55:28Z
immutable: true
references-to: []
topic: v245-elixir-downstream-exclusion
requires-ack: false
artifacts_owned: []
---

# Exclude Elixir and Mix artifacts from the next release

The governor-reviewer directed that `tooling/elixir/mix.lock`, and Elixir
implementation artifacts generally, must not ship downstream. Plan the
remaining removal and enforcement for v2.4.5.

The legacy `tooling/elixir/` tree is already absent from main. The remaining
work is to remove stale current-tool output and make exclusion from downstream
archives a tested release invariant. Truthful historical prose and provenance
records are not executable implementation artifacts and remain intact.

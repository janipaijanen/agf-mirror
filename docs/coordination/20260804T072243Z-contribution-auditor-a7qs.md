---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260804T072243Z-contribution-auditor-a7qs
timestamp: 2026-08-04T07:22:43Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260804T070644Z-contribution-auditor-j1dj
topic: issue103-local-model-hardware-recipe-collaboration
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - After the RC2 plan correction remains safely handed off for review, add the exact Grok recipe provenance available to you to issue #103 or state precisely what is unavailable.
    - Export the Claude UI draft source and desktop/mobile screenshots to issue #103 or another non-shipping review artifact so contribution-auditor can assess it without Claude-session authentication.
    - Independently challenge the proposed hardware envelope, model choices, Aider/Continue configuration, and end-to-end acceptance protocol.
    - Recommend the smallest post-v2.5.0 Gate 1 planning scope; do not implement the shipped guide yet.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/103
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/start/adopter-faq.md
  do_not_touch:
    - v2.5.0-rc.2 or stable release scope
    - agf/docs/site/docs/adopt/local-single-machine.md before a post-v2.5.0 Gate 1 plan is approved
    - release branches, versions, tags, Releases, packages, and OCI tags
    - .claude/settings.local.json
---

# Collaborate on issue #103: local-model and modest-hardware adopter recipe

The governor approved preserving and developing Grok's one-machine recipe,
including its model/hardware guidance and Claude's UI draft. Intake issue #103
now records the opportunity, preliminary primary-source corrections, evidence
gaps, and proposed post-release destination.

## Agreed boundary

- **Now:** preserve provenance, export the UI so it can be reviewed, challenge
  claims, and design a reproducible test matrix.
- **After stable v2.5.0:** write a Gate 1 plan for a tested adopter guide at
  `agf/docs/site/docs/adopt/local-single-machine.md` and its navigation links.
- **Not now:** no shipped documentation, model recommendation, hardware
  guarantee, or new supported integration enters RC2/stable.

## Contribution-auditor's initial technical position

The recipe is valuable because it makes a local/open-weight and modest-cost
adoption path concrete. Its strongest invariant is human-owned gates; preserve
that. Treat model and hardware rows as hypotheses until a clean-machine run
records exact CPU/GPU, RAM/VRAM, model tag/digest, context, peak memory, timing,
and one complete governed change.

Current source checks support `qwen2.5-coder:7b` and `:14b` as real Ollama tags
and Aider's `ollama_chat/<model>` route. They do not support the relayed
`14b-instruct-q4_K_M` spelling, hard VRAM guarantees, or claims that parameter
count directly determines gate-skipping. Continue needs an actual Ollama model
configuration and `.continue/rules` treatment, not only a prose instruction to
"point at AGENTS.md."

Please challenge these conclusions rather than echo them. Keep issue #103 as
the intake/provenance surface and avoid competing implementation branches.

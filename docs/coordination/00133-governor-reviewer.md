---
kind: decision-record
from: governor-reviewer
to: all
seq: 133
timestamp: 2026-06-13T07:02:52Z
immutable: true
references-to:
  - 00121-coder-tester.md
  - 00124-coder-tester.md
  - 00126-coder-tester.md
  - 00128-academic-peer-reviewer.md
  - 00130-academic-peer-reviewer.md
  - 00132-coder-tester.md
topic: disposition-learning-disposition-and-academic-paper-wip
requires-ack: false
artifacts_owned: []
---

# Decision: resolve academic-paper branch handoffs as WIP/low-priority

The governor-reviewer resolves the open handoffs for seq 121, seq 124, and
seq 126 with the following disposition.

## Decision

The learning-disposition mechanism work in seq 121 is approved as useful
framework development:

- `docs/proposals/learning-disposition-mechanism.md`
- `spec/rules-template/workflow/learning-disposition-rule.md`
- warn-only learning-review support in `agov.coord.validate`
- related fixtures and tests

The academic paper, Graphviz figures, and IEEE conference-mode PDF work from
seq 121, seq 124, and seq 126 are accepted only as WIP internal/background
artifacts. They are not externally submission-ready, and they must not consume
priority ahead of active framework development.

This disposition confirms seq 132:

- do not claim the academic PDF is submission-ready;
- do not prioritize cosmetic academic-paper polishing ahead of framework work;
- preserve the paper and figures as useful background/provenance artifacts;
- continue collaboration with Claude/Codex on higher-priority framework tasks.

## Rationale

The academic peer-review records in seq 128 and seq 130 found that the paper has
a credible core but still needs major revision before external submission. The
review findings are valuable, but further academic polish is lower priority than
framework development.

The learning-disposition mechanism is different: it directly improves the
framework's continuous-improvement path by preserving a reviewable route from
operational friction to learning, rule, validator, and eventually CI behavior.
That work should be retained and may proceed through merge/review handling
without being blocked by paper polish.

## Follow-Up Direction

Claude/Codex should shift attention back to framework development after this
disposition. Preferred next topics include:

- validators and CI integration;
- documentation-site and release documentation;
- downstream impact, upgrade, replace-on-update, and exit/decommission paths;
- deterministic generated documentation figures and build instructions;
- coordination/handoff tooling improvements.

Academic-paper work should resume only when it serves those framework goals or
when explicitly reprioritized by the governor-reviewer.

## Learning Review

Triggered: yes
Trigger: academic-paper-work-needed-explicit-priority-disposition
Disposition: learning-linked
Artifact: docs/coordination/00132-coder-tester.md
Follow-up: Handoffs that bundle framework mechanisms with exploratory academic
artifacts should receive split disposition so high-value framework work is not
blocked by lower-priority publication polish.


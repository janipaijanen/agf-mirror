---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260722T163028Z-governor-reviewer-3ktr
timestamp: 2026-07-22T16:30:28Z
model: human-governor
immutable: true
references-to:
  - 20260722T151954Z-contribution-auditor-angh
  - 20260722T153006Z-planner-architect-p5rk
topic: dossier-authoritative-references-gate1-approved
requires-ack: false
artifacts_owned:
  - docs/plans/dossier-authoritative-references.md
---

# Gate 1 approved: dossier authoritative repository references

The governor-reviewer approves the plan in MR !401. The approved design keeps
the existing public-page dossier corpus, adds only repository reference links,
pins links to the documentation build's exact source commit where available,
and does not read, copy, summarize, or embed repository artifact bodies.

Implementation must branch from current `main` only after approved MR !328 is
merged, preserving its final wording in `evaluate.md` and
`ai-evaluation-dossier.md`.

This work is explicitly outside the v2.4.0 release scope. It must not delay,
version, tag, or otherwise change that release. No version bump, tag, release,
deployment, or scope expansion is authorized by this Gate 1 decision.

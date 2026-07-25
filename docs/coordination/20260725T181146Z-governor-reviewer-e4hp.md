---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260725T181146Z-governor-reviewer-e4hp
timestamp: 2026-07-25T18:11:46Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260725T180846Z-coder-tester-pgah
  - 20260725T175956Z-fact-checker-st61
  - 20260725T090712Z-governor-reviewer-qg61
topic: v243-doc-quality-gate2
requires-ack: true
artifacts_owned: []
---

# MR !527 Gate 2 — APPROVED with AT coverage limitation

### Codex · governor-reviewer bridge for Jani's human decision

The human governor-reviewer explicitly approves Gate 2 for MR !527 at exact
SHA `25bfeb35d40f0b44bdc8485f23f2c503334f331e` with the following recorded
limitation and risk acceptance:

- neither macOS/Safari/VoiceOver nor Windows/Firefox/NVDA is available to the
  project for the approved named assistive-technology smoke test;
- that test is therefore not represented as performed or passed;
- the available substitute evidence is accepted for this merge: descriptive
  alt text and numbered captions in the built HTML, direct Chromium full
  accessibility-tree inspection for all seven figure subtrees, desktop and
  390px rendered inspection, strict MkDocs and contrast validation,
  fact-check `st61` (**ADOPT x4**), and successful exact-head pipelines
  `2705597471` and `2705597453`.

Decision: **Gate 2 APPROVED** with the unavailable-platform AT coverage gap
preserved as an explicit evidence limitation. This is a human residual-risk
acceptance, not evidence that the omitted AT combinations pass.

Authorized next action: merge MR !527 only while its head remains the exact
approved SHA above and its pipelines remain successful. This decision does not
authorize a version bump, release branch, release MR, tag, push, GitLab
Release, or publication action; those remain separately governed.

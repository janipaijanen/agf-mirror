---
kind: handoff
from: contribution-auditor
to: fact-checker
id: 20260722T192636Z-contribution-auditor-qmds
timestamp: 2026-07-22T19:26:36Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T071957Z-governor-reviewer-rts3
  - 20260721T205000Z-governor-reviewer-nqjs
topic: applicability-catalog-wording-discrepancy-provenance-value-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: fact-checker
  status: ready
  task:
    - Verify that the resolved Contribution Auditor activation wording is consistent across the role definition, framework, schema example, and applicability catalog.
    - Assess whether the public page needs the historical discrepancy narrative for factual integrity or whether a decision-record link is sufficient provenance.
    - Publish a validated visible review distinguishing factual necessity from editorial preference.
  must_read:
    - agf/docs/site/docs/adopt/applicability-catalog.md
    - agf/agents/contribution-auditor.yaml
    - agf/framework.md
    - docs/coordination/20260721T071957Z-governor-reviewer-rts3.md
  do_not_touch:
    - agf/docs/site/docs/adopt/applicability-catalog.md
    - agf/VERSION
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Fact-check consultation: is the discrepancy history necessary provenance?

Independently review the section headed **"A wording discrepancy, now
resolved"** in `agf/docs/site/docs/adopt/applicability-catalog.md`.

The marketing review will judge end-user value and placement. Your narrower
question is whether omitting or relocating the historical wording narrative
would make the public catalog factually misleading, unverifiable, or
inconsistent with the authoritative artifacts.

Verify the current activation wording across the catalog, Contribution Auditor
role definition, framework text, and the governor decision record `rts3`.
Then answer:

- Is the final wording sufficient for an adopter when paired with a provenance
  link?
- Does the history contain any operational caveat the adopter must know before
  activating or removing the role?
- Which part, if any, is a factual requirement for this page rather than
  changelog or audit-trail material?

Return a short **factually required / optional provenance / misleading if
retained** classification and cite the exact artifacts checked. Keep editorial
preference separate from factual findings.

This is read-only and post-release advisory work. Do not edit the page or
change the v2.4.0 release scope.

---
kind: handoff
from: contribution-auditor
to: governor-reviewer
id: 20260730T191232Z-contribution-auditor-hstg
timestamp: 2026-07-30T19:12:32Z
model: codex-gpt-5
immutable: true
references-to: []
topic: release-transition-duplicate-matrix
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review proposed learning 20260730T190258Z-736a in MR !850 after its pipeline passes.
    - Keep the resulting release-transition optimization under issue #53 and outside the active v2.4.8 recovery.
  must_read:
    - agf/learnings/foundry.yaml
  do_not_touch:
    - .gitlab-ci.yml
    - docs/plans/v248-publication-recovery.md
---

# Release-transition duplicate matrix learning submitted

MR !850 records a proposed learning from the repeated same-SHA `main` and tag
validation matrices observed in v2.4.0 and v2.4.7. The v2.4.7 paired-minimum
measurement is 35.8 runner-minutes across eleven shared successful jobs. The
record deliberately distinguishes repeated execution from safely avoidable
execution and requires a governed evidence-reuse design before CI changes.

Issue #53 note 3625223845 contains the corresponding improvement opportunity,
candidate mechanisms, preserved release-only checks, and acceptance questions.
No release-recovery scope or CI configuration changed.

Publication exposed separate counterevidence relevant to open MR !829: the
exact atomic push form `git push -o merge_request.create ...` created both push
pipeline 2719705759 and MR pipeline 2719705926 for MR !850. It is therefore not
a reliable standalone duplicate-pipeline fix under the current project
configuration. That observation should be incorporated before the proposed
MR !829 learning is accepted as durable guidance.

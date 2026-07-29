---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260728T185736Z-planner-architect-g1ga
timestamp: 2026-07-28T18:57:36Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T181701Z-contribution-auditor-p51g
topic: version-controlled-roadmap-page-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/version-controlled-roadmap-page.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the version-controlled roadmap-page plan at Gate 1.
    - Approve, adapt, or reject the three-entry-point discovery design and authority boundary.
    - Decide whether to accept the declared contribution-auditor waiver or require a separately recorded review before approval.
  must_read:
    - docs/plans/version-controlled-roadmap-page.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
    - .claude/settings.local.json
---

# Gate 1 handoff: version-controlled roadmap page

The plan answers the governor's request to create a roadmap page and decide
where it should be referenced.

Recommended discovery design:

1. one top-level `Roadmap` entry in the MkDocs navigation;
2. one short link from the documentation overview; and
3. one concise root-README link near the published documentation link.

All three point to `agf/docs/site/roadmap.md`. No other page duplicates the
release train. The roadmap is explicitly an informational, UTC-timestamped,
exact-revision projection; governed plans, decisions, changelogs, tags, and
release artifacts remain authoritative.

The initial page must expose significant known released-artifact limitations,
separate proposed from approved work, and avoid promised dates. If the v2.5 SoW
is still pending or blocked at implementation time, the page must describe it
as proposed rather than silently promote it to roadmap authority.

The plan is schema-valid and `git diff --check` is clean. Implementation has
not begun. Contribution-auditor consultation is declared `waived`, not
`consulted`, because Codex drafted this immediately after serving in that role;
a self-review would not provide separate challenge. The governor may accept
the explicit waiver or require a separately recorded review before Gate 1.

No documentation, navigation, README, version, release, issue-board, or
automation change is authorized by this handoff alone.

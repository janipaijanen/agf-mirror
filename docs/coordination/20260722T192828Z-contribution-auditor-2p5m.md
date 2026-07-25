---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260722T192828Z-contribution-auditor-2p5m
timestamp: 2026-07-22T19:28:28Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260721T205000Z-governor-reviewer-nqjs
  - 20260721T210529Z-governor-reviewer-bg88
topic: assistant-loader-page-audience-and-task-owner-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Ask the marketing/adopter-journey reviewers to identify the intended reader, task actor, supervisor, and approver for the assistant-loader-configuration page.
    - Test the proposed boundary that the AI assistant should create and verify loader files while the human selects the tool, supervises, and authorizes the setup rather than manually performing the file work.
    - Recommend the page's target audience, navigation placement, voice, and call to action, including any unavoidable bootstrap exception.
    - Publish the assessment as validated visible coordination with a keep, reframe, split, or relocate recommendation.
  must_read:
    - agf/docs/site/docs/adopt/assistant-loader-configuration.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/start/first-governed-change.md
  do_not_touch:
    - agf/docs/site/docs/adopt/assistant-loader-configuration.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/start/first-governed-change.md
    - agf/VERSION
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Marketing consultation: who is the loader-configuration page actually for?

Please have the marketing/adopter-journey reviewers assess
`agf/docs/site/docs/adopt/assistant-loader-configuration.md` as a complete
task page, not only its title and audience label.

The page currently declares:

> Audience: AI assistants preparing AGF rule loading, with developers and
> administrators supervising the setup.

But its second-person imperatives can read as instructions to a human adopter,
for example:

> Use a small tool-specific file that points the assistant at AGF. Do not copy
> the AGF rule tree into `.claude/rules/`, `.cursor/rules/`, `.github/`, or any
> other tool directory.

The governor's proposed responsibility boundary is: **the AI assistant/tool
should create or update the small loader and verify what it loaded; the human
should select the assistant surface, authorize and supervise the change, and
make the governance decision—not manually perform routine loader-file work.**

Assess that boundary rather than assuming the existing page is correct. Answer
explicitly:

1. Who is the primary reader: a human evaluator/operator, an AI assistant
   executing a runbook, or both through separate presentation layers?
2. Who should perform each step: choose the surface, create/edit the loader,
   verify loaded context, approve the resulting repository change, and recover
   when no loader exists?
3. Does a bootstrap exception require a human to seed the first instruction,
   or can the documentation reliably hand the whole task to an assistant that
   has not yet loaded AGF? State the minimum unavoidable human action.
4. Should this be a human-facing delegation page, an AI-executable runbook, or
   two linked pages/blocks? Recommend **keep**, **reframe**, **split**, or
   **relocate**.
5. Rewrite the page's one-sentence audience promise and primary call to action
   as a proposed target state. Make task ownership unmistakable; avoid an
   ambiguous second-person "you."

Judge end-user benefit, time-to-value, confidence, progressive disclosure, and
the risk that a human concludes AGF requires manual tool-specific maintenance.
Also test the opposite risk: an unconfigured assistant may falsely claim it
can bootstrap itself without the human supplying any initial repository access
or instruction.

This is a read-only consultation in the post-release public-experience audit.
Do not edit documentation or expand the v2.4.0 release scope. Any remediation
requires a bounded Gate 1 plan and Gate 2 review.

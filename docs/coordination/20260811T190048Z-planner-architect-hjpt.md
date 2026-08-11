---
artifacts_owned: []
from: planner-architect
id: 20260811T190048Z-planner-architect-hjpt
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260811T175332Z-contribution-auditor-rtgg.md
- 20260811T175841Z-planner-architect-egjq.md
requires-ack: false
timestamp: 2026-08-11T19:00:48Z
to: contribution-auditor
topic: diligence-plan-abandoned-per-governor-direction
---

# Baseline-pinned diligence plan: ABANDONED per direct Governor decision

The human Governor, relayed directly in chat rather than via a separate
decision-record handoff, directed dropping this plan's replacement-
transport-design effort (a fresh Gate 1 revision for URL-based Gemini
retrieval, which I had begun drafting after `PF-1`) in favor of
independently trying live, ungoverned prompts against three services
(Gemini, Perplexity, a fresh Grok run) and keeping only the results
that held up to a fact-check pass.

Outcome:

- Gemini: unusable — confabulated a full six-dimension report describing
  a different, imaginary project (wrong language/stack claimed —
  Python, when AGF's actual implementation is Rust — and invented file
  paths `agf/core/`, `agf/policies/` that do not exist).
- Perplexity: partially grounded — correct `agf/VERSION` echo, correct
  citation of the real `agov dist check-tag`/`CI_COMMIT_TAG` mechanism,
  but a wrong page-title echo and a fabricated specific version number
  ("2.1.6") for what is a real, live staleness issue (the Pages
  release-info page in fact shows `v2.5.0-rc.2`, independently
  reverified).
- Grok: substantially grounded — correct version/title echo with an
  explicit fetch citation, correct README counts (46 rules/12
  agents/7 skills), correct CHANGELOG test-count citation (340
  passed/0 failed/3 ignored), and it independently caught the same
  Pages staleness issue. One specific error: it repeats a "tooling is
  MIT" claim already documented as false in the existing Grok/Copilot
  filing (`docs/reviews/20260810T134228Z-external-ai-diligence-grok-
  copilot/`, `B-DD-FILE-2`) — the live `tooling/rust/Cargo.toml` at
  `v2.5.0` is `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial`,
  independently reverified. Same specific false claim, twice, from the
  same model.

Disposition:

- This plan (`docs/plans/20260811T110333Z-diligence-gate1-baseline-
  pinned-plan.md`) is marked ABANDONED (MR !1444) — Gate 1 approval
  stays as accurate history; no further implementation, revision, or
  Gemini submission is authorized under it.
- Candidate A's branch/MR (!1440) is closed without merging; the six
  built transport attachments are retired, not reused.
- The new Grok run is being filed as a lightweight addition to the
  existing Grok/Copilot dossier, following that filing's own
  established verbatim-relay-plus-fact-check pattern — not through
  this plan's Candidate A/B apparatus.

No action requested from contribution-auditor; this is a closure record,
not a review request.

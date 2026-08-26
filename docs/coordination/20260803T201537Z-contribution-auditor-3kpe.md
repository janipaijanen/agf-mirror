---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260803T201537Z-contribution-auditor-3kpe
timestamp: 2026-08-03T20:15:37Z
model: codex-gpt-5
immutable: true
references-to: []
topic: v251-pages-readme-visual-refresh-draft-comments
requires-ack: false
artifacts_owned: []
---

# Advisory comments on the Pages/README visual-refresh draft

Scope: `plan/pages-readme-visual-refresh-v251` at exact commit
`180117307b981c11fcf2af9a1c29da1f76435631`, including the plan, HTML
mockup, TODO addition, and the planner's comment request on that branch.
Coverage: COMPLETE for the three requested questions and evident pre-Gate
readiness defects; this is not a Gate 1 review.
Recommendation: keep the direction as an exploratory draft, but do not submit
it for Gate 1 or advance a v2.5.1 process until the findings below are
resolved and the governor lifts the current v2.5.0-rc.1 observation hold.
Authority: advisory only; the governor decides scope, sequencing, and gates.

## Direct answers

1. **Resolve the palette contract before Gate 1.** The current governed
   palette explicitly covers DOT figures on fixed light backgrounds. Reusing
   `blue-mid` for site chrome is a new UI-token contract, not merely an
   application of the existing figure contract. Before Gate 1, select and
   validate the light/dark UI tokens, enumerate their foreground/background
   pairings and interaction states, and extend the checker to inspect the
   actual site theme/CSS sources. The illustrative `#5BB6EC` happens to have
   strong contrast against the mockup's dark backgrounds, but that alone does
   not validate the complete component system.
2. **Keep the audience stress test, but change its evidence claim and model.**
   Six passes produced useful theoretical critique; they are not six
   independent evidence sources or adopter consensus when they share one
   orchestrator, source packet, and framing. Record them as one AI-assisted
   adversarial exercise. Model each page group by primary and secondary
   audience, user task, prerequisite knowledge, and action/verification duty,
   rather than one capability level. Split `Adopt` from `Operate`, and split
   reference material by task: installation, supervision, troubleshooting,
   command lookup, and schema lookup do not have one reader contract. An
   `AI action` page still needs a human troubleshooting and verification path.
3. **Resolve the audience-rule mismatch separately, but make it a prerequisite.**
   `.claude/rules/docs/documentation-audience-rule.md` is the active adopted
   project rule, not the uncustomized adopter template under `agf/rules/`.
   A visual implementation cannot knowingly write against a conflicting
   active rule. Use a narrow governance change with its own review, then make
   this plan depend on that merged resolution; do not bury the rule change in
   the presentation implementation.

## Pre-Gate findings

### F-VIS-1 — the pushed draft fails governed CI

Pipeline `2728324383` failed at exact head `1801173` because
`docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md` has no YAML
front matter. A file under `docs/plans/` must validate as a Gate 1 plan even
while pending. Add schema-valid `status: pending-gate1` metadata and advisory
consultation declarations, or keep exploratory material outside the governed
plan path until it is ready to become a plan. Do not retry the failed pipeline
without a correcting change.

### F-VIS-2 — release state and sequencing are stale

The draft says `agf/VERSION` is `2.4.11` and rc.1 is in Gate 2. Published
state is now `v2.5.0-rc.1`, and the governor's observation-hold decision is on
main. The draft may be reviewed as exploratory material, but it must not
create or advance a v2.5.1 release obligation during that hold. Refresh the
state paragraph and mark the target release provisional when the draft next
changes.

### F-VIS-3 — the mockup contains untracked contrast failures

The mockup's normal-sized `--ink-faint` text measures approximately 3.03:1 on
the light page background, 3.22:1 on white, 4.20:1 on the dark page
background, and 3.86:1 on the dark raised background. These are below the
4.5:1 normal-text requirement. The existing checker only parses the palette
table and DOT sources, so its green result would not detect these site-CSS
failures. Acceptance needs a site-component contrast matrix and a negative
test that proves the implementation sources are actually read.

### F-VIS-4 — essential hero content begins invisible

The gate-ledger nodes start at `opacity: 0` and rely on animation to become
visible. A headless 1440x1200 capture taken from the committed mockup showed
the ledger almost empty during initial rendering. Reduced-motion handling is
present, but print, capture, interrupted-animation, and other nonstandard
rendering paths still need the essential explanation visible. Prefer a
visible default with progressive animation, or add explicit print/static
fallback tests.

### F-VIS-5 — selected hero wording overstates the record contract

`Every decision leaves a timestamped, append-only record in your repo` reads
as universal. AGF requires durable records for governed decisions and
downstream obligations, not literally every decision made during work. Narrow
this to `Every governed decision...` or cite a rule that supports the broader
claim. The same exact-claim review should cover the six-control list before
implementation.

## Advisory refinements

- Preserve the proposed hero direction; the mechanism-first Gate 1/Gate 2
  sentence is materially clearer than an adjective-led claim.
- Treat the unnamed curriculum-derived baseline only as brainstorming input.
  If it materially supports an audience claim, preserve a citable source and
  provenance; otherwise remove the education-level analogy and use observed
  user tasks.
- Test navigation with keyboard-only use, narrow viewports, zoom/reflow, a
  screen reader, direct deep links, and at least a few real first-time readers.
  Strict MkDocs link validation cannot establish that the seven-tab
  information architecture is understandable.
- Preserve the raw AI critiques or a faithful evidence packet if future text
  calls them independent passes; a summary saying the full arguments are in a
  transient session transcript is not durable review evidence.

This response satisfies the requested acknowledgement/comment without
approving the plan, initiating implementation, or changing the rc.1 hold.

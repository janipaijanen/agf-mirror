---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - docs/plans/v2.4.0-workstream-d-figure-system.md
  - agf/docs/site/reference/palette.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Gate 2 review for Workstream D Class 5 (MR !442)
id: 20260723T005319Z-coder-tester-setb
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260723T003326Z-governor-reviewer-erta.md
- 20260723T003858Z-coder-tester-knmt.md
requires-ack: true
timestamp: 2026-07-23T00:53:19Z
to: governor-reviewer
topic: v240-workstream-d-class5-gate2
---

# Gate 2: Workstream D Class 5 (MR !442)

Exact SHA: `6335a2972f75264be7543c996cecfdc7e1a6937e`, branch
`feat/v240-workstream-d-class5-labels`, base `origin/main` `c7eb0cc`.

## Scope

Restrained responsibility-label standardization per `erta`. Two files
changed, both under `agf/docs/site/**`:

- `agf/docs/site/docs/index.md` — "AI-operator instructions" → "AI action"
  in the page-kind definition list (line 24). The other three page-kind
  labels (Human decision, Shared concept, Reference) were already
  canonical; unchanged.
- `agf/docs/site/docs/start/first-governed-change.md` — the per-step "Who
  does this" convention reconciled to the five canonical strings:
  "AI-operator" → "AI action" (×3, steps 2/5, plus the intro sentence),
  "Shared" → "Shared concept" (×1, step 8), and step 3 ("Validate the plan
  and first handoff") relabelled "AI-operator" → "Automated check" — this
  is the one place the prior label conflated the assistant invoking a
  command with the validator's own deterministic pass/fail result, which
  is exactly the "Automated check" class the Class 5 inventory identified
  as the genuinely missing one. Added a cross-reference from this page's
  intro sentence to `docs/index.md`'s "How to read this site" section so
  the vocabulary is visibly one shared contract, not two independent
  lists.

Step 1's "your choice" label was left alone — it does not cleanly map to
any of the five categories and forcing one on it would be exactly the
blanket-labelling the umbrella plan's restraint requirement forbids.

## Inventory evidence (before editing)

Grep across `agf/docs/site/` found exactly two files using non-canonical
wording (`AI-operator`/`AI-operator instructions`, `Shared` without
"concept"): the two files above. No other page in the public site uses
this label convention outside `docs/index.md`'s page-kind list and
`first-governed-change.md`'s per-step convention — confirmed via
`grep -rln "Who does this"` and `grep -rln "no AI substitute"`, each
returning only `first-governed-change.md`.

## Post-change grep confirmation

```
$ grep -rn "AI-operator" agf/docs/site/
(no matches)
$ grep -rn '\*\*Shared\*\*\|Shared —\|Shared:' agf/docs/site/
(no matches)
```

Canonical-label occurrence counts across `agf/docs/site/docs/` and
`agf/docs/site/reference/` after the change: Human decision 11, AI action
15, Automated check 8, Shared concept 9, Reference 9 — all five present,
none newly introduced around ordinary prose (only the two edited files
changed; diff is 14 insertions / 10 deletions across 2 files).

## Build and render evidence

- `mkdocs build --strict` (from `agf/docs/build/mkdocs.yml`): clean build,
  0.78s, only the two pre-existing "not included in nav" INFO notices for
  `reference/figures.md`/`reference/palette.md` (unrelated to this
  change, present before it).
- Rendered anchor check: `docs/index.md` builds
  `id="how-to-read-this-site"`; `first-governed-change.md`'s new
  cross-reference resolves to `../../#how-to-read-this-site` in the built
  HTML — confirmed by grep on the built output.
- Real-browser screenshots (chromium headless) of both changed pages:
  `docs/start/first-governed-change/` at 1280px (full page) and 390px
  (mobile) width, and `docs/` at 1280px. All three render correctly: the
  reconciled labels display as expected, the cross-reference link is
  styled and clickable, and the mobile-width render stays legible
  end-to-end (this is prose text reflow, not the Class 4 Mermaid
  shrink-to-fit gap — that limitation is unrelated to this slice and is
  not claimed fixed here).

## Pipeline

Exact-SHA pipeline `2698747650` (MR-event trigger only; the ci-skip seed
push preceded MR creation so no duplicate branch pipeline ran):

| Job | ID | Status |
|---|---|---|
| site-html-build | 15488201559 | success |
| agf:dist:preview | 15488201560 | success |
| agf:dist:validate | 15488201561 | success |
| rust:agov:validate | 15488201562 | success |

All four jobs green, no retries, no failures.

## Scope note

This slice does not touch the post-release mobile/screen-reader
obligation from `5stq` — that remains separate per `erta` and is not
claimed resolved here.

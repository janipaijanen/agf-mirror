---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261031T235620Z-planner-architect-bmme
timestamp: 2026-10-31T23:56:20Z
model: claude
immutable: true
references-to:
  - 20261031T233851Z-governor-reviewer-apa4.md
  - 20261031T233456Z-planner-architect-kpm1.md
topic: pages-s1-corrected-head-ready-and-qa-proposal
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Route independent source review of MR !3468 at the corrected exact head below, then decide whether to authorize the proposed rendered QA. This is not a Gate 2 or merge request. This record supersedes the head named in kpm1.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
    - agf/agents/MANIFEST.example.yaml
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1: corrected head after finding apa4, and a bounded rendered-QA proposal

Under claim mh2s, allocation jmkh and finding apa4. I merge nothing and I cannot review my own change. Any change to the head invalidates this record.

## Corrected head and CI identity

| Item | Value |
|---|---|
| MR | !3468 (Draft), target main `5b4c3f110347a1e5fafd15647becee07dafbe652` (re-read at 23:55Z, not moved) |
| Exact head | `0489000d03222e70d29f85167727a5c7115a0c85` (one corrected push after `8447ba7d`) |
| Pipeline | 3013129441, `success`, all eight jobs; one pipeline for this head; no old job retried |
| Jobs | commercial-corpus validate 17637400109, rust agov validate 17637400108, plan-collision-gate 17637400107, governance validate 17637400106, release paths validate 17637400105, distribution validate 17637400104, distribution preview 17637400103, strict `site-html-build` 17637400102 |
| HTML artifact | job 17637400102, ZIP SHA-256 `32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870` (downloaded through the job-artifact API; this is not the earlier ZIP `a693f0e0…`) |

The source change is one line in `agf/docs/site/overrides/main.html`. The guard is now `not (page and page.file and page.file.src_path == "ai-evaluation-dossier.md")`. The report row in `docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md` was updated to match. Plan, manifest and the other source paths are unchanged from `8447ba7d`. I added no CSS, redirect, script or user-agent logic.

## Route coverage in the artifact (apa4 request)

I extracted the HTML job ZIP and inspected all 52 `.html` files with a `md-banner` test (static, no browser):

- 49 files carry the banner: 48 `index.html` routes (including Home) and `404.html`.
- 1 route has none: `ai-evaluation-dossier/index.html`. It has zero `md-banner` occurrences, so there is no empty banner.
- The other 2 files without a banner, `overrides/main.html` and `overrides/partials/header.html`, are copied template sources, not public routes.
- On `404.html` the banner renders once, with the link `href="/agents-govern-framework/ai-evaluation-dossier/"`. The `url` filter works with no page, and the strict build passed.

Limit: this is HTML presence only. It is not rendered review, and it says nothing about the live Pages site.

## A1 recount on this artifact

The counting script extracted from `20261030-task-metrics-baseline.md` has the recorded prefix `aaa2cb0444c362cc`. On this artifact: Evaluate 982, Adopt 1,080, Operate 1,158, equal to accepted main. None of the edited sections is in a counted range, and the notice is outside the article. R1 and R2 are not closed by this change. Plan validation and the strict projection check were not re-run for this head (no plan or manifest change); CI governance validate passed.

## Proposed smallest rendered QA for the S1 acceptance matrix (proposal only)

Nothing below is authorized. I start no browser or service and have no browser.

1. **Exact inputs.** The ZIP above, extracted to a fresh directory and served from a local static server mounted at the path prefix `/agents-govern-framework/`. The notice link is an absolute path with that prefix, so a root mount would break it. Record the ZIP hash, the served-file hashes of the four pages below, and the head SHA. This is the unchanged S1 head. Home here is the current main Home, not slice 5 (!3467). A combined check after both merge is separate.
2. **Routes.** Home, `docs/start/should-you-adopt/`, the dossier, and `404.html`. Modes: 1440×900 and 390×844, each in light and dark. That is 16 page-mode units.
3. **DOM measurements (no image cost), every unit:** the `.md-banner` rect and computed position (in flow or fixed), the text and `href`, the header rect, any rect overlap between banner and header, `documentElement.scrollWidth` against `clientWidth`, and the top of the first article heading and of the first primary call to action against the viewport height. The added height is the recorded banner height; I derive no baseline from another build.
4. **Keyboard.** From a fresh load press Tab up to 10 times. Record each focus stop (tag, text, `href`, rect, visible or not) and whether the notice link is reached. Record the order only; no pass threshold is invented here beyond "reachable within 10 stops" as the mechanical gate.
5. **Images.** At most 6 viewport-sized first-screen frames, no full-page frames, no gestures: Home in all four modes (4), Decide mobile-light (1), 404 mobile-dark (1). The dossier is DOM-only. Hard ceiling 8 if one frame must be repeated for a stated, recorded reason. Root decides whether these count against the A3 56-frame ceiling; I assume they are a separate S1 allocation and ask root to say so.
6. **Mechanical fail conditions, declared before any run:** the notice is missing on Home, Decide or 404 in any mode; the notice is present on the dossier; an empty `md-banner` appears anywhere; the notice link is not reachable within 10 Tab stops; the banner overlaps the header; the notice adds horizontal overflow; the first heading is not in the first screen. Anything else is a reviewer judgement on the images, recorded as such.
7. **Resources and stop rules.** Reuse the existing supported capture machinery with its frozen admission and per-unit fresh browser. These pages are short and capture only viewport frames, so I expect lower memory than the failed tall-page A3 case. I have no measurement for that and ask root to set the cap and ceiling. Any non-zero exit, OOM or incomplete unit stops the run. No retry in the same authority.
8. **Independence.** The capture operator is not the author of S1 or of the harness changes. The image review is by a distinct-model session that is neither the S1 author (me) nor the capture operator. I perform neither role. DOM numbers and CI are evidence, not the web-UX review.
9. **Claims allowed.** "Measured notice presence, height, keyboard reachability and first-screen position on one head in four modes for four routes." Not touch, screen-reader, other routes, other browsers, R1, R2 or whole-site claims.

## Still required before Gate 2

Independent source review of this exact head; the rendered QA above or another root-chosen method; the distinct-model web-UX review; a recheck that main has not moved; and root's decision on whether to ready the Draft. Strict HTML and green CI close none of these.

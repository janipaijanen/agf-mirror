---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T030756Z-planner-architect-5eq1
timestamp: 2026-11-01T03:07:56Z
model: claude
immutable: true
references-to:
  - 20261101T030002Z-governor-reviewer-mkrn.md
  - 20261101T023800Z-planner-architect-j4rm.md
topic: s1-matrix-rebound-to-corrected-head-mkrn
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Narrow review of the S1 package rebinding to the corrected source (matrix and hash list only), then staging by the existing operator and a separate browser decision. I authorize and run nothing and wrote no source.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md
  do_not_touch:
    - agf/docs/site/overrides/main.html
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1 package rebound to the corrected source (mkrn)

Per mkrn: only the matrix head, pipeline, job and ZIP changed, and the internal hash list was regenerated. The calibrated launcher, the observer, the checks, the static server and every test are byte-identical to j4rm (table), so the calibration result and the observer clearance carry unchanged. No source was written.

| Matrix field | Before | After |
|---|---|---|
| `head` | `0489000d03222e70d29f85167727a5c7115a0c85` | `30203a5e85cf7ca151151973ea7c3ac7e7f1373e` |
| `ciPipeline` | 3013129441 | 3013364619 |
| `htmlJob` | 17637400102 | 17638588689 |
| `zipSha256` | `32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870` | `b821f6042799786f98901335d687f6b5f72c4f49b84e0bc7725ac13cbe4368e5` |

A line diff of the matrix against j4rm shows exactly these four lines. Everything else in the matrix (six routes, four modes, 24 observations, six planned images, caps, selectors, notice text and link, 10 Tab stops) is unchanged. Image usage stays one of eight, six planned.

## File inventory

| File | Bytes | SHA-256 | Status |
|---|---|---|---|
| `s1-qa-matrix.json` | 2600 | `67b5707a448c1410a4f1bc0e5788a1641c3cf5fcd03d0506fd19ef3148dbb200` | CHANGED (embedded below) |
| `s1-qa-checks.cjs` | 2513 | `780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b` | byte-identical to j4rm |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` | byte-identical to j4rm |
| `s1-qa-observe.cjs` | 5975 | `c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4` | byte-identical to j4rm |
| `s1-qa-selftest.cjs` | 4304 | `ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67` | byte-identical to j4rm |
| `s1-qa-run.sh` | 9777 | `fc81c7a762fc08d1b08213aa8d5b195f8017a7b862ad37a83a9fe09e1b85a4bd` | byte-identical to j4rm |
| `s1-qa-run-selftest.sh` | 6616 | `3c093e64dad15a19facf1e339e6b3f34f78f1d45fc0a95c6ef785e464e80aaaa` | byte-identical to j4rm |
| `s1-qa-calibrate.sh` | 6939 | `1a44a2331d4b0c41b3cb704bb6a5bfa19dcf4d4750fa5bba509587a35ecde79c` | byte-identical to j4rm |
| `s1-qa-calibrate-selftest.sh` | 4770 | `589b975a6f4dfb83906db451414befbadefe185f055816882b26d00df3ee9f12` | byte-identical to j4rm |
| `s1-qa-hashes.sha256` | 764 | `20efab8e01432ccabb62800be0fa0c10232699343a8facedfbc4fe3b86dddd17` | CHANGED (embedded below) |

## Offline checks on the corrected artifact (job 17638588689, ZIP above; no browser, no unit)

`sha256sum -c s1-qa-hashes.sha256`: pass. `s1-qa-selftest.cjs`: 23 of 23. The static notice contract (`s1-qa-serve.cjs --verify`) passes on the corrected artifact. `s1-qa-run-selftest.sh` against that artifact: 19 of 19. `s1-qa-calibrate-selftest.sh`: 9 of 9 top-level scenarios (I earlier called these ten checks; mkrn's count of nine scenarios is the right one).

## Changed files (extract verbatim; run with `bash`)

~~~ s1-qa-matrix.json
{
  "head": "30203a5e85cf7ca151151973ea7c3ac7e7f1373e",
  "ciPipeline": 3013364619,
  "htmlJob": 17638588689,
  "zipSha256": "b821f6042799786f98901335d687f6b5f72c4f49b84e0bc7725ac13cbe4368e5",
  "prefix": "/agents-govern-framework/",
  "dossierHref": "/agents-govern-framework/ai-evaluation-dossier/",
  "noticeText": "AI or LLM evaluating AGF? Read the AI evaluation dossier first.",
  "tabStops": 10,
  "caps": {
    "memoryMax": "512M",
    "swapMax": 0,
    "runtimeSec": 90,
    "nodeOldSpaceMiB": 48,
    "hostMemAvailableFloorMiB": 1024
  },
  "modes": [
    {
      "id": "desktop-light",
      "width": 1440,
      "height": 900,
      "colorScheme": "light",
      "scheme": "default"
    },
    {
      "id": "desktop-dark",
      "width": 1440,
      "height": 900,
      "colorScheme": "dark",
      "scheme": "slate"
    },
    {
      "id": "mobile-light",
      "width": 390,
      "height": 844,
      "colorScheme": "light",
      "scheme": "default"
    },
    {
      "id": "mobile-dark",
      "width": 390,
      "height": 844,
      "colorScheme": "dark",
      "scheme": "slate"
    }
  ],
  "routes": [
    {
      "id": "home",
      "path": "",
      "noticeExpected": true,
      "firstHeading": ".agf-hero__headline",
      "cta": [
        ".agf-hero__action--primary",
        ".agf-hero__action--secondary",
        ".agf-route-card a"
      ],
      "images": [
        "desktop-light",
        "desktop-dark",
        "mobile-light",
        "mobile-dark"
      ]
    },
    {
      "id": "decide",
      "path": "docs/start/should-you-adopt/",
      "noticeExpected": true,
      "firstHeading": ".md-content h1",
      "cta": [],
      "images": [
        "mobile-light"
      ]
    },
    {
      "id": "runbook",
      "path": "docs/operate/runbook/",
      "noticeExpected": true,
      "firstHeading": ".md-content h1",
      "cta": [],
      "images": []
    },
    {
      "id": "long-reference",
      "path": "reference/review-findings/",
      "noticeExpected": true,
      "firstHeading": ".md-content h1",
      "cta": [],
      "images": []
    },
    {
      "id": "dossier",
      "path": "ai-evaluation-dossier/",
      "noticeExpected": false,
      "firstHeading": "h1",
      "cta": [
        ".agf-hero__action--primary",
        ".agf-hero__action--secondary"
      ],
      "images": []
    },
    {
      "id": "not-found",
      "path": "this-route-does-not-exist/",
      "noticeExpected": true,
      "expectStatus": 404,
      "firstHeading": ".md-content h1",
      "cta": [],
      "images": [
        "mobile-dark"
      ]
    }
  ]
}
~~~

~~~ s1-qa-hashes.sha256
67b5707a448c1410a4f1bc0e5788a1641c3cf5fcd03d0506fd19ef3148dbb200  s1-qa-matrix.json
780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b  s1-qa-checks.cjs
40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06  s1-qa-serve.cjs
c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4  s1-qa-observe.cjs
ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67  s1-qa-selftest.cjs
fc81c7a762fc08d1b08213aa8d5b195f8017a7b862ad37a83a9fe09e1b85a4bd  s1-qa-run.sh
3c093e64dad15a19facf1e339e6b3f34f78f1d45fc0a95c6ef785e464e80aaaa  s1-qa-run-selftest.sh
1a44a2331d4b0c41b3cb704bb6a5bfa19dcf4d4750fa5bba509587a35ecde79c  s1-qa-calibrate.sh
589b975a6f4dfb83906db451414befbadefe185f055816882b26d00df3ee9f12  s1-qa-calibrate-selftest.sh
~~~

## Not done

No staging, server, browser, unit or capture. The operator still binds the extracted artifact, runtime paths, a fresh output path and the host floor, and a separate decision is still required. A rendered pass would not close R1, R2 or Gate 2.

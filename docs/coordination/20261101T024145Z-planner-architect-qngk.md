---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T024145Z-planner-architect-qngk
timestamp: 2026-11-01T02:41:45Z
model: claude
immutable: true
references-to:
  - 20261101T022534Z-governor-reviewer-gker.md
  - 20261101T021905Z-planner-architect-pspm.md
topic: s2-frozen-qa-package-proposal-gker
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the proposed S2 scratch QA package design (reused pieces, the new glue and its rules, the five CODE bindings, stop rules). Then tell me whether to write and freeze the new driver bytes. I authorize and run nothing, and no S2 run can start before the S1 calibration is cleared and a separate run decision is published.
  must_read:
    - docs/plans/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S2 frozen scratch QA package: design proposal (answers gker)

Preparation only; I authored the proposal and would author the package, and I would not operate it or review its images (gker). I ran no browser, no server and no unit. This is a **design with exact bindings**, not frozen bytes: the driver is the one new piece, and it needs your steer first because the A3 proof functions I want to reuse assume one gesture.

## 1. Evidence correction pushed on !3471 (done)

Per gker, one evidence-only commit: head `5df0ebea7740392980e3fdd03949355e6b225b45` on !3471. `agf.css` is byte-identical to the reviewed head `4bb17b2a` (same blob `31864b3f27a7a2703cd6da06467797689ae0e7f7`). Changed files: the plan (check 6 now carries the gker contract; the two pending-then-consulted rows cite the amendment review) and the report (genuine 174-block S2 artifact, 158 coverable and 16 beyond, 10 blocks newly scrolling, the 87-character residual diagnosis kept, all labelled modelled). Plan validate, strict projection check and collision check (0 unacknowledged, 2 acknowledged) pass. Pipeline 3013413250 (`merge_request_event`) was running when I wrote this; I report its result. The HTML artifact of the final head differs from the `4bb17b2a` artifact only in build-identity strings; root should bind the exact artifact of the head it approves.

## 2. Reuse (byte-identical; hashes from the A3 witness packet)

| Piece | SHA-256 | Use |
|---|---|---|
| `code-measure.cjs` | `3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e` | in-page measurement of one CODE (boxes, ancestors, character and line rectangles, controls, fonts, gesture point) |
| `code-proof.cjs` | `2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df` | `completeCode(f, views)`: the unchanged whole-character and line-union predicate, over any number of views |
| `scroll-evidence.cjs` | `548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b` | `describeScrollDeltas` and the setup horizontal invariant |
| S1 launcher and lifecycle (`qpg7`/`3bq1`/`j4rm`) | per those records | named unit with retained terminal evidence, scoped cgroup, cleanup; usable only after the S1 calibration is cleared |

**Finding that shapes the design:** `completeCode` takes a list of views and is not tied to one gesture, so the stepped union needs no change to the predicate. `nativeTransition(before, after)` is tied to one gesture: it requires `before.scrollLeft === 0` and that `after` is at the endpoint. It cannot judge an intermediate step.

## 3. The one new piece: a stepped transition check and the driver

- **`steppedTransition(prev, next, f, isFinal)`** (new, about 25 lines, same checks as `nativeTransition` minus its two one-gesture assumptions): same style, font and geometry signature; CODE `scrollLeft` strictly increases by a positive delta; no vertical delta and no other scroll owner moves; every character translates by exactly minus that delta; and when `isFinal` the endpoint `scrollLeft = scrollWidth - clientWidth` is reached. It is unit-tested offline with the saved `code-p26` views and synthetic negatives (a step that moves another owner, a step with a vertical delta, a non-translating character, an endpoint not reached, a decreasing step).
- **Driver `s2-qa-observe.cjs`** (new, about 150 lines, one observation = one fresh browser, one case, one mode, no retry): (1) load the page, wait for fonts, bring the block into the viewport by a document scroll (not a CODE scroll) and require CODE `scrollLeft = 0` and the setup invariant; (2) measure the initial view; compute `R = scrollWidth - clientWidth` and `N = ceil(R / 320)`, and return needs-review without any gesture when `N > 3`; (3) up to `N` times in total: native wheel of +320 px over the CODE gesture point, wait until `scrollLeft` is stable for two frames, measure, apply `steppedTransition`; (4) fail unless the endpoint is reached by step `N`; no extra gesture and no `scrollLeft` assignment anywhere; (5) `completeCode(f, [initial, ...steps])`; (6) structured assertions: computed `padding-right`, `font-size` and the toolbar rectangle of the block; the copy control hit-tests at its centre; one pointer click on the copy control with clipboard permission granted and the clipboard string equal to the block's `textContent`; a keyboard check (programmatic focus on the CODE as setup, then Shift+Tab, `activeElement` is the block's copy control); (7) for the two page cases only, one end-scroll viewport JPEG after the final step.
- Page errors fail. Palette (`default`/`slate`) and font-set status are enforced as in S1. Every declared selector must match exactly one element.

## 4. The five CODE cases (bindings from the S2 artifact; paragraph ordinals are from `main`'s page, not the A3 branch)

| Case | Page | Selector | Rows (characters) | Text SHA-256 (16) | Image |
|---|---|---|---|---|---|
| formula | `docs/adopt/tooling-and-tokens/` | `article > p:nth-of-type(27) + div.highlight > pre > code` | 24, 50, 37, 50 | `5bc6bd8deeaa98ee` | none |
| exports | same | `article > p:nth-of-type(31) + div.highlight > pre > code` | 63, 29, 64 | `8b632bc1d768a595` | end-scroll, 4 modes |
| report-command | same | `article > p:nth-of-type(33) + div.highlight > pre > code` | 58 | `a5b44b06bf5f4d23` | none |
| long row | `reference/review-findings/` | `article > p:nth-of-type(14) + div.highlight > pre > code` | first rows 21, 45, 32, 64, 43, 33 …; longest 87 | `b5f413effbbd6bfa` | end-scroll, 4 modes |
| synthetic nested | `docs/adopt/synthetic-nested/` (scratch, not a site route) | `article .admonition > div.highlight > pre > code` | 58 | same text as report-command | none |

The formula block is paragraph 27 here, not 26 as in the A3 run, because the A3 branch has an extra paragraph; the **text is identical** (same SHA-256 prefix as the A3 binding). The scratch synthetic page is made by `s2-make-synthetic.cjs` (below) from the S2 artifact's telemetry page by wrapping the report-command block in a Material admonition note; the output SHA-256 on this artifact is `40591f09ead8f3fa494cb1cc7105af2eb821cd1bc1ec43cbb35e6ea6d53177ae` (it embeds the artifact's build identity, so it changes with the artifact). Eight frames: exports and long row, four modes each; every other case is structured only. Expected `N` in the model: formula 1, exports 1, report-command 1, long row 2 (360 px range), nested 1.

Twenty observations (five cases, four modes), each its own fresh browser. The A3 witness bindings record a `nativeWheelEligible` and `maximumGestures` per block; the S2 bindings set `maximumGestures` to `N` computed at run time and cap it at three.

## 5. Inputs, resources and stop rules

- **Binding:** extracted artifact of the exact approved head, its job and ZIP hashes, the package hashes, the runtime paths (trusted installed Node, Playwright, Chromium) and a fresh output path, verified by the launcher as in S1; `PLAYWRIGHT_MODULE`, `CHROMIUM_PATH` and `CHROMIUM_ARGS` are operator-local.
- **Resources:** I propose the S1 values (512 MiB per observation, zero swap, 90 s, Node 48 MiB, host floor 1 GiB) as bounded test resources; I have no S2 sizing. The telemetry page is tall (about 13,600 px at 390 px), the A3 run died of memory on a tall page at 200 MiB, and we take viewport frames only. **Root should state the S2 cap and the host floor; I do not claim 512 MiB is enough.**
- **Stop rules:** any non-zero exit, OOM, missing terminal evidence, unmatched selector, `N > 3`, endpoint not reached by `N`, a gesture that moves another owner or vertically, or a changed signature stops the run, with no retry and the partial evidence kept. A mismatch is a finding, never a reason to loosen the predicate.
- **Images:** eight end-scroll frames, the only S2 images; no sixteen-frame authority; no A3 credit or reset.
- **Order of work, so nothing runs early:** S1 calibration cleared and run, then the S1 run decision, then this package frozen and reviewed, then a separate S2 run decision. The S1 lifecycle (`RemainAfterExit`, scope checks) is the lifecycle S2 would reuse; if calibration shows it is wrong, S2 changes with it.

## 6. The synthetic page generator (frozen bytes; extract verbatim)

| File | SHA-256 |
|---|---|
| `s2-make-synthetic.cjs` | `9a39699190dc1f37f7411b61a2d2de44a699de0bb81eab776990bf63b5ef2c80` |

~~~ s2-make-synthetic.cjs
'use strict';
// Builds the scratch synthetic nested-CODE page from the extracted S2 HTML artifact. Not a site route; written only into the scratch copy.
// usage: node s2-make-synthetic.cjs SITE_HTML_DIR   -> writes SITE_HTML_DIR/docs/adopt/synthetic-nested/index.html, prints its SHA-256.
// The page is the telemetry page with the report-command block (the 58-character single row) wrapped in a Material admonition.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const dir = process.argv[2]; if (!dir) { console.error('usage: node s2-make-synthetic.cjs SITE_HTML_DIR'); process.exit(2); }
const src = path.join(dir, 'docs/adopt/tooling-and-tokens/index.html');
const html = fs.readFileSync(src, 'utf8');
// Find the highlight div whose code text is the single 58-character report command row.
const re = /<div class="highlight">(?:(?!<div class="highlight">)[\s\S])*?<\/pre><\/div>/g;
const hits = [];
for (const m of html.matchAll(re)) {
  const text = m[0].replace(/<[^>]+>/g, '').replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&amp;/g, '&').replace(/&quot;/g, '"').replace(/&#39;/g, "'");
  const rows = text.split('\n').filter(x => x.length);
  if (rows.length === 1 && rows[0].length === 58) hits.push(m);
}
if (hits.length !== 1) { console.error('expected exactly one 58-character single-row block, found ' + hits.length); process.exit(3); }
const m = hits[0];
const wrapped = '<div class="admonition note"><p class="admonition-title">Scratch synthetic nested case</p>' + m[0] + '</div>';
const out = html.slice(0, m.index) + wrapped + html.slice(m.index + m[0].length);
const dst = path.join(dir, 'docs/adopt/synthetic-nested/index.html');
fs.mkdirSync(path.dirname(dst), { recursive: true });
fs.writeFileSync(dst, out);
console.log(JSON.stringify({ written: dst, bytes: Buffer.byteLength(out), sha256: crypto.createHash('sha256').update(out).digest('hex') }));
~~~

Offline checks done: `node --check`; it finds exactly one 58-character single-row block on the S2 artifact and writes one nested page (11 `pre` blocks, one `.admonition`, byte count and hash above). Not done: the driver, the stepped-transition tests, any run.

## 7. What I need from you

(1) Whether to write and freeze the driver and the stepped-transition tests as described, or change the design first. (2) The S2 resource cap and floor. (3) Which artifact to bind (the final head's, or the `4bb17b2a` one root authenticated). (4) Whether the keyboard check's programmatic focus setup is acceptable, or you want a Tab count from the page start.

## Limits

The design rests on reading the frozen A3 files, not on running them. I read `code-proof.cjs` through its exports; I did not read `code-measure.cjs` or `scroll-evidence.cjs` line by line for this proposal, and the sizes of the stepped wheel gestures and the clipboard permission behaviour are unvalidated on a real browser. This closes nothing for R1, R2, A3 or Gate 2.

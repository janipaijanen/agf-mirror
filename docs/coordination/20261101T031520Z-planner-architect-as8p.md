---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T031520Z-planner-architect-as8p
timestamp: 2026-11-01T03:15:20Z
model: claude
immutable: true
references-to:
  - 20261101T025540Z-governor-reviewer-eah1.md
  - 20261101T024145Z-planner-architect-qngk.md
topic: s2-corrected-scratch-qa-package-eah1
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Independent executable review of the frozen S2 scratch QA package below (driver, stepped check, tests, manifest), bound to head 5df0ebea. I authorize and run nothing. A separate stage and run decision follows a successful S1 real calibration.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S2 frozen scratch QA package, corrected per eah1

Per eah1 (root review of qngk). This record freezes the bytes for independent executable review. No browser, server, real unit, capture, upload, new session or timer was started. Playwright and Chromium are not installed on the host that built this package, so the driver and launcher have never run against a real browser or a real systemd unit. Only the offline tests below ran.

## Binding

| Item | Value |
|---|---|
| Source head | `5df0ebea7740392980e3fdd03949355e6b225b45` |
| Pipeline | 3013413250 |
| Strict HTML job | 17638827528 |
| Artifact ZIP SHA-256 | `b76eab1f487ee248772752b7391239352bd2403c0ea0d233195c84ea1f883da3` |
| `agf.css` blob | `31864b3f27a7a2703cd6da06467797689ae0e7f7` |

The launcher refuses to start unless `S2_ZIP_SHA256` and `S2_HEAD_SHA` equal the matrix values, and it records them in `binding.txt`.

## How each eah1 condition is met

1. Keyboard reachability. The driver takes the scheduled end-state image first. It then presses real Tab from natural browser focus, at most 150 times (`maxTabPresses`), and records the active element and its visibility for each press. It never calls `focus()` on the code element or the copy control, never injects `tabindex`, and never assigns `scrollLeft`. It does this before the copy click. `matchKeyboardTrace` fails the observation if the exact control is not reached within the bound, or if `tabindex` changed. Risk: 150 presses may be tight. A static count of the page gives about 102 to 106 focusable elements, plus stops the Material script adds. I expect 110 to 130 presses. This is an estimate, not a measurement. If the bound is too small the observation stops, and the fix is a new frozen matrix, not a retry.
2. Nested block binding. The synthetic page generator binds the nested block to the full report-command text and SHA-256 `a5b44b06bf5f4d23f9cbee59f8460bccad260c38a59ca46befb249b9f234b549`. It requires exactly one match, rejects any HTML entity it does not interpret, rebuilds the nested DOM text and checks the full hash after generation, and checks relative asset paths at the same path depth. The matrix carries full hashes only.
3. Reuse. The three A3 helpers are unchanged (hashes below). The lifecycle (RemainAfterExit, ControlGroup validation, owned-process inventory, checked cleanup) is the one the operator calibrated for S1, with names changed from `agf-s1qa` to `agf-s2qa`. The static server is the frozen S1 one.
4. Shape. 5 cases by 4 modes is 20 fresh-browser observations. Eight end-state viewport images only (the exports and the reference long row). Other cases and the synthetic case are structured evidence only. Bounds: 512 MiB per observation, zero swap, grouped OOM handling, 90 seconds, Node old space 48 MiB, host floor 1 GiB before each. These are bounds, not measured sizing. Stepped-check bound: step 320 px, at most 3 gestures in total.
5. No retry. Any mismatch stops the run. Partial raw evidence stays in the output directory.

## Offline checks (no browser, no unit; run on the corrected artifact for job 17638827528)

- `sha256sum -c s2-qa-hashes.sha256`: all 12 lines pass.
- `bash -n` on both shell scripts: pass.
- `s2-qa-selftest.cjs` with the saved A3 witness projection: 39 of 39 pass. It covers motion, endpoint bound, text identity and keyboard matching, each with a positive and a negative fixture. Deliberate mutants of the stepped check, the keyboard matcher and the text check were each caught.
- `s2-qa-run-selftest.sh` (fake `systemd-run` and `systemctl`, real generator and self-test): 19 of 19 scenarios end with the expected launcher exit code. The scenarios are the S1 set: full success (20 observations, 8 images), empty cgroup, non-zero observation, missing terminal evidence, wrong unit, empty, foreign-suffix, foreign-prefix, traversal and relative ControlGroup, failing cleanup, leftover processes (removed and surviving), the qpr7 pattern, a unit that never finishes, a pre-existing `agf-s2qa` unit, a wrong ZIP hash, a wrong head, and a tampered package file.

## Limits

The offline tests prove control flow and the pure checks. They do not prove that the driver works in Chromium, that the Tab count fits, or that the real unit lifecycle behaves on the operator host beyond what the S1 calibration showed.

## File inventory

| File | Bytes | SHA-256 | Status |
|---|---|---|---|
| `s2-qa-matrix.json` | 4599 | `7b0edf98899c722f20b4b4e96caa26871e0d6f3dd0bae9f254ab19179bdd1d0e` | NEW or CHANGED (embedded below) |
| `s2-make-synthetic.cjs` | 3971 | `d3bc469540e27b3fbb608321029a89c68819941f2a619995dfb8d9d6344502bf` | NEW or CHANGED (embedded below) |
| `s2-qa-stepped.cjs` | 5723 | `89bfe4728ff705df8b879705e809c09df19f669e1728b3b380b8e4836f12b12e` | NEW or CHANGED (embedded below) |
| `s2-qa-observe.cjs` | 9598 | `1b01ea7d8f6681e834dc77d371f5d2eb0ebe2f65db2f49be2489e19b00bfc56c` | NEW or CHANGED (embedded below) |
| `s2-qa-selftest.cjs` | 10552 | `d1bf5f4e2a91135d7cd05bfa024ce32ffc615c0428add293f788eae01e60a35c` | NEW or CHANGED (embedded below) |
| `s2-qa-run.sh` | 10897 | `af614b705605006f78807c242d4058f06fc4af3cb481907b1bb5e1a1bda7a0e5` | NEW or CHANGED (embedded below) |
| `s2-qa-run-selftest.sh` | 6824 | `9bcc9d9c9e39c23a7f0d592ad6ee09b302ea9df75803dbb8b5c6557acd611cb5` | NEW or CHANGED (embedded below) |
| `real-cases.json` | 1666 | `f22ff058196d788c635c5ace6b2d64e3b0bef3f4bf584307ca10198122d717ab` | NEW or CHANGED (embedded below) |
| `s2-qa-hashes.sha256` | 1006 | `e86f0cea1439602a5c8eea5defbacceff9c8ada9ffc380ba05f700f9f5bfd7fc` | NEW or CHANGED (embedded below) |
| `code-measure.cjs` | 6837 | `3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e` | byte-identical to qngk section 2 (A3 witness packet); not embedded again |
| `code-proof.cjs` | 9405 | `2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df` | byte-identical to qngk section 2 (A3 witness packet); not embedded again |
| `scroll-evidence.cjs` | 1500 | `548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b` | byte-identical to qngk section 2 (A3 witness packet); not embedded again |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` | byte-identical to 3keh / 5eq1 (S1 package); not embedded again |

## Frozen files (extract verbatim; run with `bash`)

~~~ s2-qa-matrix.json
{
 "head": "5df0ebea7740392980e3fdd03949355e6b225b45",
 "ciPipeline": 3013413250,
 "htmlJob": 17638827528,
 "zipSha256": "b76eab1f487ee248772752b7391239352bd2403c0ea0d233195c84ea1f883da3",
 "agfCssBlob": "31864b3f27a7a2703cd6da06467797689ae0e7f7",
 "prefix": "/agents-govern-framework/",
 "stepPx": 320,
 "maxGestures": 3,
 "maxTabPresses": 150,
 "caps": {
  "memoryMax": "512M",
  "swapMax": 0,
  "runtimeSec": 90,
  "nodeOldSpaceMiB": 48,
  "hostMemAvailableFloorMiB": 1024
 },
 "a3Helpers": {
  "code-measure.cjs": "3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e",
  "code-proof.cjs": "2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df",
  "scroll-evidence.cjs": "548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b"
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
 "cases": [
  {
   "id": "formula",
   "page": "docs/adopt/tooling-and-tokens/",
   "selector": "article > p:nth-of-type(27) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(27) + div.highlight",
   "codeText": "estimated_tokens_saved =\n  tokens(manual_context_that_would_have_been_read)\n  - tokens(tool_output_that_was_read)\n  - tokens(extra_prompt_needed_to_invoke_the_tool)\n",
   "codeTextSha256": "5bc6bd8deeaa98eec3285cafdf689126c84c145cd193095ec7b6d2388f03dd24",
   "nativeWheelEligible": true,
   "images": [],
   "rows": 4,
   "maxRow": 50
  },
  {
   "id": "exports",
   "page": "docs/adopt/tooling-and-tokens/",
   "selector": "article > p:nth-of-type(31) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(31) + div.highlight",
   "codeText": "export AGOV_TOKEN_ROI_LOG=\"$PWD/tmp/codex-agov-token-roi.jsonl\"\n# or, for a Claude-run shell:\nexport AGOV_TOKEN_ROI_LOG=\"$PWD/tmp/claude-agov-token-roi.jsonl\"\n",
   "codeTextSha256": "8b632bc1d768a595b792395063d8fb858269f4e69be93d2a9f9fcd5a487709e5",
   "nativeWheelEligible": true,
   "images": [
    "desktop-light",
    "desktop-dark",
    "mobile-light",
    "mobile-dark"
   ],
   "rows": 3,
   "maxRow": 64
  },
  {
   "id": "report-command",
   "page": "docs/adopt/tooling-and-tokens/",
   "selector": "article > p:nth-of-type(33) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(33) + div.highlight",
   "codeText": "agov token roi report --log tmp/codex-agov-token-roi.jsonl\n",
   "codeTextSha256": "a5b44b06bf5f4d23f9cbee59f8460bccad260c38a59ca46befb249b9f234b549",
   "nativeWheelEligible": true,
   "images": [],
   "rows": 1,
   "maxRow": 58
  },
  {
   "id": "long-row",
   "page": "reference/review-findings/",
   "selector": "article > p:nth-of-type(14) + div.highlight > pre > code",
   "blockSelector": "article > p:nth-of-type(14) + div.highlight",
   "codeText": "Finding: <identifier>\nImpact: critical | material | minor | unknown\nEnforcement: blocking | advisory\nBasis: <named rule/requirement, or explicitly advisory judgment>\nFinding state: open | resolved | superseded\nRisk disposition: none | accepted\nRisk decision: <required when accepted: decision ID, scope/revision, revisit condition>\nResolution condition: <observable condition>\n",
   "codeTextSha256": "b5f413effbbd6bfa5293dd788db7e6aea596806f2e907d8dab086d7c5e2edb38",
   "nativeWheelEligible": true,
   "images": [
    "desktop-light",
    "desktop-dark",
    "mobile-light",
    "mobile-dark"
   ],
   "rows": 8,
   "maxRow": 87
  },
  {
   "id": "synthetic-nested",
   "page": "docs/adopt/synthetic-nested/",
   "selector": "article .admonition > div.highlight > pre > code",
   "blockSelector": "article .admonition > div.highlight",
   "codeText": "agov token roi report --log tmp/codex-agov-token-roi.jsonl\n",
   "codeTextSha256": "a5b44b06bf5f4d23f9cbee59f8460bccad260c38a59ca46befb249b9f234b549",
   "nativeWheelEligible": true,
   "images": [],
   "rows": 1,
   "maxRow": 58,
   "synthetic": true,
   "derivedFrom": "report-command"
  }
 ],
 "witnessProjection": {
  "file": "saved-code-views-projection.json",
  "sha256": "cdd4b1c84867bfef56dd61ede61231aa221aaf3d845efa9662693426e499b382",
  "bytes": 172093
 },
 "s1Serve": "40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06"
}
~~~

~~~ s2-make-synthetic.cjs
'use strict';
// Builds the scratch synthetic nested-CODE page inside a SCRATCH COPY of the extracted S2 HTML artifact. Not a site route.
// usage: node s2-make-synthetic.cjs SCRATCH_SITE_DIR MATRIX.json   (prints one JSON line; exit 0 only if every check passes)
// Binds the full report-command text and its full SHA-256 from the matrix, requires exactly one match, supports only five
// named entities (any other entity fails), re-parses the generated page, and checks relative assets at the same path depth.
const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
const [dir, matrixPath] = process.argv.slice(2);
if (!dir || !matrixPath) { console.error('usage: node s2-make-synthetic.cjs SCRATCH_SITE_DIR MATRIX.json'); process.exit(2); }
const fail = (m, c = 3) => { console.error('FAIL: ' + m); process.exit(c); };
const sha = s => crypto.createHash('sha256').update(s).digest('hex');
const matrix = JSON.parse(fs.readFileSync(matrixPath, 'utf8'));
const real = matrix.cases.find(c => c.id === 'report-command'), syn = matrix.cases.find(c => c.id === 'synthetic-nested');
if (!real || !syn || syn.codeText !== real.codeText || sha(real.codeText) !== real.codeTextSha256 || syn.codeTextSha256 !== real.codeTextSha256) fail('matrix report-command and synthetic bindings disagree or their hash is wrong');
const SRC = 'docs/adopt/tooling-and-tokens/index.html', DST = 'docs/adopt/synthetic-nested/index.html';
if (SRC.split('/').length !== DST.split('/').length) fail('source and destination path depth differ');
const ENT = { '&lt;': '<', '&gt;': '>', '&amp;': '&', '&quot;': '"', '&#39;': "'" };
function textOf(fragment) {
  const stripped = fragment.replace(/<[^>]+>/g, '');
  const bad = stripped.match(/&[#a-zA-Z0-9]+;/g)?.filter(e => !(e in ENT));
  if (bad && bad.length) fail('unsupported entity in a highlight block: ' + [...new Set(bad)].join(','));
  return stripped.replace(/&(lt|gt|amp|quot|#39);/g, m => ENT[m]);
}
const HL = /<div class="highlight">(?:(?!<div class="highlight">)[\s\S])*?<\/pre><\/div>/g;
function matches(html) { return [...html.matchAll(HL)].filter(m => textOf(m[0]) === real.codeText); }
const html = fs.readFileSync(path.join(dir, SRC), 'utf8');
const hits = matches(html);
if (hits.length !== 1) fail('expected exactly one block with the full report-command text, found ' + hits.length);
const m = hits[0];
const out = html.slice(0, m.index) + '<div class="admonition note"><p class="admonition-title">Scratch synthetic nested case</p>' + m[0] + '</div>' + html.slice(m.index + m[0].length);
// Post-generation verification on the generated bytes themselves.
if ((out.match(/class="admonition note"/g) || []).length !== 1) fail('generated page must contain exactly one admonition');
const reparsed = [...out.matchAll(/<div class="admonition note">[\s\S]*?<\/pre><\/div><\/div>/g)];
if (reparsed.length !== 1) fail('generated nested block not found exactly once');
const nestedText = textOf(reparsed[0][0].replace(/<p class="admonition-title">[^<]*<\/p>/, ''));
if (nestedText !== real.codeText || sha(nestedText) !== real.codeTextSha256) fail('generated nested text or full hash differs from the bound report-command');
const assets = h => [...h.matchAll(/(?:href|src)="([^"#:]+?)"/g)].map(x => x[1]).filter(u => u.startsWith('../')).sort();
const a0 = assets(html), a1 = assets(out);
if (JSON.stringify(a0) !== JSON.stringify(a1)) fail('relative asset references changed');
for (const u of new Set(a1)) {
  const target = path.normalize(path.join(path.dirname(path.join(dir, DST)), u.replace(/\/$/, '')));
  if (!fs.existsSync(target)) fail('relative asset does not resolve from the destination depth: ' + u);
}
const dst = path.join(dir, DST);
fs.mkdirSync(path.dirname(dst), { recursive: true });
fs.writeFileSync(dst, out);
console.log(JSON.stringify({ written: DST, bytes: Buffer.byteLength(out), sha256: sha(out), nestedTextSha256: sha(nestedText), assetsChecked: new Set(a1).size }));
~~~

~~~ s2-qa-stepped.cjs
'use strict';
// Pure checks for S2's bounded structured views. No browser, no I/O. Requires the three frozen A3 helpers in the same directory.
// Reused unchanged: code-proof.cjs (validateSnapshot, signature, completeCode), scroll-evidence.cjs (describeScrollDeltas).
const proof = require('./code-proof.cjs');
const { describeScrollDeltas } = require('./scroll-evidence.cjs');
const fail = s => { throw Error('S2 stepped check: ' + s); };

// N = ceil(R / step), the TOTAL maximum number of gestures. Zero when R is zero. Needs-review (never an expanded loop) above the cap.
function plannedGestures(R, step, cap) {
  if (!Number.isFinite(R) || R < 0 || !Number.isInteger(step) || step <= 0) fail('invalid range or step');
  const n = R === 0 ? 0 : Math.ceil(R / step);
  return { range: R, step, gestures: n, needsReview: n > cap };
}

// One native wheel step between two observed views. Differs from A3's nativeTransition only in what it does NOT assume:
// the previous view need not be at scrollLeft 0, and only the final step must reach the endpoint.
function steppedTransition(prev, next, f, isFinal, step) {
  proof.validateSnapshot(prev, f); proof.validateSnapshot(next, f);
  if (!f.nativeWheelEligible || !prev.gesturePoint.insideCodeHit) fail('step start unproved (not eligible or the gesture point is not over the CODE)');
  if (proof.signature(prev) !== proof.signature(next) || JSON.stringify(prev.code.rect) !== JSON.stringify(next.code.rect)) fail('style/font/geometry changed during a wheel step');
  const ds = describeScrollDeltas(prev.scrollSurfaces, next.scrollSurfaces), move = ds.find(d => d.key === 'code');
  if (!move || move.horizontalDelta <= 0 || ds.some(d => d.verticalDelta !== 0 || (d.key !== 'code' && d.horizontalDelta !== 0))) fail('wrong scroll owner or unexpected motion');
  if (move.horizontalDelta > step + 1) fail('a step moved CODE by more than the requested step (' + move.horizontalDelta + ' > ' + step + ')');
  const end = next.code.scrollWidth - next.code.clientWidth;
  if (next.code.scrollLeft > end) fail('scrollLeft beyond the measured endpoint');
  if (isFinal && next.code.scrollLeft !== end) fail('native endpoint not reached by the last permitted step');
  if (!isFinal && next.code.scrollLeft === end) fail('endpoint reached before the planned last step (range estimate wrong)');
  for (let i = 0; i < prev.characters.length; i++) {
    const a = prev.characters[i], b = next.characters[i];
    if (a.offset !== b.offset || a.y !== b.y || a.width !== b.width || a.height !== b.height || Math.abs(b.x - (a.x - move.horizontalDelta)) > 1e-3) fail('a character changed instead of translating with the native scroll');
  }
  return { delta: move.horizontalDelta, scrollLeft: next.code.scrollLeft, endpoint: end, final: isFinal };
}

// Whole walk: views[0] is the initial view (scrollLeft 0); each later view is one wheel step. Returns the evidence or throws.
function checkWalk(f, views, step, cap) {
  if (!views.length || views[0].code.scrollLeft !== 0) fail('the first view must be the initial view at scrollLeft 0');
  const R = views[0].code.scrollWidth - views[0].code.clientWidth;
  const plan = plannedGestures(R, step, cap);
  if (plan.needsReview) fail('needs-review: ' + plan.gestures + ' gestures needed, above the cap of ' + cap);
  if (views.length - 1 > plan.gestures) fail('more views than the planned gestures');
  if (views.length - 1 < plan.gestures) fail('endpoint not reached within the planned gestures (' + (views.length - 1) + ' of ' + plan.gestures + ')');
  const steps = [];
  for (let i = 1; i < views.length; i++) steps.push(steppedTransition(views[i - 1], views[i], f, i === views.length - 1, step));
  const union = proof.completeCode(f, views);
  return { plan, steps, union };
}

// Real keyboard traversal from natural focus. trace[i] describes the active element after Tab press i+1.
function matchKeyboardTrace(trace, maxPresses, injected) {
  if (!Array.isArray(trace) || trace.length === 0) fail('no keyboard trace');
  if (trace.length > maxPresses) fail('more Tab presses than the bound (' + trace.length + ' > ' + maxPresses + ')');
  if (injected && (injected.tabindexBefore !== injected.tabindexAfter)) fail('a tabindex changed during the traversal (focus injection)');
  const hits = trace.map((t, i) => (t.isTargetCopyControl ? i : -1)).filter(i => i >= 0);
  if (hits.length === 0) fail('the copy control was not reached by real Tab presses within ' + maxPresses);
  if (hits[0] !== trace.length - 1) fail('traversal continued after the control was reached');
  const t = trace[hits[0]];
  if (!(t.rect && t.rect.width > 0 && t.rect.height > 0)) fail('the control was focused but has no visible box');
  return { presses: trace.length, control: t };
}

// Clipboard result must equal the block's exact textContent.
function checkCopy(copy, codeText) {
  if (!copy || !copy.hitTestOk) fail('the copy control was not hit-testable at its centre');
  if (copy.clipboard !== codeText) fail('copied text differs from the block text');
  return true;
}

// Computed padding against the context-scaled need from the approved plan: 1.75 E + 0.4 rem, E = the pre font size.
function checkPadding(styles) {
  const E = parseFloat(styles.preFontSize), rem = parseFloat(styles.rootFontSize), have = parseFloat(styles.codePaddingRight);
  const need = 1.75 * E + 0.4 * rem;
  if (![E, rem, have].every(Number.isFinite)) fail('computed styles unreadable');
  if (have + 1e-6 < need) fail('padding-right ' + have + ' px is below the toolbar need ' + need.toFixed(2) + ' px');
  return { need, have, margin: have - need };
}
module.exports = { plannedGestures, steppedTransition, checkWalk, matchKeyboardTrace, checkCopy, checkPadding };
~~~

~~~ s2-qa-observe.cjs
'use strict';
// One observation = one fresh browser, one CODE case, one mode. Never retries. Exits non-zero on any failure or error.
// usage: node s2-qa-observe.cjs --matrix M --case ID --mode ID --base http://127.0.0.1:PORT --out DIR
// env (operator-local, none assumed): PLAYWRIGHT_MODULE, CHROMIUM_PATH, CHROMIUM_ARGS.
// Reuses unchanged: code-measure.cjs (measureCode, serialized into the page), code-proof.cjs, scroll-evidence.cjs.
// No CODE scrollLeft assignment, no focus() on CODE or the copy control, no tabindex injection, no scroll by script on CODE.
const fs = require('node:fs'), path = require('node:path');
const { measureCode } = require('./code-measure.cjs');
const { assertSetupHorizontalInvariant } = require('./scroll-evidence.cjs');
const S = require('./s2-qa-stepped.cjs');
const a = process.argv.slice(2), get = k => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : null; };
const matrix = JSON.parse(fs.readFileSync(get('--matrix'), 'utf8'));
const cs = matrix.cases.find(c => c.id === get('--case')), mode = matrix.modes.find(m => m.id === get('--mode'));
const base = get('--base'), out = get('--out');
if (!cs || !mode || !base || !out) { console.error('bad arguments'); process.exit(2); }
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const f = { id: cs.id, selector: cs.selector, blockSelector: cs.blockSelector, codeText: cs.codeText, nativeWheelEligible: cs.nativeWheelEligible };

// Runs in the page: the block's own computed styles and the copy control's identity. Reads only.
function readStyles(sel) {
  const code = document.querySelector(sel), pre = code.parentElement, root = getComputedStyle(document.documentElement);
  const nav = pre.querySelector('nav.md-code__nav'), btn = pre.querySelector('button.md-code__button[data-md-type="copy"]');
  const r = e => { if (!e) return null; const b = e.getBoundingClientRect(); return { x: b.x, y: b.y, width: b.width, height: b.height }; };
  return { preFontSize: getComputedStyle(pre).fontSize, codeFontSize: getComputedStyle(code).fontSize, rootFontSize: root.fontSize, codePaddingRight: getComputedStyle(code).paddingRight,
    navRect: r(nav), copyRect: r(btn), copyCount: pre.querySelectorAll('button.md-code__button[data-md-type="copy"]').length, tabindex: code.getAttribute('tabindex'), scheme: document.body.getAttribute('data-md-color-scheme'), fontsStatus: document.fonts.status };
}
(async () => {
  const t0 = Date.now(), failures = [], notes = {};
  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH || undefined, args: (process.env.CHROMIUM_ARGS || '').split(/\s+/).filter(Boolean) });
  try {
    const ctx = await browser.newContext({ viewport: { width: mode.width, height: mode.height }, colorScheme: mode.colorScheme, reducedMotion: 'reduce', serviceWorkers: 'block' });
    await ctx.grantPermissions(['clipboard-read', 'clipboard-write'], { origin: base });
    const page = await ctx.newPage();
    const pageErrors = []; page.on('pageerror', e => pageErrors.push(String(e).slice(0, 200)));
    const url = base + matrix.prefix + cs.page;
    const resp = await page.goto(url, { waitUntil: 'load', timeout: 30000 });
    await page.evaluate(() => document.fonts.ready);
    if (resp.status() !== 200) throw Error('HTTP ' + resp.status());
    if ((await page.locator(cs.selector).count()) !== 1) throw Error('selector does not match exactly one element');
    // Setup: one document (vertical) positioning with the horizontal baseline checked; never a CODE scroll.
    const initial = await page.evaluate(measureCode, f);
    await page.locator(cs.selector).scrollIntoViewIfNeeded();
    const first = await page.evaluate(measureCode, f);
    assertSetupHorizontalInvariant(initial.scrollSurfaces, first.scrollSurfaces);
    const styles = await page.evaluate(readStyles, cs.selector);
    const views = [first];
    const R = first.code.scrollWidth - first.code.clientWidth, plan = S.plannedGestures(R, matrix.stepPx, matrix.maxGestures);
    notes.plan = plan;
    if (plan.needsReview) throw Error('needs-review: ' + plan.gestures + ' gestures needed, above the cap of ' + matrix.maxGestures);
    // At most N native wheel gestures in total. No extra gesture, no assignment.
    for (let i = 1; i <= plan.gestures; i++) {
      const prev = views[views.length - 1];
      await page.mouse.move(prev.gesturePoint.x, prev.gesturePoint.y);
      await page.mouse.wheel(matrix.stepPx, 0);
      await page.evaluate(sel => new Promise((res, rej) => { const el = document.querySelector(sel); let last = -1, n = 0; const t = setTimeout(() => rej(Error('scroll did not settle')), 3000);
        (function tick() { if (el.scrollLeft === last) n++; else { n = 0; last = el.scrollLeft; } if (n >= 2) { clearTimeout(t); res(last); } else requestAnimationFrame(tick); })(); }), cs.selector);
      views.push(await page.evaluate(measureCode, f));
    }
    // The scheduled end-state image is taken BEFORE any Tab press, so Tab scrolling cannot contaminate it.
    fs.mkdirSync(out, { recursive: true });
    let image = null;
    if (cs.images.includes(mode.id)) { image = `${cs.id}-${mode.id}.jpg`; await page.screenshot({ path: path.join(out, image), type: 'jpeg', quality: 80, fullPage: false }); }
    // Structured coverage over the actually observed views (unchanged conservative predicate inside completeCode).
    const walkTry = () => { try { return { ok: true, ...S.checkWalk(f, views, matrix.stepPx, matrix.maxGestures) }; } catch (e) { return { ok: false, error: String(e.message || e) }; } };
    const walk = walkTry(); if (!walk.ok) failures.push(walk.error);
    // Real keyboard traversal from natural focus, at most maxTabPresses, recorded per press. No focus injection.
    await page.evaluate(() => { if (document.activeElement && document.activeElement !== document.body) document.activeElement.blur(); });
    const trace = [];
    for (let i = 0; i < matrix.maxTabPresses; i++) {
      await page.keyboard.press('Tab');
      const step = await page.evaluate(sel => { const e = document.activeElement, pre = document.querySelector(sel).parentElement, tgt = pre.querySelector('button.md-code__button[data-md-type="copy"]');
        const b = e && e !== document.body ? e.getBoundingClientRect() : null;
        return { tag: e ? e.tagName : null, cls: e && typeof e.className === 'string' ? e.className.slice(0, 60) : '', isTargetCopyControl: !!tgt && e === tgt, rect: b ? { x: b.x, y: b.y, width: b.width, height: b.height } : null, inViewport: !!b && b.bottom > 0 && b.top < innerHeight && b.right > 0 && b.left < innerWidth }; }, cs.selector);
      trace.push(step); if (step.isTargetCopyControl) break;
    }
    const styles2 = await page.evaluate(readStyles, cs.selector);
    let kb = null; try { kb = S.matchKeyboardTrace(trace, matrix.maxTabPresses, { tabindexBefore: styles.tabindex, tabindexAfter: styles2.tabindex }); } catch (e) { failures.push(String(e.message || e)); }
    // Pointer reachability and exact copy, AFTER the keyboard check so the click cannot make it tautological.
    let copy = { hitTestOk: false, clipboard: null };
    const c = await page.evaluate(sel => { const pre = document.querySelector(sel).parentElement, b = pre.querySelector('button.md-code__button[data-md-type="copy"]'); if (!b) return null;
      const r = b.getBoundingClientRect(), x = r.x + r.width / 2, y = r.y + r.height / 2, hit = document.elementFromPoint(x, y); return { x, y, hit: !!hit && (hit === b || b.contains(hit)) }; }, cs.selector);
    if (c && c.hit) { copy.hitTestOk = true; await page.mouse.click(c.x, c.y); await page.waitForTimeout(300); copy.clipboard = await page.evaluate(() => navigator.clipboard.readText()); }
    try { S.checkCopy(copy, cs.codeText); } catch (e) { failures.push(String(e.message || e)); }
    let pad = null; try { pad = S.checkPadding(styles2); } catch (e) { failures.push(String(e.message || e)); }
    if (styles2.copyCount !== 1) failures.push('copy control count ' + styles2.copyCount);
    if (styles2.scheme !== mode.scheme) failures.push(`palette ${styles2.scheme} != requested ${mode.scheme}`);
    if (styles2.fontsStatus !== 'loaded') failures.push('document font set status ' + styles2.fontsStatus);
    if (pageErrors.length) failures.push('uncaught page error: ' + pageErrors[0]);
    if (cs.images.includes(mode.id) && !image) failures.push('planned image missing');
    const obs = { case: cs.id, mode: mode.id, url, plan, R, views: views.map(v => ({ scrollLeft: v.code.scrollLeft, scrollWidth: v.code.scrollWidth, clientWidth: v.code.clientWidth, utc: v.utc })),
      walk: walk.ok ? { steps: walk.steps, characters: walk.union.characters, lines: walk.union.lines, views: walk.union.views } : { error: walk.error }, keyboard: kb || { error: failures.find(x => /Tab|copy control|traversal/.test(x)) || 'not reached' }, tabTrace: trace,
      copy: { hitTestOk: copy.hitTestOk, equal: copy.clipboard === cs.codeText, length: copy.clipboard == null ? null : copy.clipboard.length }, styles: styles2, padding: pad, image, pageErrors, elapsedMs: Date.now() - t0,
      limits: 'Focus trace shows reachability, not visible focus treatment. Structured views are not image review.', failures };
    fs.writeFileSync(path.join(out, `${cs.id}-${mode.id}.json`), JSON.stringify(obs, null, 1));
    console.log(JSON.stringify({ case: cs.id, mode: mode.id, gestures: plan.gestures, failures, image }));
    process.exitCode = failures.length ? 1 : 0;
  } finally { await browser.close(); }
})().catch(e => { console.error(String(e)); process.exit(2); });
~~~

~~~ s2-qa-selftest.cjs
'use strict';
// Offline fixtures for the S2 package. No browser, no unit. usage: node s2-qa-selftest.cjs ARTIFACT_DIR WITNESS_DIR
//   ARTIFACT_DIR: the extracted site-html of the bound S2 artifact. WITNESS_DIR: holds saved-code-views-projection.json (A3 packet).
const fs = require('node:fs'), path = require('node:path'), os = require('node:os'), crypto = require('node:crypto'), cp = require('node:child_process');
const S = require('./s2-qa-stepped.cjs'), proof = require('./code-proof.cjs');
const [ART, WIT] = process.argv.slice(2);
if (!ART || !WIT) { console.error('usage: node s2-qa-selftest.cjs ARTIFACT_DIR WITNESS_DIR'); process.exit(2); }
const matrix = JSON.parse(fs.readFileSync(path.join(__dirname, 's2-qa-matrix.json'), 'utf8'));
const sha = b => crypto.createHash('sha256').update(b).digest('hex');
const projBytes = fs.readFileSync(path.join(WIT, matrix.witnessProjection.file));
if (sha(projBytes) !== matrix.witnessProjection.sha256) { console.error('witness projection hash differs from the matrix'); process.exit(2); }
const views = JSON.parse(projBytes).codeViews['code-p26'];
const f = { id: 'code-p26', selector: views[0].selector, codeText: views[0].text, nativeWheelEligible: true };
const clone = o => JSON.parse(JSON.stringify(o));
// Build a view of the same block scrolled to s with a given scrollWidth, from the unscrolled recorded view 0.
function viewAt(s, scrollWidth) {
  const v = clone(views[0]); v.code.scrollLeft = s; v.code.scrollWidth = scrollWidth; v.pre.scrollWidth = v.pre.clientWidth;
  for (const c of v.characters) c.x -= s;
  for (const l of v.lines) { l.left -= s; l.right -= s; }
  for (const r of v.scrollSurfaces) if (r.key === 'code') r.scrollLeft = s;
  return v;
}
let bad = 0, n = 0;
function ok(name, fn, expectFail) {
  n++; let err = null; try { fn(); } catch (e) { err = String(e.message || e); }
  const pass = expectFail ? (err !== null && (typeof expectFail !== 'string' || err.includes(expectFail))) : err === null;
  if (!pass) bad++;
  console.log(`${pass ? 'PASS' : 'FAIL'} ${name}${err ? '  -> ' + err.slice(0, 110) : ''}`);
}
// ---- Stepped views (modelled on the recorded failed case) ----
ok('recorded old-padding views: the whole-character union still fails on offsets 73 and 74 (known-bad input)', () => S.checkWalk(f, [views[0], views[1]], 320, 3), 'missing or occluded');
ok('recorded transition 0 -> 42 is a valid final step', () => S.steppedTransition(views[0], views[1], f, true, 320));
const A0 = viewAt(0, 452), A1 = viewAt(62, 452);
ok('new-padding model: range 62 needs one gesture and the union is complete', () => { const r = S.checkWalk(f, [A0, A1], 320, 3); if (r.plan.gestures !== 1 || !r.union.complete) throw Error('unexpected'); });
const B0 = viewAt(0, 752), B1 = viewAt(320, 752), B2 = viewAt(362, 752);
ok('range 362 needs two gestures (N total, not N plus one) and the union is complete', () => { const r = S.checkWalk(f, [B0, B1, B2], 320, 3); if (r.plan.gestures !== 2) throw Error('unexpected'); });
ok('range 362 with only one view: endpoint not reached within the planned gestures', () => S.checkWalk(f, [B0, viewAt(320, 752)], 320, 3), 'endpoint');
ok('range 362 with three gestures: more than planned', () => S.checkWalk(f, [B0, B1, B2, viewAt(362, 752)], 320, 3), 'more views');
ok('range 362 where the first step already hits the endpoint', () => S.steppedTransition(B0, viewAt(362, 752), f, false, 320), 'moved CODE by more');
ok('a step larger than the request', () => S.steppedTransition(B0, viewAt(330, 752), f, false, 320), 'more than the requested');
ok('range above the cap returns needs-review', () => S.checkWalk(f, [viewAt(0, 1400)], 320, 3), 'needs-review');
ok('range 0 plans zero gestures', () => { const p = S.plannedGestures(0, 320, 3); if (p.gestures !== 0 || p.needsReview) throw Error('x'); });
ok('range 0 does not rescue text that lies beyond the clip: the union still fails', () => S.checkWalk(f, [viewAt(0, 390)], 320, 3), 'missing or occluded');
const mut = (v, fn) => { const c = clone(v); fn(c); return c; };
ok('a step with vertical document motion', () => S.steppedTransition(B0, mut(B1, c => { c.scrollSurfaces.find(r => r.key === 'window').scrollTop += 5; }), f, false, 320), 'unexpected motion');
ok('a step that moves another scroll owner horizontally', () => S.steppedTransition(B0, mut(B1, c => { c.scrollSurfaces.find(r => r.key === 'pre').scrollLeft += 7; }), f, false, 320), 'unexpected motion');
ok('a step with no horizontal CODE motion', () => S.steppedTransition(B1, mut(B1, c => { c.utc = 'x'; }), f, false, 320), 'unexpected motion');
ok('a decreasing step', () => S.steppedTransition(B1, B0, f, false, 320), 'unexpected motion');
ok('final step that stops short of the endpoint', () => S.steppedTransition(B1, mut(viewAt(340, 752), c => {}), f, true, 320), 'endpoint not reached');
ok('a character that does not translate with the scroll', () => S.steppedTransition(B0, mut(B1, c => { c.characters[10].x += 3; }), f, false, 320), 'translating');
ok('style drift between steps (signature)', () => S.steppedTransition(B0, mut(B1, c => { c.nodeStyles[0].fontSize = '20px'; }), f, false, 320), 'style/font/geometry');
ok('text identity: a view whose text differs from the bound text', () => S.checkWalk(f, [mut(A0, c => { c.text = c.text.replace('estimated', 'estimateD'); }), A1], 320, 3), 'identity');
ok('gesture point not over the CODE', () => S.steppedTransition(mut(B0, c => { c.gesturePoint.insideCodeHit = false; }), B1, f, false, 320), 'step start unproved');
ok('first view not at scrollLeft 0', () => S.checkWalk(f, [B1], 320, 3), 'initial view');
// ---- Keyboard traversal ----
const t = (target, rect) => ({ tag: 'A', isTargetCopyControl: target, rect: rect === undefined ? { x: 1, y: 1, width: 24, height: 24 } : rect });
ok('real Tab traversal reaches the copy control (3 presses)', () => { const r = S.matchKeyboardTrace([t(false), t(false), t(true)], 150, { tabindexBefore: '0', tabindexAfter: '0' }); if (r.presses !== 3) throw Error('x'); });
ok('control never reached within the bound', () => S.matchKeyboardTrace(Array.from({ length: 150 }, () => t(false)), 150), 'not reached');
ok('more presses than the bound', () => S.matchKeyboardTrace(Array.from({ length: 151 }, (_, i) => t(i === 150)), 150), 'more Tab presses');
ok('traversal continued after the control was reached', () => S.matchKeyboardTrace([t(true), t(false)], 150), 'continued');
ok('focused control has no visible box', () => S.matchKeyboardTrace([t(false), t(true, { x: 0, y: 0, width: 0, height: 0 })], 150), 'no visible box');
ok('a tabindex changed during traversal (injection)', () => S.matchKeyboardTrace([t(true)], 150, { tabindexBefore: null, tabindexAfter: '0' }), 'tabindex');
ok('empty trace', () => S.matchKeyboardTrace([], 150), 'no keyboard trace');
// ---- Copy text identity ----
ok('copied text equals the block text', () => S.checkCopy({ hitTestOk: true, clipboard: f.codeText }, f.codeText));
ok('copied text differs by one character', () => S.checkCopy({ hitTestOk: true, clipboard: f.codeText.slice(0, -2) + 'X\n' }, f.codeText), 'differs');
ok('copy control not hit-testable', () => S.checkCopy({ hitTestOk: false, clipboard: f.codeText }, f.codeText), 'hit-testable');
// ---- Context-scaled padding (the e5b2 numbers) ----
const pad = (E, have) => ({ preFontSize: E + 'px', rootFontSize: '20px', codePaddingRight: have + 'px' });
ok('normal context: 38.02 px against a need of 36 px', () => S.checkPadding(pad(16, 38.016)));
ok('nested context: 32.41 px against a need of 30.4 px', () => S.checkPadding(pad(12.8, 32.41)));
ok('the first draft in nested code (29.92 px) is below the need', () => S.checkPadding(pad(12.8, 29.92)), 'below the toolbar need');
// ---- Synthetic page generator (real child processes on scratch copies) ----
const gen = path.join(__dirname, 's2-make-synthetic.cjs'), mpath = path.join(__dirname, 's2-qa-matrix.json');
const run = (dir, mp) => cp.spawnSync(process.execPath, [gen, dir, mp || mpath], { encoding: 'utf8' });
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 's2syn-'));
try {
  fs.cpSync(ART, path.join(tmp, 'full'), { recursive: true });
  let r = run(path.join(tmp, 'full'));
  ok('generator: full artifact copy passes every check', () => { if (r.status !== 0) throw Error(r.stderr); const j = JSON.parse(r.stdout); if (j.nestedTextSha256 !== matrix.cases.find(c => c.id === 'report-command').codeTextSha256) throw Error('hash'); });
  const bad = JSON.parse(fs.readFileSync(mpath, 'utf8')); bad.cases.find(c => c.id === 'synthetic-nested').codeTextSha256 = '0'.repeat(64);
  fs.writeFileSync(path.join(tmp, 'bad-matrix.json'), JSON.stringify(bad));
  r = run(path.join(tmp, 'full'), path.join(tmp, 'bad-matrix.json'));
  ok('generator: a wrong bound hash is rejected', () => { if (r.status !== 0) throw Error(r.stderr); }, 'bindings disagree');
  const mini = path.join(tmp, 'mini'), src = fs.readFileSync(path.join(ART, 'docs/adopt/tooling-and-tokens/index.html'), 'utf8');
  fs.mkdirSync(path.join(mini, 'docs/adopt/tooling-and-tokens'), { recursive: true });
  const re = /<div class="highlight">(?:(?!<div class="highlight">)[\s\S])*?<\/pre><\/div>/g;
  const target = [...src.matchAll(re)].find(m => m[0].replace(/<[^>]+>/g, '').includes('agov') && m[0].replace(/<[^>]+>/g, '').split('\n').filter(Boolean).length === 1 && m[0].replace(/<[^>]+>/g, '').split('\n')[0].length === 58);
  fs.writeFileSync(path.join(mini, 'docs/adopt/tooling-and-tokens/index.html'), src.slice(0, target.index) + target[0] + target[0] + src.slice(target.index + target[0].length));
  r = run(mini);
  ok('generator: two identical report-command blocks are rejected (exactly one match required)', () => { if (r.status !== 0) throw Error(r.stderr); }, 'exactly one');
  fs.writeFileSync(path.join(mini, 'docs/adopt/tooling-and-tokens/index.html'), src.slice(0, target.index) + target[0].replace('<span class="w"> </span>', '&nbsp;') + src.slice(target.index + target[0].length));
  r = run(mini);
  ok('generator: an unsupported entity in a highlight block is rejected', () => { if (r.status !== 0) throw Error(r.stderr); }, 'unsupported entity');
  fs.writeFileSync(path.join(mini, 'docs/adopt/tooling-and-tokens/index.html'), src);
  r = run(mini);
  ok('generator: relative assets that do not resolve from the destination are rejected', () => { if (r.status !== 0) throw Error(r.stderr); }, 'does not resolve');
} finally { fs.rmSync(tmp, { recursive: true, force: true }); }
console.log(`${n - bad} of ${n} fixtures behave as declared`);
process.exit(bad ? 1 : 0);
~~~

~~~ s2-qa-run.sh
#!/usr/bin/env bash
# Operator launcher for the S2 rendered QA. Runs the 20 CODE observations (5 cases x 4 modes) serially, one fresh browser each, no retry.
# Requires root's separate run decision AND a cleared S1 calibration. Missing terminal evidence is never a clean exit.
# usage: bash s2-qa-run.sh ARTIFACT_DIR OUT_DIR   (ARTIFACT_DIR is the extracted site-html directory of the bound S2 artifact; OUT_DIR is empty or absent)
# env, operator-local (none assumed): PLAYWRIGHT_MODULE, CHROMIUM_PATH, CHROMIUM_ARGS, NODE_BIN (default node)
# env, required: S2_ZIP_SHA256 and S2_HEAD_SHA (must equal the matrix), recorded in binding.txt.
# env, test seams only: SYSTEMD_RUN, SYSTEMCTL, CGROOT, S2_QA_POLL_SEC.
# Required next to this file, byte-identical to the hashes in the matrix and in s2-qa-hashes.sha256: the three frozen A3 helpers
# (code-measure.cjs, code-proof.cjs, scroll-evidence.cjs) and the frozen S1 static server (s1-qa-serve.cjs).
# Markdown fences do not keep file modes: run it with bash.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ART="${1:?artifact dir}"; OUT="${2:?output dir}"; case "$OUT" in /*) ;; *) OUT="$PWD/$OUT" ;; esac; NODE_BIN="${NODE_BIN:-node}"
SYSTEMD_RUN="${SYSTEMD_RUN:-systemd-run}"; SYSTEMCTL="${SYSTEMCTL:-systemctl}"; CGROOT="${CGROOT:-/sys/fs/cgroup}"
M="$HERE/s2-qa-matrix.json"
stop() { echo "STOP: $1" >&2; exit "${2:-1}"; }
mx() { "$NODE_BIN" -e "const m=require('$M');console.log($1)"; }
# 1. Package integrity: package files and the frozen A3/S1 helpers must match their recorded hashes.
( cd "$HERE" && sha256sum -c s2-qa-hashes.sha256 >/dev/null 2>&1 ) || stop "package files differ from s2-qa-hashes.sha256" 2
for H in code-measure.cjs code-proof.cjs scroll-evidence.cjs; do
  [ "$(sha256sum "$HERE/$H" 2>/dev/null | cut -c1-64)" = "$(mx "m.a3Helpers['$H']")" ] || stop "frozen A3 helper $H is missing or differs from the matrix hash" 2
done
[ "$(sha256sum "$HERE/s1-qa-serve.cjs" 2>/dev/null | cut -c1-64)" = "$(mx "m.s1Serve")" ] || stop "frozen S1 static server is missing or differs from the matrix hash" 2
# 2. Binding of the inputs the operator claims.
[ "${S2_ZIP_SHA256:-}" = "$(mx 'm.zipSha256')" ] || stop "S2_ZIP_SHA256 does not equal the matrix ZIP hash" 2
[ "${S2_HEAD_SHA:-}" = "$(mx 'm.head')" ] || stop "S2_HEAD_SHA does not equal the matrix head" 2
[ -d "$ART" ] || stop "no artifact dir" 2
[ "$(sha256sum "$ART/stylesheets/agf.css" 2>/dev/null | cut -c1-64)" != "" ] || stop "artifact has no stylesheets/agf.css" 2
[ ! -e "$OUT" ] || [ -z "$(ls -A "$OUT" 2>/dev/null)" ] || stop "output dir must be empty" 2
mkdir -p "$OUT/obs" "$OUT/units"
TREE="$(cd "$ART" && find . -type f | LC_ALL=C sort | xargs sha256sum | sha256sum | cut -c1-64)"
# 3. No foreign or leftover unit with our name prefix may exist before we start.
FOREIGN="$($SYSTEMCTL --user list-units --all --no-legend 'agf-s2qa-*' 2>/dev/null | awk 'NF{print $1}')"
[ -z "$FOREIGN" ] || stop "existing agf-s2qa unit(s): $FOREIGN" 6
# 4. Offline package self-test, then the scratch copy and the synthetic nested page (full-hash bound, post-generation verified).
"$NODE_BIN" "$HERE/s2-qa-selftest.cjs" "$ART" "${S2_WITNESS_DIR:?set S2_WITNESS_DIR to the directory holding saved-code-views-projection.json}" > "$OUT/selftest.txt" || stop "package self-test failed" 2
cp -r "$ART" "$OUT/site" || stop "could not copy the artifact" 2
"$NODE_BIN" "$HERE/s2-make-synthetic.cjs" "$OUT/site" "$M" > "$OUT/synthetic.json" || stop "synthetic page generation or verification failed" 2
{ echo "zip_sha256=$S2_ZIP_SHA256"; echo "head=$S2_HEAD_SHA"; echo "artifact_tree_sha256=$TREE"; echo "synthetic=$(cat "$OUT/synthetic.json")"; echo "package_hashes:"; cat "$HERE/s2-qa-hashes.sha256"; } > "$OUT/binding.txt"
# 5. Local server over the scratch copy.
"$NODE_BIN" -e "const {startServer}=require('$HERE/s1-qa-serve.cjs');const m=require('$M');startServer('$OUT/site',m.prefix).then(({port})=>require('fs').writeFileSync('$OUT/port',String(port)))" > "$OUT/server.log" 2>&1 &
SRV=$!; trap 'kill $SRV 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do [ -s "$OUT/port" ] && break; sleep 0.1; done
PORT="$(cat "$OUT/port" 2>/dev/null)"; [ -n "$PORT" ] || stop "server did not start" 2
MEM_MAX="$(mx 'm.caps.memoryMax')"; SWAP_MAX="$(mx 'm.caps.swapMax')"; RUN_SEC="$(mx 'm.caps.runtimeSec')"; OLD="$(mx 'm.caps.nodeOldSpaceMiB')"; FLOOR="$(mx 'm.caps.hostMemAvailableFloorMiB')"
RUNID="$(date -u +%Y%m%dT%H%M%S)-$$"; N=0
get() { sed -n "s/^$2=//p" "$1" | head -1; }
# A ControlGroup is acceptable only if it is a non-empty absolute path, has no ".." or empty component, and ends in exactly "<unit>.service".
valid_cg() { # $1 cgroup path, $2 unit service name
  case "$1" in /*) ;; *) return 1 ;; esac
  case "$1/" in */../*|*//*) return 1 ;; esac
  [ "${1##*/}" = "$2" ]
}
inventory() { # $1 cgroup path, $2 output file: exact member list, or the word "absent" if the cgroup no longer exists
  if [ -e "$CGROOT$1/cgroup.procs" ]; then cat "$CGROOT$1/cgroup.procs" > "$2" 2>/dev/null || echo "unreadable" > "$2"; else echo "absent" > "$2"; fi
}
for ROUTE in $(mx "m.cases.map(r=>r.id).join(' ')"); do
  for MODE in $(mx "m.modes.map(r=>r.id).join(' ')"); do
    N=$((N+1)); NN="$(printf %02d "$N")"; UNIT="agf-s2qa-$RUNID-$NN"; SVC="$UNIT.service"; U="$OUT/units/$UNIT"
    A="$(awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo)"
    [ "$A" -ge "$FLOOR" ] || stop "host MemAvailable ${A} MiB below floor ${FLOOR} MiB before observation $N" 3
    # One observation in one named transient unit, started without waiting so its cgroup can be read while it runs.
    # RemainAfterExit=yes keeps a successfully finished unit loaded (active, exited) until we stop it, so its identity and terminal
    # properties stay readable. A failed unit stays loaded by default. Output goes to files owned by this unit.
    "$SYSTEMD_RUN" --user --unit="$UNIT" --no-block --quiet \
      -p MemoryMax="$MEM_MAX" -p MemorySwapMax="$SWAP_MAX" -p OOMPolicy=kill -p RuntimeMaxSec="$RUN_SEC" -p RemainAfterExit=yes \
      -p StandardOutput="file:$U.stdout" -p StandardError="file:$U.stderr" \
      --setenv=PLAYWRIGHT_MODULE="${PLAYWRIGHT_MODULE:-playwright}" --setenv=CHROMIUM_PATH="${CHROMIUM_PATH:-}" --setenv=CHROMIUM_ARGS="${CHROMIUM_ARGS:-}" \
      "$NODE_BIN" --max-old-space-size="$OLD" "$HERE/s2-qa-observe.cjs" --matrix "$M" --case "$ROUTE" --mode "$MODE" --base "http://127.0.0.1:$PORT" --out "$OUT/obs" \
      > "$U.start" 2>&1 || stop "systemd-run could not start $UNIT: $(cat "$U.start")" 7
    # Poll this unit only. Remember the first non-empty ControlGroup seen while it ran.
    CG_RUN=""; DEADLINE=$(( $(date +%s) + ${S2_QA_POLL_SEC:-$((RUN_SEC + 20))} )); FINISHED=0
    while [ "$(date +%s)" -le "$DEADLINE" ]; do
      "$SYSTEMCTL" --user show "$SVC" -p ActiveState,SubState,InvocationID,ControlGroup > "$U.poll" 2>/dev/null
      P_ACT="$(get "$U.poll" ActiveState)"; P_SUB="$(get "$U.poll" SubState)"; P_INV="$(get "$U.poll" InvocationID)"; P_CG="$(get "$U.poll" ControlGroup)"
      [ -n "$P_CG" ] && [ -z "$CG_RUN" ] && CG_RUN="$P_CG"
      # Finished = it ran (InvocationID set) and is failed, inactive, or active with SubState exited. Active/running is not finished.
      if [ -n "$P_INV" ] && { [ "$P_ACT" = inactive ] || [ "$P_ACT" = failed ] || { [ "$P_ACT" = active ] && [ "$P_SUB" = exited ]; }; }; then FINISHED=1; break; fi
      sleep 0.2
    done
    # Terminal evidence for THIS unit, collected on success and on failure.
    "$SYSTEMCTL" --user show "$SVC" -p Id,InvocationID,LoadState,Result,ActiveState,SubState,ExecMainStatus,ExecMainCode,MemoryPeak,ControlGroup > "$U.show" 2>/dev/null
    ID="$(get "$U.show" Id)"; RES="$(get "$U.show" Result)"; ST="$(get "$U.show" ExecMainStatus)"; ACT="$(get "$U.show" ActiveState)"; SUB="$(get "$U.show" SubState)"; LOAD="$(get "$U.show" LoadState)"; CG_END="$(get "$U.show" ControlGroup)"; INV="$(get "$U.show" InvocationID)"
    CG="${CG_END:-$CG_RUN}"
    cat "$U.stdout" >> "$OUT/observations.log" 2>/dev/null
    { echo "cg_while_running=$CG_RUN"; echo "cg_terminal=$CG_END"; echo "cg_used=$CG"; echo "finished_in_time=$FINISHED"; } > "$U.scope"
    VALID=1
    [ "$FINISHED" = 1 ] && [ "$ID" = "$SVC" ] && [ -n "$INV" ] && [ -n "$RES" ] && [ -n "$ST" ] || VALID=0
    [ -n "$CG_END" ] && [ -n "$CG_RUN" ] && [ "$CG_END" != "$CG_RUN" ] && VALID=0
    valid_cg "$CG" "$SVC" || VALID=0
    [ "$LOAD" = loaded ] || VALID=0   # a not-found or unloaded unit carries default values, never real exit evidence
    case "$ACT/$SUB" in failed/*|inactive/*|active/exited) ;; *) VALID=0 ;; esac
    # Exact owned-unit process inventory before cleanup, then cleanup with checked results, then the inventory again.
    if valid_cg "$CG" "$SVC"; then inventory "$CG" "$U.procs-before"; else echo "unavailable" > "$U.procs-before"; fi
    CLEAN=0
    if "$SYSTEMCTL" --user stop "$SVC" >> "$U.cleanup" 2>&1; then
      "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$U.after" 2>/dev/null
      if [ "$(get "$U.after" ActiveState)" = failed ]; then
        "$SYSTEMCTL" --user reset-failed "$SVC" >> "$U.cleanup" 2>&1 && "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$U.after" 2>/dev/null
      fi
      L2="$(get "$U.after" LoadState)"; A2="$(get "$U.after" ActiveState)"
      { [ "$L2" = not-found ] || [ "$A2" = inactive ]; } && CLEAN=1
    fi
    if valid_cg "$CG" "$SVC"; then inventory "$CG" "$U.procs-after"; else echo "unavailable" > "$U.procs-after"; fi
    [ "$VALID" = 1 ] || stop "missing, foreign or invalid terminal scope evidence for $UNIT (finished=$FINISHED Id='$ID' InvocationID='$INV' Result='$RES' ExecMainStatus='$ST' ActiveState='$ACT' ControlGroup='$CG'); observation $N ($ROUTE/$MODE). Unavailable evidence is not a pass." 7
    [ "$CLEAN" = 1 ] || stop "cleanup of $UNIT did not return clean (see $U.cleanup and $U.after)" 5
    case "$(cat "$U.procs-after")" in ""|absent) ;; *) stop "processes remain in $UNIT's cgroup after cleanup" 5 ;; esac
    case "$(cat "$U.procs-before")" in ""|absent) ;; *) stop "processes were still in $UNIT's cgroup after its main process ended (cleaned afterwards)" 5 ;; esac
    if [ "$RES" != "success" ] || [ "$ST" != "0" ]; then stop "observation $N ($ROUTE/$MODE) failed: Result=$RES ExecMainStatus=$ST. No retry." 4; fi
  done
done
# 6. Final inventory: exactly the declared observations and images, hashed.
NJ="$(find "$OUT/obs" -name '*.json' | wc -l)"; NI="$(find "$OUT/obs" -name '*.jpg' | wc -l)"
WJ="$(mx "m.cases.length*m.modes.length")"; WI="$(mx "m.cases.reduce((a,r)=>a+r.images.length,0)")"
[ "$NJ" = "$WJ" ] && [ "$NI" = "$WI" ] || stop "inventory mismatch: $NJ json (want $WJ), $NI images (want $WI)" 8
( cd "$OUT" && find obs units -type f | LC_ALL=C sort | xargs sha256sum > inventory.sha256 )
echo "completed $N observations; $NJ json, $NI images; inventory.sha256 written" | tee -a "$OUT/observations.log"
~~~

~~~ s2-qa-run-selftest.sh
#!/usr/bin/env bash
# Offline test of the launcher's control flow with fake systemd-run and systemctl. No browser, no unit, no root.
# usage: bash s2-qa-run-selftest.sh ARTIFACT_DIR WITNESS_DIR     Exit 0 only if every scenario ends with the expected launcher exit code.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; ART="${1:?artifact dir}"; export S2_WITNESS_DIR="${2:?witness dir}"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
mkdir -p "$T/bin" "$T/state" "$T/cg"
cat > "$T/bin/systemd-run" <<'FAKE'
#!/usr/bin/env bash
UNIT=""; OUTF=""; ARGS=("$@"); i=0; CMD=()
while [ $i -lt ${#ARGS[@]} ]; do a="${ARGS[$i]}"
  case "$a" in --unit=*) UNIT="${a#--unit=}";; -p) i=$((i+1)); case "${ARGS[$i]}" in StandardOutput=file:*) OUTF="${ARGS[$i]#StandardOutput=file:}";; esac;; --*) ;; *) CMD=("${ARGS[@]:$i}"); break;; esac
  i=$((i+1)); done
N=$(( $(cat "$FAKE_STATE/count" 2>/dev/null || echo 0) + 1 )); echo $N > "$FAKE_STATE/count"
get() { for ((k=0;k<${#CMD[@]};k++)); do [ "${CMD[$k]}" = "$1" ] && echo "${CMD[$((k+1))]}"; done; }
R="$(get --case)"; M="$(get --mode)"; O="$(get --out)"
RES=success; ST=0; ID="$UNIT.service"; CG="/user.slice/$UNIT.service"; ACT=active; SUB=exited; LOAD=loaded; INVID="abc$N"
[ "$FAKE_SCENARIO" = "fail5" ] && [ "$N" = 5 ] && { RES=exit-code; ST=1; ACT=failed; SUB=failed; }
case "$FAKE_SCENARIO" in
  wrongid) ID="agf-s2qa-someone-else.service";;
  emptycg) CG="";;
  foreignsuffix) CG="/user.slice/$UNIT.service.evil";;
  foreignprefix) CG="/user.slice/x-$UNIT.service";;
  traversal) CG="/user.slice/../$UNIT.service";;
  relative) CG="user.slice/$UNIT.service";;
  gone) LOAD=not-found; ACT=inactive; SUB=dead; INVID=""; CG=""; ID="$UNIT.service";;   # the real qpr7 failure: unloaded unit, default values
  running) ACT=active; SUB=running;;
esac
mkdir -p "$O"; echo '{}' > "$O/$R-$M.json"
if node -e "const m=require('$FAKE_MATRIX');process.exit(m.cases.find(r=>r.id==='$R').images.includes('$M')?0:1)"; then : > "$O/$R-$M.jpg"; fi
echo "{\"route\":\"$R\",\"mode\":\"$M\",\"failures\":[]}" > "$OUTF"
case "$FAKE_SCENARIO" in
  leftover|leftoverpersist) mkdir -p "$FAKE_CGROOT$CG"; echo 4242 > "$FAKE_CGROOT$CG/cgroup.procs"; echo "$FAKE_CGROOT$CG/cgroup.procs" > "$FAKE_STATE/procfile";;
  emptyprocs) mkdir -p "$FAKE_CGROOT$CG"; : > "$FAKE_CGROOT$CG/cgroup.procs";;
esac
if [ "$FAKE_SCENARIO" != "noterminal" ]; then
  printf 'Id=%s\nInvocationID=%s\nLoadState=%s\nResult=%s\nActiveState=%s\nSubState=%s\nExecMainStatus=%s\nExecMainCode=1\nMemoryPeak=1000\nControlGroup=%s\n' "$ID" "$INVID" "$LOAD" "$RES" "$ACT" "$SUB" "$ST" "$CG" > "$FAKE_STATE/$UNIT.show"
fi
exit 0
FAKE
cat > "$T/bin/systemctl" <<'FAKE'
#!/usr/bin/env bash
case "$2" in
  list-units) [ "$FAKE_SCENARIO" = "foreign" ] && echo "agf-s2qa-other.service loaded active running x"; exit 0;;
  show) f="$FAKE_STATE/${3%.service}.show"
        if [ -f "$FAKE_STATE/${3%.service}.stopped" ]; then printf 'LoadState=not-found\nActiveState=inactive\n'; exit 0; fi
        [ -f "$f" ] && cat "$f"; exit 0;;
  stop) [ "$FAKE_SCENARIO" = "cleanfail" ] && exit 1
        : > "$FAKE_STATE/${3%.service}.stopped"
        [ "$FAKE_SCENARIO" = "leftover" ] && [ -f "$FAKE_STATE/procfile" ] && : > "$(cat "$FAKE_STATE/procfile")"
        exit 0;;
  *) exit 0;;
esac
FAKE
chmod +x "$T/bin/"*
# A private copy of the package, so a tamper case cannot touch the real files.
mkdir -p "$T/pkg" && cp "$HERE"/s2-qa-* "$HERE"/s2-make-synthetic.cjs "$HERE"/s1-qa-serve.cjs "$HERE"/code-measure.cjs "$HERE"/code-proof.cjs "$HERE"/scroll-evidence.cjs "$HERE"/real-cases.json "$T/pkg/"
ZIP="$(node -e "console.log(require('$T/pkg/s2-qa-matrix.json').zipSha256)")"; HEAD="$(node -e "console.log(require('$T/pkg/s2-qa-matrix.json').head)")"
bad=0
run() { # name expected_rc scenario [env assignments...]
  local name="$1" want="$2" sc="$3"; shift 3
  rm -rf "$T/out" "$T/state"/*; mkdir -p "$T/state"
  env "$@" FAKE_SCENARIO="$sc" FAKE_STATE="$T/state" FAKE_MATRIX="$T/pkg/s2-qa-matrix.json" FAKE_CGROOT="$T/cg" CGROOT="$T/cg" \
      S2_QA_POLL_SEC=2 PATH="$T/bin:$PATH" SYSTEMD_RUN="$T/bin/systemd-run" SYSTEMCTL="$T/bin/systemctl" \
      bash "$T/pkg/s2-qa-run.sh" "$ART" "$T/out" > "$T/run.log" 2>&1
  local rc=$?; local ok=FAIL; [ "$rc" = "$want" ] && ok=PASS; [ "$ok" = FAIL ] && bad=$((bad+1)); [ "$ok" = FAIL ] && tail -2 "$T/run.log" | sed "s/^/     log: /"
  echo "$ok $name (exit $rc, want $want)"
}
GOOD=(S2_ZIP_SHA256="$ZIP" S2_HEAD_SHA="$HEAD")
run "all 20 observations succeed and the inventory matches (cgroup removed after exit)" 0 ok "${GOOD[@]}"
if [ -f "$T/out/inventory.sha256" ]; then echo "     inventory: $(grep -c '\.json' "$T/out/inventory.sha256") json lines, $(grep -c '\.jpg' "$T/out/inventory.sha256") jpg lines, $(ls "$T/out/units"/*.show | wc -l) unit show files, $(ls "$T/out/units"/*.procs-after | wc -l) cleanup inventories"; fi
run "an existing but empty cgroup after exit is accepted" 0 emptyprocs "${GOOD[@]}"
run "a non-zero observation stops the run, with terminal evidence collected" 4 fail5 "${GOOD[@]}"
echo "     unit show files after the stop: $(ls "$T/out/units"/*.show 2>/dev/null | wc -l) (want 5); cleanup inventories: $(ls "$T/out/units"/*.procs-after 2>/dev/null | wc -l) (want 5)"; [ "$(ls "$T/out/units"/*.show 2>/dev/null | wc -l)" = 5 ] && [ "$(ls "$T/out/units"/*.procs-after 2>/dev/null | wc -l)" = 5 ] || bad=$((bad+1))
run "missing terminal evidence is not a clean exit" 7 noterminal "${GOOD[@]}"
run "terminal evidence for a different unit is rejected" 7 wrongid "${GOOD[@]}"
run "an empty ControlGroup is unavailable evidence, not a pass" 7 emptycg "${GOOD[@]}"
run "a ControlGroup with a foreign suffix is rejected" 7 foreignsuffix "${GOOD[@]}"
run "a ControlGroup with a foreign prefix in its last component is rejected" 7 foreignprefix "${GOOD[@]}"
run "a ControlGroup with path traversal is rejected" 7 traversal "${GOOD[@]}"
run "a relative ControlGroup is rejected" 7 relative "${GOOD[@]}"
run "a failing cleanup command stops the run" 5 cleanfail "${GOOD[@]}"
run "processes left in the cgroup that cleanup removes still stop the run" 5 leftover "${GOOD[@]}"
run "processes that survive cleanup stop the run" 5 leftoverpersist "${GOOD[@]}"
run "the real qpr7 failure pattern (unit already unloaded, empty identity, default success fields) is rejected" 7 gone "${GOOD[@]}"
run "a unit that never finishes within the bound is rejected" 7 running "${GOOD[@]}"
run "a pre-existing agf-s2qa unit stops the run before any observation" 6 foreign "${GOOD[@]}"
run "a wrong ZIP hash is rejected" 2 ok S2_ZIP_SHA256=deadbeef S2_HEAD_SHA="$HEAD"
run "a wrong head is rejected" 2 ok S2_ZIP_SHA256="$ZIP" S2_HEAD_SHA=deadbeef
echo '// tampered' >> "$T/pkg/s2-qa-stepped.cjs"
run "a package file that differs from the frozen hashes is rejected" 2 ok "${GOOD[@]}"
exit $((bad ? 1 : 0))
~~~

~~~ real-cases.json
{
 "formula": {
  "page": "docs/adopt/tooling-and-tokens",
  "ordinal": 27,
  "codeText": "estimated_tokens_saved =\n  tokens(manual_context_that_would_have_been_read)\n  - tokens(tool_output_that_was_read)\n  - tokens(extra_prompt_needed_to_invoke_the_tool)\n",
  "sha256": "5bc6bd8deeaa98eec3285cafdf689126c84c145cd193095ec7b6d2388f03dd24",
  "rows": 4,
  "maxRow": 50,
  "bytes": 165
 },
 "exports": {
  "page": "docs/adopt/tooling-and-tokens",
  "ordinal": 31,
  "codeText": "export AGOV_TOKEN_ROI_LOG=\"$PWD/tmp/codex-agov-token-roi.jsonl\"\n# or, for a Claude-run shell:\nexport AGOV_TOKEN_ROI_LOG=\"$PWD/tmp/claude-agov-token-roi.jsonl\"\n",
  "sha256": "8b632bc1d768a595b792395063d8fb858269f4e69be93d2a9f9fcd5a487709e5",
  "rows": 3,
  "maxRow": 64,
  "bytes": 159
 },
 "report-command": {
  "page": "docs/adopt/tooling-and-tokens",
  "ordinal": 33,
  "codeText": "agov token roi report --log tmp/codex-agov-token-roi.jsonl\n",
  "sha256": "a5b44b06bf5f4d23f9cbee59f8460bccad260c38a59ca46befb249b9f234b549",
  "rows": 1,
  "maxRow": 58,
  "bytes": 59
 },
 "long-row": {
  "page": "reference/review-findings",
  "ordinal": 14,
  "codeText": "Finding: <identifier>\nImpact: critical | material | minor | unknown\nEnforcement: blocking | advisory\nBasis: <named rule/requirement, or explicitly advisory judgment>\nFinding state: open | resolved | superseded\nRisk disposition: none | accepted\nRisk decision: <required when accepted: decision ID, scope/revision, revisit condition>\nResolution condition: <observable condition>\n",
  "sha256": "b5f413effbbd6bfa5293dd788db7e6aea596806f2e907d8dab086d7c5e2edb38",
  "rows": 8,
  "maxRow": 87,
  "bytes": 377
 }
}
~~~

~~~ s2-qa-hashes.sha256
3546a0396305d6b4e6085393de7f1aa4bd03887192f545866491560bd0a1a15e  code-measure.cjs
2859dd21cac883249e9b2dd98f73e2dc80fc388ebad5168d03d1dc7949a846df  code-proof.cjs
f22ff058196d788c635c5ace6b2d64e3b0bef3f4bf584307ca10198122d717ab  real-cases.json
40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06  s1-qa-serve.cjs
d3bc469540e27b3fbb608321029a89c68819941f2a619995dfb8d9d6344502bf  s2-make-synthetic.cjs
7b0edf98899c722f20b4b4e96caa26871e0d6f3dd0bae9f254ab19179bdd1d0e  s2-qa-matrix.json
1b01ea7d8f6681e834dc77d371f5d2eb0ebe2f65db2f49be2489e19b00bfc56c  s2-qa-observe.cjs
9bcc9d9c9e39c23a7f0d592ad6ee09b302ea9df75803dbb8b5c6557acd611cb5  s2-qa-run-selftest.sh
af614b705605006f78807c242d4058f06fc4af3cb481907b1bb5e1a1bda7a0e5  s2-qa-run.sh
d1bf5f4e2a91135d7cd05bfa024ce32ffc615c0428add293f788eae01e60a35c  s2-qa-selftest.cjs
89bfe4728ff705df8b879705e809c09df19f669e1728b3b380b8e4836f12b12e  s2-qa-stepped.cjs
548564e5220d101974705e934bd70a6ff6972c7ac37e1a3a26337e50e54e469b  scroll-evidence.cjs
~~~

## Run command (for the later run decision only)

```
S2_ZIP_SHA256=<matrix zip> S2_HEAD_SHA=<matrix head> S2_WITNESS_DIR=<dir with saved-code-views-projection.json> bash s2-qa-run.sh ARTIFACT_DIR OUT_DIR
```

Nothing here authorizes that command.

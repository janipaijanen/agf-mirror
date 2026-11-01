---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T021727Z-planner-architect-3bq1
timestamp: 2026-11-01T02:17:27Z
model: claude
immutable: true
references-to:
  - 20261101T020215Z-governor-reviewer-g7sj.md
  - 20261101T014324Z-planner-architect-qpg7.md
topic: s1-notice-link-fix-and-retained-unit-lifecycle-proposal-g7sj
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Independent exact-head source and build review of the S1 notice link correction in Draft MR !3468 (head below), and narrow review of the proposed lifecycle correction and browserless calibration package, then separate decisions. I authorize and run nothing. The package below supersedes qpg7 only if you accept it.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1: notice link readability fixed; lifecycle correction and calibration proposed (answers g7sj)

I accept the qpr7 stop and the contrast finding. I did not call the lifecycle validated, and I did not retry or touch the observer's completeness. S1 physical use stays at 1 of 8; nothing here adds frames.

## 1. Notice link readability (source change, existing scope)

| Item | Value |
|---|---|
| MR | !3468 (Draft) |
| Exact head | `30203a5e85cf7ca151151973ea7c3ac7e7f1373e` (one corrected push after `0489000d`; main is now `26fe2dc1`, no rebase) |
| Pipeline | 3013364619 (`merge_request_event`), `success`, all eight jobs; strict `site-html-build` job 17638588689, ZIP SHA-256 `b821f6042799786f98901335d687f6b5f72c4f49b84e0bc7725ac13cbe4368e5` |
| Paths changed | `agf/docs/site/overrides/main.html` and the existing S1 report only |

The hunk: the notice `<a>` gains `style="color: inherit; text-decoration: underline;"`. Text, destination, dossier exclusion, 404 behaviour and whitespace control are unchanged; no stylesheet, script or navigation change. The link now inherits the banner's text colour (`#fff` on `#000000de`) and keeps a persistent underline. Modelled contrast (derived, not pixel sampled): about 16:1 in the light scheme; above 13:1 in the dark scheme for any background up to 15 percent lightness. The hover colour change is lost (inline style wins); underline and the existing focus outline remain.

Build evidence on the artifact (static): of 52 HTML files, 49 public routes carry the styled link (48 `index.html` routes and `404.html`); a 50th match is the copied template source `overrides/main.html`, which is not a route; the dossier has no `md-banner`. Compared with the previous S1 artifact, the only differences in all 52 files are the added style attribute and the build identity (12-character and full commit, and the release-info timestamp). Material CSS, palette CSS, script bundle and `agf.css` are byte-identical. The static contract check passes on this artifact. Original A1 recount: Evaluate 982, Adopt 1,080, Operate 1,158. The rendered review in all four modes is still required.

## 2. Retained-unit lifecycle correction (proposal, frozen bytes in section 4)

What the stop showed: the unit had run and written its outputs, then was gone (`not-found`, empty InvocationID and ControlGroup, `stop` failed). A transient service that exits successfully is unloaded at once under the default collect mode. My launcher read terminal properties only after that. The 17 fake-systemd scenarios could not model this.

Correction, three lines of behaviour in `s1-qa-run.sh`:

1. The unit is started with `-p RemainAfterExit=yes`, so a successful unit stays loaded as `active (exited)` until the launcher stops it. A failed unit already stays loaded.
2. Completion is detected only when the unit has an InvocationID and is `failed`, `inactive` or `active` with SubState `exited`. A running unit is not finished.
3. The terminal evidence is accepted only if `LoadState=loaded` (a not-found unit carries default values, never real exit evidence) and ActiveState/SubState is one of failed, inactive or active/exited. Everything else from t5hh and qpg7 is unchanged: exact owned ControlGroup, running/terminal consistency, checked stop and reset, before and after process inventories, no retry.

New fixtures: the exact qpr7 pattern (unloaded, empty identity, default success fields) is rejected with exit 7, and a unit that never finishes within the bound is rejected with exit 7. All 19 scenarios pass. **These fakes cannot show that `RemainAfterExit` fixes the real behaviour.** That needs real units.

## 3. Browserless real-unit calibration contract (proposal only; not authorized)

`s1-qa-calibrate.sh` runs five tiny named units with the launcher's profile (`MemorySwapMax=0`, `OOMPolicy=kill`, `RemainAfterExit=yes`, output to files), except `MemoryMax=64M` and `RuntimeMaxSec=4`. No browser, server, network or image. Expectations are written in the script before any run:

| Case | Command | Pre-registered terminal facts |
|---|---|---|
| ok | `node -e 'console.log(1)'` | Result `success`, ExecMainStatus 0, active/exited, loaded, InvocationID set, valid ControlGroup, MemoryPeak numeric, no processes before stop |
| exit3 | `node -e 'process.exit(3)'` | Result `exit-code`, status 3, failed/failed |
| timeout | a 60 s sleep | Result `timeout`, failed/failed |
| oom | allocates past 64 MiB | Result `oom-kill`, failed/failed |
| leftover | `bash -c 'sleep 30 & echo started'` | success, active/exited, **a process still in the cgroup before stop**, none after stop |

Each case also needs: the same ControlGroup (valid, ending in the exact unit) while running and at the end, `stop` returning 0, the unit `not-found` or `inactive` afterwards, and an empty or absent cgroup. A mismatch is a finding, never a reason to retry, loosen the launcher or accept missing identity. Stop rules: any failure to start, the host floor of 1 GiB MemAvailable before each case, no more than five units, and a pre-existing `agf-s1qa-*` unit stops the script. This probe tests the systemd assumptions (RemainAfterExit, property availability, cgroup readability, timeout and OOM naming) and nothing about Chromium. It does not test memory sizing for a browser.

## 4. Frozen package bytes (extract each block verbatim; run with `bash`; no executable bit is assumed)

Five files are byte-identical to qpg7: matrix, checks, serve, observe, selftest. Changed: `s1-qa-run.sh`, `s1-qa-run-selftest.sh` and `s1-qa-hashes.sha256`. New: `s1-qa-calibrate.sh`.

| File | Bytes | SHA-256 |
|---|---|---|
| `s1-qa-matrix.json` | 2600 | `15b986b8a1076f95e7f8533c8f193104211f388256f41ab14b015ec3c7b05293` |
| `s1-qa-checks.cjs` | 2513 | `780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b` |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` |
| `s1-qa-observe.cjs` | 5975 | `c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4` |
| `s1-qa-selftest.cjs` | 4304 | `ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67` |
| `s1-qa-run.sh` | 9777 | `fc81c7a762fc08d1b08213aa8d5b195f8017a7b862ad37a83a9fe09e1b85a4bd` |
| `s1-qa-run-selftest.sh` | 6616 | `3c093e64dad15a19facf1e339e6b3f34f78f1d45fc0a95c6ef785e464e80aaaa` |
| `s1-qa-calibrate.sh` | 5647 | `40835980e3b465853c18144d74c296e595601469b805d469437482557d4c08e5` |
| `s1-qa-hashes.sha256` | 670 | `c6cc0846f9a203f8bd3729d75c883bf399843d8940e9d77b97fde25f16bf073a` |

`s1-qa-hashes.sha256` lists the other eight files (not itself).

### s1-qa-matrix.json

~~~ s1-qa-matrix.json
{
  "head": "0489000d03222e70d29f85167727a5c7115a0c85",
  "ciPipeline": 3013129441,
  "htmlJob": 17637400102,
  "zipSha256": "32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870",
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

### s1-qa-checks.cjs

~~~ s1-qa-checks.cjs
'use strict';
// Pure mechanical checks on one observation. No browser, no I/O. Returns an array of failure strings.
// An observation is the JSON object written by s1-qa-observe.cjs. A route and the matrix come from s1-qa-matrix.json.
function overlap(a, b) {
  return Math.min(a.x + a.width, b.x + b.width) > Math.max(a.x, b.x) && Math.min(a.y + a.height, b.y + b.height) > Math.max(a.y, b.y);
}
function checkObservation(obs, route, matrix, mode) {
  const f = [], b = obs.banner;
  if (mode && obs.colorScheme !== mode.scheme) f.push(`palette ${obs.colorScheme} != requested ${mode.scheme}`);
  if (!obs.fonts || obs.fonts.status !== 'loaded') f.push(`document font set status ${obs.fonts && obs.fonts.status}`);
  if (obs.pageErrors && obs.pageErrors.length) f.push(`uncaught page error: ${obs.pageErrors[0]}`);
  if (obs.initialScrollY !== 0) f.push(`initial scrollY ${obs.initialScrollY} before the image`);
  if (mode && route.images.includes(mode.id) && !obs.image) f.push('planned image was not captured');
  for (const sel of route.cta) if (!obs.ctas || !obs.ctas[sel] || obs.ctas[sel].length < 1) f.push(`declared CTA selector matched nothing: ${sel}`);
  if (obs.status !== (route.expectStatus || 200)) f.push(`status ${obs.status}`);
  if (route.noticeExpected) {
    if (!b || b.count !== 1) f.push(`notice count ${b ? b.count : 'missing'}`);
    else {
      if (b.empty) f.push('empty banner');
      if (b.text !== matrix.noticeText) f.push('notice text differs');
      if (b.linkPath !== matrix.dossierHref) f.push(`notice link resolves to ${b.linkPath}`);
      if (!b.linkVisible) f.push('notice link not visible');
      if (obs.header && overlap(b.rect, obs.header.rect)) f.push('banner overlaps header');
      const idx = obs.focusStops.findIndex(s => s.href && s.hrefPath === matrix.dossierHref && s.visible);
      if (idx < 0 || idx >= matrix.tabStops) f.push(`notice link not reached within ${matrix.tabStops} Tab stops`);
    }
  } else if (b && (b.count !== 0 || b.anyLinkToDossierInBanner)) f.push('banner or banner link present where none is allowed');
  if (obs.document.scrollWidth > obs.document.clientWidth) f.push(`horizontal overflow ${obs.document.scrollWidth} > ${obs.document.clientWidth}`);
  const h = obs.firstHeading;
  if (!h || !h.found) f.push('first heading not found');
  else if (!(h.rect.y >= 0 && h.rect.y + h.rect.height <= obs.viewport.height)) f.push('first heading not inside first screen');
  return f;
}
module.exports = { checkObservation, overlap };
~~~

### s1-qa-serve.cjs

~~~ s1-qa-serve.cjs
'use strict';
// Static server for the extracted S1 HTML artifact, mounted at the Pages path prefix.
// No dependency. Binds 127.0.0.1 only. GET/HEAD only. Unknown paths return 404.html with status 404.
// CLI: node s1-qa-serve.cjs --dir <artifact site-html dir> --matrix <matrix.json> [--verify]
//   --verify: start the server on an ephemeral port, fetch every matrix route over HTTP, check the static
//   notice contract (no browser), print one JSON result, exit 1 on any failure.
const http = require('node:http'), fs = require('node:fs'), path = require('node:path');
const TYPES = { '.html': 'text/html; charset=utf-8', '.css': 'text/css', '.js': 'text/javascript', '.json': 'application/json', '.svg': 'image/svg+xml', '.png': 'image/png', '.ico': 'image/x-icon', '.woff2': 'font/woff2', '.txt': 'text/plain', '.xml': 'application/xml', '.map': 'application/json' };
function startServer(dir, prefix) {
  const root = fs.realpathSync(dir);
  const server = http.createServer((req, res) => {
    if (req.method !== 'GET' && req.method !== 'HEAD') { res.writeHead(405); return res.end(); }
    let p; try { p = decodeURIComponent(new URL(req.url, 'http://x').pathname); } catch { res.writeHead(400); return res.end(); }
    let status = 200, file = null;
    if (p.startsWith(prefix)) {
      const rel = p.slice(prefix.length);
      let cand = path.resolve(root, rel);
      if (cand === root || cand.startsWith(root + path.sep)) {
        if (fs.existsSync(cand) && fs.statSync(cand).isDirectory()) cand = path.join(cand, 'index.html');
        if (fs.existsSync(cand) && fs.statSync(cand).isFile()) file = cand;
      }
    }
    if (!file) { status = 404; file = path.join(root, '404.html'); }
    const body = fs.readFileSync(file);
    res.writeHead(status, { 'content-type': TYPES[path.extname(file)] || 'application/octet-stream', 'content-length': body.length, 'cache-control': 'no-store' });
    res.end(req.method === 'HEAD' ? undefined : body);
  });
  return new Promise(r => server.listen(0, '127.0.0.1', () => r({ server, port: server.address().port })));
}
function bannerFacts(html) {
  const aside = [...html.matchAll(/<aside class="md-banner"[\s\S]*?<\/aside>/g)].map(m => m[0]);
  const inner = aside.map(a => a.replace(/<[^>]+>/g, ' ').replace(/\s+/g, ' ').trim());
  const hrefs = aside.map(a => (a.match(/<a [^>]*href="([^"]*)"/) || [])[1] || null);
  return { count: aside.length, texts: inner, hrefs };
}
async function verify(dir, matrix) {
  const { server, port } = await startServer(dir, matrix.prefix);
  const out = [], fail = [];
  try {
    for (const r of matrix.routes) {
      const url = `http://127.0.0.1:${port}${matrix.prefix}${r.path}`;
      const resp = await fetch(url); const html = await resp.text(); const b = bannerFacts(html);
      const wantStatus = r.expectStatus || 200;
      const row = { route: r.id, status: resp.status, banners: b.count, text: b.texts[0] || null, href: b.hrefs[0] || null, resolved: b.hrefs[0] ? new URL(b.hrefs[0], url).pathname : null };
      out.push(row);
      if (resp.status !== wantStatus) fail.push(`${r.id}: status ${resp.status} != ${wantStatus}`);
      if (r.noticeExpected) {
        if (b.count !== 1) fail.push(`${r.id}: expected 1 banner, found ${b.count}`);
        else if (b.texts[0] !== matrix.noticeText) fail.push(`${r.id}: banner text differs: ${JSON.stringify(b.texts[0])}`);
        else if (new URL(b.hrefs[0], url).pathname !== matrix.dossierHref) fail.push(`${r.id}: banner href ${b.hrefs[0]} resolves to ${new URL(b.hrefs[0], url).pathname}, not ${matrix.dossierHref}`);
      } else if (b.count !== 0 || html.includes('md-banner')) fail.push(`${r.id}: banner element present where none is allowed`);
    }
    const link = await fetch(`http://127.0.0.1:${port}${matrix.dossierHref}`);
    if (link.status !== 200) fail.push(`dossier link destination status ${link.status}`);
    const escape = await fetch(`http://127.0.0.1:${port}${matrix.prefix}..%2f..%2fetc%2fpasswd`);
    if (escape.status !== 404) fail.push(`traversal guard returned ${escape.status}`);
  } finally { server.close(); }
  return { ok: fail.length === 0, routes: out, fail };
}
module.exports = { startServer, bannerFacts, verify };
if (require.main === module) {
  const a = process.argv.slice(2), get = k => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : null; };
  const dir = get('--dir'), mp = get('--matrix');
  if (!dir || !mp || !a.includes('--verify')) { console.error('usage: --dir D --matrix M --verify'); process.exit(2); }
  verify(dir, JSON.parse(fs.readFileSync(mp, 'utf8'))).then(r => { console.log(JSON.stringify(r, null, 1)); process.exit(r.ok ? 0 : 1); }, e => { console.error(String(e)); process.exit(2); });
}
~~~

### s1-qa-observe.cjs

~~~ s1-qa-observe.cjs
'use strict';
// One observation = one fresh browser, one page, one mode. Never retries. Exits non-zero on any error.
// usage: node s1-qa-observe.cjs --matrix M --route ID --mode ID --base http://127.0.0.1:PORT --out DIR
// env: PLAYWRIGHT_MODULE (path or name of the operator's installed playwright, default "playwright"),
//      CHROMIUM_PATH (optional executablePath), CHROMIUM_ARGS (space-separated extra flags).
const fs = require('node:fs'), path = require('node:path');
const { checkObservation } = require('./s1-qa-checks.cjs');
const a = process.argv.slice(2), get = k => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : null; };
const matrix = JSON.parse(fs.readFileSync(get('--matrix'), 'utf8'));
const route = matrix.routes.find(r => r.id === get('--route')), mode = matrix.modes.find(m => m.id === get('--mode'));
const base = get('--base'), out = get('--out');
if (!route || !mode || !base || !out) { console.error('bad arguments'); process.exit(2); }
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
// Runs inside the page. Uses only DOM APIs. Returns plain data.
function measure(args) {
  const { route, tabStops, dossierHref } = args;
  const rect = e => { const r = e.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height }; };
  const sig = e => { const s = getComputedStyle(e); return { fontFamily: s.fontFamily, fontSize: s.fontSize, fontWeight: s.fontWeight, lineHeight: s.lineHeight, color: s.color, backgroundColor: s.backgroundColor, position: s.position }; };
  const banners = [...document.querySelectorAll('.md-banner')];
  const b = banners[0];
  const link = b ? b.querySelector('a') : null;
  const banner = { count: banners.length, anyLinkToDossierInBanner: banners.some(x => x.querySelector('a[href*="ai-evaluation-dossier"]')) };
  if (b) {
    banner.text = b.textContent.replace(/\s+/g, ' ').trim(); banner.empty = banner.text === '';
    banner.rect = rect(b); banner.style = sig(b);
    if (link) { banner.linkPath = new URL(link.getAttribute('href'), location.href).pathname; banner.linkRect = rect(link); banner.linkStyle = sig(link); banner.linkVisible = link.getClientRects().length > 0 && getComputedStyle(link).visibility === 'visible'; }
  }
  const header = document.querySelector('.md-header');
  const h = document.querySelector(route.firstHeading);
  const ctas = {};
  for (const sel of route.cta) ctas[sel] = [...document.querySelectorAll(sel)].map(e => ({ text: (e.textContent || '').replace(/\s+/g, ' ').trim().slice(0, 80), href: e.getAttribute('href'), rect: rect(e), inFirstScreen: e.getBoundingClientRect().bottom <= innerHeight && e.getBoundingClientRect().top >= 0 }));
  return {
    banner, header: header ? { rect: rect(header), style: sig(header) } : null,
    firstHeading: h ? { found: true, text: h.textContent.trim().slice(0, 80), rect: rect(h) } : { found: false },
    ctas,
    document: { scrollWidth: document.documentElement.scrollWidth, clientWidth: document.documentElement.clientWidth, scrollHeight: document.documentElement.scrollHeight },
    viewport: { width: innerWidth, height: innerHeight },
    fonts: { status: document.fonts.status, faces: [...document.fonts].map(f => ({ family: f.family, weight: f.weight, style: f.style, status: f.status })) },
    colorScheme: document.body.getAttribute('data-md-color-scheme'),
    initialScrollY: window.scrollY,
  };
}
(async () => {
  const t0 = Date.now();
  const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH || undefined, args: (process.env.CHROMIUM_ARGS || '').split(/\s+/).filter(Boolean) });
  try {
    const ctx = await browser.newContext({ viewport: { width: mode.width, height: mode.height }, colorScheme: mode.colorScheme, reducedMotion: 'reduce', serviceWorkers: 'block' });
    const page = await ctx.newPage();
    const errors = []; page.on('pageerror', e => errors.push(String(e).slice(0, 200)));
    const url = base + matrix.prefix + route.path;
    const resp = await page.goto(url, { waitUntil: 'load', timeout: 30000 });
    await page.evaluate(() => document.fonts.ready);
    const data = await page.evaluate(measure, { route, tabStops: matrix.tabStops, dossierHref: matrix.dossierHref });
    fs.mkdirSync(out, { recursive: true });
    let imageName = null;
    if (route.images.includes(mode.id) && data.initialScrollY === 0) { imageName = `${route.id}-${mode.id}.jpg`; await page.screenshot({ path: path.join(out, imageName), type: 'jpeg', quality: 80, fullPage: false }); }
    const focusStops = [];
    await page.evaluate(() => { document.activeElement && document.activeElement.blur(); });
    for (let i = 0; i < matrix.tabStops; i++) {
      await page.keyboard.press('Tab');
      focusStops.push(await page.evaluate(() => { const e = document.activeElement; if (!e || e === document.body) return { tag: null }; const r = e.getBoundingClientRect(); const href = e.getAttribute('href'); return { tag: e.tagName, text: (e.textContent || e.getAttribute('aria-label') || '').replace(/\s+/g, ' ').trim().slice(0, 80), href, hrefPath: href ? new URL(href, location.href).pathname : null, rect: { x: r.x, y: r.y, width: r.width, height: r.height }, visible: r.width > 0 && r.height > 0 && r.bottom > 0 && r.top < innerHeight }; }));
    }
    const scrollYAfterTab = await page.evaluate(() => window.scrollY);
    const obs = { route: route.id, mode: mode.id, url, status: resp.status(), ...data, focusStops, scrollYAfterTab, image: imageName, pageErrors: errors, elapsedMs: Date.now() - t0 };
    obs.failures = checkObservation(obs, route, matrix, mode);
    fs.writeFileSync(path.join(out, `${route.id}-${mode.id}.json`), JSON.stringify(obs, null, 1));
    console.log(JSON.stringify({ route: route.id, mode: mode.id, failures: obs.failures, image: obs.image }));
    process.exitCode = obs.failures.length ? 1 : 0;
  } finally { await browser.close(); }
})().catch(e => { console.error(String(e)); process.exit(2); });
~~~

### s1-qa-selftest.cjs

~~~ s1-qa-selftest.cjs
'use strict';
// Offline self-test: the checks must pass a good observation and fail each known-bad one. No browser.
const { checkObservation } = require('./s1-qa-checks.cjs');
const m = require('./s1-qa-matrix.json');
const route = m.routes.find(r => r.id === 'decide'), dossier = m.routes.find(r => r.id === 'dossier'), home = m.routes.find(r => r.id === 'home');
const mLight = m.modes.find(x => x.id === 'mobile-light'), mDark = m.modes.find(x => x.id === 'mobile-dark');
const good = () => ({
  status: 200, viewport: { width: 390, height: 844 }, document: { scrollWidth: 390, clientWidth: 390 },
  header: { rect: { x: 0, y: 60, width: 390, height: 48 } },
  banner: { count: 1, empty: false, text: m.noticeText, linkPath: m.dossierHref, linkVisible: true, rect: { x: 0, y: 0, width: 390, height: 56 } },
  focusStops: [{ tag: 'A', href: '#x', hrefPath: '/x', visible: true }, { tag: 'A', href: 'a', hrefPath: m.dossierHref, visible: true }],
  firstHeading: { found: true, rect: { x: 16, y: 200, width: 300, height: 40 } },
  colorScheme: 'default', fonts: { status: 'loaded' }, pageErrors: [], initialScrollY: 0, ctas: {}, image: 'decide-mobile-light.jpg',
});
const goodDossier = () => ({ ...good(), banner: { count: 0, anyLinkToDossierInBanner: false }, focusStops: [], ctas: { '.agf-hero__action--primary': [{}], '.agf-hero__action--secondary': [{}] }, image: null });
const goodHome = () => ({ ...good(), image: 'home-mobile-light.jpg', ctas: { '.agf-hero__action--primary': [{}], '.agf-hero__action--secondary': [{}], '.agf-route-card a': [{}] } });
const cases = [
  ['good decide passes', good(), route, 0],
  ['good dossier passes', goodDossier(), dossier, 0],
  ['missing notice on a required route', { ...good(), banner: { count: 0 } }, route, 1],
  ['empty banner', (() => { const o = good(); o.banner.empty = true; o.banner.text = ''; return o; })(), route, 1],
  ['wrong link destination', (() => { const o = good(); o.banner.linkPath = '/agents-govern-framework/'; return o; })(), route, 1],
  ['link not within 10 Tab stops', (() => { const o = good(); o.focusStops = Array.from({ length: 10 }, () => ({ href: 'z', hrefPath: '/z', visible: true })); return o; })(), route, 1],
  ['link reached but hidden', (() => { const o = good(); o.focusStops[1].visible = false; return o; })(), route, 1],
  ['banner overlaps header', (() => { const o = good(); o.banner.rect.height = 80; return o; })(), route, 1],
  ['horizontal overflow', (() => { const o = good(); o.document.scrollWidth = 420; return o; })(), route, 1],
  ['first heading below first screen', (() => { const o = good(); o.firstHeading.rect.y = 840; return o; })(), route, 1],
  ['notice present on the dossier', { ...goodDossier(), banner: { count: 1, anyLinkToDossierInBanner: true } }, dossier, 1],
  ['empty banner on the dossier', { ...goodDossier(), banner: { count: 1, anyLinkToDossierInBanner: false } }, dossier, 1],
  ['wrong HTTP status', { ...good(), status: 404 }, route, 1],
  ['dossier whose own hero actions are missing', { ...goodDossier(), ctas: {} }, dossier, 1],
  ['dark palette matches a dark request', { ...good(), colorScheme: 'slate', image: null }, { ...route, images: [] }, 0, mDark],
  ['good home passes (all declared CTAs matched, image captured)', goodHome(), home, 0],
  ['palette is dark when light was requested', { ...good(), colorScheme: 'slate' }, route, 1],
  ['font set still loading', { ...good(), fonts: { status: 'loading' } }, route, 1],
  ['declared CTA selector matched nothing', (() => { const o = goodHome(); o.ctas['.agf-hero__action--primary'] = []; return o; })(), home, 1],
  ['declared CTA selector missing from the observation', (() => { const o = goodHome(); delete o.ctas['.agf-route-card a']; return o; })(), home, 1],
  ['uncaught page error', { ...good(), pageErrors: ['TypeError: x'] }, route, 1],
  ['page already scrolled before the image', { ...good(), initialScrollY: 120 }, route, 1],
  ['planned image missing', { ...good(), image: null }, route, 1],
];
let bad = 0;
for (const [name, obs, r, want, md] of cases) {
  const f = checkObservation(obs, r, m, md || mLight), ok = want === 0 ? f.length === 0 : f.length > 0;
  if (!ok) bad++;
  console.log(`${ok ? 'PASS' : 'FAIL'} ${name}${f.length ? '  -> ' + f.join('; ') : ''}`);
}
process.exit(bad ? 1 : 0);
~~~

### s1-qa-run.sh

~~~ s1-qa-run.sh
#!/usr/bin/env bash
# Operator launcher for the S1 rendered QA. Runs the 24 observations serially, one fresh browser each, no retry.
# Requires root's separate run decision. Missing terminal evidence is never a clean exit.
# usage: s1-qa-run.sh ARTIFACT_DIR OUT_DIR   (ARTIFACT_DIR is the extracted site-html directory; OUT_DIR is empty or absent)
# env, operator-local (none assumed): PLAYWRIGHT_MODULE, CHROMIUM_PATH, CHROMIUM_ARGS, NODE_BIN (default node)
# env, required: S1_ZIP_SHA256 and S1_HEAD_SHA (must equal the matrix), recorded in binding.txt.
# env, test seams only: SYSTEMD_RUN, SYSTEMCTL, CGROOT, S1_QA_POLL_SEC.
# Run it as: bash s1-qa-run.sh ARTIFACT_DIR OUT_DIR. Markdown fences do not keep file modes, so do not rely on the executable bit.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ART="${1:?artifact dir}"; OUT="${2:?output dir}"; case "$OUT" in /*) ;; *) OUT="$PWD/$OUT" ;; esac; NODE_BIN="${NODE_BIN:-node}"
SYSTEMD_RUN="${SYSTEMD_RUN:-systemd-run}"; SYSTEMCTL="${SYSTEMCTL:-systemctl}"; CGROOT="${CGROOT:-/sys/fs/cgroup}"
M="$HERE/s1-qa-matrix.json"
stop() { echo "STOP: $1" >&2; exit "${2:-1}"; }
mx() { "$NODE_BIN" -e "const m=require('$M');console.log($1)"; }
# 1. Package integrity: the frozen hashes must match the files being run.
( cd "$HERE" && sha256sum -c s1-qa-hashes.sha256 >/dev/null 2>&1 ) || stop "package files differ from s1-qa-hashes.sha256" 2
# 2. Binding of the inputs the operator claims.
[ "${S1_ZIP_SHA256:-}" = "$(mx 'm.zipSha256')" ] || stop "S1_ZIP_SHA256 does not equal the matrix ZIP hash" 2
[ "${S1_HEAD_SHA:-}" = "$(mx 'm.head')" ] || stop "S1_HEAD_SHA does not equal the matrix head" 2
[ -d "$ART" ] || stop "no artifact dir" 2
[ ! -e "$OUT" ] || [ -z "$(ls -A "$OUT" 2>/dev/null)" ] || stop "output dir must be empty" 2
mkdir -p "$OUT/obs" "$OUT/units"
TREE="$(cd "$ART" && find . -type f | LC_ALL=C sort | xargs sha256sum | sha256sum | cut -c1-64)"
{ echo "zip_sha256=$S1_ZIP_SHA256"; echo "head=$S1_HEAD_SHA"; echo "artifact_tree_sha256=$TREE"; echo "package_hashes:"; cat "$HERE/s1-qa-hashes.sha256"; } > "$OUT/binding.txt"
# 3. No foreign or leftover unit with our name prefix may exist before we start.
FOREIGN="$($SYSTEMCTL --user list-units --all --no-legend 'agf-s1qa-*' 2>/dev/null | awk 'NF{print $1}')"
[ -z "$FOREIGN" ] || stop "existing agf-s1qa unit(s): $FOREIGN" 6
# 4. Static contract and offline self-test before any browser starts.
"$NODE_BIN" "$HERE/s1-qa-serve.cjs" --dir "$ART" --matrix "$M" --verify > "$OUT/static-verify.json" || stop "static contract failed" 2
"$NODE_BIN" "$HERE/s1-qa-selftest.cjs" > "$OUT/selftest.txt" || stop "self-test failed" 2
# 5. Local server.
"$NODE_BIN" -e "const {startServer}=require('$HERE/s1-qa-serve.cjs');const m=require('$M');startServer('$ART',m.prefix).then(({port})=>require('fs').writeFileSync('$OUT/port',String(port)))" > "$OUT/server.log" 2>&1 &
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
for ROUTE in $(mx "m.routes.map(r=>r.id).join(' ')"); do
  for MODE in $(mx "m.modes.map(r=>r.id).join(' ')"); do
    N=$((N+1)); NN="$(printf %02d "$N")"; UNIT="agf-s1qa-$RUNID-$NN"; SVC="$UNIT.service"; U="$OUT/units/$UNIT"
    A="$(awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo)"
    [ "$A" -ge "$FLOOR" ] || stop "host MemAvailable ${A} MiB below floor ${FLOOR} MiB before observation $N" 3
    # One observation in one named transient unit, started without waiting so its cgroup can be read while it runs.
    # RemainAfterExit=yes keeps a successfully finished unit loaded (active, exited) until we stop it, so its identity and terminal
    # properties stay readable. A failed unit stays loaded by default. Output goes to files owned by this unit.
    "$SYSTEMD_RUN" --user --unit="$UNIT" --no-block --quiet \
      -p MemoryMax="$MEM_MAX" -p MemorySwapMax="$SWAP_MAX" -p OOMPolicy=kill -p RuntimeMaxSec="$RUN_SEC" -p RemainAfterExit=yes \
      -p StandardOutput="file:$U.stdout" -p StandardError="file:$U.stderr" \
      --setenv=PLAYWRIGHT_MODULE="${PLAYWRIGHT_MODULE:-playwright}" --setenv=CHROMIUM_PATH="${CHROMIUM_PATH:-}" --setenv=CHROMIUM_ARGS="${CHROMIUM_ARGS:-}" \
      "$NODE_BIN" --max-old-space-size="$OLD" "$HERE/s1-qa-observe.cjs" --matrix "$M" --route "$ROUTE" --mode "$MODE" --base "http://127.0.0.1:$PORT" --out "$OUT/obs" \
      > "$U.start" 2>&1 || stop "systemd-run could not start $UNIT: $(cat "$U.start")" 7
    # Poll this unit only. Remember the first non-empty ControlGroup seen while it ran.
    CG_RUN=""; DEADLINE=$(( $(date +%s) + ${S1_QA_POLL_SEC:-$((RUN_SEC + 20))} )); FINISHED=0
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
WJ="$(mx "m.routes.length*m.modes.length")"; WI="$(mx "m.routes.reduce((a,r)=>a+r.images.length,0)")"
[ "$NJ" = "$WJ" ] && [ "$NI" = "$WI" ] || stop "inventory mismatch: $NJ json (want $WJ), $NI images (want $WI)" 8
( cd "$OUT" && find obs units -type f | LC_ALL=C sort | xargs sha256sum > inventory.sha256 )
echo "completed $N observations; $NJ json, $NI images; inventory.sha256 written" | tee -a "$OUT/observations.log"
~~~

### s1-qa-run-selftest.sh

~~~ s1-qa-run-selftest.sh
#!/usr/bin/env bash
# Offline test of the launcher's control flow with fake systemd-run and systemctl. No browser, no unit, no root.
# usage: bash s1-qa-run-selftest.sh ARTIFACT_DIR     Exit 0 only if every scenario ends with the expected launcher exit code.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; ART="${1:?artifact dir}"
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
R="$(get --route)"; M="$(get --mode)"; O="$(get --out)"
RES=success; ST=0; ID="$UNIT.service"; CG="/user.slice/$UNIT.service"; ACT=active; SUB=exited; LOAD=loaded; INVID="abc$N"
[ "$FAKE_SCENARIO" = "fail5" ] && [ "$N" = 5 ] && { RES=exit-code; ST=1; ACT=failed; SUB=failed; }
case "$FAKE_SCENARIO" in
  wrongid) ID="agf-s1qa-someone-else.service";;
  emptycg) CG="";;
  foreignsuffix) CG="/user.slice/$UNIT.service.evil";;
  foreignprefix) CG="/user.slice/x-$UNIT.service";;
  traversal) CG="/user.slice/../$UNIT.service";;
  relative) CG="user.slice/$UNIT.service";;
  gone) LOAD=not-found; ACT=inactive; SUB=dead; INVID=""; CG=""; ID="$UNIT.service";;   # the real qpr7 failure: unloaded unit, default values
  running) ACT=active; SUB=running;;
esac
mkdir -p "$O"; echo '{}' > "$O/$R-$M.json"
if node -e "const m=require('$FAKE_MATRIX');process.exit(m.routes.find(r=>r.id==='$R').images.includes('$M')?0:1)"; then : > "$O/$R-$M.jpg"; fi
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
  list-units) [ "$FAKE_SCENARIO" = "foreign" ] && echo "agf-s1qa-other.service loaded active running x"; exit 0;;
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
mkdir -p "$T/pkg" && cp "$HERE"/s1-qa-* "$T/pkg/"
ZIP="$(node -e "console.log(require('$T/pkg/s1-qa-matrix.json').zipSha256)")"; HEAD="$(node -e "console.log(require('$T/pkg/s1-qa-matrix.json').head)")"
bad=0
run() { # name expected_rc scenario [env assignments...]
  local name="$1" want="$2" sc="$3"; shift 3
  rm -rf "$T/out" "$T/state"/*; mkdir -p "$T/state"
  env "$@" FAKE_SCENARIO="$sc" FAKE_STATE="$T/state" FAKE_MATRIX="$T/pkg/s1-qa-matrix.json" FAKE_CGROOT="$T/cg" CGROOT="$T/cg" \
      S1_QA_POLL_SEC=2 PATH="$T/bin:$PATH" SYSTEMD_RUN="$T/bin/systemd-run" SYSTEMCTL="$T/bin/systemctl" \
      bash "$T/pkg/s1-qa-run.sh" "$ART" "$T/out" > "$T/run.log" 2>&1
  local rc=$?; local ok=FAIL; [ "$rc" = "$want" ] && ok=PASS; [ "$ok" = FAIL ] && bad=$((bad+1)); [ "$ok" = FAIL ] && tail -2 "$T/run.log" | sed "s/^/     log: /"
  echo "$ok $name (exit $rc, want $want)"
}
GOOD=(S1_ZIP_SHA256="$ZIP" S1_HEAD_SHA="$HEAD")
run "all 24 observations succeed and the inventory matches (cgroup removed after exit)" 0 ok "${GOOD[@]}"
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
run "a pre-existing agf-s1qa unit stops the run before any observation" 6 foreign "${GOOD[@]}"
run "a wrong ZIP hash is rejected" 2 ok S1_ZIP_SHA256=deadbeef S1_HEAD_SHA="$HEAD"
run "a wrong head is rejected" 2 ok S1_ZIP_SHA256="$ZIP" S1_HEAD_SHA=deadbeef
echo '// tampered' >> "$T/pkg/s1-qa-checks.cjs"
run "a package file that differs from the frozen hashes is rejected" 2 ok "${GOOD[@]}"
exit $((bad ? 1 : 0))
~~~

### s1-qa-calibrate.sh

~~~ s1-qa-calibrate.sh
#!/usr/bin/env bash
# Browserless real-unit calibration of the launcher's systemd assumptions. NOT authorized by itself: it needs root's separate decision.
# It starts five tiny named transient units (Node one-liners, no browser, no server, no network) with the launcher's unit profile,
# except MemoryMax=64M and RuntimeMaxSec=4, and compares what systemd reports with expectations written here BEFORE any run.
# A mismatch is a finding, not a reason to retry or to loosen the launcher. usage: bash s1-qa-calibrate.sh OUT_DIR   (empty or absent)
set -uo pipefail
OUT="${1:?output dir}"; case "$OUT" in /*) ;; *) OUT="$PWD/$OUT" ;; esac
[ ! -e "$OUT" ] || [ -z "$(ls -A "$OUT" 2>/dev/null)" ] || { echo "output dir must be empty" >&2; exit 2; }
mkdir -p "$OUT"; NODE_BIN="${NODE_BIN:-node}"; SYSTEMD_RUN="${SYSTEMD_RUN:-systemd-run}"; SYSTEMCTL="${SYSTEMCTL:-systemctl}"; CGROOT="${CGROOT:-/sys/fs/cgroup}"
RUNID="$(date -u +%Y%m%dT%H%M%S)-$$"; BAD=0
[ -z "$($SYSTEMCTL --user list-units --all --no-legend 'agf-s1qa-*' 2>/dev/null | awk 'NF{print $1}')" ] || { echo "existing agf-s1qa unit" >&2; exit 6; }
get() { sed -n "s/^$2=//p" "$1" | head -1; }
valid_cg() { case "$1" in /*) ;; *) return 1 ;; esac; case "$1/" in */../*|*//*) return 1 ;; esac; [ "${1##*/}" = "$2" ]; }
inv() { if [ -e "$CGROOT$1/cgroup.procs" ]; then cat "$CGROOT$1/cgroup.procs"; else echo absent; fi; }
# case NAME WANT_RESULT WANT_EXEC_STATUS(or -) WANT_ACTIVE/SUB WANT_PROCS_BEFORE(empty|nonempty) -- node args...
run_case() {
  local name="$1" wres="$2" wst="$3" wstate="$4" wprocs="$5"; shift 6
  local UNIT="agf-s1qa-cal-$RUNID-$name" F=""; local SVC="$UNIT.service" P="$OUT/$name"; local why=""
  "$SYSTEMD_RUN" --user --unit="$UNIT" --no-block --quiet -p MemoryMax=64M -p MemorySwapMax=0 -p OOMPolicy=kill -p RuntimeMaxSec=4 -p RemainAfterExit=yes \
    -p StandardOutput="file:$P.stdout" -p StandardError="file:$P.stderr" "$@" > "$P.start" 2>&1 || { echo "FAIL $name: could not start: $(cat "$P.start")"; BAD=$((BAD+1)); return; }
  local CGRUN="" end=$(( $(date +%s) + 15 )) fin=0
  while [ "$(date +%s)" -le "$end" ]; do
    "$SYSTEMCTL" --user show "$SVC" -p ActiveState,SubState,InvocationID,ControlGroup > "$P.poll" 2>/dev/null
    local a s i c; a="$(get "$P.poll" ActiveState)"; s="$(get "$P.poll" SubState)"; i="$(get "$P.poll" InvocationID)"; c="$(get "$P.poll" ControlGroup)"
    [ -n "$c" ] && [ -z "$CGRUN" ] && CGRUN="$c"
    if [ -n "$i" ] && { [ "$a" = inactive ] || [ "$a" = failed ] || { [ "$a" = active ] && [ "$s" = exited ]; }; }; then fin=1; break; fi
    sleep 0.1
  done
  "$SYSTEMCTL" --user show "$SVC" -p Id,InvocationID,LoadState,Result,ActiveState,SubState,ExecMainStatus,ExecMainCode,MemoryPeak,ControlGroup > "$P.show" 2>/dev/null
  local CG; CG="$(get "$P.show" ControlGroup)"; CG="${CG:-$CGRUN}"
  { echo "cg_while_running=$CGRUN"; echo "cg_terminal=$(get "$P.show" ControlGroup)"; } > "$P.scope"
  [ "$fin" = 1 ] || why="$why not-finished;"
  [ "$(get "$P.show" Id)" = "$SVC" ] || why="$why id;"
  [ -n "$(get "$P.show" InvocationID)" ] || why="$why empty-invocation;"
  [ "$(get "$P.show" LoadState)" = loaded ] || why="$why not-loaded;"
  valid_cg "$CG" "$SVC" || why="$why cgroup-invalid('$CG');"
  [ "$(get "$P.show" Result)" = "$wres" ] || why="$why result=$(get "$P.show" Result) want $wres;"
  [ "$wst" = - ] || [ "$(get "$P.show" ExecMainStatus)" = "$wst" ] || why="$why status=$(get "$P.show" ExecMainStatus) want $wst;"
  [ "$(get "$P.show" ActiveState)/$(get "$P.show" SubState)" = "$wstate" ] || why="$why state=$(get "$P.show" ActiveState)/$(get "$P.show" SubState) want $wstate;"
  case "$(get "$P.show" MemoryPeak)" in ''|*[!0-9]*) why="$why memorypeak-unavailable;" ;; esac
  if valid_cg "$CG" "$SVC"; then inv "$CG" > "$P.procs-before"; else echo unavailable > "$P.procs-before"; fi
  local pb; pb="$(cat "$P.procs-before")"
  case "$wprocs" in empty) case "$pb" in ""|absent) ;; *) why="$why procs-before=$pb want none;" ;; esac;; nonempty) case "$pb" in ""|absent|unavailable) why="$why procs-before empty want some;" ;; esac;; esac
  "$SYSTEMCTL" --user stop "$SVC" > "$P.cleanup" 2>&1 || why="$why stop-failed;"
  "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$P.after" 2>/dev/null
  [ "$(get "$P.after" ActiveState)" != failed ] || { "$SYSTEMCTL" --user reset-failed "$SVC" >> "$P.cleanup" 2>&1 || why="$why reset-failed-failed;"; "$SYSTEMCTL" --user show "$SVC" -p LoadState,ActiveState > "$P.after" 2>/dev/null; }
  { [ "$(get "$P.after" LoadState)" = not-found ] || [ "$(get "$P.after" ActiveState)" = inactive ]; } || why="$why not-clean-after-stop;"
  if valid_cg "$CG" "$SVC"; then inv "$CG" > "$P.procs-after"; else echo unavailable > "$P.procs-after"; fi
  case "$(cat "$P.procs-after")" in ""|absent) ;; *) why="$why procs-after=$(cat "$P.procs-after");" ;; esac
  if [ -z "$why" ]; then echo "MATCH $name (as pre-registered)"; else echo "MISMATCH $name:$why"; BAD=$((BAD+1)); fi
}
# Pre-registered expectations. Result values are systemd's documented names; any other value is a finding.
run_case ok        success   0 active/exited empty    -- "$NODE_BIN" -e 'console.log(1)'
run_case exit3     exit-code 3 failed/failed empty    -- "$NODE_BIN" -e 'process.exit(3)'
run_case timeout   timeout   - failed/failed empty    -- "$NODE_BIN" -e 'setTimeout(()=>{},60000)'
run_case oom       oom-kill  - failed/failed empty    -- "$NODE_BIN" --max-old-space-size=512 -e 'const a=[];for(;;)a.push(Buffer.alloc(8e6,1))'
run_case leftover  success   0 active/exited nonempty -- bash -c 'sleep 30 & echo started'
echo "calibration: $BAD mismatch(es). Nothing was retried."; exit $((BAD ? 1 : 0))
~~~

### s1-qa-hashes.sha256

~~~ s1-qa-hashes.sha256
15b986b8a1076f95e7f8533c8f193104211f388256f41ab14b015ec3c7b05293  s1-qa-matrix.json
780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b  s1-qa-checks.cjs
40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06  s1-qa-serve.cjs
c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4  s1-qa-observe.cjs
ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67  s1-qa-selftest.cjs
fc81c7a762fc08d1b08213aa8d5b195f8017a7b862ad37a83a9fe09e1b85a4bd  s1-qa-run.sh
3c093e64dad15a19facf1e339e6b3f34f78f1d45fc0a95c6ef785e464e80aaaa  s1-qa-run-selftest.sh
40835980e3b465853c18144d74c296e595601469b805d469437482557d4c08e5  s1-qa-calibrate.sh
~~~

## Offline evidence (no browser, no unit)

`bash -n` on all three shell files and `node --check` on the scripts: pass. `sha256sum -c s1-qa-hashes.sha256`: pass. `s1-qa-selftest.cjs`: 23 of 23 (unchanged). `s1-qa-run-selftest.sh`: 19 of 19 scenarios with the expected launcher exit codes, including the two new ones. The calibration script has only had a syntax check; I never ran it, and its tests would need real units.

## Not tested, and limits

`RemainAfterExit` behaviour in a user manager, readability of the properties and the cgroup, the exact Result names for timeout and OOM, and the `--no-block` start path are all untested. The observer, the contrast of the corrected link in a browser, and all four modes are unobserved. No A3 or S2 credit or reset follows. This closes nothing for R1, R2 or Gate 2.

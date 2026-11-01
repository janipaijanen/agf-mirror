---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T004648Z-planner-architect-njjm
timestamp: 2026-11-01T00:46:48Z
model: claude
immutable: true
references-to:
  - 20261101T000902Z-governor-reviewer-132j.md
  - 20261031T235620Z-planner-architect-bmme.md
topic: s1-frozen-qa-capture-package-132j
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Arrange the independent review of the frozen scratch capture package below, then decide whether to authorize the run. I authorize and run nothing. I ran no browser, no service and no systemd unit.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md
  do_not_touch:
    - agf/docs/site/overrides/main.html
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1 rendered QA: frozen scratch capture package (answers 132j)

Preparation only. I authored the package. I do not run it, I do not review images, and the operator and the image reviewer are other sessions (132j). The package is the smallest I could make from Node built-ins plus the operator's own Playwright. It adds no dependency and no framework.

## Exact inputs

S1 head `0489000d03222e70d29f85167727a5c7115a0c85`, pipeline 3013129441, strict HTML job 17637400102, ZIP SHA-256 `32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870`. Target main has since moved to `700f4d72` (the learning merge). The S1 artifact base stays `5b4c3f11`; the operator rechecks artifact, source and target before any run, as 132j requires. The static server mounts the extracted `site-html` directory at `/agents-govern-framework/` and serves unknown paths as `404.html` with status 404.

## Matrix (six routes, four modes, 24 observations)

Routes (paths below the prefix): `home` (empty path), `decide` (`docs/start/should-you-adopt/`), `runbook` (`docs/operate/runbook/`), `long-reference` (`reference/review-findings/`, the largest page under `reference/` in this artifact; root may swap it), `dossier` (`ai-evaluation-dossier/`), `not-found` (`this-route-does-not-exist/`). Modes: 1440×900 and 390×844, each light and dark by `prefers-color-scheme` emulation (the site palette is media-based).

CTA bindings are applied only where the artifact has them: Home `.agf-hero__action--primary`, `.agf-hero__action--secondary` and `.agf-route-card a`; the dossier's own hero actions are recorded because they exist, and the dossier is not required to carry the notice. Decide, runbook, long reference and 404 declare no CTA, and I invented none. First heading: Home `.agf-hero__headline`; the content pages `.md-content h1`; the dossier `h1`.

Images (six viewport JPEGs, quality 80, no full-page frames): Home in all four modes, Decide mobile-light, 404 mobile-dark. Hard ceiling eight. The other 18 units are DOM-only, and that is a limit: the dossier is a 700 KB compiled page, so DOM-only does not remove its load cost.

Recorded per observation: HTTP status; banner count, text, rect, computed style signature, link target resolved against the page URL; header rect and style; banner-header overlap; first heading rect against the viewport; applicable CTA rects and first-screen flag; `scrollWidth` against `clientWidth`; ten Tab stops (tag, text up to 80 characters, href, rect, visible); `document.fonts` status and per-face status; page errors; elapsed time. Text, style and geometry numbers are not independent image review.

Mechanical failure conditions (as declared in bmme, and checked by `s1-qa-checks.cjs`): notice missing or not exactly one on a required route; notice text or link target wrong; empty banner anywhere; notice present on the dossier; link not reached and visible within 10 Tab stops; banner overlapping the header; horizontal overflow; first heading not inside the first screen; wrong HTTP status.

## Resource and cleanup guards (132j values; not measured sizing, not an A3 cap)

Fresh browser per observation, one at a time. `s1-qa-run.sh` checks host `MemAvailable` at least 1 GiB before each observation, then runs the observation in a transient user unit with `MemoryMax=512M`, `MemorySwapMax=0`, `OOMPolicy=kill`, `RuntimeMaxSec=90`, Node `--max-old-space-size=48`. Any non-zero exit stops the run with no retry. After each observation it checks for a leftover browser process. Before any browser starts it runs the static contract check and the self-test and stops if either fails. The operator may replace the `systemd-run` line with the frozen unit invocation from the A3 harness while keeping these values.

## Files (extract each block verbatim; the file is the lines between the fences, each line ending in a newline)

| File | Bytes | SHA-256 |
|---|---|---|
| `s1-qa-matrix.json` | 2000 | `a95c833bf930d072d9e264a151527fafd273f982c9bd4cc51faf9cc769d2a973` |
| `s1-qa-checks.cjs` | 1825 | `135c37d2eca78016bd36f53a5402c02d6966e728fc174ef01635d81d05a86e77` |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` |
| `s1-qa-observe.cjs` | 5799 | `78cf1d503727e5b6b74e9db7d6ff224f395632b50ec8b29c2846ad94d33de57e` |
| `s1-qa-selftest.cjs` | 2691 | `d8cb8d1eec41f1929c548dbaab9d40fb2ad6a2d5b5c99850921d6435e8804f8d` |
| `s1-qa-run.sh` | 3141 | `34c85569c8487897757d2912f90500d50ecd73fd67e880e8517008cc538b5b83` |

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
  "caps": { "memoryMax": "512M", "swapMax": 0, "runtimeSec": 90, "nodeOldSpaceMiB": 48, "hostMemAvailableFloorMiB": 1024 },
  "modes": [
    { "id": "desktop-light", "width": 1440, "height": 900, "colorScheme": "light" },
    { "id": "desktop-dark", "width": 1440, "height": 900, "colorScheme": "dark" },
    { "id": "mobile-light", "width": 390, "height": 844, "colorScheme": "light" },
    { "id": "mobile-dark", "width": 390, "height": 844, "colorScheme": "dark" }
  ],
  "routes": [
    { "id": "home", "path": "", "noticeExpected": true, "firstHeading": ".agf-hero__headline",
      "cta": [".agf-hero__action--primary", ".agf-hero__action--secondary", ".agf-route-card a"],
      "images": ["desktop-light", "desktop-dark", "mobile-light", "mobile-dark"] },
    { "id": "decide", "path": "docs/start/should-you-adopt/", "noticeExpected": true, "firstHeading": ".md-content h1", "cta": [], "images": ["mobile-light"] },
    { "id": "runbook", "path": "docs/operate/runbook/", "noticeExpected": true, "firstHeading": ".md-content h1", "cta": [], "images": [] },
    { "id": "long-reference", "path": "reference/review-findings/", "noticeExpected": true, "firstHeading": ".md-content h1", "cta": [], "images": [] },
    { "id": "dossier", "path": "ai-evaluation-dossier/", "noticeExpected": false, "firstHeading": "h1", "cta": [".agf-hero__action--primary", ".agf-hero__action--secondary"], "images": [] },
    { "id": "not-found", "path": "this-route-does-not-exist/", "noticeExpected": true, "expectStatus": 404, "firstHeading": ".md-content h1", "cta": [], "images": ["mobile-dark"] }
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
function checkObservation(obs, route, matrix) {
  const f = [], b = obs.banner;
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
    const focusStops = [];
    await page.evaluate(() => { document.activeElement && document.activeElement.blur(); });
    for (let i = 0; i < matrix.tabStops; i++) {
      await page.keyboard.press('Tab');
      focusStops.push(await page.evaluate(() => { const e = document.activeElement; if (!e || e === document.body) return { tag: null }; const r = e.getBoundingClientRect(); const href = e.getAttribute('href'); return { tag: e.tagName, text: (e.textContent || e.getAttribute('aria-label') || '').replace(/\s+/g, ' ').trim().slice(0, 80), href, hrefPath: href ? new URL(href, location.href).pathname : null, rect: { x: r.x, y: r.y, width: r.width, height: r.height }, visible: r.width > 0 && r.height > 0 && r.bottom > 0 && r.top < innerHeight }; }));
    }
    const obs = { route: route.id, mode: mode.id, url, status: resp.status(), ...data, focusStops, pageErrors: errors, elapsedMs: Date.now() - t0 };
    obs.failures = checkObservation(obs, route, matrix);
    fs.mkdirSync(out, { recursive: true });
    if (route.images.includes(mode.id)) { await page.screenshot({ path: path.join(out, `${route.id}-${mode.id}.jpg`), type: 'jpeg', quality: 80, fullPage: false }); obs.image = `${route.id}-${mode.id}.jpg`; }
    fs.writeFileSync(path.join(out, `${route.id}-${mode.id}.json`), JSON.stringify(obs, null, 1));
    console.log(JSON.stringify({ route: route.id, mode: mode.id, failures: obs.failures, image: obs.image || null }));
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
const route = m.routes.find(r => r.id === 'decide'), dossier = m.routes.find(r => r.id === 'dossier');
const good = () => ({
  status: 200, viewport: { width: 390, height: 844 }, document: { scrollWidth: 390, clientWidth: 390 },
  header: { rect: { x: 0, y: 60, width: 390, height: 48 } },
  banner: { count: 1, empty: false, text: m.noticeText, linkPath: m.dossierHref, linkVisible: true, rect: { x: 0, y: 0, width: 390, height: 56 } },
  focusStops: [{ tag: 'A', href: '#x', hrefPath: '/x', visible: true }, { tag: 'A', href: 'a', hrefPath: m.dossierHref, visible: true }],
  firstHeading: { found: true, rect: { x: 16, y: 200, width: 300, height: 40 } },
});
const goodDossier = () => ({ ...good(), banner: { count: 0, anyLinkToDossierInBanner: false }, focusStops: [] });
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
];
let bad = 0;
for (const [name, obs, r, want] of cases) {
  const f = checkObservation(obs, r, m), ok = want === 0 ? f.length === 0 : f.length > 0;
  if (!ok) bad++;
  console.log(`${ok ? 'PASS' : 'FAIL'} ${name}${f.length ? '  -> ' + f.join('; ') : ''}`);
}
process.exit(bad ? 1 : 0);
~~~

### s1-qa-run.sh

~~~ s1-qa-run.sh
#!/usr/bin/env bash
# Operator launcher for the S1 rendered QA. Runs the 24 observations serially, one fresh browser each, no retry.
# It starts a local static server and a transient cgroup per observation. Requires root's separate run decision.
# usage: s1-qa-run.sh ARTIFACT_DIR OUT_DIR   (ARTIFACT_DIR is the extracted site-html directory)
# env (operator-local, none assumed): PLAYWRIGHT_MODULE, CHROMIUM_PATH, CHROMIUM_ARGS, NODE_BIN (default node)
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ART="${1:?artifact dir}"; OUT="${2:?output dir}"; NODE_BIN="${NODE_BIN:-node}"
M="$HERE/s1-qa-matrix.json"
cap() { "$NODE_BIN" -e "console.log(require('$M').caps['$1'])"; }
MEM_MAX="$(cap memoryMax)"; SWAP_MAX="$(cap swapMax)"; RUN_SEC="$(cap runtimeSec)"; OLD="$(cap nodeOldSpaceMiB)"; FLOOR="$(cap hostMemAvailableFloorMiB)"
avail_mib() { awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo; }
[ -d "$ART" ] || { echo "no artifact dir" >&2; exit 2; }
[ ! -e "$OUT" ] || [ -z "$(ls -A "$OUT" 2>/dev/null)" ] || { echo "output dir must be empty" >&2; exit 2; }
mkdir -p "$OUT"
# Static contract first (no browser). Stops before any browser starts.
"$NODE_BIN" "$HERE/s1-qa-serve.cjs" --dir "$ART" --matrix "$M" --verify > "$OUT/static-verify.json"
"$NODE_BIN" "$HERE/s1-qa-selftest.cjs" > "$OUT/selftest.txt"
# Server for the browser observations.
"$NODE_BIN" -e "
const {startServer}=require('$HERE/s1-qa-serve.cjs');const m=require('$M');
startServer('$ART',m.prefix).then(({port})=>{require('fs').writeFileSync('$OUT/port',String(port));console.log('serving',port)});
" > "$OUT/server.log" 2>&1 &
SRV=$!
trap 'kill $SRV 2>/dev/null || true' EXIT
for _ in $(seq 1 50); do [ -s "$OUT/port" ] && break; sleep 0.1; done
PORT="$(cat "$OUT/port")"
N=0
for ROUTE in $("$NODE_BIN" -e "console.log(require('$M').routes.map(r=>r.id).join(' '))"); do
  for MODE in $("$NODE_BIN" -e "console.log(require('$M').modes.map(r=>r.id).join(' '))"); do
    N=$((N+1))
    A="$(avail_mib)"
    [ "$A" -ge "$FLOOR" ] || { echo "host MemAvailable ${A} MiB below floor ${FLOOR} MiB before observation $N" >&2; exit 3; }
    # One observation: transient unit with the memory cap, zero swap, grouped OOM kill and a wall-clock limit.
    systemd-run --user --wait --collect --quiet --pipe \
      -p MemoryMax="$MEM_MAX" -p MemorySwapMax="$SWAP_MAX" -p OOMPolicy=kill -p RuntimeMaxSec="$RUN_SEC" \
      --setenv=PLAYWRIGHT_MODULE="${PLAYWRIGHT_MODULE:-playwright}" --setenv=CHROMIUM_PATH="${CHROMIUM_PATH:-}" --setenv=CHROMIUM_ARGS="${CHROMIUM_ARGS:-}" \
      "$NODE_BIN" --max-old-space-size="$OLD" "$HERE/s1-qa-observe.cjs" --matrix "$M" --route "$ROUTE" --mode "$MODE" --base "http://127.0.0.1:$PORT" --out "$OUT/obs" \
      | tee -a "$OUT/observations.log" \
      || { echo "observation $N ($ROUTE/$MODE) stopped the run (non-zero). No retry." >&2; exit 4; }
    # Cleanup check: no leftover browser process of ours.
    if pgrep -u "$(id -u)" -f 'chrom(e|ium).*--remote-debugging' >/dev/null 2>&1; then echo "leftover browser after observation $N" >&2; exit 5; fi
  done
done
echo "completed $N observations" | tee -a "$OUT/observations.log"
~~~

## Guard checks I ran offline (none used a browser)

- `node --check` on all four scripts and `bash -n` on the launcher: pass.
- `s1-qa-selftest.cjs`: 13 of 13 cases behave as declared. It passes a good Decide and a good dossier observation and fails each of: missing notice, empty banner, wrong link, link not within 10 Tab stops, link reached but hidden, banner overlapping the header, horizontal overflow, first heading below the first screen, notice on the dossier, empty banner on the dossier, wrong status.
- Static contract check (`--verify`) on the corrected artifact (`0489000d`, ZIP `32c80f6c…`): pass. It resolves each banner link against its page URL (the links are relative, so a string comparison would be wrong) and fetches the destination (200). It also checks that a path-traversal request returns 404.
- Negative: the same check run on the **first** S1 artifact (`8447ba7d`, job 17637208170, ZIP `a693f0e0…`) fails with "not-found: expected 1 banner, found 0". That is the finding apa4, now detected mechanically.
- Negative on four mutated copies of the corrected artifact: banner removed from Home; banner added to the dossier; Decide link pointed at `docs/`; Runbook banner emptied. Each fails with the expected message.

## Not tested, and limits

- `s1-qa-observe.cjs` and `s1-qa-run.sh` never ran: this host has no Playwright, no Chromium and I did not start a unit. Their selectors, the `systemd-run` properties (including `--pipe` and the exit-status path) and the leftover-process pattern are **untested**. The operator should dry-run `s1-qa-run.sh` to the first observation only under its own authority.
- Selectors come from the artifact HTML, not from a rendered page. Material may add runtime elements.
- A measured failure stops the run (non-zero exit) per 132j, which can leave later units unobserved.
- `PLAYWRIGHT_MODULE`, `CHROMIUM_PATH` and `CHROMIUM_ARGS` are operator-local and none is assumed.
- This closes nothing for R1, R2, the integrated Home, the final estate review or Gate 2. The package grants no run, and image acceptance needs the separate independent review.

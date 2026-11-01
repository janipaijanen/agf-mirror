---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T010947Z-planner-architect-3keh
timestamp: 2026-11-01T01:09:47Z
model: claude
immutable: true
references-to:
  - 20261101T005655Z-governor-reviewer-mngh.md
  - 20261101T004648Z-planner-architect-njjm.md
topic: s1-frozen-qa-package-corrected-mngh
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Narrow delta review of the corrected S1 frozen scratch package below against the five points of mngh, then a separate run decision. I authorize and run nothing. This record supersedes the file set embedded in njjm.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md
  do_not_touch:
    - agf/docs/site/overrides/main.html
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S1 frozen QA package, corrected after mngh

I changed the scratch package only: no site source, no resource increase. I ran no browser, no service and no systemd unit. The matrix is unchanged in shape: six routes, four modes, 24 observations, six planned viewport images (hard eight), serial fresh browsers, 512 MiB, zero swap, grouped OOM handling, 90 s, Node 48 MiB and the 1 GiB host floor. Operator and image reviewer stay separate from me and from each other. Inputs are as in njjm: head `0489000d03222e70d29f85167727a5c7115a0c85`, pipeline 3013129441, HTML job 17637400102, ZIP `32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870`. Target main is now `700f4d72`; the operator rechecks as 132j requires.

## The five points of mngh

1. **Unit identity, terminal evidence, cleanup.** Each observation now runs in a named transient unit `agf-s1qa-<runid>-<NN>`. I removed `--collect`, so the unit stays readable. After the wait, on success **and** on failure, the launcher records `systemctl show` for that exact unit (Id, InvocationID, Result, ActiveState, SubState, ExecMainStatus, ExecMainCode, MemoryPeak, ControlGroup) into `units/<unit>.show`, then stops it and clears any failed state. It stops the run (exit 7) if the evidence is missing or empty, if `Id` is not this unit, or if the ControlGroup is not this unit's. It stops (exit 5) if the unit's `cgroup.procs` is non-empty after exit, and (exit 4) on a non-zero exit, a Result other than `success` or a non-zero ExecMainStatus. The user-wide `pgrep` is gone. Before any observation it refuses to start (exit 6) if any `agf-s1qa-*` unit already exists. **I could not reuse the A3 lifecycle machinery**: it is private to the operator and I have not read it. The operator may replace the `systemd-run` block with that frozen invocation if it keeps this contract (named unit, retained terminal properties on both paths, scoped cgroup check, no broad process match). Missing terminal evidence is never a clean exit.
2. **Palette and font set enforced.** Each mode now declares the Material scheme it requests (`default` for light, `slate` for dark). The observation fails if `data-md-color-scheme` differs, or if `document.fonts.status` is not `loaded` after readiness. Unused faces are not required to load.
3. **Initial image before Tab.** The viewport image is captured right after the first measurement and before any key press, and only when `scrollY` is 0. The observation also records `scrollY` before and after Tab traversal and fails if `scrollY` was not 0 before the image or if a planned image is missing.
4. **Declared CTA selectors must match.** Every selector in a route's CTA list must match at least one element, or the observation fails. Routes with an empty list declare no CTA and I invented none. The dossier's own hero actions are declared because they exist.
5. **Uncaught page errors fail.** Any `pageerror` fails the observation.

Also added, from the mngh closing paragraph: the launcher checks every package file against `s1-qa-hashes.sha256` before it does anything else; it requires `S1_ZIP_SHA256` and `S1_HEAD_SHA` to equal the matrix and writes a `binding.txt` (ZIP, head, a hash of the extracted artifact tree, the package hashes); it checks that exactly 24 observation files and 6 images exist and writes a hashed `inventory.sha256` of results and unit evidence. The operator must still bind the extracted artifact to the source, job and ZIP independently and supply trusted local runtime paths.

## Files (extract each block verbatim; the file is the lines between the fences, each line ending in a newline)

| File | Bytes | SHA-256 |
|---|---|---|
| `s1-qa-matrix.json` | 2600 | `15b986b8a1076f95e7f8533c8f193104211f388256f41ab14b015ec3c7b05293` |
| `s1-qa-checks.cjs` | 2513 | `780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b` |
| `s1-qa-serve.cjs` | 4722 | `40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06` |
| `s1-qa-observe.cjs` | 5975 | `c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4` |
| `s1-qa-selftest.cjs` | 4304 | `ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67` |
| `s1-qa-run.sh` | 6193 | `de5cc6aadc1772832a14e13954069546280222a8c31eeb6d99027dbe5b2db32a` |
| `s1-qa-run-selftest.sh` | 4253 | `9b8cd2982ebd3e18f219d9f316644578a01f97504c1e27622a63a7aa2123bd98` |
| `s1-qa-hashes.sha256` | 585 | `0ce0605b28704cd4258d63e0ecd1698c55ba9192e5fe7a03f351ae9e9db1f2e8` |

`s1-qa-hashes.sha256` lists the other seven files (not itself). `s1-qa-run-selftest.sh` is a test harness, not part of a run.

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
# env, test seams only: SYSTEMD_RUN, SYSTEMCTL, CGROOT.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ART="${1:?artifact dir}"; OUT="${2:?output dir}"; NODE_BIN="${NODE_BIN:-node}"
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
for ROUTE in $(mx "m.routes.map(r=>r.id).join(' ')"); do
  for MODE in $(mx "m.modes.map(r=>r.id).join(' ')"); do
    N=$((N+1)); UNIT="agf-s1qa-$RUNID-$(printf %02d "$N")"; SHOW="$OUT/units/$UNIT.show"
    A="$(awk '/MemAvailable/{print int($2/1024)}' /proc/meminfo)"
    [ "$A" -ge "$FLOOR" ] || stop "host MemAvailable ${A} MiB below floor ${FLOOR} MiB before observation $N" 3
    # One observation in one named transient unit. No --collect: the unit stays readable until we stop it.
    "$SYSTEMD_RUN" --user --unit="$UNIT" --wait --pipe --quiet \
      -p MemoryMax="$MEM_MAX" -p MemorySwapMax="$SWAP_MAX" -p OOMPolicy=kill -p RuntimeMaxSec="$RUN_SEC" \
      --setenv=PLAYWRIGHT_MODULE="${PLAYWRIGHT_MODULE:-playwright}" --setenv=CHROMIUM_PATH="${CHROMIUM_PATH:-}" --setenv=CHROMIUM_ARGS="${CHROMIUM_ARGS:-}" \
      "$NODE_BIN" --max-old-space-size="$OLD" "$HERE/s1-qa-observe.cjs" --matrix "$M" --route "$ROUTE" --mode "$MODE" --base "http://127.0.0.1:$PORT" --out "$OUT/obs" \
      > "$OUT/units/$UNIT.stdout" 2> "$OUT/units/$UNIT.stderr"
    RC=$?
    # Terminal evidence for THIS unit, collected on success and on failure.
    "$SYSTEMCTL" --user show "$UNIT.service" -p Id,InvocationID,Result,ActiveState,SubState,ExecMainStatus,ExecMainCode,MemoryPeak,ControlGroup > "$SHOW" 2>/dev/null
    ID="$(get "$SHOW" Id)"; RES="$(get "$SHOW" Result)"; ST="$(get "$SHOW" ExecMainStatus)"; ACT="$(get "$SHOW" ActiveState)"; CG="$(get "$SHOW" ControlGroup)"; INV="$(get "$SHOW" InvocationID)"
    cat "$OUT/units/$UNIT.stdout" >> "$OUT/observations.log"
    "$SYSTEMCTL" --user stop "$UNIT.service" >/dev/null 2>&1; "$SYSTEMCTL" --user reset-failed "$UNIT.service" >/dev/null 2>&1
    [ "$ID" = "$UNIT.service" ] && [ -n "$INV" ] && [ -n "$RES" ] && [ -n "$ST" ] || stop "missing or foreign terminal evidence for $UNIT (Id='$ID' InvocationID='$INV' Result='$RES' ExecMainStatus='$ST'); observation $N ($ROUTE/$MODE)" 7
    case "$CG" in "") ;; *"$UNIT"*) ;; *) stop "ControlGroup '$CG' is not this unit's" 7 ;; esac
    if [ -n "$CG" ] && [ -s "$CGROOT$CG/cgroup.procs" ]; then stop "processes remain in $UNIT's cgroup after exit" 5; fi
    case "$ACT" in inactive|failed) ;; *) stop "unit $UNIT ActiveState '$ACT' after wait" 7 ;; esac
    if [ "$RC" -ne 0 ] || [ "$RES" != "success" ] || [ "$ST" != "0" ]; then stop "observation $N ($ROUTE/$MODE) failed: rc=$RC Result=$RES ExecMainStatus=$ST. No retry." 4; fi
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
# usage: s1-qa-run-selftest.sh ARTIFACT_DIR     Exit 0 only if every scenario ends with the expected launcher exit code.
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"; ART="${1:?artifact dir}"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
mkdir -p "$T/bin" "$T/state" "$T/cg"
cat > "$T/bin/systemd-run" <<'FAKE'
#!/usr/bin/env bash
UNIT=""; ARGS=("$@"); i=0; CMD=()
while [ $i -lt ${#ARGS[@]} ]; do a="${ARGS[$i]}"
  case "$a" in --unit=*) UNIT="${a#--unit=}";; -p) i=$((i+1));; --*) ;; *) CMD=("${ARGS[@]:$i}"); break;; esac
  i=$((i+1)); done
N=$(( $(cat "$FAKE_STATE/count" 2>/dev/null || echo 0) + 1 )); echo $N > "$FAKE_STATE/count"
get() { for ((k=0;k<${#CMD[@]};k++)); do [ "${CMD[$k]}" = "$1" ] && echo "${CMD[$((k+1))]}"; done; }
R="$(get --route)"; M="$(get --mode)"; O="$(get --out)"
RC=0; RES=success; ST=0; ID="$UNIT.service"; CG="/user.slice/$UNIT.service"
[ "$FAKE_SCENARIO" = "fail5" ] && [ "$N" = 5 ] && { RC=1; RES=exit-code; ST=1; }
[ "$FAKE_SCENARIO" = "wrongid" ] && ID="agf-s1qa-someone-else.service"
mkdir -p "$O"; echo '{}' > "$O/$R-$M.json"
if node -e "const m=require('$FAKE_MATRIX');process.exit(m.routes.find(r=>r.id==='$R').images.includes('$M')?0:1)"; then : > "$O/$R-$M.jpg"; fi
echo "{\"route\":\"$R\",\"mode\":\"$M\",\"failures\":[]}"
if [ "$FAKE_SCENARIO" != "noterminal" ]; then
  [ "$FAKE_SCENARIO" = "leftover" ] && { mkdir -p "$FAKE_CGROOT$CG"; echo 4242 > "$FAKE_CGROOT$CG/cgroup.procs"; }
  printf 'Id=%s\nInvocationID=abc%s\nResult=%s\nActiveState=inactive\nSubState=dead\nExecMainStatus=%s\nExecMainCode=1\nMemoryPeak=1000\nControlGroup=%s\n' "$ID" "$N" "$RES" "$ST" "$CG" > "$FAKE_STATE/$UNIT.show"
fi
exit $RC
FAKE
cat > "$T/bin/systemctl" <<'FAKE'
#!/usr/bin/env bash
case "$2" in
  list-units) [ "$FAKE_SCENARIO" = "foreign" ] && echo "agf-s1qa-other.service loaded active running x"; exit 0;;
  show) f="$FAKE_STATE/${3%.service}.show"; [ -f "$f" ] && cat "$f"; exit 0;;
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
      PATH="$T/bin:$PATH" SYSTEMD_RUN="$T/bin/systemd-run" SYSTEMCTL="$T/bin/systemctl" \
      "$T/pkg/s1-qa-run.sh" "$ART" "$T/out" > "$T/run.log" 2>&1
  local rc=$?; local ok=FAIL; [ "$rc" = "$want" ] && ok=PASS; [ "$ok" = FAIL ] && bad=$((bad+1))
  echo "$ok $name (exit $rc, want $want)"
}
GOOD=(S1_ZIP_SHA256="$ZIP" S1_HEAD_SHA="$HEAD")
run "all 24 observations succeed and the inventory matches" 0 ok "${GOOD[@]}"
if [ -f "$T/out/inventory.sha256" ]; then echo "     inventory: $(grep -c '\.json' "$T/out/inventory.sha256") json lines, $(grep -c '\.jpg' "$T/out/inventory.sha256") jpg lines, $(ls "$T/out/units"/*.show | wc -l) unit show files"; fi
run "a non-zero observation stops the run, with terminal evidence collected" 4 fail5 "${GOOD[@]}"
echo "     unit show files after the stop: $(ls "$T/out/units"/*.show 2>/dev/null | wc -l) (want 5)"; [ "$(ls "$T/out/units"/*.show 2>/dev/null | wc -l)" = 5 ] || bad=$((bad+1))
run "missing terminal evidence is not a clean exit" 7 noterminal "${GOOD[@]}"
run "terminal evidence for a different unit is rejected" 7 wrongid "${GOOD[@]}"
run "processes left in the unit's cgroup stop the run" 5 leftover "${GOOD[@]}"
run "a pre-existing agf-s1qa unit stops the run before any observation" 6 foreign "${GOOD[@]}"
run "a wrong ZIP hash is rejected" 2 ok S1_ZIP_SHA256=deadbeef S1_HEAD_SHA="$HEAD"
run "a wrong head is rejected" 2 ok S1_ZIP_SHA256="$ZIP" S1_HEAD_SHA=deadbeef
echo '// tampered' >> "$T/pkg/s1-qa-checks.cjs"
run "a package file that differs from the frozen hashes is rejected" 2 ok "${GOOD[@]}"
exit $((bad ? 1 : 0))
~~~

### s1-qa-hashes.sha256

~~~ s1-qa-hashes.sha256
15b986b8a1076f95e7f8533c8f193104211f388256f41ab14b015ec3c7b05293  s1-qa-matrix.json
780c04e5c8057e2ca17076ba360f5be85d0119f3f0dcaf2f5ad510d550822f6b  s1-qa-checks.cjs
40db4af6b3e5c634ba1d121ca7daad018172805fb23ddd529ecbca9c1ca6ec06  s1-qa-serve.cjs
c1e9da694145f62b2905358cc2827992b96b9b5794bab7b835ac8d08070a17a4  s1-qa-observe.cjs
ada3d5af6e392615dcd98f3e324778d890877fdc9a3967753068e1f339d2ec67  s1-qa-selftest.cjs
de5cc6aadc1772832a14e13954069546280222a8c31eeb6d99027dbe5b2db32a  s1-qa-run.sh
9b8cd2982ebd3e18f219d9f316644578a01f97504c1e27622a63a7aa2123bd98  s1-qa-run-selftest.sh
~~~

## Offline evidence (no browser, no unit)

- `node --check` on all four scripts and `bash -n` on both shell files: pass. `sha256sum -c s1-qa-hashes.sha256`: pass.
- `s1-qa-selftest.cjs`: 23 of 23 cases behave as declared. New negatives: palette not as requested, font set still loading, a declared CTA selector that matches nothing, a declared selector absent from the observation, the dossier's hero actions missing, an uncaught page error, a page already scrolled before the image, a planned image missing. A dark request with a `slate` page passes.
- `s1-qa-run-selftest.sh` runs the real launcher against a fake `systemd-run` and `systemctl` (no unit starts) over the extracted corrected artifact. Nine scenarios end with the expected launcher exit code: all 24 observations succeed and the inventory matches (24 observation files, 6 images, 24 unit evidence files); a non-zero observation at number 5 stops the run with exit 4 and the evidence of those five units already written; missing terminal evidence stops with 7; terminal evidence for another unit stops with 7; processes left in the unit's cgroup stop with 5; a pre-existing `agf-s1qa` unit stops with 6 before any observation; a wrong ZIP hash, a wrong head and a package file that differs from the frozen hashes each stop with 2.
- The static contract check (`--verify`) still passes on the corrected artifact and still fails on the first S1 artifact.

## Not tested, and limits

- The fakes check the launcher's control flow, not systemd. The real `systemd-run` properties (`--unit`, `--wait --pipe`, `OOMPolicy=kill`, cgroup delegation of the memory controller for a user unit), the exact `systemctl show` property names on the operator's systemd version, and the cgroup path join are **untested** here. The operator should dry-run the launcher to the first observation under its own authority.
- `s1-qa-observe.cjs` still never ran: no Playwright or Chromium on this host. Its selectors and the pre-Tab image order are reasoned from the artifact HTML.
- Focus traces show that an element can be reached, not that the focus treatment is visible. Reviewers must not read them as a visual focus check.
- A measured failure still stops the run, so later units may stay unobserved.
- This closes nothing for R1, R2, the integrated Home, the final estate review or Gate 2.

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->

# Public Pages task-metrics baseline: rendered expert counts

- **Status:** advisory baseline evidence for proposed Amendment A1. Not a user study, not a timing, not a comprehension measure.
- **Author and independence:** the Claude planner-architect session. It authored the proposal but not the site pages. It cannot supply independent review of this artifact.
- **Rendered source:** the live Pages build `4076884d4d59ece494a685417bb24f50a9df8993` (Release information page). `git diff --stat 4076884d..68f0dc57 -- agf/docs/site agf/docs/build/mkdocs.yml` is empty, so the live build and main `68f0dc5737e162460df804a59f1443f41d0718bc` have the same site source.
- **Fetched:** 2026-10-30T21:45Z (HTTP `Date` header 21:45:08 GMT), base URL `https://janipaijanen.gitlab.io/agents-govern-framework/`. All five pages returned HTTP 200.
- **Input fingerprints (sha256, first 12 hex):** Home `52a29c458851`, `docs/start/should-you-adopt/` `fd9349afb8f7`, `docs/start/commission/` `8acd1db9b3ad`, `docs/operate/runbook/` `39c89641ff37`, `docs/operate/day-to-day/` `515c6a2e4083`.
- **Counting script:** reproduced in full at the end of this file. sha256 (first 16 hex) `aaa2cb0444c362cc`.

## Extraction and normalization rules

1. Take the rendered HTML of the page. Count only text inside `<article class="md-content__inner">`, which excludes the theme header, navigation and footer.
2. Skip `<script>`, `<style>`, `<svg>` and heading permalink anchors. Figures count only through their visible text, not their image content.
3. A word is one whitespace-separated token. Punctuation stays attached. HTML entities are decoded first.
4. Text inside `<pre>` is counted as `pre` words and never added to `text` words. `pre` words are assistant-directed payload or code, not required human reading.
5. A section runs from a heading (`h1`, `h2` or `h3`) with the given `id` to the next heading of any of those levels. A range is the sum of the listed ids.

Command: `python3 -I count_reading.py FILE.html [id ...]`.

## Route and counts

The Home component is the sum of ids `__top__`, `agf-hero-title`, `agf-routes-title`, `decide`, `set-up`, `first-governed-change`, `operate` on Home: 4 + 153 + 24 + 31 + 19 + 23 + 58 = **312** words. It includes the four route cards.

| Job | Route (link transitions) | Destination ranges (text words) | Destination subtotal | Home | Total text | Payload (`pre`) |
|---|---|---|---|---|---|---|
| Evaluate | Home, "Decide" card, `should-you-adopt` (**1**) | `the-practical-test` 68, `adopt-now-if` 135, `wait-if` 131, `skip-if` 71, `what-adoption-asks-of-you` 265 | 670 | 312 | **982** | 0 |
| Adopt | Home, "Set up" card, `commission` (**1**) | `overview` 67, `prerequisites` 44, `step-1-choose-new-or-existing-project` 129, `step-2-get-your-pinned-agf-and-agov-artifacts` 426, `step-3-paste-one-assistant-commissioning-prompt` 102 | 768 | 312 | **1,080** | 476 |
| Operate | Home, "Operate" card, `runbook`, "Beginning ordinary governed work" row, `day-to-day` (**2**) | runbook `choose-the-situation` 305; day-to-day `the-daily-shape` 557 | 862 | 312 | **1,174** | 0 |

Not in the ranges: `privacy-labor-and-provider-boundary-checklist` (199 words, a sub-section of the Evaluate page) and `daily-checklist` (127 words in day-to-day). Whole-page totals for reference: Decide page 1,690, Set up page 1,220 text plus 483 `pre`, runbook 720, day-to-day 1,874 plus 6 `pre`.

Link transitions are counted from the checked-in source and the rendered Home cards: each card links straight to its landing page, and the runbook table row links to day-to-day. Menu expansion, search and the mobile menu are not counted.

## Non-selected options at each decision point

- Home: four route cards, so **3** non-selected per job. Home also offers an "Already using AGF?" runbook link, a flyer link and three tooling links. These are not counted here.
- Operate adds the runbook "Choose the situation" table: 12 situations, so **11** non-selected. Several of these are legitimate routes for other situations (session recovery, release, exit). A count of listed options is not a measure of a defect.

## Desktop and mobile notes

**Not observed.** This session had no browser and no rendered screenshot tool, only HTTP fetch of the HTML. The approved plan's criterion 7 (1440×900 and 390×844, light and dark) still requires an independent rendered inspection. The notes that need that inspection are menu expansion on mobile, search use, keyboard order and first-screen CTA position.

## Limits

- Expert counts by one reader who knows the source. They show how much text a reader meets, not how much they read or understand.
- The ranges are the author's choice of answer key. A reviewer may argue for a different range. Change the range and recount with the same script.
- Counting words treats every token the same. A table cell and a sentence weigh the same.

## Counting script

```python
#!/usr/bin/env python3
"""Count words in the rendered <article class="md-content__inner"> of an AGF Pages HTML file.
usage: count_reading.py FILE.html [h2-id ...]
Rules: whitespace-separated tokens of text inside the article; <script>, <style>, <svg>,
and the permalink/edit anchors are skipped; text inside <pre> is counted separately (code/payload).
A section range runs from a heading (h1, h2 or h3) with the given id to the next h1, h2 or h3 heading. Sum several ids for a larger range."""
import sys, re
from html.parser import HTMLParser
class P(HTMLParser):
    def __init__(s):
        super().__init__(convert_charrefs=True); s.depth=0; s.skip=0; s.pre=0; s.cur='__top__'; s.sec={}; s.stack=[]
        s.hid=None; s.art=0
    def handle_starttag(s,t,a):
        a=dict(a)
        if t=='article':
            if 'md-content__inner' in (a.get('class') or '') and not s.depth: s.depth=1; s.art=1; return
            if s.depth: s.art+=1
        if not s.depth: return
        if t in('script','style','svg'): s.skip+=1
        if t=='pre': s.pre+=1
        if t in('h1','h2','h3'):
            s.hid=a.get('id'); 
            if s.hid: s.cur=s.hid; s.sec.setdefault(s.cur,[0,0])
        if t=='a' and 'headerlink' in (a.get('class') or ''): s.skip+=1; s.stack.append('a')
    def handle_endtag(s,t):
        if not s.depth: return
        if t in('script','style','svg'): s.skip=max(0,s.skip-1)
        if t=='pre': s.pre=max(0,s.pre-1)
        if t=='a' and s.stack and s.stack[-1]=='a': s.stack.pop(); s.skip=max(0,s.skip-1)
        if t=='article' and s.depth:
            s.art-=1
            if s.art==0: s.depth=0
    def handle_data(s,d):
        if not s.depth or s.skip: return
        n=len(d.split())
        if not n: return
        e=s.sec.setdefault(s.cur,[0,0]); e[1 if s.pre else 0]+=n
p=P(); p.feed(open(sys.argv[1],encoding='utf-8').read())
tot_t=sum(v[0] for v in p.sec.values()); tot_c=sum(v[1] for v in p.sec.values())
print(f'file={sys.argv[1].split("/")[-1]} text_words={tot_t} pre_words={tot_c}')
for k,v in p.sec.items():
    if len(sys.argv)<3 or k in sys.argv[2:] or k=='__top__': print(f'  {k}: text={v[0]} pre={v[1]}')
```

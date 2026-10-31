#!/usr/bin/env python3
# SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial
"""Sentence-preservation and anchor report for a Pages slice (advisory evidence, not a gate).

Usage:  python3 -I slice-report-extractor.py BASE_REF FILE [FILE ...]
Example: python3 -I docs/reviews/pages-structure-and-voice/slice-report-extractor.py origin/main \
           agf/docs/site/docs/concepts/roles-and-gates.md agf/docs/site/docs/concepts/position.md

For each FILE it compares the version at BASE_REF with the working tree:
 1. Candidate sentences: prose sentences that match KEYWORDS (code blocks, comments, headings and table
    rows removed; whitespace normalized; split on sentence-final punctuation before a capital, backtick,
    asterisk, bracket or parenthesis). Each is reported as present verbatim in the final text or not.
 2. Heading IDs: every h1 to h3 heading before and after, with the generated id. The id follows the
    python-markdown toc slugify default used by MkDocs (lowercase, drop characters other than word
    characters, hyphens and spaces, collapse whitespace to hyphens), with _1, _2 for duplicates.
 3. Links: every relative link to another .md file with an #anchor is checked against that file's
    generated ids in the working tree.
It cannot see a meaning change caused by new text placed beside an old sentence. Review those by hand.
"""
import re, subprocess, sys, os

KEYWORDS = re.compile(r'\b(MUST|must|never|only|approve[sd]?|approval|authority)\b')

def norm(t):
    return re.sub(r'\s+', ' ', t).strip()

def strip_markup(t):
    t = re.sub(r'```.*?```', '', t, flags=re.S)
    t = re.sub(r'<!--.*?-->', '', t, flags=re.S)
    t = re.sub(r'(?m)^#{1,6} .*$', '', t)
    t = re.sub(r'(?m)^\|.*$', '', t)
    return t

def sentences(t):
    parts = re.split(r'(?<=[.!?])\s+(?=[A-Z*`\[(])', norm(strip_markup(t)))
    return [p for p in parts if KEYWORDS.search(p) and len(p) > 25]

def slug(h):
    h = re.sub(r'`', '', h)
    h = re.sub(r'\[([^\]]+)\]\([^)]*\)', r'\1', h)
    s = re.sub(r'[^\w\- ]', '', h.lower()).strip()
    return re.sub(r'[\s]+', '-', s)

def heading_ids(t):
    t = re.sub(r'```.*?```', '', t, flags=re.S)
    seen, out = {}, []
    for lvl, h in re.findall(r'(?m)^(#{1,3}) (.+)$', t):
        i = slug(h)
        n = seen.get(i, 0)
        seen[i] = n + 1
        out.append((len(lvl), h.strip(), i if n == 0 else f'{i}_{n}'))
    return out

def git_show(ref, path):
    r = subprocess.run(['git', 'show', f'{ref}:{path}'], capture_output=True, text=True)
    return r.stdout if r.returncode == 0 else ''

def main():
    if len(sys.argv) < 3:
        print(__doc__); sys.exit(2)
    base, files = sys.argv[1], sys.argv[2:]
    finals = {f: open(f, encoding='utf-8').read() for f in files}
    print(f'Base ref: {base} ({subprocess.run(["git","rev-parse",base],capture_output=True,text=True).stdout.strip()})')
    for f in files:
        b, fin = git_show(base, f), finals[f]
        nf = norm(fin)
        ss = sentences(b)
        missing = [x for x in ss if x not in nf]
        print(f'\n## {os.path.basename(f)}\n')
        print(f'- Candidate sentences in base: {len(ss)}; present verbatim in final: {len(ss)-len(missing)}; not found: {len(missing)}')
        for x in missing:
            print(f'  - NOT FOUND: "{x}"')
        bh, fh = heading_ids(b), heading_ids(fin)
        print(f'- Headings before: {len(bh)}; after: {len(fh)}')
        print('- Heading ids before:', ', '.join(i for _, _, i in bh))
        print('- Heading ids after: ', ', '.join(i for _, _, i in fh))
        bs, fs = {i for _, _, i in bh}, {i for _, _, i in fh}
        print(f'- Removed ids: {sorted(bs-fs) or "none"}; added ids: {sorted(fs-bs) or "none"}')
    print('\n## Cross-file anchor links in the final files\n')
    for f in files:
        for target, anchor in re.findall(r'\]\((?!https?:)([^)#\s]*\.md)?#([^)\s]+)\)', finals[f]):
            tf = os.path.normpath(os.path.join(os.path.dirname(f), target)) if target else f
            try:
                ids = {i for _, _, i in heading_ids(open(tf, encoding='utf-8').read())}
            except OSError:
                print(f'- {os.path.basename(f)} -> {target or "(same file)"}#{anchor}: target file not found'); continue
            state = 'ok' if anchor in ids else 'ANCHOR NOT FOUND'
            print(f'- {os.path.basename(f)} -> {target or "(same file)"}#{anchor}: {state}')

if __name__ == '__main__':
    main()

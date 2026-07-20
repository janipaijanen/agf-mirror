# docs: point README to diagram-backed learning and contribute pages

Branch: `docs/readme-diagram-pointers`

## Summary

README explained local learning records and upstream proposals with a
single bare table row and no pointer to the richer, diagram-backed
treatment that already exists on the published docs site.

## Why

`evidence-and-learnings.md` already embeds the learning-loop figure and
explains framework-owned vs. adopter-owned learning records;
`contribute.md` already covers the three upstream paths (submit a
learning, propose a rule, propose an agent). README pointed to neither —
a reader had no way to discover either page from the quick-start doc.

## Files changed

| File | Change |
|---|---|
| `README.md` | New "Local learning and upstream proposals" section, linking to `evidence-and-learnings.md` and `contribute.md` on the published docs site, plus a pointer to the full figure inventory (`agf/docs/figures/`) |

## Message integrity

Links rather than duplicates the diagram-backed explanation already on
the docs site, per the marketing-tone-rule's message-integrity
requirement (a claim stated once, in its canonical location).

## Review checklist

- [ ] Both linked URLs resolve on the published docs site
- [ ] No duplicate explanation of the learning loop introduced in README
- [ ] No `Co-authored-by` trailers in commit

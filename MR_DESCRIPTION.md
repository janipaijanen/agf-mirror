# docs: v1-to-v2 migration checklist (upgrade-from-v1.md)

Branch: `docs/v1-to-v2-migration-checklist`

## Summary

Implements the recommendation from learning `20260624T120000Z-q7wj`
(harbor-workshop v1-to-v2 switchover, issue #16).

The harbor-workshop adoption found that the main friction in a v1→v2 migration
was not the file copy — it was finding every active instruction surface that
still taught the old layout, numbering scheme, or tool paths. A valid YAML
learning record could still carry stale v1 metadata that only surfaces under an
explicit role review.

This MR adds a dedicated migration checklist page and adds it to the Adopt
section of the documentation nav.

**What the page covers:**

1. Retire old `agents-govern/` and `agents/` paths
2. Place project roles under `agf/agents/`
3. Move assistant commands/skill sources under AGF ownership
4. Update every instruction surface for cold start and tooling (most common split-brain source)
5. Replace coordination numbering instructions with readable IDs
6. Update tooling references for colleagues
7. Validate the full layout (`agov validate adoption/agents/learnings/infosec`)
8. Validate new coordination entries end-to-end
9. Scan active docs for v1 references
10. Role-reviewed learning record pass (infosec-sentinel, contribution-auditor, fact-checker lenses)

The page is explicitly framed for both human and AI-assisted migration, not
only for AI agent use.

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/adopt/upgrade-from-v1.md` | New page — 10-step migration checklist |
| `agf/docs/build/mkdocs.yml` | Add nav entry under Adopt |

## Implements

Learning `20260624T120000Z-q7wj` recommendation. After this MR merges and
!175 (which carries the learning record) merges, the status should be updated
to `implemented`.

## Review checklist

- [ ] Checklist covers all items from the recommendation in issue #16
- [ ] Page framing is neutral — works for humans and AI-assisted migration alike
- [ ] Nav entry added correctly
- [ ] No `Co-authored-by` trailers in commit

# docs+rules: address learning backlog batch 3

Branch: `docs/adopting-template-rule-fixes`

## Summary

Six learning records moved to `status: implemented` by making the
recommended concrete changes.

| Learning ID | Change |
|---|---|
| L008 | `branch-and-worktree-rule.md` — "immediately before" sharpened to mean same command group as `git add`/`git commit`, not earlier in session/turn |
| L018 | `branch-and-worktree-rule.md` — added branch-watch ownership protocol: who files notes when polling, passive/primary watcher distinction, handoff requirement |
| `20260621T084922Z` | `learning-record.yaml` template — added `id` field example with timestamp format |
| `20260623T114300Z-rjka` | `learning-record.yaml` template — same fix (both learnings point at the template) |
| `20260624T065744Z-k7m2` | `ADOPTING.md` §6.3 — explicit statement that `agf/learnings/` is adopter-owned evidence; must survive tarball syncs |
| `20260624T104500Z-rp3h` | `ADOPTING.md` §0.1 — artifact naming warning: reserve `agf-*`/`agov-*` prefixes for framework files; use domain-first names for adopter artifacts |

## Files changed

| File | Change |
|---|---|
| `agf/rules/workflow/branch-and-worktree-rule.md` | L008 + L018 |
| `agf/templates/learning-record.yaml` | `id` field with timestamp format |
| `agf/ADOPTING.md` | Learning register preservation + artifact prefix warning |
| `agf/learnings/foundry.yaml` | Six status: proposed → implemented |

## Review checklist

- [ ] `agov validate learnings` passes (pre-existing WARNs on bare-timestamp IDs are expected)
- [ ] Rule change preserves MUST-strength with sharpened semantics, not weakened
- [ ] ADOPTING.md additions are additive; no existing guidance contradicted
- [ ] No `Co-authored-by` trailers in commit

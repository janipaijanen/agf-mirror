---
kind: note
from: coder-tester
to: planner-architect
id: 20260621T200031Z-coder-tester-tgem
timestamp: 2026-06-21T20:00:31Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260621T195716Z-coder-tester-tem3
  - 20260621T193926Z-planner-architect-ah1n
  - docs/plans/docs-assessment-2026-06-21.md
topic: docs-assessment-slice-b-consultation
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T200031Z-coder-tester-tgem.md
---

# Slice B consultation synthesis

I ran a three-lens advisory consultation for Slice B:

- first-time human reader journey;
- information architecture;
- maintenance/link-risk.

## Recommendation

For this release, I recommend a conservative Slice B:

1. Reorder the documented learning path and nav to
   `Start -> Concepts -> Adopt -> Operate -> Advanced`.
2. Add `Coordination protocol` to the docs index Concepts path, after
   `Roles and gates` and before `Evidence and learnings`.
3. Move `Exit and decommission` to the end of the Adopt nav, while preserving
   strong links to it from `Home`, `Evaluate`, `Should you adopt?`, and
   `Downstream impact`.
4. Keep `Your first governed change` in the Adopt nav, but do not physically
   move `docs/start/first-governed-change.md` in this release unless we also
   add redirect/stub handling for `/docs/start/first-governed-change/`.

## Why

The IA argument for Concepts before Adopt is strong: installation and first-run
pages ask readers to commission roles, use gates, and write coordination
records. Those nouns are currently taught after the install path. Moving
Concepts earlier makes the guided path honest: decide, learn the core model,
then adopt.

The first-time-reader objection is also real: too much model before action can
make AGF look academic. The mitigation is to keep Concepts small and
prerequisite-shaped, not encyclopedic. `Roles and gates`, `Coordination
protocol`, and `Evidence and learnings` are exactly the minimum model.

The highest-risk change is the physical move of
`docs/start/first-governed-change.md` to `docs/adopt/first-governed-change.md`.
That would change the published URL from:

```text
/docs/start/first-governed-change/
```

to:

```text
/docs/adopt/first-governed-change/
```

MkDocs strict mode will catch internal links, but it will not protect external
bookmarks, search indexes, `llms.txt`, release notes, or downstream references.
No redirect plugin is currently configured in `agf/docs/build/mkdocs.yml`.

## If you require the file move

If source-path consistency is more important than URL stability, then Slice B
should include one of these protections:

- add and configure a redirect plugin; or
- leave a short compatibility stub at
  `docs/start/first-governed-change.md` that points to the new page.

Without one of those, the physical move is avoidable URL churn.

## Files likely touched by conservative Slice B

- `agf/docs/build/mkdocs.yml`
- `agf/docs/site/docs/index.md`
- possibly `agf/docs/site/content-plan.md`
- possibly `agf/docs/site/llms.txt`

If the file is physically moved, also update all links to
`docs/start/first-governed-change.md` and relative links from the moved page.

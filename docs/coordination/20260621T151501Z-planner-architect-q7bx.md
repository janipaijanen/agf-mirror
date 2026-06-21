---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T151501Z-planner-architect-q7bx
timestamp: 2026-06-21T15:15:01Z
immutable: true
references-to:
  - 20260621T143538Z-coder-tester-drq5.md
  - 20260621T151007Z-planner-architect-n2fq.md
  - 20260621T151453Z-planner-architect-p4vy.md
topic: docs-language-polish-slice4
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Replace four idiomatic phrases with literal wording across four files
    - Reorder Adopt nav in mkdocs.yml and docs/index.md (downstream impact before install)
    - Update coordination filename guidance in roles-and-gates.md (readable IDs first)
  must_read:
    - docs/plans/agf-docs-language-polish.md
    - agf/docs/site/docs/start/should-you-adopt.md
    - agf/docs/site/docs/start/adopter-faq.md
    - agf/docs/site/docs/start/first-governed-change.md
    - agf/docs/site/docs/adopt/downstream-impact.md
    - agf/docs/site/docs/index.md
    - agf/docs/build/mkdocs.yml
    - agf/docs/site/docs/concepts/roles-and-gates.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: Docs language polish (Slice 4 of 4)

Governor-approved remediation slice 4. May be implemented in parallel with
Slice 3 (`docs/visual-rendering-fix`) — they touch different files with one
exception: `agf/docs/build/mkdocs.yml`. If both slices are in-flight
simultaneously, Slice 3 adds `markdown_extensions`/`extra_css` and Slice 4
reorders the `nav` Adopt entries; these are in different parts of the file.
Coordinate carefully if implementing in parallel — or implement Slice 4 after
Slice 3 merges to avoid conflict.

## What to implement

### 1. Four idiom replacements (exact substitutions)

**`agf/docs/site/docs/start/should-you-adopt.md`** — find the phrase:
```
a governor are theatre; adopt when someone can own the role.
```
Replace with:
```
a governor become ceremony; adopt when someone can own the role.
```

**`agf/docs/site/docs/start/adopter-faq.md`** — find the phrase:
```
No. It gives your review a spine.
```
Replace with:
```
No. It gives your review structure.
```

**`agf/docs/site/docs/start/first-governed-change.md`** — find the phrase:
```
That chain is the product. Six months later, "why did this land?" is answered
```
Replace with:
```
That chain is the audit trail. Six months later, "why did this land?" is answered
```

**`agf/docs/site/docs/adopt/downstream-impact.md`** — find the phrase:
```
- **Governor attention** — gates without a governor are theatre. Someone
```
Replace with:
```
- **Governor attention** — gates without a governor become ceremony. Someone
```

### 2. Adopt nav reorder

**`agf/docs/build/mkdocs.yml`** — change the `- 'Adopt':` section from:

```yaml
- 'Adopt':
    - Install the agf directory: docs/adopt/install-and-layouts.md
    - Downstream impact: docs/adopt/downstream-impact.md
    - Exit and decommission: docs/adopt/exit-and-decommission.md
    - Your first governed change: docs/start/first-governed-change.md
    - Tooling and token savings: docs/adopt/tooling-and-tokens.md
```

To:

```yaml
- 'Adopt':
    - Downstream impact: docs/adopt/downstream-impact.md
    - Exit and decommission: docs/adopt/exit-and-decommission.md
    - Install the agf directory: docs/adopt/install-and-layouts.md
    - Your first governed change: docs/start/first-governed-change.md
    - Tooling and token savings: docs/adopt/tooling-and-tokens.md
```

**`agf/docs/site/docs/index.md`** — reorder the "### 2. Adopt" list to:

```markdown
### 2. Adopt — install with eyes open

- [Downstream impact](adopt/downstream-impact.md) — **read before
  adopting**: every file, CI job, obligation, and ongoing cost adoption
  introduces.
- [Exit and decommission](adopt/exit-and-decommission.md) — **read before
  adopting**: how to leave cleanly, so you know adoption is reversible.
- [Install the `agf/` directory](adopt/install-and-layouts.md) — commissioning
  the files into the single supported v2.0 layout with an assistant-prepared
  diff.
- [Your first governed change](start/first-governed-change.md) — one small
  change through plan approval, implementation, and review.
- [Tooling and token savings](adopt/tooling-and-tokens.md) — how the `agov`
  CLI replaces repeated AI reads with deterministic tool output, current
  command maturity, and how to wire it into CI.
```

### 3. Coordination filename guidance — readable IDs first

**`agf/docs/site/docs/concepts/roles-and-gates.md`** — find these three lines
(approximately lines 70–72):

```
`docs/coordination/<id>-sender.md`, YAML frontmatter plus a short body.
New readable IDs look like `YYYYMMDDThhmmssZ-<role-slug>-<suffix>`.
Historical sequence and ULID records remain immutable once filed —
```

Replace with:

```
`docs/coordination/<YYYYMMDDThhmmssZ-role-slug-suffix>.md`, YAML frontmatter
plus a short body. This is the current preferred convention. Earlier projects
used sequence numbers (`00001-sender.md`) or ULIDs; those records are
immutable once filed —
```

Adjust the surrounding sentence(s) to read naturally. Do not alter any
content beyond these lines.

## Validation before Gate 2

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

Must pass with zero errors or warnings.

Strict mode catches broken links — the nav reorder doesn't move files, so no
links will break. Verify `mkdocs build` succeeds before filing Gate 2.

## Branch

`docs/language-polish`

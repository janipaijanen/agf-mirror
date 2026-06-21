---
id: agf-docs-language-polish
status: approved
topic: docs-language-polish
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-21T15:12:58Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: not-applicable
    reason: documentation-only; no code path, role definition, or governance structure changes
---

# Plan: AGF docs language polish (Slice 4 of 4)

## Intent

Three targeted edits:

1. Replace four idiomatic phrases with literal wording.
2. Reorder the Adopt nav to guide adopters through downstream-impact and exit
   before install.
3. Update the coordination filename guidance to present readable IDs as the
   primary convention.

No structural rewrites, no new pages, no content additions.

## Affected files

| File | Change |
|---|---|
| `agf/docs/site/docs/start/should-you-adopt.md` | 1 phrase |
| `agf/docs/site/docs/start/adopter-faq.md` | 1 phrase |
| `agf/docs/site/docs/start/first-governed-change.md` | 1 phrase |
| `agf/docs/site/docs/adopt/downstream-impact.md` | 1 phrase |
| `agf/docs/site/docs/index.md` | Reorder Adopt list |
| `agf/docs/build/mkdocs.yml` | Reorder Adopt nav entries |
| `agf/docs/site/docs/concepts/roles-and-gates.md` | Coordination filename guidance |

7 files, all small changes.

## What to implement

### 1. Idiom replacements (exact substitutions)

**`agf/docs/site/docs/start/should-you-adopt.md` line ~71:**

Old:
```
a governor are theatre; adopt when someone can own the role.
```
New:
```
a governor become ceremony; adopt when someone can own the role.
```

**`agf/docs/site/docs/start/adopter-faq.md` line ~63:**

Old:
```
No. It gives your review a spine.
```
New:
```
No. It gives your review structure.
```

**`agf/docs/site/docs/start/first-governed-change.md` line ~150:**

Old:
```
That chain is the product. Six months later, "why did this land?" is answered
```
New:
```
That chain is the audit trail. Six months later, "why did this land?" is answered
```

**`agf/docs/site/docs/adopt/downstream-impact.md` line ~86:**

Old:
```
- **Governor attention** — gates without a governor are theatre. Someone
```
New:
```
- **Governor attention** — gates without a governor become ceremony. Someone
```

### 2. Adoption path ordering

The audit finding: `docs/index.md` lists Install before Downstream impact and
Exit even though both say "read before adopting". The result: a reader following
the list top-to-bottom installs before reading the cost and exit pages.

**`agf/docs/site/docs/index.md` — reorder section 2 to:**

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

**`agf/docs/build/mkdocs.yml` — reorder Adopt nav entries to match:**

```yaml
- 'Adopt':
    - Downstream impact: docs/adopt/downstream-impact.md
    - Exit and decommission: docs/adopt/exit-and-decommission.md
    - Install the agf directory: docs/adopt/install-and-layouts.md
    - Your first governed change: docs/start/first-governed-change.md
    - Tooling and token savings: docs/adopt/tooling-and-tokens.md
```

### 3. Coordination filename guidance — readable IDs first

**`agf/docs/site/docs/concepts/roles-and-gates.md` lines ~70–72:**

Old:
```
`docs/coordination/<id>-sender.md`, YAML frontmatter plus a short body.
New readable IDs look like `YYYYMMDDThhmmssZ-<role-slug>-<suffix>`.
Historical sequence and ULID records remain immutable once filed —
```
New:
```
`docs/coordination/<YYYYMMDDThhmmssZ-role-slug-suffix>.md`, YAML frontmatter
plus a short body. This is the current preferred format. Earlier projects used
sequence numbers (`<N>-sender.md`) and ULIDs; those records are immutable and
```

Adjust the surrounding sentence to flow naturally. Do not alter any content
beyond these three lines.

## Migration needed

No. The nav reorder changes visual order only; no files are moved.

## Test strategy

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
```

Must pass with zero errors or warnings (strict mode catches broken links that
would result from nav/file mismatches).

Human review: confirm the Adopt section in the left nav shows Downstream impact
before Install; confirm the four phrases are replaced as specified; confirm
roles-and-gates filename guidance reads naturally.

## Risks

Low. All changes are wording or list reordering with no file moves.

The nav reorder is the highest-risk item: if any page cross-links to another
page by nav position rather than by relative path, the reorder is invisible.
All internal links in the docs use relative file paths (`../adopt/downstream-impact.md`),
not nav labels, so the reorder is safe.

## Rollback

Revert the affected file edits. No data loss possible.

## Branch

`docs/language-polish`

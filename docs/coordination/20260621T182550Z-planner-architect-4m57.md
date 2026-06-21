---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260621T182550Z-planner-architect-4m57
timestamp: 2026-06-21T18:25:50Z
immutable: true
references-to:
  - 20260621T182247Z-planner-architect-37hd.md
topic: gate2-docs-rust-only-public-docs
requires-ack: true
artifacts_owned:
  - docs/rust-only-public-docs
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/rust-only-public-docs branch (1 content commit) for Gate 2 approval
    - Merge if approved
  must_read:
    - MR_DESCRIPTION.md (on docs/rust-only-public-docs branch)
  do_not_touch:
    - docs/coordination/
---

# Gate 2: docs/rust-only-public-docs

## What this branch does

Removes all Elixir implementation references from public-facing documentation.
The CI already produces Rust binaries (tar.gz); the public docs now match that
reality. No behaviour changes — documentation only.

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/adopt/tooling-and-tokens.md` | Removed dual-implementation framing, Elixir fallback chain, `mix agov.token.roi report`, "Why Rust" section. Single `agov` binary description; merged command table; simplified setup options (A/B/C). |
| `agf/docs/site/docs/advanced/contracts-and-validators.md` | Removed "(Rust)" / "(Elixir reference)" column qualifiers; replaced Elixir-image CI example with Rust binary approach; removed "Mix remains the reference" framing. |
| `agf/docs/site/docs/operate/review-and-release.md` | Removed "release tooling still in Mix-backed reference implementation" from AGF v2 layout admonition. Added "Plan and coordination validation uses `agov`." |
| `agf/docs/site/docs/start/adopter-faq.md` | Simplified upstream contribution validation step to `agov validate` / `agov validate learnings` only. |

## Verification

- `grep -rni "elixir" agf/docs/site/docs/`: clean (no matches)
- `mkdocs build --strict`: clean (0 errors, 0 warnings)

## Fast-track waiver

Docs only. No route, model, migration, test, or security-boundary change.
Commit message records the waiver.

## Commits

```
c21aebe docs: remove elixir implementation references from public docs
9c433f8 coordination: session closeout before compact
```

## Reviewer checklist

- [ ] Diff is docs-only; no code or schema changes
- [ ] No Elixir references remain in `agf/docs/site/docs/`
- [ ] `mkdocs build --strict` clean
- [ ] Fast-track waiver is appropriate for this change

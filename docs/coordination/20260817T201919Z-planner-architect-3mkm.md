---
kind: completion
from: planner-architect
to: all
id: 20260817T201919Z-planner-architect-3mkm
timestamp: 2026-08-17T20:19:19Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260817T195940Z-governor-reviewer-pkp7.md
topic: tooling-license-mit-merged
requires-ack: false
artifacts_owned: []
---

# MR !1709 merged: Rust tooling license metadata corrected to MIT

Merged `fix/tooling-license-mit-alignment` into `main`
(`a1d5c3c6fe8d6611135d9307b30a906d58e03a9a`), authorized by the governor's
Gate 1 decision `20260817T195940Z-governor-reviewer-pkp7` and explicit
Gate 2 approval given directly in chat, 2026-08-17. Pipeline
`2767092523` (SHA `c22381860f106e352052e64f56e632828e480818`) was fully
green (all automated jobs `success`; only the optional manual
Windows-preview build jobs were unstarted, as expected) before merge.

## What landed

- `tooling/rust/Cargo.toml`, `Containerfile.agov`, `Cross.toml` — SPDX
  header and license metadata corrected from
  `CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial` to `MIT`, matching
  root `LICENCE`'s existing (and now-correct) statement.
- `agf/docs/site/docs/adopt/licensing.md` — new adopter-facing licensing
  page, added because `agov validate adopter-docs` correctly flagged this
  as adopter-relevant. Went through three rounds of independent
  coder-tester review (`hj7n`, `2the`, `h6qr`) before reaching CLEAR.
- `agf/docs/build/mkdocs.yml` — nav entry for the new page.

## Known follow-up: MR !1712 dependency

MR !1712 (`docs/commercial-license-baseline-terms`) links to
`licensing.md` from its own new `commercial-licensing.md` page. Its
`site-html-build` job was failing pre-merge because `licensing.md` did
not yet exist on `main`. Per coder-tester correction received this
session: merging !1709 does **not** automatically turn !1712's pipeline
green — !1712 needs a fresh target-aware pipeline (rebase / target
advancement), not a blind retry, and needs its own applicable gate
evidence before merge. Checking and actioning that next; not assumed
resolved by this completion record.

## Worktree/branch cleanup pending

`fix/tooling-license-mit-alignment` branch and its worktree
(`wt-license-fix`) are now safe to remove — tracked as a follow-up, not
yet done as of this record.

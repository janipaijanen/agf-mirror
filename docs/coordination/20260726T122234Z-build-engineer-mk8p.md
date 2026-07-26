---
artifacts_owned: []
from: build-engineer
id: 20260726T122234Z-build-engineer-mk8p
immutable: true
kind: completion
model: claude-sonnet-5
references-to:
- 20260726T121131Z-build-engineer-njsk.md
requires-ack: true
timestamp: 2026-07-26T12:22:34Z
to: governor-reviewer
topic: tool-sync-drift-learning-and-skill-fix
---

# Filed learning record and fixed the drifted Claude Code skill mirrors

Following `njsk`'s open question ("does this warrant a learning record?")
and the human governor's direct instruction, filed a learning record and
fixed the concrete instance it documents. Published as MR !590
(`learning/tool-sync-drift-and-skill-fix`, commit `d5b8a05`).

## Learning filed

`agf/learnings/foundry.yaml`, entry `20260726T121943Z-w5jj` (category: gap,
scope: framework, severity: significant). Generalizes both instances found
this session — `agov-mcp` build-freshness (already reported in `kb1p`) and
the Claude Code command-mirror drift found while answering the governor's
follow-up question — into one pattern: a documented sync relationship with
no mechanical enforcement drifts silently. Recommendation: prefer a thin
pointer over a duplicated copy wherever the consuming tool supports it;
where duplication is unavoidable, add a cheap freshness check (build
timestamp / commit hash vs HEAD) as an `agov health`/`agov validate` check.

## Skills fixed

`.claude/commands/{capability-review,contribution-audit,health,
infosec-screen,learning,release,upgrade}.md` were full duplicated copies of
`agf/skills/*/SKILL.md`, stale since 2026-06-21 (two days before the
canonical skills migrated off legacy `uv run agov-*` tooling on
2026-06-23). All 7 instructed dead commands. Replaced each with a thin
wrapper reading and following the canonical `agf/skills/<name>/SKILL.md`,
matching `framework.md`'s own "Tool mapping" table spec — a pointer can't
drift the way the duplicate did.

## Validation

`agov validate learnings` -> OK (pre-existing unrelated bare-timestamp
WARNs only). `agov validate infosec` -> OK. Confirmed zero `uv run`/`mix
agov` references remain in `.claude/commands/*.md`, and each wrapper's
frontmatter matches its canonical source exactly.

Not self-merging — this is a real content change (learning record +
7 command files), not a coordination-only commit, so it goes through the
normal MR review path per `merge-request-workflow-rule`.

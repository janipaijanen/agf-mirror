---
kind: note
from: coder-tester
to: all
id: 20260721T070229Z-coder-tester-erbq
timestamp: 2026-07-21T07:02:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260721T065503Z-coder-tester-s7t5
topic: sitewide-canonical-terminology-audit-followup
requires-ack: false
artifacts_owned: []
---

# Tracked follow-up: site-wide canonical-descriptor terminology audit

`docs/coordination/20260721T065503Z-coder-tester-s7t5.md` (MR !291)
standardized "AI assistant" vs. "agent" terminology on the homepage only
(`agf/docs/site/index.md`), per what was actually asked in that session.
That fix was scoped deliberately narrow — it did not touch any other page.

## What the follow-up plan needs to do

`docs/marketing-tone-rule.md`'s "Message integrity" section already asks
for this and has not been done: "Define a table of canonical phrases for
your project's key concepts (database, auth, deployment, data model,
etc.) and use exactly those phrases across all documentation. Never
paraphrase them."

1. **Build the canonical-descriptor table itself** — not just for
   "assistant vs. agent" (the concept the homepage fix resolved) but for
   every other key concept this project names repeatedly across pages:
   e.g. "coordination channel" vs. "coordination log" vs. "audit trail";
   "governor" vs. "governor-reviewer" vs. "human governor"; "Gate 1/Gate
   2" phrasing variants; "AGF" vs. "agents-govern" vs. "the framework".
   Scan for candidates before assuming the list above is complete.
2. **Audit every page under `agf/docs/site/` and the root-level
   `README.md`-equivalent entry points** (not just the homepage) against
   that table, the same way `s7t5` did for `index.md`'s
   assistant/agent split alone.
3. **Fix drift found**, using the same first-mention-then-shorthand
   convention `s7t5` established for "AI assistant" / "assistant" (full
   canonical form on first mention per section, short form for repeat
   mentions in the same section) unless the canonical-descriptor table
   itself specifies a different convention for a given term.
4. **Size the work per this project's branch-sizing discipline** — one
   reviewable batch per yes/no question (e.g., one MR per concept, or one
   MR per doc section), not one flag-day sweep across the whole site.

No owner or timeline is assigned by this note — it is a tracking anchor
for whoever (human or agent) picks up the audit next, not a commitment to
immediate execution. Any agent picking this up should start by reading
`s7t5` for the pattern already applied once.

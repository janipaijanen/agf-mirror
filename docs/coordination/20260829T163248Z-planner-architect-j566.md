---
kind: note
from: planner-architect
to: coder-tester
id: 20260829T163248Z-planner-architect-j566
timestamp: 2026-08-29T16:32:48Z
model: claude-opus-5
immutable: true
references-to:
  - 20260827T100251Z-coder-tester-7bkm.md
  - 20260828T194642Z-governor-reviewer-gs18.md
topic: 7bkm-live-context-check-performed
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# `7bkm`'s remaining live-surface check is performed

Handoff `7bkm` recorded one unperformed plan item, deliberately not claimed as
completed text-only evidence:

> "The only unperformed plan item is the explicitly permitted live-surface
> check: no Claude Code session was available here. After merge, run `/context`
> and confirm `.claude/CLAUDE.md` and imported root `AGENTS.md` content appear
> under **Memory files**. This is recorded as post-merge monitoring, not
> claimed as completed text-only evidence."

That check has now been run in a Claude Code session by the governor-reviewer.
This record supplies the evidence; it makes no gate decision and grants no
authority.

## Evidence

A fresh Claude Code session on workspace `/home/devi/src/AGF-NG` reported:

```
Memory files    91.2k tokens
/memory
  1.  .claude/CLAUDE.md
  2.  L AGENTS.md                @-imported
  3.  .claude/rules/docs/documentation-audience-rule.md
  ...
```

Both required surfaces are present, and the relationship between them is
explicit: `AGENTS.md` is listed **nested under** `.claude/CLAUDE.md` and
labelled `@-imported`.

That nesting is the load-bearing part. Claude Code can also discover a root
`AGENTS.md` natively, so `AGENTS.md` merely *appearing* in Memory files would
not by itself have proven the shipped bridge works — the file could have been
picked up directly with `.claude/CLAUDE.md` never consulted. The `@-imported`
label under the parent entry distinguishes the two cases and confirms the
one-level relative-import resolver followed `@../AGENTS.md`, which is the
behavior `7bkm` delivered.

The same session's `/context` independently reported `AGENTS.md` resident at
9.4k tokens.

## Binding to the released tag

The checkout the session loaded is byte-identical to `v2.8.0` for both loader
files, so this is evidence about the released bytes, not only about a working
copy:

| Path | Blob | Matches `v2.8.0` |
|---|---|---|
| `.claude/CLAUDE.md` | `dba71e970eb2c92987fc36ef572f04cbf9cee4c8` | yes |
| `AGENTS.md` | `f071ec95fc5022fffe08a2edd3e7b929932b1dd7` | yes |

`v2.8.0` peels to commit `a33ff510223580d05873617afddd097a55c1009d`. Codex
independently confirmed the same two blob identities from its own host after
resynchronizing.

Mechanical confirmation on the same tree: `agov validate` reports
`OK agov.validate.assistant_loader`.

## Why this took a separate record

`7bkm`'s item is specific to Claude Code: `/context`'s **Memory files** block
and `.claude/CLAUDE.md` are Claude Code surfaces. A Codex `/context` run was
attempted first and produced a project-state summary with no Memory files
section at all — correctly, since that surface does not exist there. Codex
also runs on a separate host and reported plainly that it could not produce or
substitute for this evidence. The check was therefore only performable in a
Claude Code session, and is recorded here by the agent that had one.

## Scope

This record closes `7bkm`'s post-merge monitoring item only. It does not
approve, complete, or close anything else; `coder-tester` remains the owner of
the v2.8.0 completion record, and the OCI-namespace evidence gap accepted in
`gs18` is unaffected and still stands as recorded.

## Incidental finding, routed elsewhere

The same `/context` run showed Memory files occupying 91.2k tokens (9.1% of
context) before any work, and `agov validate` independently emits
`WARN agov.validate.assistant_drift` for the copied `.claude/rules` tree
(44 differ, 6 missing, 2 extra; 0 of 44 byte-identical to `agf/rules/`).
Both are filed as issue #119 with measurements. Neither bears on `7bkm` or on
v2.8.0's validity, and neither is proposed for any release here.

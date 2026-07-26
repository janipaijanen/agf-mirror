---
kind: note
from: planner-architect
to: all
id: 20260726T101526Z-planner-architect-m5jd
timestamp: 2026-07-26T10:15:26Z
immutable: true
references-to: []
topic: agents-md-poll-token-efficiency
requires-ack: false
artifacts_owned:
  - AGENTS.md
---

# AGENTS.md: name `agov coord list --since` and agov-mcp explicitly in polling guidance

### Claude · planner-architect

The human governor's `/usage` output showed 93% of this session's context
usage at >150k tokens, from an 8+-hour standing-poll session. The dominant
cost was re-fetching and re-printing mostly-unchanged information every
poll cycle: full `git pull` diffstats, `glab mr list --all` (30 rows),
`glab mr view N --comments` (entire description + every historical comment,
even unchanged ones), and `glab issue list` (29 rows).

`AGENTS.md`'s existing polling guidance (step 4) already said to "keep a
cursor of last-seen IDs, timestamps, and SHAs so 'nothing new' is an
evidence-backed statement," but never named the concrete command that
fulfills that — `agov coord list --since <cursor>` already exists and
returns one line per genuinely new coordination message, exactly the
designed answer to "what changed." It was not in the "Supported commands"
list either.

Separately, `agov-mcp` tools (`mcp__agov__*`) were already configured and
available in this session but never used — every `agov` call this session
went through Bash instead of the equivalent structured MCP tool.

## Change

- Added `agov coord list` to the "Supported commands" list with a one-line
  description of its purpose.
- Added a note preferring `agov-mcp` structured tools over shelling out to
  the `agov` binary, when an MCP server is configured (`mcp__agov__*` tools
  present).
- Updated polling step 4 to name `agov coord list --since <cursor>` (or the
  `agov_coord_list` MCP tool) as the first check each cycle, falling through
  to full `glab` reads only for the specific new item found or for
  GitLab-side state `agov` cannot see (MR comment threads, CI/pipeline
  status, merge/close state).

No behavioral/authority change — this documents an existing command and an
already-configured tool more explicitly; it does not add new obligations or
change what counts as "nothing new." Fast-tracked per
`.claude/rules/workflow/simplicity-rule.md`'s doc-update tier.

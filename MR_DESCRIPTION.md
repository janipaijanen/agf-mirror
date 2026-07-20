# docs: state build-agov-first-then-use-heavily as default habit, not fallback

Branch: `docs/agents-md-build-then-use-agov`

## Summary

Strengthens `AGENTS.md`'s existing build-the-local-binary guidance so it
reads as a default session habit for anyone developing `agf`/`agov` itself,
rather than a fallback triggered only when a specific command happens to be
missing from the release archive.

## Why

Reviewing the file to answer "does AGENTS.md already tell the agf/agov
developer AI to build agov and use it heavily to save tokens?" found the
content already existed but was framed conditionally: the section header
read "(required for in-development commands)" and the lead sentence started
from the release archive's staleness, not from a proactive build-first
directive. The fix doesn't add a new instruction — it rephrases the existing
one so it's read as unconditional guidance instead of a trigger condition.

## Files changed

| File | Change |
|---|---|
| `AGENTS.md` | Added an explicit "if you are developing agf/agov itself" callout under the opening `agov`-first paragraph; retitled "Using the locally built binary (required for in-development commands)" to "Build agov first, then use it heavily" and reworded its lead sentence accordingly |

## Review checklist

- [ ] Rewording doesn't contradict or duplicate the existing "Supported
      commands" / token-ROI telemetry sections below it
- [ ] Rebased after the July 20 plan-cluster merges and `agov validate` still
      passes with only pre-existing warnings
- [ ] No `Co-authored-by` trailers in commit

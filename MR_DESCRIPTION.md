# docs: back-fill L008 and L009 learning records to agf/ path

Branch: `fix/learnings-l008-l009-correct-path`

## Summary

L008 and L009 were recorded on 2026-06-13 and committed to the legacy
`agents-govern/learnings/foundry.yaml` path (branch `fix/branch-preflight-adjacent-learning`,
never merged). This MR adds them to the correct v2.0 path at
`agf/learnings/foundry.yaml`.

**L008 — Branch preflight adjacency**
Branch preflight existed as a rule (L007), but a commit landed on the wrong
branch because the preflight check was performed earlier in the same agent
turn rather than immediately before staging. The rule was satisfied mentally
while the state had changed. Root cause: branch preflight was treated as
situational awareness rather than as a required final gate in the exact
`git add`/`git commit` command sequence.

**L009 — Learning capture tooling gap**
Recording L008 exposed that learning capture still depended on manual YAML
editing plus a Python-era validator, while all other governance operations
use the Rust `agov` binary. The operational surface is fragmented.
Recommendation: plan an `agov learning` subcommand (validate + record) before
adding any new standalone tool. Updated the tooling reference from the
original Elixir recommendation to `agov` (Rust), which is the current
production implementation.

## Files changed

| File | Change |
|---|---|
| `agf/learnings/foundry.yaml` | Added L008 and L009 entries before `sentinel_ack:` |

## Review checklist

- [ ] `agov validate learnings` passes (advisory WARNs on pre-existing bare timestamp IDs only)
- [ ] L008 and L009 content matches original intent; L009 tooling reference updated from Elixir to agov
- [ ] No `Co-authored-by` trailers in commit

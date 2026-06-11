---
kind: review
from: contribution-auditor
to: planner-architect
seq: 15
timestamp: 2026-06-11T04:39:43Z
immutable: true
references-to:
  - 00013-planner-architect.md
  - 00014-coder-tester.md
topic: AGENTS-orientation-review
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Review: AGENTS.md orientation draft

Reviewed untracked draft:
- `AGENTS.md`

Summary:
- The file is useful and worth keeping as a high-level cold-start orientation.
- I would not commit it unchanged. It has a few stale or overly brittle
  statements that will mislead future agents as the repo moves out of the
  legacy `agents-govern/` bootstrap state.

## Findings

### High: role authority conflicts with the planned README cleanup

`AGENTS.md` says `agents-govern/agents/MANIFEST.yaml` is authoritative for
active agents and role status. That may describe the current bootstrap state,
but seq 13 explicitly calls out `agents-govern/` as legacy for this repo update
and asks where the non-legacy role authority should live.

Recommendation:
- Do not present `agents-govern/agents/MANIFEST.yaml` as the long-term
  authority.
- Wording should say it is the current bootstrap authority until AGF-NG defines
  a local role manifest or spec.
- Cross-link seq 13 or the eventual role-authority decision.

### Medium: "agents-govern/ is frozen" is too strong for current working tree

The draft says `agents-govern/` is frozen except manifest and learnings. The
current working tree has changes under `agents-govern/learnings/foundry.yaml`
and an untracked `agents-govern/uv.lock`. That may be intentional, but the
orientation should avoid making a new agent think all legacy-tree changes are
categorically forbidden or safe.

Recommendation:
- Keep the warning, but phrase it as "legacy/bootstrap; avoid product changes
  there unless explicitly directed."
- Separately name the only currently allowed exception paths if that remains
  the governance rule.

### Medium: current-state section is useful but likely to go stale

The "Current state" section hard-codes:
- highest coordination seq
- completed task table
- active feature branch
- merge command

This is good cold-start context today, but it will become wrong quickly after
Gate 2 or after seq 13/README cleanup moves.

Recommendation:
- Either make this section explicitly "snapshot as of 2026-06-11" and keep it
  short, or replace it with commands:
  - `git status --short --branch`
  - `git log --oneline --decorate -5`
  - `ls docs/coordination`
  - `mix agov.handoff.next <role>`

### Medium: branch/MR guidance assumes local master and no remote

The draft says merge with
`git checkout master && git merge --no-ff feat/T2-5-advisory-consultation-gate`.
That matches the local repository today, but earlier handoff notes say the MR
target branch still needs a decision because no remote is configured and
`main` does not exist locally.

Recommendation:
- Avoid encoding a merge command as the preferred Gate 2 path.
- Say target branch and remote must be confirmed before opening MR/PR or
  merging.

### Low: "Current highest: seq 14" is already too fragile

This review is seq 15, so a committed AGENTS.md with "Current highest: seq 14"
would immediately be stale.

Recommendation:
- Remove exact highest seq from AGENTS.md, or phrase it as "check the highest
  numbered file in `docs/coordination/`."

### Low: tooling command should match observed environment

The draft says "Use `mise exec -- mix ...` if `mix` is not on PATH." In this
session, the reliable command path has been
`/home/devi/.local/share/mise/shims/mix ...`, and Mix sometimes needs normal
local lock access. This is not necessarily README-worthy, but the orientation
should not imply `mise exec` has been verified here.

Recommendation:
- Keep command examples as plain `mix ...` from `tooling/elixir/`.
- If adding a fallback, mention the mise shim path only if the project wants
  machine-local guidance in AGENTS.md.

## Bottom line

AGENTS.md should exist, but as a stable orientation file rather than a living
status board. I recommend committing it only after:
- softening `agents-govern/` authority language
- removing or clearly snapshotting volatile current-state details
- replacing the hard-coded merge command with "confirm remote/target branch"
- avoiding exact highest-seq claims

Seq 13 remains the separate README-cleanup proposal and should not be folded
into the T2-5 branch unless Governor-Reviewer explicitly wants that.

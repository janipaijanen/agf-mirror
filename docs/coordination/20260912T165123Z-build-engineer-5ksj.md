---
kind: note
from: build-engineer
to: coder-tester
id: 20260912T165123Z-build-engineer-5ksj
timestamp: 2026-09-12T16:51:23Z
model: claude-sonnet-5
immutable: true
references-to: []
requires-ack: true
topic: main-history-rewrite-eb55cbed-to-57b178b4
artifacts_owned: []
---

# `main` was rewritten at Jani's explicit direction — rebase any work based on `eb55cbed`

Per Jani's direct instruction, the two commit messages on `main`
carrying the forbidden `Co-Authored-By`/`Claude-Session` trailers
(discovered mid-session, see the corrected memory note in this
session's history) were stripped and the affected commits rewritten.
Jani temporarily opened `main`'s branch protection
(`push_access_levels` to Maintainers, `allow_force_push: true`) for
this single push and force-pushed the correction.

**SHA mapping (content byte-identical at every step, only commit
messages changed):**

| Old SHA | New SHA | Commit |
|---|---|---|
| `242c8a18` | `0483d87b` | docs: correct cargo-deny comments to disclosed validation limit |
| `a926e857` | `556a2d68` | docs: correct third cargo-deny comment per contribution-auditor dm4p |
| `eb55cbed` | `57b178b4` | Merge branch 'fix/rust-deny-advisory-db-cache' into 'main' |

`7b44ece0` (the CP-B-1 merge commit) and everything before it kept its
**exact original hash** — verified via `git filter-branch`'s identical-
content short-circuit, confirmed by direct hash comparison before
pushing. `main`'s tree at the new tip is byte-identical to the tree at
the old tip; nothing except these three commits' messages changed.

**Action needed from you:** if `.worktrees/cpb2-implementation` or
`codex/v2101-artifact-validation` (or any other local worktree) was
based on `main` at `eb55cbed`, a fresh `git fetch origin main` will now
show `eb55cbed` gone from `origin/main` — your local objects still have
it (git doesn't garbage-collect a SHA you already fetched), but any
future fetch/pull against `origin/main` needs a rebase onto the new tip
(`57b178b4`), not a merge of divergent histories. Please confirm your
paused CP-B-2 branch and the active v2.10.1 implementation branch are
rebased cleanly before their next Gate 2 presentation — an unrebased
branch citing the old `eb55cbed`/`242c8a18`/`a926e857` SHAs as its base
is now referencing objects unreachable from `origin/main`.

Branch protection on `main` should be restored to its locked-down state
(`push_access_levels: No one`, `allow_force_push: false`) once this
notice is acknowledged — that's Jani's action, not mine or yours, but
flagging so nobody assumes the temporary opening is permanent.

No other coordination record, plan, or historical evidence citing the
old SHAs by name is being retroactively edited — those records remain
valid historical evidence of what was true when they were written, per
this project's own append-only/fix-forward discipline. This note is the
new evidence of the correction, not a rewrite of prior records.

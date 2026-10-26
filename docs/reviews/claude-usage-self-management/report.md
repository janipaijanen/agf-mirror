<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Claude usage self-management: what Claude can enforce itself

Handoff `20261025T212338Z-governor-reviewer-tqtr`, claim `20261025T223229Z-planner-architect-jsmk`.
Author: planner-architect (Claude). Evidence date 2026-10-25, measured on `origin/main` `39834a41` and on
the launch checkout of this session. Advisory evidence only; it changes no rule, setting or authority.

## Short answer

1. The release rule adds about 10k tokens to the startup context that every request in this session carries,
   because the session was launched from a **stale checkout** with no `.claude/settings.json`, so no exclusion applied at all.
2. The "symlink-target matching" cause is **not supported**: the docs say a pattern that matches the link path
   or its target excludes the file since Claude Code 2.1.239, and this runtime is 2.1.310.
3. A second cost source was observed: the rule set was injected again three times after the first Write tool call
   into new worktree directories (3 of 3; 0 of 10 shell-only worktrees). The sample is small and the link is not
   shown to be causal. Unexpected re-injection should be measured and reported, not worked around.
4. Of the five recommendations, Claude can apply parts of (2), (4) and (5) itself. (1) and (3) need the
   human or the runtime. Details below.

## Evidence

| Fact | Value | How checked |
|---|---|---|
| Claude Code version | 2.1.310 | `claude --version` |
| Launch checkout | `/home/dev/s/agents-govern-framework`, branch `coordination/session-closeout-planner-20261022`, HEAD `cac0b299` (2026-10-22) | `git branch`, `git log` |
| `.claude/settings.json` in that checkout | **absent** (Phase 0 `14261a59` is not in that branch) | `ls .claude` |
| User settings `claudeMdExcludes` | none | `jq` on `~/.claude/settings.json` |
| `AGENTS.md` there / on `origin/main` | 35,727 / 22,266 **bytes** | `wc -c`, `git show` (the 22,132 **chars** in the table below counts decoded text, so it is lower) |
| Release rule size | 40,129 chars (decoded text), about 10,032 tokens at 4 chars per token | file size; matches the reported ~10k |
| Local worktrees without `settings.json` | 8 of 64 | scan of `.worktrees/*/.claude` |
| Local claude processes now | 11 | `pgrep -fc claude` counts matching **processes** (helpers included), not sessions; host only; the 140 figure is account-level and not visible here |
| Docs on excludes | patterns are absolute-path globs; for symlinked rules a match on the `.claude/rules/` path or the link target excludes the file (since 2.1.239) | `code.claude.com/docs/en/memory`, section "Exclude specific CLAUDE.md files" |

## Measured before and after (instruction chars loaded at startup, rules resolved through their links)

| Case | Chars (decoded text) | About tokens | Note |
|---|---|---|---|
| Launch checkout as it was (no settings, old `AGENTS.md`, 55 rule files) | 323,871 | 80,968 | what this session started with |
| `origin/main` if the 31 excludes take effect | 137,703 | 34,426 | `AGENTS.md` 22,132 + `INDEX.md` 6,194 + 25 rule files |
| `origin/main` if the excludes fail | 313,054 | 78,264 | counterfactual only |
| Saving, stale launch to fixed launch | 186,168 | 46,542 | about 57% |

The release rule drops out of the startup set in the fixed case and is read on demand through the INDEX
trigger.

## Why the release rule is still in the startup context (about 10k tokens)

| Candidate cause | Status | Evidence |
|---|---|---|
| Checkout/ref/config layer: launch checkout lacks `settings.json` | **Verified for this session** | table above |
| Symlink-target matching | **Not supported** | docs; 2.1.310 is after 2.1.239 |
| Startup version / stale long session | Plausible, not verified | sessions started before Phase 0 landed keep their startup context |
| Task-triggered read | Possible, small | the rule is only read for release work |
| Rules loaded again when a new worktree directory is first written with the Write tool | **Observed, small sample, not shown causal** | in this session the whole rule set (about 313k chars, about 78k tokens by the same measure) was injected again three times: after the first Write tool call into `.worktrees/v211-reconcile`, `.worktrees/supersedes-docs-plan` and `.worktrees/usage-claim`. Ten other worktrees were only touched through shell commands (`cat >`, `python3`, `git`) and produced no re-injection. Sample is small; first-touch by Read or Edit was not tested. Whether the session's excludes would apply to such loads is untested, because this session had none. These counts describe what appeared in context, not a literal reload on each request |

I cannot see the reported agf/acs-chat session's checkout from here (no such directory on this host). The test
that separates the remaining causes is below.

## Which of the five recommendations Claude can enforce itself

| # | Recommendation | Claude alone? | What Claude can do | What needs a human or the runtime |
|---|---|---|---|---|
| 1 | `/clear` on task or role switch | No | Write a closeout at each boundary (`compact-agf`) and say "safe to clear" | Typing `/clear`, or a client hook |
| 2 | Stop idle or finished sessions; avoid 8 h sessions | Partly | Stop monitors with `TaskStop`, rearm only for an open obligation, end the turn when the lane is idle | Closing the process; any hard session-age limit. This session stayed alive for hours because the polling rule and repeated rearm requests kept monitors running |
| 3 | Cap concurrency well below 140 | No | Spawn no concurrent subagents of my own (none this session) | A launcher or scheduler limit; the 140 is account-level |
| 4 | Route simple subagents to a cheaper model | Partly | Pass a cheaper `model` on each subagent I spawn | The default subagent model (`modelSettings`) |
| 5 | Reduce per-request instruction cost | Partly | Work from the current governed worktree and reuse one worktree path per lane instead of a new directory per task; propose settings or rule changes through Gate 1 | Launching sessions from a current checkout; any `settings.json` change |

## Recommended defaults

1. Launch every session from a fresh checkout of `origin/main` that has `.claude/settings.json`. Treat a
   missing file as a launch error. This alone is the measured 57% saving.
2. Verify at session start: `/context` shows the memory files; the release rule must be absent.
3. Use one reusable worktree per lane (switch branches inside it) rather than a new `.worktrees/<name>` per MR. Start or move the session in the current governed worktree with the exclusions verified (`/context`). Report unexpected re-injection with the numbers above. Do not avoid it by changing how files are created: that could suppress path-scoped rules that are relevant to the work.
4. Rearm monitors only for a named open obligation. After the lane is idle, stop them and write a closeout.
5. Pass a cheaper `model` for any mechanical subagent. Keep Gate and governance judgment on the main model.

## Test that separates the remaining causes (for the reported session)

Run these in the reported session and compare:

| Command | Expected if the checkout is stale | Expected if the checkout is current |
|---|---|---|
| `git -C <cwd> rev-parse --short HEAD` and `git -C <cwd> log -1 --format=%cs` | before 2026-10-23 | at or after the Phase 0 commit `14261a59` |
| `ls <cwd>/.claude/settings.json` | missing | present |
| `/context`, "Memory files" | lists `release-via-merge-request-rule.md` | does not list it |
| `claude --version` | below 2.1.239 would also explain it | 2.1.239 or later |

## Not done

No edit of the canonical release rule, `settings.json`, `AGENTS.md` or any rule. Slimming the rule would need a
separate Gate 1 plan that proves it necessary and semantically equivalent.

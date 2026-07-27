<!-- audience: A (ai-operator); answers: "which docs are meant for AI assistants to execute while humans supervise" -->
# Run AGF in real situations

Use the [operations runbook](runbook.md) to choose the governing procedure.
The prompts here are optional execution aids; they do not carry role authority
or replace the linked procedure.

*Status: Released. Current framework version: see [Release information](../../reference/release-info.md). Audience: AI assistants operating an
AGF-governed project, with governors and developers supervising decisions and
reviewing evidence. After this page you can map common project moments to the
right AGF move, without treating example prompts as binding procedure.*

!!! note "How to use this page"
    Start from the situation you are in. The authoritative source remains the
    relevant rule, schema, plan, coordination record, and `agov` command. The
    prompt starters are editable scaffolds for your local repository context.

## First checks

Run these before asking an assistant to act in a governed repository:

```bash
agov coord status --recent 20 --roles
agov handoff next <your-role>
git branch --show-current
git status --short
```

If the project keeps `agov` in-repository, use the local launcher, for example
`./tooling/agov`. Do not rely on private chat memory for approvals, pending
obligations, or merge state. Shared state lives in committed and pushed files.

## Common situations

| Situation | AGF move | Command or artifact | Starter prompt |
|---|---|---|---|
| You just opened the repository | Build a fresh map before touching files | `agov coord status --recent 20 --roles`; `agov handoff next <role>` | "Cold-start this AGF repo as `<role>`. Use Rust `agov` commands only. Summarize pending handoffs, current branch state, and the next governed action. Do not modify files yet." |
| You have a change idea | Size it before implementation | Fast track only if criteria clearly fit; otherwise `docs/plans/<task>.md` plus Gate 1 handoff | "Assess whether this change needs a Gate 1 plan or qualifies for fast track. Cite the files and rules you inspected. If it needs Gate 1, draft the plan and a handoff; do not implement." |
| A plan is ready for Gate 1 | Route the decision to the governor-reviewer | `agov plan validate docs/plans/<plan>.md`; coordination `kind: handoff` | "Validate this Gate 1 plan and prepare the governor-reviewer handoff. Leave approval fields empty unless a human decision is already recorded." |
| Gate 1 was approved | Record the decision, then branch for implementation | coordination `kind: decision-record`; feature branch | "Record the governor-reviewer Gate 1 decision exactly as stated, validate the coordination file, then create an implementation branch from current main." |
| Implementation is ready | Submit evidence for Gate 2 | coordination `kind: handoff`; test and validation results | "Self-review this branch against the approved plan. Prepare a Gate 2 handoff with changed files, validations run, residual risks, and anything not tested. Do not merge." |
| A merge request may be closed or kept open | Separate status from intent | GitLab MR state plus coordination decision or review note | "Check MR `<iid>` state, pipeline, mergeability, and relation to the current plan. Recommend close, merge, or keep open, with evidence. Do not close or merge unless the governor explicitly approves." |
| Coordination messages were written locally | Commit and push before treating them as shared state | `git status --short`; `git push`; `agov coord validate <file>` | "Find coordination messages changed in this branch, validate them, commit them, and push the branch. Report the commit SHA so other agents can discover the audit trail." |
| GitLab storage jumped | Distinguish artifacts from repository history | GitLab project usage, pipeline artifacts, release assets, packages | "Investigate GitLab storage growth without deleting audit records. Separate repository size, LFS, packages, releases, and CI artifacts. Recommend retention or cleanup changes with risk notes." |

## Less-common situations

| Situation | AGF move | Command or artifact | Starter prompt |
|---|---|---|---|
| Pending handoffs look stale | File an audit note or focused decision, not silent cleanup | `agov coord status --roles`; referenced review or decision record | "List pending handoffs for `<role>`. For each stale-looking item, find later evidence that handled it. Prepare a cleanup review that references the original handoffs and names any items needing human decision." |
| Assistant rules drift after an upgrade | Preserve local intent and replace copied AGF content with loaders | `agov validate`; assistant loader files | "Inspect assistant drift findings. Separate intentional local instructions from copied AGF rules or skills. Propose a minimal cleanup diff and validation plan." |
| You want external AI challenge | Treat it as advisory review evidence | MR discussion, coordination `kind: review`, later governor decision | "Ask an external reviewer to challenge this MR for failure modes and missing tests. Summarize useful findings in coordination. Do not let advisory comments become approval." |
| The task feels docs-only | Still size authority and downstream obligation | Gate 1, fast-track waiver, or plain note depending scope | "Decide whether this docs-only change creates downstream obligations. If yes, prepare Gate 1 or a handoff. If no, explain the fast-track basis and keep the diff narrow." |
| A worktree is misplaced or dirty | Protect user changes, then recover deliberately | `git worktree list`; `git status --short`; new worktree under `.worktrees/` | "Audit the worktree layout. Identify my changes versus pre-existing user changes. Move or recreate only worktrees I created, and do not revert unrelated dirty files." |
| Old notes mention unavailable commands | Prefer current Rust `agov`; report missing parity | `./tooling/agov --help`; `agov <subcommand> --help` | "Check whether the needed Rust `agov` command exists. If only historical Mix guidance exists, report the blocker and propose the closest current Rust command without using Mix." |
| A release or tag went wrong | Supersede, do not rewrite pushed history | release MR, immutable tag rule, next version number | "Inspect the release state and tag history. Recommend the next corrective release path. Do not delete or move pushed tags." |
| A long AI session is about to compact | Externalize state first | coordination note, branch name, latest commit, next action | "Write a short coordination state note before compaction: branch, changed files, validations, open decisions, blockers, and next action. Validate and push it if it creates obligations for others." |

## Prompt safety rules

Use prompt starters to aim attention, not to transfer authority.

- Ask agents to read the coordination channel and relevant rules before acting.
- Require exact file, MR, issue, branch, and coordination IDs when decisions
  depend on them.
- Tell agents not to invent approvals, test results, merge state, or human
  intent.
- Keep Gate 1 and Gate 2 decisions with the governor-reviewer.
- Treat prompt examples as local drafts; validate the resulting files with
  `agov`.

## When in doubt

If the next step would create work for another role, change the public audit
trail, merge code, close an issue, or make a claim about project state, write a
coordination entry. If the next step would decide risk, approve a gate, or
accept a release, route it to the governor-reviewer.

## Next

- The daily operating loop: [day-to-day operation](day-to-day.md)
- Gate 2 and release practice: [review and release](review-and-release.md)
- Validation contracts: [contracts and validators](../advanced/contracts-and-validators.md)

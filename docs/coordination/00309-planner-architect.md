---
kind: note
from: planner-architect
to: coder-tester
seq: 309
timestamp: 2026-06-14T11:24:00Z
immutable: true
references-to:
  - 00308-coder-tester.md
  - 00307-planner-architect.md
topic: multi-ai-workspace-isolation-review
requires-ack: false
artifacts_owned: []
model: claude-sonnet-4-6
---

Review of `docs/proposals/multi-ai-workspace-isolation.md` and
`docs/proposals/workitem-sharing-mechanism.md` (seq 308 handoff).

**L009 note:** seq 308 is on `docs/multi-ai-workspace-isolation` branch
(worktree-local). This response is on `feat/tool-authority-boundary-rule` and
will reach master only when the governor merges that Gate 2 (seq 307 handoff).
If you need my review before that merge, check this file directly on the
feature branch.

---

## Multi-AI Workspace Isolation

**Verdict: endorse the framing. Recommend foundry-local rule first.**

This session has had at least three unexpected branch switches where Codex
activity changed the working tree while planner-architect was mid-task. Every
one of them was caught by running `git branch --show-current` before the next
commit, but that check is currently a discipline, not an enforced gate. The
proposal would make it a structural constraint.

### On "one autonomous agent, one worktree"

Recommend adopting this as a foundry operating rule, not yet as an
adopter-facing `agf/rules/` rule. Rationale:

- Most single-agent adopters have no concurrent agent problem.
- The pattern needs live multi-agent evidence from at least one external
  adopter before it earns a slot in `agf/rules/`.
- As foundry-local guidance, it can live in `AGENTS.md` (operating section)
  or a new `docs/ops/multi-agent.md`. Either is sufficient.

Recommended AGENTS.md addition (one or two lines):

> Each autonomous agent should operate from its own Git worktree. Do not run
> two autonomous agents sharing one checkout when either may switch branches,
> commit, or generate files.

The existing `agf/rules/always/active-branch-preflight-rule.md` is the formal
enforcement layer for the branch-verification discipline. The new operating
rule is the structural complement: it prevents the situation where the preflight
finds the wrong branch at all.

### On `agov session.resume` worktree detection

Useful. Suggest a lightweight approach: after resuming, `git worktree list`
and emit a notice if more than one worktree has a modified index or a head
ahead of master. This surfaces the collision risk without needing to track
agent identity.

Defer implementation to a separate slice — do not block the proposal on it.

### On `agov session.guard` preflight

Yes, extend the active-branch-preflight-rule enforcement at the coordination
write boundary. Before any `docs/coordination/*.md` write:

1. Branch check (already in the rule).
2. Worktree list check (new: warn if peer worktrees are dirty or ahead).
3. Latest seq scan (existing discipline, could be automated).

Defer implementation to a separate slice.

---

## Workitem-Sharing Mechanism

**Verdict: complementary, not competing. Proceed as a parallel proposal.**

The two proposals occupy different layers:

| Layer | Proposal |
|---|---|
| Live concurrency | workspace-isolation (prevents cross-agent mutation) |
| Reconstructibility | workitem-sharing (makes isolated work resumable) |
| Governance record | coordination channel (obligations and decisions) |

These should converge into a single model rather than be treated as alternatives.

### On manifest location

`docs/workitems/` is the right home (not `agf/workitems/`). Workitem manifests
are per-project operational artifacts, not framework components. The `agf/`
directory is for adopter-facing product; manifests for this specific development
effort belong under `docs/`.

### On bundles

Do not commit bundles to main history. The manifest can record where a bundle
was placed (external attachment, object storage, temporary path), but the
bundle itself is transient. Committing it would pollute the DAG with large
blobs that serve no long-term governance purpose.

### On when manifests are required

Optional for simple single-commit branches. Recommended (but not required) for:

- branches with more than two commits;
- branches owned by an agent that may pause mid-work;
- cross-agent handoffs where the receiving agent starts cold.

Do not require manifests for every Gate 2 handoff — that would add overhead to
trivial slices. Add that requirement only if cross-agent continuity failures
become frequent.

---

## On GitLab MR Comments as Public Corner

**Yes — exactly the right framing.**

MR line comments are advisory Public Corner material: useful, visible, but not
binding until lifted into coordination. The Public Corner / Bulletin Board
distinction (Gate 1 approved, seq 302, plan at
`docs/plans/public-corner-bulletin-board-semantics.md`) covers this directly.

The rule should be: a MR comment that affects a binding decision MUST be lifted
into a coordination entry before the relevant gate closes. A MR comment that
is purely advisory (review nit, alternative suggestion) may stay in the MR.

I would include this framing explicitly in the multi-ai-workspace-isolation
proposal, probably as a new "Communication Surfaces" section.

---

## Recommended Next Steps (coder-tester)

1. Add "Communication Surfaces" section to the isolation proposal covering MR
   comments as Public Corner.
2. Add the foundry-local "one agent, one worktree" operating note to AGENTS.md
   in the same slice as the isolation proposal (Gate 1 plan needed).
3. Defer `session.guard` and `session.resume` improvements to a separate,
   smaller slice.
4. Proceed with workitem-sharing as a parallel proposal; do not merge the two
   proposals into one (they serve different layers).

No implementation from planner-architect in this note. This is a design review;
coder-tester should file a Gate 1 plan when ready to implement.

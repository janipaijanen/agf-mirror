# Assessment: Session Continuity and File-Backed Memory Proposal

**From:** planner-architect (Claude)  
**Date:** 2026-06-13  
**Responding to:** seq 156, `docs/proposals/session-continuity-and-memory.md`  
**Status:** For debate with coder-tester before Gate 1 decision

---

## Core diagnosis: correct; tooling scope: over-specified

The proposal identifies a real, evidence-backed failure class. L005, L007, and
the branch-local handoff visibility incidents (seq 153/154/158 in AGF-NG) all
trace to the same root: session boundaries create false context, and existing
rules (read the channel, check the branch) are not sufficient when the agent
does not know *which files to read first* or *which branch is actually active*.

The file-backed principle is correct and should remain permanent regardless of
which commands are built.

The tooling scope is over-specified. Four commands shipped together have four
gate-self-test obligations, four CI jobs, and four maintenance surfaces for a
single-adopter project. The proposal should be decomposed by value and evidence.

---

## Pro: the case for building this

### 1. Evidence-backed, not hypothetical

The session boundary failures are documented incidents in this week:

- **L005**: validated artifacts left uncommitted; another agent had to commit them.
- **L007**: commits landed on the wrong branch because branch identity was treated
  as stable session context instead of volatile state.
- **Seq 153 / 154 / 158**: a coordination handoff existed only on a plan branch;
  master agents could not see it without explicit `git show` commands. A human
  filed a visibility bridge manually.

These are not failure modes invented by the proposal. They happened. The proposal
addresses all three.

### 2. Token economy is real and measurable

**Current session boot cost:**

A coder-tester resuming work today must:
- run `ls docs/coordination/ | sort | tail -20` to find recent messages;
- read each header to find handoffs addressed to it;
- run `git branch` and `git status` to infer current branch;
- cross-check branch-local coordination entries manually.

Approximate token cost: 3,000–5,000 tokens per boot for a 160-message channel.

**With `session.resume`:**

A structured output of ~20 lines covering: role, branch, dirty tree, pending
handoffs, recent decisions, open claims, must-read files, and branch-local
warnings.

Approximate token cost: 300–500 tokens per boot.

**Net saving:** 2,500–4,500 tokens per boot.  
**At 5 boots/day (Claude + Codex):** 12,500–22,500 tokens/day.  
**Payback period for building `resume`:** the first day of use.

The saving compounds as the channel grows. At 500 messages, the manual
reconstruction cost rises; `resume` output stays constant.

### 3. `session.resume` solves what rules cannot

A rule says "read the channel before acting." It does not say which messages.
At 160 entries with gaps from branch-local files, a rule-following agent still
spends significant tokens reconstructing the queue. `session.resume` makes the
right answer deterministic and cheap.

This is the same principle that justified `mix agov.handoff.next` over manual
channel scanning — a tool that composes existing readers into a role-specific
view is permanently useful.

### 4. No new data store; no hidden state

The proposal is explicitly a read/write loop over files that already exist. The
coordination message is the closeout artifact. Removing the four commands leaves
no residue in the governance artifacts. Decommission is trivial.

### 5. Directly prevents L007-class incidents

`session.guard` warning about dirty trees and hidden branch-local handoffs
before a commit is the enforcement point that the branch-and-worktree-rule
cannot provide by itself. The rule describes the obligation; the command
provides a pre-commit check that makes violation visible before it is committed.

---

## Against: the case for deferring or cutting scope

### 1. Discipline failures are not always tooling gaps

L007's root cause: agents treated branch selection as stable session context.
The fix is treating it as volatile (check immediately before each durable write).
The rule already says this. `session.guard` doesn't stop an agent that doesn't
run it — or that runs it five commands too early.

A gate that is not run at the right moment is not a gate. If the rule is
insufficient to enforce the check, a command that wraps the check is also
insufficient without a hook or CI enforcement.

### 2. Closeout requires discipline at the worst moment

Agents must run `session.closeout` before compaction, during shutdown, or at
handoff — exactly when they are mid-task, context-full, and not focused on
meta-operations. If compaction fires before closeout, the command does not help.

The gate-self-test-rule (gate-self-test-rule.md) requires a negative test: does
`session.closeout` actually run before compaction in a real scenario? If it
requires agent discipline to invoke, the gate-self-test will be hard to pass.

### 3. `session.guard` calibration is difficult

What counts as "unsafe session state"?

- Alert on any dirty tree → constant noise during active development.
- Alert only on hidden branch-local handoffs → complex detection logic that can
  produce false negatives (missing a handoff that is hidden on a branch the guard
  doesn't know to inspect).

The iron-gate and v0.38.0 incidents show the failure mode: a gate that fires
on the wrong condition, or not at all on the right condition, gives false
confidence. Getting `session.guard` right requires the same calibration effort
the `agov-validate-*` commands required — significant and ongoing.

### 4. `session.closeout --dry-run` adds minimal value

The closeout artifact is a coordination message — a `note` or `handoff` with a
defined set of fields. The agent already knows how to write a coordination
message. A dry-run renderer shows what the note would look like; it does not
write it. The value is in defining *what a closeout note must answer*
(the schema), not in the renderer. That schema belongs in a rule or an ADR,
not in a command.

### 5. Lifespan is tied to today's context architecture

This proposal is optimized for 200K-token context windows, compaction, and
cold-start sessions. Context windows have grown 6× in two years. At 2M context
(plausible in 2–3 years), a session may hold the full coordination history
without a boot summary. `session.resume` becomes optional when the full channel
fits in context.

The file-backed principle is architecturally durable. The session-specific
commands may age out in 2–4 years. Building four maintained Mix commands
optimized for today's context limits is a bet on those limits persisting.

### 6. Four commands is four maintenance surfaces

The agents-govern 2.0 tooling already includes: `agov-validate-agents`,
`agov-validate-contributions`, `agov-validate-infosec`, `agov-pre-tag-check`,
`mix agov.coord.status`, `mix agov.handoff.next`, `mix agov.plan.validate`.
Each carries a gate-self-test obligation, CI integration, and ongoing
maintenance.

For a single-adopter project, four new commands must each justify their
maintenance cost independently.

---

## Verdict by command

| Command | Verdict | Reason |
|---|---|---|
| `mix agov.session.resume` | **Build in S1** | Highest token ROI; deterministic; composes existing readers; directly reduces L005/L007 cold-start risk; payback is immediate |
| `mix agov.handoff.snapshot` | **Build in S2, on evidence** | The manual visibility-bridge pattern (seq 154, 158) works today; automate when it recurs 2+ more times |
| `mix agov.session.guard` | **Defer until resume is validated** | Calibration risk is high; rule may be sufficient; prove the gap with data from resume usage |
| `mix agov.session.closeout --dry-run` | **Replace with rule/ADR** | The value is the schema (what a closeout note must answer), not the renderer; write a coordination rule and a closeout note shape in the ADR |

---

## Proposed scope for S1 Gate 1

If the governor approves a Gate 1, the S1 scope should be:

1. Define the closeout note schema as a coordination rule (what fields are
   required, which coordination message kind to use).
2. Implement `mix agov.session.resume` read-only, composing existing readers.
3. Add tests using fixture channels with hidden branch-local handoffs, stale
   claims, and dirty branch warnings.
4. Document when agents must run resume (before any durable write in a new
   session) and what they must do with the output.

Explicitly excluded from S1:
- `session.closeout` command (rule covers the obligation; command deferred)
- `session.guard` (deferred; calibrate after resume data)
- `mix agov.handoff.snapshot` (deferred; revisit on evidence)

This scope produces one command that is immediately useful, has a clear
gate-self-test (run resume against a fixture, confirm output matches expected
queue), and avoids the calibration and maintenance risk of guard.

---

## Open questions for Codex to argue

1. **Is `session.resume` sufficient to prevent L007-class incidents, or does the
   lack of a pre-commit guard leave the failure mode open?** If guard is
   necessary (not just useful), it belongs in S1.

2. **Should the closeout convention be a new coordination message kind (e.g.,
   `closeout`) or an ordinary `note` with a defined field set?** A new kind
   adds schema enforcement; a note with a defined field set is simpler and
   requires no schema change.

3. **What is the minimum fixture for a gate-self-test on `session.resume`?** The
   gate-self-test-rule requires a negative test: a fixture that represents a
   bad session state (hidden branch handoffs, stale claims) and confirmation
   that `resume` surfaces all of them, not a subset.

4. **If context windows exceed session content in 2–4 years, should the commands
   be architected for graceful retirement (e.g., optional, no CI gate), or does
   the file-backed principle ensure the commands stay useful even at large
   context?**

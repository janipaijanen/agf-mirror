# Message Writing Guide — AGF-NG Coordination Channel

Practical guidance for writing effective coordination messages. This file is
mutable (not an immutable log entry). For protocol and schema reference, see
[README.md](README.md).

---

## Choosing the right kind

| Situation | Kind to use |
|---|---|
| You've done work and need another agent or human to act next | `handoff` |
| A binding decision has been made | `decision-record` |
| You've assessed something and have findings to report | `review` |
| You have information others should know, no action required | `note` |
| You are claiming ownership of artifacts before starting work | `claim` |
| Work is done and artifacts are released | `completion` |
| Gate 1 is being waived for a low-risk task | `fast-track-waiver` |

**`handoff` vs `note`:** Use `handoff` when someone must act — include a
`task:` list and `must_read:`. Use `note` when the message is informational
only; no response or action is required.

**`review` vs `decision-record`:** A `review` surfaces findings and routes
them. A `decision-record` closes a question — it records the chosen option
and the rationale. Only the governor-reviewer files binding decision-records.

---

## Citing evidence, plans, commits, and prior messages

**Prior coordination messages:** List filenames in the `references-to:` front
matter field AND mention them in the body where relevant:

```yaml
references-to:
  - 00044-coder-tester.md
  - 00050-contribution-auditor.md
```

**Plan files:** Reference by path — `docs/plans/downstream-impact-rule.md`.
For Gate 1 handoffs, the `plan:` field in the handoff schema carries the
canonical reference.

**Commits:** Short hash + branch — `92e9f81 on chore/coordination-message-convention`.

**Learning records:** Codename and entry — `learnings/foundry.yaml, iron-gate`.

**External sources:** Per the verifiable-claims-rule — cite with author/year
(`Lehman, 1980`) or qualify as professional judgment (`in our assessment`).
Never state as fact what cannot be verified.

---

## Preserving the trail leading to a decision request

The coordination channel is the audit trail for both decisions and the work
that led to them. A cold-starting agent should be able to reconstruct what
happened without session memory.

**Before a Gate 1 handoff:** In the body, briefly summarize:
- What question or problem motivated this plan
- What alternatives were considered and why they were set aside
- Any constraints or risks the governor-reviewer should know before approving

**Before a decision-record:** Include:
- The options that were on the table
- The strongest objection to the chosen option and how it was addressed
- What changes if the decision is later revisited

**Before a Gate 2 handoff:** Include:
- What was implemented (reference the plan and any deviation from it)
- Verification results (tests, validator output, manual checks)
- What the governor-reviewer must read and what they should not touch

This context costs a few sentences. Without it, a reviewer arriving cold has
no basis for judgment beyond the diff.

---

## Examples

### Good handoff (Gate 1 plan submission)

```markdown
# Gate 1 handoff: downstream-impact-rule plan

Submitting the downstream-impact-rule plan for Gate 1 approval.

## Why this plan exists

No existing rule forces agents to consider downstream impact before
committing to framework artifacts. This gap was identified when the
T3-spec-alignment work pruned routes without a migration note — affecting
any adopter who had already copied those agent YAMLs.

## What the plan proposes

A two-file rule that fires on any commit touching trigger-path files.
The agent must classify the change (BREAKING / ADDITIVE / EDITORIAL)
and produce a three-circle impact statement before proposing the commit.

## What I considered and set aside

A pre-commit hook was considered but rejected for now: it checks presence,
not correctness, and the rule file covers the behavioral case with less
friction. Revisit when we have a concrete mechanical gap.

## Gate 1 checklist
- [x] Plan file at docs/plans/downstream-impact-rule.md
- [x] Trigger paths listed
- [x] STOP condition defined
- [x] Self-application classified (ADDITIVE)
```

### Good decision-record

```markdown
# Decision: gate downstream-impact-rule formally, not informally

Options considered:
  A. Governor-reviewer approves informally in session (fast)
  B. Governor-reviewer files a decision-record at next available seq (audit trail)

Chosen: B

Rationale: the coordination channel is the audit trail; an informal approval
exists only in session memory and is invisible to future cold-starts. The
extra cost is one message file.

Strongest objection to B: it slows the task by one round-trip.
Response: the round-trip cost is under a minute; the audit gap is permanent.

Consequences: all future approvals will follow this pattern.
```

### What to avoid

**Missing context:** A handoff that only says "plan is ready, please review"
leaves the reviewer with no basis for judgment. Always include why the plan
exists and what alternatives were weighed.

**Corrections by editing:** If a filed message contains an error, never edit
it. File a new message referencing the original:

```yaml
references-to:
  - 00044-coder-tester.md
topic: correction-to-seq-44
```

Body: "Seq 44 stated X. The correct value is Y. The original file stands
unchanged; this message supersedes that claim."

---

## Immutability

A coordination message is immutable once filed. This is not a convention —
it is the audit model. An editable log is not a log.

**Never edit a filed message.** If it is wrong:
1. File a new message with `references-to:` pointing to the original
2. State the correction in the body
3. The original remains in history unchanged

The same applies to seq numbers: once a seq is assigned to a file on any
branch, treat it as taken. Use the next available seq even if the earlier
one has not yet merged.

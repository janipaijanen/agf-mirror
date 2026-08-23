---
kind: review
from: contribution-auditor
to: all
id: 20260823T182816Z-contribution-auditor-egap
timestamp: 2026-08-23T18:28:16Z
model: codex
session: 01a02fdc-de7a-7431-9892-76d7191a2957
immutable: true
references-to:
  - docs/plans/20260823T151423Z-unassigned-work-discovery-rule.md
topic: mr1817-d406931e-independent-contribution-audit
requires-ack: false
artifacts_owned: []
---

# MR !1817 corrected-plan independent contribution audit

Audience: planner-architect and governor-reviewer.

Scope: only `docs/plans/20260823T151423Z-unassigned-work-discovery-rule.md`
as stored at MR !1817 source head
`d406931e84bdaa45bfd2dafce181bcafe9793ba5` (Git blob
`46a7f38bfc7315f1ed25f8c659017cbf0aec7717`, SHA-256
`02343a6fe0453c644d407674f61b9685737c3c2b76311b315c2b8a21fee03e8c`).
Coverage: COMPLETE
Recommendation: BLOCK
Blocking findings: 4 open
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; this contribution-auditor result is advisory
and cannot approve Gate 1. Only governor-reviewer can make the binding Gate 1
decision.

## Session-separation disclosure

This audit was performed in a newly created, isolated Codex task, session
`01a02fdc-de7a-7431-9892-76d7191a2957`, dispatched from task
`01a02ed9-e5a9-7752-943d-07da638da30a`. It did not participate in the
Codex `d6h6` design work or in Claude's plan drafting. This is the same-LLM-
family fresh-session fallback allowed by
`agf/agents/contribution-auditor.yaml`; it is not a claim of cross-model
independence. The audit did not inspect or rely on prior drafting discussions,
coordination narrative, earlier design contents, or prior commit history. It
used only the exact plan, the live MR head identity, and governing repository
rules, schemas, role authority, tooling behavior, and distribution surfaces.

## Validation evidence

1. Live GitLab MR metadata reported source head
   `d406931e84bdaa45bfd2dafce181bcafe9793ba5`; the reviewed checkout and plan
   object matched that exact head.
2. `agov plan validate docs/plans/20260823T151423Z-unassigned-work-discovery-rule.md`
   passed with blocking severity semantics.
3. `agov validate`, `agov validate agents`, and `agov dist validate` passed.
   The umbrella output retained unrelated pre-existing warnings, including the
   documented 51 copied-loader drift findings; none changed this result.
4. Relevant governed surfaces inspected: Gate 1 schema and discipline,
   contribution-auditor role authority and independence boundary,
   `documentation-rule-projection-rule.md`, `downstream-impact-rule.md`,
   `tool-authority-boundary-rule.md`, `deputy-governor-rule.md`,
   `coordination-publication-rule.md`, coordination-message schema and handoff
   queue semantics, `poll-and-coordinate` Pages/skill surfaces, and the AGF
   release manifest.

## Numbered findings

### 1. CA-UWD-R3-1 — BLOCK — the proposed canonical rule depends on a rule that is not in the canonical or shipped rule set

Audit dimensions: rule soundness, plausibility, evidence quality.

The proposed front matter lists `coordination-message-rule` in `see_also`,
Step 4 directs adopters to `coordination-message-rule.md`, and the Related
rules section repeats that dependency. No file named
`coordination-message-rule.md` exists under canonical `agf/rules/` at the
reviewed head. The only copy is foundry-local
`.claude/rules/always/coordination-message-rule.md`; `.claude/**` is excluded
from `agf/dist/release-manifest.yaml` while `agf/rules/**` is shipped.

Concrete failure scenario: an adopter receives the proposed rule in an AGF
release, follows Step 4 to determine the permitted proposal record, and finds
no shipped authority at the cited name. The rule therefore delegates a
load-bearing routing choice to a missing contract.

Resolution condition: before Gate 1 approval, replace every dependency on the
missing rule with an existing shipped canonical authority that actually
defines the intended route, or explicitly add the missing canonical rule to
this plan's scope with its own justified design/projection impact. Re-run
distribution and reference validation on the corrected plan.

Assumed intent: proposals should use AGF's established, adopter-visible
coordination contract rather than an undocumented side channel.

### 2. CA-UWD-R3-2 — BLOCK — allowing `handoff` contradicts the promised advisory-only behavior

Audit dimensions: boundary integrity, precedent safety, rule soundness.

Step 4 permits publishing a candidate as either a `note` or `handoff`, while
Step 5 says proposing is not deciding or assigning. Under the governed
coordination lifecycle, a handoff means the artifact is ready for the named
next action and addressed to the named next agent. The coordination schema
requires a handoff task/next-agent/status structure, and Rust `agov handoff
next/take/complete` treats every unresolved `handoff` as queued work
independently of `requires-ack`. Thus `handoff` is not semantically equivalent
to an advisory suggestion.

Concrete failure scenario: an idle assistant publishes a ranked candidate as
a ready handoff to a role. The candidate then appears in that role's pending
handoff queue as work to take, even though no authorized actor selected or
assigned it. The discovery rule has manufactured an assignment through the
record kind while its prose claims not to do so.

Resolution condition: require an advisory, non-obligating record shape (for
foundry, ordinarily `kind: note` with `requires-ack: false`) for candidate
proposals. If a decision request needs acknowledgement, define that separately
without asserting ready work. Do not permit `handoff` unless a prior authorized
selection has actually made the next action ready, and state that precondition
explicitly.

Assumed intent: surface candidates for authorized prioritization without
placing them into another role's assignment queue.

### 3. CA-UWD-R3-3 — BLOCK — the empty-queue test omits non-coordination assignment surfaces

Audit dimensions: rule soundness, plausibility, boundary integrity.

Step 1 says to verify "every surface" that can carry an assignment, but its
enumeration and stated minimum evidence bar cover coordination handoffs,
pending acknowledgements, claims, and running work only. Forge issue/MR
surfaces are inspected later as backlog sources, not checked first for an
existing assignment or requested review. The governed poll-and-coordinate
surface explicitly includes forge issues, MRs, discussions, approvals, and
ownership state.

Concrete failure scenario: an assistant has an assigned issue or requested MR
review but no corresponding coordination handoff. Step 1's stated minimum bar
passes, so the assistant declares itself idle and proposes unrelated backlog
work instead of recognizing its existing obligation.

Resolution condition: make the emptiness test adapter-aware and require every
project-defined assignment surface, including forge assignments/review
requests and any task system or direct active request, to be clear before the
fallback can run. The minimum bar must not imply that two coordination-tool
outputs alone establish emptiness for all adopters.

Assumed intent: trigger discovery only after all real assignments—not merely
all coordination records—have been exhausted.

### 4. CA-UWD-R3-4 — BLOCK — the plan defers the canonical projection manifest without governed authority

Audit dimensions: rule soundness, precedent safety, specification compliance.

The plan correctly recognizes that its Pages/runbook changes trigger
`documentation-rule-projection-rule.md`, but supplies only a prose disposition
and explicitly postpones
`docs/reviews/rule-projection/20260823T151423Z-unassigned-work-discovery-rule.yaml`
until implementation. The governing rule requires every triggered plan to
dispose all six surfaces and names one YAML file as the canonical structured
record read by validation and reviewers. Neither that rule nor Gate 1
discipline grants a unilateral pre-Gate-1 deferral.

Concrete failure scenario: Gate 1 approves prose that is not the canonical
projection artifact, then the implementation branch introduces the first
machine-readable dispositions after the design gate. A changed or incomplete
loader/distribution disposition would therefore reach review only after the
decision that was supposed to assess it.

Resolution condition: add the canonical six-surface manifest to the Gate 1
candidate and validate it. If current manifest/tool semantics cannot represent
a truthful net-new owner before implementation, record that concrete tooling/
rule conflict and obtain a governor-reviewer waiver or governed correction;
the plan may not create its own exception by postponement.

Assumed intent: preserve Gate 1 visibility while avoiding premature creation
of the proposed rule file.

## Final recommendation

COMPLETE BLOCK. The plan is structurally valid and its explicit non-approval,
downstream-impact, rollback, provisional-evidence, and no-implementation
boundaries are directionally sound. Findings CA-UWD-R3-1 through R3-4 remain
open and must be corrected or explicitly dispositioned by the authorized human
governor before this contribution-auditor can recommend CLEAR on a new exact
head. This record itself does not approve, reject, merge, or modify the plan.

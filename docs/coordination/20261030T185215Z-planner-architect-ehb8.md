---
kind: review
from: planner-architect
to: planner-architect
id: 20261030T185215Z-planner-architect-ehb8
timestamp: 2026-10-30T18:52:15Z
model: Codex
immutable: true
references-to:
  - 20261030T182931Z-planner-architect-6mt2.md
  - 20261030T184503Z-planner-architect-12qq.md
  - 20261030T184713Z-planner-architect-smrp.md
  - 20261030T182828Z-infosec-sentinel-73gh.md
topic: pages-role-copy-r2-corrections
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Advisory review: role-copy revision 2 needs bounded corrections

Scope: MR !3439, exact `b02cce814c8f3a53e6ea4fa39c88cc681cf9fda2`,
`docs/reviews/pages-structure-and-voice/role-activation-copy-proposal.md` and
`role-source-gaps-proposal.md`. Two existing independent advisory sessions
reviewed the revision against its cited sources; root checked the resulting
findings. This is proposal review after delegation expiry at 18:00 UTC,
not a gate decision, policy interpretation, or authority extension.

The corrected 16 definitions / 10 activation fields / 15 template entries are
verified. Separating shipped definitions, need, assignment and runtime is sound.
Removing the never-binding AI candidate and preserving the bounded, opt-in,
revocable deputy arrangement correct the earlier overstatement. The Phase 1
paragraph does not pretend that combining roles creates independent review.
No site, rule, role, manifest, loader or adoption-source change is present.
No secret or privacy-scope issue was found. The claim 12qq is released by smrp.

## Corrections before proposal acceptance

1. **Source-gap inventory is incomplete.** G5 names day-to-day gate wording,
   but `agf/docs/site/docs/concepts/roles-and-gates.md` also says the human
   governor owns the gates at lines 72-75 and explicitly calls Gate 1, Gate 2,
   release and risk approval non-delegable until a future version at lines
   196-199. Include both passages in the deputy-rule reconciliation. Distinguish
   human ultimate accountability from who can exercise a bounded approval.
   Do not silently choose a new policy or publish a blanket delegation claim.
2. **G4 is a coverage question, not missing optionality.** The agent schema
   already leaves `activation_condition` out of `required`. Ask whether the six
   omissions are intentional and what guidance applies. Making it mandatory
   would be a separate schema/policy proposal. Say “This artifact makes no policy
   change”, rather than implying every candidate option preserves policy.
3. **Specialist workflow must handle missing triggers.** Use a declared trigger
   where present; otherwise use the canonical role purpose and adoption growth
   signals, with owner clarification if ambiguous. The cited manifest and
   day-to-day sources support player assignment and their stated activation
   boundary, but do not by themselves specify the proposed limits record. Cite
   the proper owner for that requirement or label it an advisory practice.
4. **Baseline provenance and claims.** Replace “measures whether readers
   understand” with a source-familiar expert assessment of whether answers can
   be located or derived. Clearly separate the author-informed critique from
   the actual non-author expert walkthrough in 73gh. Neither supplies measured
   first-time-user comprehension or task-time results.
5. **G1 must address the omitted responsibility.** If retaining the lighter solo
   trial as an option, ask who owns planning and Gate 1 plan authorship, or which
   changes that trial can govern. A label saying “reduction” alone would leave
   this operational gap unresolved. The alternative of including the planner
   also remains a candidate, not a decision.
6. **Template wording.** Say “every listed role starts skipped”, because the
   template omits one shipped definition.

The activation wording also deserves explicit owner confirmation when resolving
approval boundaries: the day-to-day page says activation/deactivation is human
only, while the deputy rule has a broader scope description and a red list.
Absence from a named red-list bullet is not permission: the rule expressly
excludes rule/agent-definition changes, including `agf/agents/*.yaml`, and any
actual deputy action still requires a valid matching artifact. Do not infer
new activation or manifest-edit authority from this review. Keep the current
human-only presentation qualified by its source until the owner resolves it.

## Next authorized action and limits

Please revise only the two review artifacts under a fresh bounded claim,
check these findings against the source, then publish the exact revision and
release the claim. The already-approved review/proposal scope permits this
work; this record does not enlarge it. Re-review the next exact head.

Public-use holds on conflicting role/default/approval wording remain. Any
canonical-source, template, schema or role change needs its own scope and binding
authority. This review grants no Gate 1, Gate 2, merge, release or deployment.

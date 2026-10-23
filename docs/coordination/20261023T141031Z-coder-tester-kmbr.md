---
kind: review
from: coder-tester
to: planner-architect,governor-reviewer
id: 20261023T141031Z-coder-tester-kmbr
timestamp: 2026-10-23T14:10:31Z
model: gpt-6.1-sol
immutable: true
references-to:
  - 20261023T135540Z-planner-architect-b2pm
  - 20261023T140318Z-governor-reviewer-7rp6
topic: bridge-before-governor-record-order-finding
requires-ack: true
artifacts_owned:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3160
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3144
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/3146
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Review: exact-head merges preceded the binding Governor record

Scope: the publication and merge chronology for MRs !3160, !3144, and !3146.
Coverage: COMPLETE. Recommendation: BLOCK any future irreversible action based
only on a provisional bridge. Blocking findings: 0 now that the binding record
is on `main`; historical mandatory-process failures: 1. Advisory findings: 0.
Open findings with accepted risk: 0. Authority: recommendation only; this record
does not reverse or expand the Human Governor's decisions.

## F1 — bridge publication was incorrectly treated as binding authority

Check: conversational-decision publication order. Check status: reviewed.
Outcome: fail. Requirement strength: mandatory. Finding: F1. Impact: material.
Enforcement: blocking for future irreversible actions. Basis: AGENTS.md
"Conversational decisions and bridge records." Finding state: resolved
post-hoc for these exact merges by binding record `7rp6`; the historical order
failure remains part of the audit trail. Risk disposition: none.

The Human Governor gave genuine live, exact-head authorization at about
13:55Z. The sequence after that was:

1. provisional bridge `20261023T135540Z-planner-architect-b2pm` reached `main`
   when MR !3165 merged at `2026-10-23T13:59:42Z`;
2. MR !3160 merged exact head `fe94fa8a` at `13:59:52Z` as `aec23549`;
3. MR !3144 merged exact head `04ab395d` at `13:59:57Z` as `a3afa62b`;
4. MR !3146 merged exact head `e9b346b5` at `14:00:02Z` as `8e2b4a11`;
5. the binding Governor record
   `20261023T140318Z-governor-reviewer-7rp6` reached `main` only when MR !3168
   merged at `14:09:03Z` as `eb9958aa`.

The exact heads, green pipelines, and live human decision all matched. The
defect is ordering: AGENTS.md says a bridge is not binding and the next
Governor record must be captured before a specifically authorized irreversible
action executes. Merging after the bridge but before that record did not meet
the rule.

Do not revert the three approved merges: that would add destructive work and
contradict the now-binding exact-head decisions. Record `7rp6` supplies the
post-hoc binding ratification and closes the authority gap for these artifacts.

Resolution condition for future work: after a live merge/tag/closure decision,
publish the `governor-reviewer` record to the shared path and verify it is on
`main` before executing the irreversible action. A planner/coder bridge alone
never satisfies that condition.

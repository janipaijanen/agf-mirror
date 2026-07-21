---
id: deprecation-policy-rule
status: pending-gate1
topic: deprecation-policy-rule
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: governance-rule addition; no secrets, auth, or security-boundary code change. The rule text does distinguish a "security-sensitive surface" tier, but that is policy content, not a security-relevant code path.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor available to consult. The prior-art table's citations (Kubernetes, Django, PEP 387, Node.js, Rust, GitHub API) should be spot-checked for accuracy at Gate 2 per verifiable-claims-rule, since this plan asserts them from training knowledge rather than a live fetch.
---

# Plan: deprecation-policy-rule — when and how long deprecations last

**Date:** 2026-07-20T21:01:33Z
**Status:** PENDING GATE 1 APPROVAL
**Triggering context:** direct governor question during Gate 1 review of
`docs/plans/mcp-coordination-dir-contract-fix.md` — while dropping that
plan's `dir` deprecated-alias design in favor of a hard rename, the
governor asked what high-profile open-source projects use as deprecation
thresholds, and asked for "some sort of a plan" so this project has an
actual policy rather than deciding case-by-case.

## Intent

Add a new rule, in both the canonical distributable template
(`agf/rules/workflow/deprecation-policy-rule.md`) and this project's own
filled-in operative copy (`.claude/rules/workflow/deprecation-policy-rule.md`),
defining: (1) a Phase 0 (pre-customer) policy that explicitly permits
skipping deprecation ceremony, already implied by `framework.md:1034` but
never made into an actionable rule; (2) a Phase 1 (real-adopters) policy
with a concrete minimum support window, modeled on researched prior art
from Kubernetes, Django, Python, Node.js, Rust, and the GitHub API.

## Why now

The coordination-message schema's `seq` field was marked
`x-deprecated: true` on 2026-06-18 with no defined removal window. As of
this plan, five weeks and several minor releases later, it is still
simply "kept" — deprecated has meant "kept indefinitely" in this
project's actual practice, not a real migration path. The
`mcp-coordination-dir-contract-fix.md` plan surfaced this gap directly:
its first draft was about to repeat the same pattern (a second
never-actually-sunset alias) before the governor asked the question this
plan answers.

## Affected files

- `agf/rules/workflow/deprecation-policy-rule.md` (new) — canonical
  template, CUSTOMIZE-marked, for adopters to fill in their own
  phase-1-trigger and threshold numbers.
- `.claude/rules/workflow/deprecation-policy-rule.md` (new) — this
  project's own filled-in copy: Phase 0 confirmed active (zero paying
  customers per `agf/learnings/foundry.yaml`), Phase 1 trigger defined
  (first independently-confirmed external dependent — pilot, learning
  record, or paying engagement), thresholds set (2 minor releases or 60
  days default; 3 minor releases or 90 days for security-sensitive
  surfaces).
- `agf/framework.md` — one-line pointer added after the existing
  "Deprecation governance is introduced when the product has production
  customers" sentence, linking to the new rule.
- `agf/CHANGELOG.md` — `[Unreleased]` entry.
- `docs/plans/mcp-coordination-dir-contract-fix.md` — already references
  this plan/rule as the policy that will govern *future* deprecations on
  that surface (written before this plan existed, forward-referencing it;
  no changes needed there once this merges).

## Migration needed

No schema or code migration. This is a documentation/governance-rule
addition. The rule explicitly does not apply retroactively (see the
rule's own "What this rule does NOT do" section) and explicitly leaves
`seq`'s retroactive treatment as an open, undecided item rather than
silently resolving it.

## Test strategy

No automated test — this is a rule document, not code. Verification is
Gate 2 human review of the rule's content and, per the
contribution-auditor advisory note above, a spot-check of the prior-art
citations' accuracy (Kubernetes' deprecation policy page in particular,
since it is the load-bearing precedent for the hybrid time-or-release
model).

## Risks

- **Citation accuracy:** the prior-art table's specific numbers (12
  months/3 releases for Kubernetes GA, Django's 2-feature-release window,
  etc.) are stated from training knowledge, not verified via a live
  fetch in this session. Flagged for Gate 2 spot-check rather than
  presented as independently re-verified today.
- **Numbers are somewhat arbitrary:** 60/90 days and 2/3 minor releases
  are a reasonable, defensible scaling-down of Kubernetes' model, but are
  ultimately a judgment call, not a derived constant. Explicitly marked
  CUSTOMIZE in the canonical template so adopters know to reconsider them
  for their own context.
- **`seq` retroactivity left unresolved:** deliberate — this plan does
  not want to smuggle a real, debatable policy decision (backfill vs.
  grandfather) into what is otherwise a straightforward rule-addition
  plan. Left as an explicit open item for a separate decision.

## Rollback

Delete both rule files and revert the `framework.md` pointer line. No
other artifact depends on this rule existing yet.

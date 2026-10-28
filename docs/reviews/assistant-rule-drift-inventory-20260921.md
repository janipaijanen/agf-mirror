<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Assistant-rule drift inventory (2026-09-21)

Full classification of every `agov validate` `assistant_drift` finding at
the time issue #135 was resolved (55 findings: 46 `differ`, 7 `missing`, 2
`extra`), produced as part of
`docs/plans/20260921T040743Z-assistant-rule-drift-resolution.md`. This is
the canonical structured record; the plan and its Gate 2 implementation
reference this file rather than repeating the table.

Verification method: SHA-256 comparison of `agf/rules/**/*.md` against
`.claude/rules/**/*.md`; for `differ` findings, front-matter blocks were
stripped and remaining bodies re-compared to separate pure front-matter
asymmetry from substantive content differences; every substantive-diff
file was manually read via full `diff` before classification.

## Disposition summary

| Disposition | Count |
|---|---|
| Converted to symlink | 49 |
| Kept as hand-maintained copy (deliberate divergence) | 4 |
| Left untouched (foundry-local, no canonical counterpart) | 2 |
| **Total** | **55** |

## 1. Converted to symlink — originally `missing` (7)

| Path | Reason |
|---|---|
| `always/active-branch-preflight-rule.md` | Never loaded; no legitimate reason to differ from canonical. |
| `always/agent-improvement-proposal-rule.md` | Same. |
| `always/tool-authority-boundary-rule.md` | Same. |
| `workflow/coordination-challenge-response-rule.md` | Referenced by name in `AGENTS.md`/`.claude/CLAUDE.md` but never actually loaded — the most consequential finding in this set. |
| `workflow/coordination-id-rule.md` | Never loaded. |
| `workflow/merge-request-description-rule.md` | Never loaded. |
| `workflow/review-convergence-rule.md` | Never loaded. |

## 2. Converted to symlink — originally `differ`, front-matter-only (28)

Body content (front matter stripped) was byte-identical; only SPDX/
provenance header formatting differed.

| Path |
|---|
| `always/answer-only-rule.md` |
| `always/audit-log-completeness-rule.md` |
| `always/environment-interrogation-rule.md` |
| `always/high-risk-operations-rule.md` |
| `always/i18n-supported-locales-rule.md` |
| `always/security-hardening-rule.md` |
| `always/uncertainty-and-scope-rule.md` |
| `docs/documentation-audience-rule.md` |
| `docs/feature-flagged-docs-rule.md` |
| `docs/marketing-tone-rule.md` |
| `ui/accessibility-rule.md` |
| `workflow/boundary-shape-testing-rule.md` |
| `workflow/commit-no-cursor-trailer.md` |
| `workflow/concept-modelling-rule.md` |
| `workflow/coordination-publication-rule.md` |
| `workflow/debugging-rule.md` |
| `workflow/destructive-op-preflight-rule.md` |
| `workflow/documentation-rule-projection-rule.md` |
| `workflow/feature-regression-guard-rule.md` |
| `workflow/file-generator-rule.md` |
| `workflow/immutable-tags-rule.md` |
| `workflow/merge-request-workflow-rule.md` |
| `workflow/migration-generation-rule.md` |
| `workflow/no-auto-package-update.md` |
| `workflow/semver-no-carry-rule.md` |
| `workflow/tdd-rule.md` |
| `workflow/testing-environments-rule.md` |
| `workflow/visible-artifact-verification-rule.md` |

## 3. Converted to symlink — originally `differ`, confirmed pure staleness (14)

Body content differed, but manual diff confirmed every difference was the
`.claude/` copy lagging a later canonical edit (path renames, terminology
updates, added sections, corrected references) with no project-specific
customization worth preserving.

| Path | Nature of staleness |
|---|---|
| `README.md` | Canonical text already argues for the pointer architecture this plan implements. |
| `always/downstream-impact-rule.md` | Canonical gained a CUSTOMIZE note scoping the rule to distributing-governance-artifact projects. |
| `always/external-anonymization-rule.md` | Stale `agov-validate-infosec` tool name and issue-46 cross-repo reference wording. |
| `always/gate-self-test-rule.md` | Stale `src/agents_govern/validate_*` tooling path (now `tooling/rust/crates/`) and issue-46 wording. |
| `always/governance-file-layout-rule.md` | Stale phrasing of the `.claude/rules/` ownership note. |
| `always/verifiable-claims-rule.md` | Stale `rules-template/` path reference (now `rules/`). |
| `docs/implementation-honesty-rule.md` | Stale maturity-vocabulary term ("Surface-defined" vs. canonical "Schema-defined"). |
| `workflow/agent-proactive-analysis-rule.md` | Stale `.claude/rules/` reference vs. canonical's generalized loader wording. |
| `workflow/branch-and-worktree-rule.md` | Canonical gained a clarifying "immediately before" note the copy lacks. |
| `workflow/ci-trigger-scoping-rule.md` | Stale `rules-template/` path reference. |
| `workflow/deprecation-policy-rule.md` | Canonical rewording of the `seq`-field motivating incident. |
| `workflow/deputy-governor-rule.md` | Stale `rules-template/`/`.claude/rules/` path references. |
| `workflow/release-via-merge-request-rule.md` | Canonical gained an adopter-docs changelog checklist item the copy lacks. |
| `workflow/systematic-code-review-rule.md` | Canonical gained a `review-convergence-rule.md` cross-reference the copy lacks. |

## 4. Kept as hand-maintained copy — confirmed deliberate divergence (4)

Not symlinked. Each carries a "Canonical owner: ..." or "... Foundry
Projection" discovery-pointer pattern that intentionally trims
substantial canonical content — confirmed deliberate content-shaping
(likely context-budget optimization for large or conditionally-active
rules), not staleness. A symlink would silently restore the full
canonical body, an unreviewed behavioral and context-budget change this
plan does not make.

| Path | Canonical length | `.claude/` copy length |
|---|---|---|
| `workflow/simplicity-rule.md` | 414 lines | 222 lines |
| `workflow/development-workflow-rule.md` | 115 lines | 87 lines |
| `always/improvement-propeller-rule.md` | 154 lines | 12 lines |
| `always/universal-polling-and-autonomous-continuation-rule.md` | 218 lines | 11 lines |

## 5. Left untouched — foundry-local, no canonical counterpart (2)

| Path | Reason |
|---|---|
| `always/coordination-message-rule.md` | Carries `# Evidence: AGF-NG operational use`; legitimately project-local, not an AGF rule. |
| `always/coordination-sequence-guard-rule.md` | Same. |

## Post-conversion expected `assistant_drift` state

Zero `missing`, zero `differ` for the 49 converted paths (a symlink's
resolved content is byte-identical to its target by construction). The 4
hand-maintained copies continue to report `differ` (expected, documented
above — not drift in the sense this validator originally targeted). The 2
foundry-local files continue to report `extra` (expected, no target
exists). Total residual findings: 6.

---
kind: decision-record
from: governor-reviewer
to: all
id: 20260721T002557Z-governor-reviewer-66gd
timestamp: 2026-07-21T00:25:57Z
immutable: true
references-to:
  - 20260720T210500Z-coder-tester-en5m
  - 20260721T001149Z-coder-tester-gqdt
topic: deprecation-policy-gate1-authority-record
requires-ack: false
artifacts_owned:
  - docs/plans/deprecation-policy-rule.md
---

# Gate 1 approval: deprecation-policy-rule

The governor-reviewer (Jani Päijänen) approves
`docs/plans/deprecation-policy-rule.md` at Gate 1.

This decision-record supplies the governor-reviewer coordination message
required by `AGENTS.md:170`, which `gqdt`'s review correctly found missing
from MR !282 — the plan-file edit alone, with no coordination record,
could not supply governor authority.

## Evidence

- Direct governor confirmation in-session, 2026-07-21.
- GitLab: MR !262 (`plan+rule: add deprecation-policy-rule`) merged by
  `janipaijanen` at `2026-07-21T00:02:18.39Z` (platform `merged_at`).

`gate1_approved_at` on the plan is corrected to `2026-07-21T00:02:18Z` to
match this merge event.

## WARN resolution (prior-art verification)

`gqdt` also flagged that the prior-art citations (Kubernetes, Django,
PEP 387, Node.js, Rust, GitHub API) remained training-knowledge assertions
with no owned follow-up. Resolved directly rather than routed as a
separate task: all six were spot-checked via live fetch this session.

- **Kubernetes** — wrong as originally stated. The published policy gives
  GA APIs no fixed removal window at all (must not be removed within a
  major version — effectively indefinite), not "12 months or 3 releases".
  Only Beta APIs carry the 9-months-or-3-releases figure, applied twice
  (introduction→deprecation, then deprecation→removal).
- **GitHub REST/GraphQL API** — wrong as originally stated. Published
  policy is at least 24 months of support after a new API version's
  release, not "about 12 months".
- **Django, PEP 387, Node.js, Rust** — confirmed accurate as originally
  stated (Django: 2+ feature releases; PEP 387: 2+ minor releases, 5-year
  period now preferred; Node.js: no fixed number, major-version-gated,
  case by case; Rust: informal near-permanent stability via the
  `#[deprecated]` lint, no single official "never removed" policy
  document found).

Both rule copies (`.claude/rules/workflow/deprecation-policy-rule.md` and
`agf/rules/workflow/deprecation-policy-rule.md`) are corrected in this
same branch.

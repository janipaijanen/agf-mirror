# rule+docs: gate posture and consequence framing

Research-driven improvements from external governance article scan (June 2026).
No Gate 1 plan required — fast-track waiver applies (docs/rules only, no behaviour change).

## What changed

**`agf/rules/always/high-risk-operations-rule.md`** (+ `.claude/rules/` copy)
- Per-area consequence/blast-radius statements added to each high-risk bullet.
  Rationale: consequence-based framing ("if broken, X") improves compliance
  vs "stop before touching" alone (normalization of deviance under repeated
  approval tasks).

**`agf/rules/workflow/systematic-code-review-rule.md`** (+ `.claude/rules/` copy)
- "Gate types" section added with structural vs advisory enforcement table.
  Honest about the gap: advisory gates depend on agent discipline; structural
  gates enforce regardless of cooperation.

**`agf/docs/site/docs/adopt/tooling-and-tokens.md`**
- "Why meter at the governance gate" section added before the evidence model.
  Covers the chokepoint principle: gate-level metering provides per-role,
  per-gate attribution that API billing cannot.

**`agf/docs/site/docs/concepts/roles-and-gates.md`**
- "Structural and advisory enforcement" section added — gate-type table and
  link to contracts-and-validators. Consistent with the systematic-code-review
  change above.

## Verification

- `mkdocs build --strict`: clean.
- Both rule copies in sync (only provenance frontmatter differs).
- No security boundaries, no new locales, no breaking changes.

## Sources

- tenureai.dev — AI governance enforcement transparency
- Amazon/The Register, June 2026 — normalization of deviance under approval pressure
- r-lopes blog — governance metering and the chokepoint principle

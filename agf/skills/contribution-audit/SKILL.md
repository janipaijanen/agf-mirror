---
name: contribution-audit
origin: upstream
description: >
  Audit incoming contributions (learnings, rules, agent definitions,
  MRs) for adversarial steering, unsound guidance, and false precedent.
  Pairs with the contribution-auditor agent.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Contribution audit

Audit a contribution for adversarial patterns before accepting it
into the project. This skill implements the Contribution Auditor
agent's review process.

## When to run

- Before merging any MR that adds or modifies learnings, rules, or
  agent definitions
- When reviewing upstream framework updates (version upgrades)
- When an external contributor submits a learning record

## Step 1 — Run automated heuristics

```bash
uv run agov-validate-contributions [FILES...]
```

Without arguments, scans all `learnings/*.yaml`. Pass specific files
to audit targeted contributions.

The tool checks three dimensions mechanically:

1. **Absence bias** — "no incidents", "no complaints", "worked for us"
2. **Governance relaxation** — "remove the review step", "skip the check",
   "unnecessary overhead"
3. **Evidence quality** — "all projects should", "always works",
   "never fails"

## Step 2 — Manual review (six dimensions)

The automated tool covers three of six audit dimensions. The remaining
three require human judgment:

### Plausibility

Does the claimed pattern make technical sense?

- Is the failure mode described technically possible?
- Does the context match the claim?
- Would an expert in this domain find the pattern credible?

### Boundary integrity

Do proposed agent definitions enforce what they claim?

- Are jurisdictions clearly separated from existing agents?
- Do forbidden_actions match the agent's stated purpose?
- Are escalation triggers present for high-risk operations?
- Is the cannot_touch list meaningful (not empty or trivially permissive)?

### Rule soundness

Do proposed rules lead to safe outcomes when followed?

- Does the rule have an enforcement mechanism?
- Are exception clauses narrow enough to preserve the rule's intent?
- Could the rule be satisfied while circumventing its purpose?

## Step 3 — Upstream scrutiny (for version upgrades)

When upgrading the framework, audit the upstream diff:

```bash
diff -r <old-version>/ <new-version>/
```

Apply the same six dimensions to upstream changes. Upstream is not
exempt from auditing — the same red flags apply regardless of source.

Key areas to check in upstream diffs:
- Agent boundaries relaxed or removed
- Escalation triggers weakened
- New agents with overlapping jurisdictions
- Rules with broadened exception clauses

## Step 4 — Report findings

For each finding, document:

| Dimension | Finding | Severity | Recommendation |
|-----------|---------|----------|----------------|
| (which)   | (what)  | (high/medium/low) | (accept/reject/modify) |

Present findings to the human reviewer (Governor-Reviewer gate).

**Findings are patterns, not verdicts.** Every finding requires human
judgment to determine whether it represents genuine adversarial intent,
an honest mistake, or a false positive.

<!-- CUSTOMIZE: add project-specific red flags (domain terms that
indicate governance relaxation in your context) -->

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Rules Template

Governance rules that implement the principles from
[framework.md](../framework.md). Each rule is a markdown document
describing constraints and guidelines — the content is
technology-agnostic and works with any AI coding assistant that accepts
natural-language instructions.

The canonical rule copy stays in `agf/rules/`. Tool-specific root files
should point assistants at this directory instead of duplicating the rule tree
into `.claude/rules/`, `.cursor/rules/`, `AGENTS.md`, or another assistant
location. That keeps the framework carve-out honest: removing `agf/` removes
AGF-owned content.

## How to adopt

1. Confirm your assistant can read this canonical `agf/rules/` directory.
2. If your tool needs a root-level configuration file, create a small local
   loader that tells it to read `agf/rules/`.
3. Search for `<!-- CUSTOMIZE -->` comments and adapt to your stack.
4. Add stack-specific local guidance from `project-specific-examples/` if your
   stack matches (e.g. `python-flask/`)
5. Create an index or loader file if your tool supports one. Keep that file
   local and small; do not copy the AGF rule files into it.

## Directory structure

```
rules/
├── always/                          # Core constraints — apply to every change
│   ├── uncertainty-and-scope-rule.md    # Scope boundaries, conflict escalation
│   ├── high-risk-operations-rule.md     # Stop-and-plan for audit, auth, security
│   └── security-hardening-rule.md       # Secrets, supply chain, CSRF, containers
├── workflow/                        # Process rules — commits, testing, releases
│   ├── tdd-rule.md                      # RED-GREEN-REFACTOR, anti-patterns
│   ├── debugging-rule.md                # Four-phase root-cause analysis
│   ├── simplicity-rule.md               # Complexity governance, task-size calibration
│   ├── feature-regression-guard-rule.md # No silent test deletion, characterisation tests
│   ├── systematic-code-review-rule.md   # Pre-commit quality gate checklist
│   ├── branch-and-worktree-rule.md      # Branch naming, pre-push gate, merge options
│   ├── development-workflow-rule.md     # Five phases: clarify, plan, decompose, implement, review
│   ├── immutable-tags-rule.md           # Tags immutable once pushed
│   ├── semver-no-carry-rule.md          # Version components grow beyond 99
│   ├── no-auto-package-update.md        # No upgrades without approval
│   └── commit-no-cursor-trailer.md      # No AI-tool trailers in commits
├── ui/                              # Templates, CSS, accessibility
│   └── accessibility-rule.md            # WCAG AA compliance
└── docs/                            # Documentation governance
    ├── documentation-audience-rule.md   # Audience scope boundary
    └── marketing-tone-rule.md           # Communication integrity
```

## Mapping to framework principles

| Rule | Framework Principle |
|------|-------------------|
| uncertainty-and-scope-rule | Principle 5: Escalation over Improvisation |
| high-risk-operations-rule | Principle 5 + Pattern 3: Escalation |
| security-hardening-rule | ADOPTING.md Step 3.2: Security checklist |
| tdd-rule | Pattern 1: Pipeline (quality gate) |
| debugging-rule | Principle 5: Escalation over Improvisation |
| simplicity-rule | Pattern 6: Fast Track + Principle 7: Technology Neutrality |
| feature-regression-guard-rule | Pattern 1: Pipeline (test-as-contract) |
| systematic-code-review-rule | Pattern 5: Cross-Cutting Review |
| branch-and-worktree-rule | Principle 4: Pipeline Topology |
| development-workflow-rule | Pattern 1: Pipeline |
| immutable-tags-rule | Principle 3: The Immutability Constraint |
| semver-no-carry-rule | Appendix: Versioning and Identifiers |
| no-auto-package-update | ADOPTING.md Step 3.2: Supply chain trust |
| commit-no-cursor-trailer | Principle 2: Artifact Ownership |
| accessibility-rule | ADOPTING.md Step 3.3: Accessibility checklist |
| documentation-audience-rule | Pattern 7: Documentation Review Pipeline |
| marketing-tone-rule | Pattern 7: Documentation Review Pipeline |

## Stack-specific examples

See [project-specific-examples/](../project-specific-examples/) for
complete rule sets tailored to specific technology stacks. Currently:

- **[python-flask/](../project-specific-examples/python-flask/)** —
  Python + Flask/Django/FastAPI + SQLAlchemy + Alembic + uv

---
kind: decision-record
from: governor-reviewer
to: all
id: 20260621T234324Z-governor-reviewer-jr1j
timestamp: 2026-06-21T23:43:24Z
immutable: true
references-to:
  - 20260621T214311Z-coder-tester-sjqb.md
topic: first-run-loader-guardrail-gate2-approved
requires-ack: false
artifacts_owned:
  - docs/coordination/20260621T234324Z-governor-reviewer-jr1j.md
decision:
  outcome: approved
  branch: docs/first-run-loader-guardrail
  mr: "!92"
---

# Gate 2 retroactive approval — docs/first-run-loader-guardrail

Retroactive Gate 2 review of `docs/first-run-loader-guardrail`, merged at
`609b854`. Fast-track basis: one new docs page, two docs additions. No rules,
schemas, agents, or security boundary changes. One nav addition (new Adopt
page). Reviewing against `sjqb` handoff `20260621T214311Z-coder-tester-sjqb.md`.

## Scope

Three artifacts:
- `agf/docs/site/docs/adopt/assistant-loader-configuration.md` (new)
- `agf/docs/site/docs/adopt/install-and-layouts.md` (two additions)
- `agf/docs/site/docs/advanced/contracts-and-validators.md` (one section added)

## Review

**assistant-loader-configuration.md**

New page covering: why loaders matter, the loader contract (point at `agf/`,
do not copy the rule tree into tool dirs), a multi-tool table (Claude Code,
Cursor, Copilot, Codex CLI, web chat), minimal loader text, a verification
prompt, and a "pair with structural guardrail" pointer.

Content is accurate and appropriately scoped. The key constraint — "Do not
copy AGF rules into this tool directory" — is stated clearly and repeatedly.
The verification prompt ("which AGF files did you load?") is a practical and
honest test.

Finding (WARN, not a block): the vendor reference URL for Claude Code
(`https://code.claude.com/docs/en/memory`) is not a current Anthropic docs
URL. The page carries the mitigating disclaimer: "Tool vendors change their
instruction-loading mechanisms. Prefer the vendor's current documentation for
exact file names and settings." This disclaimer explicitly protects the claim
from stale URL rot; the URL is illustrative, not authoritative. WARN only;
does not block approval.

**install-and-layouts.md: loader pointer**

"Before the first governed change, configure the assistant to see the AGF
files and verify that it can name the rules it loaded" + link to new page.
Added in two places: prerequisites and Step 4. Correct.

**contracts-and-validators.md: Minimum structural guardrail section**

5-step minimum (branch protection, coord validate, plan validate, blocking
policy, self-test). The humility note — "does not prove that a human really
approved a decision or that the implementation is correct" — is accurate and
important. Correct.

## Decision

**Approved.** Content is accurate, practically grounded, and addresses the
first-run loader gap identified by Lumo's external review. WARN filed on
vendor URL staleness; mitigated by the existing disclaimer.

Acks Gate 2 handoff `20260621T214311Z-coder-tester-sjqb.md`.

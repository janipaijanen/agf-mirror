---
kind: note
from: coder-tester
to: governor-reviewer
seq: 295
timestamp: 2026-06-14T02:44:25+03:00
immutable: true
references-to:
  - 00292-coder-tester.md
  - 00294-coder-tester.md
topic: public-docs-what-agf-is-and-is-not
requires-ack: false
artifacts_owned:
  - agf/docs/site/index.md
  - agf/docs/site/docs/start/should-you-adopt.md
  - agf/docs/site/llms.txt
model: codex
---

Added explicit public-facing "what AGF is / is not" language.

Updated:

- `agf/docs/site/index.md`
- `agf/docs/site/docs/start/should-you-adopt.md`
- `agf/docs/site/llms.txt`

The public docs now state that AGF is a repository-local governance framework
for software-development work involving AI agents. They also state that AGF is
not a hosted service, agent runtime, model-safety filter, legal/compliance
product, security audit, general AI product-governance system, or replacement
for human gate ownership.

This supports public-reader classification and RQ3 maturity honesty: AGF
defines governance artifacts and review discipline around existing assistants,
CI, review, and release tools; it does not itself execute agents or grant
runtime permissions.

For local preview, the ignored generated `public/llms.txt` was mirrored from
the source file. A site rebuild should regenerate the public copy from
`agf/docs/site/llms.txt`.

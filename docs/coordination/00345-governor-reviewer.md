---
kind: decision-record
from: governor-reviewer
to: coder-tester
seq: 345
timestamp: 2026-06-15T18:10:30Z
immutable: true
references-to:
  - 00340-coder-tester.md
topic: readme-pages-link-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: README documentation link

Branch: `docs/readme-pages-link`

## Change

`README.md` gains one line near the top:

```
Published docs: <https://janipaijanen.gitlab.io/agents-govern-framework/>
```

## Review

- README targets developers (correct audience per documentation-audience-rule).
- URL is the project's own GitLab Pages site — factual, not marketing.
- No security implications; no credentials; no adopter contract change.
- Curl reached the host (GitLab Pages auth redirect confirms the site exists).
- Change is self-contained; no other files modified.

## Decision

Approved. Merge `docs/readme-pages-link` into main.

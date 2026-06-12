---
kind: decision-record
from: governor-reviewer
to: all
seq: 117
timestamp: 2026-06-12T21:03:58Z
immutable: true
references-to:
  - 00116-coder-tester.md
topic: activate-fact-checker-and-academic-peer-reviewer
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Decision: activate fact-checker and academic-peer-reviewer

Governor-reviewer activates two advisory roles for the v2.0 documentation
lanes:

- `fact-checker`
- `academic-peer-reviewer`

## Reason

The deferred v2.0 documentation work now includes:

- marketing/evaluation copy;
- progressive end-user documentation;
- generated figures and visual claims;
- academic/manual PDF documentation.

These surfaces create factual, evidentiary, citation, and academic-survivability
review needs that the previously suspended/skipped statuses no longer match.

## Manifest changes

Updated both current manifests:

- `agents-govern/agents/MANIFEST.yaml`
- `agents-govern-2.0/agents/MANIFEST.yaml`

`fact-checker` is active for factual/docs claim review.

`academic-peer-reviewer` is active for the paper/PDF lane and
academic-style claims, references, definitions, and survivability review.

## Validation

```text
agents-govern: agov-validate-agents
All 12 agent definitions valid, 1 warning(s)

agents-govern-2.0: agov-validate-agents
All 12 agent definitions valid, 1 warning(s)
```

The warning is the pre-existing ownership overlap between
`academic-peer-reviewer` and `academic-visual-reviewer`.

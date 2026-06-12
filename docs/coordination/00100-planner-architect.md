---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 100
timestamp: 2026-06-12T18:53:13Z
immutable: true
references-to:
  - 00099-planner-architect.md
topic: agents-govern-2.0-gate1-pinned-latex-pdf-pipeline
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of amended agents-govern-2.0 directory structure plan"
    - "Approve or reject two-stage PDF pipeline: Markdown -> TeX via Pandoc, TeX -> PDF via kjarosh/latex:2025.1-medium"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00099-planner-architect.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Gate 1 amendment: pinned LaTeX PDF pipeline

This handoff refines seq 99. The academic PDF path should be reproducible and
release-specific:

1. Markdown remains canonical source.
2. Pandoc emits a TeX intermediate from `docs/paper/*.md`.
3. `kjarosh/latex:2025.1-medium` compiles the TeX to PDF with `latexmk`.

## Notes

The image name is `kjarosh/latex:2025.1-medium`.

The generated `.tex` should be available as a review/debug artifact for each
release. The plan leaves open whether the `.tex` is committed or emitted as a
CI artifact, but Gate 2 must inspect or reference it together with the final
PDF and build log.

## Validation

Plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```

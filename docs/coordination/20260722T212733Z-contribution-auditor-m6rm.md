---
kind: handoff
from: contribution-auditor
to: governor-reviewer
id: 20260722T212733Z-contribution-auditor-m6rm
timestamp: 2026-07-22T21:27:33Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T212557Z-contribution-auditor-hsdg.md
topic: agov-academic-figure-generation-todo-provenance-update
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the provenance-amended MR !416 when its exact-head pipeline passes.
  must_read:
    - docs/TODO.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# MR !416 TODO amended with figure provenance requirements

The governor-reviewer added one requirement after handoff `hsdg`: generated
figures must include provenance information, at least in metadata, so future
readers can track them back to their source.

The TODO now requires a versioned closed-allowlist provenance record linking
the generator version, repository commit, explicit evidence window/snapshot,
governed configuration and repository-relative input files. It also requires
DOT-to-SVG/PDF subject and SHA-256 linkage through an embedded record or
adjacent sidecar chosen at Gate 1.

Negative acceptance cases cover missing, mismatched, dangling, duplicated,
unknown-key and checksum-inconsistent metadata. Privacy fixtures prohibit
absolute/home paths, host/user identifiers, assistant/model-session/
conversation/request identifiers, and raw environment maps. Ambient generation
timestamps are not permitted unless explicitly supplied as governed input,
because they would otherwise break byte reproducibility.

The superseded exact-head pipeline `2698446331` was cancelled when this
requirement arrived; it is not failure evidence. A single ordinary acceptance
push for the amended head follows.

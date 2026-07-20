---
kind: handoff
from: coder-tester
to: coder-tester
id: 20260720T220509Z-coder-tester-qhdh
timestamp: 2026-07-20T22:05:09Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T212711Z-coder-tester-16p7.md
  - 20260720T215132Z-coder-tester-gmrh.md
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
topic: cross-llm-coordination-three-layer-learning-review
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Review the new learning record (id 20260720T220125Z-k8s7) added to
      agf/learnings/foundry.yaml, drafted from Codex's own three-property
      framing and evidence-status wording in the human/Claude/Codex
      discussion earlier this session.
    - Confirm the two evidence_refs accurately represent what 16p7/gmrh
      showed, and flag if the single-adopter confidence level (evidence
      count 2) classification should change.
  must_read:
    - agf/learnings/foundry.yaml
  do_not_touch: []
---

# New learning record: cross-LLM coordination needs three separable layers

Codex, this records the pattern you proposed in the human/Claude/Codex
discussion following the MR !268/!269 routing fix: durable Git
coordination records, disposable MR-note discovery bridges, and
observable receipt (correct role routing plus requires-ack/response/
takeover) are three separable mechanisms, not two.

Entry `20260720T220125Z-k8s7` in `agf/learnings/foundry.yaml` (category:
validation, scope: framework, severity: significant) cites two pieces of
evidence:

1. The MR !255 dir-rename Gate 1 BLOCK chain (`16p7`) — specific,
   evidence-backed findings (the residual stdout-parsing line, the stale
   MR-description contract) were fixed precisely because they named the
   exact defect.
2. The MR !268/!269 routing misfire (`gmrh` correcting `r2sr`) — a
   durable record and a visible MR note both existed and still routed the
   Gate 1 decision to the wrong role; only explicit role-routing plus
   `requires-ack` caught and fixed it.

Also folded in your correction on the durability claim: a coordination
record survives platform loss only once merged or replicated into
another clone, not merely once committed to a single remote branch.

Marked `status: proposed`, `confidence.level: single-adopter` per your
suggested evidence-status wording — not yet a rules-template candidate,
pending further corroboration. `agov validate learnings` and
`agov validate infosec` both pass on the addition.

Branch: `learn/cross-llm-coordination-three-layers` (off `main` at
`64e63d7`), commit `16ee8ea`, not yet merged.

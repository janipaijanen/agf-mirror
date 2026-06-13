---
kind: handoff
from: coder-tester
to: planner-architect
seq: 228
timestamp: 2026-06-13T19:02:59Z
immutable: true
references-to:
  - 00227-coder-tester.md
topic: agf-v2-inherited-material-verification-policy
requires-ack: true
artifacts_owned:
  - docs/TODO.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Critically review docs/TODO.md section "AGF v2 Verification Audit".
    - Argue whether inherited v1.x material should be a hard release gate or staged release gate.
    - "Recommend where verification disposition should live: inline labels, manifest, generated audit report, schema fields, or a combination."
    - Identify which AGF v2 public/distribution docs must be verified first.
  must_read:
    - docs/TODO.md
    - docs/coordination/00227-coder-tester.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Governor raised an AGF v2 provenance concern:

Any learning, rule, documentation claim, or related framework content inherited
from agents-govern v1.x must not be presented as verified AGF v2 behavior until
it has actually been checked against AGF v2. If not verified, it should be
labelled as inherited from v1.x and possibly valid for AGF v2, but not currently
verified. Adopters may then verify and report evidence upstream.

Coder-tester created `docs/TODO.md` with a proposed disposition model:

- `verified-agf-v2`
- `inherited-v1x-unverified`
- `adapted-from-v1x`
- `retired-for-agf-v2`

Codex position:

- This should become a staged release gate rather than an immediate blocker for
  all development.
- Highest-risk artifacts should be verified first: adoption/exit docs,
  downstream-impact docs, behavior-changing rules, validators, and any claim
  that says AGF v2 supports something.
- Low-risk background material may temporarily carry an
  `inherited-v1x-unverified` label, but should not appear in the primary
  adoption path as current guidance.
- Long term, a small Elixir-validated audit manifest plus human-facing inline
  labels seems cleaner than scattering status prose everywhere.

Planner-architect requested: please improve, argue, or reject parts of this
model before it becomes release policy.

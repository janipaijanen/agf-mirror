---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T051057Z-planner-architect-ahgr
timestamp: 2026-08-24T05:10:57Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T044822Z-contribution-auditor-2g8e.md
topic: learning-record-backlog-round9-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of the round-9-corrected exact head for MR
      !1856 (the critical finding from
      `20260824T044822Z-contribution-auditor-2g8e.md` addressed).
  must_read:
    - 20260824T044822Z-contribution-auditor-2g8e.md
  do_not_touch: []
---

# Learning-record backlog round 9: silent-correction escape hatches removed

`2g8e` confirmed the round-7 wording purge itself resolved, but found
two remaining direct-write paths — this plan's own implementation scope
and its generic adopter migration guidance — that silently corrected a
stale `framework_version` exactly where rounds 6–8 had already forbidden
Step 0 from doing the same thing.

## Exact head under review

MR !1856 (`adoption-version-drift-check`) at
`bb1a23a63d6902aac2c1eafd04dced7f7a6efd54` — fixes `CA-LRB-VER-R8-1`:

- **Implementation scope:** "Files expected to change" previously had
  this plan's own Gate 2 commit add `checkout_role: live` to
  `foundry.yaml` *and* directly correct `framework_version` to
  `agf/VERSION` at merge time — silently erasing the exact historical
  baseline (`2.0.0-provisional`) round 7 was fixed to preserve. Fixed:
  `foundry.yaml` gets `checkout_role: live` only; `framework_version`
  stays unchanged, so the new check correctly, honestly fires as a
  blocking finding on this project's own live record immediately after
  Gate 2 lands — not a bug to suppress.
- **New "One-time reconciliation" design subsection:** replaces the
  silent fix with a disclosed, three-step governed repair (add the
  role unmarked-version; file a coordination record naming both the
  stale and current version and whether the missed range was reviewed;
  only then correct the version in a *separate*, prompt, later commit —
  never bundled into the same commit that adds `checkout_role: live`).
  Timing language added so this doesn't leave the project's own `agov
  validate` gate broken indefinitely — the follow-up is expected
  promptly, same or next session.
- **Generic adopter Migration clause:** previously told every adopter to
  "correct the version before or in the same commit as adding
  checkout_role: live" — the identical anti-pattern at adopter scale.
  Fixed to point at the same One-time reconciliation procedure instead.
- New acceptance evidence: asserts the implementation diff never
  mutates `foundry.yaml`'s `framework_version`, and that the revised
  Migration guidance contains no same-commit-correction instruction.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the corrected head above.

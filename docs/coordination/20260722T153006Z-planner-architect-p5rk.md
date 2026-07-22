---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260722T153006Z-planner-architect-p5rk
timestamp: 2026-07-22T15:30:06Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260722T151954Z-contribution-auditor-angh
topic: dossier-authoritative-references-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/dossier-authoritative-references.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review docs/plans/dossier-authoritative-references.md for Gate 1 approval.
    - Confirm the exact-build link strategy, no-copied-artifact boundary, and MR !328 sequencing.
  must_read:
    - docs/plans/dossier-authoritative-references.md
    - docs/coordination/20260722T151954Z-contribution-auditor-angh.md
    - MR !328 exact diff and current disposition
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 handoff: dossier authoritative references

The plan is ready for governor review. It implements the direct instruction to
update the evaluation dossier without importing repository contents.

The proposed boundary is:

- preserve the existing Flow 07 public-documentation corpus;
- add a direct Evaluate-to-dossier route;
- generate a link-only authoritative repository reference table pinned to the
  documentation build's exact source commit;
- never read or copy the referenced artifact bodies into the dossier;
- make the copied Evaluate self-link resolve inside the dossier; and
- sequence implementation after the overlapping MR !328 is resolved.

The plan includes test-first hook coverage, rendered-link checks, a dynamic
public-page inventory check, exact-source link verification, and `ci.skip`
cost containment until the duplicate-pipeline defect is fixed.

`agov plan validate docs/plans/dossier-authoritative-references.md` passes with
blocking severity. Implementation remains prohibited until the governor records
Gate 1 approval.

---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260824T185045Z-governor-reviewer-a86h
timestamp: 2026-08-24T18:50:45Z
model: human-via-codex
immutable: true
references-to:
  - 20260824T163113Z-governor-reviewer-36rr.md
  - 20260824T183138Z-governor-reviewer-4grm.md
  - 20260824T184348Z-planner-architect-sbqj.md
  - 20260824T184934Z-contribution-auditor-7km1.md
topic: mr1847-fail-closed-dangling-symlink-disposition
requires-ack: false
artifacts_owned: []
---

# MR !1847: replace dangling-target resolution with fail-closed semantics

The chained relative dangling-target bypass reported by `7km1` is material and
remains the same `CA-NNP-R14-1` finding. The governor directs a simpler final
design rather than another bespoke target-resolution refinement.

For the proposed classifier, every existing symlink component must
canonicalize successfully and remain under the canonical repository root. If
canonicalizing a symlink component fails for any reason—including a direct or
chained dangling target—the result is `Invalid`, never `ContainedMissing`.
`ContainedMissing` is available only after the deepest existing component has
been successfully canonicalized within the repository and the first absent
component is not itself an existing symlink entry. The lexical absolute and
parent-directory rejections remain mandatory.

This deliberately rejects repository-internal dangling symlinks as well as
outside-pointing ones. That conservative false negative is accepted: a
dangling symlink is not sufficient evidence that a planned owner is safely
repository-contained. The plan must remove the `read_link` fallback and state
this fail-closed compatibility consequence explicitly.

Acceptance evidence is limited to the named finding: direct outside dangling
symlink, chained relative dangling symlink, existing outside-pointing ancestor
symlink, and a legitimately absent non-symlink path, through
`--allow-planned`, `projection-check-all`, and the strict route. One final
targeted proof by the original round-14 reviewer is authorized; no full review
or new findings. A CLEAR result at the exact green head activates Gate 1
approval and authorizes merge of plan-only MR !1847 on the governor-reviewer's
behalf.

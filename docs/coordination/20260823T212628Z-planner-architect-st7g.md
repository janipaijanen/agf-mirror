---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T212628Z-planner-architect-st7g
timestamp: 2026-08-23T21:26:28Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260823T205558Z-contribution-auditor-pssr.md
topic: learning-record-backlog-batch-round1-fixed-request-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - >-
      Fresh COMPLETE independent audit of all four plans at their
      round-1-corrected exact heads, following the same isolated-
      session sourcing pattern used throughout this backlog batch.
  must_read:
    - docs/plans/20260823T203534Z-upgrade-overlay-provenance-protection.md
    - docs/plans/20260823T203540Z-adoption-version-drift-check.md
    - docs/plans/20260823T203546Z-dist-path-ignore-protection.md
    - docs/plans/20260823T203552Z-review-topology-declaration.md
    - 20260823T205558Z-contribution-auditor-pssr.md
  do_not_touch: []
---

# Learning-record backlog batch: all 11 blocking findings fixed, fresh review requested

`pssr`'s COMPLETE audit found 11 blocking findings + 1 advisory across
all four plans. Each verified independently against the actual
repository state (a throwaway repo to confirm the exact `git
check-ignore` exit-code behavior claimed in `CA-LRB-IGN-1`; direct
reads of `agf/learnings/foundry.yaml` and `harbor-workshop.yaml` to
confirm `CA-LRB-VER-1`'s version-mismatch claim; direct reads of
`agf/dist/release-manifest.yaml` and `dist.rs` for `CA-LRB-IGN-2`) and
fixed:

- **MR !1855** (`3697d3ad`) — `CA-LRB-UPG-1/2/3`. The five-class
  scheme is replaced with an exhaustive 16-row truth table over
  presence × file type × content equality (every state gets exactly
  one class and one operation; unclassified/type-mismatched states
  fail closed). A new "Trust and path-confinement boundaries" section
  specifies manifest-authenticity verification, canonical path
  confinement, symlink policy, and case-collision handling for a
  subcommand that authorizes file deletion. Reclassified BREAKING with
  a named adopter page (`agf/docs/site/docs/adopt/upgrade.md`, new) and
  a rollback section that accounts for post-adoption consequences.
- **MR !1856** (`bc20a35c`) — `CA-LRB-VER-1/2/3`. Added an explicit
  `adoption.checkout_role: live`/`imported` opt-in field so the check
  never asserts a version claim about imported evidence records
  (verified against this repository's own `foundry.yaml`/`harbor-
  workshop.yaml`, which currently disagree with `agf/VERSION` for two
  different, both-legitimate reasons). Decided the full mid-upgrade
  marker lifecycle (shape, owner, 24h crash-recovery bound, default
  behavior) rather than deferring it. Reclassified BREAKING.
- **MR !1857** (`380e3cf0`) — `CA-LRB-IGN-1/2/3`. Switched to `git
  check-ignore -q --no-index` (verified: the plain `-q` form misses a
  tracked, force-added path whose ignore rule still effectively
  matches it). Manifest enumeration now reuses `agov`'s existing,
  already-tested `dist::validate_default()`/`DistSpec` instead of
  writing new glob/exclude/optional expansion logic. Wired into the
  default `agov validate` umbrella; reclassified BREAKING.
- **MR !1858** (`825b999b`) — `CA-LRB-TOP-1/2/A1`. Renamed the field to
  `blinded-to-prior-answers` plus a required independence disclosure
  (session/prompt/model-family/evidence-selection), since not having
  seen a prior answer is necessary but not sufficient for independence.
  Made explicit that review topology never changes an adopter's
  `confidence` level or increments `evidence_count` — it is orthogonal
  metadata about how a review was conducted. Advisory A1 addressed:
  the field is Level 2 (Surface-defined) guidance, not a guaranteed,
  enforced declaration.

All four heads: `agov plan validate` OK; `agov validate` (full
umbrella) shows no new findings beyond the pre-existing, unrelated
learning-ID and 51-item `.claude/rules` drift warnings.

Requesting a fresh COMPLETE audit of all four corrected heads before
Gate 1 is sought on any of them.

---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T215029Z-planner-architect-apqs
timestamp: 2026-08-23T21:50:29Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260823T213054Z-contribution-auditor-rerj.md
topic: learning-record-backlog-round2-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of the round-2-corrected exact heads for MR
      !1855, !1856, and !1857 (all 7 blocking findings from
      `20260823T213054Z-contribution-auditor-rerj.md` addressed). MR
      !1858 was already CLEAR at the round-1 head and is unchanged.
  must_read:
    - 20260823T213054Z-contribution-auditor-rerj.md
  do_not_touch: []
---

# Learning-record backlog round 2: all 7 blocking findings from `rerj` fixed

`rerj` reviewed the round-1 corrected heads and blocked three of the four
plans on 7 new implementation-boundary findings — all independently
verified against actual repository state (crate dependency graphs in
`Cargo.toml`, private-module visibility in `main.rs`,
`tool-authority-boundary-rule.md`'s current text, and
`validate-provenance`'s real CLI signature) before being fixed, per this
session's usual discipline of not rubber-stamping review findings.

## Exact heads under review

- MR !1855 (`upgrade-overlay-provenance-protection`) at
  `fda143417fb6f44b2f7bf1027f4efe4415cda986` —
  fixes `CA-LRB-UPG-R1-1` (crate placement: `upgrade_classify.rs` moved
  from `agov-coord` to `agov` itself, since `agov` is a `[[bin]]`-only
  crate with a private `mod dist;` that `agov-coord` cannot reach without
  a dependency cycle; local-tree enumeration now explicitly defined as a
  filesystem walk, not manifest-derived), `CA-LRB-UPG-R1-2` (provenance
  input contract: explicit `--previous-provenance`/`--new-provenance`/
  `--previous-version`/`--new-version` flags, exact
  `validate-provenance --artifact-dir` invocation, and named failure
  modes for tampered/wrong-version/wrong-archive records), and
  `CA-LRB-UPG-R1-3` (exit-code contract: new machine-readable `status`
  field per path — `clean`/`no-op`/`confirmation-required`/
  `manual-conflict` — with exit 0 only when nothing is outstanding, exit
  1 when any confirmation/conflict is unresolved, and a
  `--dispositions` second-invocation mode the skill must use before
  proceeding).
- MR !1856 (`adoption-version-drift-check`) at
  `aa4bc3d177b4a9f495fdb3fe8c729af570c63b8e` — fixes
  `CA-LRB-VER-R1-1` (opt-in `checkout_role: live` marking previously
  activated detection for nobody by default; now a second, always-on
  advisory finding fires whenever zero live records exist, exactly one
  live record is required — two or more is a blocking ambiguity finding
  — and both the commissioning walkthrough and the upgrade skill's Step
  4 are given concrete, decided touchpoints that set the field) and
  `CA-LRB-VER-R1-2` (adopter documentation page named exactly:
  `agf/docs/site/docs/adopt/install-and-layouts.md`, "Step 7: Initialise
  your evidence" — not deferred to implementation time).
- MR !1857 (`dist-path-ignore-protection`) at
  `1be0d1fb8b23c2853bc284574629c628ca4fb42e` — fixes
  `CA-LRB-IGN-R1-1` (crate placement: `dist_visibility.rs` moved from
  `agov-validators` to `agov` itself, same underlying reason as
  `CA-LRB-UPG-R1-1`) and `CA-LRB-IGN-R1-2` (no rule granted this check
  blocking authority per `tool-authority-boundary-rule.md`; a new,
  narrowly-scoped rule file,
  `agf/rules/always/dist-provenance-visibility-rule.md`, is added,
  modeled on that rule's own required shape for validator authors, with
  its own `documentation-rule-projection-rule.md` six-row matrix
  requirement noted at Gate 2).
- MR !1858 (`review-topology-declaration`) — unchanged since `rerj`'s
  CLEAR verdict at the round-1 head; no round-2 action needed.

Each fix was validated individually (`agov plan validate`: OK on all
three) and against the full local umbrella (`agov validate`: no new
findings beyond pre-existing, unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the three corrected heads above.

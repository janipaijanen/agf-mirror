---
kind: note
from: planner-architect
to: coder-tester
id: 20260814T072406Z-planner-architect-g722
timestamp: 2026-08-14T07:24:06Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260814T070936Z-coder-tester-amjp.md
topic: frozen-scope-plan-round13-fix
requires-ack: false
artifacts_owned:
  - docs/plans/20260813T155743Z-frozen-scope-reconstruction-rule-amendment.md
---

# Round-13 fix: tree-OID/patch-hash category error corrected; fail-closed claim narrowed to an open governor decision

Thanks for the focused recheck. Both findings confirmed correct on
independent re-read of the plan text; fixed at new head (see below), not
contested.

**`B-FSR-SEC-R13-1` (tree/patch category+temporal error) — fixed.** The
release-merge governor record and post-merge check now name and compare
the exact reviewed source HEAD's Git tree OID, recorded only after all
release-only metadata edits are frozen — never the pre-metadata
deterministic-replay patch/tree hash (`c7db9317…` / `fbe16ab1…`), and
never asserted equal to a SHA-256 patch hash. Added acceptance criterion
21 (satisfied) documenting the fix.

**`B-FSR-SEC-R13-2` (fail-closed overclaim) — fixed.** Removed
"genuinely fail-closed... no separate governor sign-off" from the
"Protected target and source" section, the "Fail-closed behavior"
section, and acceptance criterion 20's prose. The property is now
described precisely as fail-closed **at the publication boundary**
(no tag/release/namespace on mismatch), with explicit disclosure that a
proof-invalid merge can still land on the isolated, then-abandoned
protected target before detection — post-landing detection relative to
that target, not literal prevention. `apkm`'s scope (forward-sync merge
only, precedent expressly disclaimed for release-construction) does not
cover this case. Acceptance criterion 20 is now marked **open — governor
disposition required**: whether fail-closed-before-publication with
isolated-target detect-and-remediate satisfies `2r8r`, or needs its own
scoped exception, is left as a named decision for governor Gate 1, not
resolved unilaterally by this text.

New exact head: pushing now. `agov plan validate` — OK.

Per your note: "After correction, refresh the exact-head security aid
and the configured infosec-sentinel consultation before Gate 1" —
agreed; both remain outstanding. Nothing further from planner-architect
until one of those lands or the governor rules on the newly-named
criterion-20 question.

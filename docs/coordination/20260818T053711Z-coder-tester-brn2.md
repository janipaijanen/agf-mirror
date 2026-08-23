---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260818T053711Z-coder-tester-brn2
timestamp: 2026-08-18T05:37:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260817T224145Z-governor-reviewer-qt32.md
  - 20260817T225613Z-coder-tester-t5g7.md
  - 20260818T045706Z-contribution-auditor-8js4.md
  - 20260818T053050Z-contribution-auditor-31mk.md
topic: cl3-corpus-boundary-binding-decision
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Record the separate binding CL-3 corpus-boundary decision now that the authorized inventory and independent review chain are complete.
  must_read:
    - docs/plans/20260817T210816Z-commercial-licensing-gate1-recovery.md on MR !1723
    - docs/reviews/commercial-licensing/20260817T224900Z-rights-provenance-inventory/README.md on MR !1723
    - docs/coordination/20260818T045706Z-contribution-auditor-8js4.md on main
    - docs/coordination/20260818T053050Z-contribution-auditor-31mk.md on MR !1747
  do_not_touch:
    - .claude/settings.local.json
    - MR !1712 source branch
    - MR !1723 source branch
---

# CL-3 evidence complete; binding corpus-boundary decision requested

The prerequisite sequence approved in `qt32` is complete:

- MR !1723 exact head `d13bb9084fce9a9b1c7c08bb735e0fe6d08d7e98`
  contains the dated technical rights/provenance inventory;
- its exact-head pipeline `2767998762` succeeded;
- independent contribution-auditor review `8js4` found Snapshot 1 and the
  inventory structurally CLEAR and raised one advisory, `CLG-INV-1`;
- coder-tester resolved that advisory without changing `inventory.csv` or
  `candidate-sets.csv`; and
- independent recheck `31mk` in green MR !1747 is CLEAR with no outstanding
  advisory items from the review chain.

## Decision evidence

The bounded inventory contains 191 files:

- 142 classified `Licensor-authored` from the stated technical evidence;
- 49 classified `generated/vendor`, all of which are project-maintained
  `.claude/rules` or `.claude/commands` projections/loaders of canonical
  `agf/` content;
- 0 confirmed third-party or externally sourced files; and
- 0 `unknown` items under the technical rules.

The 49 generated projections have no separately evidenced
commercial-relicensing grant. Git/header evidence is not a legal conclusion,
and the inventory expressly routes legal sufficiency to the governor and
qualified counsel.

## Binding choice requested

Please record one CL-3 outcome:

1. **Option (a): restrict the commercial corpus.** Start from the 142
   `Licensor-authored` rows and exclude the 49 generated projections unless
   and until governor/counsel affirmatively clears them. This is the lower-risk
   and lower-process option, but produces a narrower initial commercial
   corpus.
2. **Option (b): establish an explicit inbound relicensing mechanism.** Add a
   CLA or an explicit relicensing-grant clause for future contributions. This
   may support a broader future corpus but adds contributor friction and does
   not retroactively clear the current 49 generated projections; those still
   require exclusion or affirmative governor/counsel clearance.

Coder-tester's operational recommendation is **option (a) for the initial
offer, with the 49 generated projections excluded pending qualified-counsel
clearance**, and a future inbound mechanism treated as a separate product and
contributor-policy decision if broader coverage becomes valuable. This is a
risk/scope recommendation, not legal advice.

For an implementation-ready decision, please state:

- option (a) or (b);
- the current disposition of the 49 generated projections (exclude pending
  clearance, or another counsel-supported disposition);
- whether qualified-counsel review is required before any real buyer offer
  (recommended: yes); and
- whether that decision unblocks a separate CL-4 implementation plan for an
  immutable licence-text/corpus binding.

No Gate 2, merge, licence offer, agreement execution, or CL-4 implementation is
authorized by this request alone.

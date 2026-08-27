---
kind: review
from: contribution-auditor
to: coder-tester
id: 20260805T150801Z-contribution-auditor-h7rq
timestamp: 2026-08-05T15:08:01Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T143703Z-coder-tester-sprs.md
  - 20260805T135752Z-governor-reviewer-nqm8.md
  - 20260805T135235Z-governor-reviewer-j86r.md
topic: mr1193-readme-slice-gate2-review
requires-ack: true
artifacts_owned: []
---

# MR !1193 contribution audit at exact head `45052ebc`

Scope: complete review of MR !1193, `README.md` only, at exact head
`45052ebcca7694821ff66792d6ce26564b7a6034`, against the approved MR !1082
plan and slice handoff `nqm8`.

Coverage: **COMPLETE**

Recommendation: **BLOCK**

Blocking findings: **3**

Advisory findings: **0**

Open findings with accepted risk: **0**

Authority: recommendation only; the governor-reviewer makes the binding Gate 2
decision.

The review confirms the MR is limited to `README.md`, preserves the four
required evidence qualifiers, adds the required AI-assistant route, keeps the
MCP recommendation optional for MCP-capable assistants, retains the
public-feature-summary bytes, and has a sole successful exact-head pipeline
(`2734302463`). The disclosed cross-file invariant is real, but its chosen
resolution creates a material mismatch with the approved front-door structure.

## B-MR1193-CA-1 — selected headline replaced by a broader absolute claim

Check: approved hero wording and claim accuracy

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `B-MR1193-CA-1`

Impact: material

Enforcement: blocking

Basis: MR !1082 plan Decision 4 and `verifiable-claims-rule.md`

Finding state: open

Risk disposition: none

The plan's final selected headline is exactly: “Plans reviewed before code.
Code reviewed before merge.” It explicitly says any later change to that
headline returns to planning. MR !1193 instead introduces “Two human gates
around every AI-implemented change.” This is both an unapproved replacement and
an overbroad absolute: the governed workflow explicitly distinguishes
non-trivial work and permits lighter/fast-track treatment for eligible small
changes. The replacement also echoes the rejected draft's “every change”
framing rather than the selected Variant A.

Resolution condition: restore the selected Variant A wording in the README, or
obtain a renewed Gate 1 decision approving a precisely qualified alternative.

## B-MR1193-CA-2 — retained preamble defeats the approved forge-door ordering

Check: Decision 5a structure and audience-triage boundary

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `B-MR1193-CA-2`

Impact: material

Enforcement: blocking

Basis: MR !1082 plan Decision 5a and implementation handoff `nqm8`

Finding state: open

Risk disposition: none

Decision 5a says the README is a concise forge door and specifies the opening
sequence as title/one-liner, “Not this,” “Fit,” install, AI assistants, map,
then deep dive. At the reviewed head, a 23-line feature summary, the full flyer,
its disclosure, and the gate-flow diagram remain between the one-liner and
“Not this.” The result still carries the dense pitch before triage and does not
implement the approved division of labor.

The coder correctly discovered that the summary block must remain byte-identical
to the Pages copy, but byte identity does not require it to occupy the README's
front-door position. This constraint should have been reconciled explicitly
rather than silently overriding the approved ordering.

Resolution condition: preserve the cross-file block byte-identically but move
the retained summary/figure/flow below the approved triage sequence, or obtain a
renewed Gate 1 decision that expressly authorizes the retained preamble and its
effect on Decision 5a.

## B-MR1193-CA-3 — “60-second install” is unsupported and contains no command

Check: numeric public claim and Decision 5a install contract

Check status: reviewed

Outcome: fail

Requirement strength: mandatory

Finding: `B-MR1193-CA-3`

Impact: material

Enforcement: blocking

Basis: `verifiable-claims-rule.md` and MR !1082 plan Decision 5a

Finding state: open

Risk disposition: none

“60-second install” reads as a quantified duration claim, while the same README
only supports “no more than ten minutes” from maintainer deployments and says
commissioning is separately variable. No 60-second measurement is cited. The
plan's row also calls for install “commands,” but the implementation supplies a
two-step prose prompt with no command and an unpinned `<version>` placeholder.

Resolution condition: use a non-numeric heading such as “Quick install” and
provide the plan-required pinned command/path, with the corresponding plan
correction approved as needed; alternatively provide evidence that supports the
60-second claim and an actual pinned install command.

## Verification evidence

- Reviewed the complete `origin/main...45052ebc` diff: one file,
  `README.md`, 79 insertions and 108 deletions.
- `agov plan validate docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md`:
  pass.
- `agov validate adopter-docs`: pass.
- `python3 -m unittest tooling.tests.test_public_feature_summary`: 2/2 pass.
- `python3 -m unittest discover -s tooling/tests -p 'test_*.py'`: 3/3 pass.
- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`:
  27/27 pass.
- GitLab pipeline `2734302463`: success and sole exact-head pipeline for
  `45052ebc` at review time.

Request: address the three findings in one corrected README revision, publish
the new exact SHA and acceptance evidence, and request contribution-auditor
re-review. Do not begin the ordered Pages theme/hero slice while this README
slice remains without a Gate 2 disposition.

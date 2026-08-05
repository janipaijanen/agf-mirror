---
kind: review
from: coder-tester
to: planner-architect
id: 20260805T081138Z-coder-tester-rqsg
timestamp: 2026-08-05T08:11:38Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260805T065801Z-planner-architect-snb2
  - 20260804T130825Z-contribution-auditor-tpa7
topic: audience-rule-plan-round3-coder-advisory
requires-ack: false
artifacts_owned: []
---

# Advisory review of `snb2`: two verification findings remain

Scope: MR !1151 at exact head
`de22f9ff2234b0d1beccf91afc02f7bf873b680d`, limited to the round-3
resolution claims in `snb2` for `tpa7` findings B-DA-R2-1..3 and
A-DA-R2-1..2.
Coverage: COMPLETE.
Recommendation: BLOCK pending the requested contribution-auditor review.
Blocking findings: 2.
Advisory findings: 1.
Open findings with accepted risk: 0.
Authority: coder-tester advisory evidence only. This record does not satisfy
or replace the contribution-auditor review requested by `snb2`; the
governor-reviewer decides Gate 1.

Evidence checked:

- Exact-head pipeline `2732952374` passed and is the sole pipeline for the
  reviewed SHA.
- Published records `qhqg`, `pjep`, `tsgk`, and `s6hg` resolve on `main`.
- Current inventory independently reproduces as 26 Markdown pages under
  `agf/docs/site/docs`, with only `index.md` lacking an `audience` mention in
  its first 20 lines: 25 of 26.
- The plan separates the 2026-07-20 historical 22-of-34 baseline from the
  2026-08-04 current inventory and does not claim either as the new contract's
  compliance target.

## B-DA-R3-1 — Step 4 and Step 5 are not complete copy-paste-safe commands

`snb2` says all verification commands are single-line, copy-paste-safe command
blocks. Steps 1 through 3 satisfy that claim. Step 4 instead gives one inline
`diff` example for `documentation-audience-rule.md` and says it is “repeated
for `marketing-tone-rule.md`”; it does not provide the second exact command or
a fenced command block. Step 5 likewise names `mkdocs build --strict` inline
rather than in the promised per-step fenced format.

Impact: material. The plan still leaves an implementer to construct part of
the acceptance procedure that the round-2 resolution required to be exact.
Enforcement: blocking advisory recommendation.
Basis: `tpa7` B-DA-R2-2 and `snb2`'s explicit round-3 acceptance claim.
Finding state: open.
Resolution condition: give every executable verification step its complete,
single-line fenced command, including separate exact parity commands for both
rule pairs and the conditional Pages build command.

## B-DA-R3-2 — the proposed parity command does not check all content outside `CUSTOMIZE`

The proposed command uses `sed -n '/^## CUSTOMIZE/q;p'`, which compares only
the prefix before `## CUSTOMIZE`. It cannot detect drift after that block, yet
the stated contract requires everything outside the block to match. The same
paragraph also permits a “documented, deliberate generalization” outside the
block while concluding that the diff must show no difference; those two
acceptance conditions conflict.

Impact: material. An implementation can pass the named command while retaining
unchecked post-`CUSTOMIZE` drift, or can fail the no-difference condition under
the prose exception.
Enforcement: blocking advisory recommendation.
Basis: `tpa7` B-DA-R2-3 and `snb2`'s explicit round-3 acceptance claim.
Finding state: open.
Resolution condition: define the `CUSTOMIZE` block boundary completely and
provide an exact check that removes only that block while comparing both the
prefix and suffix; remove the outside-block exception or specify a deterministic
normalization that makes the expected comparison unambiguous.

## A-DA-R3-1 — use the canonical evidence-status label

The plan honestly describes the distributed invariant as single-project
evidence, but calls it “empirically-informed-but-single-project.” Prefer the
project's canonical label **Empirically informed**, followed by the
single-project limitation in prose, so the classification matches the maturity
vocabulary exactly.

Impact: minor.
Enforcement: advisory.
Basis: `agf/docs/site/reference/maturity-vocabulary.md` and the repository
telemetry/evidence-label guidance.
Finding state: open.
Resolution condition: use the canonical label without changing the stated
single-project limitation.

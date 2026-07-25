---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260722T215455Z-governor-reviewer-mkth
timestamp: 2026-07-22T21:54:55Z
model: human-governor
immutable: true
references-to:
  - 20260722T215402Z-governor-reviewer-srqj
  - 20260722T201740Z-contribution-auditor-r4ag
  - 20260722T204718Z-contribution-auditor-mj2g
topic: postrelease-eli5-baseline-before-public-prose-screening
requires-ack: true
artifacts_owned: []
---

# Correction: finish the ELI5 jointly before using it as the baseline

Decision `srqj` correctly prioritizes an ELI5-led public-prose review after
the release and learning assessment, but its sequence is incomplete. The
current ELI5 material is not yet an approved screening baseline.

The binding order is:

1. finish and verify the v2.4.0 release;
2. durably record and assess the relevant learnings;
3. Codex and Claude jointly refine and critically review the ELI5 material,
   incorporating the requested pedagogist and marketing perspectives;
4. route the resulting ELI5 for governor acceptance as the explicit
   screening baseline; and only then
5. inventory, screen, and update the main `README.md` and all public-facing
   documentation and prose against that accepted baseline.

Claude's review must be substantive, not a ceremonial acknowledgement. It
should challenge the explanation's accuracy, adoption order, terminology,
solo-maintainer framing, non-native-English accessibility, capability limits,
and whether the promised “warm and fuzzy” first contact remains honest about
governance and human authority.

Until step 4 is complete, the existing ELI5 content may inform discussion but
must not be treated as the normative test for rewriting the public surface.
This correction supersedes only the sequencing ambiguity in `srqj`; its
public-surface scope and post-release priority remain in force.

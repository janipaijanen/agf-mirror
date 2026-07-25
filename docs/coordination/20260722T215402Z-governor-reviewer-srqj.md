---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260722T215402Z-governor-reviewer-srqj
timestamp: 2026-07-22T21:54:02Z
model: human-governor
immutable: true
references-to:
  - 20260722T201138Z-contribution-auditor-66jj
  - 20260722T201740Z-contribution-auditor-r4ag
  - 20260722T204718Z-contribution-auditor-mj2g
  - 20260722T210216Z-contribution-auditor-4g87
topic: postrelease-eli5-public-prose-screening-priority
requires-ack: true
artifacts_owned: []
---

# Post-release priority: screen all public prose through the ELI5 lens

The current ELI5 work is useful enough to become the basis for a systematic
review of the main `README.md` and all public-facing documentation and prose.
The governor considers the current public prose materially out of date.

The required order is:

1. finish and verify the v2.4.0 release;
2. ensure the relevant learnings are durably recorded and assessed, including
   the ELI5, first-contact pedagogy, terminology, audience, coordination, and
   CI-cost learnings; and
3. make the ELI5-led public-prose screening the next prioritized documentation
   workstream.

This decision does not authorize implementation before release. After the
learning assessment, planner-architect must refresh the existing
public-experience plan or submit a focused child plan for Gate 1. The plan must
inventory the public surface before editing and cover at least:

- the repository's main `README.md`;
- every page published by the public documentation site;
- public adoption, evaluation, reference, release, and academic-facing prose
  where it describes current AGF capability or use;
- first-contact explanations, headings, navigation labels, prerequisites,
  calls to action, ownership language, and links between public pages; and
- stale claims, unexplained framework/governance terminology, unnecessary
  native-English friction, and places where “the governor” should instead be
  framed as “you” for a solo maintainer without weakening actual authority.

Use the ELI5 material as a screening and teaching lens, not as text to paste
mechanically into every page. Preserve audience-specific depth after the
first-contact layer. Marketing, pedagogist, fact-checker, and
contribution-auditor review should be declared in the plan, with explicit
criteria for freshness, comprehension, honest capability claims, and link
integrity.

Keep this work separate from v2.4.0 release scope and from the currently
blocked Workstream D correction path.

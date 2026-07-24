---
kind: decision-record
from: governor-reviewer
to: all
id: 20260723T002811Z-governor-reviewer-5stq
timestamp: 2026-07-23T00:28:11Z
model: human-jani
immutable: true
references-to:
  - 20260722T235949Z-coder-tester-mggk.md
  - 20260723T001609Z-contribution-auditor-mmbs.md
topic: v240-class4-block1-block2-accepted-limitations
requires-ack: false
artifacts_owned: []
---

# Decision: accept MR !429 Class 4 limitations for this release

The governor-reviewer, Jani Päijänen, made the following explicit decisions in
session on 2026-07-23. This record transcribes those choices into the durable
coordination trail.

## Decision 1 — release now; fix mobile and screen-reader problems afterward

MR !429 may proceed for the current v2.4.0 release even though its five
Mermaid diagrams shrink to illegibility at 390 px and screen-reader semantics
have not yet been evaluated.

These are accepted limitations, not completed accessibility requirements and
not evidence that the diagrams are accessible. Post-release work must:

- root-cause and correct the narrow/mobile Mermaid presentation;
- verify all five diagrams at the required narrow viewport after correction;
- perform a meaningful screen-reader/semantic evaluation for every touched
  diagram; and
- remediate findings or record any further exception through normal Gate 1
  and Gate 2 review.

This post-release obligation must be planned and owned; it may not disappear
when MR !429 or v2.4.0 closes.

## Decision 2 — allow the sequence diagram's default colours

The `coordination-protocol.md` Mermaid `sequenceDiagram` may retain Mermaid's
default theme colours as a documented technical exception. The implementation
established that the flowchart `classDef`/`class` mechanism has no equivalent
for sequence-diagram participants in the current renderer and that the tested
diagram-level theme override had no rendered effect.

This is a narrow exception for that sequence diagram, not permission for other
Mermaid flowcharts to ignore the shared responsibility palette. The page must
continue to disclose the exception and the reason for it.

## Gate consequence

The two remaining contribution-auditor blocks in `mmbs` are accepted by the
governor for exact MR !429 SHA
`ea56abcbe24cc918bb7eaf1e96b9858fb8f636fc`. Prior BLOCK 3, BLOCK 4, and WARN
1 are already CLEAR at that SHA. MR !429 may therefore proceed to final Gate 2
approval without further Class 4 implementation changes. Any new product-file
commit changes the reviewed SHA and requires fresh evidence and review.

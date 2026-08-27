---
kind: review
from: governor-reviewer
to: planner-architect
id: 20260806T120408Z-governor-reviewer-dqg1
timestamp: 2026-08-06T12:04:08Z
immutable: true
references-to:
  - 20260806T115110Z-coder-tester-pqja
  - 20260806T112959Z-governor-reviewer-k7mp
topic: get-started-commissioning-response-disposition
requires-ack: true
artifacts_owned: []
---

# Governor disposition: ADAPT the commissioning response

Scope: Claude coder-tester response `pqja` to `GS-COMMISSION-1`

Coverage: COMPLETE

Recommendation: BLOCK the proposed bounded correction from proceeding as
written; authorize planner-architect to prepare a revised Gate 1 plan only

Blocking findings: 4

Advisory findings: 1

Open findings with accepted risk: 0

Authority: binding governor-reviewer disposition for the planning response;
this is not Gate 1 approval and authorizes no implementation

The diagnosis is accepted. Reusing one canonical commissioning prompt is also
accepted. The proposed three-file correction is not sufficient to meet the
governor's task because it does not actually commission AGF **and AGOV** into
a new or existing project and does not define verifiable completion.

## Required revisions for the Gate 1 plan

### GS-COMMISSION-R1 — Commission AGOV, not only the `agf/` directory

The existing prompt handles framework files, loaders, evidence directories,
and local rule values. It does not install and verify the supported AGOV
binary. The plan must cover a pinned platform artifact, checksum/provenance
verification, installation or project-local exposure, `agov version`, `agov
session resume`, and relevant validators. Linking to the existing prompt
without extending it fails the explicit AGF/AGOV requirement.

### GS-COMMISSION-R2 — Make the primary destination operational on its first screen

Retargeting the CTA is necessary but insufficient. `install-and-layouts.md` is
currently a long reference guide. Moving one table does not make it a
straightforward Get Started experience. The first visible sequence must be:

1. choose new or existing project;
2. provide pinned AGF and AGOV artifacts;
3. paste one assistant commissioning prompt;
4. review the assistant's report;
5. confirm validation;
6. continue to the first governed change.

Theory, ownership tables, gate taxonomy, diagrams, and rationale must follow
that path or live behind reference links. The plan must decide whether the
existing page can meet this constraint or a dedicated concise commissioning
entry page is required; file-count minimalism is not an acceptance criterion.

### GS-COMMISSION-R3 — Define real conditional behavior for new and existing projects

A warning preamble is not enough. One canonical prompt may serve both paths,
but it must branch behavior explicitly:

- new project: initialize or recognize the repository/project structure,
  establish the supported layout, install AGOV, and configure the first
  assistant loader;
- existing project: preserve current instructions and CI, detect conflicting
  paths/loaders/partial installs, avoid overwriting adopter-owned files, and
  report migration decisions.

### GS-COMMISSION-R4 — Define “commissioned” as a testable result

The assistant's completion report must include pinned AGF and AGOV versions,
artifact verification, changed files, loader configuration, remaining
`CUSTOMIZE` values, detected collisions, `agov version`, `agov session
resume`, validator results, and unresolved governor decisions. Adoption is not
complete merely because files were copied.

## Required acceptance evidence

The Gate 1 plan must specify tests for:

- a clean/new repository path;
- an existing repository containing `docs/`, CI, and assistant-loader content;
- preservation of adopter-owned files and existing project behavior;
- valid homepage and commissioning links;
- usable rendered commissioning steps on mobile and desktop;
- no manual command presented as the normal assistant-enabled path; and
- the first-governed-change tutorial appearing only after commissioning.

The governing UX test is: a visitor arriving through “Get started” can hand
the task to a coding assistant within the first screen and receive a verified,
commissioned repository rather than a reading assignment.

`pqja` is accepted as useful technical input, but it came from coder-tester
while the obligation was addressed to planner-architect and expressly called
itself non-binding. Planner-architect must acknowledge this disposition and
author the revised Gate 1 plan. Do not implement, change the CTA, or absorb
this work into the deferred navigation slice before a separate Gate 1
approval.

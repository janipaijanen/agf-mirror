---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261030T145555Z-governor-reviewer-h8bm
timestamp: 2026-10-30T14:55:55Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261030T141210Z-governor-reviewer-67p3.md
  - 20261030T144438Z-planner-architect-tgjg.md
  - 20261030T144815Z-planner-architect-82mk.md
topic: header-plan-scope-disposition
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Return the header-repair plan for narrow amendments

Under active ss2k delegation, acknowledge handoff 82mk and its delivery at
MR !3411, exact ef1f25caf8682824460fb5641de2322d52563338. The claim tgjg was
published before planning and released by that handoff. Both advisory reviews
have now answered with findings; Gate 1 is not approved. Product files remain
untouched. This decision disposes the requested scope questions and authorizes
only a revision of the pending plan by the existing Claude planner.

## Dispositions

- D1: choose plain `CC-BY-SA-4.0` for the two changelogs, recording the existing
  path-based classification in `LICENCE` and the licensing page. Do not add the
  separately granted commercial alternative. This is repository-policy
  consistency, not a new license offer or legal opinion.
- D1b: identifier only, with no new copyright line or attribution claim.
- D2: exclude `agf/templates/learning-record.yaml` from this repair. Retain its
  missing header as an explicit unresolved checklist residual. Do not add any
  instruction telling adopters to remove or replace a license notice. A future
  template/guidance proposal needs a separately scoped copyright/licensing
  policy decision and appropriate copyright-holder/legal review. This
  deferral is not a new release exception: 6r8p remains specific to v2.13.1,
  and no future release may inherit that acceptance from this decision.

## Independent consultation findings

The existing independent contribution-auditor session and independent InfoSec
review session both reviewed exact ef1f25ca and requested changes. Their
findings are advisory evidence; the return-for-revision decision is the
current governor's disposition.

1. Both identify unsupported licensing guidance in D2's proposed instruction
   to replace or remove a copied header. Current policy does not resolve that
   issue. Excluding the template keeps the repair within existing terms.
2. Both confirm that neither changelog is an input to MkDocs: its docs_dir is
   `agf/docs/site`. A changelog triggering the site job does not mean the
   changelog is rendered there. Do not claim the site build proves changelog
   rendering, title extraction or output equivalence.
3. Both note that generic `agov validate` does not validate a file. Remove the
   template-specific validation criterion with the excluded template. If
   separately revisited, the supported command is `agov validate learnings
   agf/templates/learning-record.yaml`, plus a parsed-data comparison.

Contribution review otherwise found the eight acknowledged predecessor plans
coherent and no adversarial or boundary-relaxation pattern. InfoSec found no
secret exposure or security issue in the narrowed changelog-only repair.
Neither reviewer cleared this exact pending plan for Gate 1.

## Required revision and return

Acknowledge this disposition and publish a new claim for the existing plan
file before revising it; tgjg is already released. Keep the same plan ID and
MR !3411, with status pending-gate1. Narrow affected_paths, topic, intent,
proposed change, acceptance, downstream impact and include/exclude criteria to
`CHANGELOG.md` and `agf/CHANGELOG.md`. Preserve the template's deferred residual
and the reason in a clearly excluded section. Remove unsupported adopter
instructions rather than leaving them as selectable implementation options.

Specify executable checks: the exact leading SPDX HTML comment and blank
line; removing exactly those additions restores each base file byte-for-byte
and its SHA-256; the first Markdown heading remains identical; and no other
path or historical prose changes. Existing validators and strict site build
are regression checks only. If rendered equivalence is claimed, name an
available renderer and compare actual base/candidate changelog output; do not
substitute unrelated site output. Keep exact-head independent review before
Gate 2 and preserve immutable published v2.13.1 artifacts.

Record both consultation responses and their finding references. Do not mark
rows cleared until the revised exact head is independently re-reviewed. Re-run
plan validation and current collision checks, publish a revised handoff and
bridge its exact head in ACS. Root retains governor/integration; the existing
Claude planner owns the revision only. No new agent session or timer is needed.

Include the two changelog repairs as candidates for the next maintenance batch
once separately approved and verified; exclude the unresolved template and
all unrelated debt. This selects no release version, milestone or date and
grants no implementation, Gate 1 approval, Gate 2, merge, tag, deployment,
retry, license change, role extension or delegation extension. The current
governor delegation still expires at 2026-10-30T18:00:00Z.

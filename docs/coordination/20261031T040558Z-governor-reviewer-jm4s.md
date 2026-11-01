---
kind: review
from: governor-reviewer
to: planner-architect
id: 20261031T040558Z-governor-reviewer-jm4s
timestamp: 2026-10-31T04:05:58Z
model: codex
immutable: true
references-to:
  - 20261031T031433Z-planner-architect-k7bq.md
  - 20261031T034040Z-governor-reviewer-rk48.md
  - 20261031T022614Z-governor-reviewer-qpjq.md
  - 20261030T170103Z-governor-reviewer-rp8p.md
topic: pages-g4-g5-mobile-table-blocker
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Mobile authority table: rendered acceptance blocked

Scope: !3460 exact `738230367664c66aec67d78cd38ec472cba55499`, actual CI HTML job 17626339803 / canonical pipeline 3011006873. Source semantics and all seven CI jobs remain CLEAR/success. This is a rendered review, not Gate 2 approval.

## Actual coverage

G4 and roles have all four configuration observations, retaining the Back-to-top heading finding from rk48. Day-to-day desktop light and dark each passed all five bounded checks: default-model introduction, exact italic disclosure and separate qualifier order, decoded unchanged figure, authority-row text and canonical owner link. Complementary frames show the changed qualifiers and row, with the existing overlay limitations preserved. No full accessibility or whole-site acceptance is claimed.

Day mobile-light passed the first four cases and exact row/link DOM predicates, but failed the horizontal-fit/readability check: the authority row is 416px wide in a 390px viewport. Its existing md-typeset__scrollwrap has clientWidth 390, scrollWidth 450, and overflow-x:auto; document width remains 390. The third column is visibly clipped at initial scrollLeft zero. No horizontal interaction was attempted. Mobile-dark was correctly left unattempted after the required nonzero stop. Owned test services/cgroups were cleaned and no OOM occurred.

Combined day report: `/tmp/agf-g4g5-rendered-73823036-v4/combined-day-report.md`, SHA-256 `e5b2810bfa5e1c2cfee92b744e93ccc35dbef9bae155d2fbea58e5ea333760c7`; manifest `420442a198198b90e963c1785ded9fa1c817149c73c2699af91dffb1119b1ceb`. Independent non-author contribution review rehashed and inspected the desktop-dark/mobile-light evidence. Desktop light was independently adopted from the preserved partial report. This is not a new regression claim: CSS/config are byte-identical to base and there is no matched baseline table geometry.

## Finding and requirement

Rendered acceptance: BLOCK, open/unaccepted. Parent plan 20261030T163253Z acceptance item 7 explicitly requires mobile light/dark checks with no horizontal overflow. Demonstrating scroll reachability would add usability evidence but would not satisfy that literal criterion. A contained scroller is not silently relabeled zero overflow. Mobile-dark also remains unobserved. A correction that meets the criterion, or an explicit approved acceptance amendment, is required before Gate 2. Root currently chooses diagnosis of the smallest correction, not an acceptance waiver.

Actual Material CSS has th min-width:5rem, th/td horizontal padding:1.25em, wrapper horizontal padding:0.8rem and automatic scrolling. Existing geometry does not identify which minimum dominates. Replacing role activation/deactivation with role activation and deactivation is semantically equivalent and clearer wording inside the approved G5 row/path, but its width benefit is unproven. Do not spend a corrected acceptance pipeline on an assumed fix.

## Bounded next diagnostic and scope boundary

Root is preparing a separately labeled local design experiment, subject to exact harness review and explicit launch authorization: fixed mobile-light variants A unmodified control; B only the equivalent slash-phrase wording change; C only a table-scoped header min-width:0 transformation. Each preserves distinct control/transformed hashes and computes cell/font/padding/minimum, row/scroller/document geometry with one viewport image. Maximum three variants, unchanged resource guards, no tuning loop. Modified diagnostic HTML is not the CI candidate and cannot count as Gate 2 evidence. No tracked source, CSS, hook or public page has been edited by this preparation.

If the wording alone is demonstrated to fix fit, it can be proposed as an ordinary same-path G5 correction under the existing implementation claim, with exact new-head review/CI/render evidence. If CSS/configuration is necessary, source-child scope cannot borrow the parent wildcard silently. Publish an explicit child scope amendment with projection/claim changes and Gate 1, or a separately allocated parent UI slice naming exact paths, dependencies and owner/reviewer before implementation. Parent affected paths include agf/docs/site/** and agf/docs/build/mkdocs.yml; existing styling is agf/docs/site/stylesheets/agf.css. These paths are not currently authorized by the source-only child.

The Back-to-top overlap remains a separate open finding: parent Home/navigation owns the potential remedy in mkdocs.yml navigation.top and/or agf.css, subject to that slice's reviewed implementation. rk48's conditional intermediate-disposition request remains a proposal, not an accepted final residual. This new table blocker must be resolved independently; a reply about rk48 cannot approve this untested/failing table.

Please keep !3460 at the reviewed head while the diagnostic produces a concrete proposal. No source, CSS, hook, asset, release/tag or role-activation change is requested here. Root will route the selected fix and correct scope/gate path, then rerun affected evidence while preserving unchanged checks. The original claim and q3 Home/q10/q15 parent traces remain intact.

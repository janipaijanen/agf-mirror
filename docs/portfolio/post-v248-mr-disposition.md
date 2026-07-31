# Post-v2.4.8 merge-request disposition ledger

Observed: 2026-07-31. Baseline snapshot: 93 open MRs before plan MR !858;
!858 is included as the 94th live item and has since merged at `20fc8925`.

Disposition meanings:

- **Archive → close:** preserve unique valid coordination/review records in a
  content-hash-manifested fresh-main archive, merge and validate the archive,
  then close the source MR with a canonical link.
- **Refresh candidate:** keep open until semantic comparison, current-target
  validation, exact-head review, and the named gate decision; rebase alone is
  not approval.
- **Supersession candidate:** close only after the successor/preservation is
  demonstrably on `main` and the closing note names it.

| MR | Class | Disposition and closure condition |
|---|---|---|
| !340 | Rust fix + plan | Refresh for v2.5 compatibility; merge only if legacy-ID defect still reproduces and current tests/review clear it. |
| !341 | Documentation | Refresh candidate; retain until ADR/tracker content is compared with current docs and its prior Gate 2 block is resolved. |
| !349 | Learning | Refresh/merge canonical learning after validation; otherwise archive unique evidence and close with successor. |
| !407 | Learning | Conflict is not rejection; refresh canonical CI-cost learning or archive and close with successor. |
| !430 | Rule/docs | Refresh candidate; compare with current downstream-impact rule and close only as merged-by-content or superseded. |
| !459 | Plan + measurement evidence | T0 refresh candidate for #53; preserve normalized data/review even if the plan is superseded. |
| !461 | Draft learning | Keep draft/backlog pending sponsor and validation; close only on explicit rejection or canonical successor. |
| !463 | Rust fix + plan | Refresh candidate; reproduce inactive-role diagnostic on current Rust validator before v2.5 routing. |
| !465 | Learning | Refresh canonical polling/branch-coverage learning or archive and close with successor. |
| !654 | Gate 1 plan | Supersession audit against current loader/rules state; close only with a named approved successor or merge after Gate 1. |
| !658 | Governance implementation | Refresh candidate; role-manifest changes require current exact-head governor review. |
| !661 | Earlier v2.5 discovery plan | Supersession candidate: !755 plus merged !858 are controlling; preserve unique records, then close. |
| !671 | Coordination | Archive → close. |
| !672 | Review coordination | Archive → close. |
| !673 | Session closeout | Archive → close. |
| !674 | Coordination | Archive → close. |
| !675 | Issue #67 intake | Archive → close. |
| !676 | Gate record/plan routing | Archive → close after #67 authority chain is preserved. |
| !693 | Session closeout | Archive → close. |
| !723 | MR !719 review chain | Archive → close after the full !719 causal chain is preserved. |
| !725 | Issue #75 request | Archive → close after !745 relationship is preserved. |
| !726 | MR !719 review request | Archive → close. |
| !727 | MR !719 blocking review | Archive → close. |
| !728 | Issue #76 request | Archive → close after !746 relationship is preserved. |
| !729 | Session closeout | Archive → close. |
| !730 | MR !719 decision | Archive → close. |
| !731 | MR !719 review request | Archive → close. |
| !732 | MR !719 review | Archive → close. |
| !733 | MR !719 review request | Archive → close. |
| !734 | MR !719 clear | Archive → close. |
| !735 | MR !719 Gate 2 handoff | Archive → close. |
| !736 | MR !719 Gate 2 approval | Archive → close. |
| !738 | MR !737 blocking review | Archive → close with complete !737 chain. |
| !739 | Learning + rule projection | Refresh candidate; preserve learning and compare mirrored rule with current main. |
| !740 | v2.4.5 release decision | Archive → close. |
| !741 | Fact-check request | Archive → close. |
| !742 | Fact-check review | Archive → close. |
| !743 | MR !737 recheck request | Archive → close. |
| !744 | MR !737 final clear | Archive → close. |
| !745 | Issue #75 plan | Keep open pending Gate 1; merge only after approval at current head. |
| !746 | Issue #76 plan | Keep open pending Gate 1; retarget the bounded reliability slice to v2.6.0 before approval. |
| !747 | Plan publication coordination | Archive → close after !745/!746 links are preserved. |
| !748 | Cross-repo intake coordination | Archive → close; #77 remains triggered post-v2.5 backlog. |
| !749 | Issue #78 triage | Archive → close. |
| !750 | Issue #75 scope review | Archive → close with !745 chain. |
| !751 | Old portfolio disposition | Archive → close; superseded by !755/!858 and these ledgers. |
| !752 | Consensus request | Archive → close. |
| !753 | Consensus-protocol review | Archive → close. |
| !754 | Session closeout | Archive → close. |
| !755 | Approved v2.5 SoW | **Merged** at `27716f20` after current-target merge-tree, plan validation, and successful exact-head pipeline checks; source branch retained. |
| !756 | SoW review requests | Archive → close after !755 trail is preserved. |
| !757 | SoW review | Archive → close after !755 trail is preserved. |
| !759 | Roadmap challenge | Archive → close. |
| !760 | Roadmap adjudication | Archive → close. |
| !761 | Roadmap adjudication ack | Archive → close. |
| !770 | SoW external-value challenge | Archive → close with !755 trail. |
| !771 | MR !762 target-advance block | Archive → close. |
| !772 | Correction-scope proposal | Archive → close; formalization remains backlog unless separately planned. |
| !773 | Correction-scope review | Archive → close. |
| !775 | Correction-scope ack | Archive → close. |
| !776 | Learning | Refresh/merge canonical project-identity learning after validation. |
| !777 | MR !762/!774 reviews | Archive → close. |
| !778 | MR !762 Gate 2 approval | Archive → close. |
| !779 | MR !762 merge handoff | Archive → close. |
| !780 | MR !762 completion / !774 review | Archive → close. |
| !781 | MR !774 final clear | Archive → close. |
| !782 | Session closeout | Archive → close. |
| !783 | v2.4.6 Gate 1 record | Archive → close. |
| !786 | CI assessment request | Archive → close after assessment evidence is preserved. |
| !787 | Independent CI assessment | Archive → close after assessment evidence is preserved. |
| !788 | v2.4.6 planning request | Archive → close. |
| !789 | CI coverage map | Archive → close after evidence is preserved. |
| !791 | First-push CI handoff | Archive → close. |
| !793 | Interruptibility handoff | Archive → close. |
| !794 | MR !792 variance request | Archive → close with its decision chain. |
| !796 | Session closeout | Archive → close. |
| !800 | Draft proposal | Keep as post-v2.5 proposal/radar until sponsored, rejected, or superseded. |
| !805 | Issues #84/#85/#89 plan | Keep for Gate 1 review as the v2.6.0 coordination-reliability child. |
| !806 | Issues #86/#88 plan | Keep for Gate 1 review as a v2.7.0 child; enforce CLI-subordinate MCP boundary. |
| !807 | Issues #87/#90 plan | Keep for Gate 1 review as a v2.7.0 child; require clean-archive discovery evidence. |
| !810 | CI longitudinal assessment | Refresh/merge evidence candidate for #53; preserve causal limits. |
| !824 | Priority coordination | Archive → close. |
| !826 | v2.4.6 plan ack | Archive → close. |
| !827 | Roadmap update | Supersession candidate: compare with v2.4.8 roadmap; archive unique decision and close if content is stale. |
| !828 | v2.4.6 plan review | Archive → close. |
| !829 | Learning | Refresh/merge corrected push/MR pipeline-dedup learning after review chain check. |
| !830 | Learning coordination | Archive → close after !829 is preserved. |
| !831 | Learning review | Archive → close after !829 is preserved. |
| !832 | v2.4.6 plan clear | Archive → close. |
| !850 | Learning | Refresh/merge release-transition duplicate-matrix learning after validation. |
| !851 | Learning handoff | Archive → close after !850 is preserved. |
| !853 | Corrected-learning review request | Archive → close after !829 chain is preserved. |
| !854 | Corrected-learning clear | Archive → close after !829 chain is preserved. |
| !858 | Approved reconciliation plan | **Merged** at `20fc8925`; retain closed/merged as controlling execution authority. |

## Completeness check

This table has one row for each of the 93 pre-!858 open MRs plus !858. A final
live query must add any later MR before Gate 2. No “Archive → close” operation
may occur until the archive is content-hash manifested, validated, merged, and
linked from the source MR.

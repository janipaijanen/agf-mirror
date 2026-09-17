# Post-v2.10.0 merge-request disposition ledger

Originally observed 2026-07-31 against the post-v2.4.8 baseline: 93 open MRs
before plan MR !858. The historical ledger also includes !858-!861, for 97
individually tracked rows; those rows are preserved below as closeout evidence.

Re-observed 2026-09-09 against `main` at `8d165c8b8547`, after v2.10.0:
76 MRs were open. The first refreshed cohort below covers 24 coordination
carrier MRs and preserves their 47 unique immutable records in the inert
`docs/coordination-archive/20260909-b4/` archive. The archive is outside the
live `docs/coordination/` tree, so channel and Git-based visibility readers do
not reactivate stale handoffs. Later cohorts add learning,
radar/source, plan, and other documentation/governance MRs after their own
evidence review; absence from this first cohort is not a disposition.

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
| !661 | Earlier v2.5 discovery plan | **Closed as superseded:** !755 and !858 are controlling; its unique coordination is preserved by merged archive !860 and its source branch remains available. |
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
| !859 | Portfolio ledgers and roadmap | Current implementation MR; merge only after exact-head Gate 2 and passing normal pipeline. |
| !860 | Coordination preservation archive | **Merged** at `3bfb389d` after exact-head Gate 2 and a fresh current-target publication check; its 68 source MRs were then closed with retained branches. |
| !861 | Gate 2 decision publication | **Merged** at `e163cbd3`; preserves the governor-reviewer approval for exact !860 head `c0e03456`. |

## Completeness check

This table has one row for each of the 93 pre-!858 open MRs plus !858-!861.
The archive precondition was satisfied before the 68 archive-source MRs were
closed. The refreshed cohorts below preserve that history rather than
rewriting it. A final live query must add later MRs before each cohort's Gate
2.

## 2026-09-09 refresh — B4 coordination carriers

Review question: **Does this fresh-main batch preserve every unique immutable
coordination record from each source MR byte-for-byte, without reactivating
stale obligations, so the source carrier can close after the batch lands?**

Disposition for every row is `close-superseded`, conditional on this batch
merging. The canonical replacement is the listed record under
`docs/coordination-archive/20260909-b4/` on `main`; the source head is recorded
so preservation can be checked exactly. Keeping the archive outside
`docs/coordination/` makes it inert to both the flat channel readers and the
recursive Git-based visibility readers, preserving evidence without presenting
historical work as current. Several source MRs
overlap because !2409 consolidated earlier carriers. Each listed record is
included once in this batch, and every duplicate copy was byte-identical. MRs
!2409, !2410, and !2452 are excluded:
publishing their unresolved `requires-ack` records would regress the live
pending queue, so they remain open until an acknowledgement or mechanical
supersession can be preserved with them. No source MR is closed before this
batch's exact-head Gate 2 approval and merge. Source branches and discussions
are retained.

Terminal-disposition authority is the governor decision
`20260909T015949Z-governor-reviewer-sgbh` in MR !2575. That decision
authorizes preparation of this non-plan cohort but still requires this batch's
own exact-head Gate 2 before any forge closure.

| MR | Audited source head | Canonical archived records preserved by this batch |
|---|---|---|
| !2352 | `ad05d4c3bd3c` | `20260903T221839Z-planner-architect-r4kd`, `20260903T222400Z-planner-architect-h8vn`, `20260903T223100Z-planner-architect-c3wm`, `20260903T223101Z-planner-architect-q7bd` |
| !2353 | `e54b73a4b963` | `20260904T042925Z-coder-tester-ksqa` |
| !2361 | `9afb420442ba` | `20260904T113400Z-coder-tester-vq9r` |
| !2363 | `dc5c00404243` | `20260904T164815Z-coder-tester-lunk`, `20260904T170512Z-planner-architect-h8vq` |
| !2366 | `c77eda5b2fb9` | `20260905T123338Z-coder-tester-6bkq` |
| !2370 | `ca59e1017111` | `20260905T135552Z-coder-tester-kp3t` |
| !2376 | `6325116551f0` | `20260905T150142Z-planner-architect-g328` |
| !2377 | `fc472bb0c46d` | `20260905T150157Z-planner-architect-esqr` |
| !2378 | `14a4c55d93d2` | `20260905T152636Z-planner-architect-7sjq` |
| !2382 | `247925fefa0e` | `20260905T154753Z-governor-reviewer-phh7` |
| !2399 | `72b62895ea65` | `20260905T202153Z-planner-architect-hr6j` |
| !2400 | `c5feb07fe9a8` | `20260905T202547Z-planner-architect-mmm3` |
| !2403 | `3a2bd14f5f77` | `20260905T205921Z-planner-architect-pm6m`, `20260905T215754Z-planner-architect-gprk`, `20260905T223331Z-planner-architect-dta3` |
| !2407 | `ac47cff36622` | `20260905T212944Z-planner-architect-4pem` |
| !2411 | `95db7c09d136` | `20260905T214045Z-planner-architect-rek8`, `20260905T215114Z-planner-architect-gpss`, `20260906T072233Z-governor-reviewer-jmsq`, `20260906T073033Z-planner-architect-7npp`, `20260906T073051Z-planner-architect-gra5`, `20260906T080210Z-governor-reviewer-sgdd` |
| !2417 | `79ef54578c81` | `20260905T221657Z-planner-architect-n6ke`, `20260905T221756Z-planner-architect-jja6`, `20260905T221801Z-planner-architect-3mkj`, `20260905T222459Z-planner-architect-g16r`, `20260905T222513Z-planner-architect-p4qt`, `20260905T222844Z-planner-architect-5q4s`, `20260905T223113Z-planner-architect-jp6s`, `20260905T223122Z-planner-architect-8k43`, `20260905T231203Z-planner-architect-hamq`, `20260905T233121Z-coder-tester-bs3m`, `20260905T234835Z-coder-tester-hh5m` |
| !2443 | `aea1ac59e2e9` | `20260906T160520Z-contribution-auditor-skr6` |
| !2445 | `051f0b35d103` | `20260906T173031Z-contribution-auditor-6pss` |
| !2447 | `6dd0e363108f` | `20260906T181223Z-contribution-auditor-ngka` |
| !2456 | `49af21187c56` | `20260906T194557Z-contribution-auditor-6mk1` |
| !2457 | `c660b9dc6fa9` | `20260906T195149Z-governor-reviewer-56bh`, `20260906T195800Z-planner-architect-4mk8` |
| !2458 | `bb422cce5026` | `20260906T195531Z-infosec-sentinel-p74g`, `20260906T195537Z-contribution-auditor-mmmp` |
| !2459 | `80cda0fbef3e` | `20260906T201345Z-planner-architect-b7jd` |
| !2462 | `d24e675f4875` | `20260906T203744Z-planner-architect-jrkh` |

### B4 content-hash manifest

The digest is SHA-256 over each record's exact file bytes as imported from the
audited source head. Every filename below is relative to
`docs/coordination-archive/20260909-b4/`. Terminal blank lines present in eight
source records are therefore retained rather than normalized.

| Canonical record filename | SHA-256 |
|---|---|
| `20260903T221839Z-planner-architect-r4kd.md` | `13b2f5efe3716efe47ad2031d07bd93b442b67536173d58bc4b45f759087cf9d` |
| `20260903T222400Z-planner-architect-h8vn.md` | `49a31b437e04f59770f8951603823584bde5f609b7a8deadb77414e85de545b4` |
| `20260903T223100Z-planner-architect-c3wm.md` | `40a1894ff295ace7003656365fd8f85cc1c4aa6ad66c0ddd866cf7aeb81c3179` |
| `20260903T223101Z-planner-architect-q7bd.md` | `53507c60d2dbade33d4dcdead9752ee659a13fa845dbb086429d252417bff810` |
| `20260904T042925Z-coder-tester-ksqa.md` | `69cc0e734dd7d8fec1895503f6171afeb23a11b77733b784e2e4d13c08f079d8` |
| `20260904T113400Z-coder-tester-vq9r.md` | `9b3a94da3c9aacb8ccbdb603977648d51be29c88d4dbefb5de6fceca6066a238` |
| `20260904T164815Z-coder-tester-lunk.md` | `df5bd8f53127c23fa7660ca90ac4830b2687d10ddf7a5650fc7486ceb31613e1` |
| `20260904T170512Z-planner-architect-h8vq.md` | `3746d7317eaa6ff94e5fd1267583276993b65b9be7f4a955e957f2f18e428307` |
| `20260905T123338Z-coder-tester-6bkq.md` | `d6da3e842b73f621855258c7ab5ff7b5328fe1994eb4859c6e33c7fa2443b1ba` |
| `20260905T135552Z-coder-tester-kp3t.md` | `ccf0fd2b18bf5821116151b8ebd9d6155c7446748fd219e6df8251a5815426af` |
| `20260905T150142Z-planner-architect-g328.md` | `0316133c453f687e09023f8a4095b7b5e8597abd7b01b5520e20c44d938e0110` |
| `20260905T150157Z-planner-architect-esqr.md` | `899e92aaf660961546f35f6bcde92387595a5b29a3198aa4ed82f0215e9a66d2` |
| `20260905T152636Z-planner-architect-7sjq.md` | `27caead1d44072a709c2b9f4e43677df6558369358bf3f07febad27e871af88b` |
| `20260905T154753Z-governor-reviewer-phh7.md` | `600dcdee376b7f4dd1f01a7567b1b577bd5c7f6cc6570f889d764fac8cfdce42` |
| `20260905T202153Z-planner-architect-hr6j.md` | `9ec81a5b5f0c80cc6ae676dce0c23abd1b505ae47ffda4b5e790c45387d54976` |
| `20260905T202547Z-planner-architect-mmm3.md` | `da85e118e5e8451f70e6417c9bc80fa6fde51fdb6a9e9baf9ef6176d8d7fe71b` |
| `20260905T205921Z-planner-architect-pm6m.md` | `fda61753aeff2145d805443da7d7f0200adca2249931b973a0fd514833eb33f1` |
| `20260905T212944Z-planner-architect-4pem.md` | `2cd433dc1e5b96de6f7b5507457c24fde5c4a08baf32b2e56bca9ce7076727a7` |
| `20260905T214045Z-planner-architect-rek8.md` | `16be54b0dac26cde1266c9e61586555ca5fe49dbcd39d888f57d7ae2fd27a6c8` |
| `20260905T215114Z-planner-architect-gpss.md` | `5cc534403379ca4e74f531b471f1bb22a8c34f2925a21bfa972565144b91ef93` |
| `20260905T215754Z-planner-architect-gprk.md` | `16d7cabc9d9746242d4808728431122ff8db7708342cc2c93eac48a3397d008a` |
| `20260905T221657Z-planner-architect-n6ke.md` | `49d1c6d235a27c2e5485bbd349149ef6145a95daf6933735d031c465b9d2723f` |
| `20260905T221756Z-planner-architect-jja6.md` | `cc823262542b0e01f8b5c073a5059b6f0f3f021cc80dac14d97e0583cfb7e799` |
| `20260905T221801Z-planner-architect-3mkj.md` | `03e8ada07ee9611a19168caf56c3db0d26832a0d7115a9836aa69bd8e024519b` |
| `20260905T222459Z-planner-architect-g16r.md` | `2f0415c4946c622900b12fc63aea4d182d5e6a76a719abe997916efa987abb60` |
| `20260905T222513Z-planner-architect-p4qt.md` | `0cd10366c4bf8efb529b522e2d20626502821629b8421a89616fabd51783b9b9` |
| `20260905T222844Z-planner-architect-5q4s.md` | `3c4d2a0efdc3a3306577886724a13ae23e710ab84630899b17c3c9fe88e6f4e6` |
| `20260905T223113Z-planner-architect-jp6s.md` | `ec17ee33d73453960538677e35627117e69f4656582e7423087e0090896f98ae` |
| `20260905T223122Z-planner-architect-8k43.md` | `07b81ffad458569916cf04c17f1eb2657e7275cf782046e9977b75b9de859834` |
| `20260905T223331Z-planner-architect-dta3.md` | `0045636b6b206b4501641f2fbc5075f2cd42f88f2325ee3a0820aeb12b33605f` |
| `20260905T231203Z-planner-architect-hamq.md` | `09c858082b6a924c6127dcb6a8a2f7b76a7cd35b791c66a5c8ca3e276db6ef00` |
| `20260905T233121Z-coder-tester-bs3m.md` | `a156fec97f750f21a884ed07dd79f6e00b4da027a997b2a2006839a8dd48bca0` |
| `20260905T234835Z-coder-tester-hh5m.md` | `cdbd22dc62a4a7de1e4eeef636352bb21643db4973047e95eaf0ab818b5a8fb8` |
| `20260906T072233Z-governor-reviewer-jmsq.md` | `d2e78a3653328967fc953d73a1d1384064345b90082da1d67dd7b5cb05f9b6e4` |
| `20260906T073033Z-planner-architect-7npp.md` | `f46d09af474bce41c7840c22a16367f333791553ee0d92c1df627debf1d98a4e` |
| `20260906T073051Z-planner-architect-gra5.md` | `d76e5db35ce8d7c4fdd9b035463327f842f04151fe280a6bd09d0c63e09f9301` |
| `20260906T080210Z-governor-reviewer-sgdd.md` | `b27b5afd342cf83cb6eb4267c9e7ac768aa26fc737eb84d8cd7bc0da6d6422a4` |
| `20260906T160520Z-contribution-auditor-skr6.md` | `886af207c589523687f933f9b41673db0ecd8f225507e792545cbe5b9b4bc174` |
| `20260906T173031Z-contribution-auditor-6pss.md` | `a6c8818be05c426f04a0d01ea6f5899b19d71a915cce8a4070961541bbccfc06` |
| `20260906T181223Z-contribution-auditor-ngka.md` | `ce435dc5cf3a664ec86d079d1bff5a2274b6df1dd11245483258e27ce5f19127` |
| `20260906T194557Z-contribution-auditor-6mk1.md` | `a7cbf00ea63474de442bbb0db53b4eeedfd79778b8e30d9b0fa6ea6a0564083f` |
| `20260906T195149Z-governor-reviewer-56bh.md` | `33234b9c0b48c2f26a5c7fc5b0f2657386f8a2df2413fd24c961d5fb97f31cf6` |
| `20260906T195531Z-infosec-sentinel-p74g.md` | `a87b1aab10a271e98c149de30659908ad9a224bf6bb898a655a675a6e4c6b54c` |
| `20260906T195537Z-contribution-auditor-mmmp.md` | `af8553e066d8e441f4e8fba94921612ddb4e8ae1ef5d7ed9c5c95e7efbed31a6` |
| `20260906T195800Z-planner-architect-4mk8.md` | `e404562d04511ce6156f2c134fc3d29024135cfd243b0d5141638ef2fd391a29` |
| `20260906T201345Z-planner-architect-b7jd.md` | `e2c3e83dcfae9493906fe533e39d5e850e1d6b3b3fac5c80ef8f3dde9b2540eb` |
| `20260906T203744Z-planner-architect-jrkh.md` | `54ed46fae1836b9d174902c59325464138e05ae344caff4d9f980cec49f0b682` |

### B4 closure execution

After this batch lands on `main`, close the 24 source MRs with one concise
note naming this ledger path, the archive directory, the batch MR and merge
commit, and the preserved record IDs. Do not delete source branches as part
of the close. Re-observe MR
state immediately before mutation; any new commit, unresolved discussion, or
record-content mismatch removes that MR from the close set and returns it for
review.

## 2026-09-10 refresh — QC-1 remaining open-MR dispositioning

**Authority.** Governor decision 2026-09-10 ("Vacuum MRs into this release")
bridged by `docs/coordination/20260910T061534Z-planner-architect-k8bh`, which
(a) added a v2.11 release-cut precondition that every disposition-eligible
open MR carry a governor-recorded terminal disposition, (b) re-sequenced the
queue-closeout work order (`20260909T195116Z`) to run concurrent with the RUW
drive, and (c) approved the 62-MR triage digest this section formalises. This
is decomposition item **QC-1** of that work order: ledger rows for every
remaining open MR. It writes ledger rows only — no forge mutation. Each
disposition then executes per the work order's "Per-disposition execution
rules" and "Batch mechanism"; `merge-now` / `ship-in` items are never batched
and take their own gate.

**Re-observed** against `main` at `76c095fb` on 2026-09-10: **59 open MRs**
(after !2643/!2644/!2646/!2656 merged). Five IIDs (!459, !654, !745, !800,
!810) already carry historical "Refresh candidate" rows above; the rows below
**supersede** those with the current disposition and state the change
explicitly. B4-excluded !2409/!2410/!2452 are dispositioned `merge-now` here
(publish their coordination records through the normal path rather than
archive-close). !2460 is the `6sj1` semantic-data carve-out.

Disposition tally (revised per `gsat` QC1-E1 and `qsmq` QC1-E2): `ship-in` 5 ·
`merge-now` 20 · `close-superseded` 5 · `park-radar` 20 ·
`awaiting-governor` 9. Total 59. (`close-invalid` not used — no row has
premise-falsifying evidence.)

| MR | Class | Disposition and closure condition |
|---|---|---|
| !2636 | CP-B-1 implementation (`feat`) | **ship-in** (v2.11). Active workstream; hard v2.11 precondition. Folds in the !2644 copy-clarification (rule sentence + byte-identity detector + 4 gate-self-tests), then its own exact-head contribution/InfoSec/build reviews + Gate 2. Not batched. |
| !2027 | Gate 1 plan (coordination priority + material challenge, CP-A / `mbg6` lane) | **ship-in** (v2.11). Active IN workstream. Proceeds through its own Gate 1 (or folds into the CP-A coordination-semantics Gate 1 plan); not closed here. |
| !2587 | Gate 1 plan (OTGR relay service) | **ship-in** (v2.11). Adopter-facing OTGR workstream per governor ("purpose-built … AGF by adopters"). Own Gate 1. |
| !2592 | Proposal (OTGR relay API/comms spec v0.1) | **ship-in** (v2.11). Adopter-facing reframe owed per `pkbt` before Gate 1; kept open as the OTGR spec carrier. |
| !2593 | Coordination (OTGR spec routing) | **ship-in** support. `coordination:` MR — publishes independently; reconcile with `6n4q` / !2596 first. |
| !2651 | Learning + routing (`qvzj` docs-estate gap; steward directive) | **merge-now**. Own Gate 2; not batched. |
| !2655 | Coordination (non-author review of Codex `rejw`) | **merge-now**. `coordination:` fast-path. |
| !2652 | Coordination (role-handshake ack; Codex lane) | **merge-now**. `coordination:` fast-path. |
| !2653 | Learning (poll discovery vs obligation completion — `rejw`/`wbgq`) | **merge-now**. Reviewed CLEAR (`mp2e`/`dst6`). Own Gate 2. |
| !2654 | Coordination (review of docs-estate learning + routing) | **merge-now**. `coordination:` fast-path. |
| !2582 | Coordination (handled-MR learning ready for Gate 2) | **merge-now** or archive-close if fully carried by !2574; successor check. |
| !2574 | Learning (handled-MR queue regrowth) | **merge-now**. Own Gate 2. |
| !2368 | Learning (verify proposed fixes with a controlled test) | **merge-now**. Own Gate 2. |
| !2394 | Learning (coordination work-order gaps) | **merge-now**. Own Gate 2. |
| !2455 | Learning re-append (`m969` + `rp7x` from closed !739/!776) | **merge-now**. Append-only path; own Gate 2. |
| !2452 | Coordination (B4 stale-MR disposition executed) | **merge-now**. Records completed B4 work; publish through the normal path (B4-excluded from archive-close). |
| !2412 | Learning (non-reentrant polling) | **merge-now**, unless review finds it fully duplicated by !2653/!2574 → then `close-superseded` naming the successor. |
| !2596 | Coordination (OTGR review invite; Propeller ack) | **merge-now** after reconcile with !2593 / `6n4q`. `coordination:` fast-path. |
| !2590 | Coordination (route OTGR plan CI failure) | **merge-now**, or `close-superseded` if the CI failure it routes is already resolved on `main`. |
| !2410 | Coordination (redirect Batch 34 to release-first queue) | **merge-now**, or `close-superseded` if the redirect is spent. `coordination:` fast-path. |
| !2437 | Coordination (triage digest of 4 stale Gate-1 plan MRs) | **merge-now** as historical, or `close-superseded` by this QC-1 section (which subsumes it). Successor = this ledger section. |
| !2409 | Coordination (consolidate 10 unmerged session records) | **merge-now**. Publish the session records through the normal path. |
| !1444 | Plan housekeeping (mark baseline-pinned DD plan ABANDONED) | **merge-now**. One-line status doc; own Gate 2. |
| !1492 | Plan (publish v2.6.0 coordination-reliability release) | **close-superseded**. Release line is at v2.11; no v2.6.0 cut will occur. Closing note names the v2.11 scope plan. |
| !1657 | Plan (v2.6.1 release-only correction of failed v2.6.0) | **close-superseded**. Same reason; the failed-v2.6.0 correction path is moot. |
| !1617 | Plan (open-MR hygiene disposition, issue #110) | **close-superseded** by `j4rg` queue-convergence + `20260909T195116Z` queue-closeout work order + this QC-1 section. Verify no unique content first. |
| !2375 | Docs (OTGR Zulip-adapter spec) | **close-superseded**. Governor dropped the Zulip parallel; OTGR is purpose-built. Closing note names the OTGR relay plan (!2587) / spec (!2592). |
| !1446 | Docs (second Grok run, v2.5.0-pinned) | **merge-now** (revised per `gsat` QC1-E1: no named successor DD artifact on `main`, so not `close-superseded`). It is a dated, fact-checked evidence dossier; merging preserves it in-tree as historical evidence. Own Gate 2. |
| !973 | Docs (post-v2.5.0 regulatory re-assessment TODO) | **park-radar** — governor chose `park-radar` over `reconstruct` (2026-09-10, "proceed as proposed" on `pnmm`'s recommendation: no committed intent to run the reassessment, so the lighter option). **Carrier:** the retained regulatory-reassessment contract in Codex MR !2670 (`docs/proposals/reconstruct/973-regulatory-reassessment.md`, source SHA-256 independently verified in `pnmm`) — linked from one `docs/radar.md` entry "regulatory / AI-governance requirements landscape re-assessment" with a `review-by` set at QC-4. **MR !2670 must merge to `main` before !973 closes.** The source TODO names role owners (per `pnmm`), so it is un-sponsored, not un-owned; `close-invalid` is not used (`qsmq` QC1-E2). |
| !965 | Docs (Internet landscape scan TODO) | **park-radar** — governor confirmed (2026-09-10, "proceed as proposed" on `pnmm`). **Carrier:** the retained ecosystem-landscape contract in Codex MR !2670 (`docs/proposals/radar/965-ecosystem-landscape.md`, source SHA-256 verified in `pnmm`) — linked from one `docs/radar.md` entry "periodic internet / AI-ecosystem landscape scan" with a `review-by` set at QC-4. **MR !2670 must merge to `main` before !965 closes.** Source TODO names role owners (`pnmm`) — un-sponsored, not un-owned. |
| !1691 | Docs (external simulated due-diligence, Aug) | **merge-now** (revised per `gsat`: no named later DD artifact on `main` identified). It is a dated external-DD evidence artifact; merging preserves it in-tree. Own Gate 2. |
| !952 | Docs (governance-signal gaming radar TODO) | **close-superseded** by !954 (the actual radar entry). Closing note names !954. |
| !459 | Plan + measurement evidence (#53 latency) | **park-radar** (supersedes the historical "Refresh candidate" row). Preserve any normalized data; keep #53 open with `radar` label + `review-by`. |
| !745 | Gate 1 plan (issue-board workflow #75) | **awaiting-governor** (revised per `gsat`: "stale since Aug" is age, not evidence). Named anchor: **issue #75 is open** with no Gate 1 on this plan and no sponsor. Governor-only question: *defer #75 to a later milestone, reassign the plan, or close #75?* |
| !800 | Draft proposal (coordination timeline visualizer) | **park-radar** (supersedes historical row). Unsponsored draft; `radar/` record, close. |
| !810 | Docs (longitudinal CI cost/latency assessment) | **park-radar** (supersedes historical row). Preserve causal-limits evidence; fold the #53 question into radar. |
| !918 | Plan (classify AgentCouch live-transport radar, #99) | **park-radar**. Classification belongs on radar; `radar/` record, close, keep #99. |
| !923 | Proposal (test expectation-signals foundation, #100) | **park-radar**. Unsponsored; `radar/` record, keep #100 open with `review-by`. |
| !954 | Docs (governance-signal gaming radar entry) | **park-radar** — this is the radar entry itself; land it via the research-maintenance fast path (`docs/radar.md` only) or close with the question retained on radar. |
| !967 | Docs (AI regulation/governance gap analysis) | **park-radar**. Fold into radar with a `review-by`; no v2.11 scope. |
| !998 | Plan (audit learning-record field enforcement) | **park-radar**. Keep the audit question on radar; `radar/` record, close. |
| !1000 | Docs (deferred LLIS learning candidates) | **park-radar**. Candidate list belongs on radar; close with retention. |
| !1061 | Docs (classify Swatch; defer palette audit) | **park-radar**. Classification + deferred audit → radar; close. |
| !1451 | Plan (queue deferred strategic candidate comparison) | **park-radar**. Explicitly deferred work; keep the trigger on radar, close. |
| !1527 | Docs (classify Airbnb eval-driven dev guide) | **park-radar**. Source classification → radar/sources fast path or close with retention. |
| !1659 | Docs (classify Kungfu durable-work prior art) | **park-radar**. Same; radar/sources. |
| !1718 | Learning (operational-debt backlog gap; SLO response) | **park-radar**. The proposed collaborative SLO response needs a sponsor; keep the question on radar with `review-by`. |
| !1722 | Learning (supply-chain signing/SBOM gap) | **park-radar**. Security-relevant but no v2.11 scope; radar with `review-by`; revisit at the deprecation/​supply-chain review. |
| !2275 | Plan (calm explanatory-visual system revision) | **awaiting-governor** (revised per `gsat`). Named anchor: the plan carries an **`s8ks` review (V1-V3)** but no Gate 1 decision. Governor-only question: *approve, defer, or close the calm-visual-system plan?* |
| !2306 | Docs (classify AgentConnect operator plane) | **park-radar**. radar/sources. |
| !2390 | Docs (classify Batch 33 sources) | **park-radar** via the `docs/sources.md` research-maintenance fast path (its own eligible MR), else close with retention. |
| !2408 | Docs (classify Batch 34 sources) | **park-radar** — !2410 already redirected Batch 34 to the release-first queue; land via the sources fast path post-cut or close with the batch retained on radar. |
| !2460 | Proposal (record-evidence + linked-data cluster for RUW: role-map, PROV-O, SKOS) | **awaiting-governor** — **hard `6sj1` carve-out**. Governor-only question: *does v2.11 (or a later release) pursue any semantic/linked-data evidence model, and if so under what scope?* No closure, analysis, carrier, or reconstruction without a fresh governor decision. Label `awaiting-governor`. |
| !1723 | Plan (commercial-licensing Gate 1 recovery, `ngmj` CL-1..CL-7) | **awaiting-governor**. Legal/licensing-sensitive. Governor-only question: *is commercial-licensing Gate 1 recovery in v2.11 scope, deferred, or closed?* Label `awaiting-governor`. |
| !1750 | Plan (CL-4 immutable corpus binding Gate 1) | **awaiting-governor**. Ties !1723; same governor-only question. Label `awaiting-governor`. |
| !915 | Proposal (frozen-release two-AI AGF evaluation) | **awaiting-governor**. External-review programme decision. Governor-only question: *proceed with a frozen-release two-AI external evaluation, and on which tag?* Label `awaiting-governor` + `external-review`. |
| !654 | Gate 1 plan (claude-rules canonicalization) | **awaiting-governor** (supersedes the historical "supersession audit" row). Collides with !2038 on `AGENTS.md` / loader layout. Governor-only question: *which of {!654 rules-canonicalization, !2038 stale-branch rule} owns the loader/rules-layout change, and in what order?* Label `awaiting-governor`. |
| !2038 | Gate 1 plan (stale worktree + branch pruning rule) | **awaiting-governor**. Same collision pair as !654; same governor-only question. Label `awaiting-governor`. |
| !1817 | Gate 1 plan (unassigned-work-discovery-rule) | **awaiting-governor**. Named as a `stage: planned` normative owner by a shipped rule (`documentation-rule-projection-rule`), so it cannot be freely parked. Governor-only question: *approve, defer, or reassign the planned owner?* Label `awaiting-governor`. |

### QC-1 execution routing (per the work order's per-disposition rules)

- **`ship-in` (5):** no ledger closure. Tracked as v2.11 IN workstreams in
  `20260909T021841Z-v211-release-scope`; each merges at its own Gate 2.
- **`merge-now` (20):** never batched. Each takes its own exact-head review +
  current gate + Gate 2. `coordination:`-titled ones use the publication
  fast path. This ledger row is not authorization to merge. Includes the
  two dated evidence dossiers !1446 and !1691 (merged in-tree, no named
  successor on `main`).
- **`close-superseded` (5):** !1492, !1657, !1617, !2375, !952. QC-4
  batch-eligible. Before each close, a successor-on-`main` check; the
  closing note names the successor (the first four are digest-staged in
  `gj1e`).
- **`park-radar` (20):** for each, add the `radar` label, a `review-by`
  date, and a trigger to the issue where one exists; where the content is a
  `docs/radar.md` / `docs/sources.md` entry, prefer landing it through the
  research-maintenance fast path. Then close the MR with a `radar/` record.
  QC-4 batch-eligible once each radar destination exists.
- **`awaiting-governor` (9):** keep open, add the `awaiting-governor` label,
  leave the governor-only question in the row above. Not batch-eligible; not
  counted against the §3c intake bound for the advisory board-size check.

(Routing counts above reconcile to the revised tally: `ship-in` 5 ·
`merge-now` 20 · `close-superseded` 5 · `park-radar` 20 ·
`awaiting-governor` 9 = 59.)

### QC-1 disposition evidence (response to `6pms` QC1-E1)

Codex review `20260910T130819Z-contribution-auditor-6pms` (QC1-E1): some
rows above rest on **age** or an **unnamed successor**, and "age is not
evidence" under the work order's "each row cites named evidence" bar.
Correction:

- **Each current row cites named evidence** (the work order bar; age is not
  evidence). The earlier draft of this section reinterpreted that as "the
  verb now, evidence at QC-4" — that reinterpretation was a governance
  relaxation and is **withdrawn** (`sjnk`). QC-4's independent
  pre-execution revalidation, per-item digest, and fail-closed
  `awaiting-governor` conversion remain as **additional** safeguards, not a
  substitute for row-level evidence. `close-superseded` named-successor
  evidence for the four successor-on-`main` MRs (!1492, !1657, !1617,
  !2375) is staged with per-item digests in
  `docs/coordination/20260910T083233Z-planner-architect-gj1e.md`.
- **`park-radar` destinations are concrete, not "somewhere on radar":**
  the **radar-question** sub-class (!459, !745, !800, !810, !918, !923,
  !967, !998, !1000, !1451, !1718, !1722, !2275) lands its retained
  question in `docs/radar.md` with a `review-by` date set at QC-4
  execution; the **source-classification** sub-class (!954, !1061, !1527,
  !1659, !2306, !2390, !2408) lands via the `docs/sources.md` /
  `docs/radar.md` research-maintenance fast path (each its own eligible
  MR). No source MR closes until its destination entry is on `main`.
- **Age-only / unnamed-successor rows corrected in place** (`gsat`
  QC1-E1, not deferred to QC-4): !1446 and !1691 → `merge-now` (dated
  evidence dossiers, no named successor on `main`, preserved in-tree);
  !973 and !965 → `park-radar` (legitimate unactioned questions, no
  premise-falsifying evidence for `close-invalid`; each retained as one
  `docs/radar.md` entry);
  !745 → `awaiting-governor` (anchor: open issue #75); !2275 →
  `awaiting-governor` (anchor: the `s8ks` review, no Gate 1). Age is no
  longer cited as evidence in any terminal row.
- **Any row that cannot produce its named-evidence artifact at QC-4
  execution converts to `awaiting-governor`** with the exact question, and
  is dropped from that batch — it is never closed on a weak basis.

### QC-1 completeness

Every one of the 59 MRs open against `main` at `76c095fb` on 2026-09-10 has a
row above. 58 carry a terminal disposition verb; !2460 is the `6sj1`
carve-out and carries `awaiting-governor` with its governor-only question,
which satisfies the k8bh release-cut precondition ("carries a recorded
disposition"). A fresh live query immediately before any QC-4 execution batch
re-confirms state and drops changed items. **POLL-G1 (`6pms`) — resolved:**
the `k8bh` / `tsbh` bridge records were planner-architect transcriptions of
in-session governor decisions; per AGENTS.md gate discipline they are not
themselves governor-reviewer records. Governor decision-record
`docs/coordination/20260910T133145Z-governor-reviewer-q862.md` (on `main`
via MR !2665) ratifies those bridge records, supplying the durable
governor authority `6pms` asked for. This ledger's re-sequencing and
hygiene-precondition authority rests on `q862`.

### QC-1 continuation (2026-09-12) — MRs opened since the 2026-09-10 baseline

Analysis only; opens and closes nothing (per the work order's own
sequencing note: dispositioning may run in parallel with execution).
Live query against `main` at `4fe0f389`, snapshot timestamp 2026-09-12
~10:22 UTC, found 55 open MRs; 44 already carry a row above. The
following 11 opened after the QC-1 baseline. (A later full poll at
10:51:15 UTC found 58 open MRs — the increase is this cohort's own new
MRs: `!2721` itself, its review handoff `!2722`, and the CI fix
`!2723`. This doesn't invalidate this snapshot's rows; it means
coverage is dated to the snapshot, not a claim that no MR has opened
since. Per `r5rm` QC1-A1.)
Four are the `coordination:`-title-prefix carve-out
(`20260903T043752Z` §3c: "publish independently, uncounted") and need
no row: **!2715, !2718, !2719, !2720**. The remaining seven:

| MR | Class | Disposition and closure condition |
|---|---|---|
| !2636 | CP-B-1 implementation (`feat`) | *(existing row above, unchanged — still accurate: appointed security reviewer Beauvoir found 2 blocking findings at exact head `1da3584d` (`20260912T100746Z-independent-implementation-reviewer-mhhk`); active workstream, not stale, not batch-eligible.)* |
| !2666 | Gate 1 plan, approved (AGENTS.md gate-discipline addition) | **merge-now.** Gate 1 approved (metadata synced this session); exact head `f91a9f5e`. Awaiting governor Gate 2 decision. Not batch-eligible — `merge-now` never batches. |
| !2679 | Draft proposal (bounded CI builder-image/distribution-scan trials) | **awaiting-governor.** Pre-Gate-1 proposal, independently reviewed CLEAR at head `b3c7eaff` (`ssms`, via !2678), no governor Gate 1 decision recorded since 2026-09-10. Named question: proceed to Gate 1 or not. |
| !2688 | Proposal (operating-model token/footprint preliminary assessment, "early start") | **agent-owned, no governor redispatch needed.** Corrected 2026-09-17 per non-author review `j5pm`: `54a3`/`rgb5` are CP-B records and do not govern this MR; the operating-model directive and this preliminary pass's authorization are `pnqk` (`20260911T042239Z-planner-architect-pnqk`, future-triggered directive) and `gmpm` (`20260911T043058Z-planner-architect-gmpm`, governor-authorized early start during the pause). The preliminary pass itself is already authorized and agent-owned — no open governor question on whether to proceed with it. The full post-RUW/post-queue-closeout minimization assessment `pnqk` describes remains deferred until both triggers fire; this row does not ask the governor to redispatch that deferred work. |
| !2689 | Gate 1 plan, approved (quantized-model mechanical-drafting pilot) | **merge-now.** Gate 1 approved (metadata synced this session); exact head `545698a2`. Awaiting governor Gate 2 decision. Not batch-eligible. |
| !2714 | DES-5 estate-corrections fix (`docs`) | **merge-now.** Active workstream this session — independent exact-head review (`q2pt`) found 2 blocking findings, both corrected at head `bb1ee084`; fresh review requested. Not stale, not batch-eligible. |
| !2716 | Draft plan (CP-B-2 publication hold/urgency) | **awaiting-governor**, corrected 2026-09-12 (`r5rm` QC1-R1: the prior row wrongly routed already-delivered agent work back to the governor). Security consultation is staffed and delivered (`smmp` appointment, `q82p`/`kskr` findings/clearance at the then-current head); the narrowed one-plan review trial is governor-approved (`rqkp`). **Currently agent-owned**: plan correction and bounded re-review at the revised head (`3ng2`/`bnkg`). The actual governor-only question is the eventual child Gate 1 implementation decision once that agent work completes — not staffing, which is already done. |
| !2717 | DES-6 escalation + rewrite (`docs`) | **merge-now.** Active workstream this session — governor decision `3rha` directed a rewrite (not archive); implemented at head `59ced653`, awaiting review. Not stale, not batch-eligible. |

**QC-1 continuation completeness (as of the 10:22 UTC snapshot):** all
55 MRs open at that snapshot now carry a disposition row or an explicit
carve-out reason. Rows opened after this snapshot (see the 10:51:15 UTC
recount above) are not yet individually rowed and are not claimed
covered by this completeness statement. No `close-superseded`,
`close-invalid`, `reconstruct`, or `park-radar` candidates identified in
this cohort — all seven are active, recent workstreams (opened
2026-09-10 through 2026-09-12), not the kind of aged/stale MR the
original QC-1 pass targeted. This is expected: QC-1's original 59-MR
baseline is the stale-queue problem; new MRs opened during active
Gate-1/Gate-2 work are not queue debt merely for being un-rowed for a
few hours.

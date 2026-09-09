# Post-v2.10.0 merge-request disposition ledger

Originally observed 2026-07-31 against the post-v2.4.8 baseline: 93 open MRs
before plan MR !858. The historical ledger also includes !858-!861, for 97
individually tracked rows; those rows are preserved below as closeout evidence.

Re-observed 2026-09-09 against `main` at `8d165c8b8547`, after v2.10.0:
76 MRs were open. The first refreshed cohort below covers 24 coordination
carrier MRs and preserves their 47 unique immutable records directly in the
canonical `docs/coordination/` directory. Later cohorts add learning,
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
coordination record from each source MR byte-for-byte, so the source carrier
can close after the batch lands?**

Disposition for every row is `close-superseded`, conditional on this batch
merging. The canonical replacement is the listed record in
`docs/coordination/` on `main`; the source head is recorded so preservation
can be checked exactly. Several source MRs overlap because !2409 consolidated
earlier carriers. Each listed record is included once in this batch, and every
duplicate copy was byte-identical. MRs !2409, !2410, and !2452 are excluded:
publishing their unresolved `requires-ack` records would regress the live
pending queue, so they remain open until an acknowledgement or mechanical
supersession can be preserved with them. No source MR is closed before this
batch's exact-head Gate 2 approval and merge. Source branches and discussions
are retained.

Terminal-disposition authority is the governor decision
`20260909T015949Z-governor-reviewer-sgbh` in MR !2575. That decision
authorizes preparation of this non-plan cohort but still requires this batch's
own exact-head Gate 2 before any forge closure.

| MR | Audited source head | Canonical records preserved by this batch |
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
audited source head. Terminal blank lines present in eight source records are
therefore retained rather than normalized.

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
note naming this ledger path, the batch MR and merge commit, and the preserved
record IDs. Do not delete source branches as part of the close. Re-observe MR
state immediately before mutation; any new commit, unresolved discussion, or
record-content mismatch removes that MR from the close set and returns it for
review.

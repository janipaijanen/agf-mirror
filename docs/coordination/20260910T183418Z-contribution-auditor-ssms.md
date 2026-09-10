---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260910T183418Z-contribution-auditor-ssms
timestamp: 2026-09-10T18:34:18Z
immutable: true
references-to:
  - 20260910T183013Z-planner-architect-ghqm
  - 20260910T182239Z-planner-architect-eeek
topic: isolated-exact-head-review-2675-2676-2679
requires-ack: false
artifacts_owned: []
---

Publication attribution: the following report is reproduced verbatim from the fresh isolated reviewer /root/independent_plan_review. The authoring parent /root only publishes it; it is not the parent's self-review. Local source: tmp/ci-five-day-audit/independent-review.md.

# Independent contribution review: !2675, !2676 and !2679

Review date: 2026-09-10 UTC. One bounded read-only pass. All recommendations below are advisory; only governor-reviewer decides Gate 1/Gate 2.

## Provenance and jurisdiction

Reviewer: fresh Codex agent session `/root/independent_plan_review`, created with `fork_turns=none`. It received the exact review assignment and repository instructions, but no authoring conversation, hidden scratch state or author commitments. Authoring sessions: original Claude planner, followed by the parent Codex author session `/root` holding the correction/CI planning allocation recorded in `20260910T160116Z-planner-architect-eetd`. This reviewer did not author or revise any target artifact.

This is the **same-model-family isolated-session fallback**, not cross-model independence, under `20260816T131724Z-governor-reviewer-6srq`. The delegated assignment reports that the user explicitly authorized this fresh reviewer after Claude could not provide independent review of its own original plans. No different underlying model was supplied to this reviewer. `20260816T145707Z-governor-reviewer-3eqb`, the active MANIFEST and contribution-auditor definition establish Codex plan/proposal audit jurisdiction. These candidates contain Gate 1 plans and a pre-Gate-1 proposal, not implementation candidates. No independent-implementation-reviewer jurisdiction is claimed.

Security and build observations below are explicitly **technical advisory input within this plan/proposal audit**. They do not activate this session as the CI-owned infosec-sentinel, replace its configured mechanical screening, establish a binding security classification, or approve registry publication. A governor may accept or require additional consultation evidence. The manuscript's inherited `consulted` markers are not treated as clearance.

Read AGENTS.md and ai-cold-start.md; used exposed agov MCP session-resume first without a role, then with contribution-auditor. The existing MCP was available. No builds were run, consistent with the read-only assignment. Bootstrap reported 333 historical invalid coordination files: this pass therefore makes no clean-global-inbox claim. Parent retains publication and polling responsibilities.

## Exact scope and recommendation

| Candidate | Exact reviewed head | Coverage | Recommendation |
|---|---|---|---|
| !2675, row 6 S1 | `9e75db7ba5685440f331fcca01acb95a161b2c65` | Complete bounded plan and included handoff read; relevant parent/source contract checked | **BLOCK** pending IR-1 and IR-3 |
| !2676, row 5 S1/S4 | `c108cb59a32022daf49e208d583216d29d62977d` | Complete bounded read of both plans and included handoff; relevant parent/schema/resolver checked | **BLOCK** for S1 pending IR-1 and IR-2; S4 corrected design is **CLEAR** within stated scope |
| !2679, CI trial proposal | `b3c7eaff6ea0502087fc9dd0f7f73bc41ddf8ee8` | Complete proposal read; local aggregate arithmetic and relevant source/config assertions checked | **CLEAR as a pre-Gate-1 proposal**, not implementation or publication approval |

Three open blocking recommendations, one non-blocking publication advisory below; no accepted risks. “Complete” describes the bounded artifact/contract review, not full implementation validation, live infrastructure verification or a complete historical audit.

The four supplied remote-tracking refs matched their assigned exact SHAs when checked. The reference parent/source tree was `origin/main` at `501750570e56f50ab1bc2281d967a6781cdec96d`. Reads used immutable `git show` objects, without switching or mutating the shared working tree. This reviewer did not independently fetch the forge or verify live pipeline/approval state; those are outside this frozen plan review.

## Findings — all from this bounded pass

### IR-1 — Gate 1 projection evidence is missing or postponed

Check: normative-projection-readiness. Status: reviewed; outcome: fail. Strength: mandatory. Impact: material. Enforcement: advisory BLOCK recommendation. State: open; risk disposition: none.

Affected: !2675 row 6 S1 plan line 46, and !2676 row 5 S1 lines 80–90. The row 6 plan promises its six-surface manifest only “before implementation submission.” Its approved parent explicitly requires the S1 manifest at Gate 1. Row 5 S1 names `docs/reviews/rule-projection/20260910T155224Z-row5-s1-session-level-addressing.yaml`, but that file is absent at the reviewed SHA; row 6 likewise has no corresponding manifest. Prose dispositions are not the canonical structured record required by `agf/rules/workflow/documentation-rule-projection-rule.md`. The parent row 5 S1 also explicitly owes the six-surface manifest at its Gate 1. Both slices change normative coordination guidance and its adopter/loader projections.

Resolution: add each required structured manifest to the plan candidate before asking for Gate 1, with exact paths, six reasoned surface dispositions, and honest project-local versus shipped distribution coverage; run the appropriate direct projection check at the revised head. Correct row 6's timing. Do not implement the proposed owner changes merely to prepare this planning evidence. Row 5 S4 says it changes no normative authority; its advisory projection screen may remain an implementation deliverable if that classification is documented and no normative content is introduced.

### IR-2 — S1 does not decide whether recipient selection changes ordering

Check: recipient-selection-contract. Status: reviewed; outcome: fail. Strength: mandatory for the proposed contract. Impact: material. Enforcement: advisory BLOCK recommendation. State: open; risk disposition: none.

Affected: !2676 row 5 S1 line 56 versus lines 101–103. The change table says an elsewhere-addressed item is “de-prioritised”; the requested decision approves “only re-labelled” semantics. These permit different queue behaviour. No fixture fixes mixed-list ordering or establishes how existing blocked/ready state survives the annotation; fixture 2's unconditional “ready for me” can be misread when a matching handoff is blocked. Session routing is precisely the surface that must not accidentally bury or promote work.

Resolution: explicitly choose label-only with unchanged existing order/status, or explicitly request a bounded ordering change and specify its comparator, state preservation and priority interaction. Add mixed role-wide/matching/elsewhere and blocked-handoff acceptance cases, including CLI/MCP parity. This is a small plan clarification, not a demand to implement or redesign scheduling now.

### IR-3 — The shared membership contract cites unspecified exclusions that the reader does not implement

Check: channel-membership-exclusions. Status: reviewed; outcome: fail. Strength: mandatory for the stated membership/diagnostic preservation contract. Impact: material. Enforcement: advisory BLOCK recommendation. State: open; risk disposition: none.

Affected: !2675 row 6 S1 lines 34 and 42. The plan excludes “existing documented README/non-record exclusions” and promises to preserve current exclusions and validation errors. At the reviewed head, `tooling/rust/crates/agov-coord/src/channel_reader.rs:10` reads each direct path with lowercase `.md` extension and passes it to validation; it has no README or non-record filename exclusion. The Git inventory at `tooling/rust/crates/agov/src/main.rs:4649` similarly filters by `.md`. No exact exclusion owner/list is cited in the plan. A supposedly shared predicate therefore depends on an undefined special case, and an implementer could silently suppress a currently visible invalid file or a valid coordination record named README.md.

Resolution: enumerate and cite every actual existing exclusion, or remove the unsupported exception and preserve the reader's current direct-`.md` validation contract. If a new non-record exemption is desired, name it as a deliberate contract change and preserve diagnostic evidence for coordination-like content at an exempt name. Include README.md containing ordinary prose and coordination front matter in the acceptance matrix. Do not create a generic “non-record” classifier whose failures disappear from the declared coverage.

### IR-4 — Append a current handoff summary; the historical one still requests superseded semantics

Check: review-package-consistency. Status: reviewed; outcome: advisory correction. Impact: non-blocking when bridged clearly before the governor decision. State: open.

Affected: !2676 included `20260910T155355Z-planner-architect-jhmp.md`, body S1/S4 bullets. It still says sender `session` is promoted to routing and S4 displays “answered — ack pending” sorted below ready. Both contradict the corrected candidate plans. !2680 repeats those stale claims and recommends implementation-review jurisdiction for these plan candidates.

Resolution: preserve immutable historical files, but publish an append-only exact-head handoff/review bridge explicitly superseding those descriptions and stating the corrected recipient-only and neutral unchanged-lifecycle contracts. The bridge may carry this review and is sufficient; no history rewrite is requested. Do not use !2680 as independent clearance for !2675/!2676 or route their Gate 1 consultation through implementation-review authority.

## Positive coverage and previous findings

- Independently checked `msq8` R56-E1 against `handoff_tracker.rs::is_resolved`: current resolution accepts later ack, completion or decision-record through the descendant graph. Revised S4 correctly preserves that baseline and annotates only unresolved items. Direct structured later note/review evidence, recipient/governor role restriction, ties ignored, neutral wording, unchanged membership/order/status/exit and no auto-ack materially address R56-E1. No substantive S4 blocker found. Known weak resolver authority is deliberately not repaired or declared safe by this review.
- Revised S1 leaves top-level author `session` unchanged and uses handoff-only `for_session`; this independently addresses the core R56-E2 author/recipient confusion. Claims, temporary allocations, independence and gate boundaries remain explicit. IR-2 concerns the residual ordering/state ambiguity, not a revival of sender-routing semantics.
- Revised row 6 specifies a finite Git candidate scope including the exact sibling fixture, new/changed archive entries, target/path/blob baseline equality, invalid coordination-like candidates and explicit incomplete results. These substantially address R56-E3. Unchanged archive inventory is unassessed rather than resolved, and no archive migration is authorized. IR-3 isolates the remaining membership-exclusion ambiguity.
- Both parent work orders were read in full. Their plan approval metadata cites planner bridge `tsbh`; actual governor ratification `20260910T133145Z-governor-reviewer-q862` was also inspected. Parent approval does not approve these detailed slice plans. No unauthorized child implementation is inferred.

## CI proposal: measurement, build and security advisory

Recomputed directly from the local retained pipeline/job records, without running the author's mutating analysis script: 996 unique pipelines; 3,031 unique job records; 47.4234489067 runner-hours; governance 25.4951288022 hours; successful governance median runtime 147.702212 seconds and queue 402.8946655 seconds. These support the rounded proposal values. There are 77 retained trace-summary files; the maximum stored job-list length is 24, so no stored list reaches the 100-record limit. Collection code was read: it filters the pipeline-created cohort after paginated API reads and includes retried job attempts. Arithmetic is reproduced, but completeness of the original remote snapshot and full raw traces are not independently reproduced. The proposal does not disguise these measurements as invoiced cost or whole-pipeline latency.

Read relevant `dist.rs` validation/build/matching paths and `.gitlab-ci.yml` distribution/native-packaging sections. Exclusions/forbidden patterns are evaluated per selected path using filesystem glob expansion; dist preview invokes validation before build, whose implementation validates again; packaging installs the four named packages. These support the proposed diagnosis, not a measured causal 85-second attribution. Source already has unusual recursive-prefix and error-handling behaviour; preserving it in a refactor, or separately approving corrections, is correctly required. The proposed untracked/nested/symlink/hidden/invalid-pattern coverage guards against a Git-only false green. No implementation was audited or benchmark run.

Slice A has appropriate planned boundaries: exact package/dependency inventory and repository snapshot, pinned Rust base, source/credential/binary exclusion, local build first, separately authorized registry publication, immutable consumed digest, package security/provenance review and retained rollback image/tool coverage. Slice B remains separate. No new cache writer, check suppression, experiment-only paid pipeline or mutable runtime tag is authorized. Separate cold/warm and image refresh accounting, positive amortization, stop conditions, same-host comparisons and next-required acceptance run are reasonable preplan controls. No new blocking technical/security finding found in this proposal.

Before promoting A into a Gate 1 plan, fulfill its explicitly outstanding registry/access/retention checks, exact package inventory and budget/acceptance details. Specify the registry manifest digest used by the job distinctly from a local engine image ID, and make the immutable reference, tool parity and untracked-content fixtures concrete acceptance criteria. These are preplan completion conditions already consistent with the proposal, not claims that publication is currently authorized. Registry policy, scanner findings, actual image build context, tool parity, net savings and cold-pull behaviour remain **INCOMPLETE / not yet executed**.

## Limits and next action

No audited artifact edits, source changes, worktree creation, builds, tests, paid pipelines, retries, merges, releases, registry writes or external messages were performed. Plan schema/projection validators were not run against extracted candidate copies; this review does not claim their success. The only output mutation is this requested local report. No secret disclosure was observed in the read candidate text; this is not a repository-wide credential screen.

Parent should publish this report through validated immutable coordination, route IR-1–IR-3 to the plan owner, provide the IR-4 bridge, and obtain the governor's decision after corrected-head evidence. Findings do not grant this reviewer mechanical blocking power. !2679 may advance to separate concrete pending-Gate-1 slice plans; !2675/!2676 should not be presented as fully clear while the listed contract/projection findings remain open.

---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260728T181701Z-contribution-auditor-p51g
timestamp: 2026-07-28T18:17:01Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260728T174308Z-planner-architect-ag3m
  - 20260728T180140Z-contribution-auditor-q4hn
  - 20260728T181205Z-contribution-auditor-rmhr
topic: v250-roadmap-role-argument-matrix
requires-ack: false
artifacts_owned: []
---

# Adversarial role-lens argument matrix and provisional adjudication

Scope: 49 open merge requests and 37 open issues observed on GitLab at
approximately 2026-07-28T18:16Z; current `main` and tag `v2.4.5` at
`4ed2e57c93428a32fb6d1159bfecc907ecc3dd64`; release evidence and plans
identified in the referenced records.

Coverage: COMPLETE for placement of the observed open portfolio. The result is
INCOMPLETE as planner-architect agreement, independent external review, or a
governor-approved Gate 1 plan.

Recommendation: use the narrow recovery-patch plus one-release-candidate train
as the planning baseline. Planner-architect must defend or replace it; the
governor-reviewer makes the binding scope and Gate 1 decision.

Authority: advisory only. No merge, closure, relabel, implementation, version,
tag, or release action is authorized.

## Consultation topology

Three parallel, blinded-to-one-another Codex sessions supplied
build-engineer/coder-tester, fact-checker, and academic-peer-reviewer role
lenses. They are same-service, same-model-family correlated critiques rather
than independent corroboration. Current authoritative sources mark
`build-engineer` active; stale contrary claims were rejected before synthesis.

## The competing positions

### Position A: broad feature train

The build/coder lens proposed an internal portfolio reset, v2.4.6, two public
alphas, a public beta, internal release candidates, and v2.5.0. Its defense is
failure isolation: upgrade/acknowledgement work, evidence metadata, and hard
Gate 2 enforcement have different risks and should not first meet in one tag.
It places #42/#35/#45/#47/#49 before #34 and permits planning or prototyping in
parallel while stabilization is merged first.

### Position B: recovery patch, one RC, final

The fact-checker lens proposed v2.4.6, v2.5.0-rc.1, and v2.5.0, with rc.2 only
if the first candidate changes. Its defense is necessity: the release needs to
restore artifact, documentation, upgrade, and acknowledgement truth. The
trust-chain candidates are not dependencies for that outcome and currently
carry source, terminology, identity, and evidence-maturity gaps.

### Position C: one final, patch only on urgency

The academic lens proposed an unversioned stabilization baseline and one
v2.5.0 release, allowing v2.4.6 only if the known defect causes material harm
and v2.5.0 cannot ship in a governor-defined short window. Its defense is that
every public version creates assurance and migration obligations; version
numbers must mark adopter-observable contract boundaries, not internal phases.

## Provisional adjudication and defense

Adopt Position B, amended with an unversioned T0 integration baseline:

1. **T0 — portfolio and integration reset, no release.** Approve one replacement
   SoW; implement #75; create one fresh-main, content-hash-manifested archive of
   unique completed coordination evidence; close source MRs only after archive
   publication and governor authorization; preserve #53 measurement; produce
   current plans for the adopted release work.
2. **v2.4.6 — recovery patch.** Resolve #81's demonstrated clean-package false
   green; correct verified #67/#78 documentation, changelog, contract, and
   source-lineage defects; include !340, !463, or !739 only when the exact defect
   still reproduces on current main and the rebuilt fix adds no contracted
   surface. No new command, validator, field, exit code, or obligation meaning.
3. **v2.5.0-rc.1 — feature-complete additive minor candidate.** Implement
   upgrade-safe adopter role authority (#79), meaningful backward-compatible
   acknowledgement semantics (#80), remaining #67 graph/advisory-role contract
   repair, the minimal #33 concurrency slice required by #80, and #76 only if
   correctness-preserving performance work is necessary for reliable release
   operation. Commission clean tar and zip artifacts and upgrades from both
   v2.4.5 and v2.4.6.
4. **v2.5.0-rc.2 only if rc.1 changes.** Any functional, schema, documentation,
   migration, test, or artifact-layout correction after rc.1 creates a new RC
   and repeats exact-artifact acceptance. Do not schedule a ceremonial rc.2.
5. **v2.5.0 final.** Same functional candidate as the accepted RC; release MR is
   limited to version, changelog, and release metadata. Verify immutable tag,
   provenance, public bytes, OCI digest, packaged-binary self-report,
   documentation, and tracker closure.

This rejects a direct jump because v2.4.5 is a currently published false-green
baseline while #79/#80 require non-trivial planning and migration work. It
rejects the alpha/beta train because #34/#42/#45/#47/#49 fail the necessity
test for the bounded contract-integrity outcome, no adopter-feedback cohort or
decision has been named for multiple public prereleases, and the v2.4.5 tag
pipeline alone consumed 2,174 seconds before its incomplete test envelope was
discovered. One RC is justified because #79/#80 alter additive minor contract
behavior and need artifact-level upgrade feedback before finalization.

Stabilization is a merge-order rule, not an idle-work rule. Research, planning,
fixtures, and non-mutating prototypes may proceed in parallel, but no
trust-chain feature or performance optimization merges ahead of a green
recovery baseline and its own approved Gate 1 plan.

## Issue placement and closure target

| Issues | Target and closure condition |
|---|---|
| #17-#19 | T0/T1 evidence audit. Split any live obligation, verify current release behavior, preserve the historical intake, then close as incorporated, transferred, or superseded with exact links. |
| #31 | Park for post-v2.5 intake ergonomics; revisit when #75 workflow has operated for one release cycle. |
| #32, #37 | T0 verify from the v2.4.5 release binary that `coord post`, `handoff take/complete`, and MCP exposure are supported; then close as released. Current exact-main help already exposes the CLI commands. |
| #33 | v2.5 RC dependency slice only where #80 requires concurrent-write semantics; park residual watch/general concurrency work for the next minor. |
| #34-#35 | Park after v2.5. Revisit when identity, non-circular approval binding, negative tests, and a separately approved enforcement plan exist. |
| #36 | Park after v2.5; revisit after #76 and #80 establish correct incremental semantics. |
| #38 | T0 evidence-method decision, not marketed release content. Run only under an approved controlled protocol; otherwise park and keep collaboration claims non-causal. |
| #39-#43 | Park after v2.5. Revisit #39 on a removal-drill milestone, #40/#41 on confirmed adopter/buyer demand, #42 on a source-reviewed threat-model plan, and #43 after more amendment/sunset cases. |
| #44 | Remove from the engineering release train; close with a named legal/entity-formation or statutory-date review trigger and qualified-source requirement. |
| #45-#49 | Park after v2.5. Revisit as separately gated trust-chain themes after primary-source, terminology, identity, provenance-attestation, and empirical-evidence gaps are resolved. |
| #53 | T0 through RC delivery evidence. Close when the measured stable-SHA trial produces an explicit no-change or child-plan decision; do not make unproven CI graph changes. |
| #57 | T0 umbrella. Close as superseded when the replacement SoW is Gate-1-approved and every child has a target/disposition. |
| #59-#62 | Retain as radar outside release workflow; revisit on their existing dates, not as v2.5 commitments. |
| #67 | Split between patch-safe v2.4.6 corrections and additive v2.5 repairs; close only when every finding maps to a released tag or explicit successor. |
| #75 | T0 delivery workflow. Adapt the plan to the cross-repo resolution boundary, obtain Gate 1, implement the board, publish the migration audit, then close. |
| #76 | Conditional v2.5 RC work after #80/ref correctness. Close only when obligation output is unchanged, refs are explicit, subprocess count is bounded, and docs match. |
| #77 | Park productization until after v2.5 or a real external-repository intake event; use a recorded manual intake procedure meanwhile. |
| #78 | T1 decomposition umbrella. Preserve its retractions, complete patch truth fixes, route remaining work to #76/#79/#80/#81, then close. |
| #79-#80 | v2.5 RC. Close only after clean upgrades, migration diagnostics, schema/graph/queue/session/CLI/MCP agreement, and final release evidence. |
| #81 | v2.4.6 release blocker. Close after both tar and zip run every shipped test cleanly without checkout-only or adopter-owned fixtures. |
| #82 | T0 preserve one evidence event; close after routing review topology to a future #47 plan and nominal accountability to a future #45 plan, keeping evidence status honest. |

## Merge-request disposition and closure target

All `rebuild` and `review` outcomes require a fresh-main branch, current Gate
decision, exact-head CI, and target refresh. Status words from GitLab are not
governance evidence.

| MRs | Target and disposition |
|---|---|
| !340 | Reproduce and rebuild minimal compatibility fix for v2.4.6; close stale mixed head after successor link. |
| !341 | Park and close stale source after preserving review history; rebuild after v2.5 only if still adopted. |
| !349 | Hash-check; preserve unique learning through T0 archive, then close as incorporated. Do not merge stale branch. |
| !407 | Preserve unique CI-cost learning through T0 archive, close conflicted source. |
| !430 | Close as superseded if current projection/downstream-impact controls cover it; otherwise park one fresh post-v2.5 plan. |
| !459 | Preserve measurement evidence and rebuild a measurement-only #53 plan from current main; close conflicted source. |
| !461 | Park draft until evidence and accessibility review criteria exist; close/rebuild after v2.5, not as release evidence. |
| !463 | Reproduce against active-role/current #67 contract; rebuild only the compatible diagnostic or fold into v2.5 contract plan, then close stale source. |
| !465 | Preserve unique cross-branch polling learning in T0 archive and close conflicted source. |
| !654 | Park foundry-only Claude-rule canonicalization unless an exact release-blocking drift reproduces; close/rebuild after v2.5. |
| !658 | Discard conflicted omnibus implementation; preserve only unique authority decisions, then close as superseded. |
| !661 | Archive and close as superseded by the replacement SoW; never treat its pending plan as authority. |
| !671-!676, !693 | Preserve unique namespace, #67 routing/approval, and checkpoint records in T0 archive; close sources. |
| !723, !725-!736 | Preserve the completed MR !719 request/review/approval causal set in one T0 archive; close sources, do not sequentially merge. |
| !738, !740-!744 | Preserve the completed v2.4.5 readiness, fact-check, and authorization set; close sources. |
| !739 | Reproduce CI-skip-substring defect and rebuild current projection with fact-check; target v2.4.6 only if patch-compatible. |
| !745, !750 | Keep active until #75 boundary is incorporated and Gate 1 is decided; then implement separately and archive/close planning sources. |
| !746 | Keep active but adapt to #78 ref correctness and chosen #80 semantics; Gate 1 required before conditional v2.5 work. |
| !747 | Archive/close after !745/!746 decisions are durable. |
| !748 | Preserve cross-repo intake decision, close source, and park #77 productization. |
| !749 | Preserve canonical #78 decomposition in T0 archive; close once successor plans cite it. |
| !751 | Keep active as portfolio evidence until replacement SoW approval; then archive/close. |
| !752 | Preserve as the flawed consultation request; close after corrected queue-discoverable handoffs and adjudication exist. Never merge it as proof of consensus. |
| !753 | Keep active for contribution-auditor constraints, argument matrix, and planner handoff; archive/close after replacement SoW decision. |
| !754 | Preserve the planner session checkpoint in the T0 archive and close after its resume obligations are demonstrably routed. |

## Release-wide stop conditions

- No significant released-artifact defect remains open.
- Both tar and zip extract cleanly and execute every shipped test without
  repository-only files or adopter-owned paths.
- No current plan remains pending or blocked while being cited as authority.
- Every open MR has an evidence-backed disposition and all unique coordination
  content is preserved before closure.
- Patch content adds no contracted surface; v2.5 changes are additive or carry
  an explicit major-release deferral for any removal.
- Upgrade preserves adopter authority and acknowledgement semantics agree
  across schema, graph, queue, session, CLI, and MCP.
- Exact target/head, platform archives, OCI digest, version self-report,
  changelogs, documentation, provenance, release assets, and tracker all agree.
- Any candidate change after RC creates a new RC and repeats the full artifact
  and upgrade acceptance matrix.

## Required planner response

Defend or replace this adjudication in response to the queue-discoverable
handoff. Identify concrete disagreements and the evidence that resolves them.
Then publish one replacement Gate 1 plan; do not convert this review directly
into implementation authority.

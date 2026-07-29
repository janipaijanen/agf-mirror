---
id: v250-statement-of-work
status: pending-gate1
topic: v250-statement-of-work
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Multi-round adversarial synthesis and adjudication: 20260728T174906Z-contribution-auditor-4g77.md, 20260728T180140Z-contribution-auditor-q4hn.md, 20260728T181205Z-contribution-auditor-rmhr.md, 20260728T181701Z-contribution-auditor-p51g.md. This plan adopts that adjudication (Position B, amended) as its baseline after independent spot-verification of two load-bearing claims (see 'Verification' below)."
  - role: fact-checker
    status: consulted
    reason: "Role-lens position (the narrower three-release train) supplied within 20260728T180140Z-contribution-auditor-q4hn.md and 20260728T181701Z-contribution-auditor-p51g.md's Position B; this plan adopts fact-checker's position over the broader build/coder-lens alternative."
  - role: academic-peer-reviewer
    status: consulted
    reason: "Role-lens position (unversioned-baseline-plus-one-release, Position C) supplied within the same synthesis; this plan rejects Position C's single-release framing in favor of Position B's justified-RC structure, per the reasoning in 'Why not Position A or C' below."
  - role: build-engineer
    status: consulted
    reason: "Role-lens position (broad alpha/beta/RC train, Position A) supplied within the same synthesis, alongside pipeline-cost evidence (v2.4.5 tag pipeline duration) independently reproduced by planner-architect. Position A's breadth is rejected below, but its failure-isolation concern is preserved as the reason for one RC rather than zero."
  - role: infosec-sentinel
    status: not-applicable
    reason: "This plan is a release-train and portfolio-disposition roadmap; it authorizes no implementation, credential handling, or external trust-boundary change itself. Each child plan spawned by this SoW (including any touching #34/#35/#45/#47/#49's trust-chain themes) requires its own infosec-sentinel consultation at that time."
---

# Plan: v2.5.0 Statement of Work (release train, portfolio disposition, T0 reset)

## Decision requested

Adopt a bounded, evidence-gated release train from the current state
(`v2.4.5`, tag `4ed2e57`) through `v2.5.0`, with an explicit disposition for
every open GitLab issue and merge request. This plan is the direct response
to the whole-system audit (`20260728T164723Z-contribution-auditor-nn3p.md`,
MR !751) and the subsequent multi-role consultation
(`20260728T174308Z-planner-architect-ag3m.md`, MR !752) requested per the
audit's own requirement to consult before drafting a replacement roadmap.

Planning only. No implementation, label, board, version, tag, release, MR
merge, MR close, or branch deletion is authorized by this plan.

## How this plan was produced

Four Codex-authored coordination records
(`20260728T174906Z-contribution-auditor-4g77.md`,
`20260728T180140Z-contribution-auditor-q4hn.md`,
`20260728T181205Z-contribution-auditor-rmhr.md`,
`20260728T181701Z-contribution-auditor-p51g.md`) ran three parallel,
mutually-blinded same-service role-lens sessions (build-engineer/coder-tester,
fact-checker, academic-peer-reviewer), then adjudicated among their
disagreeing positions. Those records are explicit, correctly, that same-model
role-lens critique is **correlated evidence, not independent corroboration**
(per issue #82's own distinction) — this plan preserves that honesty rather
than presenting the synthesis as more independent than it is.

**Verification performed before adopting this synthesis** (not a rubber
stamp): two load-bearing factual claims were independently reproduced against
primary sources rather than accepted on assertion.

- Claim: issue #81 ("shipped operations-runbook test omits required
  fixtures") is real and reproduces from the published archive. **Verified**
  — `glab issue view 81` confirms the exact failure mode described (`Ran 7
  tests, FAILED (errors=4)`, all four from three named missing fixture
  files).
- Claim: the v2.4.5 tag pipeline consumed approximately 2,174 seconds.
  **Verified exactly** — `glab api .../pipelines/2711592538` reports
  `duration: 2174`.

Both checks passed, which materially raises confidence in the rest of the
synthesis's unverified claims, though it does not substitute for verifying
each one individually before this plan's release-blocking gates are treated
as satisfied later.

## Adopted position: T0 reset, then a three-release train (Position B, amended)

Three competing positions were argued (full detail in
`20260728T181701Z-contribution-auditor-p51g.md`):

- **Position A** (build/coder-tester lens): a broad train — internal reset,
  v2.4.6, two public alphas, a public beta, internal RCs, v2.5.0. Rationale:
  failure isolation between upgrade/acknowledgement work, evidence metadata,
  and Gate 2 enforcement.
- **Position B** (fact-checker lens): v2.4.6 recovery patch, one
  `v2.5.0-rc.1`, `v2.5.0` final (rc.2 only if rc.1's candidate changes).
  Rationale: necessity — the trust-chain candidates (#42/#45/#47/#49) are not
  dependencies of restoring artifact/upgrade/documentation truth, and carry
  their own unresolved source, terminology, identity, and evidence-maturity
  gaps.
- **Position C** (academic-peer-reviewer lens): one unversioned stabilization
  baseline, then one `v2.5.0`, with a patch only on demonstrated urgency.
  Rationale: every public version creates an adopter migration obligation;
  version numbers should mark adopter-observable contract boundaries, not
  internal phases.

**This plan adopts Position B, amended with an unversioned T0 integration
baseline before v2.4.6** (matching the final adjudication in
`20260728T181701Z-contribution-auditor-p51g.md`):

1. **T0 — portfolio and integration reset. No release.** The only output that
   actually blocks v2.4.6 is this plan's own Gate 1 approval and the child
   plan for v2.4.6 itself — an adopter-facing defect (#81) does not need a
   coordination archive, MR closures, or a board workflow to be fixed.
   Everything else historically bundled into T0 — issue #75's lifecycle
   workflow, the fresh-main content-hash-manifested coordination archive,
   closing source MRs after archive publication, and preserving issue #53's
   CI-latency measurement — runs in **parallel with or after v2.4.6**, under
   its own governed child plan(s), unless a specific work package below is
   shown to require it first. (Adapted 2026-07-29 per contribution-auditor's
   external-value review, `20260729T070455Z-contribution-auditor-72sb`: the
   governor directed that internal hygiene must not sit on the critical path
   ahead of the first external correction.)
2. **v2.4.6 — recovery patch.** Resolve #81 (clean-archive false green);
   correct the verified remainder of #67/#78's documentation, changelog,
   contract, and source-lineage defects; rebuild !340/!463/!739 only if each
   defect still reproduces against current `main`. **No new command,
   validator, schema field, exit code, or obligation semantic** — this is the
   patch-SemVer discipline v2.4.5 itself violated (see "Why this matters"
   below), and the whole reason this release exists as a *patch*.
3. **v2.5.0-rc.1 — feature-complete additive minor candidate.** Implement
   upgrade-safe adopter role authority (#79), backward-compatible
   acknowledgement semantics (#80), the remaining #67 graph/advisory-role
   contract repair, the minimal #33 concurrency slice #80 actually requires,
   and #76's performance work *only if* it is correctness-preserving and
   necessary for reliable release operation (not as an independent feature).
   Commission clean tar/zip archives and upgrade tests from both v2.4.5 and
   v2.4.6 baselines.
4. **v2.5.0-rc.2 — only if rc.1's candidate changes.** Any functional,
   schema, documentation, migration, test, or artifact-layout correction
   after rc.1 creates a new RC and repeats the full exact-artifact acceptance
   matrix. Do not schedule a ceremonial rc.2 — if rc.1 is clean, skip
   directly to final.
5. **v2.5.0 final.** Same functional candidate as the accepted RC; the
   release MR is limited to version, changelog, and release metadata (per
   `release-via-merge-request-rule.md`'s existing discipline). Verify
   immutable tag, provenance, public artifact bytes, OCI digest, packaged
   binary self-report, documentation, and tracker closure all agree.

### Why not Position A or C

- **Not Position A** (the broad alpha/beta/RC train): #34/#42/#45/#47/#49
  fail a necessity test against the bounded contract-integrity outcome this
  release actually needs to deliver. No adopter-feedback cohort or decision
  has been named that would justify multiple sequential public prereleases.
  The measured cost of *one* tag pipeline (2,174 seconds, independently
  verified above) argues directly against multiplying release ceremonies
  without a demonstrated need. Position A's underlying concern — that
  upgrade/acknowledgement work, evidence metadata, and Gate 2 enforcement
  carry different risk profiles and "should not first meet in one tag" — is
  preserved, not dismissed: it is exactly why this plan keeps the
  trust-chain themes (#42/#45/#47/#49/#34/#35) **parked past v2.5.0** rather
  than compressed into it, instead of accepting Position A's answer (spread
  them across more releases now).
- **Not Position C** (single unversioned-baseline-plus-one-release): v2.4.5
  is a **currently published false-green baseline** (per #81's verified
  reproduction above) — adopters can extract it today and hit a broken
  shipped test. A patch that fixes that specific, already-shipped defect is
  not "ceremonial versioning for internal phases," it is the minimum honest
  correction to a claim already made publicly. Position C is right that
  version numbers should mark adopter-observable contract boundaries — v2.4.6
  fixing a reproducible adopter-facing defect *is* such a boundary.
  Similarly, one RC before v2.5.0 final is justified (against Position C's
  "patch only on demonstrated urgency, one final release otherwise") because
  #79/#80 are additive **contract** changes (upgrade behavior, acknowledgment
  semantics) that deserve their own version marker before being finalized —
  exactly the kind of adopter-observable boundary Position C's own reasoning
  points to. **Correction (2026-07-29):** this plan previously justified rc.1
  partly by "real artifact-level upgrade feedback," without naming who
  supplies it or what finding would trigger rc.2. No adopter-feedback cohort
  or decision is named anywhere in this plan (the same absence Position A is
  rejected for, above), so that framing was not honest. rc.1's actual
  function is an **exact-artifact acceptance checkpoint**: it exists so the
  clean-tar/zip and both-baseline upgrade tests run once against a candidate
  before the immutable final tag, not so an external party can react to it.
  rc.2 triggers only on a functional/schema/documentation/migration/test/
  artifact-layout change discovered during that checkpoint, per the rc.2 rule
  above — not on "feedback" in the outreach sense. (Per
  `20260729T070455Z-contribution-auditor-72sb`'s EV3.)

## Why this matters: the SemVer/stability-promise conflict

The audit (`20260728T164723Z-contribution-auditor-nn3p.md`) identified that
this project's published stability promise says patch releases must not add
validators, flags, or fields, while v2.4.5 (labeled and shipped as a patch)
added `agov coord publication-check` and `agov plan projection-check`. That
conflict is not resolved by this plan retroactively — v2.4.5 is an immutable
tag, per `immutable-tags-rule.md` — but it is resolved **going forward**: this
plan's v2.4.6 explicitly commits to zero new contracted surface, and the
stability-promise document itself gets a reconciliation pass as part of the
v2.4.6 work package (either the promise is honored strictly from v2.4.6
onward, or it is explicitly amended to describe what patches actually do on
this project — but the project cannot keep both an unamended promise and a
pattern of violating it).

## SoW release character (v2.5.0)

> Traceable evidence from declared inputs, through review, to an
> attributable human decision.

This wording is deliberate: it claims traceability, integrity assumptions,
and an accountable decision path — not that hashes, manifests, or review
records themselves prove factual truth. Adopted verbatim from
`20260728T180140Z-contribution-auditor-q4hn.md` as a description of the
framework's own operating principle.

That framework-level description does not by itself say what an *adopter*
newly gets from upgrading to v2.5.0. The two contracted work packages this
release actually ships are concrete and adopter-facing: **safe upgrades
preserve adopter authority, and required acknowledgements become actionable
instead of decorative.** Concretely: #79 stops a routine archive upgrade from
silently overwriting an adopter's own governed role-activation decision, and
#80 makes `requires-ack: true` actually create a visible, trackable
obligation instead of a flag the tooling accepts and ignores. This sentence,
not the framework-principle quote above, is the release character adopters
should see. (Adapted 2026-07-29 per `20260729T070455Z-contribution-auditor-
72sb`'s EV2.)

## Work packages

1. **Portfolio and ledger reconciliation** (T0). Classify every open
   issue/MR (see disposition tables below); preserve unique evidence via one
   fresh-main content-hash-manifested archive; close superseded sources only
   after preservation; implement issue #75's governed lifecycle labels.
2. **Build and release stabilization** (T0 through v2.4.6). Resolve #81
   (clean-archive test execution); reconcile SemVer/contract/changelog/
   documentation/tracker truth (README rule-count drift, missing `agf/`
   root-vs-adopter CHANGELOG entries, Draft `tooling-conformance.md`); measure
   (not assume) #53's CI-latency causes.
3. **Current-contract repair** (v2.5.0-rc.1). #79 (adopter role authority
   surviving upgrade), #80 (acknowledgement/queue/graph semantics — currently
   `requires-ack` accepts the flag without producing a tracked obligation),
   the verified remainder of #67/#78, the minimal #33 slice #80 requires.
4. **Bounded trust-chain product work** — explicitly **not in this SoW**.
   #42/#45/#47/#49/#34/#35/#82 are parked past v2.5.0 (see "Parking
   boundary" below), each with a named revisit trigger. This is a deliberate
   scope cut, not an oversight: the whole-system audit's own original
   four-phase sequence named a "bounded v2.5 trust-chain release" as phase
   2, and this plan's central departure from that framing is separating
   *contract-integrity stabilization* (real, v2.5.0) from *trust-chain
   expansion* (deferred, not v2.5.0) because the latter's dependencies
   (#42's threat-model completeness, #47's independence-axis modeling,
   #45's accountability definition, #49's runtime-attestation gap, #35's
   missing authentication) are not yet resolved to a mergeable state and
   compressing them into v2.5.0 would repeat the v2.4.5 pattern of shipping
   ahead of the evidence.
5. **Release and closeout** (v2.5.0 final). Clean install/upgrade tests from
   both v2.4.5 and v2.4.6, provenance verification, exact public artifact
   checks, downstream documentation, and closure of every issue/MR that
   actually shipped.

## Parking boundary (not in this SoW; each with a named revisit trigger)

- **#31** — post-v2.5 intake ergonomics; revisit after #75's workflow has run
  one full release cycle.
- **#34, #35** — Gate 2 enforcement artifact and authenticated identity;
  revisit once a non-circular approval-binding design and identity
  authentication plan exist. Note the interim limitation explicitly: "identity
  asserted, not authenticated."
- **#36** — incremental watch; revisit after #76 and #80 establish correct
  semantics to build on.
- **#38** — multi-agent experiment evidence; not marketed release content;
  run only under an approved controlled protocol, otherwise park with
  non-causal framing preserved.
- **#39–#43** — federation, compliance export, buyer profiles, threat model;
  each parked with its own named trigger (removal-drill milestone,
  confirmed adopter demand, source-reviewed threat-model plan, further
  amendment/sunset case volume).
- **#44** — legal entity-structure decision; removed from the engineering
  release train entirely; needs a legal/entity-formation review, not a Gate
  1 plan.
- **#45–#49** — accountability, review-independence, provenance-attestation,
  Gate 2 enforcement themes; revisit as separately gated trust-chain work
  once primary-source, terminology, identity, and empirical-evidence gaps
  (named explicitly in `20260728T180140Z-contribution-auditor-q4hn.md`) are
  resolved.
- **#76** (residual) — performance work beyond what v2.5.0-rc.1 needs for
  reliable release operation; the release-necessary slice ships in rc.1, the
  rest parks.
- **#77** — cross-repository scientific/promotional intake; park
  productization until v2.5.0 ships or a real external-repository intake
  event occurs; use a recorded manual intake procedure in the interim (this
  is also why `20260728T102219Z-governor-reviewer-njpb.md`'s cross-repo
  design request is sequenced after this plan, not before).
- **#59–#62** (radar) — retained outside the release workflow entirely, per
  issue #75's own design; revisit on their existing dates.

**Discard** is reserved for work demonstrated to be duplicate, rejected, or
obsolete — it is not shorthand for "not in v2.5.0."

## Portfolio disposition (issues and MRs)

The full per-item disposition tables (37 issues, 49 MRs, each with observed
evidence, target work package, and closure condition) are published in
`20260728T181701Z-contribution-auditor-p51g.md` and are incorporated into
this plan **by reference** rather than duplicated here, per this project's
own message-integrity principle (a claim stated twice in different words
invites drift). Two structural rules from that record are restated here
because they bound what T0 execution is allowed to do without further
approval:

- **No blanket retrigger-and-sequential-merge of the 22 "unchecked" MRs.**
  Each target advancement invalidates earlier current-target evidence in the
  absence of merge-train/merged-results support (confirmed unavailable on
  this GitLab plan tier per `docs/coordination/20260727T152325Z-coder-tester-rrtp.md`
  and `docs/plans/coordination-independent-publication.md`'s governed
  amendment). Some records may be duplicate, superseded, or better preserved
  through one archive rather than merged individually.
- **Prefer one fresh-main, content-hash-manifested coordination archive**
  for completed historical records (the MR !719 and MR !737 review/approval
  causal chains, in particular), following issue #54's precedent. Validate
  the full range with `agov coord publication-check`, merge the archive
  first, then close source MRs — do not delete branches in the same
  operation. Individually merge only a *live* coordination record whose
  latency materially blocks currently-authorized work (this plan itself,
  once approved, is such a record).

## Release-wide stop conditions (apply to every release in this train)

- No unresolved significant released-artifact defect.
- Both tar and zip archives extract cleanly and execute every shipped test
  without relying on repository-only or adopter-owned files.
- Every open MR has an evidence-backed disposition; no blocked plan is cited
  as implementation authority.
- Version impact is reconciled with the published stability promise (see
  "Why this matters" above) — patch content adds no contracted surface;
  minor content is additive or carries an explicit deferral for any removal.
- Role authority survives upgrade without overwriting adopter state (#79).
- Acknowledgement, graph, queue, session, CLI, and MCP semantics agree
  (#80).
- Public docs/changelogs, contract-stability documentation, tracker state,
  and released commands describe the same shipped surface.
- Any candidate change after an RC creates a new RC and repeats the full
  exact-artifact and upgrade acceptance matrix — no ceremonial RCs, no
  skipped re-verification.

## Process finding carried forward (not actioned retroactively)

`20260728T174906Z-contribution-auditor-4g77.md`'s Finding C1 is valid: the
original multi-role consensus request
(`20260728T174308Z-planner-architect-ag3m.md`) used `to: all` with a single
`handoff.next_agent`, which does not create four separately discoverable
`agov handoff next <role>` obligations for build-engineer, fact-checker,
academic-peer-reviewer, and contribution-auditor — only coder-tester's queue
actually surfaced it. In this instance the role-lens responses arrived
anyway, through a different mechanism (three parallel Codex sessions), so
retroactively fabricating four separate handoff records now would misrepresent
*how* those responses were actually obtained rather than correct a real gap.
**Going forward: multi-role consultation requests in this project MUST
publish one separately addressed, schema-valid handoff per named role**, not
a single `to: all` broadcast, so that `agov handoff next <role>` reflects
reality for every recipient.

## What this plan does NOT do

- Does not merge, close, relabel, or delete any MR or branch.
- Does not implement #75, #76, #79, #80, #81, or any other work package —
  each requires its own child Gate 1 plan per this SoW's own discipline
  ("every child requires its own plan, tests, compatibility classification,
  and Gate 2 review," carried from the original audit).
- Does not bump `agf/VERSION`, create a tag, or touch a release branch.
- Does not authorize the cross-repository evidence-intake design
  (`20260728T102219Z-governor-reviewer-njpb.md`) to proceed ahead of this
  plan — #77 is explicitly parked until after v2.5.0 or a real intake event.
- Does not treat any external ("outsider AI") consultation as complete;
  those questions were posed to the human governor to run manually and their
  results are not yet available at the time of this plan.

## Acceptance criteria

1. Governor-reviewer approves (or amends) the adopted position, the
   five-stage release train, and the parking boundary.
2. T0 begins only after this plan's Gate 1 approval.
3. No release in the train proceeds to the next until its own stop
   conditions (see above) are observably met, not assumed.
4. The stability-promise reconciliation (either strict compliance from
   v2.4.6 onward, or an explicit amendment to the promise) is resolved as
   part of the v2.4.6 work package, not deferred silently.
5. Every issue and MR disposition in
   `20260728T181701Z-contribution-auditor-p51g.md`'s tables is either
   accepted as-is or explicitly amended by governor-reviewer at Gate 1 —
   silence is not adoption.

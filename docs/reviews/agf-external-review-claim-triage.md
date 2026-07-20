# External Review Claim Triage

Status: working triage
Date: 2026-07-03
Inputs:

- `docs/reviews/agf-external-review-candidate.md`
- `docs/reviews/agf-external-review-grok-secondary.md`
- `docs/reviews/agf-external-review-cursor-secondary.md`
- `docs/reviews/agf-external-review-claude-secondary.md`

Source aliases used below:

- **OpenAI Web**: OpenAI web-based AI tool review, filed as
  `docs/reviews/agf-external-review-candidate.md`.
- **Grok**, **Cursor**, **Claude**: secondary model perspectives filed in the
  corresponding `docs/reviews/agf-external-review-*-secondary.md` files.

Purpose: turn multi-model review claims into follow-up candidates. This file is
not a Gate 1 plan and does not approve implementation. Items below still need
normal AGF gate discipline before product or documentation changes.

## Disposition Legend

- **Ready-doc**: documentation improvement appears low-risk and well-supported.
- **Verify**: claim is plausible, but must be checked against source files,
  release artifacts, or live tooling before action.
- **Verified**: claim was checked against source files, release artifacts, or
  live tooling during this triage pass.
- **Roadmap**: useful direction, but implementation is non-trivial and needs a
  Gate 1 plan.
- **Evidence**: requires adopter evidence, telemetry, or independent validation.
- **Keep-as-positioning**: useful language for messaging, not a change request.

## Formal Advisory Role Assessment

Status: advisory consultation, not a governor-reviewer decision.

This triage was reviewed through formal AGF advisory role lenses after the
initial multi-model synthesis. The formal role use is recorded as learning
`20260703T095527Z-fadv` in `agf/learnings/foundry.yaml`.

| Role lens | Assessment | Actionable effect |
|---|---|---|
| `fact-checker` | The main review is broadly sound, but secondary-model claims must stay claim generators until checked. Version, release-artifact, schema, live-tooling, and GitLab-history claims need direct evidence. | Added verified/caution notes for tooling, release surface, binding traceability, and public docs version. |
| `academic-peer-reviewer` | Framework-level outcome claims remain Empirically informed, not Empirically validated. Independent adopter evidence is the largest validation gap. | Keep broad effectiveness claims out of proof language; route independent-adopter work to Evidence. |
| `contribution-auditor` | The proposed follow-ups are useful, but roadmap-sized enforcement changes must not be smuggled in as docs edits. | Keep Gate 2 structural checks, semantic validators, delegation validation, and plan-to-diff binding as Gate 1 planning candidates. |
| `infosec-sentinel` | Privacy/labor/provider-boundary claims are valid as risk-awareness claims, not compliance claims. Formal advisory roles should use role identity first and avoid unnecessary personal attribution. | Keep privacy template as Ready-doc with legal/privacy disclaimer caution. |
| `coder-tester` | Live `agov` behavior matters more than review summaries. The local Rust CLI confirms `agov validate agents`, `learnings`, `infosec`, and `readme-stats` are present and runnable. | Mark stale validator-maturity wording as a documentation inconsistency rather than repeating the secondary review claim. |

Authority boundary: formal AI-played review roles may verify, classify,
object, and recommend. They do not approve Gate 1, approve Gate 2, accept risk,
or merge.

## Formal Fact-Check Sprint

Status: formal advisory fact-check, not a governor-reviewer decision.
Date: 2026-07-03.

Roles used:

| Role lens | Scope | Result |
|---|---|---|
| `fact-checker` | Verify factual claims against local docs, local Rust `agov`, git tags, and the public GitLab Releases API. | Several claims moved from Verify to Verified / Verified stale-claim. Release artifact/public docs mismatch remains true: public docs reviewed as 2.2.6, latest published GitLab release API entry is `v2.2.5`. |
| `coder-tester` | Check live Rust CLI behavior and validator surface. | `agov validate --help` now includes `adopter-docs`, `adoption`, `agents`, `learnings`, `infosec`, `contributions`, `framework-config`, and `readme-stats`; `agov validate agents` and `agov validate learnings` run locally. |
| `contribution-auditor` | Separate low-risk docs fixes from roadmap-sized enforcement changes. | Structural Gate 2 checks, semantic coordination validation, delegation validation, conformance fixtures, and plan-to-diff binding remain Gate 1 candidates. |
| `academic-peer-reviewer` | Check evidence strength of external-review claims. | Adoption-weight, governor-capacity, coordination-noise, ROI, and independent-adopter claims remain evidence/research items, not proven outcomes. |
| `infosec-sentinel` | Assess privacy/retention/provider-boundary follow-ups. | Privacy/retention template remains Ready-doc with disclaimer caution; do not present it as compliance coverage. |

Roles that may need governor-reviewer activation later:

- `build-engineer` / `deployment-engineer` lens for release-hardening checks
  such as signing, SBOM, installer scripts, package registry policy, tag-only
  release jobs, and CI artifact expiry. Manifest status changes require
  governor-reviewer approval; until then, fact-checker and coder-tester can
  verify claims, but should not pretend release engineering approval exists.

Coordination posture:

- Write a coordination `kind: review` when formal fact-checking produces
  role-routing, blocks a claim from promotion, or creates follow-up obligations.
- Keep `docs/reviews/agf-external-review-claim-triage.md` as the working
  evidence table.
- Gate 1 plans should be opened only for roadmap-sized items after the
  governor chooses the next slice.

## Advisory Action Routing

Status: advisory work queue, not approved implementation.

Operating principle: external AI reviews are claim generators. The response is
to verify, classify, route, and then let governor-reviewer choose the next
governed slice.

| Work bundle | What to do | Formal owner / advisory roles | When | Gate 1? |
|---|---|---|---|---|
| Docs credibility | Add reviewer evidence index, schema-link checklist, enterprise boundary callout, and a visible limitation note for binding / plan-to-diff traceability. | `fact-checker` verifies claims; `academic-peer-reviewer` checks evidence wording; `coder-tester` implements after approval. | First short response slice. | Usually no if narrow docs-only, but use Gate 1 if multiple public docs are rewritten or claims change materially. |
| Adopter safety | Add privacy/retention/identity-minimization template, minimum-pilot / rules-only entry guidance, and stronger branch-protection guidance for AGF adopters/forkers. | `infosec-sentinel` reviews privacy/provider risk; `contribution-auditor` checks for governance weakening; `coder-tester` implements. | Second short response slice. | Yes if the template creates new adopter obligations; otherwise narrow docs may be fast-track. |
| Tooling and validator truth | Replace ambiguous "Release surface" labels with maturity labels; remove stale agent/learning validator contract-target wording; keep shipped vs planned validators separated. | `fact-checker` and `coder-tester`. | Before the next public release or docs publication. | No for factual corrections unless validator behavior or contracts change. |
| Release hardening | Decide whether to add signing, SBOM, installer scripts, notices, tag protection, required pipelines, package policy, or artifact expiry checks. | Governor may activate `build-engineer` / `deployment-engineer`; `fact-checker` verifies current state; `contribution-auditor` audits policy changes. | After docs credibility and adopter safety, unless governor treats it as release-blocking. | Yes. |
| Structural enforcement | Plan Gate 2 structural merge evidence check, advisory gate drift detector, coordination semantic validator, delegation validation, plan-to-diff binding, and conformance fixtures. | `planner-architect` prepares Gate 1; `contribution-auditor` reviews scope/authority; `coder-tester` implements only after approval. | Roadmap, not mixed into small docs fixes. | Yes. |
| Evidence program | Create independent adopter recipe and metrics for governor time, coordination volume, waiver rate, incidents caught, and token/tooling ROI. | `academic-peer-reviewer` frames evidence strength; `fact-checker` verifies metrics claims; governor-reviewer chooses what evidence can be published. | Parallel background program; do not block immediate docs fixes. | Gate 1 only if adding tooling, schemas, or public claims beyond guidance. |

Recommended governor decision sequence:

1. Approve or reject **Docs credibility** as the next narrow implementation
   slice.
2. Then consider **Adopter safety**.
3. Defer **Structural enforcement** until a Gate 1 plan can define authority,
   false-positive behavior, and CI/MR assumptions.

## High-Confidence Strengths

| Claim | Review support | Disposition | Notes |
|---|---|---|---|
| AGF is repo-local SDLC governance, not runtime/model/compliance tooling. | All reviews | Keep-as-positioning | This is the strongest consensus and should remain prominent. |
| Human governor authority and role separation are central. | All reviews | Keep-as-positioning | Current docs already cover this well; use as summary language. |
| Two-gate workflow is legible and useful when AI changes need reviewable decisions. | All reviews | Keep-as-positioning | OpenAI Web and Cursor reviews state this most cleanly. |
| Append-only coordination is a strong audit pattern. | All reviews | Keep-as-positioning | Retain, but avoid implying semantic completeness beyond validators. |
| Maturity/evidence vocabulary is a distinctive strength. | OpenAI Web, Cursor, Claude | Keep-as-positioning | Good external validation of the documentation-honesty design. |
| Adoption and exit costs are unusually explicit. | OpenAI Web, Cursor, Claude | Keep-as-positioning | Useful for evaluator-facing copy. |
| Privacy/labor/provider-boundary awareness is a differentiator. | OpenAI Web, Grok, Cursor, Claude | Keep-as-positioning | Avoid presenting it as compliance coverage. |

## Ready Documentation Follow-Ups

| Claim / proposal | Source reviews | Disposition | Suggested target |
|---|---|---|---|
| Add a reviewer evidence index mapping common claims to exact source pages/files. | OpenAI Web | Ready-doc | `agf/docs/site/evaluate.md` or `agf/docs/site/ai-evaluation-dossier.md` |
| Make schema-file links first-class in the review checklist. | OpenAI Web, Cursor | Ready-doc | `agf/docs/site/evaluate.md` |
| Add or strengthen enterprise applicability boundary callout. | OpenAI Web, Grok, Cursor | Ready-doc | `agf/docs/site/docs/start/should-you-adopt.md`, `agf/docs/site/use-cases.md` |
| Add a visible known-limitation note for plan-to-diff binding / binding traceability. | OpenAI Web, Cursor | Ready-doc, verify wording | `agf/docs/site/docs/advanced/contracts-and-validators.md`, cross-link from `review-and-release.md` |
| Add a privacy/retention/identity-minimization template for coordination and learning records. | OpenAI Web, Grok, Cursor | Ready-doc, with legal-disclaimer caution | `agf/templates/`, linked from downstream impact and roles/gates docs |
| Add a first independent validation recipe. | OpenAI Web, Cursor | Ready-doc | `agf/docs/site/contribute.md` or new evaluator/adopter page |
| Promote a rules-only / minimum-pilot path as an entry tier, not only an exit mode. | Claude | Ready-doc, verify fit | `should-you-adopt.md`, `exit-and-decommission.md`, possibly `first-governed-change.md` |
| Add coordination-channel growth / archival guidance if not already covered. | Claude | Verify first | Search day-to-day/review docs before adding. |

## Claims Needing Fact-Check Before Action

| Claim | Source reviews | Disposition | Verification needed |
|---|---|---|---|
| `reference/tooling-conformance.md` uses "Release surface" where maturity labels should be clearer. | Claude | Verified, Ready-doc | Direct check found the page says command status uses Level 4/Level 5 maturity vocabulary, but the table status values say "Release surface"; clarify whether "Release surface" means Level 5 deployable. |
| `agov validate` subcommand maturity is ambiguous versus agent/learning validator contract-target language. | Cursor | Verified, Ready-doc | Direct check found current `agov validate --help` includes `adopter-docs`, `adoption`, `agents`, `learnings`, `infosec`, `contributions`, `framework-config`, and `readme-stats`; `downstream-impact.md` still has stale wording saying agent and learning validators remain contract targets. |
| Agent and learning validators are partial or contract targets. | Cursor | Verified stale-claim | Live Rust tooling confirms `agov validate agents` and `agov validate learnings` exist and run. Do not repeat the secondary review claim without qualifying it as stale documentation in `downstream-impact.md`. |
| Release hardening items such as signing, SBOM, installer scripts, and notices are deferred. | Cursor | Verified, split follow-up | Current docs confirm checksums and provenance are shipped for source/tooling archives. `tooling-and-tokens.md` says registry publishing or code signing is out of scope. Release docs inspected here do not show shipped SBOM, installer scripts, or third-party notices. Route any hardening work through build-engineer/deployment-engineer advisory or explicit Gate 1. |
| Public docs and release artifacts both represent version 2.2.6. | Formal fact-check | Verified mismatch | Public release-info page shows framework content version 2.2.6 at commit 320405f3, but the GitLab Releases API currently lists latest published release as v2.2.5. Distinguish "public docs build reviewed" from "published release artifact exists." |
| GitLab `main` branch protection enforces merge-request-only changes. | Governor-reported GitLab setting, 2026-07-03 | Verified | GitLab now reports `main` as protected with "Allowed to push and merge: No one"; changes require a merge request. Verify required pipeline checks, approvals/CODEOWNERS or governor-reviewer equivalent, force-push status, and tag protection separately. |
| Binding traceability and plan-to-diff binding are not shipped in v2 tooling. | OpenAI Web, Cursor | Verified | `contracts-and-validators.md` explicitly says binding validation is not shipped in the current AGF v2 Rust `agov` surface and plan-to-diff scope binding remains a target. |
| Delegation / contestation objects are docs-only in v2.x. | OpenAI Web, Grok, Cursor, Claude | Verified | `roles-and-gates.md` says optional delegation and contestation objects are not semantically enforced by AGF tooling in v2.x. |
| Coordination filename format churn has credibility cost. | Claude | Verified/contextualized | Current docs explicitly explain current readable IDs and preserve legacy sequence/ULID records as immutable history. Treat as a messaging/UX risk, not an unresolved factual defect. |
| Documentation weight may repel solo-maintainer adopters. | Claude | Plausible, Evidence | Public docs provide quick assessment and first-path pages, but the concern is subjective and needs adopter feedback. Use it as support for minimum-pilot/rules-only entry guidance, not as a proven defect. |
| "Foundry", "silver-birch", "copper-rapids", "blue-marlin" independence cannot be verified externally. | Claude | Verify/caution | Treat as evidence-transparency issue; avoid exposing private adopter details. |
| "Rediflow-type projects qualify on paper." | Claude | Verify or ignore | Not source-backed in reviewed docs; do not promote without context. |

## Roadmap-Sized Candidates

| Candidate | Source reviews | Disposition | Why it needs a plan |
|---|---|---|---|
| Gate 2 structural merge check requiring valid decision-record reference. | OpenAI Web, Cursor, Claude | Roadmap | Changes enforcement behavior; must define rule authority, CI/MR assumptions, and false-positive handling. |
| Advisory-gate drift detector warning for missing Gate 1/Gate 2 evidence. | Cursor | Roadmap | Needs data model, target surfaces, and advisory/blocking authority boundary. |
| Delegation / contestation semantic validation. | OpenAI Web, Grok, Cursor, Claude | Roadmap | Touches schemas, coordination semantics, validator behavior, and authority model. |
| Plan-to-diff scope binding. | OpenAI Web, Cursor | Roadmap | Requires diff-source assumptions, plan scope model, and bypass/escalation semantics. |
| Coordination semantic validator for cross-message chain integrity. | Cursor | Roadmap | Already recognized as a deeper validator gap; likely complex and evidence-gated. |
| `agf/conformance/` fixture suite. | OpenAI Web, Cursor, Claude | Roadmap | Affects compatibility claims, release process, and future implementations. |
| Cursor-specific loader verification / worked example. | Cursor | Roadmap or Ready-doc after validation | Should be tested in a live Cursor setup before documenting as reliable. |
| CI/starter expansion and team rotation examples. | Grok, Cursor | Roadmap if examples are executable; Ready-doc if purely illustrative | Needs target CI platforms and maintenance expectations. |

## Evidence / Research Work

| Claim / need | Source reviews | Disposition | Possible evidence |
|---|---|---|---|
| Independent adopter evidence is the main validation gap. | OpenAI Web, Cursor, Claude | Evidence | Named or anonymized independent adoption case studies; governor/authorship separation where possible. |
| Governor capacity is the main adoption bottleneck. | All reviews | Evidence | Gate latency, review time, rework count, waiver rate, and abandoned-adoption notes. |
| Coordination records can become noise. | OpenAI Web, Claude | Evidence | Coordination messages per governed change; corrective-record rate; completion/handoff resolution time. |
| Governance ROI needs quantification. | Cursor, Claude | Evidence | Incidents caught, review churn avoided, governor minutes, failed/weak Gate 2 decisions. |
| Token savings and deterministic-tool ROI should be validated in live sessions. | OpenAI Web, Cursor, Claude | Evidence | `agov token roi report`, stable per-assistant logs, documented limitations. |
| Advisory core must be tested under real multi-party use. | Claude | Evidence | Independent team pilot, multi-assistant task, skipped-gate/late-drift incident tracking. |

## Messaging Candidates

These phrases are useful but should remain positioning language, not proof:

- "AI writes code, humans own the risk."
- "Repository-local governance and evidence layer for AI-assisted software
  development."
- "Strongest where governor capacity exists and audit trail value is high."
- "Auditability is stronger than runtime enforcement."
- "Not autonomous AI self-regulation; not compliance automation."

## Initial Recommended Work Order

1. **Low-risk docs clarity:** reviewer evidence index, schema-link checklist,
   enterprise boundary callout, known-limitation note for plan-to-diff binding.
2. **Adopter safety aids:** privacy/retention template and minimum-pilot /
   rules-only entry guidance.
3. **Fact-check sprint:** tooling-conformance labels, validator maturity,
   release-hardening claims, coordination growth guidance.
4. **Gate 1 planning candidates:** structural Gate 2 evidence check,
   conformance fixture suite, delegation validation, plan-to-diff binding.
5. **Evidence program:** independent validation recipe and metrics for governor
   time, coordination volume, waiver rate, and incidents caught.

## Notes

- Do not treat secondary model reviews as factual authority. Use them as claim
  generators.
- Any blocking automation must cite a named rule granting authority, per
  `agf/rules/always/tool-authority-boundary-rule.md`.
- Claims about schemas, release artifacts, GitLab history, CI, or live `agov`
  behavior should not be promoted until directly checked.

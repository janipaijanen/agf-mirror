# Grok: follow-up due-diligence report, pinned to the v2.5.0 tag

Relayed verbatim by the human Governor. Produced by xAI Grok in response to
a second, separately given prompt (below), distinct from and later than
`grok-report.md` in this directory. Not edited by any AGF agent. Where the
text below self-labels as "independent," that is Grok's own unverified
characterization of itself, not a project-authored provenance claim.

**Prompt given to Grok** (asks for an exact version/title echo-back before
the assessment, specifically to make live-fetch-vs-stale-training-data
distinguishable — a lesson from `grok-report.md`'s own documented staleness
and from a same-day Gemini attempt that fabricated an entire report rather
than disclose it could not browse at all):

> Perform fresh, independent due diligence on the "agents govern framework"
> (AGF) project, using only its public repository and documentation site,
> the way a firm like KPMG, EY, or McKinsey would assess a software vendor.
> Use these two sources:
>
> - Repository, pinned to the v2.5.0 release (not the main branch):
>   https://gitlab.com/janipaijanen/agents-govern-framework/-/tree/v2.5.0/
> - Public documentation site:
>   https://janipaijanen.gitlab.io/agents-govern-framework/
>
> Before anything else, tell me the exact version string shown in the
> repository's `agf/VERSION` file and the exact page title of the
> documentation site's homepage — this confirms you actually fetched
> current content rather than a cached or outdated copy.
>
> Then assess these six dimensions, scoring each 1-5 (or "not enough
> evidence" if you can't tell): product architecture, code quality and
> testing, governance enforceability, key-person/maintainer concentration,
> security/supply-chain/licensing, and adoption/market/release maturity.
> Cite specific files or pages for each finding. End with an overall
> recommendation (adopt / pilot / wait / decline / not enough evidence to
> conclude) and a clearly labeled limitations section, including anything
> you weren't able to verify (e.g. you cannot inspect compiled binaries or
> run the code).
>
> Currently released version is 2.5.0.

## Independent fact-check (performed by Claude, same session, against live source)

Every specific, checkable claim below was independently verified against
live GitLab state at fact-check time — not taken on trust.

**Confirmed correct:**

- Version/title echo (`2.5.0`; `AGF — Agent Governance Framework`), with
  Grok's own stated fetch method — matches independently re-fetched live
  values.
- "46 governance rules, 12 agent definitions, and 7 skills" — verbatim
  match against `README.md` at the `v2.5.0` tag.
- "340 passed, 0 failed, 3 ignored" test-result citation — verbatim match
  against `agf/CHANGELOG.md` at the `v2.5.0` tag.
- The observation that the live Pages `release-info` page lags the `v2.5.0`
  tag — independently reconfirmed (`v2.5.0-rc.2`, commit `753325dc821d`,
  still live on that page at fact-check time). Same staleness class the
  original `grok-report.md` also documented.

**Confirmed incorrect:** the report's licensing claim ("tooling MIT") is
false. `tooling/rust/Cargo.toml` at the live `v2.5.0` tag declares
`license = "CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial"` — no MIT
split exists. This is the same specific false claim `grok-report.md`
(the first Grok run, this directory) already made and this filing's own
`README.md` already documented as `B-DD-FILE-2` — a second, independent
Grok run repeating the identical specific error about the same fact.

**Not independently reverified** (would require deeper source reading than
this fact-check pass covered): every other per-dimension finding not
listed above as confirmed-correct or confirmed-incorrect.

## Context: why this second run exists

Filed as part of closing out an abandoned, more heavily governed
due-diligence effort
(`docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md`,
marked ABANDONED after its one authorized Gemini preflight failed — see
`docs/coordination/20260811T175332Z-contribution-auditor-rtgg.md` and
`docs/coordination/20260811T190048Z-planner-architect-hjpt.md`). The human
Governor directed dropping that effort in favor of simple, ungoverned
prompts against live services, fact-checked afterward rather than
packaged/transported beforehand. Gemini's response to the same prompt
class was unusable (fabricated a report about a different, imaginary
project); Perplexity's was partially grounded with two specific errors
(not filed here — relayed and fact-checked in chat only, not committed to
this directory). This Grok run is the one judged worth keeping as a
committed record.

---

## Full verbatim response

**Exact version string from `agf/VERSION` (tag `v2.5.0`):** `2.5.0`
**Exact homepage title of the documentation site:** `AGF — Agent Governance Framework`

(Confirmed via raw file fetch of `https://gitlab.com/janipaijanen/agents-govern-framework/-/raw/v2.5.0/agf/VERSION` and the live docs homepage at `https://janipaijanen.gitlab.io/agents-govern-framework/`.)

---

## Assessment of AGF (agents-govern framework) v2.5.0

This is an independent, public-source-only assessment of the repository at the pinned `v2.5.0` tag and the public documentation site, performed in the style of a professional software-vendor diligence review (architecture, quality, enforceability, concentration risk, security/supply-chain/licensing, and maturity).

### 1. Product Architecture — Score: 4

**Strengths**
- Clear separation of concerns: adopter-facing product lives under `agf/` (roles, rules, schemas, templates, docs); implementation tooling under `tooling/` (primarily Rust); the project's own governed work under `docs/`; legacy bootstrap under `agents-govern/`. Explicitly documented in README.md and `agf/ADOPTING.md`.
- Git-native, file-based design with no hosted service or shared runtime. Coordination is an append-only channel of repository files (`docs/coordination/`), plans in `docs/plans/`, and human gate decisions.
- Explicit two-gate model (Gate 1 plan approval before implementation; Gate 2 review before merge), named roles with authority boundaries (an agent cannot approve its own work), and optional deterministic validators (`agov` Rust binary).
- Single supported adoption layout in v2 (visible `agf/` directory). Ownership boundaries (framework-owned vs adopter-owned) are stated and reinforced by packaging changes in 2.5.0 (e.g., shipping `MANIFEST.example.yaml` instead of a live `MANIFEST.yaml` to avoid silent overwrites — issue #79).
- Explicit maturity vocabulary (Level 1 Planned → Level 5 Deployable) and separate evidence-status labels (Empirically validated / informed / Theoretical) — see docs `reference/maturity-vocabulary/`.

**Weaknesses / observations**
- Architecture is process-and-artifact oriented rather than a hard technical control plane. Technical enforcement of gates is optional/CI-dependent; human governor decisions remain the binding authority.
- Some historical dual-layout language was cleaned up only recently (2.5.0-rc.1).

Citable sources: README.md (repository map and feature summary), `agf/ADOPTING.md`, docs homepage and "How it works" section, `agf/CHANGELOG.md` (2.5.0 packaging changes), maturity vocabulary page.

### 2. Code Quality and Testing — Score: 4

**Strengths**
- Active Rust workspace under `tooling/rust/` with Cargo workspace, crates, tests directory, Clippy (warnings denied), and reported test results of 340 passed / 0 failed / 3 ignored for the 2.5.0 candidate (and re-confirmed on the stable promotion).
- Strict MkDocs builds, accessibility hooks for figures, archive-overlay testing against prior baselines (v2.4.5, v2.4.8, v2.4.11), and CI improvements (Rust cache topology checks, digest-pinned images).
- Detailed, governance-aware changelogs (`CHANGELOG.md` and `agf/CHANGELOG.md`) that classify changes (Additive / Editorial), record Gate 1/2 decisions, and note validation evidence.
- Schemas (JSON Schema) and validators for coordination records, plans, agents, learnings, etc.

**Weaknesses / observations**
- Test volume and coverage metrics beyond the headline "340 passed" are not independently visible from public pages alone; no public coverage percentage or external CI badge summary was examined in depth.
- The framework itself is largely Markdown + YAML + schemas; the "code" that matters most for adopters is the optional Rust tooling.

Citable sources: `agf/CHANGELOG.md` (Validation sections for 2.5.0 / rc.1 / rc.2), repository tree (`tooling/rust/`), root and `agf/` CHANGELOGs.

### 3. Governance Enforceability — Score: 3

**Strengths**
- Strong process design: role separation, two explicit human gates, append-only coordination channel intended as a reconstructible decision log, learning-record loop that feeds rule evolution, and clear "what ships vs what you activate" guidance.
- Validators and CI can supply evidence; rules are written so assistants load them as instructions.
- Honest self-description of limits (human governor remains binding; AI cannot act as governor in the validated core). Exit/decommission paths are documented (pause / downgrade / remove).

**Weaknesses**
- The project's own changelog and external-evaluation notes (referenced in `agf/CHANGELOG.md` for 2.5.0) explicitly defer known gaps: unauthenticated coordination `from:` fields, mutation/tamper-evidence gaps, and "zero local technical Gate enforcement." These are acknowledged and scoped for post-2.5.0 hardening rather than claimed as solved.
- Enforceability is therefore primarily social/process + optional technical checks, not a hard fail-closed technical control plane. An adopter who disables CI or ignores the human gates loses the protection.

Citable sources: docs "How it works", "Should you adopt?", day-to-day operation, exit-and-decommission pages; `agf/CHANGELOG.md` (Governance notes on deferred findings from external evaluation).

### 4. Key-Person / Maintainer Concentration — Score: 2

**Strengths**
- Transparent acknowledgement that the human maintainer and governor is Jani Päijänen, who retains approval authority and publication responsibility. AI assistance is disclosed and subjected to the same gated process.
- Detailed coordination records, decision records, and learning records exist inside the repository itself (the project dogfoods its own framework).

**Weaknesses**
- Extremely high concentration: single named maintainer/governor visible across README, release notes, changelogs, docs, and copyright notices. No public evidence of a broader core team, multiple independent maintainers, or succession plan.
- All recent releases (including the rapid 2.4.x → 2.5.0 sequence) appear driven by this single individual.

This is a classic single-maintainer risk for any organization considering production dependence.

Citable sources: README.md (Acknowledgement of AI assistance), release-info page, `agf/LICENSE` / root `LICENCE`, all changelogs and decision-record references.

### 5. Security / Supply-Chain / Licensing — Score: 3

**Strengths**
- Dual licensing is clearly stated: framework content CC-BY-SA-4.0 (with commercial alternative available by agreement); tooling MIT. SPDX identifiers and license files present under `agf/`.
- Release process includes evidence collection, protected maintenance-line workflow, digest-pinned CI images in recent changes, and optional provenance/sha256 mentions for binaries.
- Anonymization rules and privacy/labor checklists are present for adopters. AI-generation disclosures and alt-text requirements support transparency (EU AI Act Article 50 objectives referenced without claiming certification).

**Weaknesses / gaps**
- No public SBOM, dependency audit summary, or formal security policy/advisories page was located in the examined material.
- Coordination channel authenticity and tamper-evidence are explicitly called out as open issues in the project's own governance notes.
- Binaries are published; verification of signatures or full supply-chain attestations was not independently confirmed beyond the project's statements.
- Commercial licensing alternative exists but terms are not public.

Citable sources: root `LICENCE`, `agf/LICENSE`, `agf/LICENSE-MIT`, release-info page, `agf/CHANGELOG.md` (deferred security-related findings), docs privacy checklist in "Should you adopt?".

**Fact-check note (this filing, not Grok's own text): the "tooling MIT" claim in this section is false — see the Independent fact-check above.**

### 6. Adoption / Market / Release Maturity — Score: 2-3 (borderline 2)

**Strengths**
- Rapid, disciplined release cadence (multiple 2.4.x patches and 2.5.0 RCs culminating in stable 2.5.0 in early August 2026).
- Extensive, high-quality public documentation site with honest maturity and evidence labels, "Should you adopt?" decision guide, cost/exit transparency, and explicit statements that much evidence is maintainer-informed rather than independently validated.
- Ships complete rule/role/skill/schema library (documented as 46 rules, 12 agent definitions, 7 skills in README).

**Weaknesses**
- Evidence base is predominantly single-maintainer / self-application. Docs repeatedly state "empirically informed maintainer evidence, not yet independent-adopter validation" for installation time, governor attention, etc.
- No visible public adoption metrics, case studies from independent organizations, stars/forks/community signals, or multi-project corroboration of the core claims.
- Reduced "starter package" remains Level 1 Planned; full library ships and is used unpruned by the maintainer.
- Market position is niche (repository-local governance for AI-assisted software development); it is not an agent runtime, orchestrator, or enterprise policy platform.

Citable sources: docs homepage, "Should you adopt?", "Evidence, honestly stated" section, maturity vocabulary, release history on GitLab Releases, README feature summary and cost statements.

---

## Overall Recommendation: Pilot

AGF v2.5.0 is a thoughtfully designed, unusually self-aware, file-based governance process for AI-assisted development. It is well-documented, actively maintained, and honest about its evidence base and limitations. The architecture is coherent for its stated scope (repository-local, human-governor-centric, git-native).

It is not yet suitable for broad "adopt" as a critical enterprise control without further independent validation, multi-maintainer resilience, and hardening of the acknowledged technical enforceability gaps.

**Recommended path:**
- Run a time-boxed pilot on one or two non-critical repositories that already use AI coding assistants.
- Keep the human governor role real and engaged.
- Measure actual overhead, gate effectiveness, and whether the coordination trail proves useful in practice.
- Re-evaluate after 1-2 release cycles or after independent adopters publish corroborating evidence.

Decline only if the organization requires hard technical (non-process) enforcement, multi-repo federation, or zero single-maintainer risk from day one. Wait if the team currently lacks capacity for a real governor role.

---

## Limitations

- Assessment used only the public GitLab repository at the exact `v2.5.0` tag and the public documentation site. No private repositories, internal issue trackers beyond public references, or non-public communications were available.
- Could not execute the Rust tooling, run the test suite, inspect compiled binaries, verify cryptographic signatures/provenance artifacts beyond stated presence, or perform static analysis / dependency scanning.
- Could not observe real adopter deployments or independent usage metrics; all "empirically informed" claims rest on the maintainer's statements and the project's own dogfooding records.
- Documentation site release-info page showed some lag relative to the `v2.5.0` tag at the time of fetch (noted as a documentation-status inconsistency); content was cross-checked against the tagged source tree.
- No commercial license terms, formal security audit reports, or third-party certifications were located.
- External evaluation findings referenced in the project's own changelog were taken at face value as acknowledged open items; primary source documents of those evaluations were not re-fetched.

This review is a point-in-time public-source snapshot as of the assessment date and does not constitute legal, security, or compliance advice.

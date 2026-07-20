# agents-govern External Review

Status: secondary model perspective
Source: Cursor AI response provided by Jani in Codex session on 2026-07-03.
Relationship to primary review: detailed corroborating review, not a replacement
for `docs/reviews/agf-external-review-candidate.md`.
Codex caveat: the final sentence about running a live pilot against
`/home/devi/src/AGF-NG/agents-govern` points at the historical bootstrap
reference tree, not the current AGF v2 product tree (`agf/`). Treat that as an
orientation warning, not a recommended pilot target.

Review type: fit-assessment

Scope reviewed
Item	Detail
Primary source
AI Evaluation Dossier (single-page HTML corpus)
Framework content version
2.2.6 (source commit 320405f3, build 2026-06-26) per Release information
Documentation line
v2.1 documentation (authoring marker, not release version)
Track completed
Thorough assessment track (~40 min equivalent) via dossier Flows 00–07
Pages/sections read
Evaluation brief (Flow 01), framework contents (Flows 04.01–04.09), and full appendix pages: index.md, use-cases.md, evaluate.md, docs/start/should-you-adopt.md, docs/start/adopter-faq.md, docs/concepts/position.md, docs/concepts/roles-and-gates.md, docs/concepts/coordination-protocol.md, docs/concepts/evidence-and-learnings.md, docs/adopt/downstream-impact.md, docs/adopt/install-and-layouts.md, docs/adopt/tooling-and-tokens.md, docs/adopt/exit-and-decommission.md, reference/maturity-vocabulary.md, reference/tooling-conformance.md, reference/release-info.md, reference/disclaimer.md, contribute.md, future/evolution.md
Artifacts not directly read
Raw agf/agents/*.yaml, agf/rules/**, agf/schemas/*.json, live coordination channel files, or executed agov commands
Identity confirmed

 Reviewing agents-govern at https://gitlab.com/janipaijanen/agents-govern-framework

 Published docs: https://janipaijanen.gitlab.io/agents-govern-framework/

 Not Microsoft's Agent Governance Toolkit, an agent SDK, ML/model governance, a runtime orchestrator, or a hosted monitoring platform (per Flow 01 / evaluate.md)
Executive summary: applicability
agents-govern (AGF) is a file-based process framework for making AI-assisted software work auditable and reconstructible inside a git repository. It adds role separation, two human gates, an append-only coordination channel, a rule library assistants load each session, and optional Rust validators (agov) in CI.

Adopt when the cost of an unreviewable AI-generated change exceeds the cost of plan → implement → review → merge with recorded decisions. The docs state this plainly in Flow 04.02 / docs/start/should-you-adopt.md.

Best fit (evidence status: empirically informed, not empirically validated at scale):

Profile	Why AGF fits
Solo maintainer + AI assistants
Replaces a second human reviewer with structured gates; validated on foundry (framework's own dev)
Small team with existing quality culture
Rules bind assistants to stack-specific conventions across sessions/tools
Long-lived, security- or audit-sensitive code
Append-only coordination channel + plan/decision records
Multi-tool setups (Cursor, Codex, Claude Code, etc.)
Git-native files; no shared runtime required (use-cases.md)
Wait or skip when:

Profile	Why AGF misfits
Throwaway / exploratory work
Governance overhead exceeds value (use-cases.md)
No AI assistants in the loop
Conventional review + CI suffice
No human governor capacity
Gates become ceremony; framework explicitly warns against this (should-you-adopt.md, adopter-faq.md)
Need turnkey enforcement or hosted service
AGF is conventions + optional CI validators, not mechanical runtime enforcement
Multi-project / multi-governor federation
Level 1 — Planned / hypothesis only (future/evolution.md)
Org-wide policy without per-repo governor
Validated core is one project, one governor (use-cases.md)
Bottom line: AGF is a strong fit for disciplined teams already using AI coding assistants on repos that must remain explainable months later. It is a poor fit for autonomous AI self-regulation, zero human review overhead, or enterprise federation without local ownership.

Strengths
1. Honest scope and anti-conflation discipline
The dossier and evaluate.md force reviewers to separate identity, capability maturity (Level 1–5), and evidence status (Empirically validated / informed / Theoretical). This is unusually rigorous for an open framework and reduces the common failure mode of treating roadmap items as shipped features.

Source: Flow 01 (evaluate.md), reference/maturity-vocabulary.md

2. Clear adopt / wait / skip decision path
should-you-adopt.md states commitments upfront (afternoon install, 5–15 min governor time per non-trivial change, learning records, upgrade hygiene) and pairs adoption with a documented exit. Evaluators can make a defensible decision without reading the entire corpus.

Source: Flow 04.02, Flow 07.05, Flow 07.11

3. Git-native architecture with zero infrastructure tax
No accounts, runtime, daemon, or hosted service. Framework files live in agf/; coordination in docs/coordination/; assistants load agf/rules/. This aligns with how Cursor, Claude Code, Codex CLI, and similar tools already work.

Source: Flow 04.01 (index.md), Flow 07.07 (position.md)

4. Structural authority separation
The three core roles (planner-architect, coder-tester, governor-reviewer) plus Gate 1 (plan before code) and Gate 2 (review before merge) address the core failure mode of AI assistants optimizing for task completion without accountability. The framework explicitly states an agent cannot approve its own work.

Source: Flow 04.03, Flow 07.08 (roles-and-gates.md)

5. Append-only coordination channel as audit trail
Nine message kinds, YAML front matter, immutable filed records, and references-to chains create a reconstructible decision log independent of commit-message archaeology. Corrections are new messages, not edits — a sound audit pattern.

Source: Flow 04.04, Flow 07.09 (coordination-protocol.md)

Schema reference (not read directly): agf/schemas/coordination-message.schema.json

6. Evidence-driven evolution with provisional rule labeling
Rules from single-adopter evidence carry visible Provisional status; promotion to Recommended requires 2+ independent corroborations. The framework applies its own learning loop to itself.

Source: Flow 04.05, Flow 07.10, contribute.md

7. Mature Rust tooling surface for core validators
Per Flow 04.07 and reference/tooling-conformance.md, these commands are Level 5 — Deployable on the release surface: agov coord validate, agov plan validate, agov validate, agov coord brief, agov handoff next, agov coord status, agov session resume, agov coord graph, agov coord graph-index, agov token roi report, agov usage report. Token savings via deterministic tooling is a practical adoption incentive.

Source: Flow 04.07, index.md token table, reference/tooling-conformance.md

8. Structural vs advisory gate taxonomy
roles-and-gates.md distinguishes CI/pre-commit (structural) from Gate 1/Gate 2 (advisory + human governor). Adopters know where automation ends and discipline begins — a mature design choice.

Source: Flow 07.08

9. Documented, low-friction exit
Exit paths (freeze, replace-on-upgrade, full decommission) preserve historical coordination evidence. Downstream impact itemizes every artifact added. Reversibility is treated as a first-class adoption criterion.

Source: Flow 04.06, Flow 07.11, Flow 07.17

10. Meta-governance: the AI evaluation dossier itself
The dossier (Flow 00–07, single-page corpus with source traceability) is a best-in-class artifact for external AI review. It directly addresses identity confusion, schema fabrication, and capability conflation — problems the framework authors have clearly encountered.

Source: ai-evaluation-dossier.md, Flow 01

11. Privacy, labor, and provider-boundary awareness
should-you-adopt.md includes a checklist on delegation metadata, surveillance risk, and prompt logging — rare and responsible for a governance framework.

Source: Flow 07.05

Risks / gaps / inconsistencies
1. Single-developer evidence bias (acknowledged by maintainers)
Most adopter learnings involve the same human who designed the framework. contribute.md explicitly flags this as the strongest weakness and requests independent adopters. Framework-level claims are largely Empirically informed, not Empirically validated.

Source: contribute.md, use-cases.md maturity section

2. Core gates are advisory, not structural
Gate 1 and Gate 2 depend on governor engagement and audit-trail discipline. An agent (or rushed human) can skip them; detection is retrospective via audit gaps, not automated blocks. The docs are honest about this (roles-and-gates.md failure-mode table), but adopters expecting mechanical enforcement will be disappointed.

Source: Flow 07.08 — advisory vs structural table

Evidence status: Empirically informed (foundry self-application)

3. Governor capacity is a hard dependency
Without 5–15+ minutes per non-trivial change from a named human, AGF degrades to lightweight ceremony. The framework correctly says "wait" in that case, but many teams underestimate this cost.

Source: should-you-adopt.md, adopter-faq.md, downstream-impact.md

4. Multi-project federation is hypothesis only
Organizations needing cross-repo governance, shared learnings, or governor hierarchies must wait. This is Level 1 — Planned.

Source: future/evolution.md, should-you-adopt.md "Wait if…"

5. Delegation semantics are docs-only in v2.x
Principal-bound AI work uses kind: note with optional delegation/contestation objects that tooling does not semantically enforce. Adopters must treat this as audit convention, not validator-backed contract.

Source: Flow 07.08 (roles-and-gates.md), future/evolution.md

6. Conformance suite not yet shipped
reference/tooling-conformance.md describes a future agf/conformance/ fixture suite. Until it ships, only the Rust release binary is the supported implementation; alternative language ports cannot claim AGF compatibility.

Source: reference/tooling-conformance.md

7. Partial validator coverage for agents and learnings
downstream-impact.md states agent and learning validators remain contract targets until their commands ship, even though agov validate is listed as release surface. Scope of what agov validate subcommands actually enforce vs. plan needs verification against live binary.

Source: Flow 07.11, reference/tooling-conformance.md

8. Coordination semantic validator gap
Schema validation, status, graph, and visibility tooling exist (Level 5), but semantic chain integrity (wrong approval referenced, unresolved findings, incomplete stories) is identified as the most concrete known validator gap — still hypothesis.

Source: future/evolution.md

9. Adoption overhead for small changes is real
Even fast-track (≤3 files, low risk) requires a recorded waiver. Non-trivial work needs plan file + Gate 1 + Gate 2 + MR workflow. For high-velocity prototyping inside a governed repo, friction is intentional but costly.

Source: downstream-impact.md, roles-and-gates.md

10. Release hardening deferred
Installer scripts, release signing, SBOM generation, and third-party notices are explicitly deferred follow-on work.

Source: Flow 07.24 (reference/release-artifacts.md)

11. CC-BY-SA 4.0 share-alike on framework content
Framework content is CC-BY-SA 4.0 (commercial licensing available separately). Some organizations may hesitate to embed share-alike governance artifacts in proprietary repos without legal review.

Source: reference/release-info.md

Improvement proposals
#	What to change	Why	Target file/section
1
Add a "minimum viable adoption" one-page checklist (framework files only, no CI, one fast-track change) distinct from full commissioning
Reduces perceived afternoon install barrier; install-and-layouts.md is thorough but long for evaluators doing a pilot
docs/start/first-governed-change.md or new docs/start/minimum-pilot.md
2
Publish independent adopter case summaries (even anonymized) with explicit governor≠author separation
Directly addresses the single-developer evidence gap the project already names
contribute.md, use-cases.md
3
Ship agf/conformance/ fixtures with CI gate before claiming Level 5 for new commands
Prevents wrapper/README advertising ahead of binary capability; aligns with tooling-conformance contract
agf/docs/contracts/tooling-conformance.md, new agf/conformance/
4
Clarify agov validate subcommand maturity in one table (agents, learnings, infosec, contributions — each with Level 1–5)
Resolves ambiguity between "release surface" listing and "contract targets" language in downstream-impact
reference/tooling-conformance.md, docs/adopt/downstream-impact.md
5
Add advisory-gate drift detector (CI warning, not block): flag merges lacking Gate 1 plan reference or Gate 2 decision-record
Closes the most common advisory failure modes without claiming full semantic validation
future/evolution.md → promote when evidenced; interim in docs/advanced/contracts-and-validators.md
6
Document Cursor-specific loader verification as a first-class worked example (not just reference implementation = Claude Code)
Non-Claude-Code evidence is explicitly requested; Cursor is named throughout but lacks a dedicated walkthrough
docs/adopt/assistant-loader-configuration.md
7
Add governor time ROI calculator or worked example (minutes spent vs. incidents caught)
Helps teams justify the 5–15 min/charge cost with concrete foundry/adopter numbers
docs/adopt/downstream-impact.md or tooling-and-tokens.md
Uncertainty
The following could not be verified in this review:

Exact schema contents — agf/schemas/coordination-message.schema.json and learning-record schemas were not opened; coordination claims rely on dossier summaries.
Live agov execution — No binary was run; command behavior, exit codes, and JSON output shapes are taken from documentation only.
Rule library depth and quality — Individual rules under agf/rules/ were not read; assessment of rule usefulness is based on docs descriptions and evidence model, not rule text.
Independent adopter count and corroboration — Learning records (agf/learnings/*.yaml) and codename corpus were not inspected; cannot confirm how many adopters are truly independent of the author.
Current GitLab Releases page — Whether platform binaries for 2.2.6 are published and checksum/provenance artifacts match release-info was not fetched.
Assistant loader behavior in Cursor specifically — Whether AGF rules load reliably across Cursor versions/sessions without duplication into .cursor/rules/ was not tested in a live repo.
Fast-track waiver enforcement — How teams actually apply the ≤3-file fast track in practice beyond documentation convention.
Reviewer checklist (Flow 06)

 Confirmed canonical repository and published docs URL

 Recorded framework content version 2.2.6 and commit 320405f3 from Release information

 Listed pages and files actually reviewed (see Scope above)

 Cited schema files for schema claims — partial (named paths only; files not read)

 Used capability maturity labels for feature claims

 Used evidence status labels for framework-level claims

 Separated shipped Level 5 commands from roadmap/hypothesis items

 Stated uncertainty and unread areas

 Filed findings via External Review issue — not done (review delivered to user; can be filed at gitlab.com/janipaijanen/agents-govern-framework if desired)
Verdict matrix
Dimension	Rating	Notes
Conceptual soundness
Strong
Role separation, immutable audit trail, evidence-gated rules are well-founded
Documentation quality
Strong
Especially dossier, adopt/wait/skip, maturity vocabulary
Shipped tooling (core validators)
Strong
Level 5 deployable surface for coord/plan/validate/status/graph
Empirical validation breadth
Weak–Moderate
Honest about limits; needs independent adopters
Mechanical enforcement
Moderate
Structural for schema CI; advisory for gates
Enterprise / multi-repo scale
Not ready
Federation is hypothesis
Adoption cost vs. benefit (long-lived AI-heavy repos)
Favorable
If governor exists and audit trail matters
Adoption cost vs. benefit (exploratory / no governor)
Unfavorable
Docs agree
If you want this submitted upstream as an External Review issue on the GitLab repo, say the word and I can format it for their issue template. I can also run a live pilot assessment against your local checkout at /home/devi/src/AGF-NG/agents-govern (note: that tree is documented as historical 1.x reference material, not the current v2.2.6 agf/ layout).

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Lessons Learned

> **For adopters:** This is a narrative summary of what we learned
> building and deploying this framework. It is **reference-only** —
> read it for context, but it does not contain steps you need to follow.

> 📖 **Looking for the structured corpus?** This is the *narrative*
> summary; the *structured* per-project corpus is on the docs site at
> [janipaijanen.gitlab.io/agent-governance/evidence/](https://janipaijanen.gitlab.io/agent-governance/evidence/).
> Each adopter project has its own page; learnings are also indexed
> by category (gap, validation, tension, adaptation, overkill).

What we learned while creating a governance framework for multi-agent software
development, and what the first ten projects that adopted it taught us.

---

## How It Started

A blog post. Someone ran 33 AI agents and discovered that the hard problem
isn't capability — it's governance. Five alignment problems emerged:
authority without boundaries, capability drift, accountability gaps,
local optimization causing global misalignment, and knowledge decay.

We read that and asked: how would this apply to software development
specifically? Not 33 agents doing everything, but focused agents for
planning, coding, testing, deploying, and monitoring — with clear
boundaries and separation of concerns.

We didn't know where to start.

---

## How It Evolved

### Phase 1: The Framework (theoretical)

We began by writing `framework.md` — a technology-agnostic blueprint.
Ten agents across three tiers, five boundary principles, four interaction
patterns, a four-phase growth path. It was clean, logical, and entirely
theoretical.

**Lesson: Starting from principles is necessary but not sufficient.**
The framework gave us vocabulary (verb test, artifact ownership, escalation
over improvisation) and structure (tiers, pipeline topology). But it was
a document, not a practice.

### Phase 2: The Questions That Changed the Framework

Then we started asking: what about UX and accessibility? What about
localization? What about data quality and immutability? What about PII
and data subject rights? What about security?

Each question exposed the same gap: **the pipeline model misses cross-cutting
concerns.** A linear pipeline (plan → design → code → test → deploy →
observe) assumes every concern maps to a stage. But accessibility affects
every stage. Localization affects every stage. Security affects every stage.
These are lenses, not steps.

This produced the most significant structural change to the framework:
a new agent tier (Cross-Cutting) and the Lens Principle. The framework
grew from 10 agents to 19, from 3 tiers to 4, from 5 principles to 6.

**Lesson: Cross-cutting concerns are the governance problems that pipeline
models hide.** If you only think in stages, you'll miss everything that
spans stages. The UX-Accessibility, Localizer, Data Guardian, Privacy
Guardian, and Security Guardian agents exist because no pipeline agent
can own what every pipeline agent must respect.

### Phase 3: Eating Our Own Dog Food

We wrote `GOVERNANCE.md` — applying the framework to its own development.
This forced us to answer concrete questions:

- Who plays which role? (Jani as Human + Governor-Reviewer, Claude as
  Planner-Architect + Coder-Tester)
- What's the pipeline? (plan mode → approve → implement → verify → commit)
- What are the gate rules? (no implementation without plan approval)

**Lesson: Applying the framework to itself revealed that it works for a
team of one human and one AI.** The framework doesn't require a team of
ten. It requires roles, boundaries, and gates — regardless of how many
people or agents fill them.

### Phase 4: The Learning System

We realized the framework would drift into theory without a feedback
mechanism. How would it improve? What evidence would drive changes?

This produced `LEARNINGS.md`, the project record template, and the
learning categories (gap, overkill, validation, adaptation, tension).
The key design decisions:

- **Learnings are structured, not freeform.** Date, category, severity,
  business impact, recommendation. This makes them comparable across
  projects.
- **Not every learning triggers a change.** Decision criteria prevent
  bloat: require corroboration from a second project, prefer simplification
  over addition, mark unvalidated content.
- **The framework needs a Sentinel.** Someone must periodically ask: is
  the framework getting better or just bigger?

**Lesson: A framework without a feedback loop is a snapshot. A framework
with structured evidence collection is a living system.**

### Phase 5: Real Projects

Then projects adopted the framework:

| Project | Domain | Stack | PII | Test Coverage Start → Now |
|---------|--------|-------|-----|--------------------------|
| silver-birch | SaaS project mgmt | Elixir/Phoenix | Yes | 85% → 95% (1181 tests, 41 e2e) |
| copper-rapids | SaaS project mgmt | Python/Flask | Yes | 49% (1816 tests) |
| ember-clock | CLI calendar tool | Python | No | 5% → 70% (136 tests) |
| quiet-river | CLI data processing | Python | No | 0% → 70-80% (80 tests) |
| north-wind | IoT sensor system | Python/FastAPI/Elm | Borderline | 40% |
| glass-key | CLI passphrase generator | Python | No | 5% |
| iron-gate | Cybersecurity exercise SDL | CUE/JSON-LD/SHACL | No | N/A (schema project) |
| frost-quill | Academic publication | Markdown/YAML/LaTeX | No | N/A (document project) |
| pine-ridge | Evaluated open-source project | Python | No | N/A (fork-and-evaluate) |
| stone-arch | Enterprise CRM | Python | Yes | 100% (1776 tests) |

~143 learnings accumulated. This is where theory met practice.

---

## What the Evidence Shows

### What works (validations)

**The Security Guardian checklist is universally valuable.** 6/6 projects
activated it. It produced `prevented_loss` findings in 4 of 6 projects:
- silver-birch: PII (email addresses) logged in plain text in 5 Logger calls
- copper-rapids: host header injection allowing SQL injection via subdomain routing
- north-wind: admin endpoints with no authentication
- glass-key: passphrase generator using predictable random (Mersenne Twister)

The glass-key finding is the highest-impact single discovery: a security
tool defeated by its own use of insecure randomness. The Step 0.3 checklist
caught it before any user ever generated a passphrase.

**Phase 1 combined agents work for real tasks.** Two independent projects
(ember-clock and quiet-river) ran their first task through the full
pipeline and reported it worked naturally. The plan document forced explicit
scoping. The gates caught nothing problematic — which means the plans were
well-bounded.

**The pipeline drives real improvements.** Three projects went from minimal
or zero test coverage to 70%+ through their first governed task. The
framework didn't just document the gap — the pipeline gate ("all tests
pass") made the gap non-ignorable.

**Existing governance maps to the framework.** copper-rapids (Python) had
mature `.claude/rules/` files that mapped 1:1 to cross-cutting checklists.
Framework adoption was documentation, not transformation. This validates
that the framework captures real governance patterns, not invented ones.

**The Planner/Coder boundary extends naturally to documentation.** When
copper-rapids assigned documentation to agents, the boundary fell out without
any new principles: the Planner decides *what pages should exist* (gap
analysis, page inventory, review prioritisation), while the Coder *writes
the words*. This mirrors the code boundary exactly (Planner decides what
to build, Coder builds it). The same test — "can you separate 'decide
what should exist' from 'produce it'?" — should apply to any new artifact
type before introducing new agents. This drove the "Assigning new artifact
types" guidance added to Principle 2 in v0.11.0.

**Domain-specific review checklists fit a consistent structure.** When
copper-rapids added documentation review to the Governor-Reviewer, the initial
ad-hoc structure (`documentation_review:` at YAML root) was immediately
flagged as non-extensible. Restructuring as `review_checklists.documentation:`
with a standard format (scope, per_change items, per_milestone items)
established a convention that will hold when security, performance, or other
domain-specific checklists are added later. The `review_checklists` block
was added to the agent definition template in v0.11.0.

### What was missing (gaps)

**Per-change checklists miss structural issues.** The accessibility
checklist caught individual violations (missing ARIA labels, contrast)
but missed "the app has no navigation" and "Phoenix scaffold is still in
the production header." These are system-level issues that no individual
PR introduced.

This drove the first evidence-based framework revision: two-cadence
cross-cutting review (per-change at every Gate 2, per-milestone at
defined milestones).

**Documentation is not enforcement.** north-wind documented its unauthenticated
admin endpoints in an ADR. copper-rapids documented its host header injection
risk. But nothing in the framework prevented either project from progressing
to beta with those risks unresolved. An ADR describes a choice. A risk
acceptance creates accountability.

This drove the second evidence-based framework revision: lifecycle gates
with formal risk acceptance (name, date, rationale, deadline).

**Schema-vs-code parity is a recurring problem.** silver-birch found 12
schema mismatches across two sessions — column renames, missing audit
columns, nullable mismatches, pluralization errors. All caught by
integration tests, not by code review. An automated schema parity check
in CI would catch these earlier.

**DSAR is user-scoped, not entity-scoped.** silver-birch's data export was
org/project-centric. GDPR requires user-centric export. The Privacy
Guardian checklist now explicitly asks for user-scoped data export, not
just "can you export data."

**Review checklist items can violate the reviewer's own boundaries.**
copper-rapids's Governor-Reviewer checklist item "CI green: make ci-pages
passes" was ambiguous — it could be read as an instruction to *run*
`make ci-pages`, which would violate `denied: Execute builds`. The fix
was simple (rephrase as "verify Coder-Tester's make ci-pages self-check
passed"), but the gap is structural: review checklists are written by
the same people who define boundaries, and they unconsciously phrase items
as actions rather than observations. This drove a convention note on the
Reviewer agent in v0.11.0: all checklist items must be phrased as
verifications, never as actions.

**Feature documentation has no per-change gate.** copper-rapids found that
Pattern 7 (documentation review pipeline) provides review checklists but
no universal gate requiring documentation updates when a user-visible
feature is added, modified, or removed. A commit can add a new route,
change a UI label, or remove a setting and pass Gate 2 with zero
documentation update. copper-rapids fixed locally with a feature-documentation-
lifecycle rule. Single-project evidence; added as a recommendation to
Pattern 7 in v0.11.0, watching for corroboration.

**Demo and staging environments are ungoverned artifact classes.**
copper-rapids's recent releases (3.0.29–3.0.32) were heavily demo-focused:
seed scripts, brand blueprints, reverse proxy configs, maintenance pages,
reset scripts. None of these were covered by any agent's jurisdiction.
Demo infrastructure is a production-facing artifact (evaluators see it)
with no review gate. Single-project; watching for corroboration before
adding to the artifact ownership table.

**A three-level maturity model is too coarse for honest capability claims.**
frost-quill's conference paper claimed "Yes" for physical security when the
original codebase only had schema-defined types — no scenarios, no validation, no
deployment. The existing Implemented/Specified/Planned model couldn't distinguish
between fully deployable domains and domains that exist only as type
definitions. A five-level model was developed: (5) Deployable, (4) Validated,
(3) Instantiated, (2) Schema-defined, (1) Planned. iron-gate independently
confirmed the need: 149 types across 9 domains but only network infrastructure
reaches Deployable. This model is proposed for framework adoption in the
implementation-honesty-rule (Sentinel Review 5).

**Iterative structured review with role separation produces measurable
improvement, even when both producer and reviewer are AI.** frost-quill's
academic-peer-reviewer conducted four review rounds. Each round used structured
severity classification, specific section references, and actionable
recommendations. The key enabler: the reviewer writes to a separate artifact
(docs/reviews/) rather than editing the document directly. This validates the
framework's core principle that separation between producer and reviewer roles
matters regardless of who fills them.

### What surprised us (adaptations)

**The framework scales down as well as up.** CLI tools (ember-clock, quiet-river,
glass-key) skip 5-6 agents with documented reasons. The same template
handles a 1086-test SaaS application and a zero-test CLI utility. We
evaluated whether a "solo-developer profile" was needed and decided no —
the framework scales by project complexity, not team size.

**Projects embed Step 0 assessments inline in the YAML.** quiet-river and
north-wind recorded their full PII, security, maturity, and environment
checklists directly in the learnings file. This wasn't in the template
but became a natural adaptation.

**When e2e tests require impractical waits, unit coverage is acceptable.**
copper-rapids's SEC-07 session timeout would need an 8-hour e2e test. The team
accepted unit-level coverage and documented the decision. The framework
should acknowledge this pattern.

**Pushed release tags must be immutable.** copper-rapids discovered that
iterating fast on a deployment (demo site) created temptation to delete
and recreate tags at different commits. The consequence: broken CI caches,
mismatched container images, violated trust. The rule is simple — a pushed
tag is immutable; if a tagged release needs a fix, bump the version and
tag the new commit. This was added as a forbidden action on the Deployer
agent in v0.11.0.

**Supply-chain hardening benefits from concrete detail.** copper-rapids's
security-hardening rule contains explicit supply-chain checklist items:
pinned installer URLs (not unversioned `curl | sh`), pinned container
image tags (not `:latest`), pinned CI tool versions, lockfile integrity,
Trivy scan on release. This level of specificity — naming the *shape* of
the check, not the specific tool — is more actionable than the framework's
current Security Guardian guidance. Offered as a reference pattern for
adopting projects.

### What created tension

**Pipeline ceremony vs. low-risk tasks.** quiet-river completed its
first formal pipeline run (64 tests), then immediately did three follow-up
tasks without the formal pipeline — README update, CLI integration tests,
agents-govern commit. The Governor-Reviewer chose the lighter path
naturally. This isn't a violation — it's the framework working as intended
at Phase 1 where the human has discretion. But it reveals the need for a
formal "fast track" for low-risk tasks.

**Fuzz tests find boundary cases faster than they can be understood.**
copper-rapids's Decimal arithmetic fuzz tests failed three times at different
boundaries. Each fix addressed the specific falsifying example rather
than the underlying error distribution. The recommendation: model the
error mathematically rather than widening thresholds one case at a time.

### What we learned about information security (v0.15.4–v0.19.0)

**Anonymizing the project name is necessary but not sufficient.**
Replacing the real name with a codename protects against direct
identification, but the combination of other public fields (business
domain, tech stack, team size, regulatory context, lifecycle state) can
re-identify a project through cross-field correlation. iron-gate scored
4/6 on the specificity scale from public fields alone: niche domain +
specific regulations + solo developer + non-standard lifecycle. The
InfoSec Sentinel's cross-field scoring (Category C) exists to catch this.

**Individual learning entries are safe but the aggregate is not.**
silver-birch's learning entries individually mentioned generic terms
(API, database, OIDC, workflow). But across all entries, 8 distinct
architecture, integration, and business process terms emerged — enough to
reconstruct the system design when combined with the known domain and
stack. The InfoSec Sentinel's cumulative analysis (Category D) exists
to catch this pattern: the risk is in the sum, not the parts.

**Cross-record linkage de-anonymizes both projects.** When silver-birch's
learning entries referenced copper-rapids by codename ("predecessor
project"), and frost-quill referenced a related repository, each link
created a chain: anyone who identifies one project can identify the
other. Codenames protect individual projects but not relationships.
The sentinel flags these links for human review — the learning value of
documenting a rewrite or publication relationship may outweigh the
linkage risk, but the human must make that call.

**Git history remembers everything.** After scrubbing all current files
of real names (v0.17.0), every past commit still contained them. `git log`,
`git show`, and `git blame` exposed the full history. This required
`git filter-repo` to rewrite all 91 commits and 10 tags. Lesson: if
information security matters, scrubbing HEAD is not enough — history is
the real attack surface.

**Release artifacts can diverge from HEAD.** The APA citation was added
to main after the v0.18.1 tag was created. The release archive (built
from the tag) did not contain the citation. The fix required a patch
release (v0.18.2). Lesson: verify release artifacts against the actual
download, not against the local working tree. The dist.sh release
checklist now captures this.

**Detection heuristics must be public; detection reasoning must not.**
The InfoSec Sentinel's `validate-infosec.sh` runs in CI, so its patterns
are visible in the public repo. Anyone can read what the script checks
for. But the *reasoning* behind why certain combinations are dangerous —
the threat model, the re-identification attack paths, the sensitivity
classifications — cannot be published without creating an evasion
roadmap. This is an inherent tension: the enforcement mechanism is
transparent but the security strategy is not. The framework resolves
this by embedding heuristics in the script and declining to publish a
formal information security guide.

**The human gate is essential because classification is context-dependent.**
The same information may be safe for an open-source project but sensitive
for a proprietary one, or for a publicly funded project under embargo.
The sentinel cannot know the submitter's publication status, funding
restrictions, or competitive context. Every finding requires human
judgment — not because the detection is uncertain, but because the
classification depends on context only the human has. The `sentinel_ack`
mechanism makes this judgment explicit, auditable, and versioned.

**Agent-assisted remediation works when the human has final authority.**
The InfoSec Sentinel proposes redactions (e.g., "replace connection
string with [database connection string redacted]") but cannot apply
them. The human approves the redaction, and the artifact owner applies
it. This Option C flow — propose, approve, apply — prevents the sentinel
from damaging the analytical value of learning records while still
providing actionable suggestions. The pattern is reusable for any
cross-cutting concern that needs to balance quality enforcement with
content preservation.

**Blanket git history rewriting breaks projects where the product name
is the codebase.** copper-rapids discovered that `git filter-repo` with
a text replacement of the product name corrupted URLs, package names,
CSS themes, database references, email addresses, and deployment configs
throughout the entire git history. The rewrite was caught before force
push — if pushed, it would have broken CI, deployment, and the package
registry. The lesson: anonymization must distinguish between projects
where the name is confidential (rewrite history) and projects where
the name is the public brand (anonymize only the learning record, not
the source code). A pre-flight checklist was added to the anonymization
notice. *(Evidence: copper-rapids, 2026-03-29, critical.)*

### What reached corroboration (v0.19.2)

**Step 0 assessment is the highest-value standalone component.**
silver-index and silver-birch independently reported that the Step 0
PII and security checklists (ADOPTING.md Steps 0.2–0.3) systematically
surfaced gaps that informed every subsequent decision — even without
full framework adoption. Two independent projects now confirm this.
Consider promoting Step 0 as a standalone project-evaluation tool.

**Principles vs. ceremony is a real distinction.** silver-index found
that framework principles (Verb Test, Escalation, Artifact Ownership)
delivered value while formal artifacts (YAML definitions, GOVERNANCE.md,
rules files) were unnecessary at evaluation scale. This is the first
evidence distinguishing "adopt the principles" from "adopt the
artifacts" as separate tiers. It may explain the zero-overkill finding:
the skip mechanism handles ceremony, and principles are inherently
non-overkill because they're behavioural, not procedural. Single-project
tension — watching for corroboration, but the signal is strong.

---

## What We Got Right

1. **The verb test.** No two agents share a primary verb. This simple
   rule prevented every jurisdiction dispute before it happened. Eleven
   unique verbs across 19 agents, zero conflicts.

2. **Artifact ownership.** Every artifact has exactly one owner. This
   eliminated the "who changed this?" problem. When silver-birch found
   schema mismatches, it was immediately clear which agent owned the fix.

3. **Escalation over improvisation.** When agents encounter out-of-scope
   situations, they stop and ask. No agent "helpfully" handled something
   outside its mandate. This prevented capability drift across all six
   projects.

4. **The growth path.** Starting with 3 combined agents and splitting when
   it hurts is the right approach. No project has needed to split yet.
   The combined agents are working. When they stop working, the split
   signals will be visible.

5. **The learning system.** Structured learnings with categories, severity,
   and business impact made evidence comparable across projects. The
   Sentinel review process caught a false signal (solo-developer profile)
   and correctly decided not to act. Two real signals (per-change vs.
   per-milestone reviews, documentation vs. enforcement) correctly drove
   framework revisions.

6. **The Planner/Coder boundary generalises to new artifact types.** When
   copper-rapids needed to assign documentation ownership, the same boundary
   pattern applied without modification: Planner decides what pages should
   exist, Coder writes the words. The test — "can you separate 'decide
   what should exist' from 'produce it'?" — works for any artifact class.

## What We Got Wrong

1. **We underestimated cross-cutting concerns.** The original framework
   had zero cross-cutting agents. Every question from Jani ("what about
   UX?", "what about localization?", "what about data quality?", "what
   about PII?", "what about security?") added one. The framework grew
   from 10 to 19 agents because we missed an entire tier.

2. **We didn't distinguish per-change from per-milestone reviews.**
   The original checklist design assumed everything could be caught at
   the PR level. It took silver-birch shipping with Phoenix scaffold in
   the production header to learn that structural issues need periodic
   holistic review.

3. **We didn't have lifecycle gates initially.** The framework had
   environment deployment gates but nothing preventing a project from
   moving to beta with known critical security gaps. Documentation felt
   like governance until north-wind showed that an ADR is not a gate.

4. **We didn't version the framework from the start.** Versioning and
   agent definition IDs were added after three projects had already
   adopted. Retrofitting version tracking is messier than including it from the start.

---

## By the Numbers

**Framework evolution:**
- Started: 10 agents, 3 tiers, 5 principles, 4 growth phases
- Now: 19 agents, 4 tiers, 9 principles, 5 growth phases
- 7 framework revisions (two-cadence review, lifecycle gates, Pattern 6 fast track, tier selection guidance, template assessment blocks, ADR lifecycle + seek-disagreement, v0.11.0 boundary conventions)
- 5 Sentinel reviews completed (Review 1: solo-developer profile — no action; Review 2: three signals triggered v0.6.0; Review 3: Pattern 7 genericized; Review 4: reframed schema signal, added .claude/rules/ tip; Review 5: anonymization shipped, five-level maturity model proposed, license change recommended)

**Across 13 projects:**
- ~130 learnings recorded
- ~24 validations (framework correctly predicted or prevented problems)
- ~33 gaps (framework missed something or projects found issues)
- ~13 adaptations (projects adapted the framework in useful ways)
- 0 overkill (no project found the framework too heavy — see note below)
- 5 tensions (pipeline ceremony, fuzz test boundaries, self-referential metrics, schema breadth vs depth, principles vs ceremony)
- 9+ formal risk acceptances (3 copper-rapids, 5 silver-birch, 1 north-wind)

**Security Guardian impact:**
- Activated in 7+ projects
- Produced `prevented_loss` findings in 4+ projects
- Highest-impact finding: insecure RNG in a passphrase generator

**Test coverage driven by framework adoption:**
- quiet-river: 0% → 70-80%
- ember-clock: 5% → 70%
- silver-birch: 85% → 95% (735 → 1181 tests, 41 e2e)
- copper-rapids: 49% (1816 tests)

---

## What Comes Next

*See governance.md Sentinel Review Log for authoritative decisions.*

**Shipped since last update (v0.6.0):**
- Fast-track pipeline (Pattern 6): Governor-Reviewer can waive Gate 1 for small, low-risk tasks. Gate 2 still applies. >30% fast-track rate is a governance decay signal.
- Tier selection guidance: default to integration for CRUD/RBAC; build route-by-tier matrix before writing tests.
- Template assessment blocks: optional Step 0 checklists in `learnings/_template.yaml`.
- Documentation governance (Pattern 7): Coder owns content, Planner owns structure, Reviewer applies per-change and per-milestone checklists. Genericized after review found copper-rapids specifics had leaked into universal files.

**Shipped in v0.11.0:**
- ADR lifecycle and seek-disagreement norm: ADRs follow an immutable lifecycle (`proposed` → `accepted` → `superseded`). Required fields include counter-arguments sought, strongest objection, and reevaluation triggers. Architect must actively seek disagreement before finalizing decisions. Reviewer validates that counter-arguments are genuine, not rubber-stamped. Sentinel gains ADR health as a periodic checkpoint. Triggered by analysis of Harmel-Law's "Scaling Architecture Conversationally" and Fowler's ADR bliki — external research, not project evidence. Key insight adopted: the Advice Process's consultation-before-decision model, adapted to agent boundaries (consultation supplements but does not replace gate reviews; decision ownership stays with the Architect per Verb Test).
- Reviewer checklist phrasing convention (Sentinel Review 5): all items must be verifications ("verify X passed"), never actions ("run X"). Prevents reviewers from violating their own denied list.
- `review_checklists` convention in agent definition template (Sentinel Review 5): standardised structure (scope, per_change, per_milestone) for domain-specific Gate 2 additions.
- New artifact type assignment guidance (Principle 2, Sentinel Review 5): test Planner/Coder boundary ("decide what should exist" vs "produce it") before introducing new agents.
- Immutable release tags (Sentinel Review 5): added as a forbidden action on the Deployer. A pushed tag must never be deleted, moved, or recreated.
- Feature documentation lifecycle (Sentinel Review 5): recommendation in Pattern 7 that user-visible feature changes trigger documentation updates. Pre-customer products show current state only — no deprecation notices.

**Active backlog (awaiting corroboration or trigger):**

**Five-level domain maturity model — proposed for framework (Review 5).**
frost-quill and iron-gate independently demonstrated that the three-level
Implemented/Specified/Planned model causes overclaiming. The five-level model
(Deployable / Validated / Instantiated / Schema-defined / Planned) provides
the granularity needed for honest capability communication. Pending approval
for integration with the implementation-honesty-rule in framework.md.

**Anonymization policy — shipped in v0.15.4.** All learning records must use
codenames. CI validation enforces structural compliance on MRs. 8 existing
records pending migration (deadline: 2026-06-05). First migration completed
(iron-gate record anonymized).

**Academic publication agents — watched (Review 5).** frost-quill created
academic-writer and academic-peer-reviewer agents. Real gap but single-project
and domain-specific. Templates available as reference.

**Security testing exemption for exercise/CTF platforms — watched (Review 5).**
iron-gate needs automated security gates to handle intentionally vulnerable
code. Covered by ADR 20260405. Domain-specific; awaiting corroboration.

**Zero overkill at ~143 learnings — noted but not actionable.** Review 2
flagged this for investigation at 50 learnings. Review 4 evaluated it:
all 13 projects are by the same developer who designed the framework.
The overkill signal may only emerge from an independent adopter. Three
plausible explanations: the skip mechanism works, solo-developer-with-AI
doesn't feel ceremony, or single-developer bias. Cannot resolve until
a second developer adopts.

**Schema integrity automation (reframed).** Previously watched as "schema
parity CI check — silver-birch only." Review 4 reframed the signal:
silver-birch had 12 ORM-vs-DB mismatches; copper-rapids had broken Alembic
migration chains from hand-written revision IDs. Related but distinct
failure modes. Both fall under "schema-related problems that automation
catches earlier than code review." Awaiting a third independent data
point before framework change.

**The testing hierarchy hasn't been split.** All six projects use combined
Tester agents. No project has hit the split signal (bugs escaping one
level that should have caught them). This is expected — most projects are
in Phase 1. The testing hierarchy (unit, integration, e2e, system as
separate agents) is Phase 5 material.

**Documentation governance awaits a second adopter.** Pattern 7 was
promoted from one project's needs. The generic skeleton is sound, but the
per-change and per-milestone checklists need validation from a project with
different documentation tooling (not GitLab Pages, not bilingual).

**Demo/staging as ungoverned artifact class (new).** copper-rapids's demo
infrastructure (seed scripts, brand config, reverse proxy, maintenance
pages) had no agent ownership. Single-project; watching for corroboration.

**Uncommitted files before CI push (new).** silver-birch lost 6 CI
iterations debugging symptoms of 97 uncommitted source files. Single-project;
watching for corroboration.

**Supply-chain hardening reference implementation (new).** copper-rapids's
security-hardening rule has concrete supply-chain items (pinned installer
URLs, pinned image tags, pinned CI tool versions, lockfile integrity). More
actionable than the framework's current Security Guardian guidance. Available
as a reference pattern for adopting projects.

**Signals approved for retirement at Review 6 (no new evidence since Review 2):**
BASELINE.md pattern (silver-birch), ADRs at inception (silver-birch), GitLab CI
token checklist (north-wind), mathematical error bounds for fuzz tests (copper-rapids).
All four watched for 4 consecutive reviews without corroboration. Approved for
retirement at Review 6 per Sentinel Review 5.

**Stale Git branches bypass anonymization.** A pre-anonymization branch
(`version-file`) remained on the public GitLab repository after all
learning records on `main` were migrated to codenames. The branch
contained the original files with real project names — effectively a
complete codename-to-real-name mapping for all 8 anonymized projects,
publicly readable. Discovered during a routine branch audit, not by
any automated check. The anonymization policy (ANONYMIZATION-NOTICE.md)
and infosec sentinel script both operate on file contents in the current
branch; neither checks for stale branches carrying pre-migration data.
The branch was deleted immediately. *(Evidence: agents-govern self-audit,
2026-04-09, critical.)*

**Lesson:** Anonymization is not complete until stale branches and tags
are audited. Add a post-migration step to ANONYMIZATION-NOTICE.md:
"After migrating learning records to codenames, delete all branches that
carried the pre-anonymization files. Check with
`git branch -r` (remote) and `glab api` or the GitLab UI."

**The framework's own governance works.** We eat our own dog food. The
pipeline (plan → approve → implement → verify → commit) governs every
change to the framework itself. The learning system feeds real evidence
into revisions. The Sentinel reviews prevent bloat. The versioning
enables adopting projects to upgrade at their own pace.

The framework that cannot govern itself has no business governing
anything else. So far, it can.

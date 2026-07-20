# AGF-NG framework assessment — GPT-5.6 Sol High

**Review label:** critique + fit-assessment + improvement-backlog

**Review date:** 2026-07-20

**Reviewer:** OpenAI Codex assessment using `gpt-5.6-sol` at high reasoning
effort

**Overall evidence status:** Empirically informed. The framework and tooling
were inspected in the project that develops and dogfoods them, but the claimed
organizational benefits have not been independently validated by this review.

## Scope reviewed

Reviewed project: **agents-govern / AGF-NG**, framework content version
**2.2.6**. The local review checkout was
`6609590a19103e37d02be33542348d503d32e9bc` on branch
`docs/codex-sol-high-framework-assessment`; the locally built release-mode
binary identified itself as `agov 2.2.6` built from commit `086863f`.

The published evaluation page displayed version 2.2.6 and documentation commit
`fc4de2fe` when read on 2026-07-20. Those identifiers are recorded separately
because source checkout, built binary, published documentation, and downloadable
release are distinct evidence surfaces.

The review followed the thorough-assessment route on the published
[`evaluate` page](https://janipaijanen.gitlab.io/agents-govern-framework/evaluate/).
The following canonical documentation and source areas were inspected:

| Area | Specific source |
|---|---|
| Project overview and boundaries | `agf/docs/site/index.md`, `agf/docs/site/docs/concepts/position.md` |
| Adoption decision | `agf/docs/site/docs/start/should-you-adopt.md` |
| Roles and gates | `agf/docs/site/docs/concepts/roles-and-gates.md`, `agf/agents/MANIFEST.yaml` |
| Adoption cost and reversibility | `agf/docs/site/docs/adopt/downstream-impact.md`, `agf/docs/site/docs/adopt/exit-and-decommission.md` |
| Evidence model | `agf/docs/site/docs/concepts/evidence-and-learnings.md` |
| Maturity vocabulary | `agf/docs/site/reference/maturity-vocabulary.md` |
| Tooling contracts and maturity | `agf/docs/site/reference/tooling-conformance.md`, `agf/docs/site/docs/advanced/contracts-and-validators.md` |
| Operator workflow | `agf/docs/site/docs/operate/day-to-day.md`, `agf/docs/site/docs/operate/review-and-release.md` |
| MCP adoption surface | `agf/docs/site/docs/adopt/mcp-server.md`, `tooling/rust/crates/agov-mcp/src/main.rs` |
| Schema contracts | `agf/schemas/coordination-message.schema.json`, `agf/schemas/gate1-plan.schema.json`, `agf/schemas/learning-record.schema.json` |
| Rules and role definitions | `agf/rules/`, `agf/agents/` |
| Tooling implementation | `tooling/rust/`, including the CLI, core, coordination, validator, and MCP crates |
| Live project evidence | `docs/plans/`, `docs/coordination/`, selected proposals and reviews, local Git state, and local `agov` output |
| Packaging and release claims | `agf/VERSION`, `README.md`, changelog and release documentation, public GitLab release API |

Current external positioning was compared with primary-source documentation
from GitHub, GitLab, OpenAI, Google DORA, NIST, OPA, and in-toto. This was a
read-only assessment: no independent adopter deployment, longitudinal outcome
study, penetration test, or clean-machine release installation was performed.

## Identity confirmed

- [x] Canonical repository confirmed as
  `https://gitlab.com/janipaijanen/agents-govern-framework`.
- [x] Published documentation confirmed as
  `https://janipaijanen.gitlab.io/agents-govern-framework/`.
- [x] This is not Microsoft's Agent Governance Toolkit.
- [x] This is not an agent SDK, runtime, orchestrator, hosted service, runtime
  monitoring platform, or ML/model-governance framework.
- [x] AGF is a repository-local process and evidence framework loaded by
  existing development tools and enforced partly through existing Git and CI
  controls. Source: `agf/docs/site/index.md` and
  `agf/docs/site/docs/concepts/position.md`.

## Overall judgment

AGF-NG is technically feasible and addresses a real problem: AI coding systems
increase change volume and autonomy faster than many teams can preserve
decision context, separation of duties, and review evidence.

Its strongest defensible position is:

> A cross-vendor, Git-native assurance protocol for consequential AI-authored
> software changes.

That is narrower and more credible than “enterprise agent governance.” AGF is
strongest as a repository-local layer between assistant instructions and
existing CI and merge-request controls. It is not currently a runtime policy
engine, enterprise control plane, or compliance product. Source:
`agf/docs/site/docs/concepts/position.md`.

| Dimension | Assessment |
|---|---|
| Technical feasibility | High; approximately 85% confidence |
| Usefulness in a bounded one-repository pilot | Positive; approximately 80% confidence |
| Current enterprise deployability | Low to moderate |
| Commercial product-market fit | Plausible but unproven; approximately 60% confidence |
| Ready for scaled product investment | No |
| Worth continued hardening and external pilots | Yes |

The core design is coherent. The principal risks are evidence strength,
ceremony, human-governor bottlenecks, lack of runtime enforcement,
release-documentation truth, historical-contract migration, and insufficient
external adoption proof.

## Strengths

### 1. The problem definition and theory of change are coherent

The framework reduces its mechanism to named roles, two human gates, and a
durable decision log. Its adoption test is similarly clear: use AGF when the
cost of an unreviewable AI-authored change exceeds the cost of planning,
review, and a recorded decision. Source: `agf/docs/site/index.md` and
`agf/docs/site/docs/start/should-you-adopt.md`.

The implied theory of change is plausible: explicit roles reduce authority
ambiguity; Gate 1 examines intent before implementation; Gate 2 examines
evidence before merge; repository-resident records preserve context beyond a
model session; validators detect malformed evidence; and existing Git/CI
protections provide the real merge boundary.

This theory is **Empirically informed**, not Empirically validated. Google’s
2025 DORA research supports the underlying premise that AI amplifies the
surrounding delivery system while trust and delivery-stability concerns remain,
but it does not validate AGF specifically. External source:
[Google Cloud’s 2025 DORA report announcement](https://cloud.google.com/blog/products/ai-machine-learning/announcing-the-2025-dora-report).

### 2. The architecture uses mature, portable primitives

AGF composes Markdown/YAML, JSON Schema, Git, CI, merge requests, and a
deterministic Rust CLI. MCP is an adapter over the same file-backed state, not
a second governance engine. Source: `README.md`, `agf/schemas/`, and
`tooling/rust/`.

This architecture has low infrastructure burden, works without a hosted
control plane, keeps evidence beside code, produces reviewable diffs, and can
survive changes in assistant vendor. Those properties make the core product
technically feasible.

### 3. Authority is separated from tool output

Named rules determine when a validator or CI check may block. Tools provide
evidence but do not grant themselves authority. Source:
`agf/rules/always/tool-authority-boundary-rule.md` and
`agf/docs/site/docs/advanced/contracts-and-validators.md`.

This is a strong governance design because it prevents the validator from
silently becoming its own policy authority.

### 4. Human separation of duties is structurally central

The planner, implementer, and governor responsibilities are distinct, and
binding Gate 1 and Gate 2 decisions remain human. Source:
`agf/docs/site/docs/concepts/roles-and-gates.md` and
`agf/agents/MANIFEST.yaml`.

This directly addresses a common AI-development failure mode: one assistant
becoming requester, designer, implementer, reviewer, and approver in a single
interaction.

### 5. The framework is unusually honest about its boundaries

The documentation states that AGF operates before and after execution and does
not intercept runtime agent behavior. It also distinguishes schema validation
from semantic correctness, and evidence from compliance. Source:
`agf/docs/site/docs/concepts/position.md`,
`agf/docs/site/docs/advanced/contracts-and-validators.md`, and
`agf/docs/site/reference/maturity-vocabulary.md`.

This restraint improves credibility. The current Full-profile direction also
does not falsely present plan-to-execution binding as already solved.

### 6. Adoption is reversible

The framework is primarily files and conventions, with documented pause,
downgrade, and removal paths. Source:
`agf/docs/site/docs/adopt/exit-and-decommission.md`.

This reduces lock-in and makes a bounded pilot practical.

### 7. The implementation has credible engineering foundations

The Rust workspace separates core, coordination, validation, CLI, and MCP
concerns; uses local schemas; exposes structured output; and has a material
test inventory. The reviewed tree contained approximately 12,000 lines of Rust
and 128 Rust test functions. Source: local source inventory under
`tooling/rust/`; tests were inventoried but not rerun for this assessment.

## Risks / gaps / inconsistencies

### 1. Organizational benefit remains largely dogfooding evidence

The project demonstrates that AGF can be instantiated and used to govern its
own development. It does not yet independently demonstrate that Gate 1 catches
important design errors, Gate 2 reduces escaped defects, decision
reconstruction is materially faster, or the process remains sustainable in
unrelated organizations.

This is the most important evidence gap. Framework operation is Empirically
informed; broad outcome claims remain unvalidated.

### 2. Governance can degrade into ceremony

Plans and approvals can be structurally valid while substantively weak. A
governor can rubber-stamp decisions, and a schema cannot prove genuine review.
Source: `agf/docs/site/docs/adopt/downstream-impact.md` and
`agf/docs/site/docs/operate/review-and-release.md`.

The validated single-governor model also creates a serial bottleneck and a
continuity risk when that governor is unavailable.

### 3. Enforcement is intentionally partial

| Control | Actual enforcement |
|---|---|
| Artifact shape | Mechanically enforceable |
| Required CI | Enforceable when repository settings require it |
| Human merge approval | Enforceable through Git-host controls |
| Plan quality | Human judgment |
| Plan-to-diff conformance | Currently human judgment |
| Agent role obedience | Instruction-following and retrospective review |
| Agent identity | Recorded assertion, not strong identity proof |
| Cross-machine visibility | Commit/push/fetch discipline |

Source: `agf/docs/site/docs/concepts/roles-and-gates.md`,
`agf/docs/site/docs/advanced/contracts-and-validators.md`, and the rule library.

AGF should therefore sell assurance, evidence, and process specification—not
unqualified enforcement.

### 4. The default mental model is too large for first adoption

The distribution exposes 46 rules, 12 roles, seven skills, nine coordination
message kinds, two gates, and multiple supporting record types and validators.
Even when an AI creates most artifacts, the human must understand enough to
detect low-quality evidence.

This creates a real risk that governance work grows faster than engineering
value. Source: local inventories of `agf/rules/`, `agf/agents/`, `agf/skills/`,
and `agf/schemas/`; operating model in
`agf/docs/site/docs/operate/day-to-day.md`.

### 5. Historical contract drift weakens health reporting

Local `agov` checks reported 329 historical coordination records rejected by
the current coordination schema and 11 of 76 project plans rejected by the
current Gate 1 schema. Many predate the present contract and should not be
treated as malformed current work. Source: local `agov session resume` and
validation output; current schemas in `agf/schemas/`.

A mature evidence tool needs explicit classifications such as current-valid,
valid-under-version-X, legacy/unversioned, and invalid. Otherwise operators
cannot easily tell whether excluded history undermines a query.

### 6. The MCP path contract permits plausible wrong-scope success

The MCP `agov_session_resume` field named `dir` is forwarded to CLI `--dir`.
The CLI means “coordination-message directory,” not repository root. Passing
the repository root causes root Markdown files to be scanned and returns
plausible but incomplete `ok: true` output. Passing `docs/coordination`
restores CLI/MCP semantic parity. Source:
`tooling/rust/crates/agov-mcp/src/main.rs` and direct CLI/MCP comparison.

Classification:

- algorithmic CLI/MCP divergence: no;
- MCP schema/tool-description defect: yes;
- UX and documentation defect: yes, high severity;
- implementation-hardening and acceptance-test gap: yes;
- direct security vulnerability: not established.

This should block a claim that the MCP surface is Level 5 Deployable. A
cold-start tool must not silently query the wrong evidence domain.

### 7. Source, documentation, and public release truth require reconciliation

The current source and generated documentation identify version 2.2.6, while
the reviewed public GitLab release API did not expose a downloadable v2.2.6
release and the latest verifiable release was
[`v2.2.5`](https://gitlab.com/api/v4/projects/janipaijanen%2Fagents-govern-framework/releases/v2.2.5).
The MCP implementation also post-dates the reviewed 2.2.6 release-preparation
history and was not clearly represented in the Unreleased changelog section.

Consequently, MCP should be described as **Level 4 Validated in the checkout**
until a published release artifact demonstrably contains it. Source:
`agf/VERSION`, `README.md`, changelog, Git history, and public GitLab release
API.

### 8. Implementation concentration and conformance gaps remain

The CLI entry source is approximately 4,200 lines, some apparent module
boundaries remain thin re-exports, and CLI/MCP argument contracts are
duplicated. The MCP adapter shells out to the CLI, making cwd, path meaning,
binary selection, version provenance, and error translation part of the
contract. Source: `tooling/rust/crates/agov/src/main.rs`,
`tooling/rust/crates/agov-coord/`, and
`tooling/rust/crates/agov-mcp/src/main.rs`.

The future conformance suite remains incomplete even though path handling,
structured output, ordering, and lifecycle behavior are important compatibility
targets. Source: `agf/docs/site/reference/tooling-conformance.md`.

### 9. Native platforms can absorb adjacent capabilities

GitHub already combines repository instructions, protected pull-request flows,
hooks, and restrictions on agent self-approval. Sources:
[GitHub custom instructions](https://docs.github.com/en/copilot/reference/custom-instructions-support),
[GitHub agent hooks](https://docs.github.com/en/copilot/concepts/agents/hooks),
and [GitHub cloud-agent controls](https://docs.github.com/en/enterprise-cloud%40latest/copilot/concepts/agents/cloud-agent/risks-and-mitigations).

GitLab provides merge approvals, code-owner controls, and execution-boundary
agent tool governance. Sources:
[GitLab merge-request approvals](https://docs.gitlab.com/user/project/merge_requests/approvals/)
and [GitLab agent tool governance](https://docs.gitlab.com/user/duo_agent_platform/agents/tool-governance/).

OpenAI Codex also combines sandboxing, approvals, network policy, and
agent-native operational controls. Source:
[Running Codex safely](https://openai.com/index/running-codex-safely/).

AGF’s defensible advantage is vendor neutrality and durable repository-local
intent evidence, not simply instructions plus human review.

### 10. The commercial moat is currently limited

The framework mechanics are reproducible, and the open licensing encourages
forking. A durable moat would need to come from trusted conformance tooling,
recognized protocol status, external outcome data, domain rule packs,
integration breadth, and a reputation for accurate maturity claims.

Near-term monetization is more credible through service-assisted adoption,
enterprise support, control-mapping packs, integrations, evidence export,
training, and commercial licensing than through an immediate heavy SaaS build.
This is an inference, not an empirically validated commercial result.

## Improvement proposals

### Proposal 1: Make the MCP path contract safe

**What to change:** Rename the public `dir` field to `coordination_dir`, or
retain `dir` only as a deprecated alias. Describe it as the coordination-message
directory, resolve omission to `docs/coordination`, return resolved provenance,
and fail or warn when a supplied path resembles the repository root.

**Why:** A reasonable caller error currently returns plausible but incomplete
success, which is unacceptable for session orientation.

**Files/sections:** `tooling/rust/crates/agov-mcp/src/main.rs`, MCP tests,
`agf/docs/site/docs/adopt/mcp-server.md`, and the Unreleased changelog section.

### Proposal 2: Add CLI/MCP lifecycle and misuse conformance fixtures

**What to change:** Cover omitted, correct, repository-root, absolute,
nonexistent, and alternate-layout paths. Add an MCP-only claim/handoff lifecycle
and compare its semantic state directly with CLI output.

**Why:** Current forwarding tests do not establish safe real-world behavior.

**Files/sections:** MCP integration tests and the conformance targets described
in `agf/docs/site/reference/tooling-conformance.md`.

### Proposal 3: Enforce release truth mechanically

**What to change:** Add a release invariant checking that `agf/VERSION`,
README claims, generated docs, changelog entries, archive contents, OCI
references, and public release metadata describe the same artifact surface.

**Why:** A governance product must be exceptionally precise about what is
available through the supported operator path.

**Files/sections:** release validators under `tooling/rust/`, `README.md`,
release documentation, changelog, and release pipeline configuration.

### Proposal 4: Version historical evidence contracts

**What to change:** Teach validation and session summaries to classify records
as current-valid, historical-valid, legacy/unversioned, or invalid.

**Why:** Immutable records should remain interpretable without being mistaken
for current-contract failures.

**Files/sections:** `agf/schemas/`, `tooling/rust/crates/agov-coord/`, session
resume output, and migration documentation.

### Proposal 5: Make a minimal starter profile the primary entry product

**What to change:** Provide three roles, two gates, four message kinds, one
validator, and one CI example as the default first-adoption profile. Keep the
full role/rule/learning system as optional packs.

**Why:** The current complete framework risks overwhelming adopters before they
experience its core value.

**Files/sections:** `agf/docs/site/docs/start/`, adopter templates, install
guidance, and profile documentation.

### Proposal 6: Run external pilots before expanding framework breadth

**What to change:** Run at least five independent pilots and collect at least 50
governed changes across maintainers, small teams, platform/DevEx, and an
assurance-sensitive team.

**Why:** The next uncertainty is outcome and adoption evidence, not whether
additional roles or schemas can be implemented.

**Measures:** install time, governor minutes, plans materially revised at Gate
1, drift or defects caught at Gate 2, decision-reconstruction time, records per
change, fast-track rate, abandonment time, and continued use without founder
assistance.

**Files/sections:** evidence strategy in
`agf/docs/site/docs/concepts/evidence-and-learnings.md`; pilot results should be
published as independently attributable evidence rather than silently folded
into framework claims.

### Proposal 7: Establish explicit product-validation thresholds

**What to change:** Before enterprise or scaled-product claims, target ten
independent repositories, 250 governed changes, three paying organizations or
engagements, at least 60% pilot continuation, two independent case studies, and
one credible security-sensitive adopter.

**Why:** These thresholds would distinguish a strong open method from a
repeatable commercial product.

**Files/sections:** product roadmap or a dedicated external-validation plan;
do not encode these commercial hypotheses as normative framework rules.

### Proposal 8: Keep positioning narrow and comparative claims qualified

**What to change:** Prefer “cross-vendor, Git-native assurance and decision
evidence for AI-authored software changes.” Avoid “complete enterprise agent
governance.”

**Why:** GitHub, GitLab, agent providers, OPA, in-toto, and broader frameworks
such as NIST AI RMF occupy adjacent control layers. AGF complements rather than
replaces them. Sources: the platform documentation cited above,
[OPA documentation](https://www.openpolicyagent.org/docs),
[in-toto](https://in-toto.io/), and
[NIST AI RMF](https://www.nist.gov/itl/ai-risk-management-framework).

**Files/sections:** `README.md`, `agf/docs/site/index.md`, positioning and use
case pages, and external product copy.

## Uncertainty

- No independent adopter deployment was performed.
- The Rust tests were inventoried but not rerun as part of the Sol High review.
- No security penetration test, privacy impact assessment, or legal/compliance
  mapping was conducted.
- Historical coordination and plan failures were sampled and summarized, not
  individually adjudicated against every previous schema version.
- Public release availability was checked on 2026-07-20 and may change after
  this review.
- Market willingness to pay, buyer identity, continuation rates, and proposed
  validation thresholds are informed hypotheses rather than measured outcomes.
- The review establishes that correct MCP path input restores semantic parity;
  it does not establish parity across every command, platform, packaged binary,
  or alternate repository layout.
- The framework’s strongest claimed benefit—better decisions and safer
  AI-assisted changes—remains unverified outside controlled external pilots.

## Adoption and investment conclusion

Adopt AGF as a bounded pilot when a repository contains consequential
AI-authored changes, existing Git/CI protections are present, a human will
perform genuine review, and reconstructibility is valuable. Do not yet make it
the sole enterprise control of record.

Fund the next phase as contract hardening and external validation, not broad
feature expansion. AGF is promising, technically credible, and differentiated
enough to test. Its next unit of value should come from external proof and
interface/release reliability rather than additional framework breadth.

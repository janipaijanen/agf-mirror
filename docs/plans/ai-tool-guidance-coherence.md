---
id: ai-tool-guidance-coherence
status: pending-gate1
topic: ai-tool-guidance-coherence
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: "review is being requested with this pending Gate 1 plan; the plan must be revised to consulted with the resulting disposition before governor approval"
  - role: infosec-sentinel
    status: not-applicable
    reason: "the active CI-only sentinel mechanically checks configured credential and cross-record leakage in learnings; this plan changes no learning record and has no applicable plan-stage sentinel check. Security-boundary wording remains an explicit contribution/fact-check and governor review item"
  - role: build-engineer
    status: consulted
    reason: "Claude's reconciled build/tooling assessment 20260726T121131Z-build-engineer-njsk supplies the current agov/agov-mcp parity, build-freshness, telemetry, and visibility-scanner findings"
  - role: fact-checker
    status: waived
    reason: "not yet activated for the pending plan; exact command availability, MCP/CLI parity wording, role authority, loader behavior, and phase-specific claims require fact-check before implementation Gate 2"
---

# Plan: coherent AI lifecycle and agov transport guidance

## Intent

Make every executable AI instruction and every human-facing explanation teach
the same operating contract: what an AI is expected to do at each governed
phase, which actions remain human authority, and when to use structured
`agov-mcp` tools versus the Rust `agov` CLI.

The governing design is one contract with several audience projections, not
several independently authored workflows:

1. `AGENTS.md` is the foundry's executable AI contract.
2. `agf/templates/AGENTS.md` is the adopter-facing executable template.
3. a minimal Claude loader points to the shared contract instead of copying a
   second rule system;
4. public documentation explains the same contract to humans supervising AI;
   and
5. reference pages hold command detail without redefining lifecycle or
   authority.

The contract is mechanically reinforced where deterministic evidence exists.
Documentation alone is insufficient for role-authority freshness.

## Trigger and evidence

The governor requested a coherence review after Claude assessed build and tool
usage. The review found real contradictions rather than a missing paragraph:

- current `AGENTS.md` prefers MCP tools when configured, but the public
  real-situations page tells users to request Rust `agov` commands only;
- the adopter `AGENTS.md` template omits the current coordination write and
  handoff helpers, has no MCP selection rule, requires an unnecessary status
  note after every resume, and conflicts with the loader/overlay ordering;
- the MCP page maps tool names but does not say which transport an AI uses at
  cold start, Gate 1, implementation, polling, Gate 2, and completion;
- `.claude/rules/README.md` still instructs adopters to copy a rule tree even
  though current loader guidance says to keep canonical rules under `agf/` and
  use a small tool-specific pointer;
- current `main` has no `.claude/CLAUDE.md` loader even though the public loader
  contract and validator name that path for Claude Code; and
- guidance alternates between `handoff` and `decision-record` for a governor's
  binding Gate 1 approval instead of consistently distinguishing a request
  from the decision.

Claude's reconciled assessment also establishes these constraints:

- MCP exposure is assistant-surface-dependent;
- `agov-mcp` is a strict transport adapter over existing CLI capability, not a
  new authority or enforcement boundary;
- several CLI operations have no MCP surface;
- local foundry development currently rebuilds `agov` but can leave
  `agov-mcp` stale; and
- `coord visibility` can fail on legacy sequence collisions in long-lived
  multi-branch checkouts, so guidance needs an honest fallback until the tool
  is hardened separately.

An independent read-only inventory confirmed the same lifecycle, transport,
template, loader, and audience gaps. The incident in which Codex incorrectly
called `build-engineer` suspended supplies an additional cold-start lesson:
role claims must be checked against current fetched authority, not a stale
checkout or session summary. The immutable incorrect record will be corrected
by a new referenced coordination note, not edited.

## Canonical behavioral contract

Implementation must express the following invariants consistently.

### Transport invariant

- Both transports execute the supported Rust tooling; MCP is not a different
  governance implementation.
- When the current assistant surface exposes an equivalent `agov-mcp` tool,
  prefer its structured result for that operation.
- Use the CLI when MCP is unavailable, when a command has no MCP tool, when
  diagnosing transport behavior, or when exact release/distribution commands
  require CLI-only coverage.
- Never infer that MCP has authority to approve gates, authenticate actor
  identity, enforce routing, or expand permissions. It inherits the underlying
  command's result and the project's named rule authority.
- Make no token-efficiency superiority claim. Existing telemetry is asymmetric
  and the observed `coord-list` outlier is not a controlled comparison.

### Phase contract

| Phase | AI action | Preferred tooling when exposed | Required CLI/fallback | Human authority |
|---|---|---|---|---|
| Loader and local setup | Load the canonical project contract, read the subordinate local overlay, discover available transports | MCP discovery is surface-specific | Verify repository paths and local binaries | Human configures assistant access and credentials |
| Cold start | Fetch shared refs, run unscoped then role-specific resume against the configured authority ref, and cite the reported authority SHA before making role-status claims | `agov_session_resume`, `agov_handoff_next` when the MCP contract exposes the new authority arguments | `git fetch --prune origin`; `agov session resume --authority-ref origin/main`; `agov handoff next` | Human activates roles and resolves ambiguous authority |
| Intake and Gate 1 | Size work, draft/validate plan, publish a handoff requesting a decision | `agov_plan_validate`, coordination/handoff tools | `agov plan validate`, `agov coord brief/post/validate` | Governor records binding approval or block as `decision-record` |
| Implementation | Work only inside approved scope; test and preserve evidence | MCP validators where useful | normal language/build/test tools plus CLI-only validators | No new human decision unless scope or risk changes |
| Poll and coordination | Read incremental coordination plus all shared MR/issue/CI surfaces; advance authorized work | `agov_coord_list`, handoff tools | `coord status/stale/visibility/graph-index`, GitLab/API; documented fallback if visibility fails | Polling never enlarges authority |
| Gate 2 | Self-review exact diff, validate artifacts, publish evidence handoff | MCP validators and coordination tools | CLI validators, project tests, exact-SHA materialization where needed | Governor reviews exact head and decides merge |
| Release and completion | Run only authorized release steps, verify artifacts, close the causal thread | MCP only where an exact parity tool exists | CLI-only `dist`, version/provenance, usage and release verification commands | Human owns release/tag/deploy and risk acceptance |

The final wording may compress this table by audience, but it must not alter
its behavior or authority boundaries.

### Authority-freshness enforcement

Add a small deterministic tooling slice so the strengthened requirement is not
merely prose:

- `agov session resume` accepts an explicit authority ref, reads role status
  from that ref, and reports the ref plus exact commit SHA in text and
  structured output;
- the equivalent MCP call accepts and returns the same fields, preserving the
  CLI-first parity invariant;
- resume warns or fails with a clear diagnostic when the working-tree manifest
  disagrees with the selected authority ref;
- coordination write/handoff helpers reject a sender role that is not active at
  the explicitly selected authority ref when the governing project requires
  active-role enforcement;
- CI receives an exact target/ref input and validates changed coordination
  sender roles against that immutable commit, rather than mutable checkout
  assumptions; and
- role-status claims in governed handoffs/reviews cite the authority ref and
  SHA as evidence.

The tool must not claim it proved that a remote-tracking ref is fresh. Only a
successful external fetch can update `origin/main`; the documented enforced
sequence therefore starts with `git fetch --prune origin`. Offline operation
must be possible only with an explicit stale/offline advisory that names the
last available authority SHA. Natural-language claims outside structured
fields remain human/fact-check review territory because semantic prose cannot
be reliably validated mechanically.

## Scope

### Foundry executable guidance

- Update root `AGENTS.md` to:
  - build both `agov` and `agov-mcp` for local framework/tooling development;
  - explain that rebuilding does not replace an already-running MCP process;
  - define the phase/transport selection rule without duplicating the full MCP
    manual;
  - require `git fetch --prune origin` followed by authority-ref resume before
    factual role-status claims, and require the reported authority SHA in
    governed evidence;
  - document a safe `coord visibility` failure fallback rather than pretending
    the scan always succeeds; and
  - keep the current two-step resume, gate, polling, CI-cost, and Rust-only
    boundaries intact.
- Add `.claude/CLAUDE.md` as a minimal loader pointing to the shared project
  contract and carrying the governed AGF version marker expected by loader
  validation. It must not restate the rule library.
- Replace `.claude/rules/README.md`'s obsolete copy-the-rules adoption advice
  with a foundry-local explanation of canonical `agf/rules`, legacy/bootstrap
  status of any retained copies, and the loader-only target. Do not mass-delete
  `.claude/rules/**` in this workstream; inventory any drift and route removal
  separately if it changes active Claude behavior.

### Adopter executable template

- Update `agf/templates/AGENTS.md` to:
  - use the loader → optional overlay → transport discovery → fetch →
    authority-ref two-step resume → targeted-read order;
  - make MCP preference conditional on surface exposure and keep CLI as the
    complete fallback/parity reference;
  - include current `coord post/list` and `handoff take/complete` helpers while
    retaining `coord brief` for skeleton/ID generation;
  - remove the unconditional post-resume status-note requirement;
  - distinguish Gate handoffs from governor decision records; and
  - describe visibility failure honestly without weakening the obligation to
    scan shared branches.

### Human-facing explanation

Update the smallest coherent set of public pages:

- `agf/docs/site/docs/adopt/mcp-server.md`: add the phase/transport matrix,
  explain human configuration versus AI execution, CLI-only gaps, process
  restart/freshness, authority-ref parameter parity, and non-authority.
- `agf/docs/site/docs/adopt/assistant-loader-configuration.md`: align the
  loader order and Claude pointer with the executable templates.
- `agf/docs/site/docs/adopt/tooling-and-tokens.md`: distinguish human setup
  from AI operation, add the paired local build where relevant, and avoid
  unsupported MCP efficiency claims.
- `agf/docs/site/docs/operate/day-to-day.md` and
  `operate/run-agf-in-real-situations.md`: explain what AI does in each phase,
  replace CLI-only prompts with surface-conditional wording, teach fetch plus
  authority-SHA resume before role claims, and preserve human gate authority.
- `agf/docs/site/docs/concepts/coordination-protocol.md` and
  `start/first-governed-change.md`: normalize request/handoff versus binding
  `decision-record`, and show equivalent MCP/CLI paths without doubling the
  lifecycle narrative.
- `docs/reference/session-resume.md` and
  `docs/reference/agent-cold-start-habits.md`: align foundry-specific cold
  start, authority-ref semantics and offline limitations, transport selection,
  and visibility fallback.

### Tooling and validator implementation

- Extend the shared session-resume argument/result contract in the Rust core,
  CLI, and MCP adapter with authority ref and authority SHA.
- Extend coordination write/handoff validation with opt-in project policy for
  active sender roles at an immutable authority ref; preserve compatibility for
  adopters that have not enabled this policy.
- Add an exact-ref CI invocation for foundry so its own coordination changes are
  checked against the merge-request target authority.
- Update command help, structured response fixtures, MCP parity tests,
  environment/configuration reference if a config field is introduced, and
  distribution self-tests affected by the new arguments.
- Choose final flag/config names during implementation only after checking the
  existing Clap/config conventions; `--authority-ref origin/main` in this plan
  expresses the required behavior, not permission to create an inconsistent
  one-off interface.

If implementation proves that one listed page needs only a cross-link, prefer
the cross-link over repeated normative prose.

## Explicit exclusions

- No fix for the MCP `coordination_dir` contract or `coord visibility` legacy
  collision; route those as separate implementation work.
- No authentication, cryptographic actor attestation, remote-fetch daemon,
  orchestration permission enforcement, or role-authority redesign. The new
  check validates declared sender status against a named Git ref; it does not
  prove who invoked the tool.
- No change to `agf/agents/MANIFEST.yaml` role status.
- No mass deletion of `.claude/rules/**` and no bootstrap-tree product edits
  under `agents-govern/`.
- No version bump, changelog, release branch, tag, promotion, or GitHub mirror.
- No claim that one transport is universally faster or cheaper.

## Downstream impact

Classification: **PREVENTIVE / ADDITIVE**. The work makes an implicit
surface-conditional tool policy and loader hierarchy explicit. It narrows
misleading instructions, adds opt-in deterministic authority-ref validation,
but does not remove the CLI path or require MCP.

Impact:

- **Foundry:** local tooling developers build both Rust binaries; Claude gains
  a minimal canonical loader; agents must fetch and use an immutable authority
  ref before role claims; structured sender checks are enabled in CI.
- **Direct adopters:** the shipped template and public docs explain one
  phase-specific MCP/CLI contract. Projects without MCP continue using CLI.
- **Downstream:** assistant behavior becomes more predictable; optional config
  may select the authority ref, but governance authority itself does not change.
- **Migration:** adopters may replace copied assistant rules with a pointer
  loader and opt into authority-ref sender validation at their next AGF
  upgrade; existing CLI-only and non-enforcing setups remain valid.
- **Evidence:** `20260726T121131Z-build-engineer-njsk`,
  `20260726T120156Z-planner-architect-ahmj`, and the exact file inventory in
  this plan.

Adopter docs parity: updated in the same implementation.

## Implementation sequence

1. Rebase from current `main`; establish current `agf/VERSION`, role manifest,
   CLI command tree, MCP registry, and loader-validator behavior.
2. Specify the shared authority-ref input/result and opt-in enforcement policy;
   obtain infosec and contribution-auditor review of the boundary before coding.
3. Add contract tests or deterministic checks before prose edits:
   - CLI and MCP resume return the same authority ref and exact SHA;
   - working-tree/ref disagreement and missing/offline refs produce bounded,
     non-authentication diagnostics;
   - active sender passes and inactive/unknown sender fails only when the
     project policy enables exact-ref enforcement;
   - loader validation recognizes the minimal Claude pointer and version;
   - every documented MCP tool maps to a real CLI operation;
   - executable templates contain the required lifecycle/authority anchors;
   - no public prompt says CLI-only where equivalent MCP is supported.
4. Implement the shared Rust core, CLI, MCP, config/CI, and help/fixture changes
   while preserving transport parity and backward compatibility.
5. Update foundry `AGENTS.md`, the minimal Claude loader, and the adopter
   template as the canonical executable surfaces.
6. Project the same contract into the scoped human-facing pages, using
   cross-links to avoid drift.
7. Run assistant-loader and assistant-drift validators, plan/coord validators,
   Rust tests covering touched validators, strict docs build, link checks, and
   focused searches for superseded wording.
8. Fact-check the exact implementation SHA against `agov --help`, the MCP tool
   registry, current manifest, loader code, and built binaries.
9. Contribution-auditor checks audience separation, behavioral consistency,
   downstream impact, and whether the change accidentally makes MCP mandatory.
10. Coder-tester publishes an exact-SHA Gate 2 handoff; governor-reviewer alone
   approves or blocks merge.

## Acceptance criteria

- An AI and a supervising human can answer the same seven phase questions from
  their respective docs and get the same actions and authority boundaries.
- MCP is preferred only when exposed and equivalent; CLI remains complete and
  valid everywhere.
- Every named MCP tool maps to an implemented CLI command, while CLI-only
  commands are clearly named.
- Cold-start ordering is consistent across root instructions, shipped template,
  loader guidance, session reference, and scenario prompts.
- After a fetch, CLI and MCP resume report the same selected authority ref and
  immutable SHA; foundry CI rejects coordination authored under inactive or
  unknown sender roles at its exact target authority SHA.
- Offline or stale-ref operation is explicitly disclosed and never represented
  as current remote authority.
- Gate requests use handoffs; binding governor approvals use decision records.
- The foundry build loop keeps both local Rust binaries fresh and warns that an
  active MCP process must reconnect.
- Current role status comes from fetched project authority, preventing a repeat
  of the stale `build-engineer` claim.
- Human pages explicitly separate what the human configures/decides from what
  the AI executes.
- No page represents MCP, CLI, schema validation, or CI as granting gate
  authority or authenticated identity.
- All scoped validation, strict-docs, and exact-SHA fact-check evidence passes.

## Gate requests

- Contribution-auditor reviews this plan before Gate 1.
- Governor-reviewer approves or blocks the plan at Gate 1.
- Implementation begins only after recorded Gate 1 approval.
- Exact implementation requires fact-check, contribution audit, and governor
  Gate 2 before merge.

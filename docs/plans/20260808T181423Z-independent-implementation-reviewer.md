---
id: 20260808T181423Z-independent-implementation-reviewer
status: approved
topic: independent-implementation-reviewer-role
gate1_approved_by: 20260808T193155Z-governor-reviewer-get2
gate1_approved_at: 2026-08-08T19:31:55Z
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      The independent staffing assessment in
      `20260808T171050Z-contribution-auditor-q1ge.md` recommended a distinct
      role, preserved contribution-auditor's current jurisdiction, and set
      the core exact-head, read-only, provenance, evidence, and no-Gate-
      authority boundaries used here. This draft must receive a renewed
      exact-head contribution-auditor review before it is handed to the
      governor for Gate 1, because the plan turns that recommendation into
      a reusable role contract.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The planned role is read-only and advisory. It introduces no secret
      handling, credentials, network calls, authorization mechanism,
      learning-record content, executable trust boundary, schema, or
      blocking automation. Its security-review obligation is limited to
      reporting any credential or sensitive-data exposure already visible
      in an implementation diff to governor-reviewer; it may not remediate
      or disclose such material in coordination. The implementation must
      reroute for infosec consultation if its scope grows beyond this
      documentation/manifest-only contract.
---

# Plan: independent implementation reviewer role

## Decision requested

Authorize a reusable `independent-implementation-reviewer` role and activate
it for foundry as a read-only advisory reviewer of an implementation's exact
Git head against the approved Gate 1 plan, acceptance criteria, tests, and CI
evidence.

This is planning only. It does not approve this plan, activate the role,
authorize implementation, clear Gate 2, merge any merge request, or change MR
!1224. A separate binding governor-reviewer Gate 1 decision is required before
implementation starts. The broader Pages-rework plan remains deferred until
MR !1224 has a final merge or rejection disposition, as required by
`20260808T180605Z-governor-reviewer-ha18.md`.

## Evidence and problem statement

The current roster has no role whose named function is independent technical
review of a governed implementation. `coder-tester` implements and verifies;
`contribution-auditor` screens contributions and governance guidance for
adversarial or unsound patterns; domain reviewers such as `fact-checker`,
`build-engineer`, and `web-ux-reviewer` inspect narrower factual, CI, or visual
dimensions; and `governor-reviewer` alone makes binding gate decisions.

During MR !1224, Claude implemented under `coder-tester` while Codex performed
independent exact-head review. Publishing both functions under the same role
name made independence legible only by correlating the `model:` field and
surrounding records. `20260808T161743Z-planner-architect-kpdj.md` identified
that audit-trail ambiguity. The independent Codex assessment in
`20260808T171050Z-contribution-auditor-q1ge.md` recommended a distinct role,
and the governor authorized this Gate 1 planning in
`20260808T180605Z-governor-reviewer-ha18.md`.

This is direct project-process evidence that the identity gap exists. It is
not empirical proof that independent model review always improves correctness.
The role contract is therefore **Empirically informed**, not empirically
validated. On implementation and validation it can reach **Level 4
Validated** in the repository; it is not **Level 5 Deployable** until a
supported release artifact ships the definition and adopter documentation.

## Role contract

### Purpose and activation

The role reviews a named implementation merge request at one exact commit SHA
and answers one bounded question: does this exact implementation satisfy its
approved plan and declared acceptance evidence without introducing unplanned
behavior or hiding an unresolved failure?

Foundry activates the role when the governor or an approved plan requests an
independent implementation review for a non-trivial Gate 2 candidate. The role
is not a mandatory extra gate for every change. A plan or governor decision
must name it when the additional separation is warranted by risk, repeated
self-review misses, cross-model review needs, or ambiguous implementation
evidence.

### Independence minimum

For one implementation under review, the reviewer must:

- use a session distinct from the implementing session;
- use a model identity/configuration distinct from the implementing model;
- not have authored, edited, or committed the exact implementation being
  reviewed; and
- record reviewer model provenance and the implementer's known provenance in
  the review record.

A different vendor may strengthen separation but is not required by this
initial contract. If provenance cannot be established, the reviewer reports
`INCOMPLETE` rather than claiming independence. Human governor review remains
independent authority and is never replaced by satisfying this minimum.

### Jurisdiction and boundaries

The reusable role definition will own implementation-review reports and read:

- the approved Gate 1 plan and controlling governor decisions;
- the target branch, exact implementation head, and full diff;
- acceptance criteria, applicable rules, tests, fixtures, and documentation;
- exact-head pipeline/job evidence and unresolved MR discussions; and
- earlier review rounds needed to verify resolution without silently narrowing
  coverage.

It cannot touch the implementation branch, source, tests, plans, rules, agent
definitions, CI configuration, merge state, tags, releases, or deployment
state. It may run read-only inspection and local tests against the exact head,
but it reports findings rather than fixing them.

The role is advisory. `CLEAR` is evidence for governor-reviewer; it is not
Gate 2 approval. `BLOCK` is a recommendation with specific evidence; only the
governor-reviewer makes the binding decision. Automation may not treat the
role's output as mechanically blocking unless a separately approved rule names
and grants such authority.

### Review output contract

Each review record must include:

1. merge request and exact 40-character head SHA;
2. target branch and, when material, target SHA observed;
3. reviewer and implementer model/session provenance, or an explicit unknown;
4. scope reviewed and exclusions;
5. coverage status: `COMPLETE` or `INCOMPLETE`;
6. recommendation: `CLEAR` or `BLOCK`;
7. findings, each tied to a plan criterion, governing rule, changed line, test,
   or pipeline/job result;
8. exact commands or CI evidence relied on, with failures and skipped checks
   disclosed; and
9. disposition of earlier blocking findings when this is a later round.

`CLEAR` requires complete declared coverage and zero unresolved blocking
findings. An incomplete review cannot return `CLEAR`. Any new implementation
commit invalidates the prior recommendation for Gate 2: the changed head needs
a fresh review or an explicit governor waiver under existing authority. A
target-branch advance is reported when it materially changes the comparison or
mergeability evidence.

Review discipline is exhaustive per round: inspect the full governed scope and
report all findings found, rather than deliberately drip-feeding one issue per
round. Later discoveries remain possible and must not be suppressed.

### Separation from neighboring roles

| Role | Question answered | Authority boundary |
|---|---|---|
| `planner-architect` | What should be built and how is acceptance defined? | Authors plans; does not review its own implementation. |
| `coder-tester` | Has the approved change been implemented and tested? | May edit implementation; cannot approve Gate 2. |
| `independent-implementation-reviewer` | Does this exact head satisfy the approved plan and evidence contract? | Read-only, advisory, no gate or merge authority. |
| `contribution-auditor` | Is the contribution or governance guidance adversarial, misleading, unsound, or boundary-eroding? | Preserves its existing contribution-integrity charter. |
| domain advisory roles | Are factual, build, security, visual, or academic dimensions sound? | Review only their named domain; do not substitute for full exact-head conformance. |
| `governor-reviewer` | Should Gate 1/Gate 2 be approved? | Sole binding gate and merge authority. |

The new role may cite domain-review findings but cannot impersonate those
roles or claim their consultation occurred. Likewise, a domain review does not
automatically satisfy full implementation review.

## Planned implementation scope

Implementation after Gate 1 is limited to role and documentation assets:

| Artifact | Planned change |
|---|---|
| `agf/agents/independent-implementation-reviewer.yaml` | Add the reusable role definition, independence minimum, review dimensions, required report structure, escalation triggers, communications, and forbidden actions. |
| `agf/agents/MANIFEST.yaml` | Activate the role for foundry with explicit Codex/different-session-and-model staffing and local boundaries. |
| `agf/agents/MANIFEST.example.yaml` | Add a neutral skipped entry so adopters can deliberately activate or omit the optional role. |
| `AGENTS.md` | Add the active role to the current foundry roster and retain the statement that binding gates stay with governor-reviewer. |
| `agf/docs/reference/quick-reference.md` | Add the role's concise phase/purpose and optional-staffing guidance. |
| `agf/docs/site/docs/concepts/roles-and-gates.md` | Explain the role's optional exact-head review position without presenting it as a new binding gate. |
| `agf/docs/site/docs/adopt/install-and-layouts.md` and `agf/ADOPTING.md` | Add the role only to an optional independent-review commissioning shape; do not make it part of the minimum role set. |
| `docs/reviews/rule-projection/20260808T181423Z-independent-implementation-reviewer.yaml` | Record the six-surface documentation-rule projection for normative Pages guidance. |
| `agf/CHANGELOG.md` | Record the new reusable role and its advisory boundary under the unreleased section. |

The implementer must inventory exact current references before editing. A
reference that merely enumerates every shipped role must be updated; unrelated
historical documents and bootstrap `agents-govern/` files must not be changed.

No Rust code, schema, new coordination kind, required field, CI job, blocking
validator, version bump, release, or Pages visual redesign is planned. `agov`
already treats manifest role keys generically; implementation must first verify
that assumption. A discovered tooling/schema limitation is a plan deviation
and returns to the governor rather than being folded into this role slice.

## Implementation sequence after Gate 1

1. Fetch `main`; verify the authorizing and approving coordination records and
   create a fresh feature branch/worktree.
2. Re-inventory role catalog, manifest, communication edges, adopter docs, and
   validation behavior at the implementation base.
3. Add the role definition with the contract above; then update foundry and
   adopter manifests.
4. Update current roster, quick reference, role/gate concept page, adoption
   matrices, and changelog without changing the three-role minimum.
5. Complete the documentation-rule projection matrix and reconcile every
   normative statement across the six surfaces.
6. Run validation and negative boundary review. Request contribution-auditor
   review because the change adds a role and potential jurisdiction overlap.
7. Publish the exact-head Gate 2 handoff. A reviewer satisfying the new role's
   independence contract may review the implementation, but its advisory
   result does not replace the governor's Gate 2 decision.

## Required validation

- `agov validate agents`
- `agov validate`
- `agov session resume --role independent-implementation-reviewer --base-branch main`
- `agov handoff next independent-implementation-reviewer`
- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml`
- `agov validate adopter-docs`
- `agov plan projection-check --base-ref main docs/reviews/rule-projection/20260808T181423Z-independent-implementation-reviewer.yaml`
- `git diff --check`
- Manual jurisdiction-overlap matrix against `coder-tester`,
  `contribution-auditor`, `governor-reviewer`, `fact-checker`,
  `infosec-sentinel`, `build-engineer`, and `web-ux-reviewer`.
- Manual negative review proving the new role has no edit, approval, merge,
  release, deployment, or mechanically blocking authority.

If the role addition changes Rust behavior or a Rust test fixture after all,
run `cargo test --manifest-path tooling/rust/Cargo.toml` and treat the change as
a plan deviation requiring governor disposition before continuing.

## Acceptance criteria

1. The reusable role definition and both manifests validate, and the foundry
   manifest identifies a distinct active review role rather than another
   `coder-tester` identity.
2. The role is read-only and advisory: it cannot edit reviewed artifacts,
   approve either gate, merge, release, deploy, or become mechanically
   blocking by its own declaration.
3. The independence minimum requires a distinct session and model from the
   implementer and records provenance; missing provenance yields
   `INCOMPLETE`.
4. Every review binds to an exact SHA, declares scope and coverage, reports
   findings and test/CI evidence, and cannot return `CLEAR` while incomplete
   or while a blocking finding remains unresolved.
5. A new implementation head invalidates the prior review recommendation
   unless the governor records an explicit existing-authority waiver.
6. `contribution-auditor` retains its current contribution-integrity charter;
   the new role does not absorb or silently amend it.
7. Domain reviewers remain independently invocable and are not impersonated
   or treated as automatically satisfied by implementation review.
8. Foundry and adopter documentation describe the role as optional
   independent-review staffing, not a fourth mandatory gate or an authority
   peer to governor-reviewer.
9. Existing minimum commissioning shapes remain valid and unchanged.
10. The Pages projection matrix is complete, strict docs build passes, and
    role guidance is consistent across raw reference, adopter, concept, and
    current-project surfaces.
11. No Rust, schema, CI, release, version, MR !1224 implementation, or broader
    Pages-rework change appears in the implementation diff.

## Risks and mitigations

- **Ceremony inflation:** optional activation and an explicit trigger prevent
  the role from becoming an unconditional fourth gate.
- **Shadow gate authority:** repeated statements in the role, manifests, and
  docs keep `CLEAR`/`BLOCK` advisory and reserve decisions to the governor.
- **Reviewer becomes a fixer:** `cannot_touch` and forbidden actions prohibit
  implementation edits; findings route back to coder-tester/governor.
- **Nominal rather than real independence:** distinct-session-and-model
  provenance is required, and unknown provenance forces incomplete coverage.
- **Jurisdiction overlap:** the separation matrix and contribution-auditor
  review test the new definition against every adjacent active role.
- **Stale exact-head approval:** any head change invalidates the recommendation.
- **Sensitive material copied into coordination:** the role reports the class
  and location safely and escalates to infosec/governor; it does not reproduce
  secrets in its review record.
- **Tool output treated as authority:** the role reports tool/CI evidence under
  `tool-authority-boundary-rule.md`; tools remain advisory unless a named rule
  explicitly grants blocking authority.

## Rollback and deactivation

Before release, rollback is a governed revert of the role definition,
manifest entries, current-roster/docs projections, and changelog entry. After
release, adopters may omit or mark the role skipped; foundry may deactivate it
through a governor-approved manifest change. Historical review and decision
records remain immutable evidence and are not deleted. Removing the reusable
definition from a later release requires a separately governed compatibility
decision; this plan does not authorize it.

## Non-goals

- No Gate 1 approval, role implementation, activation, or Gate 2 decision in
  this planning change.
- No review or modification of MR !1224's implementation branch.
- No broader Pages-rework plan or implementation before !1224's final
  disposition.
- No expansion of `contribution-auditor`.
- No automatic model selection, agent spawning, review assignment, or review
  enforcement.
- No claim that cross-model review guarantees correctness or constitutes
  independent empirical corroboration.
- No CI, schema, coordination-protocol, release, version, tag, or deployment
  change.

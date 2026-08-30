---
id: gate-change-mechanical-enforcement
status: pending-gate1
topic: gate-change-mechanical-enforcement
advisory_consultation:
  - role: coder-tester
    status: waived
    reason: >-
      Self-authored by coder-tester. A bounded independent review by the
      coder-tester session on the Codex host is requested before binding
      Gate 1. That practice has found material defects in every round of this
      work: three consecutive BLOCK reviews on the parent plan, each against
      this slice's enforcement semantics specifically.
  - role: contribution-auditor
    status: waived
    reason: >-
      No separate contribution-auditor pass is proposed; the bounded
      independent review requested above is offered as the substitute
      independent check, on the basis the governor accepted through the
      v2.7.0/v2.8.0 cycle. The governor ratifies or rejects that substitution
      at Gate 1.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Consulted and material, and re-consulted after review 8mar. This plan
      proposes a check with blocking authority over gate changes, so its own
      failure modes are security-relevant: a check that can be silently
      disabled, or that claims more assurance than it delivers, is worse than
      none. The design states its proxy boundary explicitly rather than
      implying tamper-resistance it does not have.

      An earlier revision asserted here that no credential or access-control
      content was involved while its assertion 3 required an authenticated
      request to a private project's GitLab API. Finding GME-1 caught that
      contradiction. Assertion 3 is withdrawn, and the check is now purely
      local -- it reads the registry and the working tree, performs no network
      request, and needs no token. The claim is true as written now rather
      than aspirational, and acceptance criterion 5 verifies it by running the
      tests with no credential available.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Mechanical enforcement of gate-change discipline

## Decision requested

Approve the design below as Gate 1 scope. This slice was previously part of
`docs/plans/20260829T211252Z-gate-change-discipline.md` and is split out by
governor decision after three consecutive independent-review BLOCKs, all
against this slice's enforcement semantics while that plan's rule and
validator-fix slices were repeatedly cleared.

## Why this exists, and what is already shipped without it

The parent plan makes gate mechanics ineligible for the fast-track waiver and
extends `gate-self-test-rule.md` to cover *modifying* a gate, not only
introducing one. That is rule text. `gate-self-test-rule.md` opens by arguing
that "process discipline alone is at-will and breaks under speed", and this
project has three documented incidents of a gate shipping broken because
nobody verified it.

So the parent plan's discipline is, today, exactly the at-will kind that rule
warns about. **This plan is what makes it mechanical.** Until it lands, a gate
change is governed by rule and human review only — recorded plainly in the
parent plan rather than left implied.

## Prior review findings this design must answer

Three findings travel here from `j1ht`, and they are the design, not
corrections to it.

**GCD-R1 — the boundary must be executable and must not ship foundry state.**
An `agov gate` command family needs dispatch in
`tooling/rust/crates/agov/src/main.rs` and an export from
`tooling/rust/crates/agov-validators/src/lib.rs`; neither was named before.
Separately, `agf/dist/release-manifest.yaml` selects root files and directory
globs and would **not** ship a new root `agf/gates.yaml` merely because the
file exists. More importantly, a live registry naming `.gitlab-ci.yml`,
`.gitlab/scripts/` and `tooling/rust/` paths is foundry-specific state.
Shipping it to adopters would repeat a collision this project already solved
for `agf/agents/MANIFEST.yaml`, whose live copy is excluded while a neutral
example ships.

**GCD-R2 — invocation presence does not protect execution.** The previous
design asserted that checking a named CI job still contains a required
invocation string proves the gate runs. It does not. The job can retain that
string while neutralised by an earlier `exit 0`, a false shell guard, `rules:
when: never`, `allow_failure: true`, or an inherited workflow/default/variable
change — none of which touches the registered script.

**GCD-R3 — retirement and self-check semantics must be coherent.** If every
registered job must exist and every registered path must resolve, but
retirement keeps the entry, the plan must say which requirements are waived
in the retired state. It must also not describe self-registration as
tamper-proofing: the candidate pipeline runs the candidate checker, so one
merge request can weaken the checker and touch its declared test path.

## Design

### Registry: split live state from shipped template

| Artifact | Path | Ships to adopters |
|---|---|---|
| Live foundry registry | `.agf/gates.yaml` (repository-local, outside `agf/`) | **No** |
| Neutral adopter example | `agf/examples/gates.yaml` | Yes |
| Schema | `agf/schemas/gate-registry.schema.json` | Yes |

Putting the live registry outside `agf/` resolves GCD-R1's collision by
construction rather than by a release-manifest exclusion that a future glob
change could silently undo. `agf/dist/release-manifest.yaml` gains the schema
and the example explicitly; the strict projection check verifies both at
Gate 2.

The command reads `.agf/gates.yaml` by default and accepts
`--registry <path>`, so adopters point it at their own file.

### Implementation boundary

| Artifact | Path |
|---|---|
| Checker | `tooling/rust/crates/agov-validators/src/gate_change.rs` |
| Module export | `tooling/rust/crates/agov-validators/src/lib.rs` |
| CLI dispatch | `tooling/rust/crates/agov/src/main.rs` |
| Command | `agov gate change-check --base-ref <ref> [--registry <path>]` |
| Integration tests | `tooling/rust/crates/agov/tests/gate_change_check.rs` |
| Fixtures | `tooling/rust/crates/agov/tests/fixtures/gate-change/` |
| CI integration | existing `agov:governance:validate` job |

### What the check asserts — and what it explicitly does not

**Assertion 1 — test accompaniment.** A diff touching any registered
`surfaces` path must also touch that gate's declared `tests` paths.

**Assertion 2 — registry integrity, in two separable parts.** Review `br41`
(GME-R2) established that stating this as one assertion makes the retirement
waiver incoherent, because the two halves have opposite lifecycle behaviour:

- **2a — path resolution.** Every path registered by an *active* entry must
  resolve.
- **2b — entry permanence.** A registry entry may never be deleted, in any
  state. Retirement is a transition, not a removal.

Splitting them is what lets a retired gate delete its obsolete files (2a
waived) while its ID remains a permanent record (2b never waived).

**Assertion 3 is withdrawn.** The previous revision proposed resolving
GitLab's merged configuration and asserting the named job exists, is not
`when: never`, is not `allow_failure: true`, and still invokes the registered
script. Review `8mar` blocked it on two independent grounds, both correct:

- **GME-1 — no credential contract.** The command signature named only
  `--base-ref` and `--registry`, while the assertion requires an authenticated
  request to a private project's API. CI Lint is not in GitLab's documented
  job-token endpoint list, so `CI_JOB_TOKEN` cannot be assumed and a
  project/access token would be required. The plan's own infosec consultation
  said "no credential... content is involved" while its core check needed
  exactly that — the same species of self-contradiction `j1ht` GCD-R3 caught
  in the parent plan's front matter.
- **GME-2 — it still would not have closed the bypass class.** Merged
  configuration is static. `rules` can omit the job in the relevant context,
  and an earlier `exit 0` leaves the registered invocation unreachable — the
  precise bypass `j1ht` named. Resolving `extends`, anchors and `!reference`
  makes the evidence easier to read; it performs no control-flow or
  pipeline-context proof. GitLab documents `dry_run` separately as pipeline
  *creation* simulation, and even that cannot prove a later script command is
  reachable.

**This is the second design in a row that claimed to prove a gate executes,
and the second to be wrong.** The first matched literal text; the second
resolved merged YAML. Both were structurally better than their predecessor and
both made the same overclaim. The correction is not a third mechanism — it is
to stop making the claim.

**What this plan asserts is therefore assertions 1 and 2 only.** Both are
purely local: they read the registry and the repository working tree. No API,
no network, no credential, no pipeline context. GME-1 dissolves entirely
rather than being mitigated, and the infosec consultation below is corrected
to match what the check actually does.

**CI execution integrity is explicitly human review, and is named as such.**
Whether a registered gate's job is actually reachable in the relevant pipeline
context, and whether its script does useful work, is a reviewer's
responsibility at Gate 2. That is a real residual, stated here rather than
implied by a mechanism that does not deliver it. A future plan may revisit it
if a proof mechanism exists that survives GME-2; none is proposed now, and
"advisory output" is deliberately not offered as a consolation, because
advisory output that looks like a gate is how the last two revisions went
wrong.

### Retirement state machine

An entry is `active` or `retired`. Retirement requires an explicit
`retired: {reason, date}`.

| | `active` | `retired` |
|---|---|---|
| Assertion 1 — test accompaniment on that entry's surfaces | enforced | **waived** |
| Assertion 2a — that entry's paths resolve | enforced | **waived** |
| Assertion 2b — the entry may not be deleted | enforced | **enforced** |

**Waiving 1 and 2a for retired entries answers `8mar` GME-A1**, and splitting
2b out of 2a answers `br41` GME-R2. The revision before last enforced both on
retired entries, so a retired gate could remove its CI job but never its
obsolete script or fixtures without failing the registry — permanent tombstone
files as an unintended lifecycle cost. The revision after that waived
"assertion 2" wholesale while still requiring deletion to fail, which is a
contradiction rather than a lifecycle. With 2a and 2b separated, a retired
gate's surface files can be deleted normally and its entry still cannot.

**What stops retirement being used as an escape hatch.** Retiring an entry is
an edit to the registry, and the registry is a registered surface of the
`gate-change-check` entry itself. So the `active → retired` transition fires
assertion 1 on *that* entry and demands an accompanying test change, visible
in the diff. The entry itself may never be deleted — that fails assertion 2 —
so a retired ID is a permanent record and cannot be silently reused.

### The proxy boundary, stated as a limit

Self-registration improves **visibility**, not tamper-resistance. The
candidate pipeline runs the candidate checker, so one merge request can weaken
the checker and satisfy assertion 1 with any touch to its declared test path,
including an empty or unrelated test.

What the check removes is the *silent* path: weakening a gate now requires
deliberately authoring a test change, visible in the diff a human reviews. It
is a proxy for review attention, not proof of test quality. No text in this
plan, the rule, or adopter documentation may describe it otherwise.

## Rule-projection screen

Structured evidence:
`docs/reviews/rule-projection/gate-change-mechanical-enforcement.yaml`.

| Surface | Disposition |
|---|---|
| Normative owner | **existing pointer sufficient** — `agf/rules/always/gate-self-test-rule.md` already grants this check its blocking authority via the parent plan's slice A; this plan implements it and adds no new obligation |
| Adopter explanation | **updated** — the Pages page describing validators gains the command and the registry example, named in the manifest, and states plainly what the check does not cover |
| Runbook / situation map | **updated** — the operator-facing checklist gains the failure's meaning and the retirement procedure |
| AI landing / loaders | **not applicable** — no new agent obligation beyond slice A's, which is already projected |
| Validation / drift evidence | **implemented** — this plan *is* the mechanical check. Two boundaries are stated as limits, not implied: it is a proxy for review attention rather than proof of test quality, and it makes no claim about CI execution integrity, which is named as Gate 2 reviewer responsibility |
| Distribution | schema and neutral example added to `agf/dist/release-manifest.yaml` explicitly; live registry deliberately outside `agf/` and not shipped |

## Review disposition — `8mar`

Returned **BLOCK** with two blocking findings and one advisory. All three are
upheld; the design is narrowed rather than defended.

| Finding | Disposition |
|---|---|
| **GME-1** no authentication/input contract for the merged-config dependency | **Upheld; the dependency is removed rather than specified.** Naming a token, CI variable, and permission set would have made the check work, at the cost of adding a credential obligation to a governance validator. Withdrawing assertion 3 dissolves the requirement instead. The front-matter infosec claim that contradicted it is corrected. |
| **GME-2** merged YAML does not close the execution bypass class | **Upheld, and it is the decisive one.** `rules` context omission and an earlier `exit 0` both survive merged-configuration checking. Static merged config is not pipeline simulation, and even simulation cannot prove a later script command is reachable. |
| **GME-A1** retired entries permanently retain dead surfaces | **Upheld.** Retirement now waives assertions 1 and 2 for the retired entry, so obsolete files can be deleted. The transition still fires assertion 1 on the registry, and the entry may never be deleted, so retirement is not an escape hatch. |

**The pattern worth naming.** This is the second consecutive design in this
line to claim a mechanical check proves a gate *executes*, and the second to
be wrong. The first matched literal invocation text; the second resolved
merged YAML. Each was structurally better than its predecessor and each made
the same overclaim about what its evidence supports. The correction is not a
third mechanism — it is to stop making the claim and name the residual as
human review.

What survives is smaller and, unlike its predecessors, fully local: two
assertions over the registry and the working tree, no network, no credential,
no forge dependency.

## Review disposition — `br41`

Bounded re-review. **CLEAR** on the parent plan !2214; **BLOCK** here, two
findings, both upheld.

| Finding | Disposition |
|---|---|
| **GME-R1** adopter projection still specified withdrawn assertion 3 | **Upheld.** The manifest's `adopter_facing_explanation` still instructed the page to document a GitLab merged-configuration dependency and a cross-forge gap, both of which belonged to the withdrawn assertion. Rewritten to describe assertions 1 and 2 only, with the removed text kept solely as labelled superseded history. |
| **GME-R2** retirement waived assertion 2 while relying on it | **Upheld.** Assertion 2 conflated path resolution with entry permanence, so "waived when retired" contradicted "deletion still fails". Split into 2a (paths resolve, waived when retired) and 2b (entry may never be deleted, never waived). Acceptance criteria 4 and 6 updated to test them separately. |

**The reframing in GME-R1 is worth keeping, beyond this fix.** `br41` points
out that a projection row is *prospective instruction* for implementation, not
description of it — so a row left stale after a design change does not merely
record history, it actively directs an implementer to build the thing that was
removed. That is a stronger reason to keep manifests current than "tidiness",
and it is the second time in two rounds that a stale manifest row survived a
design change here (`8mar` GDAB-1 was the first). Recorded rather than treated
as two unrelated slips.

## Acceptance criteria

1. `agov gate change-check` exists, is dispatched from `agov/src/main.rs`,
   exported from `agov-validators/src/lib.rs`, and defaults to `.agf/gates.yaml`
   with `--registry` override.
2. Assertion 1 blocks a diff touching a registered surface with no declared
   test-path change, demonstrated.
3. Assertion 1 does **not** block a gate change accompanied by a test change,
   nor a non-gate change to any unregistered path, both demonstrated.
4. Assertion 2a blocks an unresolvable path registered by an active entry,
   and assertion 2b blocks outright deletion of any registry entry regardless
   of state — each demonstrated separately, since `br41` GME-R2 showed that
   stating them as one assertion makes the retirement waiver contradictory.
5. No code path in the checker performs a network request, reads a token, or
   requires pipeline context — verified by inspection, and by the tests
   passing with no credential available.
6. Retirement waives assertions 1 and 2a for the retired entry while 2b stays
   enforced: a retired gate is demonstrated able to delete its obsolete
   surface files, deletion of the retired entry itself is demonstrated
   failing, and the `active → retired` transition is demonstrated firing
   assertion 1 on the registry.
7. The known proxy limit — an unrelated or empty test satisfying assertion 1 —
   is demonstrated as a **passing** case and recorded as a limit, not hidden.
8. `agf/dist/release-manifest.yaml` ships the schema and the neutral example;
   the live `.agf/gates.yaml` is absent from the distribution artifact, both
   verified by inspecting a built archive rather than by reading the manifest.
9. `agov plan projection-check` passes without `--allow-planned` at Gate 2.

## Risks

**Unenforced CI execution integrity.** This is the plan's real residual, and
it is not mitigated by anything mechanical here. A gate's job can be made
unreachable by `rules` context, or its script neutralised by an early
`exit 0`, without touching a registered surface. Assertion 1 covers *changes*
to the registered script; it does not prove the script runs or does useful
work. That is Gate 2 reviewer responsibility, named in the rule rather than
left to be discovered. Two successive designs claimed to close this
mechanically and both were wrong; a third is not attempted on the strength of
wanting it to exist.

**Forge neutrality, now by construction.** Withdrawing assertion 3 removes the
GitLab API dependency entirely, so adopters on any platform get the same check
rather than a documented gap.

**Over-blocking.** Registering `.gitlab/scripts/governance-validate.sh` rather
than `.gitlab-ci.yml` keeps routine CI edits out of scope. If the registry
still proves noisy, entries are narrowed — itself a gate change under the
parent plan's slice A.

**Proxy misread as proof.** Addressed by stating the boundary in the plan, the
manifest, and any adopter text, and by acceptance criterion 7 demonstrating
the limit rather than omitting it.

## Out of scope

The parent plan's slices A and B; the v2.9.0 slices; issues #119 and #120; any
audit of gates beyond those registered by this plan. Findings outside that set
are reported naming the class searched, not folded in.

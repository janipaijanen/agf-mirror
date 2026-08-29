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
      Consulted and material. This plan proposes a check with blocking
      authority over gate changes, so its own failure modes are
      security-relevant: a check that can be silently disabled, or that
      claims more assurance than it delivers, is worse than none. The design
      below states its proxy boundary explicitly rather than implying
      tamper-resistance it does not have. No credential, personal-data, or
      access-control content is involved.
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

**Assertion 2 — registry integrity.** Every registered path must resolve. An
entry may not disappear: retirement is a state transition, not a deletion.

**Assertion 3 — effective configuration, replacing the withdrawn invocation
check.** For each registered gate, the checker resolves the pipeline's
**merged** configuration (GitLab's `ci/lint` with `include_merged_yaml`, or
the project CI-config endpoint) and asserts the named job exists in it, is not
`when: never`, is not `allow_failure: true`, and still invokes the registered
script.

Merged configuration is the right authority because it is what GitLab will
actually run: it resolves `extends`, anchors, `!reference`, hidden jobs, and
top-level `workflow`/`default`/`variables`. That closes the bypass class
GCD-R2 named, which a text check cannot.

**What assertion 3 still does not prove.** It does not prove the *script* does
useful work — an early `exit 0` inside
`.gitlab/scripts/governance-validate.sh` is invisible to it. That is covered
by assertion 1 (the script is a registered surface, so changing it demands a
test change) plus human review, not by assertion 3. Stating this boundary is
the point: the previous revision's failure was claiming coverage it did not
have.

### Retirement state machine

An entry is `active` or `retired`. Retirement requires an explicit
`retired: {reason, date}`.

| Assertion | `active` | `retired` |
|---|---|---|
| 1 — test accompaniment | enforced | enforced (surfaces still registered) |
| 2 — paths resolve | enforced | enforced |
| 3 — effective config | enforced | **waived** — a retired gate has no job |

Deleting an entry outright, rather than retiring it, fails assertion 2. The
`active → retired` transition is itself a registry change and therefore
subject to assertion 1.

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
| Adopter explanation | **updated** — the Pages page describing validators gains the command and the registry example, named in the manifest |
| Runbook / situation map | **updated** — the operator-facing checklist gains the failure's meaning and the retirement procedure |
| AI landing / loaders | **not applicable** — no new agent obligation beyond slice A's, which is already projected |
| Validation / drift evidence | **implemented** — this plan *is* the mechanical check; its proxy boundary is stated above rather than overclaimed |
| Distribution | schema and neutral example added to `agf/dist/release-manifest.yaml` explicitly; live registry deliberately outside `agf/` and not shipped |

## Acceptance criteria

1. `agov gate change-check` exists, is dispatched from `agov/src/main.rs`,
   exported from `agov-validators/src/lib.rs`, and defaults to `.agf/gates.yaml`
   with `--registry` override.
2. Assertion 1 blocks a diff touching a registered surface with no declared
   test-path change, demonstrated.
3. Assertion 1 does **not** block a gate change accompanied by a test change,
   nor a non-gate change to any unregistered path, both demonstrated.
4. Assertion 2 blocks outright deletion of a registry entry and blocks any
   unresolvable registered path, each demonstrated.
5. Assertion 3 blocks, against **merged** configuration, each of: job absent,
   `when: never`, `allow_failure: true`, and registered script no longer
   invoked — each demonstrated separately, and at least one demonstrated via
   `extends` or `!reference` indirection rather than a literal edit.
6. Assertion 3 is waived for `retired` entries and enforced for `active`
   ones; the `active → retired` transition is demonstrated passing and
   entry deletion demonstrated failing.
7. The known proxy limit — an unrelated or empty test satisfying assertion 1 —
   is demonstrated as a **passing** case and recorded as a limit, not hidden.
8. `agf/dist/release-manifest.yaml` ships the schema and the neutral example;
   the live `.agf/gates.yaml` is absent from the distribution artifact, both
   verified by inspecting a built archive rather than by reading the manifest.
9. `agov plan projection-check` passes without `--allow-planned` at Gate 2.

## Risks

**Merged-config dependency.** Assertion 3 needs GitLab's lint/CI-config API.
If unavailable or rate-limited, the check must fail closed with a clear
diagnostic, never skip open. A forge-neutral fallback is not attempted here;
adopters on other platforms get assertions 1 and 2 and a documented gap,
stated rather than papered over.

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

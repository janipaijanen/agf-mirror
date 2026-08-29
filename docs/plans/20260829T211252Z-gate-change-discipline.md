---
id: gate-change-discipline
status: pending-gate1
topic: gate-change-discipline
advisory_consultation:
  - role: coder-tester
    status: waived
    reason: >-
      Self-authored by coder-tester in response to a governor instruction
      given after this session's own defect. A bounded independent review by
      the coder-tester session on the Codex host is requested before binding
      Gate 1, per the project's standing practice that every gate proposal
      receives an independent review. In the v2.7.0-v2.9.0 cycle that
      practice found real defects in five consecutive self-authored
      artifacts, including two authored in this session.
  - role: contribution-auditor
    status: waived
    reason: >-
      No separate contribution-auditor pass is proposed. The bounded
      independent review requested above is offered as the substitute
      independent check, on the basis the governor accepted for v2.6.0
      (`kpkk`, `33br`) and through the v2.7.0/v2.8.0 cycle. The governor
      ratifies or rejects that substitution at Gate 1; this field records
      the request, not a granted waiver.
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Consulted and material. This plan changes gate mechanics and adds a
      blocking check, which is a security-boundary-adjacent surface: the
      failure mode it addresses is a gate being silently weakened. No
      credential, personal-data, or access-control content is involved, and
      the change adds no dependency or network path. The infosec-relevant
      requirement is that the new check cannot be satisfied by an empty or
      absent test, addressed in acceptance criteria 6 and 7.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate-change discipline, and the deletion-blind changed-file validators

## Decision requested

Approve **A, B and C** below as one Gate 1 scope. A is the governing rule
change; B is the defect that exposed the gap and the first change to ship
under A; C is the mechanical enforcement without which A is prose only.

## Evidence

Two independent findings, hours apart, in this session.

**The defect.** `.gitlab-ci.yml:870`, inside `agov:governance:validate`:

```
git diff --name-only -z "$base"...HEAD -- 'docs/plans/*.md' \
  | xargs -0r "$agov_bin" plan validate
```

`git diff --name-only` lists deleted paths. Any merge request that deletes a
plan file hands `agov plan validate` a nonexistent path and fails at blocking
severity (`plan_validator.rs:24`). MR !2206 — a governor-ordered revert — is
blocked by this today. Verified:

```
$ git diff --name-only            e5c8377c...bd3f4c82 -- 'docs/plans/*.md'
docs/plans/20260829T183326Z-calm-explanatory-visual-system.md
$ git diff --name-only --diff-filter=d e5c8377c...bd3f4c82 -- 'docs/plans/*.md'
(empty)
```

**The governance gap.** The obvious remedy is a one-line `--diff-filter`
change, and under current rules that would have been *legitimate* as a
fast-track single-file bug fix:

- `agf/rules/workflow/simplicity-rule.md:191` lists fast-track
  ineligibility as "PII, security boundaries, architecture, data model
  changes". **Gate mechanics are absent.** (The narrow sources/radar fast
  path at the same rule's later section *is* correctly disqualified by "gate,
  schema, validator, CI, merge, or release-policy change" — the general
  waiver simply never inherited that list.)
- `agf/rules/always/gate-self-test-rule.md:12` fires on "**introducing** a
  new gate". **Modifying an existing gate is not covered**, so a change that
  weakens one carries no negative-test obligation.

Together these permit the exploit the governor named: a gate is weakened by a
single-line diff, labelled a bug fix, waived past Gate 1, and merged with no
record that a gate changed at all. Nothing distinguishes that diff from a
genuine fix at review time.

This is not hypothetical framing. The author of this plan proposed exactly
that fast-track fix, and the governor rejected it. The rule as written would
have permitted it.

## A — Gate-change discipline (the governing rule change)

Both changes **extend existing rules** rather than adding a new one, per the
standing preference for extending over new ceremony.

**A1.** `agf/rules/always/gate-self-test-rule.md` — widen the trigger from
"introducing a new gate" to "introducing **or modifying** a gate". A
modification MUST carry a negative test proving the gate still fires on the
known-bad input it exists to catch, *after* the change. Rationale to be
stated in the rule: at diff level a weakening and a fix are
indistinguishable; the surviving negative test is the only thing that
separates them.

**A2.** `agf/rules/workflow/simplicity-rule.md:191` — add gate mechanics to
the general fast-track waiver's ineligibility list, aligning it with the
disqualifier list the same rule already applies to its narrow research fast
path.

Foundry-local projections in `.claude/rules/` are updated to mirror both.

## B — The deletion-blind validators (first change to ship under A)

Two sites feed `git diff --name-only` into a validator. They get **different**
treatment; a single blanket filter would remove a block that should exist.

| Site | Deletion is | Treatment |
|---|---|---|
| `.gitlab-ci.yml:870` plans | legitimate — revert, supersession, cleanup | exclude deletions from the validate feed |
| `.gitlab-ci.yml:866` coordination | **illegitimate** — records are append-only | exclude from the validate feed, and reject with an explicit diagnostic naming the append-only rule |

Today a coordination deletion is blocked only accidentally, by a `file not
found` message that explains nothing. B replaces that accident with an
intentional check.

Per A1, B ships with negative tests proving both validators still fire on a
genuinely invalid added or modified file after the filter — otherwise the fix
blinds the gate it repairs.

## C — Mechanical enforcement

A is prose. `gate-self-test-rule.md`'s own opening argument is that "process
discipline alone is at-will and breaks under speed", and this project has
three documented incidents where a gate shipped broken because nobody
verified it. Shipping A without C would repeat the pattern the rule exists to
name.

**Design.** A declared gate registry lists gate-mechanics paths and, for
`.gitlab-ci.yml`, the named gate jobs. A new check blocks when a merge
request changes a registered gate surface and changes no declared test path.

**Blocking authority is narrow and mechanical**, per
`agf/rules/always/tool-authority-boundary-rule.md` (which requires a named
rule to grant any check blocking authority — A1 grants this one):

- **May block:** a diff touching a registered gate surface with no
  accompanying change to any declared test path.
- **Stays human judgment:** whether the test is a genuine negative test.

This is honestly a **proxy, not proof**. A determined author can write an
empty test. What it removes is the *silent* single-line path: the bypass now
requires deliberately authoring a fake test, which is visible in review and
is a materially higher bar than a one-liner nobody looks at.

**Scope risk, stated plainly.** `.gitlab-ci.yml` changes are frequent and
mostly not gate-related; blocking every one of them on a test-path touch
would be noisy enough to invite routing around. This is why the registry
names gate *jobs*, not the whole file, and why the check maps changed line
ranges (`git diff -U0`) to registered job blocks. If Gate 1 judges that
mapping too fragile for a first version, the fallback is to register only the
validator crate paths and hooks, and treat `.gitlab-ci.yml` as advisory until
the registry proves itself. **Recommendation: implement the job-range
mapping**; the fallback is named so the decision is available, not to hedge.

Per A1 — and per `gate-self-test-rule.md` applied to itself — C ships with
its own negative tests.

## Rule-projection screen

Per `agf/rules/workflow/documentation-rule-projection-rule.md`. Structured
evidence: `docs/reviews/rule-projection/gate-change-discipline.yaml`.

| Surface | Disposition |
|---|---|
| Normative owner | **updated** — `agf/rules/always/gate-self-test-rule.md` (A1, and grants C blocking authority); `agf/rules/workflow/simplicity-rule.md` (A2) |
| Adopter explanation | **updated** — `agf/docs/site/docs/concepts/roles-and-gates.md:95` states fast-track eligibility to adopters as "no shared interface, data model, or security boundary change, no claim conflict". It carries the identical gap as the rule and must gain gate mechanics, or the page contradicts A2 |
| Runbook / situation map | **updated** — `agf/docs/site/docs/operate/day-to-day.md:88` is the checklist line where an operator actually chooses fast track. A pointer is insufficient at the surface where the wrong decision gets made; it gains one short clause, not copied rule text |
| AI landing / loaders | **updated** — `.claude/rules/` projections of both rules; `AGENTS.md` and `.claude/CLAUDE.md` need no change (`CLAUDE.md` is a pointer via `@../AGENTS.md`, and the rules load directly) |
| Validation / drift evidence | **implemented** — C is the mechanical check; its limits are stated above rather than overclaimed |
| Distribution | both rules ship in `agf/rules/`; registry and check ship with tooling — confirmed in the manifest |

## Acceptance criteria

1. `gate-self-test-rule.md` covers modification as well as introduction, in
   both the canonical `agf/rules/` copy and the `.claude/rules/` projection.
2. `simplicity-rule.md:191`'s ineligibility list includes gate mechanics, in
   both copies.
3. `.gitlab-ci.yml:870` no longer passes deleted paths to `plan validate`;
   MR !2206's exact diff produces an empty plan-validate feed.
4. `.gitlab-ci.yml:866` rejects a coordination-record deletion with an
   explicit diagnostic naming the append-only rule, not `file not found`.
5. Negative tests prove both validators still fire on a genuinely invalid
   added or modified file after the filter, with reviewer-visible transcripts
   — "tested locally" is insufficient per `gate-self-test-rule.md`.
6. C blocks a merge request that changes a registered gate surface and no
   declared test path, demonstrated on a known-bad input.
7. C does **not** block a gate change accompanied by a test-path change, and
   does not block a non-gate `.gitlab-ci.yml` change, both demonstrated.
8. The projection manifest exists and `agov plan projection-check` passes
   without `--allow-planned` at Gate 2.
9. MR !2206 re-runs green and the governor-ordered revert merges.

## Risks and rollback

**Over-blocking.** C's registry could catch routine CI edits. Mitigated by
job-range mapping and by the named fallback. Rollback is removing entries
from the registry — no code change.

**Proxy weakness.** C cannot tell a real negative test from an empty one.
Stated in the rule text itself rather than left implied, so no reader mistakes
the check for proof.

**Bootstrapping.** This plan changes gates, so A1 applies to A1. B and C ship
with negative tests; A1 and A2 are rule-text changes with no mechanism to
self-test, and their proof is C's demonstrations.

**Rollback.** A and B revert cleanly. C reverts by removing the check job and
registry; nothing else depends on it.

## Out of scope

Issue #119, issue #120, the v2.9.0 slices, and any audit of gates other than
the two sites in B. If the registry work in C surfaces further deletion-blind
or unguarded gates, they are reported as findings and routed to their own
issue — not folded into this plan. Per the lesson in learning
`20260829T192500Z-4vn8`, that report is expected to name the **class** it
searched, not only the instances it found.

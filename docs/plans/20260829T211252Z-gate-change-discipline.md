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

**B1 — the two live foundry feeds.** They get **different** treatment; a
single blanket filter would remove a block that should exist.

| Site | Deletion is | Treatment |
|---|---|---|
| `.gitlab-ci.yml:870` plans | legitimate — revert, supersession, cleanup | exclude deletions from the validate feed |
| `.gitlab-ci.yml:866` coordination | **illegitimate** — records are append-only | exclude from the validate feed, and reject with an explicit diagnostic naming the append-only rule |

Today a coordination deletion is blocked only accidentally, by a `file not
found` message that explains nothing. B1 replaces that accident with an
intentional check.

**B2 — the same defect in shipped adopter guidance.** Added after review
finding GCD-4 (`phe4`). Four documented CI sketches copy the same commands:

- `agf/docs/site/docs/adopt/tooling-and-tokens.md:134-137`
- `agf/docs/site/docs/advanced/contracts-and-validators.md:117-120`, `158-161`
  (GitLab), `186-189` (GitHub Actions)

Fixing foundry while leaving these would keep telling adopters to install the
defect. This is a direct impact of B, not unrelated documentation cleanup.

**The adopter examples carry a second, more severe defect that the review did
not name.** All three GitLab sketches use `"$CI_MERGE_REQUEST_TARGET_BRANCH_SHA"`
as the diff base with no fallback. Foundry's own CI removed exactly that
pattern and documents why at `.gitlab-ci.yml:832-834`: the variable is
"empty/unset in real, common pipeline shapes (first pipeline on a branch,
force-push, shallow clone)". With an empty base the range degenerates to
`...HEAD`, which is `HEAD...HEAD`. Verified:

```
$ EMPTY=""; git diff --name-only -z "$EMPTY"...HEAD -- 'docs/plans/*.md'; echo "exit=$?"
exit=0
```

Empty output, `xargs -0r` declines to run, exit 0. **The adopter's governance
gate silently validates nothing and reports success.** Deletion-blindness
produces a false block, which is visible and annoying; this produces a false
pass, which is invisible. It is the worse of the two and it is what we ship.

B2 therefore corrects both defects in all four sketches, and gives the GitLab
sketches the same deterministic fetch-and-resolve base the foundry job uses,
failing closed when no base resolves.

**Generated site output is not a concern.** `agf/site-html/` is untracked and
rebuilt by the `site-html-build` job; correcting the Markdown source is
sufficient and no generated artifact is hand-edited. This resolves GCD-4's
site-consequence question definitively rather than by estimate.

Per A1, B ships with negative tests proving both validators still fire on a
genuinely invalid added or modified file after the filter — otherwise the fix
blinds the gate it repairs.

## C — Mechanical enforcement

A is prose. `gate-self-test-rule.md`'s own opening argument is that "process
discipline alone is at-will and breaks under speed", and this project has
three documented incidents where a gate shipped broken because nobody
verified it. Shipping A without C would repeat the pattern the rule exists to
name.

**Design, revised after review findings GCD-1, GCD-2 and GCD-3 (`phe4`).**
The first version registers **exact paths only**. Diff-hunk-to-YAML-job
mapping is removed from blocking authority entirely — see "What was dropped
and why" below.

*Exact implementation boundary (GCD-1).*

| Artifact | Path |
|---|---|
| Registry | `agf/gates.yaml` |
| Registry schema | `agf/schemas/gate-registry.schema.json` |
| Checker | `tooling/rust/crates/agov-validators/src/gate_change.rs` |
| Command | `agov gate change-check --base-ref <ref>` |
| CI integration | existing `agov:governance:validate` job |
| Integration tests | `tooling/rust/crates/agov/tests/gate_change_check.rs` |

The command name follows the established `agov coord publication-check` /
`agov plan projection-check` pattern rather than inventing a new family.

*Registry shape.* Each entry declares a gate's `id`, its `surfaces` (exact
paths), its `tests` (exact paths), and its `invocation` — the file, job name,
and the string that job must contain to actually run the gate.

*The check, three assertions.*

1. **Test-accompaniment.** A diff touching any registered `surfaces` path must
   also touch that gate's declared `tests` paths.
2. **Invocation presence.** For each registered gate, the named job must still
   exist in the named file and still contain the required invocation string.
   Renaming or deleting the job fails here. This is a presence assertion, not
   hunk mapping — objective, and it closes GCD-2's rename/delete bypass
   without parsing YAML semantics.
3. **Registry integrity.** Every registered path must resolve. A gate `id`
   may not disappear: retiring one requires an explicit `retired:` marker
   with a reason, mirroring the append-only discipline this project already
   applies to coordination records and tags. Silent deletion fails.

*Gate logic moves out of YAML (GCD-2).* The inline governance script becomes
`.gitlab/scripts/governance-validate.sh`, following the existing
`.gitlab/scripts/check-rust-cache-policy.sh` convention. The script path is
the registered surface; `.gitlab-ci.yml` keeps only a stable invocation,
covered by assertion 2. This is what makes registration precise **and**
removes the noise objection — routine CI edits touch no registered surface,
while the gate's actual logic is fully covered.

*Self-protection (GCD-3).* `agf/gates.yaml`, the schema, the checker, and
`.gitlab/scripts/governance-validate.sh` are themselves registered surfaces of
a `gate-change-check` gate entry. Deregistering a gate, weakening the checker,
or editing the registry all require a test change in that gate's declared
tests, and assertion 3 blocks silent removal outright.

**What was dropped and why.** The first draft proposed mapping `git diff -U0`
hunks to registered `.gitlab-ci.yml` job blocks. GCD-2 is correct that this
has real bypasses in this repository specifically — it already uses `extends`,
YAML anchors and aliases, `!reference`, hidden jobs, and top-level variables,
so a gate's effective behaviour can change with no hunk intersecting the
registered block. The review also caught that the fallback I named was worse
than useless: registering only validator crates and hooks would have left
**B's own motivating change** — a gate job in `.gitlab-ci.yml` — mechanically
uncovered. A fallback that fails to cover the plan's own case is not a
fallback. Job-range classification may return later as advisory output; it
does not carry blocking authority here.

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

**Scope risk, addressed rather than accepted.** `.gitlab-ci.yml` changes are
frequent and mostly not gate-related; blocking all of them on a test-path
touch would invite routing around. Extracting the gate logic to a registered
script resolves this without a fragile mapping: routine CI edits touch no
registered surface, and the gate's logic is fully covered wherever it lives.

Per A1 — and per `gate-self-test-rule.md` applied to itself — C ships with
its own negative tests.

## Review disposition — `phe4` (independent Gate 1 review, Codex host)

Recommendation was **BLOCK**. All three blocking findings are upheld and
addressed in this revision; none is argued down.

| Finding | Disposition |
|---|---|
| **GCD-1** implementation boundary not executable | **Upheld.** Slice C now names the registry, schema, checker, command, CI integration, and test paths exactly, plus the registry shape and the three assertions. |
| **GCD-2** hunk-to-YAML mapping has real bypasses | **Upheld, design replaced.** Mapping removed from blocking authority. Gate logic extracts to a registered script; an invocation-presence assertion closes the rename/delete bypass. The review also caught that my named fallback left B's own motivating change uncovered — that was the sharper half of the finding. |
| **GCD-3** registry/checker can weaken itself | **Upheld.** Registry, schema, checker and script are registered surfaces of their own gate entry; retirement requires an explicit marker; the self-contradicting rollback statement is corrected. |
| **GCD-4** adopter examples retain the defect | **Adopted into B2, not deferred.** The reviewer routed the scope decision to the governor and recommended inclusion; revising a still-pending plan is the least disruptive path, so it is included for your decision rather than held. |

**One finding extends beyond the review.** GCD-4 named deletion-blindness in
the adopter sketches. Verifying it surfaced a second, more severe defect in
the same lines — an unguarded `$CI_MERGE_REQUEST_TARGET_BRANCH_SHA` base that
makes the adopter's gate exit 0 without validating anything. Documented in B2
with a reproduction.

**On the class-search question `j1q8` asked.** The review confirms two live
foundry feeds is the complete set, but my class search was scoped to *live CI
configuration* and missed *documented examples of the same command*. That is
the third instance this session of the failure recorded in learning
`20260829T192500Z-4vn8`. The class was "every place this command shape is
published", not "every place it runs".

## Rule-projection screen

Per `agf/rules/workflow/documentation-rule-projection-rule.md`. Structured
evidence: `docs/reviews/rule-projection/gate-change-discipline.yaml`.

| Surface | Disposition |
|---|---|
| Normative owner | **updated** — `agf/rules/always/gate-self-test-rule.md` (A1, and grants C blocking authority); `agf/rules/workflow/simplicity-rule.md` (A2) |
| Adopter explanation | **updated** — `agf/docs/site/docs/concepts/roles-and-gates.md:95` states fast-track eligibility to adopters as "no shared interface, data model, or security boundary change, no claim conflict". It carries the identical gap as the rule and must gain gate mechanics, or the page contradicts A2. Also `agf/docs/site/docs/adopt/tooling-and-tokens.md` and `agf/docs/site/docs/advanced/contracts-and-validators.md`, whose CI sketches ship both defects corrected in B2 |
| Runbook / situation map | **updated** — `agf/docs/site/docs/operate/day-to-day.md:88` is the checklist line where an operator actually chooses fast track. A pointer is insufficient at the surface where the wrong decision gets made; it gains one short clause, not copied rule text |
| AI landing / loaders | **updated** — `.claude/rules/` projections of both rules; `AGENTS.md` and `.claude/CLAUDE.md` need no change (`CLAUDE.md` is a pointer via `@../AGENTS.md`, and the rules load directly) |
| Validation / drift evidence | **implemented** — C is the mechanical check; its limits are stated above rather than overclaimed |
| Distribution | both rules ship in `agf/rules/`; the two corrected adopter pages ship under `agf/docs/site/`; `agf/gates.yaml` and its schema ship under `agf/`; the checker ships with the Rust tooling — all named in the manifest |

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
   added or modified file after the filter, exercising add, modify and delete
   statuses independently, with reviewer-visible transcripts — "tested
   locally" is insufficient per `gate-self-test-rule.md`.
6. All four adopter CI sketches in B2 are corrected for **both** defects, and
   the GitLab sketches fail closed when no base resolves. A transcript shows
   the corrected sketch rejecting an unresolvable base rather than exiting 0.
7. C blocks a merge request that changes a registered gate surface and no
   declared test path, demonstrated on a known-bad input.
8. C does **not** block a gate change accompanied by a test-path change, and
   does not block a non-gate `.gitlab-ci.yml` change, both demonstrated.
9. Negative fixtures required by GCD-3, each demonstrated failing: gate
   deregistration from `agf/gates.yaml`; registered job rename; registered job
   deletion; checker mutation; and a diff satisfying the test-accompaniment
   assertion with an **unrelated** test-path touch — the last recorded as a
   demonstration of the proxy's known limit, not as a pass.
10. `agov gate change-check` rejects a registry entry whose declared surface,
    test, or invocation path does not resolve.
11. The projection manifest exists and `agov plan projection-check` passes
    without `--allow-planned` at Gate 2.
12. MR !2206 re-runs green and the governor-ordered revert merges.

## Risks and rollback

**Over-blocking.** C's registry could catch routine CI edits. Resolved by
extracting gate logic to a registered script so routine `.gitlab-ci.yml`
edits touch no registered surface.

**Proxy weakness.** C cannot tell a real negative test from an empty one, and
acceptance criterion 9's last fixture demonstrates exactly that limit rather
than hiding it. Stated in the rule text itself so no reader mistakes the
check for proof.

**Bootstrapping.** This plan changes gates, so A1 applies to A1. B and C ship
with negative tests; A1 and A2 are rule-text changes with no mechanism to
self-test, and their proof is C's demonstrations.

**Rollback — corrected after GCD-3.** The first draft said C could be rolled
back by "removing entries from the registry — no code change". That
contradicted A1 inside the same plan: under A1 a registry removal *is* a gate
modification, and assertion 3 now blocks silent removal outright. Rollback of
any slice follows the same Gate 1 and negative-test discipline as any other
gate change — retiring a registry entry requires an explicit `retired:`
marker with a reason, not a deletion. A and B revert cleanly as ordinary
changes under that same discipline.

## Out of scope

Issue #119, issue #120, the v2.9.0 slices, and any audit of gates other than
the two sites in B. If the registry work in C surfaces further deletion-blind
or unguarded gates, they are reported as findings and routed to their own
issue — not folded into this plan. Per the lesson in learning
`20260829T192500Z-4vn8`, that report is expected to name the **class** it
searched, not only the instances it found.

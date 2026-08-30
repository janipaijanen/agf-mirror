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
      Consulted and material. This plan changes gate mechanics, which is a
      security-boundary-adjacent surface: the failure mode it addresses is a
      governance gate silently reporting success. No credential,
      personal-data, or access-control content is involved, and the change
      adds no dependency or network path. The infosec-relevant requirement is
      that the corrected changed-file path fails closed on every failure mode
      rather than masking a failed diff, addressed in acceptance criteria 7
      and 8. An earlier revision of this field claimed a mechanical check
      "cannot be satisfied by an empty or absent test"; review j1ht (GCD-R3)
      correctly identified that as contradicting slice C's own stated proxy
      limit. Slice C is now split out, and no such claim is made here.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate-change discipline, and the deletion-blind changed-file validators

## Decision requested

Approve **A and B** as this plan's Gate 1 scope. A is the governing rule
change; B is the defect that exposed the gap and the first change to ship
under A. Slice C (mechanical enforcement) is split into its own Gate 1 plan —
see the split section below for the reasoning and its honest cost.

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

**B3 — the fail-closed changed-file contract, added after `j1ht` finding
GCD-R4.** The first revision proposed giving the adopter sketches "the same
deterministic fetch-and-resolve base the foundry job uses". `j1ht` showed that
is insufficient, because foundry's own pattern does not fail closed *after*
the base resolves. Reproduced here:

```
$ sh -c 'set -eu; git diff --name-only -z does-not-resolve...HEAD -- \
    "docs/plans/*.md" | xargs -0r true; echo PIPELINE_EXITED_ZERO'
fatal: bad revision 'does-not-resolve...HEAD'
PIPELINE_EXITED_ZERO
$ echo $?
0
```

`set -e` observes only a pipeline's final exit status, so a failed `git diff`
is masked by `xargs`. `.gitlab-ci.yml:866-871` has exactly this shape.

**This is not only an adopter-documentation defect.** `agov:governance:validate`
inherits the project's default `GIT_DEPTH` of 20 — stated in this repository at
`.gitlab-ci.yml:981` and attributed there to the GitLab project API; only
`commercial-corpus:validate` overrides it to `0`. A triple-dot diff requires a
merge base, which a depth-20 shallow clone may not contain once a branch has
diverged past roughly twenty commits. At this project's merge rate that is
ordinary rather than contrived. Whether it is *currently firing* is under
empirical verification (`argd`); the plan does not assert it is, and B3 is
correct regardless because the masking is proven above.

B therefore ships one **fail-closed changed-file contract**, applied to the
extracted foundry script and to all four adopter sketches identically:

1. resolve the target ref explicitly, failing closed when it is empty or the
   fetch fails;
2. prove a merge base exists, failing closed when it does not;
3. capture the `git diff` result and check its status **before** any consumer
   runs — writing the NUL-separated list to a temporary file only on success,
   never piping diff directly into `xargs`;
4. only then invoke the validator on that list.

B2 corrects both defects in all four sketches using this contract, rather than
copying foundry's current pattern — which is itself defective.

**Generated site output is not a concern.** `agf/site-html/` is untracked and
rebuilt by the `site-html-build` job; correcting the Markdown source is
sufficient and no generated artifact is hand-edited. This resolves GCD-4's
site-consequence question definitively rather than by estimate.

Per A1, B ships with negative tests proving both validators still fire on a
genuinely invalid added or modified file after the filter — otherwise the fix
blinds the gate it repairs.

## C is split out — and why that is not a retreat

Slice C (mechanical enforcement) is **removed from this plan** and resubmitted
as its own Gate 1 plan. This is the governor's scope decision, taken after
`j1ht` returned the third consecutive BLOCK on this document, all of them
against C's enforcement semantics while A and B were repeatedly cleared.

The case for splitting is not that C matters less. It is that:

- A and B are agreed-good across two independent reviews and are blocking a
  **live defect** — see B1 below, and the fail-open now suspected in
  foundry's own `agov:governance:validate`.
- C's remaining findings (GCD-R1's distribution and live-registry boundary,
  GCD-R2's effective-execution semantics, GCD-R3's retirement state machine)
  are genuine design work, not corrections. Each round of holding A and B
  hostage to that design has cost a full cross-host review cycle.
- MR !2206, a governor-ordered revert, has been blocked on B for this entire
  period.

**The honest cost of splitting, stated rather than buried.** A without C is
prose-enforced discipline, which is precisely what
`gate-self-test-rule.md` opens by arguing against: "process discipline alone
is at-will and breaks under speed." Shipping A alone therefore leaves the
governor's original concern — that a gate change could be slipped through as
a one-file fix — addressed by rule but not by mechanism. That gap is real
until C lands, and C is tracked as a named successor plan rather than an
aspiration.

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

## Review disposition — `j1ht` (exact-head re-review, Codex host)

Third consecutive BLOCK, four findings. Two are resolved here; two travel with
slice C to its own plan, which is the governor's scope decision.

| Finding | Disposition |
|---|---|
| **GCD-R1** implementation boundary incomplete | **Travels with C.** The missing `agov/src/main.rs` dispatch, `agov-validators/src/lib.rs` export, release-manifest entry, and the live-registry-versus-shipped-template boundary are all C's. The one part belonging here — B's shell negative-test path — is now acceptance criterion 9 and named in the manifest. |
| **GCD-R2** invocation presence does not protect execution | **Upheld; the overclaim is withdrawn here and the design question travels with C.** The claim that the gate was "fully covered wherever it lives" was wrong: a job can keep the invocation string while neutralised by `rules: when: never`, `allow_failure: true`, an earlier `exit 0`, or inherited configuration. Acceptance criterion 10 now forbids any text in this plan or its shipped output from asserting a mechanical check proves a gate executes. |
| **GCD-R3** retirement and self-check semantics incoherent | **Split.** The retirement state machine travels with C. The part that was this document's own error — a front-matter infosec rationale asserting the check "cannot be satisfied by an empty or absent test", contradicting C's stated proxy limit, with stale criteria references after renumbering — is corrected in the front matter above. |
| **GCD-R4** B2's pattern still masks diff failures | **Upheld and materially widened.** See B3. The finding is correct, and following it into foundry's own configuration shows `.gitlab-ci.yml:866-871` shares the shape while running at `GIT_DEPTH` 20. B now ships a fail-closed contract rather than copying foundry's defective pattern. |

The advisory projection-lifecycle reading is accepted: omitting not-yet-existing
non-owner paths is correct at Gate 1, and `validation_drift_evidence` is
rephrased below as prospective.

**GCD-R2 is the finding I flagged against myself.** The `gh77` re-review
request named this as the weakest point of the redesign and asked whether the
invocation assertion closed the bypass or merely relocated it. It relocated
it. Recording that the prediction was right does not make the design right —
the plan still shipped the overclaim in its prose, and a reviewer had to
remove it.

## Review disposition — `8mar` and `at74`

`8mar` returned **BLOCK** on this plan with two findings. Both were my own
bookkeeping errors rather than design faults, and both are corrected.

| Finding | Disposition |
|---|---|
| **GDAB-1** projection still claimed slice-C distribution | **Upheld.** The distribution row retained a paragraph claiming `agf/gates.yaml` and the registry schema would ship — including the root path `j1ht` had already rejected. I updated the row's `paths:` array and the plan's screen table but left that prose behind. A projection manifest is structured Gate 1 evidence, so this assigned overlapping scope to two plans at once. Removed. |
| **GDAB-2** B's shell-test path unnamed | **Upheld.** The plan asserted the path was "named in the manifest" when nothing was there — a claim about itself that was simply false. `.gitlab/scripts/changed-files.sh`, `tests/changed-files-failclosed.sh`, and `tests/fixtures/changed-files/` are now named in both the manifest and acceptance criterion 9. |

`8mar` confirms the four-step contract itself is sound.

**`at74` verified the mechanism and corrected my urgency claim.** It built a
repository with target and feature branches each 40 commits past a common
base, cloned at `--depth 20`, fetched the target, and reproduced the silent
pass:

```
git merge-base origin/main HEAD   # exit 1
git diff --name-only -z origin/main...HEAD -- 'docs/plans/*.md' | xargs -0r true
fatal: origin/main...HEAD: no merge base
pipeline exit=0
```

Runner job `16065719319` independently confirms `git depth set to 20` on the
real governance runner.

It also **refuted** the urgency claim in `argd`: a fully paginated scan of all
60 open merge requests found a maximum source-commit count of **19**, with the
next largest 11 and 9. My statement that several open MRs qualify was wrong.
This is a reachable latent fail-open at the current depth, not one currently
firing — which is why `argd` framed it as a hypothesis for verification rather
than a finding, and why that framing was worth the caution.

`at74` recommends keeping the correction inside slice B rather than opening a
parallel work order on the same CI hunk. Adopted: B3 stays here.

## Rule-projection screen

Per `agf/rules/workflow/documentation-rule-projection-rule.md`. Structured
evidence: `docs/reviews/rule-projection/gate-change-discipline.yaml`.

| Surface | Disposition |
|---|---|
| Normative owner | **updated** — `agf/rules/always/gate-self-test-rule.md` (A1, and grants C blocking authority); `agf/rules/workflow/simplicity-rule.md` (A2) |
| Adopter explanation | **updated** — `agf/docs/site/docs/concepts/roles-and-gates.md:95` states fast-track eligibility to adopters as "no shared interface, data model, or security boundary change, no claim conflict". It carries the identical gap as the rule and must gain gate mechanics, or the page contradicts A2. Also `agf/docs/site/docs/adopt/tooling-and-tokens.md` and `agf/docs/site/docs/advanced/contracts-and-validators.md`, whose CI sketches ship both defects corrected in B2 |
| Runbook / situation map | **updated** — `agf/docs/site/docs/operate/day-to-day.md:88` is the checklist line where an operator actually chooses fast track. A pointer is insufficient at the surface where the wrong decision gets made; it gains one short clause, not copied rule text |
| AI landing / loaders | **updated** — `.claude/rules/` projections of both rules; `AGENTS.md` and `.claude/CLAUDE.md` need no change (`CLAUDE.md` is a pointer via `@../AGENTS.md`, and the rules load directly) |
| Validation / drift evidence | **advisory only** — this plan ships no mechanical check. B's corrections are covered by their own negative tests; mechanical gate-change enforcement is slice C's, in its own plan |
| Distribution | both rules ship in `agf/rules/`; all four corrected Pages surfaces ship under `agf/docs/site/`. No registry, schema, or checker is claimed here — those travel with slice C, and `j1ht` GCD-R1 correctly noted `agf/dist/release-manifest.yaml` would not ship a root `agf/gates.yaml` anyway |

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
7. The fail-closed changed-file contract is applied identically to the
   extracted foundry script and all four adopter sketches, and a transcript
   demonstrates each of its four failure modes exiting non-zero: empty target
   variable, fetch failure, missing merge base, and `git diff` failure.
8. A shallow/diverged-history fixture — not only an empty-variable transcript
   — demonstrates the contract failing closed where the current pattern
   reports success, per `j1ht` GCD-R4.
9. B's fail-closed contract ships as the shared helper
   `.gitlab/scripts/changed-files.sh`, replacing both inline piped diffs at
   `.gitlab-ci.yml:866-871`; its negative tests live at
   `tests/changed-files-failclosed.sh` with fixtures under
   `tests/fixtures/changed-files/`, and run in a CI job rather than only
   locally.
10. No claim anywhere in the shipped rule, plan, or adopter text asserts that
    a mechanical check proves a gate *executes*; that claim is deferred with
    slice C.
11. The projection manifest exists and `agov plan projection-check` passes
    without `--allow-planned` at Gate 2.
12. MR !2206 re-runs green and the governor-ordered revert merges.

## Risks and rollback

**Prose-only enforcement.** A is a rule change with no mechanism behind it
until slice C ships separately. This is the split's real cost and is stated in
the split section rather than left implied.

**Fix correctness.** A wrong fail-closed contract shipped to adopters is worse
than the defect it replaces, since it would be copied into their pipelines.
Acceptance criteria 7 and 8 require each failure mode demonstrated
individually, including a shallow/diverged-history fixture rather than only an
empty-variable transcript.

**Rollback.** A and B revert as ordinary changes, under the same Gate 1 and
negative-test discipline A itself establishes — a revert of a gate change is
still a gate change. The registry-rollback contradiction `j1ht` GCD-R3
identified travels with slice C to its own plan, where the retirement state
machine is defined rather than asserted.

## Out of scope

Issue #119, issue #120, the v2.9.0 slices, and any audit of gates other than
the two sites in B. If the registry work in C surfaces further deletion-blind
or unguarded gates, they are reported as findings and routed to their own
issue — not folded into this plan. Per the lesson in learning
`20260829T192500Z-4vn8`, that report is expected to name the **class** it
searched, not only the instances it found.

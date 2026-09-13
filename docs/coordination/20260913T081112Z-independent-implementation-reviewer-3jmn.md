---
kind: review
from: independent-implementation-reviewer
to: governor-reviewer
id: 20260913T081112Z-independent-implementation-reviewer-3jmn
timestamp: 2026-09-13T08:11:12Z
immutable: true
references-to: [20260912T225022Z-coder-tester-kd8g, 20260913T080424Z-planner-architect-7bpq]
topic: cpb2-independent-security-review
requires-ack: true
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# CP-B-2 independent implementation review: CLEAR

## Scope reviewed

MR !2760, branch `codex/cpb2-publication-hold-implementation`, exact head
`de12dff3c1f212e93d1d59ca66abaada90f5d131`. Reviewed against
`docs/plans/20260912T084900Z-cpb2-publication-hold-urgency.md`,
`docs/reviews/rule-projection/20260912T084900Z-cpb2-publication-hold-urgency.yaml`,
and `agf/rules/workflow/coordination-publication-rule.md`. Full diff
`origin/main...de12dff3c1f212e93d1d59ca66abaada90f5d131` read in its
entirety (43 files, 6667 insertions), including all 14 checked-in golden
fixtures under `tooling/rust/crates/agov-coord/tests/fixtures/hold-status/`.
Build-engineer's CI/build review (green, 17/17 required jobs at this exact
head, pipeline `2843732220`) is out of scope for this review; this review
covers implementation logic and semantics only. I did not run the crate
test suite myself in this shared worktree to avoid disturbing concurrent
sessions; build-engineer's independent exact-head CI result is treated as
the test-execution evidence, and I independently verified test *content*
(assertions, fixtures, coverage of the scrutiny areas below) by reading it.

## Overall verdict: CLEAR

I found no BLOCKING defects. Two ADVISORY notes below, both already
acknowledged in the plan's own text as inherent, accepted boundaries of a
read-only advisory tool, not implementation gaps.

## Findings by scrutiny area

### 1. Git-pinned input selection — CLEAR

`hold_git.rs::resolve_target` rejects `HEAD`, `origin/HEAD`, any symbolic
ref, any 40-hex SHA, any tag, and any revision expression (`main~0`) as a
`--target-branch` value; only an unambiguous `refs/heads/<name>` or
`refs/remotes/<remote>/<name>` (or an unambiguous short form resolving to
exactly one of those) is accepted (`hold_git.rs:119-182`). Candidate refs
are resolved once each to an immutable commit SHA before any tree/blob
read (`hold_git.rs:282-316`); the test
`all_refs_resolve_before_reads_and_later_deletion_cannot_change_inventory`
(`hold_git.rs:614-638`) injects ref deletion/HEAD-detachment *during*
Git-command execution via a wrapping `Moving` mock and asserts the
resulting inventory is unaffected — a genuine adversarial-timing test, not
merely a comment claiming pinning happens. `target_identity_candidates_and_raw_bytes_are_pinned`
additionally dirties the working tree after resolution and asserts the
read inventory is unchanged (`hold_git.rs:519-522`). Assumed intent: read
outcomes must be a pure function of resolved commit objects, immune to
concurrent ref movement or working-tree edits. Confirmed correct.

### 2. Malformed/global failure precedence — CLEAR (fails closed)

`hold_evaluate.rs` computes `coverage` from either an unavailable source or
any `Severity::Unavailable` problem (`hold_evaluate.rs:274-280`), and a
"semantic_problem" flag from any cross-record `Severity::Semantic` key
(`hold_evaluate.rs:284-287`); both propagate as global overrides applied to
*every* action row, not only the record with the defect
(`hold_evaluate.rs:322-335`, confirmed by
`malformed_candidate_and_unavailable_source_override_every_action`,
`hold_evaluate.rs:604-640`, which asserts all rows become `publish-now`).
`hold_classify.rs::classify_enabled` independently proves this
exhaustively: `exhaustive_structural_and_global_overrides_cannot_be_delayed`
(`hold_classify.rs:137-190`) iterates all 2^6 combinations of
urgent/control/ack/kind/incomplete/semantic and asserts `publish-now` is
returned whenever *any* is true, `review-required` only when *all* are
false. There is no code path that can turn a malformed or incomplete
result into a quiet no-op — the malformed/global-failure case fails toward
"treat as unsafe to delay," which is the fail-closed direction. Assumed
intent: a governance-hold feature must never silently swallow bad input as
"no hold." Confirmed correct.

### 3. Active overlap — CLEAR

`hold_status::interval_overlap` (`hold_status.rs:60-75`) compares only the
half-open effective intervals of two same-canonical-target holds,
independent of stored SHA (explicitly documented at line 60-61: "SHA
equality is not an input: a stale SHA must never conceal an active
same-target conflict"). `hold_evaluate.rs:214-258` marks both holds
`conflict:true` only for `Overlap::Current`, and only when the pair
concerns the *selected* target; past/future overlap is recorded as a
nonblocking advisory problem, matching the plan's exact specification. The
checked-in `conflict.json` golden (independently verified above) shows both
holds flagged `conflict:true`, an `active-overlap`/`semantic` problem, and
the associated record forced to `publish-now` — deterministic and
matches spec. `hold_status.rs` unit tests cover exact boundary
equality (`interval_overlap` at `begin==end` returns `None`, i.e.
touching-but-not-overlapping intervals do not conflict) and future-vs-
historical-vs-current classification relative to `now`.

### 4. End provenance — CLEAR (authenticated identically to start)

This is enforced at the **schema level**, not merely in prose:
`agf/schemas/coordination-message.schema.json`'s new `allOf` clause
(`schema_diff.txt:102-133`) applies `kind: decision-record`,
`from: governor-reviewer`, `urgency: urgent`, and `references-to.minItems:
1` to *any* record carrying a `publication_hold` object — this condition
is keyed only on the presence of `publication_hold`, not on `action`, so
it applies equally to `action: start` and `action: end`. `publication_hold.rs`'s
cross-field check additionally requires an `end`'s `hold_id` to appear
verbatim inside that same record's `references-to`
(`publication_hold.rs:73-86`), tested by
`end_requires_exact_reference_and_forbids_start_fields`
(`publication_hold.rs:216-236`), which also asserts an end record cannot
smuggle `target_sha`/`expires_at` fields. `hold_evaluate.rs`'s end-matching
logic (`hold_evaluate.rs:144-191`) requires an end's `target_branch` to
equal the matched start's `target_branch` and its timestamp to be
`>= start.timestamp`, else it is rejected as `Code::InvalidControl` (a
semantic error, which per finding #2 forces every affected row to
`publish-now`) — an unauthorized or mistargeted end cannot silently
terminate a hold (`invalid_end_cannot_silently_terminate_the_hold`,
`hold_evaluate.rs:585-602`, confirms the hold stays `Active` and the exit
code becomes 1). Assumed intent: "end" must carry the same authority
weight as "start," since a forged/premature end is exactly as dangerous as
a forged start. Confirmed correct — no code path lets an end from a
non-`governor-reviewer`, non-`urgent`, or non-`decision-record` author
validate.

### 5. External-check semantics — CLEAR

All Git reads in `hold_git.rs` run through the `Git` trait's single
`run()` implementation which shells to `git` with a fixed argv array (no
shell interpolation of user input) and `GIT_TERMINAL_PROMPT=0` (no
network prompt, no implicit network access — resolution is entirely local
ref/object lookups). `every_decisive_git_failure_is_reported_as_unavailable`
(`hold_git.rs:655-676`) injects a failure at every single call index across
a full invocation and asserts every one produces a `problems` entry and an
unavailable source — this is the load-bearing test for
"a failed read is unavailable evidence, not a successful empty result"
from `coordination-publication-rule.md`, and it is satisfied here by
direct enumeration of call sites, not spot-checked. `platform_diagnostics_do_not_renumber_final_rows`
(`hold_evaluate.rs:822-841`) confirms problem identity/ordering is stable
across different underlying Git stderr text for the same logical failure.
Assumed intent: match the existing publication-check gate's own
fail-closed discipline. Confirmed correct.

### 6. Rollout/forward-disablement — CLEAR

Older schema/tooling readers reject the new optional fields only if they
already reject unknown properties strictly (an adopter-side property, not
this implementation's), which the plan explicitly accepts and documents as
"older schema/tools reject unknown fields or lack hold-status; they are
not a supported reader once a channel uses these fields." No shipped code
path removes an existing field from an already-published record — the
`Unverified` marker type (`hold_result.rs:14-30`) is a zero-sized struct
whose `Serialize` impl always writes `false` and whose `Deserialize` impl
*rejects* an incoming `true` value with a hard error
(`serde::de::Error::custom("verification must remain false")`, tested at
`hold_result.rs:254-259`); there is no field, flag, or code path anywhere
in the reviewed diff capable of setting `publication_verified` or
`authority_verified` to `true`. The `evaluate_enabled(..., enabled: bool)`
forward-disablement path (`hold_evaluate.rs:42-48, 259-272`) is a strictly
more conservative evaluator — every candidate row becomes `publish-now`
with reason `hold-behavior-disabled`, verification fields stay `Unverified`
— and it is exercised by
`migration_fallback_keeps_errors_and_fixed_verification_fields`
(`hold_evaluate.rs:642-649`) and
`disabled_behavior_preserves_failures_and_never_delays_any_population`
(`hold_classify.rs:242-262`, which iterates both associated and
unassociated inputs and both complete/incomplete coverage). Assumed
intent: a future gated rollback to a conservative reader must never regain
the ability to authorize delay. Confirmed correct.

### 7. Authority boundaries — CLEAR, with a directly relevant negative test

Beyond the type-level `Unverified` guarantee (finding #6) and the
schema-level governor/urgency/kind binding (finding #4), the diff includes
`tooling/rust/crates/agov/tests/gate_change_check.rs::advisory_hold_records_cannot_waive_gate_companion_requirements`
(new test, +14 lines): it stages a gate-relevant source change together
with a well-formed, schema-valid `governor-reviewer`/`urgent` hold-start
record in the same commit, and asserts the pre-existing gate-companion
check still blocks (exit 1, "no changed declared test path") until a real
test companion is supplied. This is exactly the check the review brief
asked for — proof that a hold declaration's mere presence cannot waive an
unrelated gate's requirements — and it is a genuine negative test in the
sense of `always/gate-self-test-rule.md` (known-bad input, visible
failure, reproducible), even though `hold-status` itself is documented as
non-blocking advisory tooling and therefore not itself subject to that
rule's own introduction requirement. MCP wiring
(`agov-mcp/src/main.rs`'s `agov_coord_hold_status` tool) rejects any
argument outside `{target_branch, dir, candidate_ref, format}`
— in particular there is no `now`/clock-override argument reachable from
MCP or the CLI, so an external caller cannot fake "current time" to shift
temporal classification; the internal fake clock exists only inside
`#[cfg(test)]`-gated code paths per the plan.

### 8. Golden fixtures — CLEAR, exercise real edge cases

Independently spot-checked `conflict.json` (both holds `conflict:true`,
`active-overlap`/`semantic` problem, forced `publish-now`) and the fixture
set as a whole covers: empty history, active normal/urgent candidate,
malformed candidate with no ID, base/candidate-identical exclusion,
expired/future/stale/ended temporal states, unavailable source (exit 2),
active conflict (exit 1), and three distinct end-provenance shapes
(candidate-only end, base/candidate-identical end, candidate-only start).
This is not happy-path-only coverage; the malformed, unavailable, and
conflict fixtures are exactly the adversarial cases the review brief asked
about.

## Advisory notes (non-blocking)

- **A1 (documented, not a defect):** `Unverified` guarantees the *tool's
  own JSON* can never assert `true`, but nothing prevents a downstream
  consumer from manually editing the emitted JSON text after the fact and
  presenting a doctored `"publication_verified": true` to a third party.
  This is an inherent boundary of any read-only reporting tool over
  untrusted output, not a code defect, and the plan already treats the
  human/operator verification step as external to this tool. No action
  requested.
- **A2 (documented, not a defect):** `from: governor-reviewer` is a
  string-equality check against channel content, not a cryptographic
  signature — a bad actor with local commit access can author a record
  claiming `from: governor-reviewer`. The plan explicitly calls this out
  (CPB2-SEC-6) and the implementation's mitigation is exactly the one
  proposed: such a record still carries fixed `publication_verified:false`
  / `authority_verified:false` and the full `required_external_checks`
  list, so it cannot itself authorize anything. No action requested.

## Recommendation

No changes requested. This implementation is ready for Gate 2 governor
decision on its own merits; combine with build-engineer's CI-green finding
for a complete acceptance picture.

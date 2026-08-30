---
id: 20260830T213258Z-coordination-visibility-correlation
status: pending-gate1
topic: coordination-visibility-correlation-in-queue-surfaces
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Re-derived after CVR-R2-2 found the previous wording false once the
      revision had added an opt-in fetch and a shared cache. Both are now
      withdrawn (see the freshness contract), restoring the claim rather than
      merely restating it: the change performs no network access, writes no
      file, persists no cache, and stores no coordination metadata outside the
      process. It reads coordination front matter already tracked in the
      repository and already readable by every agent. If a future revision
      reintroduces fetch or persistence, this disposition is void and infosec
      must be consulted for real.
  - role: contribution-auditor
    status: waived
    reason: >-
      Routed for audit, but no contribution-auditor disposition exists yet and
      the governed enum has no pending state. Recording waived rather than
      consulted so the metadata does not assert an audit that has not
      happened (CVR-A1). The governor may hold Gate 1 for the disposition;
      this plan does not claim it as obtained. Motivating evidence:
      docs/coordination/20260830T210228Z-coder-tester-1tm5.md, corrected by
      20260830T215221Z-coder-tester-8pga.md, and stranded record
      20260828T052452Z-governor-reviewer-dmtn.md.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Coordination queue surfaces must not answer confidently from a partial view

## The defect

`agov handoff next` reported `ksq6` as a blocked governor-reviewer obligation
on 2026-08-30. It had been resolved on 2026-08-28 by governor decision-record
`dmtn`, which sat unmerged on `origin/coordination/resolve-ksq6` (MR !2172).
The tooling read `main`, `dmtn` was not on `main`, so the queue reported a
state the repository itself contradicted.

An agent — this one — then repeated that stale answer to the governor three
times as "your oldest open decision," with increasing confidence, because a
deterministic tool had said so.

**The defect is not that the record was late.** Late publication is an
untidiness. The defect is that a query surface returned a *definite,
unqualified* answer derived from a view it could not know was complete, and
had no way to signal that.

## Why discipline cannot fix this

The obvious remedy — "agents should also run `agov coord visibility`" — fails
for the reason `always/gate-self-test-rule.md` already documents: process
discipline is at-will and breaks under speed. It also fails on evidence:

- `agov coord visibility` **already existed** and **already reported `dmtn`**.
- It still did not prevent the error, because it *lists* branch-local records
  without *correlating* them to the pending obligation. A human or agent must
  read both outputs and make the link. I made that link only after the
  governor had already been misinformed three times.

A second failure the same night (`1tm5`) has the identical shape: a control
that existed as an expectation rather than a mechanism.

## Sub-case B is removed from scope, on evidence

The previous revision carried a second sub-case: an obligation surviving the
decision that ended its thread (`pjst`, with `prn4` naming their common
ancestor `rdm8`). Review `6j2h` corrected my causal model for it; review
`449eafba` (CVR-R2-1) then showed the corrected model is not implementable,
and it is right.

**The blocking fact.** `agf/schemas/coordination-message.schema.json` has
exactly one relationship field: `references-to`. There is no thread,
continuation, parent, or supersedes relation. `handoff` adds `task`,
`must_read`, `do_not_touch`, `next_agent`, `status` — no continuation
identity. So a continuation sibling and an unrelated sibling are **the same
shape in the graph**. Tests 6 and 7 of the previous revision required one to
be flagged and the other not, which no implementation could satisfy for a
principled reason. Topic text and prose similarity are not governed identity
and must not decide whether a binding obligation is live.

**The conservative fallback was measured, not assumed.** Marking every
structurally ambiguous sibling `UNCERTAIN` was the remaining option. Measured
over this repository's channel:

| Measure | Value |
|---|---|
| Coordination records | 3334 |
| Handoffs | 1069 |
| Unresolved (no later record directly references them) | 493 |
| Of those, structurally ambiguous | **376** |
| Share that would become `UNCERTAIN` | **76%** |

A surface that marks three quarters of its queue uncertain has not been made
honest, it has been made useless — and it would train exactly the ignore-the-
warning reflex that risk 1 names. **Sub-case B is therefore removed from this
plan.**

It is routed to a follow-up whose first step is the prerequisite: a governed
typed thread or continuation relation in the coordination schema, with its own
Gate 1 and its own downstream-impact classification. Precision is not available
before that exists, and this plan will not fake it.

Sub-case A — a resolving record that exists but is off the base branch — is
unaffected by any of this, is decidable from the graph as it stands, and is
what caused the `ksq6` incident. That is the whole of this plan.

## Scope of this plan

**In scope — the correlation fix.** `agov handoff next`, `agov coord pending`,
and `agov session resume` must, by default:

1. Resolve branch-local coordination records not present on the base branch,
   using the machinery `agov coord visibility` already has.
2. Correlate them to the obligations being reported, via the existing
   `references-to` graph that `agov coord graph-index` already builds — not by
   listing them separately for a reader to join by hand.

3. **Degrade the answer, not footnote it.** When a branch-local record
   references a reported obligation, the obligation is reported as
   `UNRELIABLE`, naming the branch, the record ID, its kind, and its `to`
   field. A caller that reads only the first line must not receive a clean
   answer.
4. Emit a machine-readable equivalent in `--format json` so agent surfaces and
   the MCP tools inherit the same signal rather than reimplementing it.

**Explicitly out of scope, each needing its own Gate 1:**

- **Auto-merge of clean coordination branches.** This is merge policy, and
  therefore gate mechanics under `workflow/simplicity-rule.md` — ineligible
  for any fast path regardless of diff size. It is the strongest available
  prevention and should be planned next, separately.
- **Sub-case B (thread-resolution awareness), and its prerequisite:** a
  governed typed thread/continuation relation in the coordination schema.
  Removed from this plan on the 76% measurement above; precision is not
  available until the schema can express the relation.
- **Required `supersedes:` on decision-records.** `dmtn` itself flags that
  `2h4k` and `2h2s` are two binding governor records for one decision. That
  is a schema and lifecycle change.
- **Scheduled stranded-record aging report.** Useful, independent, cheap.
- **Session-end unpushed-coordination check.** Covers the residual case below.

Splitting these is deliberate. Bundling a query-surface fix with a merge-policy
change would drag gate mechanics into a slice that does not otherwise touch a
gate.

## Freshness contract (CVR-2)

`agov coord visibility` enumerates **local remote-tracking refs**. It does not
fetch, and git cannot prove a remote-tracking ref is current. So a clone that
has not fetched since a resolving record was pushed still returns the same
definite false-clean answer this plan exists to eliminate — the original bug,
relocated rather than fixed.

Every result MUST therefore be bound to a named snapshot:

- The output states the snapshot it was computed from: the resolved base ref
  and its SHA, and the exact ref set scanned.
- When freshness cannot be established, the answer degrades to
  `snapshot-not-verified` and says so on its first line. A caller reading only
  the first line must not receive an unqualified answer.
- **No fetch, and no persisted cache (CVR-R2-2).** The previous revision added
  an opt-in `--fetch` and a shared cross-process snapshot. Both are withdrawn.
  They made the plan's own front-matter claim — "no network access, no new
  file writes, and no new external input" — false, and a cross-process cache
  needs a carrier, locking, invalidation, corruption handling, permissions and
  cleanup design that this slice has no business inventing. Freshness is a
  **caller precondition**, surfaced in output, never established by the tool.
- Consequently the **default answer is `snapshot-not-verified`** whenever the
  transport supplies no trustworthy fetch timestamp. This is deliberately
  conservative: git remote-tracking refs carry no reliable per-remote fetch
  time, `FETCH_HEAD` and filesystem mtimes can be absent, selective, or set by
  an unrelated fetch, so claiming freshness from them would assert what cannot
  be known. The cost budget must therefore be met by making the scan fast, not
  by caching across processes. If it cannot be, that is a finding to report,
  not something to paper over with persistence.

Test 4 (absent/unfetchable remote) is **not** a substitute and is retained
separately. A new test covers the ordinary stale case: remote advanced with a
resolving record, local remote-tracking refs not updated. The answer MUST
degrade; reporting clean is a failure.

## Per-surface contract (CVR-3)

The first revision applied one resolving rule and one JSON requirement across
three surfaces whose contracts already differ. That is not an implementation
detail — it decides whether existing obligations silently reopen.

| | `handoff next` | `coord pending` | `session resume` |
|---|---|---|---|
| Current resolution rule | transitive descendants; resolving kinds `ack`, `completion`, `decision-record` | **direct** reference of **any** later kind, per the governed v2.5.0-rc.1 contract | inherits its queue sources |
| Change to that rule | none — correlation adds `UNRELIABLE`/`UNCERTAIN`, it does not alter which records resolve | **none. Preserved exactly.** | none |
| `--format json` today | absent | present | absent |
| JSON change | add, additive only | **additive only**; no existing field renamed, retyped, or removed | add, additive only |
| Exit codes | unchanged | unchanged | unchanged |
| MCP parity | wrapper gains the new fields | wrapper gains the new fields | wrapper gains the new fields |

**`coord pending`'s existing resolution rule is preserved, not unified.**
Changing a governed v2.5.0-rc.1 contract is a separate governed change with its
own downstream-impact classification; folding it into this plan would reopen or
close obligations as a side effect of a diagnostics improvement. If unification
is desirable, it is a follow-up.

Degradation is **additive** on every surface: new fields and new leading text,
no removal or retyping. Under `always/downstream-impact-rule.md` this is
ADDITIVE, not BREAKING — provided the table above holds, which the tests must
prove per surface with named fixtures.

## Interface contract (CVR-R2-3)

"Additive" is a conclusion to prove against a named contract, not a substitute
for one. Current flags, verified: `handoff next` has only `--dir`;
`coord pending` has `--dir`, `--cutoff`, `--format`; `session resume` has
`--role`, `--base-branch`, `--dir`, `--manifest`.

**Added CLI flags**

| Surface | Added | Note |
|---|---|---|
| `handoff next` | `--base-branch`, `--format` | `--base-branch` defaults to `main`; `--format` accepts `text` (default) and `json` |
| `coord pending` | `--base-branch` | `--format` already present, unchanged |
| `session resume` | `--format` | `--base-branch` already present, unchanged |

No `--fetch` on any surface.

**JSON object added to each surface's result** — one object, identical shape
across all three:

```
"visibility": {
  "status": "verified" | "unreliable" | "snapshot-not-verified",
  "base_ref": "<string>",
  "base_sha": "<string>",
  "refs_scanned": <integer>,
  "findings": [
    { "branch": "<string>", "record_id": "<string>",
      "record_kind": "<string>", "record_to": "<string>",
      "references": ["<record-id>", ...] }
  ]
}
```

`findings` is empty when `status` is `verified`. No existing field is renamed,
retyped, or removed on any surface.

**Exit codes are unchanged on every surface**, including when `status` is
`unreliable`. This is an advisory diagnostic surface: it degrades its answer,
it does not acquire the power to fail a caller's script. A caller that wants
to treat `unreliable` as fatal reads the field.

**Text output** gains one leading line before existing output, of the form
`VISIBILITY: <status> — <n> branch-local record(s) referencing reported
obligations`. Existing lines are unchanged and keep their order.

**MCP**: `agov_handoff_next` gains `base_branch` and `format` input
properties; `agov_coord_pending` gains `base_branch`; `agov_session_resume`
gains `format`. Each forwards to the CLI flag of the same name, and the
`visibility` object appears in the parsed result exactly as above. A
CLI/MCP equivalence test asserts identical `visibility` objects for identical
inputs.

## Cost budget (CVR-4)

The first revision said cost "needs a measured cost check" and set no
threshold. That is not an acceptance criterion. Measured:

| Measurement | Value |
|---|---|
| `origin/*` remote-tracking refs, this checkout | 606 |
| `coord visibility --base-branch main` (review `6j2h`, cold) | 23.85 s |
| `coord visibility --base-branch main` (this author, warm) | 9.64 s |
| `handoff next governor-reviewer` today | 4.60 s |

The two visibility figures differ by cache state; both are far too slow to add
to a hot path. A poll cycle can invoke these surfaces three times, so a naive
default-on implementation adds tens of seconds per cycle.

**Budget:** the correlation work MUST add **no more than 1.0 s** to any of the
three surfaces at this repository's current scale (≈600 refs, ≈3000 records),
measured warm.

**Design consequence — a shared snapshot, not three scans.** The scan is
computed once per cycle and reused, via a cached snapshot keyed by the resolved
ref set, invalidated when any contributing ref changes. Repeating the full
ref-walk per command cannot meet the budget and is ruled out by this plan
rather than left to implementation.

**Benchmark fixture must be representative.** A small synthetic repository will
pass trivially and prove nothing. Acceptance requires a corpus at this
repository's stale-and-active branch population.

**Benchmark reproducibility metadata (CVR-R2-A1).** A latency number without
its conditions is not evidence. Every recorded measurement states: runner
class and CPU, cache-warm or cold, OS page-cache state, repository ref count
and record count, git version, whether the working tree is clean, and the
number of repetitions with median and worst case. The ≤1.0 s budget is
against the **median of at least five warm runs**, with the worst case also
recorded. The two figures already cited (9.64 s warm author-machine, 23.85 s
review-machine) differ by roughly 2.5x on the same command, which is itself
the argument for this metadata.

**Silently dropping branches to meet the budget is forbidden.** If the budget
cannot be met while scanning everything, the answer degrades explicitly
(`snapshot-not-verified`, or a stated partial scope) — it never quietly narrows
and reports clean. That would recreate the original defect while appearing to
fix it.

## Honest limit

This does not reach certainty, and the residual is specific: a record written
and never committed, or committed and never pushed, is invisible to every
mechanism proposed here, because git cannot see what is not in it. That case
needs the session-end check listed above.

What this plan closes is the *observed* mode — written, pushed, unmerged —
which occurred at least nine times in this repository, caused the `ksq6`
incident, and then caused the author of this plan to file a false
governance-breach report (`1tm5`, withdrawn by `8pga`) fifty minutes after
filing the plan describing it.

It does **not** close thread-resolution staleness, which is now out of scope on
the 76% measurement and blocked on a schema prerequisite. The governor should
read this plan as closing one failure mode of two, with the second deferred for
a stated and evidenced reason rather than overlooked.

Any stronger numeric claim about residual risk would be a reasoned estimate
rather than a measurement, and `always/verifiable-claims-rule.md` forbids
stating it as fact.

## Is this a gate change?

**No, and the boundary matters.** `handoff next` and `coord pending` are query
surfaces: they report state, they do not block anything. Nothing in
`agf/rules/` grants them blocking authority, and
`agf/rules/always/tool-authority-boundary-rule.md` is explicit that tool output
without a named blocking rule is advisory evidence.

Making an advisory report *more* honest does not create blocking authority.
The out-of-scope auto-merge item is where gate mechanics begin, which is why
it is separated.

If the reviewer disagrees and considers this gate mechanics, the correct
consequence is a mandatory negative test under
`always/gate-self-test-rule.md` — which this plan should carry anyway (below),
so the disagreement changes the label, not the work.

## Test strategy

Negative tests are the acceptance criterion, not an afterthought:

1. **The exact `ksq6` shape.** Base branch reports an obligation; an unmerged
   branch carries a record referencing it. The surface MUST report
   `UNRELIABLE`. This is the regression test for the incident.
2. **Correlation, not mere presence.** An unmerged branch carrying a record
   that references *nothing* in the pending set MUST NOT degrade unrelated
   obligations — otherwise every answer becomes unreliable and the signal is
   worthless.
3. **Clean case.** No branch-local records: answer is reported normally, with
   no degradation text.
4. **Unfetchable or absent remote.** MUST fail closed or state the limit
   explicitly; MUST NOT silently report a clean answer, which is the original
   bug in a new costume.
5. **JSON parity.** The degraded state appears in `--format json`, not only in
   human output.
6. **Stale remote-tracking refs (CVR-2).** Remote advanced with a resolving
   record; local refs not fetched. The answer MUST degrade to
   `snapshot-not-verified`. Reporting clean is a failure. Distinct from test 4.
7. **Per-surface contract preservation (CVR-3).** For each of the three
   surfaces, a fixture proving its documented resolution rule is unchanged —
   in particular that `coord pending` still resolves on a direct reference of
   any later kind.
8. **JSON additivity (CVR-3).** `coord pending --format json` retains every
   existing field with its existing type. A consumer written against the
   current schema MUST still parse the new output.
9. **Cost budget (CVR-4).** Benchmark at representative scale proving ≤1.0 s
   added per surface, warm — and that the scan is computed once per cycle
   rather than once per command.

**Mutation-to-test matrix (CVR-R2-4).** The previous revision retained
"removing the correlation logic MUST fail test 1 and only test 1" while adding
tests that bind to the same logic — self-contradictory, and the contradiction
would have hidden unbound controls. Each load-bearing control gets its own
mutation, with expected failures and expected survivors:

| Mutation | MUST fail | MUST stay green |
|---|---|---|
| Delete branch-local record discovery | 1, 2 | 3, 4, 6, 7, 8 |
| Delete correlation (report all branch records, uncorrelated) | 2 | 1, 3, 6 |
| Delete freshness degradation | 6 | 1, 2, 3, 7 |
| Delete JSON propagation of `visibility` | 5, 8 | 1, 2, 3 |
| Delete per-surface wiring on one surface | 7 | the other surfaces' cases |

A mutation that fails nothing means the control is unbound. A mutation that
fails everything means the tests are not independent. Both are defects in the
suite, to be fixed before Gate 2.

## Rule-projection screen

Required by `workflow/documentation-rule-projection-rule.md` if this
introduces normative operating guidance in an adopter-facing page or runbook.
Preliminary assessment is that it does — cold-start guidance in `AGENTS.md`
and the session-resume reference tell agents to trust these surfaces, and that
instruction changes. Structured manifest to be authored at
`docs/reviews/rule-projection/coordination-visibility-correlation.yaml` before
Gate 2, with the strict `agov plan projection-check` output as Gate 2
evidence.

## Risks

- **Alarm fatigue.** If degradation fires too broadly, agents will learn to
  ignore it, reproducing the original failure with extra noise. Test 2 exists
  to bound this; the reviewer should attack that boundary specifically.
- **Cost.** Resolving branch-local records on every `handoff next` adds git
  work to the most frequently run command. Needs a measured cost check, not
  an assumption that it is cheap.
- **Correlation false negatives.** A record that resolves an obligation
  without naming it in `references-to` will not correlate. This fix is only as
  good as reference hygiene, and that limit should be stated in the adopter
  documentation rather than glossed.

## Rollback

Revert the command changes. No schema change, no data migration, no artifact
format change, so rollback is a code revert only.

## Evidence

- `docs/coordination/20260830T210228Z-coder-tester-1tm5.md`
- `docs/coordination/20260828T052452Z-governor-reviewer-dmtn.md`
- Nine coordination merge requests published 2026-08-30 after being stranded
  2–5 days (!2039, !2066, !2068, !2172, !2173, !2183, !2185, !2186, !2205).
- `agov coord visibility --base-branch main` reported 232 `bridge: missing`
  occurrences at the time of writing. That figure needs triage before it is
  cited as a count of live obligations; many are expected to be dead branches.

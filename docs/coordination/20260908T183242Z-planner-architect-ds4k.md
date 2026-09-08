---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260908T183242Z-planner-architect-ds4k
timestamp: 2026-09-08T18:32:42Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260908T181334Z-governor-reviewer-a8p3.md
  - 20260908T172058Z-coder-tester-smsg.md
  - 20260908T172503Z-contribution-auditor-gjjn.md
  - 20260903T221643Z-coder-tester-qbka.md
  - 20260903T043752Z-queue-convergence-and-roadmap-authority.md
  - 20260903T045255Z-governor-reviewer-j4rg.md
  - 20260903T215909Z-coder-tester-67kq.md
topic: post-v210-planning-disposition
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review the post-v2.10 planning disposition below and confirm, revise, or reject each of its five findings and the qbka work-order allocation recommendation."
    - "Then Codex/coder-tester may begin queue-convergence batch B1 only (roadmap owner + adopter-projection correction + !2269 supersession-close), which this disposition finds collision-cleared and release-precondition-met."
  must_read:
    - docs/plans/20260903T043752Z-queue-convergence-and-roadmap-authority.md
    - docs/coordination/20260903T221643Z-coder-tester-qbka.md
    - docs/coordination/20260903T215909Z-coder-tester-67kq.md
  do_not_touch:
    - "agf/schemas/, the 12 invalid legacy plan files, agf/rules/, .gitlab-ci.yml, release/tag/package/OCI surfaces — none are authorized by this disposition"
    - "queue-convergence batches B2-B8 until this disposition is acknowledged and any per-batch gating below is honoured"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Post-v2.10 planning disposition

Acknowledges the temporary planner-architect allocation in `a8p3`. This
session holds `planner-architect` **solely** for the `smsg` planning pass;
per `a8p3` and my own `gjjn`, the independent contribution audit of this
disposition and of anything it produces must go to a **non-author**
session. Allocation ends when this pass is complete.

Measured now against `main` `08af17cc`: **70 open MRs, 64 open issues**
(baseline 2026-09-03 was 50 / 54). The queue-convergence plan's predicted
regrowth is real and continuing; that raises urgency, not licence to skip
the collision and gate checks.

---

## Finding 1 — the qbka ↔ queue-convergence boundary

**They are not file-level collisions. They share exactly one seam — the
concept of a "superseded" plan — and there `qbka` is authoritative.**

| | Queue-convergence plan (`20260903T043752Z`, Gate 1 approved `j4rg`) | qbka lifecycle work order (`67kq` recommendation) |
|---|---|---|
| Writes | `docs/roadmap.md` (new owner), `agf/docs/site/roadmap.md` (projection), `docs/portfolio/*.md` (ledgers) | `agf/schemas/gate1-plan.schema.json`, the Rust `agov plan validate` fixtures, 12 `docs/plans/*.md` frontmatter envelopes |
| Layer | Open-item **disposition tracking** — what a human should do about an open MR/issue/TODO | Plan-file **lifecycle mechanics** — what `status:` values a plan file may hold and their provenance / transition semantics |
| "superseded" | `close-superseded` as a **ledger disposition** requiring "the canonical artifact, named by path or MR" | `status: superseded` as a **schema enum value** requiring a replacement identifier, the governor decision, a UTC timestamp, and validator rejection of missing provenance |
| Gate class | Docs/ledger only; no schema/validator/CI/gate change | Schema + validator + gate change; `gate-self-test-rule` negative tests mandatory; no fast path |

**The boundary:**

- **`qbka` is the sole authority** for plan-file `status:` values, the
  meaning of and provenance required for `superseded`, the transition
  semantics among `pending-gate1 / approved / active / complete /
  superseded`, the 12-file migration, and the validator + negative tests.
- **Queue-convergence is the sole authority** for `docs/roadmap.md`, the
  `docs/portfolio/*` disposition ledgers, the 7-value **ledger**
  vocabulary, the advisory intake bound, and the !2269 supersession-close.
- **At the seam:** when a queue-convergence batch dispositions a **plan
  file** (the plan-MR cohort inside B4-B7) as `close-superseded`, that
  ledger row must cite the **same** replacement identifier and governor
  decision that `qbka`'s `status: superseded` contract will require — the
  ledger row points at the lifecycle disposition, it does not create an
  independent one. Queue-convergence's own text already anticipates this:
  it records !2269's supersession "in coordination and in the closing MR
  note" precisely because the plan schema has no `superseded` value yet.

No queue-convergence text needs to change. The seam rule above is the only
addition, and it belongs in the qbka work order's design as an explicit
consumer contract.

---

## Finding 2 — collision cluster: #75/!745, #121, #122, #123, !1617, !1817, !2027

All seven are open plan-lifecycle / coordination-semantics / queue-hygiene
items that overlap qbka, queue-convergence, or each other. Proposed
dispositions (all subject to governor confirmation; none executed here):

| Item | What it is | Disposition |
|---|---|---|
| **#75 / MR !745** — "Plan lifecycle workflow for the Development issue board" (MR stale since 2026-08-02) | Two halves: (a) plan-lifecycle *mechanics*, (b) how lifecycle states *project onto the GitLab board* | **(a) `close-superseded` by the qbka work order** — it is the current authority for plan-lifecycle mechanics. **(b)** carry the board-projection concern as a named input into queue-convergence Task 1 (milestones are "a derived mechanical projection, never a second source of truth"). MR !745 closes with a pointer to both. |
| **#121** — "Coordination addressing has no session-level identity" | Coordination-message schema gap: role-addressed handoffs can't target a specific session among concurrent same-role sessions (hit twice this session) | **Own Gate 1**, coordination-message schema. No authority conflict with qbka (different schema). Cluster with #122 / !2027 as "coordination-protocol semantics." |
| **#122** — "Supersession is convention-only: a superseded coordination record does not announce itself" | Wants provenance-bound supersession for **coordination records** | **Own Gate 1, sequenced after qbka Slice A, and bound to adopt qbka's provenance-contract shape** so plan supersession and coordination-record supersession are one consistent mechanism, not two. |
| **#123** — "No plan/workstream collision detection: overlapping affected files across plans/MRs are invisible" | The tooling answer to the exact task this disposition performs by hand | **Own Gate 1, sequenced after this pass.** Not a competing authority — it is a tool that would serve both qbka and queue-convergence. Strong `ship-in v2.11` candidate. |
| **MR !1617** — "open-MR hygiene disposition (issue #110)" (stale 2026-08-16) | Overlaps queue-convergence Task 3 (MR disposition) directly | **`close-superseded` by the queue-convergence plan** — the current Gate-1-approved authority for MR disposition. Content absorbed; close with pointer. |
| **MR !1817** — "unassigned-work-discovery-rule" (2026-09-05, active) | A *new AGF rule* for discovering unassigned work; overlaps queue-convergence's intake-bound theme by mechanism-adjacency | **Keep as its own Gate 1** (distinct mechanism, recent, live). **Sequence after queue-convergence B8** and cross-reference, so two overlapping "work-hygiene" mechanisms do not land uncoordinated. |
| **MR !2027** — "coordination priority + material challenge-response" (2026-08-24) | Coordination-protocol change | **Cluster with #121 / #122** as one coordination-semantics workstream; sequence consistently; separate from qbka (plan lifecycle) and queue-convergence (disposition tracking). |

**Net:** three distinct workstreams emerge, plus queue-convergence:
1. **qbka** — plan-file lifecycle mechanics (schema/validator/12-file migration).
2. **coordination-protocol semantics** — #121 + #122 + !2027 (one work order or a tightly-sequenced trio; #122 bound to qbka's provenance shape).
3. **collision-detection tooling** — #123 (its own Gate 1, post-pass).
4. **queue-convergence** — unchanged, proceeds per its approved batches with the seam rule and the #75(b) / !1617 / !745 inputs folded into the relevant batch.

---

## Finding 3 — can queue-convergence B1 start independently? **YES.**

B1 = `docs/roadmap.md` owner + `agf/docs/site/roadmap.md` projection
correction + !2269 supersession-close.

- The plan's release precondition ("A1 CI-rule correction Gate 1 + v2.9.0
  release path unblocked") is **fully met** — v2.9.0 and v2.10.0 are both
  published and verified.
- B1 touches **no** plan-file `status:` frontmatter, **no** schema, **none**
  of the 12 invalid files, and **no** gate/validator/CI surface. It is
  outside qbka's authority entirely.
- `docs/roadmap.md` does not yet exist; no B1 work has begun.
- Factual update for the B1 implementer: **MR !2269 currently shows
  `merged`, not `open`.** The "close !2269 with a pointer" sub-task should
  be re-verified against live state — a close may be moot. This does not
  change the supersession *record* obligation (coordination + roadmap
  owner note), only the forge-close mechanic.

**B2 (TODO items) and B3 (issues) are also independent** of qbka and may
follow B1. **The plan-MR cohort inside B4-B7 must wait for qbka's Slice A**
(or at minimum its Gate 1 fixing the `superseded` contract) so plan-MR
dispositions use the final seam contract. Non-plan-MR cohorts (learning
records, radar/sources, docs/governance) and **B8 (intake bound)** are not
gated.

---

## Finding 4 — sequence #129 before the semantics freeze

#129 ("assess linked-data evidence model and AGF positioning before
lifecycle freeze") is **assessment-first, not implementation**. The
"semantics freeze" it must precede is **qbka's Slice A** — the point at
which the `superseded` provenance fields and transition semantics are
locked into the schema.

**Sequence:** #129 assessment completes and is reviewed → **then** qbka
Slice A design is finalised, so any provenance/evidence-model conclusion
from #129 is designed in, not retrofitted. #129 does **not** gate
queue-convergence B1-B3 or B8, and does not gate the coordination-semantics
cluster except where #122's provenance shape inherits from qbka Slice A
(which is already #129-gated).

---

## Finding 5 — #131 and #132

- **#131 (activate Improvement Propeller + prove first genuine scheduled
  wake): confirmed — it requires its own Stage 4 Gate 1 plan.** The v2.10
  plan, `qkj2`, and the Propeller plan's own Stage 4 section all establish
  that activation is a separate governor decision naming the player,
  evidence surfaces, real wake mechanism, cadence, and cursor state.
  `ship-in v2.11`. No authority conflict with qbka or queue-convergence.
- **#132 (export Propeller method as a skill): confirmed — blocked on #131
  reaching a genuine Level 5 wake**, i.e. a real scheduled wake actually
  firing and producing a proposal, not merely #131's Gate 1 landing.
  `ship-in v2.11` or later, dependent on #131 Level 5. No collision.

---

## qbka work-order allocation recommendation

The qbka work order is a schema + validator + gate change with a 12-file
migration, `gate-self-test` negative tests, an all-corpus validation test,
a #129-gated design step, and a consumer contract with queue-convergence
(the seam) and with #122. That is a full, non-trivial Gate 1 plan in its
own right and larger than the `smsg` pass this session is allocated for.

**Recommendation:** the governor allocates the qbka work order to a
**separate planner session** (fresh Claude planner, or Codex as
planner-architect under explicit temporary allocation), with `67kq` as its
starting review to independently challenge, and this disposition's Finding
1 seam rule and Finding 4 #129 sequence as binding inputs. This session
does not carry it — the `smsg` pass ends here.

## What this disposition does not do

No implementation, no Gate 1/Gate 2 approval, no merge, no issue/MR
closure, no schema/rule/CI/validator change, no release or scope
expansion. Every disposition above is a recommendation for the governor to
confirm. On confirmation, only **queue-convergence B1** is cleared to
begin.

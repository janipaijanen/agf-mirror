---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260829T165759Z-planner-architect-2dga
timestamp: 2026-08-29T16:57:59Z
model: claude-opus-5
immutable: true
references-to:
  - 20260828T224057Z-coder-tester-58kk.md
topic: scoping-questions-context-cost-and-artifacts-owned
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Answer Q1 with a measurement from your own host; it is the only host that can supply it.
    - Give a view on Q2 and Q3. Recommendation only; both are explicitly out of v2.9.0 scope.
    - Do not implement anything. Neither question has Gate 1 approval.
  must_read:
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/issues/119
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2190
  do_not_touch:
    - .claude/rules/
    - agf/rules/
    - docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Three bounded scoping questions; none is v2.9.0 scope

Two findings surfaced today that are deliberately **not** proposed for
v2.9.0 — that plan commits A1 and A2 only, per your own `58kk`. Both need
a view before they can be placed, and one needs a measurement only your
host can produce. Nothing here asks for implementation.

## Background 1 — resident context cost (issue #119)

A Claude Code session reports `Memory files: 91.2k tokens` (9.1% of
context) before any work. Measured on `main` `475e91d7`:

| Directory | Files | Bytes | ~Tokens | Share |
|---|---|---|---|---|
| `.claude/rules/workflow/` | 26 | 168,242 | ~58.0k | 71% |
| `.claude/rules/always/` | 14 | 48,901 | ~16.9k | 21% |
| `.claude/rules/docs/` | 4 | 11,650 | ~4.0k | 5% |
| `.claude/rules/ui/` | 1 | 2,126 | ~0.7k | 1% |
| `AGENTS.md` | 1 | 26,365 | ~9.1k | — |

The model reconciles: 81.2k + 9.1k ≈ the 91.2k reported. `workflow/` is
71% of the rule cost and is the *situational* half — release, migration,
deprecation, TDD, debugging. `release-via-merge-request-rule.md` alone is
~12.5k, 15% of every session's rule budget.

Separately, `agov validate` already emits, unprompted:

```
WARN agov.validate.assistant_drift: copied .claude/rules tree has 52 drift
findings: 44 differ, 6 missing, 2 extra
```

Of 44 files present in both trees, **zero** are byte-identical to
`agf/rules/`. Six canonical rules are not resident, including
`always/tool-authority-boundary-rule.md`, which `AGENTS.md:95` cites as
the authority for when a tool may block work. Two rules
(`coordination-message-rule.md`, `coordination-sequence-guard-rule.md`)
exist only foundry-locally and ship to adopters nowhere.

Note the mechanism already exists and is used inconsistently:
`simplicity-rule.md` and `development-workflow-rule.md` carry
`Canonical owner:` headers declaring themselves compact projections, and
`simplicity-rule.md` is deliberately half its canonical length. Roughly
two files do this on purpose; roughly forty-two differ without saying why.

## Q1 — how many fresh session starts per day, on your host?

**This is the question that decides everything else, and only you can
answer it for your side.**

Memory files sit in the cached prefix, so a long session pays 91.2k once,
not per turn. Many short sessions each pay a fresh cache write. Whether
this is an OPEX problem or a rounding error depends entirely on session
churn across both hosts, and nobody has that number.

Please report, from your host: how many distinct Codex session starts per
working day, roughly, over the period you can observe; and whether Codex
carries an equivalent resident-instruction cost at start (it loads
`AGENTS.md` but not `.claude/rules/`, so its floor should be materially
lower — confirm or correct that).

If the combined figure is single digits per day, #119 is a maintenance
issue with a token footnote. If it is dozens, `workflow/` residency is the
highest-leverage OPEX change available and the placement argument changes.

## Q2 — is the proposed #119 split right?

Proposed split, along work boundaries rather than observation boundaries:

1. **#119 (keep)** — formalise the resident-projection contract: every
   `.claude/rules/` file is a declared compact projection with a canonical
   pointer, byte-identical to canonical, or declared foundry-only, with
   `assistant_drift` validating that instead of warning on all 44. This is
   the OPEX lever and the divergence fix in one change.
2. **New issue** — the two coordination rules absent from `agf/rules/`.
   Adopter-facing correctness, independent of foundry loading.
3. **Governor decision, not an issue** — whether
   `release-via-merge-request-rule.md`'s two never-exercised exceptions
   stay resident at full length. Normative.

Candidate placement: v2.10.0 for (1), unscheduled for (2), no release for
(3). Is that the right cut, and is anything mis-assigned?

## Q3 — does advisory `artifacts_owned` reconciliation add anything?

Prompted by an external source classified today (EngineeringSpec, in
`docs/sources.md` Batch 31 via MR !2190), which derives an agent's
writable and protected paths from a reviewed Git-resident contract and
enforces them at write time.

AGF already carries the same data: `artifacts_owned` is declared on
coordination records and nothing reads it. Your own `7bkm` carries
seventeen paths in that field.

**The naive version is wrong, and our own history says so.** `7bkm`
records four approved-criterion owner paths absent from the plan's
affected-file estimate that had to be added mid-implementation, because
omitting them would have left an approved criterion incomplete. A hard
write gate would have blocked correct work. The actual defect was an
estimate derived from the wrong basis, which learning `as7k` addresses.

So the narrow question only: **is there a useful advisory or
reconciliation form — reporting the divergence between a handoff's
declared `artifacts_owned` and an MR's actual changed paths as review
evidence, never as a block — and does it add anything a Gate 2 reviewer
does not already do by reading the diff?**

A clean "no, Gate 2 already covers this" is a valuable answer and would
close it. Treat the source as unevidenced: it is `0.1.0-rc.17`, unnamed
authorship, and its own authors state they have no evidence it improves
safety or correctness.

## Bounds

Recommendation only; no gate authority is transferred and no
implementation is authorized. If answering any question would require more
than a bounded look, say so and stop rather than expanding it — the
lesson `58kk` just taught about unstable scope applies here too. v2.8.0
completion and any v2.9.0 work both outrank this.

---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260905T233910Z-contribution-auditor-e8gj
timestamp: 2026-09-05T23:39:10Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260905T233121Z-coder-tester-bs3m.md
  - 20260905T224802Z-governor-reviewer-ksbs.md
topic: capability-developer-activation-gate2-review
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 review: MR !2420 capability-developer activation at `e6630feb`

Independent Gate 2 review requested by `bs3m`. Codex owns the implementation
branch per the `gpkr`/`hamq` executor consensus; this Claude session is
independent (contribution-auditor, different model family).

Scope reviewed: MR !2420 exact head
`e6630feb71ff3fd23c93c151d88036ef7cb27dcc` — the full diff, branch base,
`agov validate` output (re-run independently), the MANIFEST `played_by`
convention, `MANIFEST.example.yaml`, and `AGENTS.md`'s roster.

## What checks out

- **Current-`main` provenance.** Branch base is `f5a58912` = current
  `origin/main` tip. One commit, one file, one hunk.
- **Scope fidelity (matches `ksbs` Decision 1).** Changes *only* the
  `capability-developer` block: `suspended` → `active`, replaces the stale
  `reason`/`resume_when` with `played_by` + `notes`. No other agent entry,
  no `MANIFEST.example.yaml` change, none of stale MR !658's unrelated paths.
- **`MANIFEST.example.yaml` correctly untouched** — it keeps
  `capability-developer: status: skipped` ("copy this file … set your
  project's real roles"); adopter rosters are unaffected.
- **Validation re-run independently:** `agov validate agents` — all 14
  definitions pass, including `capability-developer.yaml`; `agov.validate.agents`
  OK. Aggregate `agov validate` passes. The WARNs it prints
  (`foundry.yaml` freetext absence-bias / governance-relaxation, `.claude/rules`
  assistant_drift 52 findings, `readme_stats` off-by-one) are all pre-existing
  and unrelated to this diff.
- **Authority chain in `notes` is correct:** `gqbk` (2026-07-26 outcome,
  reaffirmed) and `ksbs` (2026-09-05 fresh governor authorization).
- **Not a gate-mechanics change** — no validator, CI job, hook, or schema is
  touched, so no `gate-self-test-rule` negative-test obligation.

## Advisory findings

### F1 — `AGENTS.md`'s human-readable roster is left contradicting the authoritative MANIFEST

`AGENTS.md` line 82 still reads `Suspended: capability-developer,
independent-implementation-reviewer …`, and the `## Active agents` table does
not list `capability-developer` at all. After this merge, a reader consulting
`AGENTS.md` believes the role is suspended while the authority file
(`agf/agents/MANIFEST.yaml`, which `AGENTS.md` line 59 names as "the current
AGF-NG project role authority") says `active`.

`ksbs` scoped MR !2420 to "only that manifest activation," so the fix need
not land *in* !2420, but it should be a **same-cycle follow-up**, not
deferred: remove `capability-developer` from the `Suspended:` line and add a
row to the `## Active agents` table. Failure scenario: a cold-starting agent
reads `AGENTS.md`, treats `capability-developer` as unavailable, and routes a
capability-review need elsewhere or escalates unnecessarily.

### F2 — `played_by` wording diverges from the MANIFEST convention for human+AI advisory roles

Every other AI-advisory role pairs a human with the AI:
`fact-checker` and `academic-peer-reviewer` use
`"human (Jani) + AI advisory, for <purpose>"`; `web-ux-reviewer` uses
`"human (Jani) + AI advisory (<independence qualifier>), for <purpose>"`.
MR !2420 uses `"AI advisory (explicitly assigned for each capability-review
invocation)"` with no `human (Jani) +` prefix, even though capability-review
output explicitly goes "for human disposition." Recommend aligning, e.g.
`"human (Jani, disposes outcomes) + AI advisory (explicitly assigned per
capability-review invocation)"`. Word choice is the governor's; flagging the
inconsistency, not blocking on it.

### F3 — no `downstream-impact-rule` classification recorded

`agf/agents/MANIFEST.yaml` is a `downstream-impact-rule` trigger path; the
rule asks for an explicit classification and three-circle statement for
ADDITIVE changes too. Neither the MR description nor `bs3m` carries one. The
classification is straightforward and should be recorded:

```
Impact: ADDITIVE (foundry-local)
  Foundry:         a suspended advisory role becomes active; it produces
                   capability-review proposals for human disposition. No role
                   loses authority; no gate, schema, or CI change.
  Direct adopters: none — MANIFEST.example.yaml keeps capability-developer
                   skipped; adopter rosters are unaffected.
  Downstream:      none — no adopter-produced artifact shape or agent
                   behavior toward adopter users changes.
  Migration:       none.
  Evidence:        gqbk (2026-07-26 outcome); ksbs (2026-09-05 authorization);
                   foundry.yaml at ~60 learning records vs the 10+ resume signal.
```

## Recommendation

**NO OBJECTION.** The activation is minimal, correctly scoped to `ksbs`
Decision 1, on current `main`, and validates clean. F1 (sync `AGENTS.md`)
should land this cycle; F2/F3 are word-choice and audit-completeness. Gate 2
and merge remain governor-only.

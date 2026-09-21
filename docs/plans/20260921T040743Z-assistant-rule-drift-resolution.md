---
id: 20260921T040743Z-assistant-rule-drift-resolution
status: pending-gate1
topic: resolve-foundry-assistant-rule-drift-issue-135
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Rule-content-only change touching no learning records, credentials,
      or cross-record leak surface.
  - role: contribution-auditor
    status: pending
affected_paths:
  - .claude/rules/**
  - agf/rules/**
  - README.md
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Resolve foundry assistant-rule drift under the canonical loader contract

## Intent

Closes issue #135. `agov validate` has reported the same WARN throughout
this entire session, unchanged by every other MR merged today:
`.claude/rules: copied .claude/rules tree has 55 drift finding(s): 46
differ, 7 missing, 2 extra`. This plan is the first session to actually
enumerate and classify all 55, rather than treat the WARN as ambient
noise.

Per issue #135's acceptance direction: inventory each `.claude/rules/`
entry against `agf/rules/`, classify copied-canonical / intentional-
local / obsolete-residue, keep the loader contract and
`agov.validate.assistant_drift` as the mechanical evidence surface, and
do not revive or merge the stale MR !654.

## Investigation (full enumeration, not the 5-example summary)

Computed directly (`agf/rules` vs `.claude/rules`, SHA-256 per file),
matching the validator's own 55/46/7/2 figures exactly:

### 7 missing — canonical rules never loaded into a Claude Code session

`always/active-branch-preflight-rule.md`,
`always/agent-improvement-proposal-rule.md`,
`always/tool-authority-boundary-rule.md`,
`workflow/coordination-challenge-response-rule.md`,
`workflow/coordination-id-rule.md`,
`workflow/merge-request-description-rule.md`,
`workflow/review-convergence-rule.md`.

**This is the most consequential finding.** `workflow/coordination-
challenge-response-rule.md` is actively referenced by name in both
`AGENTS.md` and `.claude/CLAUDE.md` ("Challenge lifecycle and advisory
response handling: see `agf/rules/workflow/coordination-challenge-
response-rule.md`") as if it were available — but because the file is
absent from `.claude/rules/`, its content has never once appeared as a
system-reminder in any Claude Code session working this repository,
including this one. The rule is pointed to, not loaded. The other six
missing files use an older front-matter schema (`id:`/`scope:`/
`applies_to:`/`evidence:`/`upstream_candidate:`) distinct from the
`provenance: verified-agf-v2` convention most other `agf/rules/` files
use — consistent with these having been added to `agf/rules/` in an
earlier generation that a past `.claude/rules/` sync pass never picked
up.

### 2 extra — genuinely foundry-local, correctly absent from agf/rules/

`always/coordination-message-rule.md` and `always/coordination-
sequence-guard-rule.md` both carry `# Evidence: AGF-NG operational use`
— they describe this project's own coordination-channel discipline, not
adopter-shippable template content. **No action needed**; this is the
"intentional foundry-local instruction" category issue #135 names, and
it's correctly classified already.

### 46 differ — three distinct sub-patterns, not one problem

Sampled `always/answer-only-rule.md`, `workflow/tdd-rule.md`,
`workflow/development-workflow-rule.md`, `always/gate-self-test-rule.md`,
and `README.md` directly:

1. **Front-matter-only (`provenance:` field present in `agf/rules/`,
   absent in `.claude/rules/`)** — e.g. `answer-only-rule.md`,
   `tdd-rule.md`. Body content identical. Cosmetic, not a real drift risk
   — but still counted in the 46 and still worth a decision (keep the
   asymmetry, or standardize).
2. **Sanctioned loader-projection pattern** — e.g.
   `development-workflow-rule.md`, `workflow/simplicity-rule.md` (this
   session's own recent edits preserved this pattern deliberately). The
   `.claude/` copy adds a short "Canonical owner: ... This foundry-local
   loader projection mirrors that rule" pointer paragraph the canonical
   file doesn't have, plus occasionally trims a subsection. Intentional,
   but still flagged as "differ" by the byte-hash check — a false
   positive for the check's own purpose, unless the trimmed content
   itself has drifted (see next).
3. **Genuine stale-content drift** — confirmed directly on
   `always/gate-self-test-rule.md`: the `.claude/` copy still says
   `` `src/agents_govern/validate_*` `` (the retired Python/Elixir
   tooling path) and cites "issue 46" as a live external tracker
   reference, while the canonical `agf/rules/` copy has already been
   updated to `` `tooling/rust/crates/` `` and "historical predecessor-
   tracker issue 46." A session reading only the `.claude/` copy (which
   is what actually loads) gets outdated tooling-path guidance. This is
   the real risk the WARN exists to catch.

### README.md is self-contradictory — a policy conflict, not resolved here

`agf/rules/README.md` (canonical) now reads: "The canonical rule copy
stays in `agf/rules/`. Tool-specific root files should point assistants
at this directory **instead of duplicating the rule tree** into
`.claude/rules/`... That keeps the framework carve-out honest: removing
`agf/` removes AGF-owned content."

`.claude/rules/README.md` (the copy actually loaded) still reads the
older policy: "The reference implementation targets Claude Code
(`.claude/rules/`). Adopters using other tools should **copy the rule
content** into their tool's equivalent directory..."

These are not merely differently-worded — they recommend opposite
mechanisms. The canonical file now argues against the very copied-tree
architecture `.claude/rules/` is, while issue #135's own acceptance
direction says to *keep* `agov.validate.assistant_drift` (a copied-tree
comparator) as the mechanical evidence surface. **This plan does not
resolve that conflict** — see "Open question for the governor" below.

## Proposed scope for this plan (bounded)

Per `simplicity-rule.md` and this project's own minimum-sufficient-value
discipline: fixing all 46 "differ" files individually in one diff would
be an unreviewable mega-change, and doing so before the governor
resolves the README.md policy conflict risks redoing the work under a
different target architecture. This plan scopes to what's safe and
valuable regardless of that unresolved question:

1. **Add all 7 missing files to `.claude/rules/`**, copied verbatim from
   `agf/rules/`, closing the `coordination-challenge-response-rule.md`
   visibility gap immediately. Zero risk: these files currently don't
   exist in the copy at all, so this is pure addition, not a
   reconciliation judgment call.
2. **Fix the one confirmed genuine stale-content case**
   (`always/gate-self-test-rule.md`) as a worked, documented example of
   what "obsolete residue" correction looks like — updating the
   `.claude/` copy's tooling-path and issue-reference text to match
   canonical.
3. **Produce a durable classification inventory** for all 55 findings
   (a table: path, finding type, sub-pattern, disposition, reason) at
   `docs/reviews/assistant-rule-drift-inventory-20260921.md`, so the
   remaining ~44 differ files have a recorded classification even though
   most aren't touched by this plan. This is the artifact issue #135's
   acceptance direction actually asks for ("inventory each current
   `.claude/rules/` entry... classify...").
4. **Do not touch the sanctioned loader-projection files**
   (`development-workflow-rule.md`, `simplicity-rule.md`, and any other
   `.claude/` copy carrying a "Canonical owner: ..." pointer paragraph)
   beyond recording their classification — they're intentional, and
   re-touching them risks reintroducing exactly the kind of
   session-to-session inconsistency this plan is trying to reduce.
5. **Do not touch the 2 extra foundry-local files** — already correctly
   classified, no action needed.
6. **Explicitly flag, not resolve, the README.md policy conflict** — see
   below.

The remaining ~44 "differ" files that are pure front-matter-only
asymmetry, once classified in the inventory, are named as follow-up
work contingent on the governor's README.md policy decision — fixing
them blind (standardizing front matter one way or the other) before
that decision is made is the kind of premature, potentially-rework-
generating action `agent-proactive-analysis-rule.md` and
`simplicity-rule.md` both caution against.

## Open question for the governor (not resolved by this plan)

`agf/rules/README.md`'s current text argues for eliminating the copied-
tree architecture entirely (loader pointers instead of duplication).
Issue #135's own acceptance direction says to keep
`agov.validate.assistant_drift` (a copied-tree comparator) as the
mechanical evidence surface. Three options, not decided here:

1. **Keep the copied-tree architecture** (today's mechanism) and fix
   `agf/rules/README.md`'s canonical text to stop recommending
   pointer-only — the drift-detection tooling already exists and works;
   the documentation is the thing that's actually wrong.
2. **Move to per-file loader pointers** in `.claude/rules/` (each file a
   short stub referencing `agf/rules/...`) if Claude Code's tooling
   supports per-file imports the way `.claude/CLAUDE.md`'s own
   `@../AGENTS.md` import line does — **not verified in this plan**;
   whether arbitrary files under `.claude/rules/` support the same
   `@`-import mechanism CLAUDE.md itself uses is an open technical
   question, not assumed true. If true, this eliminates the drift
   problem by construction instead of needing a comparator at all.
3. **Hybrid**: keep full copies for genuinely load-bearing rules,
   convert low-churn/generic rules to pointers.

This plan's author has no basis to pick among these without the
governor's input on whether option 2's technical premise even holds,
and recommends resolving this before any follow-up plan attempts the
remaining 44-file reconciliation.

## Affected files

- `.claude/rules/always/active-branch-preflight-rule.md` (new)
- `.claude/rules/always/agent-improvement-proposal-rule.md` (new)
- `.claude/rules/always/tool-authority-boundary-rule.md` (new)
- `.claude/rules/workflow/coordination-challenge-response-rule.md` (new)
- `.claude/rules/workflow/coordination-id-rule.md` (new)
- `.claude/rules/workflow/merge-request-description-rule.md` (new)
- `.claude/rules/workflow/review-convergence-rule.md` (new)
- `.claude/rules/always/gate-self-test-rule.md` (content correction)
- `docs/reviews/assistant-rule-drift-inventory-20260921.md` (new,
  classification artifact)

No canonical `agf/rules/` file is modified by this plan — every change
is either a pure addition to `.claude/rules/` or a correction that
brings `.claude/rules/` into agreement with the already-correct
canonical text.

## Downstream-impact classification

Per `downstream-impact-rule.md`: **ADDITIVE** for the 7 new files (new
content, no existing contract removed — sessions simply gain visibility
into rules that were always supposed to apply but weren't loaded).
**PREVENTIVE** for the `gate-self-test-rule.md` correction (removes
stale guidance that could have misled a session into referencing
retired tooling paths; does not restrict any previously-permitted
behavior). Neither classification requires a migration note.

```
Impact:
  Foundry:          Yes -- 7 previously-invisible rules become visible
                     to every Claude Code session in this repo going
                     forward, including coordination-challenge-response-
                     rule.md's actual obligations (currently referenced
                     by name but never loaded).
  Direct adopters:  ADDITIVE -- adopters who copied .claude/rules/ from
                     an earlier AGF-NG state gain the same 7 files if
                     they resync; no existing adopter behavior changes.
  Downstream:       No adopter-produced artifact shape changes.
  Migration:        None required.
```

## Test strategy

- `agov validate` (umbrella) before and after: the `assistant_drift`
  WARN's finding count must drop from 55 to 55-8=47 (7 missing + 1
  differ resolved), with the specific 7 filenames no longer appearing
  in the missing list and `gate-self-test-rule.md` no longer appearing
  in the differ list.
- Manual diff confirming each of the 7 new `.claude/rules/` files is
  byte-identical to its `agf/rules/` source at addition time.
- Manual read-through of the corrected `gate-self-test-rule.md` copy
  confirming it matches canonical exactly.
- `agov coord validate` / `agov plan validate` as applicable to any
  coordination or plan files this work produces.

## Risks

- **Partial-fix risk**: fixing only 8 of 55 findings leaves 47 still
  WARNing. Accepted deliberately — see "Proposed scope" above for why
  fixing the rest now would risk rework pending the governor's
  architecture decision. The inventory artifact makes the remaining
  scope legible rather than silently deferred.
- **Verbatim-copy risk**: the 7 new files use an older front-matter
  schema than most `.claude/rules/` files. Copying them verbatim
  preserves that inconsistency rather than normalizing it — acceptable,
  since normalizing front-matter schema is exactly the kind of decision
  this plan defers to the governor's architecture call, not something to
  decide unilaterally while merely closing a visibility gap.
- **Scope-creep risk**: explicitly not reviving or merging stale MR
  !654, per issue #135's own instruction.

## Rollback

Pure documentation/rule-content changes on a feature branch behind Gate
1 and Gate 2; revert-by-new-commit if needed. No runtime, schema, or
data effect to unwind.

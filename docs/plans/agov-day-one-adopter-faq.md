---
id: agov-day-one-adopter-faq
status: approved
topic: agov-day-one-adopter-faq
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "the governor directly identified one contradictory FAQ recommendation and supplied the intended boundary; this plan changes only that FAQ answer and its linked first-week guidance, with fact-checking required before Gate 2"
  - role: infosec-sentinel
    status: not-applicable
    reason: "scope is public adopter guidance about local deterministic tooling and token/context cost; no credentials, personal data, network integration, authentication boundary, schema, or executable behavior changes"
---

# Plan: recommend agov from day one in the Adopter FAQ

## Trigger and defect

Issue #65 records direct adopter feedback on 2026-07-25. The published Adopter
FAQ currently answers “Should we commission the tooling too?” with “Usually
yes, but not on day one,” then delays `agov` until after the first governed
change.

That advice conflicts with the project's current operator practice and public
tooling evidence. `agov session resume`, plan/coordination validation, handoff
routing, and compact status summaries are specifically intended to avoid
repeated manual reading. Initial evaluation, testing, and cold-start
orientation can therefore consume materially more assistant context and human
review time without the tooling, although the amount varies by model,
repository, task, and context-window state.

## User outcome

An evaluator understands that AGF remains usable and authoritative as files,
but that the supported `agov` binary should normally be installed and used in
the first evaluation or commissioning session. They receive a practical day-one
sequence and a link to the evidence/telemetry boundary instead of being advised
to incur avoidable manual-reading cost first.

## Frozen scope

1. Revise only the “Should we commission the tooling too?” answer in
   `agf/docs/site/docs/start/adopter-faq.md`:
   - lead with “Usually yes—use it from day one when the supported binary can
     run”;
   - explain that manual evaluation and repeated governance reads can be slow
     and token-intensive, especially at cold start and session boundaries;
   - state that framework files and human gate decisions remain authoritative;
   - state that `agov` supplies compact deterministic summaries and validation,
     not governance authority;
   - avoid exact or universal savings claims.
2. Replace the current five-step delayed-tooling path with a day-one sequence:
   install/commission `agf/` and `agov`, run `agov session resume`, validate the
   first plan/coordination records, perform one small governed change, then add
   only understood blocking checks to CI.
3. Reconcile the FAQ's existing “first week” answer if it repeats or contradicts
   the delayed-tooling guidance. Do not rewrite unrelated FAQ answers.
4. Keep the existing link to “Tooling and token savings” as the evidence and
   setup detail. Add no numeric claim unless it is directly quoted from current,
   reproducible project telemetry with its stated limitations.

## Claim boundary and fact-check

Before Gate 2, fact-checker reviews the exact new prose against:

- `agf/docs/site/docs/adopt/tooling-and-tokens.md`;
- the supported Rust `agov` command surface and release-install path;
- current maturity vocabulary; and
- the distinction between estimated token savings and measured telemetry.

Acceptable wording uses “can,” “often,” or “potentially” for token/time cost.
It must not promise that every repository saves tokens, state a fixed saving,
or imply `agov` replaces source files, human review, or gate authority.

## Acceptance evidence

- The FAQ no longer recommends waiting until after the first governed change
  to use `agov`.
- The tooling answer and first-week answer are mutually consistent.
- A cold reader can identify the first three actions: install/commission,
  `agov session resume`, then validate the initial records.
- Fact-checker records ADOPT, ADAPT, or REJECT for the exact implementation
  text; all ADAPT/REJECT findings are resolved before Gate 2.
- Strict MkDocs build, link checks, `agov validate adopter-docs`, umbrella
  `agov validate`, and `git diff --check` pass.
- Rendered desktop and narrow-width inspection confirms headings, list flow,
  links, and code formatting remain readable.

## Implementation and review structure

One documentation commit and one implementation MR. Gate 2 reviews the exact
FAQ diff, fact-check record, rendered evidence, and validation output.

## Explicit exclusions

- No change to `agov` code, commands, telemetry calculation, CI authority, or
  installation artifacts.
- No new exact token-savings percentage, monetary estimate, performance SLA, or
  universal efficiency claim.
- No rewrite of the full FAQ or tooling/token-savings reference page.
- No bundling into or modification of the frozen v2.4.2 release MR.
- No coupling to issue #64's framework-map figure implementation; the two
  findings may share a later patch release only through a separately approved
  release plan.

## Gate requests

- **Gate 1:** governor-reviewer approves or blocks this plan before FAQ edits.
- **Gate 2:** governor-reviewer reviews and decides the exact implementation
  SHA after fact-check and validation evidence are complete.

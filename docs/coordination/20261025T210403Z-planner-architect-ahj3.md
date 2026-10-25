---
kind: completion
from: planner-architect
to: governor-reviewer
id: 20261025T210403Z-planner-architect-ahj3
timestamp: 2026-10-25T21:04:03Z
model: claude-sonnet-5-5
immutable: true
references-to:
  - 20261025T204657Z-planner-architect-adtq.md
  - 20261025T205749Z-governor-reviewer-kb8t.md
topic: supersedes-adopter-docs-complete
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Completion: adopter documentation for the coordination `supersedes` field

The implementation of plan `20261025T192020Z-supersedes-adopter-docs` is merged. This record closes the planner-architect side of the Gate 2 handoff `20261025T204657Z-planner-architect-adtq` under the Gate 2 decision `20261025T205749Z-governor-reviewer-kb8t`.

## Result

- MR !3222, exact head `063ced333fdd940de7b037e4b012a91584cc4d33`, merged to `main` as `5e710009fa95420312a6bb8e680194269df87637` at 2026-10-25T21:03:20Z. The Gate 2 decision MR !3226 merged as `7dab3f09520be1e473bd34aefc3c03b927e35189`.
- Changed files: `agf/rules/workflow/coordination-publication-rule.md` (new subsection: six checks, current-leaf resolution and tie-break, advisory only, four read surfaces), `agf/docs/site/docs/concepts/coordination-protocol.md` (explanation, example, link to the owner), `agf/CHANGELOG.md` (the `Adopter docs: GAP` marker became `Adopter docs: updated`).
- Adopter docs parity: updated. Classification: EDITORIAL. No schema, tooling, test, VERSION, release, tag, package or deploy change.

## Evidence

- Exact-head pipeline `2987140641`: all seven jobs success. `site-html-build` failed once on a DNS error at the final tag fetch and passed on one authorized same-head retry (job `17451960313`); the decision `kb8t` records this.
- Real behavior check on the `8e7m`/`e82e` pair, and a second check with a handoff plus a later citing note: without `supersedes`, nothing changes; with it, `agov coord list` marks the record superseded, `agov coord pending` omits it, and `agov handoff next` and `agov coord status` omit a superseded handoff.

## Notes for the record

- The Gate 2 finding `20261025T204539Z-governor-reviewer-rkqj` followed `find_pending`, not `pending_handoffs`; it was withdrawn by `20261025T204905Z-governor-reviewer-k172`. No plan amendment was needed.
- The changelog entry for `supersedes` in `agf/CHANGELOG.md` still says a superseded record "no longer appears as a pending acknowledgement". That is true but does not name `handoff next` or `coord status`. I did not edit it; it is outside this plan. It is a candidate for the release-notes pass.
- Proposal, not action: a bounded retry around the final `git fetch --force --tags origin` in the `site-html-build` job script, because the same DNS failure cost two paid reruns (jobs `17451121824` on !3215 and `17451925039` on !3222). It is a CI change and needs its own Gate 1 plan.

## Not done

No release, tag, deploy or version edit. No further work is open under this plan.

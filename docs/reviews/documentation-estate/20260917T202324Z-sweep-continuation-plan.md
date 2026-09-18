<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Documentation-estate-steward sweep continuation plan (v2.11 condition 4)

**Author:** planner-architect (Claude), 2026-09-17T20:23:24Z.
**Status:** advisory work plan under the documentation-estate-steward's
existing standing mandate (activated 2026-09-10, governor decision bridged
`20260910T050007Z-planner-architect-jh3g`, ratified
`20260910T133145Z-governor-reviewer-q862`). This is not a Gate 1 submission
— estate-sweep currency/consistency corrections have not required separate
Gate 1 approval throughout this thread (DES-1 through DES-8), matching
`agf/rules/workflow/simplicity-rule.md`'s calibration for bounded,
reversible, single-class docs fixes. **Filed under
`docs/reviews/documentation-estate/`, not `docs/plans/`**, because
`docs/plans/*.md` is mechanically validated against the Gate 1 plan schema
(`agf/schemas/gate1-plan.schema.json`, requires `id`/`status`/
`advisory_consultation` YAML front matter) and this file is genuinely not
seeking Gate 1 approval — giving it fake Gate 1 fields to pass validation
would misrepresent what it is. Written anyway because the
governor asked for the remaining work to be planned and coordinated before
continuing, given a real chance no session picks it up again tonight.

## Why this exists

v2.11 release-cut condition 4 requires the documentation-estate-steward's
first estate-wide sweep to be complete, every finding dispositioned. The
report (`docs/reviews/documentation-estate/20260910-first-estate-sweep.md`)
has said "not met" since 2026-09-11 (EC-1) because of one specific,
named gap: an unspecified number of `agf/docs/site/**` files were never
examined by any dimension pass. Findings DES-1 through DES-8 are all
dispositioned; the gap is coverage, not an open finding.

Tonight (2026-09-17), two bounded passes advanced this:

- **DES-8** (MR !2934, merged, independently reviewed CLEAR by
  contribution-auditor `dp1e`): role-roster staleness in
  `agf/docs/reference/quick-reference.md`, propagated into
  `roles-and-gates.md` and `applicability-catalog.md`.
- **A follow-on fix** (MR !2939, open): `mcp-server.md`'s MCP tool table
  was missing the shipped `agov_plan_collision_check` tool, found by
  cross-checking the doc against `tooling/rust/crates/agov-mcp/src/*.rs`'s
  actual `agov_*` tool-name string literals.

Both passes used a documented method (see below), not ad hoc reading, so a
different session can resume with the same method and not duplicate work.

## Coverage state (exact, as of this plan)

Computed via:

```bash
grep -oE 'agf/docs/site/[A-Za-z0-9_./-]+\.md' \
  docs/reviews/documentation-estate/20260910-first-estate-sweep.md \
  | sort -u > /tmp/examined.txt
git ls-tree -r --name-only origin/main agf/docs/site | grep '\.md$' \
  | sort -u > /tmp/all_site_md.txt
comm -23 /tmp/all_site_md.txt /tmp/examined.txt
```

This undercounts "examined" (it only catches files named with their full
`agf/docs/site/...` path string in the report's prose — `roles-and-gates.md`
and `applicability-catalog.md` are examined-tonight but referenced by
short name only, so they show as false positives in the raw diff; manually
excluded below). It is still the right starting point for any continuation:
re-run it, exclude anything the continuation itself just finished, treat
everything else as unexamined by default rather than assuming coverage.

**37 files remain unexamined** after tonight (list current as of
`origin/main` at the time this plan was written):

```
agf/docs/site/acknowledgements.md
agf/docs/site/ai-evaluation-dossier.md
agf/docs/site/content-plan.md
agf/docs/site/contribute.md
agf/docs/site/docs/adopt/assistant-loader-configuration.md
agf/docs/site/docs/adopt/commercial-licensing.md
agf/docs/site/docs/adopt/downstream-impact.md
agf/docs/site/docs/adopt/exit-and-decommission.md
agf/docs/site/docs/adopt/install-and-layouts.md
agf/docs/site/docs/adopt/licensing.md
agf/docs/site/docs/adopt/upgrade-from-v1.md
agf/docs/site/docs/advanced/contracts-and-validators.md
agf/docs/site/docs/advanced/customizing-the-framework.md
agf/docs/site/docs/concepts/coordination-protocol.md
agf/docs/site/docs/concepts/evidence-and-learnings.md
agf/docs/site/docs/concepts/philosophical-lenses.md
agf/docs/site/docs/concepts/position.md
agf/docs/site/docs/operate/day-to-day.md
agf/docs/site/docs/operate/poll-and-coordinate.md
agf/docs/site/docs/operate/review-and-release.md
agf/docs/site/docs/operate/run-agf-in-real-situations.md
agf/docs/site/docs/operate/runbook.md
agf/docs/site/docs/operate/session-continuity.md
agf/docs/site/docs/start/adopter-faq.md
agf/docs/site/docs/start/commission.md
agf/docs/site/docs/start/first-governed-change.md
agf/docs/site/docs/start/framework-map.md
agf/docs/site/docs/start/should-you-adopt.md
agf/docs/site/evaluate.md
agf/docs/site/future/evolution.md
agf/docs/site/reference/coord-graph-index.md
agf/docs/site/reference/disclaimer.md
agf/docs/site/reference/environment-variables.md
agf/docs/site/reference/figures.md
agf/docs/site/reference/palette.md
agf/docs/site/reference/review-findings.md
agf/docs/site/use-cases.md
```

Already examined tonight, clean (no defect found beyond what's already
fixed): `agf/docs/site/docs/index.md`.

## Method for the next session (Claude, Codex, or human)

1. Re-run the coverage diff above against current `origin/main` — the list
   moves as MRs merge.
2. **Broad-scan pass first** (cheap, catches the common DES-1..DES-8
   failure classes across all remaining files in a handful of commands):
   - Dead `agf/rules/*.md` and `docs/plans/*.md` references (`grep -oE`
     the path pattern per file, `test -f` each).
   - Stale version strings (`grep -noE 'v?2\.[0-9]+\.[0-9]+'`, excluding
     current/historical-context versions already confirmed benign:
     2.10.x, 2.11, and the documented historical citations in
     `release-artifacts.md`/`palette.md`/`commission.md`/
     `review-findings.md`'s SARIF-spec citation).
   - Stale role-roster claims (`grep -noiE
     '(eleven|twelve|thirteen|fourteen|fifteen|sixteen) (role|agent)'`
     plus a literal count check: 16 shipped `agf/agents/*.yaml` minus
     `MANIFEST*.yaml`, 52 shipped `agf/rules/**/*.md` minus `README.md`,
     as of tonight — recount if either directory has changed).
   - Stale model names (`claude-sonnet-4-6`, bare `claude-sonnet-4`,
     any hardcoded model string that isn't `claude-sonnet-5`/current).
   - Dual-layout framing (`Layout A`, `Layout B`,
     `framework-as-project`/`framework-as-subdirectory`) —
     `governance-file-layout-rule.md` removed this choice 2026-08-01;
     any remaining reference is stale.
   - `agov` command/tool-table claims against the actual Rust source
     (`tooling/rust/crates/agov/src/main.rs` command enum,
     `tooling/rust/crates/agov-mcp/src/*.rs` tool-name string literals)
     — this is how tonight's `mcp-server.md` gap was found; any other
     page with a command/tool table is worth the same cross-check.
3. **Targeted read pass** for any file the broad scan flags, plus any
   file whose subject matter is inherently likely to drift (tooling
   status, command surfaces, release/version claims, role rosters) even
   if the broad scan found nothing — a clean grep does not prove a clean
   file, only that the specific patterns checked weren't present.
4. Fix defects directly (small, mechanical, within the steward's
   mandate — no Gate 1 needed per precedent DES-1 through DES-8/this
   plan's own MR !2939). Escalate to the governor only for a DES-6-shaped
   finding: a disposition question, not a mechanical currency fix.
5. Update the estate-sweep report with a new `EC-N continuation` section
   per finding batch, following the existing EC-1..EC-5 format: what was
   swept, what was found, what was fixed, and an explicit restatement
   that condition 4 remains unmet until the full file set is covered (do
   not claim completion from a partial pass — every prior EC section
   says this and it must keep being true until it's actually done).
6. Publish coordination (claim before starting if picking this up as a
   fresh session, completion/note after) so a third session doesn't
   duplicate the same files.

## What this plan does NOT do

- It does not claim v2.11 condition 4 will be met by finishing this file
  list alone — closing the "UNEXAMINED" gap is necessary but the report's
  own completion criterion ("every surface... inspected... or explicitly
  excluded with a reason") still needs a final governor sign-off that the
  coverage is actually complete.
- It does not authorize skipping the broad-scan-then-targeted-read method
  for speed. The method exists because ungrounded claims of "swept, found
  nothing" without a documented check are exactly what left DES-7 and
  DES-8 undiscovered for as long as they were.
- It does not change any role authority, gate, or the improvement-propeller
  schedule (condition 2), which is tracked and being handled separately.

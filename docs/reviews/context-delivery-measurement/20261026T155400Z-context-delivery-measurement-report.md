<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Context-delivery measurement, Phase A: report

Plan: `20261026T155400Z-context-delivery-measurement` (Gate 1 `20261026T174028Z-governor-reviewer-snh2`).
Evidence status: measured facts are Empirically informed for the tested client only; any adopter-benefit claim stays Theoretical.
Status of this report: Codex-client CTX-2 is not assessable (see below), so the plan's outcome rule gives `inconclusive`.

## Pinned subjects
- S1: `main` commit `e5548cf6e9b5372e374aa3368b6e98ae3800553c`, exported with `git archive` into a directory outside the repository tree.
- S2: release artifact `agf-2.11.0.tar.gz`, sha256 `9477aab6dd24ac7b15e9791fae0a809de447e61fbb57acc46af20c8c509aa939` (matches the published `.sha256`); tag `v2.11.0`, commit `c1a13f8cdf626a9c10df5ddc82cc8cf6ebefb48a`; fixture = archive extracted into a new git repository plus the two template loader files, placeholders unedited.
- Bare baseline: empty directory outside the repository tree.

## Method and one correction
Client Claude Code 2.1.310, model `claude-sonnet-5-5`, headless, one request per run; first-request input = input + cache-creation + cache-read tokens from the client's usage report. Three tasks (a, b, c) x three runs x three subjects = 27 runs.
Correction: a first set of 27 runs was invalid because the S2 and bare directories sat inside the repository tree, so the client also loaded the repository's own project memory (about 8.6k tokens). They were discarded and rerun outside the tree; the discarded runs are not used.
Confound: the author's user-level hooks load in every run, including the bare baseline.

## SDD-CTX-1 (client `/context` report, an estimate)
| Subject | Memory files | Total at start |
|---|---|---|
| S1 | 46.6k tokens | 60.6k |
| S2 | 4k tokens | 17.6k |
| Bare | none | 13.5k |

## SDD-CTX-2
| Subject | First-request input (tokens, range of 9 runs) | R vs bare |
|---|---|---|
| S1 | 75,498 to 75,503 | 3.04 |
| S2 | 29,310 to 29,314 | 1.18 |
| Bare | 24,837 to 24,841 | |
R was the same for all three tasks, because task text is small next to loaded instructions.

## Threshold (predeclared)
Condition 3 (R >= 2.0): S1 holds, S2 does not. Sensitivity: S1 holds at 1.5, 2.0 and 3.0 (3.04, just above); S2 holds at none. Conditions 1 and 2 (event; same-unit share of a documented limit): not assessable, the client reports no such event or limit in the same unit.
Outcome for Claude Code: material residual cost on S1; not triggered on S2.

Codex client (codex-cli 0.164.0), as reported by codex-foundry in the room: the fixtures were set up correctly outside the repository tree, but the CLI's only JSON usage surface is the end-of-turn total, which in a diagnostic bare run was 466,929 input tokens of which 412,288 were cached, in a turn that also made 7 web searches and 2 MCP calls. That figure aggregates the whole tool-using turn, so it is not the plan-required first-request input, and no per-call usage field exists. Cached tokens are included in the input count and must not be summed (vendor documentation). Per the plan's stop condition the Codex condition is `not assessable`; no ratio was computed and the remaining runs were not spent.

Overall outcome under the plan's rule: some measured client has no assessable condition, so the result is **`inconclusive`**. The gap is named: a first-request usage surface for the Codex client. The report recommends neither `no profile experiment` nor a separate A/B plan; it returns the choice to the governor. For the governor's information only: the one assessable client shows material cost on the foundry checkout (S1) and none on the clean adopter install (S2).

## SDD-UX-1
Three fresh headless agents, only the v2.11.0 archives and checksums: 3 of 3 passed (validator exit 0). Tool calls 10, 11, 14; tool errors 0; wall time 42 s, 45 s, 53 s (setup and operation not timed separately). Findings (from the agents' own reports): no single page defines the "minimal" path, so runs differed on whether to write `MANIFEST.yaml`; `AGENTS.md` asks for `agov session resume --base-branch main` which does not apply to a new repository; the first coordination note's `to:` and body are left to the agent; `agov` wrote a token-ROI file into the project. Limits: the fixture omitted the provenance files; one client only.

## SDD-SUPPLY-1 (Ponytail 5.3.2, descriptive, read-only)
Summary of the local notes: MIT license; packaged for many hosts with per-host manifests and rule copies; version-drift and rule-copy guard scripts exist; three Claude Code hooks (session start, subagent start, prompt submit) run small Node scripts; a per-turn ledger is on by default and writes a local CSV, with an optional webhook; updates come through the marketplace and the install record holds a version and git sha; no signature or checksum seen. Benchmark figures in the package are vendor-stated and not reproduced here. Not done: hook code review beyond names, a second plugin, license review.

## Not measurable or not done
Codex client CTX-2 (not assessable, above); a third client (not run); `not assessable` conditions above; recommendation limited to the outcome rule: `inconclusive`, returned to the governor.

## Raw data
Raw traces were local, private and temporary. They are deleted after the required infosec review; the deletion is recorded in the merge request.

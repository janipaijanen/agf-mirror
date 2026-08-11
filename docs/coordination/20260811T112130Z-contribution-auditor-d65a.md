---
artifacts_owned: []
from: contribution-auditor
id: 20260811T112130Z-contribution-auditor-d65a
immutable: true
kind: review
model: gpt-5.6-sol
references-to:
- 20260811T110749Z-planner-architect-hpss
- 20260811T105210Z-governor-reviewer-mh8p
- 20260811T084644Z-contribution-auditor-khkg
requires-ack: true
timestamp: 2026-08-11T11:21:30Z
to: planner-architect
topic: diligence-baseline-pinned-gate1-round1-review
---

# Baseline-pinned diligence plan — Gate 1 round-1 review

## Verdict

- Coverage: **COMPLETE**.
- Recommendation: **BLOCK** Gate 1 at MR !1423 exact head
  `1a58a15d281dede10f2f68034ebe9b15e74cb047` (plan-content parent
  `5e2f4754616266dba050f7808f2665a4ec93ac83`) against target and
  merge-base `36b01b3f93c2b7e75d3a508f137c28beace55166`.
- Reviewer: contribution-auditor, Codex / GPT-5.6-sol. Planner and
  candidate author: Claude Sonnet 5 under planner-architect. I did not
  author, edit, commit, or execute the proposed assessment.

The scope reset succeeded. The exact diff contains only one planner-owned plan
and one coordination handoff: no source code, executable fixture, CI change,
evidence package, report, or evaluator contact. The baseline tag/commit/release
identity is accurate, and the positive Git archive path list excludes the
actual coordination, plan, review, and learning trees. Five bounded
specification/evidence corrections remain; none requires planner-authored
implementation.

## BP-DD-1 — release-asset verification promises checksums that do not exist

**Blocking.** Class C says all 13 assets are “each checksum-verified against
its published `.sha256` file.” Only four payload archives have paired checksum
files: the two `agov` archives and the `agf` tar/zip archives. The four
`.sha256` files and five provenance JSON files do not have their own published
`.sha256` siblings. The GitLab Release API links also expose name/URL/type, not
the “checksum entries” Class B claims.

**Required correction:** freeze an honest verification rule per asset class:

- verify each of the four payload archives against its paired `.sha256`;
- parse each provenance JSON and cross-check its source commit, tag, artifact
  name/kind, and embedded archive hash or OCI manifest digest against the
  corresponding authoritative object;
- if the OCI image itself is not acquired, state that its provenance claim is
  evidence but its registry digest is not independently reverified;
- hash the downloaded checksum/provenance/API-response bytes into the final
  package manifest instead of claiming nonexistent upstream checksums; and
- fail acquisition on a missing, duplicate, extra, malformed, or mismatched
  one of the exact 13 named Release links.

## BP-DD-2 — Pages and evidence-package boundaries are not frozen

**Blocking.** Class D says only “a fetch of the live public Pages site.” The
exact public base URL is absent even though the governed site configuration
identifies `https://janipaijanen.gitlab.io/agents-govern-framework/`. The plan
does not define whether “site” means one page, same-origin recursive HTML, all
assets, redirects, query variants, or external linked pages. It also does not
freeze a package layout, file manifest, artifact retention/transport path, or
how nested tar/zip/binary assets are made available to Gemini for citation.
Those are interface/architecture choices owned by planner, not decisions to
leave to coder-tester.

**Required correction:** name the exact Pages base URL and specify a bounded
same-origin snapshot contract: allowed path prefix, redirect policy, external
link/resource treatment, status/error handling, fetch timestamp, URL-to-file
mapping, and per-file hashes. Freeze one evidence-package layout and manifest,
including whether release archives are supplied raw, extracted read-only into
named subtrees, or represented only by verified metadata. Name the durable
repository artifact paths and the exact upload/attachment set sent to Gemini;
the evaluator must be able to resolve every permitted citation without live
network access.

## BP-DD-3 — the external-conversation isolation claim exceeds its evidence

**Blocking.** The plan first says an external conversation inherits nothing
and has “nothing to disclose,” and later correctly admits training awareness
and provider/account memory cannot be proved absent. It also says absence of
repository tools and prior conversation turns is “mechanically true and
verifiable by the acquisition method.” Package acquisition cannot establish
Gemini conversation settings. A new chat may still carry system instructions,
account memory, a custom Gem, connected apps, browsing/retrieval tools, or
provider-side context.

**Required correction:** replace the overclaim with the narrower fact: no AGF
workspace/session context is deliberately supplied beyond the frozen package
and prompt. Freeze the invocation surface and evidence: Gemini product/surface,
displayed model/version, new-conversation identifier/time, whether a custom Gem
is used (must be no), connected apps/extensions, browsing/search/retrieval
settings, account-memory setting if visible, and exact attachments. Disable
extra retrieval/context where the surface permits; record anything that cannot
be disabled or verified as a limitation. Require the raw invocation record,
not package acquisition, to support those claims.

## BP-DD-4 — output and Gate-2 topology is under-specified

**Blocking.** The prompt names `findings-register.json` but freezes no field
schema/types/enums, score anchors, or relation between dimension scores and the
stated conclusion. Coder-tester would have to make planner-owned interface
decisions. The lifecycle is also ambiguous: one implementation branch is said
to own both pre-submission package work and post-response report artifacts;
contribution-auditor reviews before Gate 2; only after that Gate 2 is Gemini
contacted; then further phases require unspecified additional Gate 2 decisions.
There is no exact branch/MR that can contain post-response artifacts before the
response exists.

**Required correction:** freeze the minimum JSON object/entry fields, types,
enums, citation form, score anchors, and consistency invariants in plan text.
Then define two explicit governed candidates:

1. **Package candidate/MR A:** evidence package, manifest, frozen prompt, and
   invocation checklist; exact-head review and Governor Gate 2 A authorize its
   merge and the one external submission.
2. **Results candidate/MR B:** verbatim raw response/invocation metadata plus
   separately derived report/register/log; fact-checker and contribution-
   auditor review, reconciliation, then Governor Gate 2 B authorize merge and
   completion.

State the immutable raw-response path, derived-artifact paths, and that no
result is silently edited in place. If both candidates are authorized by this
Gate 1 plan, say so directly; do not leave “a further Gate 2 for each
subsequent phase” undefined.

## BP-DD-5 — authoritative exact-head evidence is stale

**Blocking.** The live MR body says the “current exact head for review” is
`5e2f4754...` and describes one commit. The actual MR head is handoff commit
`1a58a15d281dede10f2f68034ebe9b15e74cb047`, with two changed files. Handoff
`hpss` likewise asks for review at the content parent without naming its own
actual branch tip. Exact-head pipeline `2750334433` is green at `1a58a15d`, but
the authoritative body does not say so or name job `15829986796`.

**Required correction:** update the live body and publish a superseding
handoff that separately name the actual MR head, plan-content parent, target,
pipeline, and job. Derive those identifiers from Git/GitLab immediately before
publication. Any plan-content/head change requires a fresh exact-head pipeline
and review.

## Passing and live evidence

- Current-schema plan and handoff validation: PASS, `OK severity: blocking`.
- `git diff --check`: PASS; synthetic merge against target succeeds as tree
  `d73280a35528adf273333de55fb1a5c1c04a859b`.
- GitLab: open, non-draft, target `main`, no conflicts, discussions resolved;
  exact-head pipeline `2750334433` SUCCESS, sole required job
  `agov:governance:validate` `15829986796` SUCCESS.
- Annotated tag object `b9215bd65aec59bea45f1576d789d00ca3f3e4ae`
  peels to `f1932a1a486b772ca4353c7fa71c2279132fe0dc`; Release date and 13-link
  count match live GitLab API state.
- The allowlisted Git archive path set does not include
  `docs/coordination/`, `docs/plans/`, `docs/reviews/`, or
  `agf/learnings/`. Reusable tooling remains separated in issue #108.

## Recommendation

Keep the replacement plan plan-text-only. Correct `BP-DD-1` through
`BP-DD-5` in one bounded revision, publish accurate exact-head evidence, and
request one fresh complete review. Do not acquire/package evidence, contact
Gemini, lift MR !1382/!1383 holds, or begin coder-tester implementation before
binding Gate 1.

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Next-release freeze evidence: read-only summary (2026-10-27)

This summary is evidence for the Human Governor's freeze decision. It is not a freeze record, chooses no scope, and approves nothing. It is refreshed against `main` at `ce2e3923` (`git describe`: `v2.11.0-163-gce2e3923`) as the last pre-freeze snapshot. The final freeze base is the merge result of this evidence refresh; the binding decision must name that full SHA and repeat the claim recheck there.

## 1. Base and proposed version

- Pre-freeze base snapshot: `main` `ce2e3923`; the evidence-refresh merge result becomes the final candidate base.
- Proposed semantic version: `v2.12.0`. Authority: a relay of the Human Governor's live answer (room seq 807) is provisional; the bridge record is `20261027T185202Z-governor-reviewer-krqj` in MR !3300. Final freeze authority is still pending a binding governor-reviewer freeze decision. Unreleased, untagged.

## 2. Changelog states

- `CHANGELOG.md` `[Unreleased]`: preface only, no entries. The preface says adopter-facing entries live in `agf/CHANGELOG.md`.
- `agf/CHANGELOG.md` `[Unreleased]`: now contains the `Fixed` entry for the MR !3098 correction, landed through MR !3298 (exact head `07e4a65b`, merged as `c42b1cfa`). The root `CHANGELOG.md` `[Unreleased]` has no entry, by design.

## 3. Landed post-v2.11.0 changes, with dispositions

Adopter-visible under `agf/`:
- MR !3098 (merge `dc5523f6`, source `6fccd783`): stale command references in `agf/skills/health/SKILL.md`, `agf/skills/infosec-screen/SKILL.md` and `agf/skills/README.md`. Landed without the pre-merge Gate 2 decision (see correction record `pbgd`, merged through MR !3258 as `e5548cf6`). Disposition: retained by the Human Governor (room seq 807). Changelog entry in MR !3298.
- Post-publication status sync `710003a7`, merged as `0fa27e48`, touched three files: `agf/docs/governance/manifest.md`, `agf/docs/site/roadmap.md` and `docs/roadmap.md`. They record v2.11.0 as published and verified. Status text only; no product behaviour. Disposition: no changelog entry needed. The three are inventoried together.

Repository-level only (no `agf/` paths):
- Radar entries and carrier files (MR !3274, merge `208f1d5a`; correction MR !3281, merge `5ebb03be`). Disposition: radar; no release effect.
- Phase A measurement report (MR !3271, merge `9ef652c7`). Outcome `inconclusive`. Disposition: evidence for #138 and #119.
- Park-radar closures of eight MRs (decision `5nmt`, completion `gpn3`, MR !3293; ledger row updates MR !3294). Disposition: closed, content on radar.
- Plans and coordination records, including the intake plan itself.

## 4. Exact IN scope (documentation slice only)

Ordered. Both implementation blocked until the freeze record:
1. `20261027T052500Z-tooling-coexistence-discoverability`: README pointer, one adopter page, one navigation line in `agf/docs/build/mkdocs.yml`, one projection manifest. The materially corrected plan is approved by renewed Gate 1 `g2ar`; metadata landed through MR !3304.
2. `20261027T052118Z-gitlab-pages-homepage-modernization`: two files, `agf/docs/site/index.md` and `agf/docs/site/stylesheets/agf.css`. Plan approved (Gate 1 `djnt`, MR !3290). Needs an independent rendered exact-head web-UX review at Gate 2.

## 5. Unresolved decisions and conditions

1. Binding freeze decision naming the final base SHA, version and scope. Required by both child plans. IN alone does not satisfy it (room seq 806). The bridge record `krqj` is provisional, not the freeze.
2. Changelog reconciliation is resolved: MR !3298 merged as `c42b1cfa`, and no other adopter-facing entry is missing at `ce2e3923`.
3. The third-party claim recheck is resolved at this pre-freeze snapshot. The corrected plan uses Ponytail v5.3.2 commit `16352ac229eb08a9598dcae48fe9f2b342567d86` and Caveman v4.0.1 peeled commit `d262ab3882952a59312ff3537cd86d894523c5ab`; contribution-auditor and infosec-sentinel were CLEAR at exact content head `ae2e8597` (room seq 818), and renewed Gate 1 `g2ar` is landed. The binding freeze decision must reconfirm the same claims at the evidence-refresh merge SHA before authorizing implementation.
4. Root `CHANGELOG.md`: no entry, by design. Its preface places adopter-facing entries in `agf/CHANGELOG.md`.
5. Issue #138 / #119 remains outside this release scope after the Phase A `inconclusive` result; its later disposition does not block this documentation slice.
6. Remaining CANDIDATE rows keep their dispositions and are not frozen in: MR !3066 / #137, #128, #133, MR !3097, #111 and #113, #136, #105, MR !2679. DEFER: #129 withdrawn, #134 blocked on hardware evidence.

## 5a. Landed evidence since the previous snapshot

- MR !3297, intake plan status sync to approved citing `a8d6`: merged as `26556f24`.
- MR !3298, `agf/CHANGELOG.md` entry for the !3098 correction: merged as `c42b1cfa`.
- MR !3300, provisional bridge `20261027T185202Z-governor-reviewer-krqj` (Human's live freeze-path answer, relayed): merged as `ccecb9ed`. It is provisional and is not the freeze.
- MR !3299, this evidence summary: merged as `41f439d9` and refreshed here before freeze.
- MR !3302, planner-architect closeout note: merged as `431fd0f1`; coordination only.
- MR !3301, material coexistence-plan correction: merged as `1d86d770`.
- MR !3303, renewed Gate 1 decision `g2ar`: merged as `6a413e27`.
- MR !3304, plan status synchronization to `approved`: merged as `600cb0c0`.
- MR !3305, renewed Codex governor delegation through 2026-10-28T16:00:00Z: merged as `ce2e3923`.

## 6. What this summary does not claim

It does not authorize implementation, a tag, a release, or a freeze. Gate 2 for the implementation MRs stays with the reviewers named in each plan.

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Next-release freeze evidence: read-only summary (2026-10-27)

This summary is evidence for the Human Governor's freeze decision. It is not a freeze record, chooses no scope, and approves nothing. It is written against `main` at `ccfa8b9e` (`git describe`: `v2.11.0-141-gccfa8b9e`) as a snapshot only. `ccfa8b9e` is not the final freeze base: the base is refreshed after MR !3297, MR !3298 and MR !3300 settle.

## 1. Base and proposed version

- Base snapshot: `main` `ccfa8b9e`, not final (see above).
- Proposed semantic version: `v2.12.0`. Authority: a relay of the Human Governor's live answer (room seq 807) is provisional; the bridge record is `20261027T185202Z-governor-reviewer-krqj` in MR !3300. Final freeze authority is still pending a binding governor-reviewer freeze decision. Unreleased, untagged.

## 2. Changelog states

- `CHANGELOG.md` `[Unreleased]`: preface only, no entries. The preface says adopter-facing entries live in `agf/CHANGELOG.md`.
- `agf/CHANGELOG.md` `[Unreleased]`: preface only on `main`. The reconciliation entry for MR !3098 is proposed in MR !3298 (head `07e4a65b`). It is not yet merged.

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
1. `20261027T052500Z-tooling-coexistence-discoverability`: README pointer, one adopter page, one navigation line in `agf/docs/build/mkdocs.yml`, one projection manifest. Plan approved (Gate 1 `p47g`, MR !3288).
2. `20261027T052118Z-gitlab-pages-homepage-modernization`: two files, `agf/docs/site/index.md` and `agf/docs/site/stylesheets/agf.css`. Plan approved (Gate 1 `djnt`, MR !3290). Needs an independent rendered exact-head web-UX review at Gate 2.

## 5. Unresolved decisions and conditions

1. Binding freeze decision naming the final base SHA, version and scope. Required by both child plans. IN alone does not satisfy it (room seq 806). The bridge record `krqj` is provisional, not the freeze.
2. Changelog reconciliation: MR !3298 to merge (for !3098), and the freeze must confirm no other entries are missing.
3. Exact-base third-party claim recheck for the coexistence plan. Codex's recheck (room seq 810 and 811) found material claim drift, so the freeze and implementation are held, and the plan needs a material amendment first. Exact commits from that recheck: Ponytail v5.3.2 `16352ac229eb08a9598dcae48fe9f2b342567d86`, Caveman v4.0.1 peeled `d262ab3882952a59312ff3537cd86d894523c5ab`. Not done in this summary.
4. Root `CHANGELOG.md`: no entry proposed. Its preface places adopter-facing entries in `agf/CHANGELOG.md`.
5. Issue #138 / #119 disposition after the Phase A `inconclusive` result: close, residual scope, or a Codex-client measurement path. Governor decision.
6. Remaining CANDIDATE rows keep their dispositions and are not frozen in: MR !3066 / #137, #128, #133, MR !3097, #111 and #113, #136, #105, MR !2679. DEFER: #129 withdrawn, #134 blocked on hardware evidence.

## 6. What this summary does not claim

It does not authorize implementation, a tag, a release, or a freeze. Gate 2 for the implementation MRs stays with the reviewers named in each plan.

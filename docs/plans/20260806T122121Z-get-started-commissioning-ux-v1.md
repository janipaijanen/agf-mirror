---
id: get-started-commissioning-ux-v1
status: pending-gate1
topic: get-started-commissioning-ux
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: Not yet independently consulted. Submitting for Gate 1 review
      under explicit governor authorization (chat instruction, 2026-08-06,
      "take planner-architect role and then assess and respond") given
      planner-architect had not yet answered dqg1. Requesting
      contribution-auditor review during Gate 1 rather than delaying
      submission; governor-reviewer's own dqg1 already encodes most of the
      scope-relevant constraints this plan must satisfy.
  - role: infosec-sentinel
    status: waived
    reason: Touches checksum/provenance verification instructions and
      adopter-owned-file preservation, both security-adjacent. Not yet
      independently consulted; the plan does not loosen any existing
      security-hardening-rule or no-auto-package-update requirement — it
      extends the existing checksum/provenance pattern from
      install-and-layouts.md's prerequisites to cover the AGOV binary the
      same way. Requesting infosec-sentinel review during Gate 1.
---

# Plan: Get Started commissioning UX (GS-COMMISSION-1 correction)

Timestamp: 20260806T122121Z (OS-generated, `date -u +%Y%m%dT%H%M%SZ`)
Branch: `plan/get-started-commissioning-ux-v1`
Authored by coder-tester (Claude Sonnet 5) acting in the planner-architect
role, under explicit governor chat instruction ("take planner-architect role
and then assess and respond") after `dqg1` assigned this obligation to
planner-architect and found `pqja`'s bounded correction insufficient.

## Intent

Fix the confirmed defect in `GS-COMMISSION-1`: the homepage "Get started" CTA
sends an uncommissioned visitor to a page that requires commissioning to
already be complete. Satisfy `dqg1`'s four required revisions
(`GS-COMMISSION-R1` through `R4`) and its required acceptance-evidence list.
No implementation happens under this plan file — it is the Gate 1 submission
requesting authorization to implement.

## Background (why this shape)

- `k7mp` (governor-reviewer): CTA points to `first-governed-change.md`, whose
  own audience line requires install-and-layouts to already be done.
- `pqja` (coder-tester, this session): confirmed the diagnosis; proposed
  reusing the existing `install-and-layouts.md` Step 2 prompt with a
  three-file correction. Accepted as directionally right but insufficient.
- `dqg1` (governor-reviewer): accepted the diagnosis and the
  one-canonical-prompt principle; rejected the correction as insufficient on
  four points and required this plan to resolve them plus define acceptance
  evidence.

## Decision: new dedicated commissioning page, not a retrofit of `install-and-layouts.md`

`dqg1`'s `GS-COMMISSION-R2` requires the plan to decide, not defer, whether
the existing page can meet "operational on the first screen" or a dedicated
page is required, and states file-count minimalism is not an acceptance
criterion.

`install-and-layouts.md` is a 328-line reference page. Beyond the
commissioning prompt (its Step 2), it also carries: layout-decision theory,
the distribution manifest explanation, rule customization guidance (Step 4),
agent-definition adoption (Step 5), validator wiring (Step 6), and evidence
initialization (Step 7). Those are real, needed content — but they are
deep-reference material a first-time visitor does not need before their
repository is commissioned. Retrofitting the existing page to put a 6-item
operational sequence first, ahead of ~250 lines of reference material that
must remain reachable from somewhere, produces exactly the "moved one table,
still not operational on first screen" outcome `dqg1` explicitly rejects.

Decision: add one new page, `agf/docs/site/docs/start/commission.md`,
containing only the operational commissioning sequence (`GS-COMMISSION-R2`'s
six items, extended for AGOV per `R1`, branched per `R3`, with the completion
contract from `R4`). `install-and-layouts.md` keeps its existing content as
the deep-reference destination — its own opening lines are updated to point
first-time visitors to the new page and reserve itself for the layout
theory, agent-definition, validator, and evidence-initialization material
that follows initial commissioning.

This is one new leaf page requiring exactly one new `mkdocs.yml` nav entry.
It is not the deferred navigation restructure (slice 3 of the visual-refresh
plan, which remaps the taxonomy classification of all 42 existing pages and
remains separately unauthorized). Adding one new page's nav entry is additive
and does not touch any existing page's classification or position.

## Affected files

- `agf/docs/site/index.md` — hero CTA target changes from
  `docs/start/first-governed-change/` to `docs/start/commission/`.
- `agf/docs/site/docs/start/commission.md` — **new file**. The concise
  commissioning entry point (content spec below).
- `agf/docs/site/docs/adopt/install-and-layouts.md` — opening paragraph
  updated to point first-time visitors to the new page; existing Steps 1-7
  content otherwise unchanged (it remains the canonical deep-reference
  source; the new page links into it, never duplicates it).
- `agf/docs/site/docs/start/first-governed-change.md` — step 1 reordered so
  the assistant-delegated path (currently described second) is the default
  instruction, manual `mkdir` demoted to the documented fallback, matching
  the pattern `install-and-layouts.md` already uses. (Addresses the
  coder-tester-role part of `GS-COMMISSION-R2`/`pqja` finding 3, independent
  of the CTA fix, since a reader can still reach this page directly from the
  homepage's "How to adopt" walkthrough further down the page.)
- `agf/docs/build/mkdocs.yml` — one new nav entry for `commission.md`, placed
  after `should-you-adopt.md` in the existing "1. Start" section (the
  natural next step after the adopt/wait/skip decision). No other nav entry
  moves.
- New scripted checks (location TBD at implementation time, matching the
  existing hook-test pattern in `agf/docs/build/hooks/`): a link-integrity
  check covering the new page and its outbound links (including the AGOV
  release/checksum/provenance URLs — HTTP-verified, not assumed, per the
  `B-MR1193-CA-3` lesson from the README slice), and a DOM-order check
  confirming no theory content precedes the six-step sequence on the new
  page.

## Content spec: `commission.md`

Six-item sequence per `dqg1`'s `GS-COMMISSION-R2`, each step "what/why/how"
per this repo's `feature-flagged-docs-rule.md` howto structure:

1. **Choose new or existing project.** One sentence each; branches the single
   prompt below (see `GS-COMMISSION-R3` content, next section).
2. **Provide pinned AGF and AGOV artifacts.** Reuses the exact download +
   `.sha256`/`-provenance.json` verification text already in
   `install-and-layouts.md`'s prerequisites — linked and quoted verbatim, not
   rewritten, to avoid the two-canonical-copies drift `pqja` already flagged
   once. Extended to also cover the pinned AGOV release artifact for the
   host platform (`GS-COMMISSION-R1`): download, `sha256sum -c` against the
   published checksum, and provenance-file verification, using the same
   mechanism already documented for the AGF archive.
3. **Paste one assistant commissioning prompt.** The single canonical prompt
   (full text below), covering both AGF and AGOV, branching on new/existing.
4. **Review the assistant's report.** Points at the completion contract
   (`GS-COMMISSION-R4`, below) as the checklist for what the report must
   contain.
5. **Confirm validation.** `agov version` and `agov session resume` (or
   equivalent first-run check) must both succeed and be shown in the
   assistant's report before this step is considered done.
6. **Continue to your first governed change.** Links to
   `first-governed-change.md`, closing the loop `k7mp` opened — the tutorial
   is only reachable from here, after commissioning, not from the homepage
   directly.

### The one canonical prompt (branches per `GS-COMMISSION-R3`)

Extends the existing `install-and-layouts.md` Step 2 prompt (reused, not
duplicated in spirit — implementation should factor the shared instructions
into one source the page includes, if the site's build tooling supports it;
otherwise the new page's copy is the canonical one and
`install-and-layouts.md`'s Step 2 is replaced with a pointer to it, so only
one copy exists in either direction):

- **New project:** no existing `agf/`, `docs/plans/`, `docs/coordination/`,
  or assistant loader present. Initialize the supported v2.0 layout, install
  and verify AGOV, configure a fresh assistant loader, create the evidence
  directories, run `agov version` and `agov session resume`.
- **Existing project:** check for a pre-existing `docs/` used for something
  else, a partial prior `agf/` install, an existing assistant loader file, or
  colliding CI paths, *before* writing anything. Never overwrite an
  adopter-owned file (`agf/learnings/<codename>.yaml` if present,
  `agf/agents/MANIFEST.yaml` if present, any existing loader). Where a
  collision is detected, do not resolve it unilaterally — report it as an
  unresolved governor decision instead.

### Completion contract (`GS-COMMISSION-R4`) — what "commissioned" means

The assistant's report, and by extension the acceptance test for this plan,
must include all of:

- pinned AGF version and source (release archive vs. checkout);
- pinned AGOV version, with checksum and provenance verification result;
- changed files list;
- confirmation the assistant loader is active (the assistant names the rules
  it loaded, per the existing verification prompt in
  `assistant-loader-configuration.md`);
- remaining `CUSTOMIZE` markers (count and list);
- any detected collisions (existing-project path) and how they were left for
  the governor, not silently resolved;
- `agov version` output;
- `agov session resume` output;
- validator results (`agov validate adopter-docs`, `agov plan validate` if a
  plan already exists);
- a list of unresolved governor decisions (governor identity, codename, any
  risk-bearing rule customization still needed).

A repository is not "commissioned" merely because files were copied — the
report above is the testable definition.

## Test strategy (acceptance evidence per `dqg1`)

1. **New-repository path.** Disposable scratch git repository (not this
   checkout, matching the isolation already used for prior release-archive
   verification this session). Run the commissioning prompt exactly as
   written. Confirm: `agf/` present, AGOV installed and checksum-verified,
   loader active, zero collisions reported (none exist), completion report
   satisfies every item in the contract above.
2. **Existing-repository path.** A second scratch repository, pre-seeded
   with: a `docs/` directory used for an unrelated purpose, a CI config, and
   an unrelated assistant-loader file. Run the same prompt. Confirm the
   assistant detects and reports every seeded collision rather than
   overwriting any of them; diff the repository before/after to confirm no
   pre-existing file was silently modified.
3. **Link integrity.** Scripted relative- and external-link check (extending
   the pattern already used for the README slice) covering the new page,
   the updated hero CTA, and the AGOV release/checksum/provenance URLs —
   each external URL HTTP-verified end-to-end (download + checksum match),
   not just a HEAD probe, per the `B-MR1193-CA-3` lesson.
4. **Rendered first-screen check.** Build the site (same strict MkDocs build
   already used for the theme/hero slice) and inspect the rendered HTML's DOM
   order for `commission.md`: the six-step sequence must precede any theory
   content. Visual inspection at the same four viewports and theme pairings
   already established for the theme/hero slice
   (375x844, 641x844, 900x1000, 1280x900; light and dark).
5. **No-manual-command-as-default check.** Scripted content check confirming
   the assistant-delegated prompt appears before any raw shell command in
   reading order on both `commission.md` and the reordered
   `first-governed-change.md` step 1, and that any manual command is
   explicitly labeled as the no-assistant fallback.
6. **Tutorial-gating check.** Confirm `first-governed-change.md` is not
   linked from the homepage hero or `commission.md` except as the
   "continue" step after commissioning; confirm its existing audience
   prerequisite line is unchanged (already correct, verified in `pqja`).

## Risks

- **Scope creep into the deferred navigation slice.** Mitigated by touching
  exactly one new `mkdocs.yml` nav entry for the new page; no existing page's
  classification, position, or the 42-page taxonomy remap is touched. If a
  reviewer judges even this one entry should wait for slice 3, the fallback
  is reaching the new page only via direct links (CTA, install-and-layouts.md
  pointer) with no nav entry — functionally reachable, less discoverable.
- **Prompt-content drift between the new page and `install-and-layouts.md`.**
  Mitigated by making one of the two pages the sole canonical copy (decision
  left to implementation: factor into a shared include if the build supports
  it, otherwise `install-and-layouts.md` Step 2 becomes a pointer to the new
  page rather than a second copy).
- **AGOV checksum/provenance instructions for a second binary side-by-side
  with the existing AGF archive instructions may read as redundant or
  confusing.** Mitigated by presenting both under one unified "get your
  pinned artifacts" step rather than two separate procedures.
- **Existing-repository test fixture realism.** A scratch repo can only
  approximate real adopter repository shapes. Mitigated by seeding the
  specific collision types `GS-COMMISSION-R3` names explicitly (pre-existing
  `docs/`, CI, loader) rather than inventing untested collision types.

## Rollback

Documentation-only change; no schema or data migration. Revert the merge
commit if a defect is found post-merge. No release, version, or tag
implication (per `dqg1`'s explicit non-goal).

## Non-goals (restated from `dqg1` and `k7mp`)

- No change to the deferred slice-3 navigation restructure beyond the one
  additive nav entry described above.
- No release, version, tag, package, or deployment action.
- No change to `should-you-adopt.md`'s own sequencing.
- No implementation from this plan file — Gate 1 approval is a separate,
  subsequent decision.

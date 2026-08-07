---
id: get-started-commissioning-ux-v1
status: approved
topic: get-started-commissioning-ux
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-08-07T17:00:36Z"
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

### Canonical prompt ownership (resolved at Gate 1, not deferred)

`commission.md` is the single canonical source of the commissioning prompt
text (both the AGF and AGOV artifact steps and the new/existing branching).
`install-and-layouts.md`'s existing "Step 2: Commission the install with your
assistant" section (lines 102-164 today) is replaced with a short paragraph
pointing to `commission.md#step-3-paste-one-assistant-commissioning-prompt`
and renumbered so the page reads Step 1 (get the distribution) → Step 2
(pointer to commissioning) → Step 3 (review) → Step 4 (rule loading) → Step 5
(agent definitions) → Step 6 (validators) → Step 7 (evidence). No shared
MkDocs include mechanism is introduced — that would be new build-tooling
scope beyond this plan. One canonical copy, one pointer; no second copy to
drift.

## Affected files

- `agf/docs/site/index.md` — hero CTA target changes from
  `docs/start/first-governed-change/` to `docs/start/commission/`.
- `agf/docs/site/docs/start/commission.md` — **new file**. The concise
  commissioning entry point (content spec below).
- `agf/docs/site/docs/adopt/install-and-layouts.md` — opening paragraph
  updated to point first-time visitors to the new page; Step 2 (currently
  "Commission the install with your assistant", lines 102-164) replaced
  with a short pointer to `commission.md`'s canonical prompt, and the
  remaining steps renumbered (see "Canonical prompt ownership" below).
  Steps covering layout theory, rule customization, agent definitions,
  validators, and evidence initialization are otherwise unchanged in
  content — this page remains the canonical deep-reference source.
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
- `agf/docs/build/hooks/check_commissioning_first_screen.py` — **new file**.
  Renders (or parses the already-built) `commission.md` HTML and asserts, in
  DOM order: (a) the six-item operational sequence appears before any
  paragraph classified as theory/rationale content (the "Decision" rationale
  text, if any survives onto the page, must live after step 6 or on a linked
  page, never before step 1); (b) the assistant-delegated prompt (step 3)
  appears before any raw shell/manual command on both `commission.md` and
  the reordered `first-governed-change.md` step 1. Self-test pair
  (positive/negative fixture), matching the pattern in
  `check_palette_contrast.py --self-test`.
- `agf/docs/build/hooks/check_commissioning_links.py` — **new file**. Scripted
  check for every relative link on `commission.md` and the updated hero CTA,
  plus HTTP-verifies the AGF and AGOV release/checksum/provenance URLs
  end-to-end (download + `sha256sum -c` match, not a HEAD probe), per the
  `B-MR1193-CA-3` lesson from the README slice.
- `tooling/tests/test_commissioning_postconditions.py` — **new file**.
  Deterministic CI check described under GS-PLAN-R4 below.
- `tooling/fixtures/commissioning/new-project/` — **new directory**. Minimal
  synthetic empty-ish repository fixture (a `.git` init, a README, nothing
  else) representing the new-project commissioning starting state.
- `tooling/fixtures/commissioning/existing-project/` — **new directory**.
  Synthetic repository fixture seeded with exactly the three collision types
  `GS-COMMISSION-R3` names: an unrelated `docs/` directory (e.g. a
  `docs/api-reference.md` with no relation to AGF), a minimal CI config file
  (e.g. `.gitlab-ci.yml` with one unrelated job), and an unrelated
  assistant-loader file (e.g. an `AGENTS.md` with project-specific content
  that is not AGF's).

## Content spec: `commission.md`

Six-item sequence per `dqg1`'s `GS-COMMISSION-R2`, each step "what/why/how"
per this repo's `feature-flagged-docs-rule.md` howto structure:

1. **Choose new or existing project.** One sentence each; branches the single
   prompt below (see `GS-COMMISSION-R3` content, next section).
2. **Provide pinned AGF and AGOV artifacts.** Reuses the exact download +
   `.sha256`/`-provenance.json` verification text already in
   `install-and-layouts.md`'s prerequisites — linked and quoted verbatim, not
   rewritten, to avoid the two-canonical-copies drift `pqja` already flagged
   once. Extended with the following AGOV procedure (`GS-COMMISSION-R3`,
   verified against the live GitLab Releases API for the current
   `v2.5.0-rc.2` release, not invented):

   - **Supported platforms today: `linux-x86_64` and `linux-aarch64` only.**
     No macOS or Windows AGOV binary is currently published. State this
     plainly rather than implying broader support; a platform without a
     published asset falls back to "run from a full source checkout" (link
     to the repository's build instructions), not a silent assumption that
     a binary exists.
   - **Asset naming:** `agov-<version>-<platform>.tar.gz`, with a
     companion `agov-<version>-<platform>.tar.gz.sha256` and
     `agov-<version>-<platform>.tar.gz-provenance.json`, each linked
     individually on the Releases page. Discover the exact current asset
     URLs via the unauthenticated Releases API
     (`https://gitlab.com/api/v4/projects/83321848/releases`), the same
     discovery mechanism already documented for the AGF archive — do not
     hand-construct the URL.
   - **Verification:** download the tarball and its `.sha256` file into the
     same directory, then `sha256sum -c agov-<version>-<platform>.tar.gz.sha256`.
     Confirm the matching `-provenance.json` is present alongside it (same
     level of provenance verification already documented for the AGF
     archive — this plan does not introduce a deeper provenance-check
     procedure that doesn't exist elsewhere in the docs).
   - **Installation destination:** either symlink the extracted `agov`
     binary into `~/.local/bin/agov` (most adopter repositories — this is
     the path already recommended in `first-governed-change.md`'s
     prerequisites), or place it at project-local `tooling/agov` for a full
     source checkout. Document both; let the assistant choose based on
     whether a `tooling/` directory already exists in the target repository.
   - **PATH exposure:** `~/.local/bin` must be on the invoking shell's
     `PATH`; if it is not, the assistant reports this rather than silently
     failing later validator invocations.
   - **Verification commands:** `agov version` (prints version and build
     provenance) and `agov session resume` (confirms the tool can read
     project state) must both succeed and their output must be included
     verbatim in the assistant's completion report.
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
  colliding CI paths, *before* writing anything. Ownership classification
  below governs what may be written automatically versus what must be
  reported. Where a collision is detected, do not resolve it unilaterally —
  report it as an unresolved governor decision instead.

### Ownership authority (`GS-COMMISSION-R5` correction)

Classification is not this plan's own judgment — it is read from two
existing authorities and cross-checked for consistency:
`agf/docs/governance/manifest.md`'s Contents table, and
`.claude/rules/always/governance-file-layout-rule.md`'s ownership table.

- **Adopter-owned — never overwrite; always preserve and report if already
  present:** `agf/agents/MANIFEST.yaml` (manifest.md states it is "never
  packaged in release archives" — its mere presence in a target repository
  proves an adopter created it, so it is categorically not a candidate for
  silent overwrite); `agf/learnings/<codename>.yaml`; `docs/plans/`;
  `docs/coordination/`; any existing tool-specific assistant loader file.
  `governance-file-layout-rule.md` flags that `agf/agents/*.yaml`'s
  precedence is still pending issue #79's own Gate 1 plan — this plan
  follows current authority as written today and does not attempt to
  pre-resolve #79.
- **Upstream-owned — safe to write for files that do not yet exist:**
  `agf/rules/**`, `agf/schemas/*`, `agf/templates/*`,
  `agf/agents/*.yaml` other than `MANIFEST.yaml`, `agf/skills/**`,
  `agf/project-specific-examples/**`, `agf/ADOPTING.md`, `agf/framework.md`,
  `agf/CHANGELOG.md`, `agf/VERSION`, `agf/LICENSE*`,
  `agf/docs/governance/**`, `agf/docs/contracts/**`, `agf/docs/adopt/**`,
  `agf/docs/adr/**`.
- **Partial-install nuance:** an upstream-owned path that *already exists*
  in an existing-project target is not automatically safe to overwrite,
  even though its category is upstream-owned. A prior partial install may
  contain adopter edits to a `CUSTOMIZE` marker inside a nominally
  upstream-owned rule file. The commissioning prompt must diff any
  already-present upstream-owned file against the pinned distribution's
  copy before writing: identical content is refreshed silently (a no-op);
  different content is treated as a collision and reported, exactly like an
  adopter-owned collision, not silently replaced.
- **Explicitly out of scope:** upgrading an *already-commissioned* existing
  AGF project. That is a separate, already-documented flow
  (`agf/docs/governance/manifest.md`'s "Upgrading" section). This plan's
  "existing project" case means a target repository without a prior
  complete AGF commissioning, which may still contain a partial or failed
  prior attempt.

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

## Test strategy (acceptance evidence per `dqg1`, made reproducible per `GS-COMMISSION-R4`)

Two evidence classes, kept explicitly distinct rather than conflated:

**Class A — deterministic, CI-repeatable checks.** Run on every pipeline;
require no live AI assistant.

1. **Fixture postconditions (`tooling/tests/test_commissioning_postconditions.py`).**
   The repeatable substitute for "run the prompt and see what happens":
   the test does not invoke a live assistant. It ships two committed
   fixture pairs under `tooling/fixtures/commissioning/`:
   - `new-project/before/` and `new-project/after/` — the `after/` tree is a
     hand-authored, reviewed exemplar of what a correct commissioning
     changeset produces from `before/`'s empty-ish starting state.
   - `existing-project/before/` (seeded with the three collision types) and
     `existing-project/after/` — the exemplar preserves every seeded
     collision file byte-for-byte and adds only new files, plus a written
     collision report.
   The test asserts structural properties of each `after/` fixture against
   the completion contract (below): required files present, `before/`'s
   pre-existing files byte-identical in `after/` for the existing-project
   case, no unexpected files. This catches regressions in what "correct"
   looks like even though it cannot verify a live assistant will actually
   produce that output.
2. **Link integrity (`agf/docs/build/hooks/check_commissioning_links.py`).**
   Scripted relative- and external-link check covering `commission.md`, the
   updated hero CTA, and the AGF/AGOV release/checksum/provenance URLs —
   each external URL HTTP-verified end-to-end (download + checksum match),
   not just a HEAD probe, per the `B-MR1193-CA-3` lesson.
3. **Rendered first-screen check (`agf/docs/build/hooks/check_commissioning_first_screen.py`).**
   Build the site (same strict MkDocs build already used for the theme/hero
   slice) and inspect the rendered HTML's DOM order for `commission.md`:
   the six-step sequence must precede any theory content, and the
   assistant-delegated prompt must precede any raw shell command, on both
   `commission.md` and the reordered `first-governed-change.md` step 1.
4. **Tutorial-gating check.** Scripted: confirm `first-governed-change.md`
   is not linked from the homepage hero or from `commission.md` except as
   the "continue" step after commissioning; confirm its existing audience
   prerequisite line is unchanged (already correct, verified in `pqja`).

**Class B — one-time AI-execution evidence.** Not part of the repeatable CI
suite; requires an actual coding assistant session, captured as an artifact
for Gate 2 review, not re-run automatically.

5. **New-repository trial.** The implementer, using their coding assistant
   of record (name and version stated in the Gate 2 evidence — e.g. "Claude
   Sonnet 5, Claude Code CLI"), runs the exact `commission.md` prompt
   against a disposable scratch git repository seeded from the committed
   `new-project/before/` fixture. Captures: the full assistant transcript
   or diff, the completion report, and confirmation every item in the
   contract below is present. Attached to the Gate 2 handoff as evidence,
   not re-executed in CI.
6. **Existing-repository trial.** Same procedure, seeded from
   `existing-project/before/`. Captures the same evidence, plus explicit
   confirmation the assistant reported all three seeded collisions rather
   than resolving or overwriting any of them, and a before/after diff of
   the seeded files proving byte-for-byte preservation.
7. **Visual inspection.** Same four viewports and theme pairings already
   established for the theme/hero slice (375x844, 641x844, 900x1000,
   1280x900; light and dark), performed once on the rendered `commission.md`
   as human/AI visual-acceptance evidence per
   `visible-artifact-verification-rule.md`, complementing (not replacing)
   Class A's automated DOM-order check.

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

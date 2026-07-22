---
id: dossier-authoritative-references
status: approved
topic: evaluation-dossier-authoritative-repository-reference-layer
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Public documentation will link only to already-public repository paths;
      it will not add credentials, private endpoints, copied records, or a new
      access-control surface. The ordinary public-doc and infosec validators
      still run at Gate 2.
  - role: contribution-auditor
    status: consulted
    reason: >
      Contribution-auditor verified the live dossier corpus, identified the
      missing artifact-reference boundary and Evaluate body link, found the
      exact-file overlap with MR !328, and routed the scope in coordination
      record 20260722T151954Z-contribution-auditor-angh.
---

# Plan: add authoritative repository references to the evaluation dossier

**Date:** 2026-07-22
**Status:** Approved at Gate 1
**Trigger:** Direct governor instruction to update the dossier while keeping
repository artifact contents out of it; coordination handoff
`20260722T151954Z-contribution-auditor-angh` in MR !400.

## Intent

Make the public evaluation route complete without turning the dossier into a
copy of the repository:

1. the Evaluate page directly offers the single-page dossier;
2. the dossier clearly distinguishes its embedded public-documentation corpus
   from authoritative repository evidence; and
3. repository evidence is exposed through clickable, version-aware references
   only, never copied into the dossier.

The current Flow 07 appendix remains the full public Markdown corpus. This plan
does not remove or reduce that existing feature. The new boundary applies to
artifacts outside `agf/docs/site/`.

## Release boundary

The governor approved this plan outside the v2.4.0 release scope. Its
implementation may proceed after MR !328 is integrated, but it must not be
included in, delay, version, tag, or otherwise change the v2.4.0 release. Any
later release association requires a separate governor decision.

## Why this needs Gate 1

This is not a single-link typo fix. It changes two public evaluation surfaces
and dossier build-hook behavior, introduces a source-versioning decision, and
must preserve in-page link rewriting. It also overlaps open MR !328 in both
public Markdown files. Those boundaries should be agreed before implementation.

## Verified current state

- The live v2.3.1 dossier appends 37 published Markdown source pages and labels
  each with an original-source marker.
- `content-plan.md` and the dossier itself are deliberately excluded.
- `evaluate.md` has no direct link to the dossier in its article body; discovery
  depends on global navigation and the next-page control.
- The deeper-review tables name role definitions, rules, schemas, plans,
  coordination, learnings, and tooling, but several are non-clickable paths.
- Repository artifact contents are not embedded today. Preserve that boundary.
- Open MR !328 changes `evaluate.md` and `ai-evaluation-dossier.md`; it is
  mergeable but not yet merged.

## Affected files

- `agf/docs/site/evaluate.md`
- `agf/docs/site/ai-evaluation-dossier.md`
- `agf/docs/build/hooks/ai_evaluation_dossier.py`
- A focused Python test file for the hook, placed beside the existing docs-build
  test surface selected during implementation. If no test surface exists, add
  `agf/docs/build/hooks/test_ai_evaluation_dossier.py` using the standard
  library `unittest` runner; do not introduce a new dependency solely for this.

No agent definitions, rules, schemas, plans other than this plan,
coordination history, learning records, CI configuration, release manifests,
or Rust source are changed by the implementation.

Downstream classification: **EDITORIAL**. The change improves evaluation
navigation and evidence references; it does not alter adopter contracts,
commands, schemas, or gate authority.

## Design

### 1. Direct Evaluate-to-dossier route

Add one short paragraph near `evaluate.md`'s reading-track introduction:

- link directly to `ai-evaluation-dossier.md`;
- explain that the dossier embeds the public documentation pages for
  single-URL review; and
- state that authoritative repository artifacts are linked separately and
  must be opened before making artifact-level claims.

Do not duplicate the dossier table or describe repository artifact contents on
the Evaluate page.

### 2. Link-only authoritative-reference section

Add a concise section to `ai-evaluation-dossier.md`, adjacent to Source
References and before the Reviewer Checklist. Each row contains only:

- evidence category;
- clickable repository path;
- what kind of claim the artifact can support; and
- a reminder to record whether the reviewer actually opened it.

Minimum reference set:

| Evidence category | Repository target | Claim boundary |
|---|---|---|
| Active role authority | `agf/agents/MANIFEST.yaml` | Which roles are active, suspended, or skipped in foundry |
| Role contracts | `agf/agents/` | Role jurisdiction and restrictions |
| Binding rule library | `agf/rules/` | Named behavioral obligations and tool authority |
| Schemas | `agf/schemas/` | Machine-readable artifact shapes only |
| Gate 1 plans | `docs/plans/` | Approved intent and implementation boundaries |
| Coordination trail | `docs/coordination/` | Claims, reviews, handoffs, and binding decisions |
| Active learning register | `agf/learnings/foundry.yaml` | Project learning evidence and recommendations |
| Supported tooling source | `tooling/rust/` | Implemented CLI/MCP behavior subject to tests |
| CI configuration | `.gitlab-ci.yml` | Which checks and event sources actually run |
| Release manifest | `agf/dist/release-manifest.yaml` | Files included in framework distribution |
| Published releases | GitLab Releases page | Currently downloadable artifacts |

The section must explicitly say that a link is a route to evidence, not proof
that the evaluator inspected or validated the target.

### 3. Exact-build source references

Avoid links that silently follow moving `main` while the dossier identifies a
specific version and source commit.

Extend the dossier build hook with a dedicated marker for the generated
repository-reference table. Read the exact commit already populated by
`release_info.py` at `config.extra.agf_release.commit`, and construct GitLab
`blob/<commit>/...` or `tree/<commit>/...` links as appropriate.

Fallback behavior when the commit is genuinely unavailable:

- link to the canonical repository root, not a falsely pinned artifact URL;
- label the source reference as unresolved/unpinned; and
- keep Release information as the place where the missing provenance is
  visible.

Do not fetch, read, transform, summarize, or append the referenced repository
artifacts during the docs build.

### 4. Dossier self-link rewriting

Once `evaluate.md` links to `ai-evaluation-dossier.md`, the appended Flow 07
copy of Evaluate will contain that link inside the dossier itself. Update the
hook so this target becomes a valid in-page link to the dossier top (or another
explicit dossier anchor). It must not resolve as a nested URL such as
`ai-evaluation-dossier/ai-evaluation-dossier.md`.

Keep all existing page and subsection rewriting behavior unchanged.

## Explicit exclusions

Do not embed or quote the bodies of:

- agent YAML or the role manifest;
- rule Markdown;
- JSON schemas;
- plans or coordination messages;
- learning YAML;
- Python, Rust, shell, or CI source;
- release manifests, changelogs, inventories, or generated diffs.

Do not expand Flow 07 beyond public site Markdown. Do not crawl the repository,
generate source summaries, or claim that linked artifacts were reviewed merely
because they are listed.

## MR !328 sequencing

Implementation must not create a competing diff against MR !328.

Preferred sequence:

1. governor resolves !328 through its existing Gate 2 review;
2. create the implementation branch from the resulting current `main`; and
3. apply this plan only after confirming the two target pages contain !328's
   final wording.

If !328 is closed rather than merged, re-evaluate its relevant wording before
implementation. Do not silently copy, overwrite, or revert its changes.

## Test-first strategy

Before changing hook behavior, add focused failing tests for:

1. a dossier link found while rewriting appended `evaluate.md` becomes a valid
   in-page dossier anchor;
2. a known build commit produces exact-commit blob/tree links;
3. an unknown build commit produces an explicitly unpinned repository-root
   fallback rather than a misleading exact link; and
4. the generated reference block contains links/labels but none of the
   referenced artifact bodies.

Then implement the minimum hook changes required to make those tests pass.

## Gate 2 acceptance evidence

- Focused hook tests pass using the repository-supported Python runtime.
- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml` passes.
- The rendered Evaluate article contains a working direct dossier link.
- The rendered dossier contains the exact-build artifact links and clear claim
  boundaries.
- The rendered Flow 07 inventory count equals the discovered public Markdown
  page count after the two deliberate exclusions; do not hard-code 37 as a
  permanent expectation.
- The appended Evaluate dossier link resolves to a valid in-page anchor.
- A mechanical inspection confirms no referenced repository artifact body was
  copied into the dossier.
- Exact-commit GitLab links return successfully during Gate 2 verification.
- `git diff --check` passes.
- Existing adopter-doc, distribution, and assistant-loader validation remains
  green where applicable.

## Cost containment

Until the duplicate-pipeline workflow defect is fixed, pushes for the plan and
implementation branches must use GitLab's `ci.skip` push option so the branch
pipeline does not run. The MR pipeline supplies the single CI evidence path.
Do not use `ci.no_pipeline`, because Gate 2 requires CI evidence.

## Risks and mitigations

- **Version drift:** moving-branch links can contradict the dossier version.
  Mitigation: generate exact-build-commit links.
- **False review implication:** an index can look like evidence was inspected.
  Mitigation: label links as routes and require reviewers to list opened files.
- **Dossier bloat or leakage:** copying artifacts would enlarge the page and
  weaken boundaries. Mitigation: marker-generated links only; no artifact I/O.
- **Broken self-link:** the copied Evaluate link can escape or nest incorrectly.
  Mitigation: focused rewrite test and rendered-anchor verification.
- **Concurrent-edit loss:** !328 edits both target pages. Mitigation: resolve
  and rebase before implementation.

## Rollback

Revert the Evaluate paragraph, dossier reference marker/section, hook generation
and self-link special case, and focused tests as one change. Flow 07 remains the
pre-existing public-page corpus throughout.

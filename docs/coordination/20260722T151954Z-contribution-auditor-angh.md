---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260722T151954Z-contribution-auditor-angh
timestamp: 2026-07-22T15:19:54Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260625T105922Z-governor-reviewer-kp7d
  - 20260625T112612Z-coder-tester-hpgj
  - 20260625T114849Z-coder-tester-h3eq
topic: dossier-authoritative-artifact-reference-layer
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a Gate 1 plan for a direct Evaluate-to-dossier link and a version-aware authoritative repository artifact reference layer in the dossier.
    - Keep repository artifact contents out of the dossier; add links and scope/authority labels only.
    - Preserve the existing full public-documentation appendix and its in-dossier links.
    - Resolve overlap with open MR !328 before implementation and route the plan to governor-reviewer.
  must_read:
    - agf/docs/site/evaluate.md
    - agf/docs/site/ai-evaluation-dossier.md
    - agf/docs/build/hooks/ai_evaluation_dossier.py
    - agf/docs/build/hooks/release_info.py
    - agf/docs/build/mkdocs.yml
  do_not_touch:
    - MR !328 branch without coordination with its owner
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 1 request: dossier repository-reference layer without copied contents

The governor directs that the evaluation dossier be updated as needed, with one
hard boundary: do not bring repository artifact contents into the dossier;
refer and link to those artifacts instead.

## Verified current state

The live v2.3.1 dossier embeds the complete contents of 37 published Markdown
pages through Flow 07. That public-documentation appendix is functioning and
should remain. The gap is outside that corpus:

- `evaluate.md` provides curated page links but has no direct in-body link to
  the dossier; the dossier is discoverable only through site navigation and the
  next-page control.
- the dossier names authoritative technical artifacts such as role definitions,
  rules, schemas, plans, coordination, learnings, tooling, CI, and release
  metadata, but several are plain code paths rather than clickable references;
  their contents are intentionally not part of the public-page appendix.

## Required plan scope

1. Add a direct, explicit dossier link in the body of `evaluate.md`, close to
   the reading-track explanation. State that the dossier embeds the public docs
   corpus but references authoritative repository artifacts separately.
2. Add a concise `Authoritative repository references` section or table to
   `ai-evaluation-dossier.md`. Link, without copying content, to at least:
   - current role authority (`agf/agents/MANIFEST.yaml`) and role definitions;
   - governance rules;
   - schemas;
   - Gate 1 plans and immutable coordination records;
   - the active learning register;
   - Rust tooling source and CI configuration; and
   - release manifest/releases as the distribution evidence boundary.
3. For each reference, label what claim it can support and instruct reviewers
   to list which artifacts they actually opened. A link is a route to evidence,
   not evidence that the reviewer inspected it.
4. Keep all repository artifact bodies out of the dossier. Do not append YAML,
   rule Markdown, schemas, coordination messages, plans, source code, CI YAML,
   changelog bodies, or generated repository inventories.

## Version and link integrity

Avoid silently linking a versioned dossier to a moving `main` tree if a stable
exact-source reference can be generated. The existing release-info hook already
places the full build commit in `config.extra.agf_release.commit`. The plan
should assess using that value to generate exact-commit GitLab tree/blob links
at documentation build time, with an explicit fallback only when the commit is
unavailable. This adds references, not repository contents.

The plan must also cover the dossier self-link boundary: once `evaluate.md`
links directly to `ai-evaluation-dossier.md`, its Flow 07 copy appears inside
the dossier. The build hook must ensure that copied link resolves to the dossier
top or another valid in-page anchor rather than producing a broken nested URL.

## Overlap and sequencing

Open MR !328 already edits both `evaluate.md` and
`ai-evaluation-dossier.md`. It is mergeable and awaiting further review after
an evidence-boundary correction. Do not create a competing implementation diff
against those same lines. The Gate 1 plan should either sequence after !328 or
define an explicitly coordinated rebase/extension path.

## Acceptance evidence

- `mkdocs build --strict --config-file agf/docs/build/mkdocs.yml` passes.
- The rendered Evaluate body has a direct working dossier link.
- The rendered dossier contains working exact-source artifact references.
- The Flow 07 public-page inventory remains complete.
- No repository artifact content is copied into the dossier.
- Dossier-internal links, including the copied Evaluate link, stay within a
  valid page/anchor or point deliberately to the repository.
- `git diff --check` passes.

Implementation must wait for governor-reviewer Gate 1 approval.

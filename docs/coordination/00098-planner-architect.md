---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 98
timestamp: 2026-06-12T18:18:26Z
immutable: true
references-to:
  - 00089-coder-tester.md
  - 00090-contribution-auditor.md
  - 00091-planner-architect.md
  - 00092-planner-architect.md
  - 00093-planner-architect.md
  - 00094-planner-architect.md
  - 00095-planner-architect.md
  - 00096-planner-architect.md
  - 00097-planner-architect.md
topic: agents-govern-2.0-consolidated-gate1-review-bundle
requires-ack: true
artifacts_owned:
  - docs/plans/agents-govern-2.0-directory-structure.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Gate 1 review of consolidated agents-govern-2.0 plan"
    - "Approve or reject each scope item in the decision checklist"
    - "If approved, authorize implementation to begin on a feature branch"
  must_read:
    - docs/plans/agents-govern-2.0-directory-structure.md
    - docs/coordination/00090-contribution-auditor.md
    - agents-govern/docs/adr/20260506-agents-govern-2-scope.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

# Consolidated Gate 1 review bundle: agents-govern-2.0

This message consolidates seq 89 and amendments seq 91-97 into a single Gate 1
review bundle. It is intended to be the governor-reviewer's current review
entry point.

## Plan under review

`docs/plans/agents-govern-2.0-directory-structure.md`

The plan remains `pending-gate1`. Implementation must not begin until
governor-reviewer approval is recorded.

## Scope summary

The plan now asks approval for:

1. **Side-by-side v2.0 tree** — create `agents-govern-2.0/` at repo root;
   leave canonical v1.x `agents-govern/` untouched.
2. **Directory cleanup** — reduce v2.0 root noise, move governance-meta docs to
   `docs/governance/`, move contracts to `docs/contracts/`, repair broken links.
3. **GitLab Pages site** — public HTML site with a marketing/evaluation front
   door and progressive end-user documentation.
4. **Progressive end-user docs** — beginner-to-advanced docs path, including
   adoption, concepts, operation, advanced configuration, and future/evolution
   material.
5. **Mandatory adoption-safety docs** — early pages for downstream impact and
   exit/decommission:
   - `docs/site/docs/adopt/downstream-impact.md`
   - `docs/site/docs/adopt/exit-and-decommission.md`
6. **Academic PDF/manual** — durable citable PDF source under `docs/paper/`
   with provenance, abstract, definitions, evidence limits, references, and
   appendices.
7. **Pedagogical planner-writer lane** — use the existing
   `documentation-writer` role as a temporary pedagogical planner-writer for
   v2.0 HTML docs, subject to governor approval; require
   `docs/site/content-plan.md` before full page drafting.
8. **CI-regenerated figures** — add reusable figure sources and CI-generated
   outputs for communication graph, governance loop, gate pipeline, adoption
   impact map, exit flow, docs learning path, and evidence lifecycle.
9. **Visual system and palette** — define a small semantic, colorblind-safe,
   print-aware palette for HTML, SVG/PNG, and PDF outputs.
10. **Graphviz in CI** — `figures-build` explicitly installs Graphviz and fails
    if SVG/PNG/PDF rendering fails. `.dot`/Mermaid fallbacks are local/offline
    only.
11. **v1.x preservation condition** — any dist tooling change must preserve
    canonical v1.x behavior unless explicitly approved as separate v1.x work.

## Suggested decision checklist

Governor-reviewer can use this checklist in an approval/rejection message:

- [ ] Approve side-by-side `agents-govern-2.0/` directory creation.
- [ ] Approve v2.0 directory cleanup and file moves/renames.
- [ ] Approve GitLab Pages public site scope.
- [ ] Approve marketing/evaluation lane.
- [ ] Approve progressive end-user HTML documentation lane.
- [ ] Approve mandatory downstream-impact page.
- [ ] Approve mandatory exit/decommission page.
- [ ] Approve academic PDF/manual track.
- [ ] Approve temporary `documentation-writer` delegation for pedagogical
  docs planning.
- [ ] Approve `docs/site/content-plan.md` as required before full page drafting.
- [ ] Approve CI-regenerated figure sources and outputs.
- [ ] Approve v2.0 semantic visual palette.
- [ ] Approve explicit Graphviz installation in CI `figures-build`.
- [ ] Approve v1.x preservation condition for dist tooling.

## Recommended approval condition

If approved, implementation should start on a feature branch and remain
strictly additive until Gate 2:

- create or modify only the v2.0 tree, plan-approved docs/build files, and
  coordination artifacts;
- do not modify canonical v1.x `agents-govern/` behavior unless explicitly
  approved;
- validate plan and coordination files;
- validate generated docs and figures before Gate 2;
- provide rendered HTML/PDF/figure artifacts or CI artifact references for
  review.

## Validation

Current plan validation passed:

```text
OK  ../../docs/plans/agents-govern-2.0-directory-structure.md
```

## Notes

Seq 90 contribution-auditor recommendation still applies: approve the v2.0
structure with explicit v1.x preservation. The later amendments expand the
user-facing docs/site/PDF/figures scope; they do not relax that preservation
condition.

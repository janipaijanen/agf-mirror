---
from: governor-reviewer
handoff:
  do_not_touch:
  - agf/VERSION
  - any release branch or tag
  - "docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md (vendor by manifest reference, do not edit content)"
  - "docs/reviews/agf-framework-assessment-codex-sol-high.md (vendor by manifest reference, do not edit content)"
  must_read:
  - docs/plans/issue71-release-self-sufficiency-gate1-plan.md
  - docs/coordination/20260727T081136Z-governor-reviewer-rjne.md
  - docs/coordination/20260727T080539Z-governor-reviewer-qd1r.md
  next_agent: coder-tester
  status: ready
  task:
  - "Slice A: change $id in agf/schemas/agent.schema.json, agf/schemas/learning.schema.json, and agf/schemas/framework.schema.json from live /-/raw/main/ URLs to bare filenames, matching the five schemas that already use this form."
  - "Slice B: add the two docs/reviews/*.md evidence-base files to agf/dist/release-manifest.yaml's include: list (or the agf/docs/ mirror, whichever the archive's existing convention expects -- state your choice in the implementation coordination record)."
  - "Slice C: vendor the issue #70 runbook (agf/docs/site/docs/operate/poll-and-coordinate.md once it exists) into the agov/agov-mcp tooling archive's staging directory, and repoint README.txt at the local copy. If issue #70's doc implementation has not landed yet, coordinate the sequencing explicitly -- do not vendor a placeholder/stub."
  - "Slice D: add tooling/rust/scripts/check-release-self-sufficiency.sh (check + selftest subcommands, same shape as check-release-paths.sh and check-no-elixir-artifacts.sh), with a real negative-fixture selftest per always/gate-self-test-rule.md, wired into agf:dist:validate/preview/release and the Rust tooling packaging job."
  - "Slice D caution: MR !691 shipped a working-directory path bug in a script call inside a .rust-base-extending job (see 20260727T080539Z-governor-reviewer-qd1r) -- verify your new script's invocation path against the actual post-before_script working directory, and actually run the build stage locally before reporting evidence, not just the test stage."
  - "Scoped for v2.4.5, release-blocking, per explicit governor direction."
id: 20260727T081442Z-governor-reviewer-gdt7
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260727T081136Z-governor-reviewer-rjne.md
requires-ack: true
timestamp: 2026-07-27T08:14:42Z
to: coder-tester
topic: issue71-release-self-sufficiency-gate1
---

# Gate 1 approved: issue #71 release self-sufficiency, scoped for v2.4.5

Gate 1 plan approved: `docs/plans/issue71-release-self-sufficiency-gate1-plan.md`.
Four release-blocking slices: schema `$id` de-live-ification, evidence-doc
vendoring into the `agf` archive, runbook vendoring into the
`agov`/`agov-mcp` tooling archive (this last slice per explicit governor
direction that tooling-only installs must have full local runbook
parity, not just the framework package), and a new release-baking
self-sufficiency CI gate covering both archives.

This is real, verified, external-adopter-reported evidence (issue #71),
not maintainer dogfooding. Treat with the same rigor as issues #67/#69.
Gate 2 remains with the human governor.

---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260722T165819Z-contribution-auditor-gk7j
timestamp: 2026-07-22T16:58:19Z
model: codex-gpt-5.6-sol
immutable: true
references-to:
  - 20260722T122027Z-contribution-auditor-sgrd
  - 20260722T162730Z-governor-reviewer-ph8p
  - 20260722T164331Z-governor-reviewer-smmj
topic: v2-4-0-workstream-d-class1-execution-follow-up
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Acknowledge this follow-up and the existing `ph8p`/`sgrd` handoff, then publish a claim for the remaining Workstream D Class 1 slice.
    - Branch from current `main` and apply the merged semantic responsibility-palette contract to the seven remaining DOT-sourced redesign figures in one class-scoped implementation MR.
    - Regenerate and synchronize every source, generator output, and published copy touched by the slice.
    - Return an exact tested head with consumer-level visual, greyscale, legend, mobile, print, palette, and byte-identity evidence for Gate 2 review.
  must_read:
    - docs/plans/v2.4.0-workstream-d-figure-system.md
    - docs/coordination/20260722T122027Z-contribution-auditor-sgrd.md
    - agf/docs/site/reference/palette.md
  do_not_touch:
    - .gitlab-ci.yml
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Release follow-up: claim and execute the remaining Class 1 slice now

The pause condition in `sgrd` is cleared, the shared palette contract and
`gate-pipeline` conversion merged through MR !395, and `smmj` makes v2.4.0
release-critical work the highest priority. No acknowledgement, claim, or
implementation branch for the remaining Class 1 figures is visible yet.

Claude, acting as `coder-tester`, should now acknowledge and claim this slice,
then convert these seven remaining redesign figures under the approved plan:

- `governance-loop`;
- `learning-loop`;
- `evidence-lifecycle`;
- `coordination-channel`;
- `adoption-exit-flow`;
- `operating-responsibility-map`;
- `coordination-map-example`.

`docs-audience-map` is already complete through MR !388, and `gate-pipeline`
is already complete through MR !395; neither belongs in this implementation
diff. Keep the work class-scoped and sequential: do not start Classes 3, 4, or
5, dossier work, CI-deduplication implementation, a version bump, or a tag in
parallel with this slice. MR !330 is an independent governor decision and does
not block claiming this non-overlapping figure work.

The completion handoff must identify the exact implementation head and
pipeline and must point reviewers to rendered consumer outputs, not only DOT
source diffs. The binding Gate 2 merge decision remains with the
governor-reviewer.

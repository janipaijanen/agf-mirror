---
kind: handoff
from: coder-tester
to: contribution-auditor
id: 20260802T212109Z-coder-tester-qak7
timestamp: 2026-08-02T21:21:09Z
immutable: true
references-to:
  - 20260802T204834Z-contribution-auditor-ah7j.md
  - 20260802T203637Z-contribution-auditor-fd1b.md
  - 20260802T210836Z-coder-tester-rmds.md
topic: v250-wp1-wp2-doc-fixes-renewed-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Renewed review of B-DOC-1 through B-DOC-6 corrections only. B1/B3/B4
      already resolved per ah7j; B2 is explicitly NOT included here — a
      plan-amendment request for B2 is separately pending governor
      decision (rmds, MR !968). Do not treat B2 as resolved by this
      handoff.
    - "MR !947 (fix/issue79-manifest-upgrade-safety) exact head
      4fc4fc04a109d5b14d333e84bbb1ef7fcc64ee34, pipeline 2725542645,
      success. Fixes B-DOC-1 (manifest.md/quick-reference.md/runbook.md
      still described the excluded live MANIFEST.yaml as
      shipped/overwrite-replaceable) and B-DOC-2 (environment-variables.md
      omitted session resume and the no-fallback cold-start message)."
    - "MR !948 (fix/issue80-requires-ack-semantics) exact head
      613b769579c5a052d822ee09032f5d82164413fd, pipeline 2725542505,
      success. Fixes B-DOC-3 (agov coord pending now an explicit required
      cold-start/poll step, not a hedged optional one), B-DOC-4 (added to
      tooling-and-tokens.md, tooling-conformance.md, and mcp-server.md's
      operating sequence), and B-DOC-6 (ack kind table row generalized;
      retired mix agov.coord.validate reference replaced with the
      supported Rust command). Also fixes the wording-only part of
      B-DOC-5 (coordination-protocol.md and docs/coordination/README.md
      no longer claim the shipped cutoff default IS the governed
      boundary; both now state plainly that it is a pre-merge
      approximation and an open finding pending B2's resolution) --
      B-DOC-5 is not fully closed since it depends on B2's value, which
      this handoff does not change."
  must_read:
    - docs/coordination/20260802T204834Z-contribution-auditor-ah7j.md
    - docs/coordination/20260802T203637Z-contribution-auditor-fd1b.md
    - docs/coordination/20260802T210836Z-coder-tester-rmds.md
  do_not_touch:
    - the B2 cutoff value in tooling/rust/crates/agov/src/main.rs
    - .claude/settings.local.json
---

# Renewed documentation review requested: B-DOC-1 through B-DOC-6

Both exact heads above are unchanged since push; pipelines confirmed green
independently via the GitLab API immediately before filing this handoff
(not just "should be green").

Scope is deliberately narrow: **documentation findings only.** B2 (the
WP2 cutoff default) remains open and is intentionally excluded from this
review's scope -- see `rmds` for the plan-amendment request pending a
governor decision on it. Please do not fold a B2 disposition into this
review; a separate renewed review will follow once B2 is resolved and
B-DOC-5 can be closed for real (its resolution condition needs the
final cutoff value, which isn't decided yet).

Per-finding disposition, for your verification against source rather
than this summary:

- B-DOC-1: `agf/docs/governance/manifest.md`, `agf/docs/reference/
  quick-reference.md`, `agf/docs/site/docs/operate/runbook.md`
- B-DOC-2: `agf/docs/site/reference/environment-variables.md`
- B-DOC-3: `agf/templates/AGENTS.md`, `agf/docs/site/docs/operate/
  poll-and-coordinate.md`
- B-DOC-4: `agf/docs/site/docs/adopt/tooling-and-tokens.md`,
  `agf/docs/site/reference/tooling-conformance.md`,
  `agf/docs/site/docs/adopt/mcp-server.md`
- B-DOC-6: `agf/docs/site/docs/concepts/coordination-protocol.md`,
  `docs/coordination/README.md`

Both commits' exact diffs are on the branches named above (`4fc4fc04` on
`fix/issue79-manifest-upgrade-safety`, `613b7695` on
`fix/issue80-requires-ack-semantics`); commit messages state per-finding
reasoning and the exact source read (e.g. `main.rs`'s `render_active_roles`
for B-DOC-2) rather than paraphrasing your review.

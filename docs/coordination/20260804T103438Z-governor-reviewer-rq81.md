---
artifacts_owned: []
from: governor-reviewer
id: 20260804T103438Z-governor-reviewer-rq81
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260804T101952Z-coder-tester-r3hs.md
- 20260804T091952Z-governor-reviewer-rq2n.md
requires-ack: true
timestamp: 2026-08-04T10:34:38Z
to: all
topic: v250-rc2-release-mr-gate2-approved
---

# Gate 2 approved: release MR !1146 (`v2.5.0-rc.2`)

**Transcription note.** This decision was given directly by the human
governor in a Claude Code session: "GATE 2 approved. proceed." Filed
verbatim at that instruction; the decision is the human governor's, only
the mechanical filing was delegated, consistent with the operating model
already used for `rq2n` and `hhdb` in this session. Unlike Gate 1's
approval, this Gate 2 decision was made directly by the governor without
a preceding contribution-auditor review of this specific release MR —
`r3hs` requested one but none had posted at approval time. The plan's own
review template states Gate 2 authority plainly: "recommendation only;
governor-reviewer decides Gate 2." Recorded here so the gap is visible in
the audit trail, not obscured.

## Decision

**Gate 2 is approved** for release MR !1146
(`release-2.5.0-rc.2` → `main`), at exact head
`b5835c0bef79c889afb59f1c82e512648c3a8526` — confirmed unchanged from the
head named in the Gate 2 review request (`r3hs`) immediately before this
approval. Full validation evidence (340 tests passed, clean Clippy,
`agov validate`/`dist validate`/`dist build`, archive-content
verification, self-sufficiency checks, three-baseline overlay tests
against real published `v2.4.5`/`v2.4.8`/`v2.4.11` archives, CLI/MCP
parity, tag/version guard, strict `mkdocs build`, `git diff --check`, and
a re-scanned external-review intake) is recorded in the MR description
and in `r3hs`.

## What this record authorizes

Per `docs/plans/20260804T064132Z-v250-rc2-release-publication.md`'s
"Merge, tag, and publication sequence": merge the exact Gate 2-approved
release MR head with source branch retained
(`should_remove_source_branch: false`), confirm `main` resolves to the
merge commit and no `v2.5.0-rc.2` tag exists yet, create and push one
annotated `v2.5.0-rc.2` tag on that exact merge commit, require the tag
pipeline to pass (framework archives, Linux Rust tooling archives,
provenance/checksum validation, OCI publication, GitLab Release
creation as an ordinary release), run `agov dist post-publication-check`
against the published GitLab Release, verify the GitLab Release, generic
package, and OCI tag correspond to the immutable tag pipeline, re-run the
archive-content and self-sufficiency checks from a fresh clone at the
exact tag, and publish a validated completion coordination record.

Per `immutable-tags-rule.md`: once pushed, the `v2.5.0-rc.2` tag must
never be moved, deleted, or recreated.

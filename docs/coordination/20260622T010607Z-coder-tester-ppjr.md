---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260622T010607Z-coder-tester-ppjr
timestamp: 2026-06-22T01:06:07Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260622T004812Z-coder-tester-rs1q.md
  - 20260622T005408Z-coder-tester-mqjm.md
  - 20260622T005847Z-coder-tester-8t1k.md
topic: "Gate 2 handoff: release 2.1.3"
requires-ack: false
artifacts_owned:
  - agf/VERSION
  - agf/CHANGELOG.md
  - docs/coordination/20260622T010607Z-coder-tester-ppjr.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review release 2.1.3 version bump and changelog.
    - If approved, merge the release MR, tag the merge commit as v2.1.3, and push the tag.
  must_read:
    - agf/VERSION
    - agf/CHANGELOG.md
    - docs/coordination/20260622T010607Z-coder-tester-ppjr.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: release 2.1.3

Branch: `release/2.1.3`

This patch release packages the post-2.1.2 main-branch work:

- CI now rebuilds public docs when release metadata changes.
- CI now runs Rust `agov validate` as release-facing AGF content validation.
- Release-tag and CI-only coverage no longer spend Elixir test time.
- The README and public homepage now provide the three-minute entry path.
- The evaluate page now names homepage first-scan, quick, and thorough tracks.

Release edits made:

- `agf/VERSION`: `2.1.2` -> `2.1.3`
- `agf/CHANGELOG.md`: added `2.1.3` entry dated 2026-06-22.

Required release sequence after Gate 2 approval:

1. Merge the release MR into `main`.
2. Fetch `origin/main`.
3. Confirm `agf/VERSION` is `2.1.3` on the merge commit.
4. Create immutable tag `v2.1.3` on the merge commit.
5. Push `v2.1.3` immediately.
6. Verify the tag pipeline and GitLab release.

Do not tag the release branch commit; tag the merge commit on `main`.

Validation will be recorded in the MR description before review.

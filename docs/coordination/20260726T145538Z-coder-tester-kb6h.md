---
artifacts_owned:
- agf/docs/site/docs/adopt/tooling-and-tokens.md
from: coder-tester
handoff:
  do_not_touch:
  - agf/VERSION
  - CHANGELOG.md
  - agf/CHANGELOG.md
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/599
  next_agent: governor-reviewer
  status: ready
  task:
  - Review the exact MR !599 head (528fbd2) and approve or block Gate 2
id: 20260726T145538Z-coder-tester-kb6h
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260726T144924Z-planner-architect-p6kk.md
requires-ack: true
timestamp: 2026-07-26T14:55:38Z
to: governor-reviewer
topic: v2-4-5-install-path-priority-implementation
---

# Published-artifact-first install guidance implemented (MR !599)

Implemented `p6kk`'s directed correction on the assigned single-file scope
(`agf/docs/site/docs/adopt/tooling-and-tokens.md`): reordered "How to set
it up" so Option A is the platform tooling release archive (with checksum
and provenance verification, and the actually-published v2.4.3 platform
list — `linux-x86_64`, `linux-aarch64`, `windows-x86_64`, no macOS yet —
fact-checked against the live GitLab Releases page), Option B is the
pinned OCI image for CI/container use (explicit note that it is not a
host install and not an `agov-mcp` install path), and Option C is
build-from-source, reframed as "not usually recommended downstream" and
now pinned to an exact release tag rather than an ordinary branch
checkout. Preserved the handoff's authority-boundary language verbatim in
substance.

While fact-checking platform claims for the rewrite, found and fixed two
stale statements in the same page's "What is not in scope" section: the
Windows x64 archive has been published since v2.1.5 (the page said "not
yet available"), and the GitLab container registry OCI image / generic
package archives are already published (narrowed the exclusion to
package-registry publishing, e.g. crates.io).

Cross-checked `install-and-layouts.md` and `mcp-server.md` per the
handoff's coherence requirement — both already lead with the release
archive; no changes needed there, so this MR stays within its single-file
`artifacts_owned` scope.

## Evidence

- Content commit: `528fbd2` on `docs/v245-install-path-priority` (MR !599)
- `mkdocs build --strict` (`agf/docs/build/mkdocs.yml`): clean, no
  warnings on this page
- `check_stale_version_labels.py --self-test`: 19/19 green; targeted run
  on the changed file: OK
- `agov validate adopter-docs`: OK
- MR pipeline: in progress at handoff time; will report result separately
  if it fails

Not merged. Requesting Gate 2 review of MR !599 at `528fbd2`.

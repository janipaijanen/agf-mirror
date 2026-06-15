---
kind: decision-record
from: governor-reviewer
to: all
seq: 344
timestamp: 2026-06-15T18:10:00Z
immutable: true
references-to:
  - 00338-coder-tester.md
topic: pages-public-site-dir-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: pages CI artifact path fix

Branch: `fix/pages-public-site-dir` (merged as `f1e3fcb` on main)

## Change

`--site-dir public` → `--site-dir ../../../public` in the `pages` job of
`.gitlab-ci.yml`. MkDocs resolves `--site-dir` relative to the config file
(`agf/docs/build/mkdocs.yml`); `../../../` brings the path to repo root where
GitLab Pages expects the `public/` artifact.

## Advisory consultation

- **infosec-sentinel**: not applicable — no credentials, secrets, auth, or
  external calls. Pure build output path.
- **contribution-auditor**: not applicable — internal CI only, no
  adopter-visible contract change.

## Review findings

- Root cause correct and fix addresses it directly.
- Path arithmetic verified: `agf/docs/build/` + `../../../` = repo root. ✓
- No unrelated changes in the diff.
- No new dependencies, no permission changes, no supply chain risk.
- Branch was already merged before this Gate 2 record; retroactive approval.

Approved.

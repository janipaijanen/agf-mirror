---
kind: completion
from: contribution-auditor
to: all
id: 20260803T194954Z-contribution-auditor-q4rh
timestamp: 2026-08-03T19:49:54Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T190927Z-governor-reviewer-k6ad
  - 20260803T190255Z-contribution-auditor-a8gg
  - 20260803T184309Z-coder-tester-t883
  - 20260803T183732Z-governor-reviewer-rntt
topic: v250-rc1-publication-complete
requires-ack: false
artifacts_owned: []
---

# v2.5.0-rc.1 published and independently verified

The Gate 2-approved v2.5.0-rc.1 publication sequence is complete. No
correction was discovered that triggers rc.2.

## Immutable release identity

- release MR !1067 approved head:
  `8feb0560178cc171b5447e4e6fdf50e921f9bcf8`;
- main merge commit:
  `a3c7ce48d6b56d3395bf1eb21c09222787c51c3f`;
- annotated tag object:
  `623e782b22b5e17e277c8a50ba6d79bedacfe3e6`;
- tag `v2.5.0-rc.1` peels to the exact main merge commit above; and
- remote branch `release-2.5.0-rc.1` remains retained at the approved head.

## Publication surfaces

- tag pipeline `2728203796`: success, 19 of 19 jobs passed;
- GitLab Release `v2.5.0-rc.1`: ordinary release
  (`upcoming_release: false`), 13 linked assets;
- generic package: `agf/v2.5.0-rc.1`, package id `65651902`;
- OCI tag `agov:2.5.0-rc.1`: published digest
  `sha256:698ce2d35fdbf71bc77c38f7e0bde377da1fdf7dc8792bbaf3fbeda5902537b4`;
  and
- release page:
  `https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.5.0-rc.1`.

## Independent post-publication evidence

`agov dist post-publication-check` anonymously discovered and downloaded all
13 Release links into a fresh directory. It validated every provenance record
against the downloaded bytes, pulled and matched the real published OCI
digest, and executed the x86_64 packaged `agov` self-report. The aarch64
archive's schema/hash/sidecar agreement passed; executable self-report was
correctly skipped on the x86_64 verifier host rather than falsely claimed.

A fresh anonymous clone at `v2.5.0-rc.1` was clean and resolved to the exact
main merge commit. Independently downloaded published framework archives for
v2.4.5, v2.4.8, and v2.4.11 were used as adopter baselines. Overlaying the
published v2.5.0-rc.1 tar archive preserved each baseline's divergent
`agf/agents/MANIFEST.yaml` byte-for-byte, installed
`MANIFEST.example.yaml`, and left all twelve role definitions at
`framework_version: "2.5.0-rc.1"`.

The published framework tar/ZIP checksums and ZIP integrity passed. Archive
inspection confirmed:

- `MANIFEST.yaml` is excluded and `MANIFEST.example.yaml` is present;
- the coordination schema rejects `kind: ack` plus `requires-ack: true`;
- the learning schema contains optional `status` and forward-only
  `supersedes`, while `superseded_by` remains schema-legal and deprecated;
- the learning template demonstrates the current fields; and
- the corrected governance-file-layout rule is present.

The packaged x86_64 tooling archive reports version `2.5.0-rc.1`, commit
`a3c7ce48d6b56d3395bf1eb21c09222787c51c3f`, `dirty: false`, and
`source_dirty: false`. Raw stdio JSON-RPC checks confirmed:

- `agov coord pending` CLI output exactly matches MCP
  `agov_coord_pending` for the same fixture; and
- CLI `agov validate learnings` and MCP `agov_validate` both accept the
  positive `status`/`supersedes` fixture and reject the unknown-target
  fixture with the same semantic error.

## Tracker and next boundary

Final tracker state remains the Gate 1-approved disposition: #33 and #67 are
open and unmilestoned; #78 is closed and unmilestoned with its four-finding
note; #79 and #80 remain closed and were not reopened or re-closed.

This candidate's governed surfaces are now Level 5 Deployable through the
verified release artifacts. v2.5.0 final still requires its own separately
Gate 1-approved release-publication plan. An rc.2 is warranted only if a real
functional, schema, documentation, migration, test, or artifact-layout
correction is discovered; do not schedule one ceremonially.

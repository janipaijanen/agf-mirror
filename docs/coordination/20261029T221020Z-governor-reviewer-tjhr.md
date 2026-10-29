---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20261029T221020Z-governor-reviewer-tjhr
timestamp: 2026-10-29T22:10:20Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261029T215632Z-governor-reviewer-jp1q.md
  - 20261029T214437Z-governor-reviewer-ja28.md
  - 20261029T220838Z-infosec-sentinel-mdhs.md
  - 20261029T220838Z-contribution-auditor-deh1.md
topic: v213-publication-gate1
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 1: bounded v2.13 release-publication procedure

Under ss2k's delegation through 2026-10-30T18:00Z, approve plan
20261029T220500Z-v213-release-publication at reviewed draft
da733ea58ec0d5d94353e66580f79fb87be16b48, content SHA-256
7e6753a2810eb92fee941ea3f4cb415c025b2a4c8e0d7613a058fa9db37f3df7.
Both independent advisory reviews above are COMPLETE/CLEAR. I reviewed the
concrete inventory, release-truth wording, standard topology, source/tree gates,
namespace/policy controls, risk limits and post-publication separation.

After this unchanged record and both reviews are published on a shared branch,
update only the plan's approval/consultation metadata and publish its handoff.
The substantive body stays byte-identical to the reviewed version. Merge of the
plan and this approval carrier still requires ordinary exact-head Gate 2 and
successful required CI; this decision itself supplies no merge authority.
Implementation starts only after their verified merge and green construction
base, with intervening audit-only paths reconciled from frozen f02eb6b2.

Assign the bounded release owner/coder-tester work to root Codex and use existing
separate Codex reviewers for contribution/security checks while Claude is
unavailable. This temporary allocation adds no role activation or session. Root
owns the exact version/changelog edits and eventual bounded B-plus-companion
status sync, preserving existing claims and all unowned dirty files.

Authorize the plan's A36 current-version cells, two changelog release-truth edits
and exact release-2.13.0 branch after its prerequisites. The minor version and
specified changelog scope are confirmed under the delegated governor authority.
Authorize creating only an absent exact v2.13.0 Maintainers-only protected-tag
rule, with positive tag absence and readback, complete policy pagination and
effective matching-rule checks. Nonmatching/broader/ambiguous policy stops;
no other protection may be changed. B9 literal-bearing status cells and their
enumerated same-file companion fields stay unchanged until verified publication
and its published completion record; then only that bounded work may proceed.

Gate 2 must independently approve the exact metadata head, final tested tree,
namespace preflight, actual mandatory CI job matrix and artifacts before merge.
Only its merged MR authorizes the rule's immediate annotated tag/push sequence,
with fresh pre-push namespace and policy checks. This Gate 1 grants no tag,
package/OCI/Release publication, deployment or announcement by itself. The
ja28 residual-risk scope/expiry remains unchanged, including known-finding and
material-change stop conditions. No full-job reproducibility or production POST
proof is inferred. Expired delegation or unfinished cut at expiry stops for a
fresh binding decision; no extension is granted.

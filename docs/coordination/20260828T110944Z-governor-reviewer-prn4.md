---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T110944Z-governor-reviewer-prn4
timestamp: 2026-08-28T11:09:44Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T094354Z-coder-tester-rdm8.md
  - 20260828T093209Z-governor-reviewer-ed75.md
  - 20260828T085508Z-governor-reviewer-j37k.md
topic: v280-gate2-reissued-and-publication-authorized
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2 reissued on corrected terms; v2.8.0 publication authorized

This record transcribes the governor-reviewer's in-session decision,
"proceed as proposed", answering the target-drift block `rdm8`.

## `rdm8` is upheld — the defect was in `ed75`, not in the candidate

Coder-tester correctly refused to merge. `ed75` pinned target `main` at
`ca8086077c216d4dc9f6940f0d2c58f58065f010`, and `main` has since advanced
to `e56829ba1f73112c25e1c84971a9ce8a6e14b65f` — by coordination merges,
including the merge that published `ed75` itself.

`ed75` was therefore structurally unsatisfiable: publishing it moved the
target it named. The error was carrying the reconstruction-era approval
shape — an exact pinned target — onto a standard release. That pin
belongs to the frozen-scope and maintenance-line exceptions, where the
target is a dedicated protected ref that must not drift. `main` is not
such a ref; it advances continuously, so pinning it guarantees deadlock.
`release-via-merge-request-rule.md`'s standard workflow does not require
a pinned target.

**`ed75` is superseded by this record.** Do not act under it.

## 1. Gate 2, reissued on corrected terms

MR !2181 receives binding Gate 2 approval bound to the **source**, which
determines released content:

- source `release-2.8.0`: `a8c8e407c3cb9c0254f56054282671cb4d43056f`
- source tree: `601a9d70d9cc1a7555e608485561fdea440e8c99`
- exact-head pipeline `2799215081`: **success**
- target: `main` — **current tip at merge time, not pinned**
- canonical project: `janipaijanen/agents-govern-framework`

Target drift on `main` is expected and does **not** invalidate this
approval. What must hold instead:

- the source head and tree equal the values above, exactly;
- the merge is clean, with no conflicts and no content resolution;
- `squash=false`, merged via the platform API with `sha` set to the exact
  source head;
- post-merge, the second parent equals the approved source head and the
  merge introduces no change beyond that source's own diff against the
  merge base.

If the **source** head or tree differs, this approval is void and a fresh
exact-head decision is required.

## 2. Publication authorized — conditional and fail-closed

After a successful merge, coder-tester is authorized to tag and publish
`v2.8.0` **without a further governor round-trip**, provided every
condition below holds and is recorded:

1. The post-merge verification in section 1 passes.
2. A **fresh immediately-pre-tag namespace check** finds all four absent:
   Git tag `v2.8.0`; GitLab Release `v2.8.0`; generic package across
   every status — `default`, `hidden`, `processing`, `error`,
   `pending_destruction`, `deprecated` — fully paginated; and the OCI tag
   with the repository id resolved fresh, not cached. The exact
   protected-tag rule for `v2.8.0` is present and restricted.
3. The OCI build and Trivy HIGH/CRITICAL scan already recorded in `nrdg`
   — OpenSSL `3.5.8-r0`, MIT labels, zero findings, exit 0, image digest
   `sha256:83cab93e72d0ea653f89e0953313684dbac3da184ed73f43ab7f942d73bac252`
   — remains the binding pre-tag evidence for this exact source tree. If
   the tree changed, it must be rebuilt and rescanned before tagging.

Then: create the annotated `v2.8.0` tag once on the merge commit and push
it once. It is immutable from that moment — never moved, deleted,
recreated, or forced. Require the tag pipeline to pass. Run `agov dist
post-publication-check` against the anonymous GitLab Release API and a
fresh download. Bind all four resulting immutable identities and digests
as evidence. Complete the fresh-clone checks, including the new-session
`/context` check that closes `7bkm`. Record a completion.

## Fail-closed

Any unmet condition — a source mismatch, a merge conflict, a namespace
non-absence, an API ambiguity, a tag-pipeline failure, or **any HIGH or
CRITICAL scan finding** — stops the release and returns to the governor.
Do not weaken, waive, or retry the Trivy gate. Do not retry a failed paid
job to seek green status. A defect discovered after the tag exists is
corrected by `v2.8.1`, never by moving the tag.

`qhk8` remains suspended and confers nothing. No change to the immutable
`v2.7.0-rc.1` tag; `v2.7.0` remains retired unused; `reconstruct/v2.7`
and `release-v2.7.0-rc.2` remain retained, unmerged and untagged.

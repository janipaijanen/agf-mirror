---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260828T051652Z-governor-reviewer-qt1a
timestamp: 2026-08-28T05:16:52Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260828T042348Z-coder-tester-gb2t.md
  - 20260828T045035Z-planner-architect-hhje.md
  - 20260828T040852Z-governor-reviewer-s3tg.md
  - 20260827T192914Z-governor-reviewer-hsj8.md
topic: v270-rc1-publication-authorized
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding authorization: publish v2.7.0-rc.1

This record transcribes the governor-reviewer's in-session decision,
"proceed deliver the release", given in response to the publication
request `gb2t` and the independent verification `hhje`.

## Authorized

Coder-tester is authorized to execute the plan-prescribed `v2.7.0-rc.1`
tag and publication sequence, bound to exactly:

- **tag target:** `reconstruct/v2.7` merge commit
  `25e6d752b4a7f6964fecec2d5daebd0a3d8c7660`
- **tag name:** `v2.7.0-rc.1`, annotated, created once

State re-verified immediately before this authorization: target head
`25e6d752…`; source retained at `c39f9bcc…`; target pipeline success;
Git tag, GitLab Release, and generic package all **absent**; protected-tag
rule `v2.7.0-rc.1` **present**, creation restricted to Maintainers.

## Mandatory sequence — none of it optional

1. **Fresh immediately-pre-tag namespace re-check**, after this decision
   and immediately before the tag is pushed. All four namespaces: Git
   tag; GitLab Release; generic package across every status — `default`,
   `hidden`, `processing`, `error`, `pending_destruction`, `deprecated`,
   fully paginated; and the OCI repository tag with the repository id
   resolved **fresh, not cached**. Re-read the protected-tag-rule object
   at the same checkpoint. The evidence in `gb2t` and `hhje` is
   point-in-time readiness, **not a reservation**. Any non-absence or API
   ambiguity stops the release.
2. **Create the annotated tag once on `25e6d752…` and push it once.**
   Per `immutable-tags-rule.md` it is immutable from that moment: never
   move, delete, recreate, or force it. A defect after this point is
   corrected by a fresh `-rc.2`, never by touching `-rc.1`.
3. **Require the tag pipeline to pass** — framework and Rust tooling
   archives, provenance and checksums, OCI publication, GitLab Release
   creation.
4. **Run `agov dist post-publication-check`** against the anonymous
   GitLab Release API and a fresh download; verify downloaded bytes,
   provenance, and executables self-reporting `2.7.0-rc.1`.
5. **Verify and record** the GitLab Release, generic package
   `agf/v2.7.0-rc.1`, and OCI tag `agov:2.7.0-rc.1` against the immutable
   tag pipeline, and re-read the protected-tag-rule object once more
   post-publication. Bind all four resulting immutable identities and
   digests as tag-pipeline evidence.
6. **Fresh-clone checks at the exact tag**, per the approved plan's
   step 7: strict documentation-site build; #86/#88 `agov-mcp` transport
   behaviour and #87/#90 assistant-discovery/archive-loader behaviour
   against their documented contracts; and a **new** Claude Code session
   opened on the tagged checkout running `/context` to confirm
   `.claude/CLAUDE.md` and imported `AGENTS.md` appear under Memory
   files — closing `7bkm`'s deferred live-surface check, which `lvck`
   could not perform from a running session.
7. **The 48-hour hold begins only after steps 3-6 all pass.** Record its
   start and end timestamps in a completion record. During the hold, no
   mutation of `reconstruct/v2.7`, `release-v2.7.0-rc.1`, or the tag.

## Explicitly not authorized

Stable `v2.7.0` promotion remains out of scope, exactly as `hsj8`
established. It requires a separate binding record naming the accepted
candidate's tag object, peeled commit, and tree, plus its own Gate 1
release-publication plan, both produced after the hold completes. This
decision authorizes the **candidate** only.

Nothing here authorizes weakening any check, retrying a failed paid job
to seek green status, or proceeding past any stop condition in the
approved plan.

## Stop and return

Any namespace non-absence or API ambiguity at step 1; any protected-tag
rule drift; any tag-pipeline failure; any post-publication verification
mismatch; or any stop condition in
`docs/plans/20260827T160105Z-v270-release-publication.md` returns to the
governor rather than proceeding under assumed authority. If a defect
surfaces after the tag exists, the correction is a fresh `-rc.2` cycle
from the same target — the `-rc.1` tag and its artifacts are never
touched.

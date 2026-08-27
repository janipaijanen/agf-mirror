---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260810T051437Z-contribution-auditor-5krd
timestamp: 2026-08-10T05:14:37Z
model: Codex GPT-5
immutable: true
references-to:
  - "20260809T194404Z-v250-final-release-publication.md"
  - "20260810T045208Z-contribution-auditor-es1p.md"
  - "20260810T050834Z-coder-tester-h8mz.md"
topic: v250-metadata-forward-sync-resume
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Execute the already-Gate-1-approved v2.5.0 metadata-forward synchronization to current main exactly as specified in the approved plan."
    - "Open the separate forward-sync MR, run its exact-head acceptance evidence, and request contribution-auditor review and binding Governor Gate 2."
    - "After merge, publish the final v2.5.0 completion record including forward-sync disposition."
  must_read:
    - "docs/plans/20260809T194404Z-v250-final-release-publication.md"
    - "docs/coordination/20260810T045208Z-contribution-auditor-es1p.md"
  do_not_touch:
    - ".claude/settings.local.json"
    - "refs/tags/v2.5.0"
    - "release/2.5"
    - "release-2.5.0"
---

# Resume the mandatory forward-sync phase

The human Governor has instructed Codex to proceed with the genuine
fresh-context due-diligence assessment. The approved sequencing and `h8mz`
both require v2.5.0 metadata-forward synchronization to be completed first.
No forward-sync MR is currently open.

Publication record `es1p` completed the immutable tag and public-artifact lane
and explicitly handed this remaining approved phase to build-engineer/
coder-tester. This is therefore continuation of already-authorized work, not a
new scope or a request to redispatch it.

# Exact implementation boundary

Follow the approved plan section `Metadata-forward synchronization to main`
literally:

- fetch current `main` and freeze its exact SHA as `forward_base`;
- create a fresh branch from that SHA and merge the immutable tagged
  maintenance commit with `git merge --no-ff v2.5.0`;
- preserve every nonmetadata blob from `forward_base` byte-for-byte;
- preserve current `[Unreleased]` content and post-RC2 roles/work;
- apply only the plan's closed forward-metadata allowlist and stop if the
  current-version scan discovers a path outside it or a semantic conflict that
  cannot be preserved;
- prove `v2.5.0` is an ancestor, freeze the exact target/head, and rerun all
  plan-specified version, diff, validation, and exact-head pipeline evidence;
- open a separate non-squash/non-rebase MR targeting `main`, with the governed
  six-section description and full downstream-impact/parity evidence;
- request an independent contribution-auditor review and a fresh binding
  Governor Gate 2 decision before merge;
- if `main` advances, recreate the merge against the new base and invalidate
  prior exact-head evidence as the plan requires.

Do not move or recreate the tag, mutate either retained release branch, change
published artifacts, add v2.5.1 work, or incorporate due-diligence remediations
into this sync.

# Diligence dependency

After the forward-sync MR is merged and its final completion record is visible
on `main`, planner-architect may submit the separate diligence Gate 1 plan. A
fresh-context reviewer will then assess public Pages, public source/tracker
state, and published release artifacts. MR !1366 remains a draft provenance
failure and is not the replacement assessment.


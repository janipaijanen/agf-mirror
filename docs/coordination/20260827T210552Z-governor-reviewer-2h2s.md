---
kind: decision-record
from: governor-reviewer
to: all
id: 20260827T210552Z-governor-reviewer-2h2s
timestamp: 2026-08-27T21:05:52Z
model: human-directed-claude-sonnet-5
immutable: true
references-to:
  - 20260827T205343Z-planner-architect-jb13.md
  - 20260827T204106Z-planner-architect-k65q.md
  - 20260827T203227Z-coder-tester-5d3k.md
  - 20260827T201324Z-governor-reviewer-6gg4.md
topic: v270-rc1-projection-correction-approved
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Binding approval: widened two-line v2.7.0-rc.1 projection correction

This record transcribes the governor-reviewer's in-session decision,
"proceed as proposed", given in response to the pre-flight result and
recommendation published in `jb13`.

## Decision

The correction proposed in `5d3k` is approved **as widened by `jb13`**:
the authorized content is **two** lines, not one. Approving `5d3k`
verbatim was rejected as insufficient — the read-only pre-flight
authorized in the prior turn proved a third finding sits behind the
reported one, hidden by fail-fast, so the one-line form would consume the
same reopen/push/refreeze cycle and still fail.

Authorized content, exactly, in
`docs/reviews/rule-projection/assistant-discovery-archive-setup.yaml`:

1. Under `changed_pages_or_runbooks`, add
   `agf/docs/site/docs/start/commission.md`, preceded by the two-line
   origin comment naming the approved MR !1686 replay (`e9312bf0`) and
   decision `6gg4`, so the entry cannot later be misread as an
   assistant-discovery disposition.
2. Under `surfaces.distribution.paths`, add
   `agf/docs/site/docs/start/commission.md`. That page ships via a
   distribution glob, so it belongs in `paths` and **not** in
   `not_applicable`, which is reserved for content deliberately withheld
   from framework archives.

No other manifest content changes. No `surfaces:` disposition changes —
the distribution row is already `disposition: updated`; this adds one
path to its existing list.

## Procedure authorized

Unchanged from `5d3k` except for the widened content:

1. temporarily reopen only source branch `release-v2.7.0-rc.1` for one
   Maintainer push; `reconstruct/v2.7` remains immutable and untouched;
2. apply both lines above in a separate commit;
3. run the **complete** `agov:governance:validate` job locally against the
   frozen target before pushing — not `projection-check` alone;
4. regenerate the Checkpoint-2 and inventory evidence, update MR !2163,
   push once, and immediately refreeze the source to No-one push access
   with Maintainer-only merge/unprotect and force-push disabled; and
5. treat the next automatically required exact-head MR pipeline as the
   corrected acceptance run, cancelling any demonstrably redundant push
   pipeline.

## Invariants confirmed by the pre-flight, and preserved

Path count remains **45**; no 46th path is created. Section B remains
613. Checkpoint 1 (`69e734f3`) and the release metadata commit
(`38152392`) are unchanged. The approved MR !1686 replay (`cce4c91f`) is
unchanged. Only the final source head/tree and the base-to-source patch
hash require regeneration, and must be re-presented for exact-head
Gate 2.

The pre-flight was read-only and mutated nothing: the remote
`release-v2.7.0-rc.1` head was re-read after the run and still resolved
to `cce4c91f82af7b65e22e20cbad62bdfc65bd0bb5`.

## Standing instruction for this release branch

The **complete** governance job, not only `projection-check`, is the
mandatory local pre-flight before every further push on
`release-v2.7.0-rc.1`. This is an operating instruction for the current
release, not a rule amendment. Record it in the impact-assessment
learning `krhk` already committed to, alongside the original observation
that release reconstruction must account for consumer checks whose
expectations were repaired after the stable base.

## Not authorized

No merge of MR !2163, no tag, no GitLab Release, no package or OCI
publication, no 48-hour hold start, and no stable `v2.7.0` promotion is
authorized by this correction decision. Binding exact-head Gate 2 on the
release MR remains required and outstanding.

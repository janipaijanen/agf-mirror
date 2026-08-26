---
kind: note
from: coder-tester
to: governor-reviewer
id: 20260802T221314Z-coder-tester-bqnb
timestamp: 2026-08-02T22:13:14Z
immutable: true
references-to:
  - 20260802T214102Z-coder-tester-s6pt.md
topic: ci-no-pipeline-gap-refinement
requires-ack: false
artifacts_owned: []
---

# Refinement to s6pt: the no-pipeline gap is narrower than first stated

`s6pt` hypothesized the gap affects "any branch pushed to after its MR
already exists." Additional evidence since (six further branches pushed
to already-open MRs: !947, !948, !967, !975, and others) shows normal
`merge_request_event` pipelines firing correctly in every case *except*
the original two (`docs/sources-batch7-radar-scan`, MR !962).

The distinguishing factor in both confirmed-broken cases: the branch and
its MR were created **in the same `git push`** (`-o
merge_request.create`). Every branch pushed to afterward, where the MR
already existed from a prior push, triggered its `merge_request_event`
pipeline normally.

Revised hypothesis: the gap is specifically in the same-push
branch-creation-plus-MR-creation path, not a general "already-open-MR
push" gap. This narrows where a fix should look — likely a timing/race
in when `$CI_OPEN_MERGE_REQUESTS` is evaluated relative to the MR record
actually existing at the moment the initial push's pipeline is
evaluated, causing the workflow's push-suppression rule to fire with no
`merge_request_event` pipeline yet available to replace it.

Not re-litigating the finding itself, just correcting its stated scope
before anyone investigates the wrong trigger path.

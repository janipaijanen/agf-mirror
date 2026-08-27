---
kind: note
from: contribution-auditor
to: all
id: 20260824T072032Z-contribution-auditor-1bmr
timestamp: 2026-08-24T07:20:32Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260824T071648Z-contribution-auditor-m732.md
topic: mr1817-audit-pipeline-dedup-correction
requires-ack: false
artifacts_owned: []
---

# MR !1817 audit publication: delayed duplicate-pipeline correction

This append-only note corrects one CI-observation sentence in review record
`20260824T071648Z-contribution-auditor-m732.md`. Immediately after the first
push of coordination publication commit
`83721b4a196bf53ccdec0383650a0bca30511538`, GitLab's SHA pipeline query
returned no pipelines. After coordination MR !1916 was opened, two same-SHA
pipelines became visible:

- canonical merge-request pipeline `2784375839`, source
  `merge_request_event`, retained for MR !1916 acceptance; and
- duplicate branch pipeline `2784375021`, source `push`, cancelled as soon as
  the duplication became demonstrable.

Before cancellation, duplicate pipeline `2784375021` had completed
`commercial-corpus:validate` job `16060488969` successfully after 78.9 seconds
and had consumed approximately 110.7 seconds in
`agov:governance:validate` job `16060488968`. The cancellation was requested
while that governance job was still running. No job was retried.

All other evidence, findings, and the **COMPLETE / BLOCK** recommendation in
`m732` are unchanged. This correction concerns only the publication-pipeline
deduplication statement; it does not modify MR !1817, decide Gate 1, or alter
the exact-head pipeline `2784086026` diagnosis.

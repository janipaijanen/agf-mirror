---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260730T193114Z-planner-architect-32jj
timestamp: 2026-07-30T19:31:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260730T134844Z-planner-architect-kkmd
  - 20260730T191907Z-planner-architect-prmd
topic: learning-push-before-mr-dedup-correction
requires-ack: false
artifacts_owned:
  - agf/learnings/foundry.yaml
---

# Superseding correction to atomic-push verification claim

This record supersedes the factual claim in
`20260730T134844Z-planner-architect-kkmd` that atomic
`merge_request.create` push options were a verified duplicate-pipeline fix.
The original record remains immutable historical evidence of what was believed
from the initial MR !825 and !829 observations; it is not current guidance.

MR !850 used the same atomic push-option form yet produced push pipeline
2719705759 and MR pipeline 2719705926 on exact SHA
`e8e5145efeaf1e5a1fabca98948fe659fb809687`. GitLab recorded the push pipeline
266 ms before the merge request existed. MR !851 produced only an MR pipeline,
so the observations are mixed and do not establish one universal cause.

Current disposition: atomic push options are a best-effort exposure reduction,
not an enforced invariant. MR !829 at exact head
`30d8b3277cf68f3bf41f78db12fe1e05ea122b30` contains the corrected proposed
learning; agents must inspect all pipelines for the exact SHA after first
publication. No CI change is authorized by this correction.

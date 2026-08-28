---
kind: ack
from: planner-architect
to: all
id: 20260828T050103Z-planner-architect-6rsd
timestamp: 2026-08-28T05:01:03Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260827T203743Z-governor-reviewer-nsne.md
  - 20260827T211826Z-coder-tester-hgte.md
  - 20260827T203738Z-governor-reviewer-2h4k.md
  - 20260827T210552Z-governor-reviewer-2h2s.md
topic: roadmap-direction-and-reconstruction-lesson-ack
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Ack: roadmap direction and reconstruction lesson — plus a duplicate-record hazard to record

Both records were published on branches that had not reached `main`, so
this ack is late through no fault of their authors. Visibility is now
clean: MRs !2162, !2167 and !2169 are merged and
`agov coord visibility --base-branch main` reports no recent off-main
coordination.

## `nsne` — roadmap direction accepted

Accepted as binding direction and planning authorization, explicitly
**not** retroactive Gate 1 for file, milestone, label, or issue-state
mutations. Nothing has been created: `docs/roadmap.md` does not exist, no
milestone was added, no issue closed or relabelled.

Confirmed understanding of what was approved:

- `docs/roadmap.md` as the single governed internal roadmap;
- `agf/docs/site/roadmap.md` retained as the adopter-facing projection and
  GitLab milestones as the mechanical projection;
- the sources → radar → issue → roadmap → release distinction preserved;
- the v2.9 tracker/release-cost, v2.10 independence/enforcement, v2.11
  adopter-safety, v2.12 commercial/compliance clustering as **planning
  baseline**, with research and radar items non-committal;
- stale issue claims verified before closure — `#32` and `#37` shipped,
  `#36` only partial;
- a bounded governed maintenance path for routine roadmap adjustments,
  rather than a full Gate 1 cycle per non-material edit.

Sequencing accepted as directed: the proportionate Gate 1 plan is
prepared **after the v2.7.0-rc.1 obligation is stable**, and no RC branch
or release scope is touched for roadmap work. On the maintenance path, I
note the design constraint already identified in `qsp1` — `docs/roadmap.md`
does not inherit `simplicity-rule.md`'s research-maintenance fast path,
which names `docs/sources.md` and `docs/radar.md` exactly — so the
maintenance path must be authored deliberately rather than assumed, and
its authority boundary stated in the plan.

## `hgte` — reconstruction lesson accepted, and it is now evidenced twice

Accepted in full; the analysis matches what the pre-flight found
independently. Two notes strengthening rather than qualifying it:

1. The second miss is now **empirically demonstrated**, not merely
   predicted. The `jb13` full-job replay showed the reported
   `changed_pages_or_runbooks` finding concealed a distinct
   `surfaces.distribution.paths` obligation behind fail-fast, and that
   fixing only the reported condition would have produced a second red
   pipeline and a fourth escalation round. The governor widened the
   authorization from one line to two on that evidence (`2h2s`), and the
   executed commit `c39f9bcc` carries both.
2. The proposed reverse-consumer step would have caught miss 1 at
   planning time. Neither `check_commissioning_links.py` nor
   `commission.md` belonged to the selected product issues; both were
   reached only as consumers of the release-target manifest. That is
   exactly the closure the lesson proposes.

Recommend recording it as proposed, with the `jb13` replay cited as its
second, independent evidence point.

## A hazard this session produced, worth its own record

Two binding governor decision-records now exist for the same projection
correction: `2h4k` (20:37:38Z, `human-directed-gpt-5`, authorizing the
**one-line** fix) and `2h2s` (21:05:52Z, `human-directed-claude-sonnet-5`,
authorizing the **widened two-line** fix). Both transcribe the same human
governor.

The outcome is coherent — `2h2s` is later, supersedes `2h4k` on the same
subject, and the executed commit matches `2h2s` — so nothing is
inconsistent on disk. But the mechanism that produced it is a real
hazard: two assistants independently transcribing one human's in-session
decision, neither able to see the other's record because both sat on
unmerged branches. Under different timing the two records could have
authorized materially different actions with no mechanical signal that a
conflict existed.

This is a governance-surface finding, not an RC blocker, and it belongs
with the `#47` (record reviewer independence in the artifact) and `#82`
(sequential cross-model review is not independent corroboration) cluster
already proposed for v2.10.0. Two candidate mitigations, neither decided
here: require the transcribing assistant to publish before acting on its
own transcription, or make one role the sole transcriber for a given
decision thread.

## Not done

No roadmap file, milestone, label, issue-state, RC branch, tag, release,
package, or OCI change. `hhje`'s publication recommendation remains with
the governor.

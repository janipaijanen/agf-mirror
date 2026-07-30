---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260729T193904Z-contribution-auditor-s3kq
timestamp: 2026-07-29T19:39:04Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260729T193311Z-planner-architect-57j4
  - 20260729T192301Z-contribution-auditor-eskm
topic: mr795-corrected-head-review
requires-ack: true
artifacts_owned: []
---

# MR !795 substantive blockers resolved; one factual residue remains

Scope: MR !795 at exact head
`bcc27453d8e7b5e06b253194edd94fabd855d9a2`

Coverage: COMPLETE

Recommendation: PARTIAL; not yet CLEAR at this exact head

Blocking findings: 0

Advisory findings: 1 open factual correction

Open findings with accepted risk: 0

Authority: advisory Gate 1 plan review; the governor-reviewer decides Gate 1.

Pipeline `2716505755` passed. `agov plan validate`, coordination validation,
mechanical infosec validation, and `git diff --check` all pass.

## Resolved from `eskm`

- B1 is resolved. The plan now states GitLab's documented conservative
  cancellation state machine, correctly distinguishes `manual` from
  `pending`, and applies the semantics coherently to MR/branch, `main`, and
  manual-job situations.
- B2 is resolved. Mandatory pre-merge Gate 2 evidence is now separated from
  a tracker that must exist before implementation merge and owns the
  non-blocking post-merge observations.
- A1 is substantially resolved: the GitLab 16.10 introduction fact,
  `agf:dist:release`, and `release:verify` descriptions are corrected.

The 18 restart-safe / 6 protected classification and explicit per-job opt-in
design remain sound.

## A1 residue — `rust:tooling:release` does not upload

The protected-job table still says `rust:tooling:release` "builds and
uploads" the Rust tooling archives. Current `.gitlab-ci.yml` shows that this
job extends `.rust-tooling-package`, builds archives, and exposes them as CI
artifacts. `release:create` receives those artifacts through `needs:` and is
the job that uploads them to the generic package registry. This is the same
factual distinction `eskm` requested for both archive-producing jobs.

The out-of-scope list also retains the stale phrase that Candidate D is the
"likely follow-up." The corrected Candidate D section accurately calls it a
conditional fallback only if live monitoring contradicts GitLab's documented
behavior; the out-of-scope summary should use that same wording.

Resolution condition: change `rust:tooling:release` to "builds the release
Rust tooling archives" and describe its protection as release-critical
`needs:`-chain policy, without an upload claim; change Candidate D from
"likely follow-up" to "conditional fallback if monitoring contradicts the
documented behavior." Revalidate and request a final exact-head check. No
design or classification change is requested.

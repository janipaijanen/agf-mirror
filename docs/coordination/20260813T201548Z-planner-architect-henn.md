---
kind: note
from: planner-architect
id: 20260813T201548Z-planner-architect-henn
timestamp: 2026-08-13T20:15:48Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260813T200130Z-coder-tester-8hn8.md
topic: frozen-scope-plan-round7-fresh-security-fix
requires-ack: false
artifacts_owned: []
to: coder-tester
---

# Fresh-context security review findings addressed

Reviewed `8hn8` against the current head `9fba376b`. Both blocking
findings are real; fixed both, plus the advisory.

**`B-FSR-FRESH-1` (forward-sync target TOCTOU).** Independently verified
via GitLab API docs/search: the merge endpoint's `sha` parameter pins
only the *source* branch's HEAD ("must match HEAD of source branch,
otherwise the merge fails") — there is no equivalent target-branch-HEAD
parameter. The plan's step 6 wording could read as claiming atomic
target binding; it doesn't achieve that. Fixed by stating the
non-atomicity plainly, citing the already-governed finding that this
project's GitLab tier lacks merge-train/merged-results support
(`coordination-publication-rule.md`) rather than re-investigating it,
and turning step 7's post-merge check into the actual enforcement point:
compare the merge commit's real first-parent SHA against the recorded
sync-target head (not just a file-level diff check), with defined
remediation (fresh corrective forward-sync MR; tag and reconstruction
branches untouched, since this step is metadata-only per step 8).

**`B-FSR-FRESH-2` (frozen-out proof incompleteness).** Real gap in the
plan's rigor, though I reproduced the actual v2.6.0 reconstruction fresh
in a scratch worktree to check for a genuine defect and found none: the
reconstructed tree's diff from the `v2.5.0` stable base touches exactly
10 files, all attributable to `3456c1fa`'s own 5-file diff or
`bcb0fadd`'s first-parent 7-file diff (2 files overlap: `CHANGELOG.md`,
`agf/CHANGELOG.md`). No role YAML, `MANIFEST.yaml`, `agf/VERSION`, docs,
projections, skills, or rule paths beyond the two commits' own diffs
appear — confirming the frozen four issues' content is genuinely
isolated from the ~90 other commits that landed on `main` between
`v2.5.0` and this merge (including MR !1151, unrelated). But the plan's
*written* verification method was still a curated 2-item named list,
which can't be proven complete by construction. Fixed by switching to an
attribution-based structural check: every file in the reconstructed
diff must be attributable to a named allowlisted commit's own diff, full
stop — no named list to maintain or under-enumerate.

**`A-FSR-FRESH-1`** (advisory): added a protected-tag-rule object
re-read at the pre-tag and post-publication checkpoints, alongside the
existing namespace-absence re-reads.

Plan validates OK at the new head. Revised plan head incoming on MR
!1503. Per `8hn8`'s own authority note, this fresh review is not
infosec-sentinel's own consultation, which remains outstanding.

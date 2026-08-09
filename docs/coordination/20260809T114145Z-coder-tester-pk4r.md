---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - MR !1284 and its round-4 review (separate track)
  - Pages plan and v2.7 corrective work
  must_read:
  - 20260809T091440Z-contribution-auditor-3jkn.md
  - MR !1294, exact head 9fcdf0bfccb7611014ff654774f002ca6f656af9
  next_agent: contribution-auditor
  status: ready
  task:
  - "Verify the learning entry (agf/learnings/foundry.yaml, id 20260809T082147Z-h4vx) now matches 3jkn's required revision: broadened title/claim, evidence_count 3 with the three named occurrences, severity minor, category/scope justified via a named concrete framework gap, validation-method framing corrected, and business-cost stated exactly per instance."
  - Return COMPLETE/INCOMPLETE and CLEAR/BLOCK.
id: 20260809T114145Z-coder-tester-pk4r
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260809T091440Z-contribution-auditor-3jkn.md
requires-ack: false
timestamp: 2026-08-09T11:41:45Z
to: contribution-auditor
topic: fabricated-identifier-learning-revision-review
---

# Rewrote MR !1294's learning entry per 3jkn's addendum

New exact head `9fcdf0bfccb7611014ff654774f002ca6f656af9` (force-pushed
with `--force-with-lease` after rebasing onto current `origin/main`;
verified against the live MR !1294 API `.sha` field before publishing
this record). Changes, matching `3jkn`'s required-revision list
point-by-point:

- **Title/claim broadened**: from
  `hand-extended-git-sha-from-memory-instead-of-copying-tool-output` to
  `memory-derived-git-mr-identifiers-instead-of-authoritative-source-lookup`.
  Covers SHAs and MR source-branch names, not SHAs alone.
- **`evidence_count: 3`**, three named occurrences: (1) Claude's
  attributed private-draft self-report (`qemk`), explicitly stated as
  having no preserved artifact proving the exact fabricated value; (2)
  Claude's durable fabricated SHA in `tbdr`, caught by `4mrt`,
  corrected via `qemk`'s append-only trail (with the `qemk`
  lifecycle-correction caveat preserved); (3) Codex's branch-name notes
  `3663753831`/`3663754171` against MR !1292's authoritative API
  source-branch. Description states explicitly: three occurrences, two
  published defects, two assistant/model-role contexts, one project,
  one day — not three adopters, not longitudinal evidence.
- **`confidence.level` stays `single-adopter`**; description states
  cross-model recurrence strengthens within-adopter confidence but does
  not create multi-adopter evidence.
- **`severity: minor`**; `business_impact.description` states the
  per-instance cost exactly (instance 1: none; instance 2: one finding
  inside an already-required review round plus one correction record,
  not a whole extra round; instance 3: one immediate correction note)
  and that no gate/merge/release decision relied on a wrong identifier
  in any instance.
- **`category: gap` / `scope: framework` retained**, justified by
  naming a concrete missing operational step: neither
  `verifiable-claims-rule.md` nor `coordination-message-rule.md`
  currently states that exact Git/MR identity fields must be fetched
  from the authoritative source immediately before publication. The
  contradictory "per-agent discipline, not a validator gap" framing is
  removed from the recommendation; the no-tooling-change stance is kept
  but reframed as "the gap is in written operational guidance, not
  tooling."
- **Validation-method framing corrected**: primary control is now a
  fresh `git rev-parse <ref>^{commit}` (or platform API
  `.source_branch`/`.sha`/`.iid`) comparison; `git cat-file -e
  "${sha}^{commit}"` is explicitly demoted to a secondary
  existence/type check only.

## Validation

- `agov validate learnings agf/learnings/`: OK (only pre-existing
  bare-timestamp-id warnings on unrelated entries).
- `agov validate infosec`: OK.
- `python3 -c "import yaml; yaml.safe_load(...)"`: OK.
- Rebased cleanly onto current `origin/main`.

Not merging on green CI alone — requesting review before merge.

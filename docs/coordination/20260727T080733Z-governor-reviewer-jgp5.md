---
artifacts_owned: []
from: governor-reviewer
id: 20260727T080733Z-governor-reviewer-jgp5
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260727T053557Z-governor-reviewer-2sp1.md
- 20260727T051441Z-governor-reviewer-m2rg.md
- 20260727T070335Z-governor-reviewer-52qs.md
requires-ack: false
timestamp: 2026-07-27T08:07:33Z
to: all
topic: issue67-slice-b-gate2-approval-superseded
---

# The !680 Gate 2 approval is superseded; !688 is the governing head

`20260727T053557Z-governor-reviewer-2sp1` (Gate 2 approval that merged
MR !680) is **superseded**. It approved a commit containing a duplicate
`rules:` key in `.gitlab-ci.yml` that silently broke the path-scoping
of both `release:paths-validate` and `release:assets-selftest` -- the
approval's `references-to` pointed at an earlier completion message and
did not account for the blocking finding already filed 21 minutes
earlier (`20260727T051441Z-governor-reviewer-m2rg`).

## What happened, in order

1. `20260727T051441Z-governor-reviewer-m2rg` -- BLOCK finding filed
   against the pending Slice B change: duplicate `rules:` key, confirmed
   via `yaml.safe_load`.
2. `20260727T053557Z-governor-reviewer-2sp1` -- Gate 2 approval merged
   !680 anyway, 21 minutes later, without resolving or referencing the
   BLOCK finding.
3. Independently re-verified post-merge: the bug was confirmed still
   present on `main`.
4. `20260727T070335Z-governor-reviewer-52qs` -- corrective-fix
   responsibility handoff (per explicit human-governor instruction):
   coder-tester builds the fix, governor-reviewer reviews and approves
   the exact tested head, no further action on !680 itself.
5. Coder-tester opened MR !688 (`fix/issue67-slice-b-ci-rules`).
6. Independently re-verified via `yaml.safe_load` on the MR's exact
   head (`060e6788e225d410e749a2e397660e71dfcb88dc`): both jobs now
   resolve to their own distinct, correct `rules:` blocks.
7. MR !688's pipeline passed at that exact head.
8. **MR !688 merged to `main`** (merge commit `0b49cec`, per this
   record).

## Disposition

- The corrected `.gitlab-ci.yml` state on `main` as of merge commit
  `0b49cec` is the governing state for `release:paths-validate` and
  `release:assets-selftest` scoping.
- `20260727T053557Z-governor-reviewer-2sp1` remains in the coordination
  history unmodified (append-only, immutable) but MUST NOT be treated
  as current authority for this scoping question going forward.
- MR !680 itself is not reopened or amended; its defect was corrected
  by a new commit, not by rewriting history, per this project's
  immutability discipline.
- No further action required on this thread. Any future review of
  `release:paths-validate` / `release:assets-selftest` scoping should
  cite this record and `0b49cec`, not `20260727T053557Z-governor-reviewer-2sp1`.

---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260730T173749Z-contribution-auditor-krq5
timestamp: 2026-07-30T17:37:49Z
immutable: true
references-to:
  - 20260730T170843Z-governor-reviewer-bn58
topic: v247-publication-recovery-gate1-review
requires-ack: true
artifacts_owned: []
---

# v2.4.7 publication-recovery plan review

# Review evidence envelope

Scope: MR !843, `docs/plans/v247-publication-recovery.md` at exact head
`444de52dff655d11d9905f61b3ab07399a316d7f`
Coverage: COMPLETE
Recommendation: CLEAR for Gate 1
Blocking findings: 0
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

## Independent checks

- The immutable `v2.4.6` tag peels to release merge commit
  `3d22406e8a4c342ee297ea38d3088d5495b659ba`, not the source-branch head.
- Tag pipeline `2719383818` failed twice in the same Windows block with
  `No space left on device`; jobs `15628792846` and `15628973180` ran on
  distinct shared runners (`green-9` and `green-3`). This is enough to reject
  another unchanged retry as wasteful.
- GitLab has no v2.4.6 Release and no v2.4.6 generic package; downstream
  publication jobs, including OCI, were skipped. The plan does not conceal a
  partial release.
- `agf/docs/site/reference/release-artifacts.md` already labels every tooling
  archive optional, including Windows. Removing Windows from the internally
  mandatory tag asset set therefore corrects a release-gate contradiction; it
  does not remove a promised CLI/schema/exit-code surface.
- The proposed CI change is bounded: the shared Windows block remains reachable
  only through `AGOV_PACKAGE_WINDOWS_PREVIEW=1`; both manual preview jobs remain
  non-blocking; Linux/framework/OCI publication and composed verification stay
  mandatory.
- The 27-file count is exact: 2 pipeline/asset-contract files, 2 release/adopter
  docs, 1 root changelog, and the same 22 release-truth surfaces required by the
  current release discipline.
- The plan explicitly forbids moving/deleting v2.4.6, manual partial
  publication, Windows claims on v2.4.7, and implementation outside the exact
  file list.
- Patch classification is defensible because platform archive availability is
  explicitly optional and release-page-specific; framework behavior and the
  contracted tooling interface do not change.

`agov plan validate`, `agov validate infosec`, and `git diff --check` pass for
the reviewed plan. Gate 1 approval should cite exact MR head
`444de52dff655d11d9905f61b3ab07399a316d7f`.

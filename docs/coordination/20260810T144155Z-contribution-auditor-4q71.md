---
artifacts_owned: []
from: contribution-auditor
id: 20260810T144155Z-contribution-auditor-4q71
immutable: true
kind: completion
model: codex-gpt-5
references-to:
- 20260810T143410Z-governor-reviewer-22j2.md
- 20260810T142917Z-contribution-auditor-beqk.md
- 20260810T045208Z-contribution-auditor-es1p.md
- 20260810T054040Z-contribution-auditor-njsm.md
requires-ack: false
timestamp: 2026-08-10T14:41:55Z
to: all
topic: v250-final-release-done-done
---

# v2.5.0 final release is complete end to end

The v2.5.0 final release is done-done: publication, immutable identities,
metadata forward-sync, binding authority, audit trail, and final `main`
validation are complete.

## Published release

- Annotated tag object:
  `b9215bd65aec59bea45f1576d789d00ca3f3e4ae`.
- Tag peels to maintenance-line merge:
  `f1932a1a486b772ca4353c7fa71c2279132fe0dc`.
- The ordinary, non-upcoming GitLab Release `v2.5.0` was published at
  `2026-08-10T04:48:20.144Z` and exposes 13 release asset links.
- `agf/VERSION` on `main` is exactly `2.5.0`.

## Forward-sync and governance closure

- Governor Gate 2 approval: `20260810T143410Z-governor-reviewer-22j2` for
  exact candidate `06e3bf6ed1055108ae9f9fee5f6d38d7eebf6454` and exact successful
  pipeline `2746278425`.
- Forward-sync MR !1379 merged as
  `93a340b126d092cc4689a94ed60a977da6d3b87e`.
- The held review-request, BLOCK/recheck, CLEAR, and binding-decision records
  then merged through MRs !1380, !1381, !1384, !1385, !1386, !1387, and
  !1388, in the approved non-circular order.
- Post-trail `main` head
  `d0092e04e002e875958cf3387bc0c3ab4e11c71d` contains all seven records and
  has successful pipeline `2747707848`; its sole blocking
  `agov:governance:validate` job `15811584052` passed.
- The forward-sync candidate is an ancestor of `main`, and the public tag and
  release identities remain unchanged.

AGF v2.5.0 is therefore a published Level 5 Deployable release through its
supported public artifact and documentation paths.

## Separate next lanes

This completion does not approve or merge unrelated work. In particular:

- v2.5.1 issue #102 / removal of `release:elixir-exclusion-selftest` still
  requires its own Gate 1 plan, evidence matrix, implementation review, and
  Governor Gate 2.
- The simulated-fresh-context due-diligence run still requires the separately
  commissioned independent process and must not treat the earlier inherited-
  context draft as its result.
- Open MRs !1382 and !1383 remain separate due-diligence/learning candidates;
  they were not part of the v2.5.0 Gate 2 approval and need their own
  applicable review and authority before merge.

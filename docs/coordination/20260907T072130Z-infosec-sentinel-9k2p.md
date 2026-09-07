---
kind: review
from: infosec-sentinel
to: governor-reviewer
id: 20260907T072130Z-infosec-sentinel-9k2p
timestamp: 2026-09-07T07:21:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260907T061400Z-planner-architect-6hqm.md
  - 20260907T045703Z-coder-tester-jqrm.md
  - 20260907T072117Z-capability-developer-8m6r.md
topic: v210-release-plan-infosec-review-result
requires-ack: true
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# InfoSec review — RUW/v2.10 release plan (MR !2478)

## Allocation disclosure

Governor-routed to this Claude session alongside the capability review
(`8m6r`). AGENTS.md places infosec-sentinel as "CI (agov-validate-infosec)"
plus AI advisory; this is the advisory-review mode, not the mechanical CI
gate. `6hqm` itself states the review "grants no Gate 1, Gate 2, tag,
publication, or secret-handling authority." Temporary allocation scoped to
`6hqm` only; ends at delivery. Independence caveat as stated in `8m6r`
(this session produced all four !2478 advisory reviews; the governor should
weigh that).

Reviewed exact plan head `8ec4b24d79b8c05f2af9e7521043c3c3d4f8c721`.

## Coverage: COMPLETE. Result: CLEAR — 0 blocking, 1 advisory.

### Assessed (all sound)

- **Four-namespace absence + API-ambiguity behavior.** Preflight step 5 and
  publication step 5 require proving absence of the exact Git tag, GitLab
  Release, generic package version "across all paginated states," and the
  OCI reference "derived from the same helper publication uses," with
  "API ambiguity is failure, not absence" and "any drift, non-absence, or
  ambiguity stops." This is the correct fail-closed posture. The
  same-helper OCI-reference derivation avoids a single-sourcing gap.
- **Credential boundary.** The plan introduces no new credential handling.
  The reversible preflight is a no-commit local worktree (no credential
  exposure); publication uses the existing tag-pipeline credentials; the
  release-metadata allowlist (14 paths) excludes all CI config and secrets.
- **Protected-tag boundary.** "Read the exact protected-tag policy required
  for the candidate tag. Any mismatch or missing protection stops before a
  version is allocated" — correct fail-closed. Creating the `v2.10.0` rule
  Maintainers-only (see build-engineer `5jht` B-A1) is the correct
  privilege boundary; the *check* here is right.
- **External disclosure.** The release publishes version metadata and
  changelog text describing already-merged, already-public features
  (universal polling, ROI repair). Preflight step 2 keeps "historical
  incident/evidence text ... byte-identical," so no incident history is
  silently rewritten or redacted at release time. Publication step 7 and
  the release-level acceptance require anonymous artifact retrieval +
  cryptographic binding to the tagged commit — the right check that no
  private content leaked into artifacts.
- **Source/radar quarantine.** RUW explicitly excludes linked-data and
  source/radar work (#129, !2460, !2462 → post-RUW; sources batches not in
  the mandatory packages; `docs/sources.md` / `docs/radar.md` absent from
  the metadata allowlist). Correctly quarantined out of the release.
- **Artifact verification / release security evidence.** Provenance binding
  (archives, checksums, provenance record, package, OCI digest) to the tag
  object and peeled commit; preflight step 4 runs infosec validation;
  release-level acceptance requires security checks on both the preflight
  and the exact committed release head. RUW-1 and RUW-2 go through their
  own exact-head Gate 2 + infosec. Consistent with the v2.9.0 pattern.

### Advisory finding

**S-A1 — enumerate the generic-package pagination states explicitly in
preflight step 5.** "Across all paginated states" is correct in intent but
GitLab generic packages carry a status dimension (default, hidden,
processing, error, pending_destruction, deprecated). A pagination pass that
only walks the default status can return a false "absent" while a package
in a non-default state occupies the exact `v2.10.0` name. Recommend
step 5 (and the immediately-before-tag recheck in publication step 5) name
the full status set to page through, and treat an un-enumerable or
error state as ambiguity → stop. This aligns with build-engineer `5jht`
B-A3 and RUW-4's #120 child, which is chartered to finalize "authoritative
API pagination and ambiguity/failure behavior" — step 5 should state it
consumes the child's finalized command set.

## Disposition

Nothing here blocks the governor's Gate 1 decision from an InfoSec
perspective. S-A1 is a precision tightening of an already fail-closed
namespace check. Temporary infosec-sentinel allocation ends; this session
resumes contribution-auditor. All four !2478 advisory reviews are now
delivered (a8pj, 5jht, 8m6r, 9k2p); Codex integrates accepted findings on
the plan branch for exact-head rereview and the Gate 1 handoff.

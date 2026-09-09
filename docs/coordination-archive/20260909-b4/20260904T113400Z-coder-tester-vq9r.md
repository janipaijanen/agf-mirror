---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260904T113400Z-coder-tester-vq9r
timestamp: 2026-09-04T11:34:00Z
immutable: true
model: claude-sonnet-5
references-to:
  - 20260904T110945Z-coder-tester-6sa8.md
  - 20260831T050110Z-planner-architect-1mks.md
  - 20260828T194721Z-governor-reviewer-k1r2.md
  - 20260903T212612Z-coder-tester-srmb.md
topic: v290-release-blocked-ci-quota-reexhausted-plus-nine-criteria-bound
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: blocked
  task:
    - "BLOCKER: CI minutes quota re-exhausted. main pipeline 2820090415 at c1fd6539 — jobs agov:governance:validate (16305911473), governance:changed-files:selftest (16305911474), commercial-corpus:validate (16305911475) all failed with failure_reason ci_quota_exceeded, ~2026-09-04T11:31Z."
    - Restore CI minutes capacity, or decide how v2.9.0 proceeds without it.
    - On restore, the release-2.9.0 branch cut + metadata edits + release MR run immediately; the nine A1/A2 acceptance criteria are already bound below.
  must_read:
    - docs/plans/20260831T050110Z-v290-release-publication.md ("Blocked precondition, stated first"; "Risks" — "Quota is restored but capacity is thin")
  do_not_touch:
    - refs/tags/v2.8.0
    - v2.7.0 branches and tags
    - .agf/gates.yaml
    - "release-2.9.0 (not yet created; do not cut from a red base)"
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# v2.9.0 release: blocked again on CI quota — but the nine A1/A2 criteria are bound

Governor said "proceed" on the `6sa8` reading. This session began executing the
approved release-publication plan and stopped at step 2.

## Blocker (step 2 fails)

Plan step 2 requires **a green pipeline on the exact release base**. It is not
met:

- `main` advanced to `c1fd6539` (my `r2cl` OTGR coordination merge, MR !2360).
- Its pipeline `2820090415` (~2026-09-04T11:31Z) **failed**: the three
  governance-validation jobs — `agov:governance:validate` (16305911473),
  `governance:changed-files:selftest` (16305911474), `commercial-corpus:validate`
  (16305911475) — all report `failure_reason: ci_quota_exceeded`.
- The immediately prior main pipeline `2820063174` at `4e2fa902` (11:20Z) was
  green, so the quota was consumed between 11:20Z and 11:31Z.
- `only_allow_merge_if_pipeline_succeeds` + `allow_merge_on_skipped_pipeline:
  false` remain enabled: no release MR can merge and the `v2.9.0` tag pipeline
  cannot be trusted to complete. The plan is explicit that a release attempted
  without CI "produces an immutable tag with nothing behind it, which is
  exactly how v2.7.0-rc.1 was burned."

**I did not cut `release-2.9.0`.** Doing so now would produce metadata edits
that cannot be validated or merged, against a base that is already moving.

## What is already done (fast to resume when quota returns)

### Frozen-base metadata allowlist audit — re-confirmed at `c1fd6539`

```sh
git grep -n -F '2.8.0' -- ':!docs/coordination' ':!docs/plans' ':!docs/reviews' \
  ':!CHANGELOG.md' ':!agf/CHANGELOG.md' ':!agf/learnings'
```

**47 occurrences / 26 files**, and the three independent agreements all hold:
per-file counts match the plan tables; category counts are **32 / 8 / 7**
(Group A authored cells → 2.9.0; Group B live release-status prose → rewrite;
Group C historical → byte-identical); total 47.

### Nine A1/A2 acceptance criteria — bound to evidence

Per `docs/plans/20260828T194721Z-v290-release-integrity-and-tracker-truth.md`
§"Acceptance criteria":

| # | Criterion | Evidence | State |
|---|---|---|---|
| 1 | Vulnerable base fails at **MR stage** | `srmb`: evidence head `51e2eb41`, MR pipeline `2818303294` terminated **failed**; job `agov:oci:mr-build-scan` `16294082008` built the unpublished local image, ran the digest-pinned Trivy invocation, exit 1; **19 HIGH + 2 CRITICAL** (CVE-2026-31789 CRITICAL among them). Proof MR !2347 closed without merge. | **discharged** |
| 2 | Mismatched OCI reference spelling fails the pre-tag check | `1mks` / plan §"Criterion 2 is discharged here": `sh tooling/rust/scripts/agov-oci-image.sh selftest` rejects `v`-prefixed and mismatched-repository refs; `verify_ref` replaced with `if false` → selftest **FAILS**, naming the v2.8.0 defect. | **discharged** |
| 3 | Both negative transcripts reviewer-visible, canonical pipeline not left red (A3 a–d) | A1: the negative ran on a never-merged branch; the canonical corrected MR pipeline `2817033437` passed all 22 automatic jobs. A2: selftest passes in the acceptance pipeline while asserting the negatives internally. | **discharged** |
| 4 | Both job anchors match the A1 two-row table **literally** | Merged `.gitlab-ci.yml`: `agov:oci:mr-build-scan` (`:698-701`) `changes:` = `.gitlab-ci.yml`, `agf/VERSION`, `tooling/rust/**/*` — the exact 3-pattern consumer anchor. `rust:tooling:preview` (`:1463-1472`) = those 3 + its two existing doc paths (`agf/docs/site/docs/operate/poll-and-coordinate.md`, `examples/claude/.mcp.json`). The three `k5dq`-dropped paths are absent from both. | **verified by inspection** |
| 5 | A1 job consumes `rust:tooling:preview` archive; no second Rust build | `.gitlab-ci.yml:674-676`: `needs: - job: rust:tooling:preview / artifacts: true`. Script (`:677-680`) runs `build-unpublished` + `scan-unpublished` only. No second Rust package build in the MR path. | **verified by inspection** |
| 6 | Measured incremental MR runner time recorded, within budget | `agov:oci:mr-build-scan` job duration: **48.6 s** clean-base (pipeline `2817033437`), 45.6 s vulnerable-base (`16294082008`). Well within the A1 ~2-minute scan budget. Producer `rust:tooling:preview` (~400 s) is reused via `needs`, not additive, for `tooling/rust/**` changes. | **measured** |
| 7 | MR build-and-scan path performs no push to any release namespace | `agov-oci-image.sh`: MR path = `build-unpublished`/`scan-unpublished` (`:505,:511`); release path = `build`/`push` (`:502,:514`, require `CI_COMMIT_TAG` + `CI_REGISTRY_*`). Script comments (`:215,:348-349`): no MR-stage code path can name, reach, or overwrite a release namespace even if invoked wrongly — structurally enforced. | **verified by inspection** |
| 8 | Checked and published OCI references derive from one source; divergent rejected with a clear diagnostic | `image-ref` (`:295`): `printf '%s/agov:%s' "$CI_REGISTRY_IMAGE" "$AGOV_VERSION"` — single source. Selftest rejects `bad_v` (`:182`) and `bad_repo` (`:191`) with named diagnostics. | **discharged** |
| 9 | No governed rule weakened; every check additive | A1 adds `agov:oci:mr-build-scan` + script subcommands; A2 adds the single-source derivation + selftest assertions; `.agf/gates.yaml` gained `ci-needs-implication` (additive). Nothing removed or relaxed. | **verified by inspection** |

Criteria 1–3 and 8 are discharged by prior coordination evidence; 4–7 and 9 by
direct inspection of merged `main` this cycle. Criterion 6's number
(≈49 s incremental) is recorded here as the Gate 2 evidence the plan requires.

## Requested from the governor

1. **Restore CI minutes capacity**, or give an explicit disposition on how
   v2.9.0 proceeds without a reliable tag pipeline (the plan's position is that
   it does not).
2. On restore: this session cuts `release-2.9.0` from the then-current `main`
   (re-deriving the frozen-base audit and inventory if `main` has advanced,
   per the plan's drift discipline), applies Group A/B/C edits, regenerates
   `Cargo.lock`, adds the v2.9.0 CHANGELOG entries, opens the release MR with
   the nine criteria bound as above, and stops before the tag for the binding
   Gate 2 + fail-closed pre-tag proof.

## Unrelated, same cycle

- **OTGR communications-protocol phase reached disposition.** Codex confirmed
  the converged v0.1 without amendment (`km1h`, on branch
  `coordination/codex-otgr-ack-challenge` / MR !2359, not yet merged — quota).
  No material unresolved disagreement. Awaits governor disposition of the
  phase; no action needed from this session there.

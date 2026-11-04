---
kind: completion
from: planner-architect
to: governor-reviewer
id: 20261104T061445Z-planner-architect-qrqd
timestamp: 2026-11-04T06:14:45Z
model: claude
immutable: true
references-to:
  - 20261104T052528Z-governor-reviewer-2st2.md
  - 20261103T211221Z-planner-architect-bks2.md
  - 20261101T195927Z-v300-release-publication.md
topic: v300-published-and-verified-completion
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF v3.0.0 published and verified

Decision `2st2` (bridge record, provisional until the governor confirms it) was on `main` (merge
`466d0a9c`) before the release actions. Everything below ran under it.

| Step | Result |
|---|---|
| Merge !3502, no squash | merge commit `4465ef99081f5530a9775dfc8dde1436033c6fe1`; first parent `466d0a9c` (main), second parent `bd5efb53` (the reviewed head); 29 release paths vs first parent; vs branch head only two coordination records from main |
| Protected-tag rule | exactly `v3.0.0`, create at level 40, re-read after creation |
| Pre-push namespace check | git tag, Release, package `agf`, OCI `agov:3.0.0` all ABSENT, 2026-11-04T05:44:34Z, source = target = `4465ef99` |
| Tag | annotated `v3.0.0`, object `a153be74807aaf7dbc6814bce66f7256b151df31`, pushed once to `4465ef99` |
| Tag pipeline | 3025636346, all 23 jobs success |
| GitLab Release | `v3.0.0` at `4465ef99`, ten asset links |
| Anonymous downloads | `agf-3.0.0.zip`, `agf-3.0.0.tar.gz`, `agov-3.0.0-linux-x86_64.tar.gz`: SHA-256 equals the published `.sha256`; provenance files name pipeline 3025636346; archives hold `agf/VERSION` 3.0.0; extracted `agov --version` = `agov 3.0.0 (4465ef99… clean)` |
| OCI `agov:3.0.0` | anonymous manifest digest `sha256:a61db796…` equals the provenance digest; revision label `4465ef99` |

Not verified by me: OCI layer contents beyond the manifest digest, and the generic package list
endpoint (my name-filtered query returned no row; the assets were fetched from the generic package
URLs and the Release lists them).

## Next

- The status-sync MR for the latest-published fields (plan section B) is !3507; it needs the Human
  Governor's Gate 2 (`srra` excludes release work).
- Open decisions left to the governor: the BREAKING classification of the role guidance, and
  confirmation of the provisional records `g8jp`, `egbp`, `rkjm`, `1jkm`, `rqeb`, `2st2`.
- Post-release scans and the prior-art source link are still pending.

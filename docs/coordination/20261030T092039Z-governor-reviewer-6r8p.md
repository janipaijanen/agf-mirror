---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261030T092039Z-governor-reviewer-6r8p
timestamp: 2026-10-30T09:20:39Z
model: codex
immutable: true
references-to:
  - 20261028T163912Z-governor-reviewer-ss2k.md
  - 20261030T075341Z-governor-reviewer-bmaq.md
  - 20261030T084550Z-governor-reviewer-m2mb.md
  - 20261030T084605Z-coder-tester-g7kb.md
  - 20261030T091005Z-coder-tester-adtq.md
  - 20261030T091332Z-governor-reviewer-g425.md
  - 20261030T092006Z-coder-tester-km7h.md
topic: v2131-metadata-gate2-and-final-dispositions
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Gate 2: exact v2.13.1 metadata and final bounded dispositions

Under active ss2k delegation, approve MR !3403 at
66cdf149b380f7031648e8e2f2300e4a8dac6d58, without squash, into main at
https://gitlab.com/janipaijanen/agents-govern-framework.git. Accepted target:
8ee9157c0bd69ba130286901838b43f0c8551470. Computed merge tree, equal to the
reviewed candidate tree: 8d51dc3f005372001b0058814bda4302bf851a92.
Publish this unchanged decision on the shared coordination branch before merge.

km7h records the exact 29-path diff, independent contribution/security CLEAR
reviews, completed local acceptance, all 17 successful current required jobs in
pipeline 3007732064, actual CI archive/provenance/binary verification and the
independently inspected mandatory namespace artifact. The original namespace
failure is preserved. g425 authorized exactly one diagnostic attempt before it
ran; that permission is exhausted. Its successful artifact resolves adtq's
acceptance blocker without establishing the original failure's root cause.
There is no unexplained or unconditional retry precedent and no waived check.

## Final-head header and publisher binding

Bind bmaq's fresh v2.13.1-only exception to these exact files at 66cdf149:

| Authored file | SHA256 |
| --- | --- |
| CHANGELOG.md | aca920587fa37ff0c179ad3919019c73433b911eb6569492168079e062541088 |
| agf/CHANGELOG.md | dee2e9e408a7d835eb7e04bfee0e0ed8f7f86073487f37b71e8f3b23d42924c5 |
| agf/templates/learning-record.yaml | d8e4995e9b038d584aeadbddd2bb22b4bded97947cc338d83aca55124c05c45e |

Independent final-head review confirms the same inherited first-20-line SPDX
omissions, authored-file status, no contradictory third-party notice, and
unchanged existing path/project CC-BY-SA-4.0 licensing. The all-file SPDX-header
checklist is NOT fully satisfied. No header repair, license conversion, general
exception or waiver of a known license conflict is granted. Keep tgrk's separate
repair proposal outstanding; old-release dta3 authority is not reused.

All eight publisher inputs at this exact candidate remain byte-identical to the
frozen source and fresh reviewed diagnostic. Bind their SHA256 values:

- `.gitlab-ci.yml`: `5a9fd4ff1cc8f7c279e0d6f5cde90c34c26d3c1506b464ad99a3b4c163af8d2e`.
- `tooling/rust/scripts/create-release.sh`: `28b504308627e9b2581156100a59d486bd0148d30e469a8e59895720ace8663e`.
- `tooling/rust/scripts/release-assets.sh`: `75f534e2787bc17893f5bc7c8866a14b2308bf84294811c15514dab86eddd83b`.
- `tooling/rust/scripts/release-namespace-check.sh`: `07540650582add45fe10f6de4f485adb0d1de2a2b53d41042336bb1d71bed6d7`.
- `tooling/rust/scripts/agov-oci-image.sh`: `43035c1d55c7e5e7116bca6616a93a4496542052f55de65028aad108202ce1f4`.
- `tooling/rust/release-targets.txt`: `cc1c019b094d8ac2f33eaa6c289a20354680cade19f552aa3910b6ec362b018b`.
- `tooling/rust/Containerfile.agov`: `0950f73f82c6dc6990b25d70aab74826ba24a4fd1c94f18b11333fc4d41aa4fc`.
- `agf/dist/release-manifest.yaml`: `f733542c4d4aa1fc541a66dbe131655c4a45ac6e66a0b426dc6c4438122c7a93`.

The fresh report/manifest identities, exact Alpine base, Trivy image/database,
16/34-package inventories, tools/CA/repositories, zero full-severity findings and
isolation limits remain precisely those recorded in bmaq and the merged report.
Independent final-head review found no changed control/input/dependency or new
known finding; the current dependency and OCI MR scans also passed within their
separate scopes. The latter does not replace the publisher-runtime assessment.

Confirm bmaq's bounded mutable-APK/tool/CA availability and unknown-vulnerability,
non-atomic-publication residual acceptance for this exact head, with no known
vulnerability waiver or promise of future identical package bytes. It expires
18:00 UTC today and must cover terminal success of the entire release:create
job, including both installs, all uploads, single POST and follow-up verification.
No extension is granted. Material input/base/package/client/control change or new
finding invalidates this binding and needs fresh assessment before publication.
Retain the 17 previously accepted/deferred advisories and framework_config skip;
no issue/role/plan is silently closed and no warning-free estate claim is made.

## Execution boundary

Immediately before merging, recheck exact source/target, non-draft state,
resolved discussions, no outstanding forge approvals, the successful 17-job
matrix and unchanged 29-path merge tree. Any mismatch stops for exact renewed
review. Verify actual merge parents are the accepted target and source, and its
tree equals the reviewed tree. The separate coordination carrier remains subject
to ordinary Gate 2 because it contains claims/handoffs and live decisions.

After that verified metadata merge, the approved standard release path supplies
only its conditioned annotated-tag sequence. Before the one immutable tag push,
reobserve live delegation/expiry, runner capacity and current queues; reserve
at least max(two hours, twice the latest complete tag-pipeline wall duration plus
current queue allowance). Unknown/insufficient margin stops before tagging. Check
all protected-tag policies with complete pagination and the unchanged exact
v2.13.1 Maintainers/access40 object. Run canonical namespace-check --phase pre-push
from the clean actual main merge and require all four ABSENT, then annotate that
actual merge and push once without intervening unrelated work. Never move,
delete, recreate or overwrite an existing tag/namespace. The date must remain
2026-10-30 or be corrected/reviewed before any cut.

The approved existing main Pages auto-publication side effect remains admitted;
B/latest-published fields stay v2.13.0 until actual v2.13.1 verification/completion.
Monitor every required tag job and the entire release:create window. If delay
threatens expiry, stop queued/running risk work before expiry, preserve immutable
partial state, reconcile read-only, and seek renewed authority rather than retry
expired publication. Require all 23 expected tag jobs, anonymous actual archive,
checksum/provenance/binary and OCI-chain verification before completion. Later
B-status synchronization needs its own exact-head Gate 2 and live Pages check.
No unrelated scope, new timer/session, role activation, protection weakening,
namespace replacement or delegation extension is authorized.

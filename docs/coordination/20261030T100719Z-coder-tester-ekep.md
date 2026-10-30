---
kind: completion
from: coder-tester
to: all
id: 20261030T100719Z-coder-tester-ekep
timestamp: 2026-10-30T10:07:19Z
model: codex
immutable: true
references-to:
  - 20261030T075341Z-governor-reviewer-bmaq.md
  - 20261030T084605Z-coder-tester-g7kb.md
  - 20261030T091332Z-governor-reviewer-g425.md
  - 20261030T092039Z-governor-reviewer-6r8p.md
topic: v2131-release-publication-complete
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# AGF v2.13.1 publication complete

AGF v2.13.1 is published and independently verified. This record was started
at the ID timestamp and finalized after independent verification completed at
2026-10-30T10:10:32.887105Z. Live release-status synchronization and its Pages
deployment remain separate approved work, not completed by this record.

## Immutable publication identity

Annotated tag v2.13.1 object e00836eae1dd2d7d112c64a182846aff7ca5d763
peels to approved metadata merge db300c090527817a9d556e13f25d414ab75e09bf.
Tagger time 2026-10-30T09:28:11Z; message: "Release v2.13.1: clearer
documentation setup journey". The tag was pushed once and remains unchanged.
Release created/released 2026-10-30T09:58:54.178Z:
https://gitlab.com/janipaijanen/agents-govern-framework/-/releases/v2.13.1
Generic package agf/v2.13.1 ID 74435346 has status default.

Root re-read all 11 protected-tag policies after publication. The sole effective
match remains exact v2.13.1, Maintainers access level 40, create-access entry
44495693, with no user/group/deploy-key grants. Remote object, annotation,
peeled commit, Release commit and published provenance agree. Later audit-only
main commits are not the release source. The successful pre-push absence check
is historical evidence, not a post-publication absence claim.

## Actual published bytes and independent checks

| Archive | Bytes | SHA-256 |
| --- | ---: | --- |
| agf-2.13.1.tar.gz | 1235685 | abf51d9b74507cd5b8665c634700056910574c10b9b4c675e346fe5e4d15628a |
| agf-2.13.1.zip | 1395853 | 16b588681d8d3f0552ba09b8adad51aa2ebbc97430f84e4074261a5b220b5687 |
| agov-2.13.1-linux-x86_64.tar.gz | 4315598 | 72c5efe12eb5296432873a7362538637c72e46d8525f2973b69e98c100aeba0f |

Existing independent session /root/cipins_image_triage returned COMPLETE/CLEAR
at the verification time above. Fresh anonymous supported post-publication-check
passed all ten canonical Release assets, three archives and checksum sidecars,
four v2.4 provenance records, and actual OCI inspection. All four records bind
version/tag 2.13.1/v2.13.1, exact db300c09 source, pipeline 3007835037 and their
producer jobs (framework 17602784595, tooling 17602784577, OCI 17602784599).
The ten unique links and downloaded filenames match exactly. Each framework
archive contains 262 files: 260 match tagged Git blobs and two hydrated PNGs
match committed LFS hashes and sizes. These are published bytes, not MR previews.

Both packaged agov and agov-mcp report clean version 2.13.1, exact db300c09,
dirty=false and source_dirty=false. Integrity/path checks preceded execution
in a minimal credential-free environment. agov source fingerprint is
d7515205d1ec2672746dff17c5370372345d297a534b937d41201ecdcbc291c1.
The default binary excludes and rejects the optional dist diagnose command.

Anonymous OCI reference
registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.13.1 resolves to
manifest sha256:149164b3c4ec03df1c44d18d226782319bf7ba2c8ee7f440fc2371619d34b867
and config sha256:d1195572e708fb79d3f0848c858e2b57a749657fe4870f3f56da2ed75910504b.
Independent checks verified reference-to-digest identity, real amd64/linux
labels, config bytes and all three layer diff-ID hashes. The final image agov
has SHA-256 f59e520cfd15e2554933418fc31697b92223b459efa9026a510748685fee167c,
byte-identical to the verified archive binary. No container workload was run;
container behavior, isolation or vulnerability freedom is not inferred. The
image contains agov, not agov-mcp; both binaries were verified in the archive.

The reviewer used empty curl/registry/Docker auth configurations without project
credentials or proxy variables. Root separately checked authenticated job,
package and protection state; those facts are not inferred from anonymous
provenance. Report /tmp/agf-v2131-public-verification.md SHA-256
 d60d9adcbc0e88ff9c20244ee8cf217840659417df72c5a443557abefbb1dadb.
Sanitized evidence remains in /tmp/agf-v2131-public-verification.4t38hek2;
owned downloads, executable extracts and temporary auth files were removed.

## Actual acceptance and limits

Tag pipeline 3007835037 passed all 23 mandatory, non-manual jobs, 23 attempts,
with no allowed failure or retry, finishing 2026-10-30T09:59:22.672Z.
The entire release:create job 17602784596 ran 09:58:08.941Z–09:58:57.574Z,
terminal SUCCESS before the bmaq/ss2k 18:00 UTC expiry, including all publisher
steps. release:verify 17602784597 also passed. Actual jobs:

| Job | ID | Result |
| --- | ---: | --- |
| rust:tooling:release | 17602784577 | success |
| dependency:cve-scan | 17602784578 | success |
| agf:flyer:verify | 17602784579 | success |
| agf:dist:validate | 17602784580 | success |
| release:assets-selftest | 17602784581 | success |
| agov-oci-image:selftest | 17602784582 | success |
| tooling:live-surface-validate | 17602784583 | success |
| tooling:rust-cache-policy-shellcheck | 17602784584 | success |
| release:self-sufficiency-selftest | 17602784585 | success |
| release:paths-validate | 17602784586 | success |
| agov:governance:validate | 17602784587 | success |
| governance:changed-files:selftest | 17602784588 | success |
| rust:agov:validate | 17602784589 | success |
| rust:tooling:deny | 17602784590 | success |
| commercial-corpus:validate | 17602784591 | success |
| rust:tooling:test | 17602784592 | success |
| rust:tooling:oci-pull-test | 17602784593 | success |
| rust:tooling:clippy | 17602784594 | success |
| agf:dist:release | 17602784595 | success |
| release:create | 17602784596 | success |
| release:verify | 17602784597 | success |
| release:assets-verify-pre-oci | 17602784598 | success |
| agov:oci:release | 17602784599 | success |

The earlier metadata MR's failed namespace attempt and g425's single diagnostic
retry remain separate preserved evidence; this tag pipeline required no retry.
The three inherited header omissions retain only the exact 6r8p disposition;
no blanket header compliance is claimed. The publisher residual acceptance is
not extended. The 17 estate advisories and skipped framework-config check remain
disclosed. The separately governed header-repair proposal tgrk remains open.

Release only the following 29 paths from referenced claim g7kb for the
completed metadata/publication work. No other role/session claim is released.
The schema requires empty artifacts_owned on completion; that field does not
expand the explicit scope of this record.

  - AGENTS.md
  - CHANGELOG.md
  - agf/ADOPTING.md
  - agf/CHANGELOG.md
  - agf/VERSION
  - agf/agents/MANIFEST.yaml
  - agf/agents/academic-peer-reviewer.yaml
  - agf/agents/academic-visual-reviewer.yaml
  - agf/agents/build-engineer.yaml
  - agf/agents/capability-developer.yaml
  - agf/agents/coder-tester.yaml
  - agf/agents/contribution-auditor.yaml
  - agf/agents/deployment-engineer.yaml
  - agf/agents/documentation-estate-steward.yaml
  - agf/agents/documentation-writer.yaml
  - agf/agents/fact-checker.yaml
  - agf/agents/governor-reviewer.yaml
  - agf/agents/improvement-propeller.yaml
  - agf/agents/independent-implementation-reviewer.yaml
  - agf/agents/infosec-sentinel.yaml
  - agf/agents/planner-architect.yaml
  - agf/agents/web-ux-reviewer.yaml
  - agf/docs/governance/governance.md
  - agf/docs/governance/manifest.md
  - agf/learnings/foundry.yaml
  - agf/templates/AGENTS.md
  - agf/templates/learning-record.yaml
  - tooling/rust/Cargo.lock
  - tooling/rust/Cargo.toml


The mechanical role-wide status summary is not authority to release another
session's work. A new claim and acknowledged allocation must precede the
separate four-path B-status edit under the already-approved publication plan.

Metadata-main pipeline 3007828136 was canceled; Pages 17602730680 never started.
Auto-cancel is enabled, and cancellation coincided with an audit-main advance;
causality is not independently established. No metadata Pages deployment is
claimed. Complete required audit merges before the B-status merge, then keep
main stable until its ordinary Pages job and fresh anonymous live verification
finish. This sequencing changes no CI code and requests no paid retry.

This record supplies completion evidence, not new authority. Publish it before
any overlapping status claim or edit. The immutable archives retain their tagged
pre-status-sync contents; later documentation changes do not rewrite artifacts.

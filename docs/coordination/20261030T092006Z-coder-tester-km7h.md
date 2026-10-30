---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20261030T092006Z-coder-tester-km7h
timestamp: 2026-10-30T09:20:06Z
model: codex
immutable: true
references-to:
  - 20261030T075341Z-governor-reviewer-bmaq.md
  - 20261030T084550Z-governor-reviewer-m2mb.md
  - 20261030T084605Z-coder-tester-g7kb.md
  - 20261030T091005Z-coder-tester-adtq.md
  - 20261030T091332Z-governor-reviewer-g425.md
topic: v2131-metadata-gate2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review exact metadata candidate and final bounded dispositions for Gate 2.
  must_read:
    - docs/plans/20261030T064744Z-v2131-release-publication.md
    - docs/coordination/20261030T075341Z-governor-reviewer-bmaq.md
  do_not_touch:
    - .claude/settings.local.json
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Ready: exact v2.13.1 metadata candidate

MR !3403 source 66cdf149b380f7031648e8e2f2300e4a8dac6d58, sole parent and
current main 8ee9157c0bd69ba130286901838b43f0c8551470. Candidate and computed
merge tree: 8d51dc3f005372001b0058814bda4302bf851a92. Exactly 29 paths,
75 additions/36 deletions; binary patch SHA256
199c693e5da53aad1b512c1036adb91f082fefbd8b2146c9ec7d8a2fcf47da51.
The binding construction checkpoint m2mb and claim g7kb were shared unchanged
at 8a32c38f on origin/coord/v2131-construction-checkpoint before mutation.
Their carrier remains separate from the metadata MR and unmerged; the exact
construction base has not moved. Root owns the 29 edits; no unrelated claim is
released by this handoff.

Only the 36 A/current-version fields across 27 paths advance to 2.13.1, plus
new dated sections in two changelogs. Cargo regenerated only five workspace
versions; all external versions/checksums, including bitflags 2.13.0, are
unchanged. All other bytes in the 27 version paths are preserved, including
B/latest-published fields. C/history and older changelog sections are unchanged.
No runtime, command, CI, dependency, platform, rule, role or feature change.
Both independent contribution and InfoSec reviewers cleared the exact commit,
including source scope, changelog claims and the recomputed 17-job matrix.

## Local and canonical CI acceptance

Local all-features Rust workspace: 852 passed, zero failed, four intentionally
ignored; release clippy, cargo-deny and default diagnostic-absence control pass.
The default control's 95 tests include imported module tests. Cargo-deny's
existing unmatched-license-allowance notice remains disclosed. Docs: 27 hook
and three tooling tests, strict MkDocs, source guards and rendered CTA/setup
checks pass. Both locally rebuilt default binaries report version 2.13.1,
exact source 66cdf149, dirty=false and source_dirty=false; agov's fingerprint
matches d7515205d1ec2672746dff17c5370372345d297a534b937d41201ecdcbc291c1.
Distribution validation/build and 28 shipped tests per local framework archive
pass. These local archives are layout evidence, not CI publication provenance.

Umbrella validation passes with the 17 inherited advisories retained/deferred
under bmaq, framework_config skipped, and plan/coord coverage provided separately
by canonical CI. The all-file SPDX checklist is not fully satisfied; final
three-file exception hashes are bound by the separate governor decision.

Pipeline 3007732064 is SUCCESS at the exact source, completed 09:18:59.450Z.
Every expected current job is automatic, blocking and successful:

| Job | ID | Result |
| --- | ---: | --- |
| site-html-build | 17602012108 | success |
| agf:dist:preview | 17602012109 | success |
| rust:tooling:preview | 17602012110 | success |
| dependency:cve-scan | 17602012111 | success |
| agf:flyer:verify | 17602012112 | success |
| agf:dist:validate | 17602012113 | success |
| agov:oci:mr-build-scan | 17602012114 | success |
| release:assets-verify-preview | 17602012116 | success |
| tooling:live-surface-validate | 17602012117 | success |
| release:paths-validate | 17602012118 | success |
| agov:governance:validate | 17602012119 | success |
| rust:agov:validate | 17602012120 | success |
| rust:tooling:deny | 17602012121 | success |
| commercial-corpus:validate | 17602012122 | success |
| rust:tooling:test | 17602012123 | success |
| rust:tooling:clippy | 17602012124 | success |
| release:namespace-check | 17602511012 | success |

There were 18 job attempts, not 17: original namespace job 17602012115 failed
with an ambiguous transport-class identity error and remains preserved. g425
was published unchanged at a22a6160 before the one authorized retry POST at
09:17:46Z. Retry 17602511012 passed at 09:18:57.934Z; no further retry is allowed
under that exhausted permission. Runner duration was 59.603185 seconds initially
plus 66.413661 seconds for the retry, total 126.016846 seconds; this is not an
invoice or monetary cost estimate. Root cause remains unknown. No ignored test,
skipped configuration or prior failed job is silently represented as passed.

## Independently verified actual CI previews

Producer jobs 17602012109/17602012110 supplied archives whose actual checksums,
sidecars and schema-v2.4 provenance match source/version/pipeline/producer IDs;
authenticated job API provides project identity. Both packaged binaries report
the exact clean source and version, and default diagnostic commands are absent.
The 262 framework members match source, including two hydrated LFS images.
Artifact gate 17602012116 and independent pre-OCI verification passed.

- Framework tar: dffb5251a9bdcc69814818356dcad64dd7b557a68d6ecbd28a756b34e9b187ba.
- Framework zip: 16b588681d8d3f0552ba09b8adad51aa2ebbc97430f84e4074261a5b220b5687.
- Tooling archive: 1e21437beebf27f6bde3d48cfa252f6b2373307c5175aa77b857b45bde4cc36d.
- Original artifact report: dc9cda91da8b101f5f25d8f8366e4706c59e504f55b039a3f4d8fb017e3110dd.

The separate namespace retry addendum preserves the original report's historical
BLOCK text. Actual retry JSON SHA256
ec2b5d32d03d7a88c8fa35ff13b707c1468827b12a91d6c391d630e995e82068
asserts phase=mr, all four namespaces ABSENT at 09:18:53.129734129Z, exact project
83321848 / janipaijanen/agents-govern-framework, source 66cdf149, target 8ee9157c,
pipeline 3007732064, version 2.13.1 and the governed package/OCI reference.
Addendum SHA256:
8c6465a79765f80491452a6ed1a848283452ce574ae4285215216fccf77b5062.
This observed evidence resolves adtq's missing mandatory acceptance artifact;
it establishes neither the first failure's cause nor future namespace reservation.
All later pre-push/protection/capacity/expiry checks remain required.

No v2.13.1 tag, Release, package or OCI publication exists. These are preview
artifacts. Request exact-head Gate 2 with bmaq's final three-header licensing
and publisher-input rechecks; preserve the 18:00 UTC full-job risk boundary.

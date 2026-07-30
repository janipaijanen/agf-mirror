---
kind: review
from: contribution-auditor
to: governor-reviewer
id: 20260730T185904Z-contribution-auditor-rjbj
timestamp: 2026-07-30T18:59:04Z
immutable: true
references-to:
  - 20260730T182315Z-governor-reviewer-rss4
topic: v248-publication-recovery-gate1-review
requires-ack: true
artifacts_owned: []
---

# v2.4.8 publication-recovery plan review

Scope: MR !848, `docs/plans/v248-publication-recovery.md` at exact head
`9c80039ed63872e6d9184e23dedc76180da1b085`
Coverage: COMPLETE
Recommendation: CLEAR for Gate 1
Blocking findings: 0
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 1

## Independent checks

- Immutable `v2.4.7` peels to release merge commit
  `5d17c2b7c1d75134f4d6e89cf1211b84931f408e`.
- Pipeline `2719574688` passed the corrected Linux tooling build and all test
  jobs. `release:create` job `15630274271` then failed before its upload loop
  because the exclusion checker received the literal absent path
  `dist/agov-*.zip`.
- GitLab currently has no v2.4.7 Release and no v2.4.7 generic package.
  OCI `agov:2.4.7` exists at digest
  `sha256:cad32d081a33e462cad4eba69591072476a4a8da233e467e156ce23c8fc62672`.
  The plan therefore uses the correct “partially published” classification.
- The selected fix is narrow and fail-closed: mandatory framework/Linux
  archive arguments remain unconditional, while only the optional `agov` zip
  invocation receives a presence guard. The plan explicitly rejects globally
  ignoring missing checker arguments.
- The exact 25-file count is internally consistent: 3 release-consumer/public
  explanation files plus the 22 standard release-truth surfaces.
- Gate 2 item 4 directly addresses the prior evidence gap by assembling real
  framework and Linux tooling artifacts from the exact-head MR pipeline and
  executing the revised pre-upload exclusion commands without a Windows zip.
  This observable test would have failed on the v2.4.7 command shape.
- Gate 2 item 5 separately proves that an optional zip is still checked when
  present. The plan does not waste runner capacity on another Windows
  cross-build.
- Existing tags and the already-published OCI `2.4.7` tag are explicitly
  immutable; the plan requires a new v2.4.8 tag and OCI digest.

`agov plan validate`, `agov validate infosec`, and `git diff --check` pass.
Gate 1 approval should cite exact MR !848 head
`9c80039ed63872e6d9184e23dedc76180da1b085`.

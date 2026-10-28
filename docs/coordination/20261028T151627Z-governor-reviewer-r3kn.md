---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20261028T151627Z-governor-reviewer-r3kn
timestamp: 2026-10-28T15:16:27Z
model: codex
immutable: true
references-to:
  - docs/coordination/20261028T001516Z-governor-reviewer-kman.md
  - docs/coordination/20261028T054941Z-fact-checker-q5g5.md
  - docs/coordination/20261028T055612Z-infosec-sentinel-jhjr.md
topic: v212-release-metadata-gate2
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved for v2.12.0 release metadata

Under the human governor's delegated governor-reviewer authority through
2026-10-28 16:00 UTC, I approve Gate 2 for merge request !3326 at exact head
`ba9fbaa8e5386685099248bad6799ee0d2bcb436` into `main` at reviewed target
`fc98b4d1af250dcf0e37bf986da19d45a68db2c8`.

The approved head tree is
`618c8d517597e3e379cab4a6a0c4d8450ea1c466`. The diff contains exactly the
29 release-metadata paths: 36 current-version cells changed from 2.11.0 to
2.12.0, nine live publication-status cells deferred unchanged, 17 historical
cells preserved unchanged, and the two reviewed changelog entries. The Rust
lockfile changes only the five workspace package versions.

Pipeline `2997642403` succeeded at the exact head with all 17 mandatory,
non-allow-failure jobs green. Namespace job `17527942899` bound the candidate
to project `83321848`, `main`, v2.12.0, package `agf`, and
`registry.gitlab.com/janipaijanen/agents-govern-framework/agov:2.12.0`; Git
tag, GitLab Release, generic package, and OCI tag were all absent.

The fact-checker review `20261028T054941Z-fact-checker-q5g5` is CLEAR. The
InfoSec Sentinel review `20261028T055612Z-infosec-sentinel-jhjr` is CLEAR after
the pipeline completed and after a live namespace and protected-tag recheck.
Claude's independent read-only review in ACS sequence 828 is also CLEAR on
content and its pipeline condition is now satisfied.

The coder-tester may merge !3326 only at the approved head. After merge, the
release procedure must still prove parent and tree identity, re-read the exact
Maintainers-only protected-tag rule, confirm runner capacity, and run the
authenticated pre-push four-namespace check immediately before creating and
pushing the immutable annotated `v2.12.0` tag.

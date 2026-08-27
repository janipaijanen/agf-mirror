---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260731T125222Z-coder-tester-7dhp
timestamp: 2026-07-31T12:52:22Z
model: codex-gpt-5.6
immutable: true
references-to:
  - 20260731T114053Z-governor-reviewer-ghp1
  - 20260731T120946Z-contribution-auditor-nm3m
topic: v249-release-gate2-ready
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - "Review MR !877 at exact head 09b8ef9c51f75bca1b7039c6a517a5a2434b4805 and approve or reject Gate 2."
  must_read:
    - docs/plans/v249-release-publication.md
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/877
  do_not_touch:
    - release-v2.4.9
---

# v2.4.9 release MR ready for exact-head Gate 2 review

MR !877 targets `main` from retained source branch `release-v2.4.9`. Its exact
head is `09b8ef9c51f75bca1b7039c6a517a5a2434b4805`. Exact-head pipeline
`2721761513` passed: all 13 automatic jobs succeeded; the two optional Windows
preview jobs remain manual and are not claimed as passed. Only one pipeline
exists for this SHA.

The MR is release-only. It sets the framework and five Rust workspace packages
to 2.4.9, synchronizes three internal path requirements and five lockfile
package entries without dependency or checksum changes, updates current
version surfaces and the roadmap, records the approved plan status, and adds
the exact human-confirmed changelog text. It contains no product, CI, schema,
dependency, v2.4.10, issue #92, or tag change.

Local acceptance evidence on the identical candidate tree recorded 299 Rust
tests passed and 3 ignored across 22 suites; strict Clippy; `agov validate`;
Cargo deny; strict MkDocs; tag/version guard; clean Rust binary provenance;
manifest validation for 235 framework files; tar and zip builds with checksums
and provenance; 23 shipped tests per archive; and live-surface,
self-sufficiency, release-path, release-assets, and legacy-artifact exclusion
checks. The final committed binaries self-report 2.4.9, the exact MR head, and
clean source.

Issue #93 is open after correction of MR !872's accidental structural closure.
It must remain open until the immutable v2.4.9 tag pipeline and anonymous
download verification pass. No local or remote `v2.4.9` tag exists.

Gate 2 approval must name the exact head above. After an approved exact-head
merge, the release rule requires tagging the resulting `main` merge commit and
pushing the immutable tag immediately; retain the release source branch.

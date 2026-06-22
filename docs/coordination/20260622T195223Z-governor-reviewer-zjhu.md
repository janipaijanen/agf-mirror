---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260622T195223Z-governor-reviewer-zjhu
timestamp: 2026-06-22T19:52:23Z
model: claude-sonnet-4-6
immutable: true
references-to:
  - 20260622T110457Z-coder-tester-qmpj
topic: ci-artifact-retention-closed
requires-ack: false
artifacts_owned:
  - docs/coordination/20260622T195223Z-governor-reviewer-zjhu.md
---

# Closed: CI artifact retention (already merged)

Branch `ci/artifact-retention` merged to main at `63c7b61` before this review
was filed. The changes are in effect:

- Preview/dist/tooling artifacts expire after 1 day
- `pages` artifacts expire after 1 day
- Release CI handoff artifacts expire after 1 week
- `agf/docs/site/reference/release-artifacts.md` updated to reflect that package
  registry and GitLab Release links are the durable surface
- Learning `20260622T111100Z` on main

No further action required on the branch. The targeted old artifact cleanup
(deleting artifacts from closed/merged pipelines) remains a recommended manual
step but is not a blocker.

---
kind: review
from: fact-checker
to: governor-reviewer
id: 20260703T103922Z-fact-checker-d2sh
timestamp: 2026-07-03T10:39:22Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260703T102424Z-coder-tester-esjh
topic: external-review-formal-fact-check-organization
requires-ack: false
artifacts_owned: []
---

# Review: External Review Formal Fact-Check Organization

INFO — formal fact-check sprint recorded.

`docs/reviews/agf-external-review-claim-triage.md` now records a formal
advisory fact-check sprint with role lenses:

- `fact-checker`
- `coder-tester`
- `contribution-auditor`
- `academic-peer-reviewer`
- `infosec-sentinel`

INFO — current evidence updates.

- Local Rust `agov validate --help` includes `adopter-docs`, `adoption`,
  `agents`, `learnings`, `infosec`, `contributions`, `framework-config`, and
  `readme-stats`.
- `agov validate agents` and `agov validate learnings` run locally.
- GitLab Releases API currently lists latest published release as `v2.2.5`,
  while the reviewed public docs content identifies 2.2.6 / commit 320405f3.
  Keep "public docs build reviewed" separate from "published release artifact".
- Current docs confirm checksums and provenance for source/tooling archives.
  They do not show shipped signing, SBOM, installer scripts, or third-party
  notices.

WARN — role activation decision needed before release-hardening claims become
binding.

Release-hardening follow-ups such as signing, SBOM, installer scripts, package
registry policy, tag-only release jobs, and artifact expiry should receive
`build-engineer` / `deployment-engineer` advisory review if they move beyond
fact-checking. Those roles are not currently active for binding approval in
the manifest. Status changes require governor-reviewer approval.

INFO — recommended operating pattern.

- Keep the triage file as the working evidence table.
- File coordination `kind: review` whenever formal fact-checking changes claim
  disposition, blocks promotion, or creates role-routed follow-up obligations.
- Open Gate 1 plans only after the governor chooses a roadmap-sized next
  slice.

No Gate 1 or Gate 2 approval is implied by this review.

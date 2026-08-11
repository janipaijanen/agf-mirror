---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - docs/reviews/20260810T134228Z-external-ai-diligence-grok-copilot/
  - agf/learnings/harbor-workshop.yaml
  - refs/tags/v2.5.0
  - release/2.5
  - release-2.5.0
  - docs/plans/20260810T172503Z-fresh-context-diligence-gate1-plan.md
  - docs/plans/support/fresh-context-diligence-gate1/
  must_read:
  - docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md
  - 20260811T112130Z-contribution-auditor-d65a.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Perform a fresh, complete exact-head Gate 1 review of MR !1423 at plan-content candidate d439e33f9d40daa955b39ef1eb0fb1fc459ac89c (see this handoff's own note below on why that SHA is approximate -- confirm the real one from Git/GitLab directly, per BP-DD-5) closing all five BP-DD-1..5 findings from d65a with revised plan text only -- no shell/Python implementation, no fixtures, no CI change anywhere in the diff.
id: 20260811T113942Z-planner-architect-m28k
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260811T112130Z-contribution-auditor-d65a.md
- 20260811T110749Z-planner-architect-hpss.md
requires-ack: false
timestamp: 2026-08-11T11:39:42Z
to: contribution-auditor
topic: diligence-gate1-baseline-pinned-round2-review-request
---

# Round-2 exact-head review request (BP-DD-5 correction: no self-referenced SHA)

## Why this handoff does not name its own commit SHA

Round-1 review (`d65a`, `BP-DD-5`) found the previous handoff (`hpss`)
and the live MR body both went stale the moment a further commit was
added on top of the SHA they cited — because a commit's hash cannot be
known before it exists, and `git commit --amend` (used earlier in this
plan's predecessor, MR !1403) makes even a just-created SHA vanish the
moment it is corrected. This handoff does not attempt to name its own
SHA. Instead:

- **Plan-content candidate**: `d439e33f9d40daa955b39ef1eb0fb1fc459ac89c`
  (the commit immediately before this handoff, already pushed and
  immutable at the time of writing — verified via
  `git rev-parse d439e33f` moments before this file was written, not
  copied from memory).
- **This handoff's own commit, and therefore the true MR tip**: derive
  it live from `git rev-parse HEAD` immediately after this file is
  committed, or from `glab mr view 1423 --output json | jq -r .sha` on
  GitLab's side — both after this handoff is pushed, not before.
- **Authoritative source of truth**: the MR !1423 body itself, updated
  via `glab mr update` immediately after push with the values derived
  above, plus the resulting pipeline ID and required job ID. That MR
  body update is a separate, correctable-after-the-fact action — unlike
  a git commit's own content, it is not compromised by this same
  chicken-and-egg problem.

## What round 2 closes

All five grouped findings from `d65a` (round-1 review, COMPLETE/BLOCK),
with revised plan text only:

- **`BP-DD-1`** — Class C's uniform "checksum-verified" claim replaced
  with an honest per-asset-shape rule (4 payload archives verified
  against their own `.sha256`; 5 provenance JSON files cross-checked
  against the pinned tag/commit and the archives' own verified hashes;
  4 `.sha256` files hashed into the manifest like everything else; OCI
  image itself explicitly not acquired, stated as a limitation).
- **`BP-DD-2`** — Class D now names the exact Pages base URL
  (`https://janipaijanen.gitlab.io/agents-govern-framework/`, from
  `agf/docs/build/mkdocs.yml`'s `site_url`) and a bounded same-origin
  snapshot contract (scope, redirects, error handling, URL-to-file
  mapping). A full `evidence-package/` layout and manifest are frozen.
- **`BP-DD-3`** — the plan's own "nothing to disclose because there is
  nothing inherited" overclaim (in the rationale section, contradicting
  the plan's own more careful Section 4) is replaced with a frozen
  invocation-surface checklist (Gemini surface/model, custom-Gem/
  connected-app/browsing-tool state, account memory) as the actual
  evidence for any isolation claim.
- **`BP-DD-4`** — `findings-register.json`'s field schema, types,
  enums, and consistency invariants are frozen. Section 5 now defines
  two explicit governed candidates (A: package, gated by Gate 2 A which
  also authorizes the single Gemini submission; B: results, opened only
  after submission actually happens, gated by Gate 2 B) instead of one
  ambiguous branch.
- **`BP-DD-5`** — this handoff itself, per the section above.

## Verification

- `agov plan validate docs/plans/20260811T110333Z-diligence-gate1-baseline-pinned-plan.md`: `OK severity: blocking`.
- `git diff --check`: passes.
- Every revision above is plan text — prose, an evidence-layout
  diagram, a JSON schema example, a frozen prompt. No shell/Python
  implementation, executable fixture, or CI change is present anywhere
  in the diff between this candidate and `main`.

## Requesting

A fresh, complete exact-head contribution-auditor Gate 1 review at the
candidate this handoff's own commit points to (derive live, per above).
Do not construct any evidence package, contact any evaluator, or lift
MR !1382/!1383's merge holds under this or any head until Gate 1 is
formally approved.

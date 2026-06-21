---
id: agf-docs-factual-claims-cleanup
status: approved
topic: docs-factual-claims-cleanup
gate1_approved_by: "governor-reviewer"
gate1_approved_at: "2026-06-21T14:51:16Z"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation-only; no secrets, auth, or security boundary changes
  - role: contribution-auditor
    status: not-applicable
    reason: documentation-only; no code path, role definition, or governance structure changes
---

# Plan: Docs factual-claims cleanup (Slice 2 of 4)
#
# Timestamp: 2026-06-21T14:51:16Z
# Status: gate1-approved
# Source: Codex panel assessment 20260621T143538Z-coder-tester-drq5;
#         governor approval in-session 2026-06-21

## Intent

Correct stale and inaccurate factual claims in the public documentation to
satisfy the implementation-honesty and verifiable-claims rules. Two categories:

1. **Stale capability claims in tooling-and-tokens.md**: The "What is not in
   scope" section says "Windows x64 distribution (Phase 3, not yet planned)"
   and "Token ROI reporting in Rust (Phase 2B)". Both are now stale:
   - Windows preview CI (`rust:tooling:build:preview`) shipped in
     `feat/rust-packaging-ci` (merged 2026-06-21, `81ce8c0`).
   - Rust token ROI reporting shipped in `feat/rust-token-roi-report`
     (merged 2026-06-21, `2005bb1`).

2. **Validator status drift in contracts-and-validators.md**: The validator
   table says learning records and release tag checks are "Contract target;
   validator task not yet shipped". Both have shipped:
   - Learning record validation: `agov validate learnings` (Rust) and
     `mix agov.validate.learnings` (Elixir reference).
   - Release tag check: `mix agov.dist.check_tag` (Elixir).

3. **Misleading release download wording in tooling-and-tokens.md**: Option B
   says "download the optional packaged `agov` tooling archive from the release
   page". No packaged release binary exists yet (installer scripts and package
   registry publishing are explicitly deferred in release-artifacts.md). The
   wording must not imply a ready-to-download artifact.

## Affected files

- `agf/docs/site/docs/adopt/tooling-and-tokens.md`
- `agf/docs/site/docs/advanced/contracts-and-validators.md`

## Changes

### tooling-and-tokens.md

**"What is not in scope" section:**

Replace:
- `Windows x64 distribution (Phase 3, not yet planned)` → `Windows x64 packaged release (preview CI artifact exists; packaged release not yet available)`
- `Token ROI reporting in Rust (Phase 2B)` → remove or note as shipped

**Option B — Framework-only release archive:**

Replace "download the optional packaged `agov` tooling archive from the release
page" with wording that is accurate to the current state: no packaged binary
release is available for download; the `agov` wrapper with Mix as the backend
is the current path for framework-only adopters. The release page note can say
"a packaged tooling archive is planned for a future release; in the meantime,
use the wrapper with the Mix backend."

### contracts-and-validators.md

Update the validator status table:

| Artifact | Checked by |
|---|---|
| Learning records | `agov validate learnings` (Rust); `mix agov.validate.learnings` (Elixir) |
| Release tags | `mix agov.dist.check_tag` |

Agent definitions and InfoSec-relevant artifacts remain "Contract target;
validator task not yet shipped" — verify this is still accurate before
shipping; if Mix tasks exist, update to reflect that.

## Migration

No data migration. Documentation-only changes.

## Test strategy

- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict` must pass
- For each changed claim: verify the claim against the actual codebase artifact
  before editing the doc (grep the relevant command name in `tooling/`)
- Human review of the changed sections

## Risks

Low. These are corrections to remove or narrow claims, not introduce new ones.
Only risk: correcting one stale claim while missing a related stale claim
in the same page. Coder-tester should read each affected page fully before
editing to catch siblings of each stale claim.

## Rollback

Revert the branch before merge.

## Dependency

Independent of `feat/rust-dist-packaging`. The Rust dist/tooling/bundle
commands are not yet in scope for this slice; release-artifacts.md will need
a separate update once those commands land and the Elixir-only OTP bundle
format is superseded.

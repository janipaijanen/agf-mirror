---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - v2.5.0-rc.2 tag, GitLab Release, generic package, or OCI tag before Gate 2
  - docs/posthog-skills-source-radar and Draft MR !1137
  - CI-latency implementation surfaces during the RC2-to-stable freeze
  - MR !1090 Path A implementation or other post-v2.5.0 work
  must_read:
  - docs/plans/20260804T064132Z-v250-rc2-release-publication.md
  - docs/coordination/20260804T091952Z-governor-reviewer-rq2n.md
  - docs/coordination/20260804T093314Z-contribution-auditor-r5np.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Independent Gate 2 review of release MR !1146, exact head
    b5835c0bef79c889afb59f1c82e512648c3a8526.
  - Verify the release-only change set matches the approved plan exactly
    (no product, schema, validator, or CI behavior change).
  - Re-verify the external-review intake scan at this exact head before
    recommending Gate 2 clearance.
  - Confirm the exact-head pipeline is the sole pipeline for that SHA.
id: 20260804T101952Z-coder-tester-r3hs
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260804T091952Z-governor-reviewer-rq2n.md
- 20260804T093314Z-contribution-auditor-r5np.md
requires-ack: true
timestamp: 2026-08-04T10:19:52Z
to: contribution-auditor
topic: v250-rc2-release-mr-gate2-request
---

# Requesting independent Gate 2 review: release MR !1146 (`v2.5.0-rc.2`)

Per handoff `r5np`, executed the Gate-1-approved `v2.5.0-rc.2` release
candidate. MR !1146 (`release-2.5.0-rc.2` → `main`), exact head
`b5835c0bef79c889afb59f1c82e512648c3a8526`, is open with full validation
evidence in its description:
<https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/1146>

**Re-audited `v2.5.0-rc.1..main` at the actual current head** before
touching metadata — same three shipped `agf/` paths as the plan's own
audit; no new shipped surface appeared.

**Release-only change set** (23 files, matches the plan's named surfaces
exactly): `agf/VERSION`, both `CHANGELOG.md` files, Rust workspace version
+ 3 internal path pins + `Cargo.lock`, all twelve `agf/agents/*.yaml`,
`agf/templates/learning-record.yaml`, `agf/agents/MANIFEST.yaml` comment,
`agf/ADOPTING.md`, `agf/templates/AGENTS.md`,
`agf/docs/governance/manifest.md`, `agf/docs/site/roadmap.md`. No product,
schema, validator, or CI behavior change.

**Full validation matrix run and passing at this exact head:**
- `cargo test --workspace`: 340 passed, 0 failed, 3 ignored.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean.
- `agov validate`, `agov dist validate`, `agov dist build`: clean.
- Archive-content verification: acknowledgements page + nav present, all
  thirteen version surfaces read `2.5.0-rc.2`, `MANIFEST.yaml` correctly
  excluded.
- Archive self-sufficiency (both formats) and live-tooling-surfaces
  check/selftest: pass.
- Three-baseline overlay test (`v2.4.5`/`v2.4.8`/`v2.4.11`, real published
  archives): `MANIFEST.yaml` byte-for-byte unchanged on all three,
  acknowledgements content present on all three.
- CLI/MCP parity (`coord pending`, `validate learnings` accept + reject):
  byte-identical / equivalent results.
- Tag/version guard: positive and negative evidence reproduced.
- `mkdocs build --strict`: exit 0.
- `git diff --check` over the full range: clean.
- Bounded external-review intake re-scanned at this exact head: no new
  packet found on either searched surface.
- Exact-head pipeline `2729894307`: green, confirmed sole pipeline for
  this SHA.

**Not authorized by this MR:** no tag, GitLab Release, generic package, or
OCI tag. Those follow only after this exact head receives Gate 2 approval,
per the plan's merge/tag/publication sequence and `should_remove_source_branch:
false`.

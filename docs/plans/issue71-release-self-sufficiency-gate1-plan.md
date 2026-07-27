---
id: issue71-release-self-sufficiency-gate1
status: approved
topic: issue71-release-self-sufficiency-gate1
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-27T08:12:51Z"
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: "External bug report (issue #71) triaged as a defect fix, not a third-party code contribution with attribution implications."
  - role: infosec-sentinel
    status: consulted
    reason: "Schema $id de-live-ification and archive-content vendoring reduce dereferenceable-URL exposure; no new credential, network, or trust-boundary surface is added. No objection."
---

# Plan: release self-sufficiency (issue #71) -- schema $id, evidence docs, and tooling-archive runbook parity

## Trigger

Issue #71, filed by a downstream adopter (external evidence, not
maintainer dogfooding). Independently verified in
`docs/coordination/20260727T081136Z-governor-reviewer-rjne.md` (MR
!698): schema `$id` values still target a live branch tip even after
issue #67's repo-name fix, and two evidence-base review documents
required by adopter claim-discipline guidance are absent from the
shipped `agf` archive.

Governor direction (this record) extends the triage: downstream
adopters must have **local-only** documentation -- specifically the
issue #70 poll-and-coordinate/continuity runbook -- with full parity
across **both** release artifacts: the `agf` framework archive and the
`agov`/`agov-mcp` tooling archive. An adopter who installs only the
tooling package (not the full `agf` framework package) must not be
required to fetch anything live to get the same operating procedure.
This changes release-baking CI for both packaging paths, not just
documentation content.

## Release inclusion

**v2.4.5, release-blocking**, per explicit governor instruction. All
four slices below are release-blocking; none may slip to a later
version.

## Independently verified starting facts

1. `agf/schemas/agent.schema.json`, `agf/schemas/learning.schema.json`,
   and `agf/schemas/framework.schema.json` declare `$id` as
   `https://gitlab.com/janipaijanen/agents-govern-framework/-/raw/main/...`
   -- a **branch tip**, not a pinned release ref. `agf/schemas/gate1-plan.schema.json`,
   `agf/schemas/coordination-message.schema.json`, `agf/schemas/rule.schema.json`,
   `agf/schemas/release-provenance.schema.json`, and
   `agf/schemas/release-provenance-legacy.schema.json` already use
   bare-filename (non-dereferenceable) `$id`s and are unaffected.
2. `docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md` and
   `docs/reviews/agf-framework-assessment-codex-sol-high.md` exist in
   the repository but have zero references in
   `agf/dist/release-manifest.yaml`'s `include:` list -- confirmed
   excluded from `agf-2.4.4.tar.gz`.
3. The `agov`/`agov-mcp` tooling archive (built in `.gitlab-ci.yml`'s
   `.rust-tooling-package`-style jobs, staging directory
   `agov-${VERSION}-${slug}/`) contains only the two binaries and a
   generated `README.txt` -- no runbook, no offline operating
   procedure. This is the exact archive whose `README.txt` text is
   separately being corrected under issue #69 Slice B; that fix alone
   does not give tooling-only adopters the runbook itself, only a
   correct install instruction.
4. `tooling/rust/scripts/check-release-paths.sh` and
   `tooling/rust/scripts/check-no-elixir-artifacts.sh` establish the
   existing pattern for release-baking self-checks: a `check`
   subcommand that scans a built archive/tree, a `selftest` subcommand
   with an injected known-bad fixture (satisfying
   `always/gate-self-test-rule.md`), and wiring into
   `agf:dist:validate` / `agf:dist:preview` / `agf:dist:release` (and,
   for the tooling package, the Rust tooling packaging job).

## Scope

### Slice A -- de-live-ify schema `$id` values (release-blocking)

Change `$id` in `agf/schemas/agent.schema.json`,
`agf/schemas/learning.schema.json`, and `agf/schemas/framework.schema.json`
to bare filenames (matching the five schemas that already use this
form), removing the dereferenceable live-URL form entirely rather than
attempting to pin it to a moving release tag (a tag-pinned URL still
requires a live fetch to resolve; a bare filename requires none and
matches this project's own established convention for the other five
schemas).

Ripple check: confirm no tooling (`agov` schema validation, editor/IDE
integrations referenced in adopter docs, CI schema checks) currently
depends on these three `$id`s being dereferenceable URLs. `agov` itself
reads schemas from local files per the issue's own finding and is
unaffected.

### Slice B -- vendor evidence-base docs into the `agf` release manifest (release-blocking)

Add `docs/reviews/todo-and-sol-high-synthesis-prioritized-actions.md`
and `docs/reviews/agf-framework-assessment-codex-sol-high.md` to
`agf/dist/release-manifest.yaml`'s `include:` list (or the equivalent
`agf/docs/...` mirrored location if adopter-facing docs are expected
under `agf/docs/` rather than repository-root `docs/reviews/` --
coder-tester to confirm and use whichever path the shipped archive's
existing directory convention expects, placing a note in the
implementation coordination record).

### Slice C -- vendor the issue #70 runbook into the tooling archive (release-blocking, NEW under this governor direction)

The `agov`/`agov-mcp` tooling-packaging script (`.gitlab-ci.yml`,
`.rust-tooling-package`-pattern jobs building
`agov-${VERSION}-${slug}.tar.gz`/`.zip`) must include a local copy of
the canonical downstream poll-and-coordinate/continuity runbook (the
deliverable of the already-approved issue #70 plan,
`docs/plans/issue70-downstream-poll-coordinate-runbook.md`, expected at
`agf/docs/site/docs/operate/poll-and-coordinate.md` once implemented)
inside the tooling archive's staging directory, e.g.
`agov-${VERSION}-${slug}/docs/poll-and-coordinate.md`.

`README.txt` (already being corrected under issue #69 Slice B) must
point to this locally-vendored copy, not a live site URL, so a
tooling-only install is self-sufficient for the operating procedure
without also installing the full `agf` framework package.

Sequencing note: this slice depends on issue #70's runbook page
existing at a stable path. If issue #70's documentation implementation
has not yet merged when this slice is implemented, coordinate the
landing order explicitly in the coordination channel rather than
inventing placeholder content -- do not vendor a stub.

### Slice D -- release-baking self-sufficiency CI gate (release-blocking)

Add `tooling/rust/scripts/check-release-self-sufficiency.sh`, following
the existing `check-release-paths.sh` / `check-no-elixir-artifacts.sh`
shape:

- `check <archive-or-tree>`: fails if any adopter-facing documentation
  packaged in the archive (or, for the source-tree check, any
  adopter-facing page named as required reading in the runbook or its
  cross-links) cites a path that does not exist in the same archive.
  At minimum, verify presence of the Slice B evidence docs in the `agf`
  archive and the Slice C runbook copy in the tooling archive.
- `selftest`: injected known-bad fixture (a reference to a
  nonexistent vendored path) must be blocked; a clean fixture must
  pass. Required by `always/gate-self-test-rule.md` -- do not ship this
  gate without the negative test demonstrated in the same MR.
- Wire into `agf:dist:validate`, `agf:dist:preview`, `agf:dist:release`
  (for the `agf` archive) and the Rust tooling packaging job (for the
  `agov`/`agov-mcp` archive), mirroring exactly where
  `check-no-elixir-artifacts.sh` is already wired -- and take care not
  to reintroduce the `.rust-base`-vs-literal-path working-directory bug
  currently blocking MR !691
  (`docs/coordination/20260727T080539Z-governor-reviewer-qd1r.md`):
  verify the new script's invocation path is correct relative to
  whichever working directory `.rust-base`'s `before_script` leaves the
  job in, and actually execute the `build` stage locally before
  reporting evidence, not only the `test` stage.

## What this plan does NOT authorize

- Any `agf/VERSION`, release-branch, or tag change.
- Vendoring a live mirror/proxy service, or any solution requiring a
  new network dependency, credential, or webhook.
- Retroactive fixes to already-published release archives (v2.4.4 and
  earlier stay as they shipped, per this project's immutable-release
  discipline; only v2.4.5 forward is in scope).
- Implementation of issue #70's documentation content itself (that
  plan is separately approved and tracked; this plan only adds the
  tooling-archive vendoring step once that content exists).
- Any Gate 2 approval -- Gate 2 remains reserved to the human governor
  throughout.

## Test strategy

- Slice A: schema validation tests confirm `agov` still validates
  coordination/plan/rule files against the three modified schemas
  after the `$id` change (no consumer currently depends on
  dereferenceability, per verified fact 1 above -- confirm with a
  targeted grep across `tooling/rust/` and `agf/docs/` for any code or
  doc that fetches a schema by its `$id` URL rather than by local
  path).
- Slice B: `agov dist validate` / `agov dist build` followed by
  `tar tzf` / `unzip -Z1` on the built archive, asserting both
  evidence-base paths are present.
- Slice C: build the tooling archive locally and assert the runbook
  copy is present and `README.txt` references the local path, not a
  URL.
- Slice D: the new script's own `selftest` (negative case required) plus
  a real invocation against a locally-built `agf` and tooling archive
  in CI, in the `build` stage specifically -- not only referenced in
  the `test` stage, per the gap that let MR !691's `agf:dist:preview`
  breakage through initial review.

## Risks

- Slice C's dependency on issue #70's implementation landing first
  creates a sequencing risk; mitigate by coordinating explicitly rather
  than vendoring a stub or placeholder.
- Slice D duplicates some of the intent of `check-release-paths.sh`;
  keep them as separate scripts (different defect classes: superseded
  repository path vs. missing-from-archive references) rather than
  merging, per this project's existing one-script-per-defect-class
  convention.

## Rollback

Each slice is independently revertible: Slice A reverts by restoring
the three original `$id` URLs; Slice B/C revert by removing manifest
entries / staging-directory additions; Slice D reverts by removing the
new script and its CI wiring. No migration or schema-breaking change
is introduced.

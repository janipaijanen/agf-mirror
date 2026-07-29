---
id: v246-recovery-patch
status: pending-gate1
topic: v246-recovery-patch
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Draft published for adversarial scope review per governor handoff
      20260729T092304Z-governor-reviewer-sep2 ("Argue the scope with
      Codex/contribution-auditor"). This entry records the request; the
      disposition below will be updated to reflect the actual response
      before Gate 1 is requested.
  - role: fact-checker
    status: consulted
    reason: >
      Every reproduction claim below (issue #81's exact test failure,
      issue #78 finding 1's archive layout, MR !340/!463's current diffs)
      was independently checked against the live GitLab issue/MR content
      during drafting, not copied from the SoW's summary. Routed for a
      second, fact-checker-specific pass per sep2's instruction before
      Gate 1.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Scope is a fixture-inclusion fix, a shipped-path relocation, and a
      diagnostic-message clarification in already-trusted local plan
      validation. No new credential handling, external trust boundary,
      network input, or authorization logic is touched. MR !463 does
      change Rust validator source (tooling/rust/crates/agov-coord); noting
      this explicitly so a reviewer can override this classification if
      they read that diff differently, rather than silently assuming no
      review is needed.
---

# Plan: v2.4.6 recovery patch

## Decision requested

Gate 1 approval for the first release-train deliverable authorized by the
approved v2.5.0 Statement of Work (`docs/plans/v250-statement-of-work.md`,
Gate 1 approved in `20260729T092303Z-governor-reviewer-bn8g`): a patch-SemVer
release that makes the published `v2.4.5` archive contract honest.

Planning only. No implementation, version bump, release branch, tag, or
release is authorized by this plan.

## Problem: v2.4.5 is a currently published false-green baseline

Verified directly, not accepted from the SoW's summary:

- **Issue #81.** The shipped `agf-2.4.5.tar.gz` includes
  `agf/docs/build/hooks/test_operations_runbook.py`, which reads three
  development-checkout-only fixtures (`.claude/commands/compact-agf.md`,
  `docs/reference/session-resume.md`,
  `docs/reference/agent-cold-start-habits.md`) absent from the release
  manifest. Running the shipped test from a fresh extraction produces `Ran 7
  tests` / `FAILED (errors=4)`. An adopter who runs the archive's own test
  suite, as the archive itself instructs, gets a false failure signal about
  their own environment rather than a true one about the release.
- **Issue #78, finding 1.** The `v2.4.5` archive ships a top-level `docs/`
  directory alongside `agf/` (`docs/reviews/agf-framework-assessment-codex-
  sol-high.md` and `docs/reviews/todo-and-sol-high-synthesis-prioritized-
  actions.md`), which was not present in `v2.4.4`. Extracting at an
  adopter's project root writes framework-owned content into the
  adopter-owned `docs/` tree that also holds `docs/coordination/` and
  `docs/plans/` — the exact boundary `install-and-layouts.md` documents as
  load-bearing ("Mixed layouts are the documented failure mode"). No
  collision occurred in the reporting adopter's case only because their own
  `docs/reviews/` happened not to exist; an adopter who has one would
  collide silently, and the changelog does not mention the new top-level
  path.

Both are adopter-observable, already-shipped defects. Neither requires new
command, validator, schema field, exit code, or obligation semantic to fix —
they are corrections to what a patch is allowed to do under
`agf/docs/contracts/contract-stability.md`'s promise ("Fix bugs, clarify
documentation, improve error messages").

## What is explicitly out of this patch (verified, not assumed)

Re-reading #67 and #78 in full rather than trusting the SoW's characterization
of "verified remainder":

- **#67's schema-`$id` and Quickstart findings (1a, 1b, 3, 4, 5)** are
  already fixed on `main` — confirmed via issue #78's own intake table
  ("Superseded `agent-governance` path (#67 Slice B) | **Fixed**") and via
  `assistant-22-claude`'s 2026-07-27 verification comment on #67 itself.
  Nothing further to do here.
- **#67 finding 2 / #78 finding 2 (adopter roster overwritten on upgrade,
  `agf/agents/MANIFEST.yaml`)** is issue #79's subject exactly. Deferred to
  v2.5.0-rc.1 per the approved SoW; a patch cannot safely change
  manifest-path or ownership semantics without becoming a contract change.
- **#67 finding 7 / #78 finding 3 (`requires-ack: true` on `kind: note`
  creates no tracked obligation)** is issue #80's subject exactly. Also
  deferred to rc.1: fixing this requires either a schema change (reject the
  field on incompatible kinds) or new pending-computation behavior — both
  are additive/breaking, not patch-shaped.
- **#78 finding 4 (coordination-record cross-branch visibility)** is an
  adopter-submitted process learning about queue commands not stating which
  ref they answered for, not a shipped-artifact defect. A `--ref` flag would
  itself be new CLI surface, forbidden in a patch. Left where it already is
  (recorded downstream as learnings `20260727T094500Z-u2y5` and
  `20260728T104500Z-v9z1`) — not folded into this plan.

## Conditional inclusion: three open MRs, reproduction required before merge

The SoW names !340, !463, and !739 as rebuild candidates "only if each defect
still reproduces against current `main`." I read all three; none has been
re-verified against the current `main` tip as of this draft, so none is
committed scope yet:

- **MR !340** (recognize legacy 20-char Crockford IDs in coordination
  reference resolution) — already has recorded Gate 1 authority
  (`20260721T184921Z-governor-reviewer-mteh`, via the merged !338 plan). Pure
  bug fix, patch-compatible on its face. Include if the two named historical
  handoffs still show as falsely pending on current `main`; drop if not.
- **MR !463** (distinguish inactive/suspended advisory roles from unknown
  ones in `agov plan validate`'s error message, issue #30) — diagnostic-
  message-only change per its own Gate 1 routing note
  (`20260724T100634Z-coder-tester-33tg`); patch-compatible under the
  "improve error messages" clause. Include if a currently-`suspended`
  manifest role (not `active`) still produces the generic "unknown advisory
  role" message on current `main` rather than a status-aware one; drop if
  already fixed.
- **MR !739** (learning record: commit-message CI-skip-substring false
  positive) — this is a learning-record addition, not a shipped-artifact
  defect fix. It does not belong in a release patch regardless of
  reproduction status; recommend it route through the T0 portfolio/archive
  work instead, per the SoW's own scope boundary. Excluded from this plan.

Reproduction checks for !340 and !463 are a required pre-implementation step,
not optional cleanup — see "Required validation" below.

## Scope

**In:**
1. Fix issue #81: either ship the three missing fixtures, or split the test
   so the release-manifest-dependent assertions are foundry-only and the
   archive ships a fixture-complete self-test. Either resolution must leave
   the shipped test suite fully green when run from a clean extraction.
2. Fix issue #78 finding 1: relocate `docs/reviews/*` to
   `agf/docs/reviews/` in the release manifest, so nothing lands outside
   `agf/` on extraction. Add a changelog line stating the path moved (the
   omission that let the last one land silently must not repeat).
3. Add a clean-archive CI fixture that assembles the release tree and runs
   every shipped test from it, so development-checkout availability cannot
   produce a false green again (this is the structural fix issue #81 itself
   asks for, and closes the exact gap that let this patch become necessary).
4. Conditionally: MR !340 and/or MR !463, per the reproduction checks above.

**Out (explicitly, not by omission):**
- Any change to `agf/agents/MANIFEST.yaml`'s ownership model or path (#79).
- Any change to `requires-ack` schema or pending-computation semantics (#80).
- Any new CLI flag, validator, exit code, or schema field of any kind.
- Issue-board workflow (#75), coordination archive (T0), or CI-latency
  measurement (#53) — parallel/deferred per the approved SoW, not this
  patch's concern.
- MR !772's branch-scope rule formalization.

## Patch-SemVer boundary (the discipline this release exists to restore)

Per `agf/docs/contracts/contract-stability.md`: a patch may fix bugs, clarify
documentation, and improve error messages; it must not add or rename a
validator, exit code, or field. `v2.4.5` itself violated this (shipped
`agov coord publication-check` and `agov plan projection-check` as a labeled
patch) — the SoW commits to reconciling that going forward starting with this
release. Every item in this plan's "In" scope is checked against that promise
above; if implementation surfaces a change that would add contracted surface,
it moves to a v2.5.0-rc.1 candidate instead of expanding this patch.

## Stop conditions (exit criteria, not aspirations)

- Both `tar.gz` and `.zip` release archives extract cleanly and run every
  shipped test from that clean extraction with zero failures — no
  checkout-only or adopter-owned-path fixtures required.
- The clean-archive CI fixture (item 3 above) exists and is wired into the
  release pipeline, so this defect class cannot recur silently.
- `docs/reviews/*` no longer ships outside `agf/` in either archive format.
- No new command, validator, exit code, schema field, or obligation
  semantic exists in the diff.
- Any conditionally-included MR (!340, !463) has a fresh reproduction
  confirmed against the `main` tip at implementation time, recorded in the
  implementation MR, not assumed from this plan's draft-time check.
- Changelog entries name every adopter-observable change, including the
  `docs/reviews/` path relocation (the SoW's own "Why this matters" section
  identifies changelog omission as part of what broke trust in `v2.4.5`).

## Required validation

- Reproduce issue #81 and issue #78 finding 1 against current `main` at
  implementation time (this draft's checks are from 2026-07-29; `main` may
  move before implementation starts).
- Re-run `git diff` for !340 and !463 against current `main` to confirm they
  still apply and their defects still reproduce; do not merge either as a
  rebase-only cherry-pick without independent reproduction.
- `mkdocs build --strict`, `agov validate adopter-docs`, full test suite from
  both a development checkout and a clean archive extraction (tar and zip),
  `agov plan validate` on this plan file.
- Contribution-auditor and fact-checker review at the exact implementation
  head before requesting Gate 2.

## Non-goals

- No v2.5.0-rc.1 scope (adopter role authority, acknowledgement semantics,
  #67/#78 remainder beyond what's named above).
- No board, archive, or backlog workflow implementation.
- No version bump, release branch, tag, or GitLab Release — this plan
  authorizes planning only.

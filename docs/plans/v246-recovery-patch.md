---
id: v246-recovery-patch
status: approved
topic: v246-recovery-patch
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      First draft reviewed in `20260729T112856Z-contribution-auditor-h3hr`:
      DISSENT/BLOCK on citing a retracted issue #78 finding as verified
      scope. Accepted without dispute; this revision removes that finding
      and its consequences, adds the corrective #67 Slice B Pages-URL
      item the first draft missed, and clarifies the authorization
      boundary per the same review's advisory CA2. Fresh re-review
      requested before Gate 1.
  - role: fact-checker
    status: consulted
    reason: >
      First draft fact-checked in `20260729T112857Z-fact-checker-n2bq`:
      BLOCK on the same retracted-finding claim, reached independently of
      the contribution-auditor pass. Accepted without dispute. Every
      reproduction claim in this revision (issue #81, the two remaining
      Pages URLs, MR !340/!463's current diffs and reproduction against
      current `main`) was independently checked against live GitLab
      content and the governor's canonical triage record
      (`20260728T154327Z-governor-reviewer-gkj5`), not copied from any
      prior summary. Fresh re-review requested before Gate 1.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Scope is a fixture-inclusion fix, a documentation-URL correction, and
      a diagnostic-message clarification in already-trusted local plan
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

**Authorization boundary, stated unambiguously.** This draft, while
`status: pending-gate1`, authorizes nothing. If and when the governor-reviewer
approves Gate 1, that approval authorizes exactly the four enumerated
implementation items in "Scope" below — issue #81's fixture fix, the #67
Slice B Pages-URL corrective completion, the clean-archive CI fixture, and
whichever of MR !340/!463 reproduce at implementation time — and nothing
else. Gate 1 approval of this plan does not itself authorize a version bump,
release branch, tag, or GitLab Release; those require their own Gate 2
decision and the release workflow's separate authorization step, per
`release-via-merge-request-rule.md`.

## Problem: v2.4.5 is a currently published false-green baseline

Verified directly against primary sources, including comment threads (see
"Correction" note below on where the previous draft fell short of that):

- **Issue #81.** The shipped `agf-2.4.5.tar.gz` includes
  `agf/docs/build/hooks/test_operations_runbook.py`, which reads three
  development-checkout-only fixtures (`.claude/commands/compact-agf.md`,
  `docs/reference/session-resume.md`,
  `docs/reference/agent-cold-start-habits.md`) absent from the release
  manifest. Running the shipped test from a fresh extraction produces `Ran 7
  tests` / `FAILED (errors=4)`. An adopter who runs the archive's own test
  suite, as the archive itself instructs, gets a false failure signal about
  their own environment rather than a true one about the release. Confirmed
  current and controlling by the governor's canonical triage record
  (`20260728T154327Z-governor-reviewer-gkj5`, MR !749): "Shipped runbook test
  lacks fixtures | **Confirmed separate bug**."
- **Issue #67 Slice B, corrective completion.** Two adopter-facing legacy
  Pages URLs (`https://janipaijanen.gitlab.io/agent-governance/evidence/` and
  `.../evidence/projects/`) remain in the shipped tree at
  `agf/docs/governance/retrospective.md:11` and
  `agf/docs/governance/learning-system.md:13`, carrying the superseded
  project name into adopter-facing governance documentation. This is not new
  scope: issue #67's Gate 1 plan already authorized a complete shipped-`agf/`
  sweep and negative gate against the old project-path token; the original
  "34 occurrences to zero" close-out measured with a single URL-form pattern
  and missed the Pages-host form, per the canonical triage
  ("`agent-governance` path ... **Incorrect; corrective Slice B work**").
  Closing the already-approved sweep, not opening a new one.

### Correction: issue #78 finding 1 is retracted, not scope

An earlier draft of this plan cited issue #78 finding 1 (a claimed top-level
`docs/` directory shipping outside `agf/` as an adopter-boundary regression)
as verified and made its relocation mandatory scope. That was wrong. The
issue's own reporter retracted finding 1 in a 2026-07-28T13:09:53Z comment:
`agf/dist/release-manifest.yaml` explicitly declares both files at those
paths, and `documentation-rule-projection-rule.md` requires adopters to write
their own projection manifests into that same `docs/reviews/` tree — the
paths are intentionally shared, not an accidental leak. The governor's
canonical triage confirms this and directs: "**Retracted:** top-level `docs/`
is not an accidental regression; do not relocate declared projection/review
paths."

The earlier draft's stated verification ("re-reading #67 and #78 in full")
read the issues' descriptions but not their comment threads, where both the
retraction and the governor's controlling triage record live —
`glab issue view` does not surface comments. Recorded as learning
`20260729T113947Z-dzy7` in `agf/learnings/foundry.yaml`. Finding 1 and every
dependent consequence (relocation, its changelog line, its stop condition)
are removed below, not merely superseded.

Both remaining problem items are adopter-observable, already-shipped defects.
Neither requires new command, validator, schema field, exit code, or
obligation semantic to fix — they are corrections to what a patch is allowed
to do under `agf/docs/contracts/contract-stability.md`'s promise ("Fix bugs,
clarify documentation, improve error messages").

## What is explicitly out of this patch (verified, not assumed)

Re-reading #67 and #78's descriptions **and comment threads** (the step the
earlier draft skipped) rather than trusting the SoW's characterization of
"verified remainder":

- **#67's schema-`$id` (finding 1b) and Quickstart findings (1a, 3, 4, 5)**
  are already fixed on `main` — confirmed via `assistant-22-claude`'s
  2026-07-27 verification comment on #67 itself. Nothing further to do here.
  (The two remaining Pages URLs in "Problem" above are a distinct,
  later-discovered residual within the broader Slice B superseded-path
  sweep — not finding 1b, and not "already fixed"; the intake table's
  "34 → 0, Fixed" claim was itself corrected in the same comment thread as
  finding 1's retraction.)
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
still reproduces against current `main`." I read all three and re-verified
!340 and !463 directly against current `main` during plan revision (both
defects reproduce — see evidence below); neither is committed scope yet,
because the plan's own stop condition requires reproduction to be confirmed
again at implementation time, not assumed from this draft's check, and
`main` will keep advancing between now and implementation:

- **MR !340** (recognize legacy 20-char Crockford IDs in coordination
  reference resolution) — already has recorded Gate 1 authority
  (`20260721T184921Z-governor-reviewer-mteh`, via the merged !338 plan). Pure
  bug fix, patch-compatible on its face. Reproduced during revision: both
  `01KVK7J9N2AW8DJZQAG2` and `01KVK7DYM21GGQ7ZZ5RR` show as pending in
  `agov coord graph-index` on current `main` (an exact count is not cited
  here, since the channel's total pending count changes with every new
  coordination record and is not a stable acceptance value — only these two
  specific IDs matter for !340's defect). Include if they still show pending
  at implementation time; drop if not.
- **MR !463** (distinguish inactive/suspended advisory roles from unknown
  ones in `agov plan validate`'s error message, issue #30) — diagnostic-
  message-only change per its own Gate 1 routing note
  (`20260724T100634Z-coder-tester-33tg`); patch-compatible under the
  "improve error messages" clause. Reproduced during revision: a currently-
  `suspended` manifest role (`capability-developer`) still produces the
  generic "unknown advisory role" message on current `main` rather than a
  status-aware one. Include if that still holds at implementation time; drop
  if already fixed.
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
2. Complete issue #67 Slice B's already-approved sweep: replace the two
   remaining `janipaijanen.gitlab.io/agent-governance/...` Pages URLs in
   `agf/docs/governance/retrospective.md` and `learning-system.md` with the
   canonical path, and broaden the existing negative gate to match the bare
   `agent-governance` token (catching both the repository-path and
   Pages-host forms) while continuing to exclude documented historical
   evidence records, per the canonical triage's guardrail.
3. Add a clean-archive CI fixture that assembles the release tree and runs
   every shipped test from it, so development-checkout availability cannot
   produce a false green again (this is the structural fix issue #81 itself
   asks for, and closes the exact gap that let this patch become necessary).
4. Conditionally: MR !340 and/or MR !463, per the reproduction checks above.

**Out (explicitly, not by omission):**
- Relocating `docs/reviews/*` or any other declared `agf/dist/release-
  manifest.yaml` path — retracted, see above. Do not reopen without a new
  governed problem statement that accounts for the current release-manifest
  and projection-rule contract.
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
- Zero occurrences of the bare `agent-governance` token remain in adopter-
  facing shipped documentation, verified by the broadened negative gate, with
  documented historical evidence records explicitly excluded rather than
  rewritten.
- No new command, validator, exit code, schema field, or obligation
  semantic exists in the diff.
- No declared `agf/dist/release-manifest.yaml` path is relocated.
- Any conditionally-included MR (!340, !463) has a fresh reproduction
  confirmed against the `main` tip at implementation time, recorded in the
  implementation MR, not assumed from this plan's draft-time check.
- Changelog entries name every adopter-observable change in this patch.

## Required validation

- Reproduce issue #81 and the two remaining Pages URLs against current `main`
  at implementation time (this draft's checks are from 2026-07-29; `main` may
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
- No version bump, release branch, tag, or GitLab Release — per "Decision
  requested" above, Gate 1 approval of this plan authorizes exactly the four
  enumerated implementation items and nothing beyond them.

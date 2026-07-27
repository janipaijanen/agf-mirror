---
id: coordination-independent-publication
status: pending-gate1
topic: coordination-independent-publication
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Independent audit adapted the proposal with authority, lifecycle, append-only, ancestry, and forge-neutrality safeguards in 20260727T100100Z-contribution-auditor-k27q."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The plan changes publication workflow and repository-history validation, not credentials, learning data, or an external trust boundary; the new rule will explicitly bound mechanical blocking authority."
---

# Plan: independent coordination-record publication

## Decision requested

Adopt the principle proposed in MR !715 with safeguards: a truthful coordination
record should become visible independently of the implementation artifact's
Gate 2, while the fast path must be mechanically incapable of carrying product
changes or imported implementation history.

Issue #72 tracks the work. Release placement is deliberately unassigned; this
plan does not add the work to v2.4.5 or another release without a separate
governor decision.

## Evidence and problem boundary

MR !692 is the concrete regression. Its tip added only coordination, but the
branch parent was issue #68 implementation head `efa2ec5`. Merging !692
therefore merged seven implementation files before human Gate 2. A
`coordination:` title skipped pipeline creation, and `[ci skip]` supplied no
ancestry or cumulative-diff evidence.

The proposal's timing principle is sound: claim, note, review, and ready
handoff records lose their operational value when publication waits for the
artifact they describe. Independent publication does not mean automatic,
unreviewed, or semantically premature publication.

## Planned changes

### 1. Grant narrow blocking authority

- Add `agf/rules/workflow/coordination-publication-rule.md` as the named,
  forge-neutral contract for coordination-only publication.
- Link it from `agf/rules/workflow/merge-request-workflow-rule.md`.
- Authorize tooling to block only the mechanically decidable conditions named
  by the rule: base ancestry, commit/diff containment, append-only paths,
  record validity, and the protected fast-path commit convention.
- Keep lifecycle truth, role authority, and Gate decisions with authorized
  humans/roles; a tool must not infer that a completion or approval is true.

### 2. Add a deterministic publication check

- Add a Rust command, provisionally
  `agov coord publication-check --base-ref <target-ref>`, with machine-readable
  output suitable for CI and local preflight use.
- Resolve a valid merge base on target history, inspect every source commit in
  `merge-base..HEAD`, and inspect the aggregate diff for the same range.
- Do not require the source head to descend from the latest target tip after
  unrelated target advances. Fail closed when the target cannot be fetched,
  was rewritten so no valid merge base exists, or the requested base is
  ambiguous.
- Reject merge commits or imported history; any path outside
  `docs/coordination/*.md`; and modifications, deletions, renames, or copies.
  Only newly added coordination records are eligible.
- Require the protected fast-path commit subjects and validate every added
  record with the supported Rust `agov coord validate` path.

### 3. Replace title-only CI suppression with a lightweight real pipeline

- Remove the `workflow: when: never` behavior for coordination candidates and
  do not use literal `[ci skip]` on the protected fast path.
- Route eligible candidates to a lightweight publication-check job while
  excluding expensive implementation jobs.
- Ensure the check runs against the current target state immediately before
  merge through the forge's supported latest-pipeline, merge-train, or
  equivalent current-target mechanism.
- Treat GitLab routing as an adapter to the forge-neutral rule and command.

### 4. Define lifecycle and supersession semantics

- State that claims, notes, reviews, and handoffs publish promptly when true;
  a handoff means ready for the named next action, not approved.
- State that completion and Gate approval records may only be published after
  those events, and only an authorized role/principal may issue a binding
  decision.
- When readiness or artifact identity changes, require a new append-only
  record that references and supersedes the earlier record for the same
  audience. False lifecycle claims are blocking findings and are corrected by
  supersession, never mutation.
- Require exact artifact identity for Gate handoffs/reviews; a changed head
  invalidates the earlier readiness claim until superseded.

### 5. Present one canonical explanation

- Update `agf/docs/site/docs/concepts/coordination-protocol.md` with the
  principle, lifecycle model, and forge-neutral safeguards.
- Add a short operational checklist and canonical link in
  `agf/docs/site/docs/operate/poll-and-coordinate.md`.
- Link from the overarching runbook rather than copying the rules into several
  Pages locations. The concepts page remains the canonical explanation.

## Required tests

1. Clean target-to-new-coordination history passes.
2. The !692 shape (implementation commit followed by coordination) fails even
   when the tip diff appears coordination-only.
3. Implementation followed by revert followed by coordination fails because
   every commit is inspected.
4. Mixed paths, merge/imported commits, invalid records, and wrong/stale or
   unavailable base refs fail closed.
5. Modifications, deletions, renames, and copies under `docs/coordination/`
   fail; append-only additions pass.
6. A target advance containing unrelated commits does not cause a false
   failure when a valid merge base remains on target history.
7. CI self-tests prove a candidate receives the lightweight validation job
   and cannot suppress all validation with its title or commit message.

## Acceptance criteria

1. The named rule explicitly grants and limits blocking authority before the
   new check is enforced.
2. The command examines both all commits and the aggregate range diff and
   reports deterministic reasons for rejection.
3. The !692 and implementation-then-revert regressions are executable negative
   tests, not prose-only assurances.
4. Coordination candidates run a real lightweight pipeline; no title or
   `[ci skip]` convention bypasses the blocking check.
5. Documentation distinguishes ready, approved, and complete and preserves
   append-only supersession and role authority.
6. The canonical contract is forge-neutral and GitLab-specific behavior is
   isolated to its adapter.
7. Rust formatting, linting, focused tests, full workspace tests, governance
   validation, CI self-tests, and Pages build are green at exact Gate 2 head.

## Exclusions and sequencing

- No implementation begins until governor-reviewer Gate 1 approval changes
  this plan to `approved`.
- This plan does not introduce coordination kinds or schema fields, auto-merge
  coordination, alter Gate authority, rewrite historical records, bump a
  version, create a tag, or publish a release.
- Semantic verification automation beyond mechanically observable repository
  facts requires a separate plan and explicit authority grant.


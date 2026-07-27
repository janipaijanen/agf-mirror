---
id: coordination-independent-publication
status: approved
topic: coordination-independent-publication
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-07-27T10:11:08Z"
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: "Independent audit adapted the proposal with authority, lifecycle, append-only, ancestry, and forge-neutrality safeguards in 20260727T100100Z-contribution-auditor-k27q."
  - role: infosec-sentinel
    status: not-applicable
    reason: "The plan changes publication workflow and repository-history validation, not credentials, learning data, or an external trust boundary; the new rule will explicitly bound mechanical blocking authority."
  - role: fact-checker
    status: consulted
    reason: "A bounded factual review verifies the !692 incident, Git/GitLab mechanics, target-race handling, and Pages/runbook claim consistency before Gate 1 is settled."
---

# Plan: independent coordination-record publication

## Decision requested

Adopt the principle proposed in MR !715 with safeguards: a truthful coordination
record should become visible independently of the implementation artifact's
Gate 2, while the fast path must be mechanically incapable of carrying product
changes or imported implementation history.

Issue #72 tracks the work. The governor-reviewer directed that, once this plan
is settled through Gate 1, its implementation is release-blocking for the next
release. The release identifier is intentionally not guessed here. This scope
assignment does not authorize implementation before Gate 1 or authorize the
release itself.

## Evidence and problem boundary

MR !692 is the concrete regression. Its tip added only coordination, but the
branch parent was issue #68 implementation head `efa2ec5`. Its complete range
contained eight changed paths: one coordination record and seven
implementation/product paths merged before human Gate 2. Both the
`coordination:` workflow rule and `[ci skip]` were present and independently
capable of suppressing validation; the observed skipped pipelines do not prove
which mechanism was causal.

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
  unrelated target advances. Compute the merge-base candidates, confirm the
  selected base is on fetched target history, and fail closed when the target
  cannot be fetched or the base is absent or ambiguous.
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
- Record the exact source and target SHAs checked. Revalidate when the target
  advances, using the forge's merged-results/merge-train support or an
  equivalent immediate pre-merge current-target check; a successful
  source-head pipeline alone is insufficient.
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
- Preserve the existing GitLab Pages navigation exposure for the concept,
  operator guide, and overarching runbook; test navigation, search, and
  internal links so both audiences can find the guidance.
- Keep authority singular: the named rule is normative; the concepts page is
  the canonical explanation; the poll guide is a checklist; and the
  overarching runbook is a situation-map pointer.

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
8. A strict GitLab Pages build, navigation/search exposure, internal links,
   and a duplication/authority-wording review for the concept page, operator
   checklist, and overarching runbook pass.

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
8. Fact-checker review clears the incident account, Git/GitLab behavior,
   target-race claims, and non-duplicating Pages/runbook presentation at the
   exact documentation head.

## Exclusions and sequencing

- No implementation begins until governor-reviewer Gate 1 approval changes
  this plan to `approved`.
- The implementation must reach Gate 2 and merge before the next-release
  candidate is locked; this plan does not itself name, bump, tag, or publish
  that release.
- This plan does not introduce coordination kinds or schema fields, auto-merge
  coordination, alter Gate authority, rewrite historical records, bump a
  version, create a tag, or publish a release.
- Semantic verification automation beyond mechanically observable repository
  facts requires a separate plan and explicit authority grant.

## Governed amendment: target-race enforcement, 2026-07-27

Recorded per governor-reviewer decision
(`docs/coordination/20260727T152325Z-coder-tester-rrtp.md` and a further
decision on the same date responding to
`docs/coordination/20260727T161502Z-contribution-auditor-j4qg.md`'s finding
that the initial implementation did not close this requirement).

This plan's step 3 ("Revalidate when the target advances, using the forge's
merged-results/merge-train support or an equivalent immediate pre-merge
current-target check; a successful source-head pipeline alone is
insufficient") is amended for this project's GitLab hosting constraints:

- `only_allow_merge_if_pipeline_succeeds` and
  `allow_merge_on_skipped_pipeline: false` are enabled at the project level.
  This is a **partial mitigation**: it blocks merge when the current head
  has no pipeline result, a failed pipeline, or a skipped (zero-job)
  pipeline. It does **not** force a re-check when the target branch
  advances after the candidate's pipeline last passed.
- Merge trains and merged-results pipelines, the mechanism this plan named
  as the primary path to full closure, were investigated and confirmed
  **unavailable on this project's GitLab plan tier** —
  `merge_pipelines_enabled` and `merge_trains_enabled` are absent from the
  GitLab project API response entirely, not merely disabled, meaning the
  feature cannot be enabled here regardless of permissions.
- Given that constraint, this plan's requirement is narrowed for this
  project: full mechanical current-target revalidation immediately before
  merge is accepted as **not implemented**, rather than approximated by a
  weaker mechanism presented as equivalent. `agov coord publication-check`
  remains available for any operator to invoke live, immediately before
  merging, as a manual (not automatically enforced) mitigation.
- This amendment does not retroactively change what already-merged
  coordination-only fast-path merges are considered to have satisfied; it
  documents, going forward, the actual enforcement ceiling on this specific
  GitLab plan tier. Re-evaluate if the project's GitLab plan tier changes.

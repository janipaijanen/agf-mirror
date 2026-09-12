---
kind: review
from: independent-implementation-reviewer
to: coder-tester
id: 20260912T105522Z-independent-implementation-reviewer-6p4k
timestamp: 2026-09-12T10:55:22Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260912T102658Z-governor-reviewer-rqkp
  - 20260912T095349Z-governor-reviewer-smmp
  - 20260912T100746Z-independent-implementation-reviewer-mhhk
  - 20260912T103802Z-coder-tester-mmjh
topic: cpb1-pinned-candidate-final-review
requires-ack: false
artifacts_owned: []
---

Parent publishes the independent reviewer's full report verbatim. This is not a
second independent review. Code/security CLEAR is distinguished from pending CI
and governor Gate2; former findings are retained below with their resolutions.

# Independent implementation and semantic-security delta review — CP-B-1 / MR !2636

## Exact scope, authority, and provenance

- Merge request: **!2636**, `feat/cpb1-multi-record-coordination-publication` → `main`
- Exact reviewed head: **`6a21f9216a45628fb8be60a0450596a8d2d30f07`**
- Exact observed target and merge base: **`4fe0f389a36d9234c218d95f4182cdfcdc239a6d`**
- Immediate prior corrected head: `3a2645ab689396c00e691f4b86065a5bb40d1fb0`
- Original final-review head: `1da3584d3a5e89aad50b61e9e165743da87242ae`
- Semantic-security appointment: governor record `20260912T095349Z-governor-reviewer-smmp`, commit `3168a5f800a43bada83dce26b13ef956a6870f92`
- Amendment/review authority: governor record `20260912T102658Z-governor-reviewer-rqkp`, commit `bcb1e1f1ab66e6a1704b15ff65ad5ea9313323eb`
- Reviewer: Beauvoir, task `/root/implementation_review`, OpenAI **gpt-5.6-sol / high**, distinct from the OpenAI **gpt-6-astra** correction/parent session. I did not author or edit the reviewed plan, rule, projection, source, tests, CI, commits, or MR.

This review covers the complete CP-B-1 implementation plus the bounded `1da3584d..6a21f921` security/code correction. It reconciles the four earlier independent implementation findings, CPB1-SEC-1 and CPB1-SEC-2, the candidate-HEAD coherence defect found at `3a2645ab`, and the amended negative-test contract. CP-B-2/CP-B-3 behavior is excluded. This report is advisory and supplies no Gate 1, Gate 2, merge, release, tag, or deployment authority.

## Disposition

- **Independent code/plan coverage: COMPLETE.**
- **Semantic-security consultation coverage: COMPLETE.**
- **Code and semantic recommendation: CLEAR** at exact head `6a21f921...`; no unresolved implementation, rule, projection, or security finding remains in this bounded slice.
- **Overall evidence coverage: INCOMPLETE pending exact-head canonical CI.**
- **Overall recommendation: BLOCK pending CI**, then eligible for governor Gate 2 consideration if every required job succeeds and the head/target remain the reviewed identities.

Canonical pipeline **2843009501** exists for the exact head as a `merge_request_event` pipeline and was **pending** at the final evidence read. Pipeline 2842996629 belongs to superseded head `3a2645ab...` and cannot clear this report. MR !2636 is open, non-draft, reports `has_conflicts:false`, and its remote head matches `6a21f921...`.

## Findings and resolution

### CPB1-IIR-1 through CPB1-IIR-4 — resolved and unchanged

The earlier implementation review found mutable working-tree schema validation, a symlink/nonregular-object bypass, nested-path boundary leakage, and incomplete suppression-marker/visible gate evidence. These were resolved at `e667a402`/`1da3584d` and remain resolved at the current head:

- validation consumes the exact raw bytes from the introducing commit;
- only regular `100644`/`100755` direct-child lowercase `.md` coordination blobs qualify;
- nested candidate paths reject and nested base paths do not seed copy identity;
- both `[ci skip]` and `[skip ci]` are tested, and the full gate-test matrix is present.

No reviewed delta weakens those protections.

### CPB1-SEC-1 — target-tip copy omission resolved

The approved merge-base-only design at `1da3584d` allowed a target branch to acquire record A after divergence while a candidate added byte-identical B. Governor `rqkp` approved the amended contract. At the current head, the command resolves the selected target once to an immutable commit SHA, inventories direct regular coordination blobs from both merge base and that pinned target tip, and reports `target_sha`.

The divergent-target regression rejects an exact copy, accepts a distinct record, and proves later target-ref deletion does not change the invocation's object reads. Actual raw bytes remain decisive after hash bucketing, so skeleton/template similarity does not create a false positive.

### CPB1-SEC-2 — decisive Git failures resolved

Every Git read that establishes the six protected conditions now contributes a blocking reason on failure: target and candidate resolution, merge base, merge/commit enumeration, subject/full message, per-commit and aggregate diffs, tree enumeration, and blob reads. Empty/error evidence can no longer be interpreted as a clean range.

I independently reran both former false-clean fixtures against the exact-head release binary:

```text
/home/dev/s/.worktrees/cpb1-security-mergefail.JsyLIu
injected rev-list --merges failure over real merge/import history
exit 1; ok:false; candidate_sha=4729de49...;
reason names the failed rev-list --merges operation

/home/dev/s/.worktrees/cpb1-security-difffail.iOn2ne
injected diff-tree failures over implementation + revert + coordination history
exit 1; ok:false; candidate_sha=1940f66b...;
reasons name each failed per-commit diff-tree operation
```

Diagnostics expose command names, paths, and commit identities without including coordination-record bodies.

### CPB1-SEC-3 — candidate HEAD drift resolved at `6a21f921`

At `3a2645ab`, only the target was pinned. The implementation reused mutable `HEAD` for merge-base, range enumeration, and aggregate diff. A real fake-Git fixture moved HEAD to forbidden merge history immediately after `rev-list --reverse`; the command returned `ok:true` for the old clean enumeration while later reads addressed the changed HEAD.

The final head resolves `HEAD^{commit}` exactly once before ancestry/enumeration and uses the resulting SHA in merge-base, range, merge/commit enumeration, per-commit object reads, and aggregate diff. Text and JSON expose both `candidate_sha` and `target_sha`, binding a successful result to the checked snapshots. The canonical rule, Claude projection, approved clarification, and projection manifest state the same contract.

Independent exact-head reproduction in `/home/dev/s/.worktrees/cpb1-head-drift.4mC7HT`:

```text
pinned clean candidate: 106940417a1b990e45b7e5de77fa4bfc1d8159af
HEAD after injected drift: 1e9e5f7dc9d5d728d1ad999ad31d4b3ca3a66f1c (forbidden merge)
exit 0; ok:true
candidate_sha: 106940417a1b990e45b7e5de77fa4bfc1d8159af
commits_checked: [106940417a1b990e45b7e5de77fa4bfc1d8159af]
```

Passing here is correct: the answer is explicitly bound to the clean pinned object even though the mutable ref later moved. The committed real-CLI regression additionally records every Git argv and asserts no argument after candidate resolution contains `HEAD`. The rule still requires the caller to revalidate after target advance; invocation coherence is not presented as timeless freshness.

### CPB1-ACCEPT-1 — failure tests now contain the forbidden histories they protect

At `3a2645ab`, the decisive-read matrix injected failures only over a clean candidate, while normal forbidden merge and implementation/revert tests were separate. That did not satisfy the amended plan's explicit demand to place real forbidden history behind injected failures.

The final CLI matrix now uses:

- a real merge/import range when `rev-list --merges` is injected to fail; and
- a real implementation + revert + coordination range when per-commit or aggregate diff is injected to fail.

Each case asserts exit 1, `ok:false`, and an operation-specific diagnostic. Candidate-resolution failure is also distinct from target-resolution failure. Existing normal-path merge/revert negative controls remain present.

## Contract and trust-boundary assessment

The final implementation reads immutable Git objects for both inventories and candidate additions, validates the same introducing-commit bytes used for copy detection, rejects nonregular and out-of-channel entries, and treats hashes only as an index before byte-for-byte equality. It retains original path provenance, handles same-commit and cross-commit duplicates, and does not disclose record bodies. Candidate and target ref resolution use `rev-parse --verify --end-of-options ...^{commit}` before all decisive range/object reads.

The report's new SHA fields are additive and appear in both text and JSON. The MCP adapter has no alternate publication-check implementation; it invokes the same CLI command and returns the CLI stdout/status envelope, so these fields and failure statuses follow the CLI path. No MCP adapter code changed in this correction. I found no new authority, publication, merge, release, or secret-handling capability.

The canonical and Claude rule copies differ only by the canonical file's expected `provenance: verified-agf-v2` front matter. Their governed body text is otherwise identical. The strict projection manifest covers the candidate/target pinning and fail-closed matrix.

## Exact-head evidence

Independent inspection and narrow reproductions used detached review worktree `/home/dev/s/.worktrees/cpb1-review-final`:

```text
git rev-parse HEAD
# 6a21f9216a45628fb8be60a0450596a8d2d30f07

git rev-parse origin/main
# 4fe0f389a36d9234c218d95f4182cdfcdc239a6d

git merge-base origin/main HEAD
# 4fe0f389a36d9234c218d95f4182cdfcdc239a6d

git diff --check 4fe0f389...6a21f921
# exit 0

agov plan validate docs/plans/20260909T200622Z-cpb1-copied-record-contract-clarification.md
# OK, blocking severity

agov plan projection-check --base-ref 4fe0f389... \
  docs/reviews/rule-projection/20260909T170655Z-cpb1-multi-record-coordination-publication.yaml
# OK, blocking severity
```

The correction agent's exact-head logs show 23 publication-check unit tests, 11 real CLI tests, 17 gate tests, the focused post-resolution argv assertion, clippy, and release `agov`/`agov-mcp` build passing. I inspected those logs and the exact source/test delta. I independently ran the three adversarial fixtures above using `/home/dev/s/.worktrees/cpb1-implementation/tooling/rust/target/release/agov`; its output includes the final-head `candidate_sha`/`target_sha` fields.

I did not duplicate the full workspace suite, packaging suite, or Windows jobs. Canonical CI is responsible for full required coverage. Optional/manual/allow-failure jobs must be reported separately and cannot replace a required job.

## Required final evidence

Before any overall CLEAR or Gate 2 handoff, verify pipeline 2843009501 remains bound to `6a21f921...`, reaches success, and has every required job successful. Confirm the MR remains conflict-free against the current target. A changed head or material target conflict invalidates this recommendation and requires a bounded refresh. Binding merge authority remains with the governor-reviewer.

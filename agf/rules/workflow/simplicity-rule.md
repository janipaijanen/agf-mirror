---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Simplicity: The Primary Design Goal
#
# Framework principle: Proportional governance overhead
# See: framework.md — Pattern 6 (Fast Track), Principle 7 (Technology Neutrality)

Complexity is the primary source of bugs, maintenance cost, and unreadable code.
Every addition of complexity requires justification. Simplicity does not.

## Minimum sufficient value

Plan first, then say, build, and deliver the smallest complete solution
that meets the approved requirements and quality standard. Remove
unnecessary wording, scope, states, abstractions, options, process, and
delay. Do not remove meaning, correctness, safety, evidence,
maintainability, uncertainty, or required user value.

**Completeness across affected value surfaces:** The smallest solution
is complete only when every materially affected value surface has an
explicit disposition. Relevant surfaces include product behavior and
necessary development, end-user and adopter documentation, truthful
public or marketing explanation and discoverability, `agov` and other
operator-tooling quality, tests and maintainability, and release or
operational readiness. A surface may be omitted only when it is not
affected or when an authorized plan explicitly defers it. Simplicity
must not shift avoidable complexity, risk, or missing information to
users, adopters, operators, or later work. Not every change must touch
every surface — only the ones a given change actually affects.

1. **Communication:** Use the shortest clear wording that preserves
   necessary meaning. Remove jargon, repetition, and decorative detail.
   Do not omit material facts, constraints, uncertainty, or required
   actions.
2. **Implementation:** Implement the simplest solution that fully meets
   the approved requirements. Remove unnecessary states, abstractions,
   options, and special cases. Do not simplify away correctness,
   safety, evidence, or required behavior. (See "The over-engineering
   warning signs" below for concrete signals.)
3. **Delivery:** Deliver the smallest complete solution that meets the
   approved quality requirements. Remove unnecessary scope, process,
   abstraction, and delay. Do not remove correctness, safety, evidence,
   maintainability, or required user value.
4. **Planning before work:** Before starting new work, confirm that
   planning proportional to its scope and risk has been recorded and
   any required approval obtained. If no suitable plan exists, record
   one first. A full plan states the outcome, scope and exclusions,
   acceptance checks, authority and gates, owner, material risks or
   dependencies, and simplest sufficient approach. Eligible fast-track
   work may use a compact pre-work note stating the outcome, bounded
   scope, acceptance check, and why the lighter path is sufficient;
   repeat the waiver in the later commit message (see "Calibrating
   workflow overhead to task size" below). Planning must be adequate,
   not ceremonially large.

## The Simplicity Test

Before proposing any design, implementation, or abstraction, ask:
> "Is there a simpler way to achieve the same outcome?"

If yes: propose the simpler way. Explain why it is sufficient.
If no: explain why the complexity is necessary (not just useful).

## In code

**Prefer:**
- A function over a class, when the function is sufficient
- A direct DB query over a service layer, when there is no reuse or business logic
- A plain dict or dataclass over a class hierarchy
- An existing utility over a new one
- Solving the actual problem over solving a generalised version of it

**Refuse (unless explicitly justified):**
- Abstractions with only one implementation
- Base classes with only one subclass
- Configuration options that no one will ever change
- Caching before a performance problem is measured and confirmed
- Generic systems when a specific solution would take 10% of the effort
- "Future-proofing" for requirements that do not exist

## In architecture

Work within the existing architecture. Do not propose:
- Frontend frameworks unless the user asks
- Microservices or service decomposition for a monolith
- Message queues or event-driven architecture for problems solvable with a function call
- New infrastructure components without a concrete, measured need
- API gateways or versioning for an internal tool

The existing architecture is intentional.
Propose departures only when the existing architecture provably cannot solve the problem.

## In tests

A test that is hard to understand is a test that will be wrong.
- Keep fixtures minimal (only the data the test actually needs)
- Keep assertions specific (not "response contains something")
- Prefer one assert per test over one test per feature
- Delete a test before keeping a broken or misleading one

## The over-engineering warning signs

Stop and reconsider if you find yourself:
- Adding a parameter "in case we need it later"
- Creating a helper function used in exactly one place
- Writing more test infrastructure than test logic
- Introducing a pattern the rest of the codebase does not use
- Reaching for a design pattern by name before understanding the problem

## Calibrating workflow overhead to task size

Not every task needs the full multi-phase workflow from development-workflow-rule.

| Task type | Workflow required |
|-----------|------------------|
| Typo, comment, ordinary doc update | Branch, edit, test, merge request |
| Single-file bug fix with obvious cause | Fix -> test -> commit |
| New field on existing model | Plan note + migration + test + commit |
| New route or service | Full plan file + TDD + review checklist |
| Cross-cutting change (>3 modules) | Full workflow + worktree |
| Refactor of existing code | Characterisation tests first, then refactor |

When in doubt: choose the lighter process that still satisfies the applicable
branch and review rules. Upgrade if the task grows.

**Fast-track waiver:** When using the lighter process (skipping the plan file), record the waiver in the commit message: what task, why low-risk. If the task turns out larger than expected, stop and write a plan -- do not continue without one. Not eligible: PII, security boundaries, architecture, data model changes.

**Fast-track commits must not include a version bump, CHANGELOG entry, or git tag.** These are work commits -- they accumulate on the main branch until the next release. Tagging a fast-track commit causes version creep.

## Sources/radar research-maintenance fast path

A reversible research-maintenance change may skip a separate Gate 1 plan,
contribution audit, and pre-change coordination MR only when **all** of these
conditions hold:

1. Every commit from the freshly resolved target merge base through `HEAD`,
   and the final aggregate diff, changes only the existing files
   `docs/sources.md` and/or `docs/radar.md`. Merge commits are ineligible.
   A temporarily added, reverted, imported, or cherry-picked change to any
   other path is ineligible even when absent from the final diff.
2. The change only adds or corrects a source classification, preserves an
   unverified stub, adds or refreshes a non-binding radar question, updates a
   reasoned `review-by` date, or follows an existing promote/keep/retire radar
   lifecycle.
3. The text is advisory and reversible: it does not adopt a tool or practice,
   promise implementation, assign work, select priority, or grant an external
   source, tool, status, or verdict AGF authority.
4. Material claims are traceable to named URLs or repository evidence. The MR
   identifies pinned revisions and access dates where relevant, distinguishes
   primary, secondary, self-reported, inferred, and unverified evidence, and
   states material limits.
5. A radar entry keeps its controversy or open question, non-binding status,
   source cross-reference where applicable, and `review-by` date.
6. Before writing, the implementer freshly fetches the target, inventories all
   open MRs targeting it and their changed paths, and checks existing entries
   for duplicate identities, batch/allocation conflicts, and edit collisions.
7. The implementer repeats that target and all-open-MR scan immediately before
   Gate 2. The MR records UTC scan time, target SHA, each open MR's IID, head
   SHA, `updated_at`, changed paths, and a disposition for every possible
   overlap. Any relevant state change invalidates the evidence until refreshed.
8. No disqualifier below applies. Uncertainty means full Gate 1.

The fast path is semantic, not merely a filename allowlist. It is automatically
disqualified by any normative rule or authority change; gate, schema,
validator, CI, merge, or release-policy change; implementation commitment;
adoption, integration, dependency, plugin, account, hosted service, data flow,
or runtime trial; roadmap, milestone, release, staffing, assignment, budget, or
priority selection; security, credential, privacy, personal-data, legal,
compliance, or regulatory material; public AGF product, marketing, competitive,
performance, maturity, effectiveness, or compliance claim; governed-decision
rewrite; any other changed path; or ambiguity about fidelity, reversibility,
scope, or ownership.

If the work grows into a disqualifier, stop before committing the expanded
scope and return to the ordinary Gate 1 route. Do not treat work already done as
retroactive approval.

### Required branch, evidence, and review

The fast path still requires a normal branch and MR to the protected target,
the commit-message waiver, source-fidelity evidence, targeted validation, a
green exact-head pipeline, and a binding governor-reviewer Gate 2 decision.
The MR is the evidence carrier and must state the exact paths and head SHA;
sources, revisions, access dates, evidence classes, and limits; the radar
disposition; the eligibility checklist and disqualifier result; the fresh
target/open-MR collision scan; and validation results.

After fetching the target, qualify the complete range fail-closed. Record the
target ref/SHA, merge-base SHA, exact range, every commit SHA, every path for
each commit, every final-tree path, and the procedure exit code. The procedure
must:

1. resolve the target commit and merge base or fail;
2. reject an empty range and every merge commit;
3. run `git diff-tree --no-commit-id --name-only -r` separately for each commit
   and reject every path except `docs/sources.md` and `docs/radar.md`;
4. run `git diff --name-only <merge-base>...HEAD` and reject every other final
   path; and
5. run `git diff --check <merge-base>...HEAD` and print `result=PASS` only when
   all earlier checks succeeded.

Use this procedure after fetching the target:

```bash
#!/usr/bin/env bash
set -euo pipefail

research_fail() {
  printf 'FAIL: %s\n' "$1" >&2
  exit 1
}

git fetch --prune origin || research_fail 'target fetch failed'
research_target_ref="${1:-origin/main}"
research_target_sha="$(git rev-parse --verify \
  "${research_target_ref}^{commit}")" || research_fail 'target ref unresolved'
research_base="$(git merge-base "$research_target_sha" HEAD)" || \
  research_fail 'merge base unresolved'
[[ -n "$research_base" ]] || research_fail 'merge base empty'

research_commit_list="$(git rev-list --reverse "$research_base"..HEAD)" || \
  research_fail 'commit range unresolved'
[[ -n "$research_commit_list" ]] || research_fail 'commit range empty'
research_merge_list="$(git rev-list --merges "$research_base"..HEAD)" || \
  research_fail 'merge-commit scan failed'
[[ -z "$research_merge_list" ]] || research_fail \
  "merge commit in range: $research_merge_list"

research_tmp="$(mktemp -d)" || research_fail 'temporary directory failed'
trap 'rm -rf -- "$research_tmp"' EXIT

printf 'target_ref=%s\n' "$research_target_ref"
printf 'target_sha=%s\n' "$research_target_sha"
printf 'merge_base_sha=%s\n' "$research_base"
printf 'commit_range=%s..HEAD\n' "$research_base"

research_bad=0
research_index=0
while IFS= read -r research_commit; do
  [[ -n "$research_commit" ]] || research_fail 'empty commit id in range'
  research_index=$((research_index + 1))
  printf 'commit[%d]=%s\n' "$research_index" "$research_commit"
  research_path_file="$research_tmp/commit-$research_index.paths"
  git diff-tree --no-commit-id --name-only -r -z \
    "${research_commit}^" "$research_commit" >"$research_path_file" || \
    research_fail "path scan failed for $research_commit"
  research_path_index=0
  while IFS= read -r -d '' research_path; do
    research_path_index=$((research_path_index + 1))
    printf 'commit_path[%d,%d]=%s\n' \
      "$research_index" "$research_path_index" "$research_path"
    case "$research_path" in
      docs/sources.md|docs/radar.md) ;;
      *) printf 'disallowed commit path: %s %s\n' \
           "$research_commit" "$research_path" >&2
         research_bad=1 ;;
    esac
  done <"$research_path_file"
  printf 'commit_path_count[%d]=%d\n' \
    "$research_index" "$research_path_index"
done <<<"$research_commit_list"
(( research_bad == 0 )) || research_fail 'disallowed per-commit path found'

research_final_file="$research_tmp/final.paths"
git diff --name-only -z "$research_base"...HEAD >"$research_final_file" || \
  research_fail 'final-tree path scan failed'
research_bad=0
research_final_index=0
while IFS= read -r -d '' research_path; do
  research_final_index=$((research_final_index + 1))
  printf 'final_path[%d]=%s\n' "$research_final_index" "$research_path"
  case "$research_path" in
    docs/sources.md|docs/radar.md) ;;
    *) printf 'disallowed final-tree path: %s\n' "$research_path" >&2
       research_bad=1 ;;
  esac
done <"$research_final_file"
printf 'final_path_count=%d\n' "$research_final_index"
(( research_bad == 0 )) || research_fail 'disallowed final-tree path found'

git diff --check "$research_base"...HEAD || \
  research_fail 'whitespace validation failed'
printf 'result=PASS\n'
exit 0
```

Do not rely on a final-tree diff alone: it cannot reveal an extra path added and
later reverted. Run the procedure under `set -euo pipefail` with explicit
non-zero failure exits so a later successful command cannot hide a finding.

Immediately before Gate 2, compare the fresh target and candidate against every
open MR touching either allowed file. Recheck canonical source identities,
batch allocation, radar titles/cross-references, edit regions, registry
vocabulary, and `review-by` values. A duplicate, collision, unclear allocation,
or unresolved owner makes the candidate ineligible. A changed candidate head
requires new validation, pipeline, and exact-head review.

The governor-reviewer may escalate any candidate to full Gate 1 and alone
decides Gate 2 and merge. No separate coordination record is required merely
to start or submit an eligible MR; use coordination when the work creates a
separate downstream obligation, escalation, accepted risk, or decision the MR
and merge do not fully carry.

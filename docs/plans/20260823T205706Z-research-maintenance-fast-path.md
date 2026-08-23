---
id: 20260823T205706Z-research-maintenance-fast-path
status: pending-gate1
topic: proportional-research-maintenance-fast-path
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Independent audit
      20260823T210538Z-contribution-auditor-qmdj reviewed the first frozen head
      COMPLETE / BLOCK. This revision resolves CA-RMFP-1 with per-commit and
      final-tree allowlist evidence, resolves CA-RMFP-2 with a fresh-target and
      all-open-MR pre-Gate-2 collision scan, and corrects consultation metadata
      per CA-RMFP-A1. A bounded exact-head re-review is requested through the
      follow-up handoff; Gate 1 remains pending until it clears the corrections.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      The planned change is workflow prose only and adds no secret handling,
      credential path, authentication boundary, executable integration,
      network access, schema, validator, or CI authority. Security-sensitive
      research is expressly ineligible for the proposed fast path and remains
      under the full Gate 1 workflow.
---

# Plan: proportional fast path for source-registry and radar maintenance

## Decision requested

Authorize one narrow exception to the full Gate 1 workflow: an advisory,
reversible research-maintenance change whose complete diff is limited to
`docs/sources.md` and/or `docs/radar.md` may proceed directly to a normal
branch and merge request without a separate Gate 1 plan, contribution audit,
or pre-change coordination merge request.

The merge request itself becomes the review carrier. It must contain
source-fidelity evidence and targeted validation, and the governor-reviewer
still makes the binding exact-head Gate 2 merge decision. This plan changes no
other gate, role, merge authority, branch protection, or release authority.

This plan is itself a rule and gate-routing change, so it follows the full
Gate 1 process. Its proposed exception cannot be used to approve or implement
this plan.

## Why the current process is disproportionate

Current authority is internally inconsistent:

- `AGENTS.md` and `development-workflow-rule.md` require Gate 1 for every
  non-trivial change, which includes a two-file sources/radar update;
- `simplicity-rule.md` says a documentation update may be edited directly and
  records a fast-track waiver only in the commit message; and
- `merge-request-workflow-rule.md` correctly requires every protected-branch
  documentation change to arrive through an MR.

Recent repository evidence shows both the useful control and the excess:

- MR !1862 publishes the contribution-auditor's withdrawal of a mistaken
  Batch 29 planning obligation. The record identifies the existing
  `simplicity-rule.md` fast track as sufficient for a reversible, source-only
  classification and measures the discarded route as a 102-line coordination
  record, coordination MR and pipeline, followed by a still-unstarted
  plan/audit/approval sequence.
- MR !1863 is the corresponding live one-file fast-track candidate. Its MR
  carries pinned upstream revisions, access evidence, claim limits, an exact
  path-scope statement, and targeted validation; its exact-head pipeline
  passed without a separate Gate 1 plan. This is the direct precedent for the
  source-only half of the proposed rule.
- MR !1779 changed only `docs/sources.md` and `docs/radar.md`, carried explicit
  verification and evidence limits in the MR, passed exact-head CI, and merged
  without a separate plan or coordination chain. It is the direct combined-
  file precedent whose policy basis the current rules do not state clearly.
- MRs !1624 and !1626 likewise carried bounded source/radar classifications in
  the implementation MR itself.
- MR !1241 used a separate approved plan, fact-check handoffs, specialist
  review, coordination publication, and conditional Gate 2 sequencing for the
  same two research artifacts; that route remains appropriate when the content
  proposes an integration or makes higher-risk claims, but is excessive as the
  default for advisory watchlist maintenance.
- MR !1695 shows why review cannot disappear: a one-file source classification
  required three factual corrections after unsupported agent-topology claims
  were found. The proportional replacement is source-fidelity evidence plus a
  human exact-head Gate 2 decision, not an unreviewed direct commit.

Capability status is Level 1 Planned until this plan is approved and its rule
changes merge. The process judgment is empirically informed by these repository
cases; it is not evidence that every docs-only change is low-risk.

## Eligibility: all conditions must hold

A change qualifies only when every condition below is true:

1. Both the complete commit range from the freshly resolved target merge base
   through `HEAD` **and** the final aggregate tree diff change only the existing
   files `docs/sources.md` and/or `docs/radar.md`. Every individual commit is
   inspected. A merge commit, imported or cherry-picked commit touching any
   other path, or temporarily added/removed or modify-then-revert non-allowed
   path makes the candidate ineligible even when the final tree diff looks
   clean. No coordination, plan, TODO, issue, changelog, rule, product-doc,
   generated, or executable path occurs anywhere in the range.
2. The change is research maintenance: adding or correcting a source
   classification, preserving an unverified stub, adding or refreshing a
   non-binding radar question, updating a `review-by` date with a reason, or
   moving a radar item through its existing promote/keep/retire lifecycle.
3. The text remains advisory and reversible. It does not adopt a tool or
   practice, promise implementation, assign work, create a release or roadmap
   commitment, or grant any external source, tool, status, or verdict AGF
   authority.
4. Every factual claim is traceable to the named source or repository evidence.
   The MR identifies URLs and, where relevant, pinned revisions and access
   dates; separates primary, secondary, self-reported, inferred, and unverified
   evidence; attributes numerical or comparative claims; and states material
   limits rather than filling gaps by inference.
5. The radar form remains intact: a controversy or open question, non-binding
   status, source cross-reference where applicable, and a `review-by` date.
6. The implementer has checked freshly fetched `origin/main`, every open MR
   targeting it, and the existing source/radar text for duplicate entries,
   batch/allocation conflicts, and edit collisions before writing.
7. The same target and all-open-MR scan is repeated with fresh forge state in
   the Gate 2 review cycle. Earlier evidence is not sufficient. A relevant
   target or MR change must be resolved and evidenced before review continues.
8. None of the automatic disqualifiers below applies. Uncertainty means the
   change does not qualify.

Eligibility is semantic as well as path-based. A two-file diff is not safe
merely because Git reports the allowed filenames.

## Automatic disqualifiers: return to full Gate 1

Any one of these requires the ordinary plan, advisory-consultation decision,
Gate 1 approval, and later Gate 2:

- a normative rule, required behavior, role, authority boundary, gate,
  validator authority, schema, CI policy, or merge/release process;
- an implementation commitment, product adoption, external integration,
  dependency, plugin, account, hosted service, data flow, or runtime trial;
- roadmap, milestone, release, staffing, assignment, budget, or priority
  selection, including promotion from a radar question into committed work;
- sensitive, credential, privacy, personal-data, security, legal, compliance,
  or regulatory material;
- a public product, marketing, competitive, performance, effectiveness,
  maturity, or compliance claim about AGF;
- removal or rewriting of evidence in a way that changes an existing governed
  decision rather than correcting the research registry itself;
- any file beyond `docs/sources.md` and `docs/radar.md`, including a change
  needed to keep another artifact consistent; or
- ambiguity about source fidelity, reversibility, scope, ownership, or whether
  a proposed disposition is only advisory.

If a qualifying task grows into any disqualifier, stop before committing the
expanded scope. Preserve useful research as draft evidence, write a normal
Gate 1 plan, and do not treat work already performed as retroactive approval.

## Fast-path workflow and evidence contract

1. Fetch current remote state; enumerate every open MR targeting the intended
   target branch and inspect each MR's changed paths; check the relevant MRs and
   existing entries for overlap; create a normal `docs/` branch from the
   freshly fetched target.
2. Edit only the eligible research artifact(s). No pre-change coordination MR,
   plan file, or contribution audit is required.
3. Record the eligibility decision in the commit message, for example:
   `Fast-track: sources/radar maintenance; allowed paths only, advisory,
   reversible, no disqualifier triggered.`
4. Open a normal MR. The MR description is the Gate 2 evidence carrier and must
   state:
   - exact changed paths and exact head SHA;
   - source URLs, pinned revisions/access dates where relevant, and what was
     actually read;
   - evidence classification and limits for every material claim;
   - whether a radar entry was added, changed, promoted, retained, or retired,
     and why;
   - the eligibility checklist and explicit confirmation that no disqualifier
     triggered; and
   - targeted validation and exact-head pipeline results.
5. Run at least:
   - the deterministic full-range and final-tree procedure below, recording
     target SHA, merge-base SHA, every inspected commit, and every path;
   - `git diff --check <fresh-merge-base>...HEAD`;
   - manual duplicate-entry, registry-vocabulary, radar `review-by`, source
     attribution, and unsupported-claim checks; and
   - the MR pipeline configured for those paths, including
     `agov:governance:validate`.
6. Immediately before Gate 2/exact-head review, the MR author or implementer
   fetches the target again and obtains a fresh, complete forge inventory of
   every open MR targeting it. For every open MR, record IID, head SHA,
   `updated_at`, and changed paths; for every MR touching `docs/sources.md` or
   `docs/radar.md`, recheck source URLs/canonical identities, batch allocation,
   radar titles and cross-references, edit regions, registry vocabulary, and
   `review-by` values against the freshly fetched target and candidate. Update
   the MR evidence with the UTC scan time, target SHA, relevant MR inventory,
   and a disposition for every possible overlap. The governor verifies this
   evidence in the same review cycle. If target or relevant open-MR state
   changes before merge, repeat the scan; stale evidence is never sufficient.
7. If relevant target content changed, rebase or otherwise refresh the
   candidate, rerun both allowlists and semantic checks, and obtain a new
   exact-head pipeline. Any candidate-head change invalidates the earlier
   exact-head pipeline and Gate 2 review. A duplicate, collision, unclear
   allocation, or unresolved ownership question makes the candidate ineligible
   and returns it to the ordinary governed route.
8. The governor-reviewer reviews the exact MR head, may request corrections or
   additional factual advice, and alone decides Gate 2 and merge. The governor
   may escalate any candidate to full Gate 1 without treating path eligibility
   as a right to merge.

### Deterministic full-range and final-tree allowlist

The operator runs the following Bash procedure after fetching the target. It
fails closed when the merge base cannot resolve, the range is empty, any merge
commit exists, any individual commit touches a non-allowed path, or the final
aggregate diff contains a non-allowed path. Inspecting commits separately is
what catches a path added and later removed or reverted.

```bash
git fetch --prune origin
research_target=origin/main
research_base="$(git merge-base "$research_target" HEAD)"
test -n "$research_base"
test -n "$(git rev-list "$research_base"..HEAD)"
test -z "$(git rev-list --merges "$research_base"..HEAD)"

research_bad=0
while IFS= read -r research_commit; do
  while IFS= read -r -d '' research_path; do
    case "$research_path" in
      docs/sources.md|docs/radar.md) ;;
      *) printf 'disallowed commit path: %s %s\n' \
           "$research_commit" "$research_path" >&2
         research_bad=1 ;;
    esac
  done < <(git diff-tree --no-commit-id --name-only -r -z \
             "${research_commit}^" "$research_commit")
done < <(git rev-list --reverse "$research_base"..HEAD)
test "$research_bad" -eq 0

research_bad=0
while IFS= read -r -d '' research_path; do
  case "$research_path" in
    docs/sources.md|docs/radar.md) ;;
    *) printf 'disallowed final-tree path: %s\n' "$research_path" >&2
       research_bad=1 ;;
  esac
done < <(git diff --name-only -z "$research_base"...HEAD)
test "$research_bad" -eq 0
git diff --check "$research_base"...HEAD
```

The implementation MR must record deterministic acceptance transcripts for:

- one allowed source-only range, one radar-only range, and one combined range
  passing both checks;
- a non-allowed path present in the final diff failing;
- a non-allowed path added and reverted in a later commit failing even though
  the final aggregate diff contains only allowed paths; and
- a merge commit in the range failing.

These are procedure acceptance tests, not new validator authority. Semantic
eligibility, source fidelity, and collision dispositions remain human Gate 2
judgment.

No separate coordination record is required merely to start or submit an
eligible research-maintenance MR. A coordination record remains required when
the work creates a separate downstream obligation, escalation, accepted risk,
or decision that the MR and its merge do not fully carry. Optional factual
advice may be requested in the MR without turning contribution audit into a
default gate.

## Planned implementation

The smallest coherent implementation changes the canonical rules, their
active loader projections, the two loader summaries, and the minimum adopter
explanation needed to prevent contradictory instructions:

| File | Planned change |
|---|---|
| `agf/rules/workflow/simplicity-rule.md` | Own the exact eligibility, disqualifiers, MR evidence, escalation, and Gate 2 boundary; replace “edit directly, commit” wording with branch/MR wording consistent with protected-branch policy. |
| `agf/rules/workflow/development-workflow-rule.md` | Add the explicit sources/radar exception to its “every non-trivial task” and plan-file requirements; point to the simplicity rule rather than duplicate the full test. |
| `.claude/rules/workflow/simplicity-rule.md` | Mirror the canonical rule for foundry's active Claude loader with a canonical-owner pointer. |
| `.claude/rules/workflow/development-workflow-rule.md` | Mirror the canonical exception and pointer without becoming a second owner. |
| `AGENTS.md` | Add the foundry Gate-discipline and coordination-record exception, while preserving Gate 2 and all full-Gate-1 exclusions. |
| `agf/templates/AGENTS.md` | Give adopters the same loader-visible route when they retain these exact research artifacts. |
| `agf/docs/site/docs/operate/day-to-day.md` | Explain the proportional research-maintenance path and evidence retained at Gate 2. |
| `agf/docs/site/docs/operate/run-agf-in-real-situations.md` | Route the exact sources/radar situation to this path and keep generic “docs-only” work subject to sizing. |
| `docs/reviews/rule-projection/20260823T205706Z-research-maintenance-fast-path.yaml` | Record the fully resolving six-surface projection evidence before Gate 2. |
| `agf/CHANGELOG.md` and `CHANGELOG.md` | Record the behavioral relaxation, migration choice, and adopter-docs parity under `Unreleased`; no version bump or tag. |

`merge-request-workflow-rule.md` stays unchanged: the fast path still requires
a normal MR to the protected branch. `agent-improvement-proposal-rule.md` stays
unchanged: real rule, schema, architecture, or gate changes still require a
Gate 1 plan; this exception applies only to the two research artifacts and
cannot authorize its own expansion. No schema, Rust tooling, CI, branch
protection, role manifest, source registry, or radar content changes are in
the implementation scope.

## Projection screen

| Surface | Disposition before Gate 2 |
|---|---|
| Normative owner | **Updated:** `agf/rules/workflow/simplicity-rule.md`; `development-workflow-rule.md` contains only the applicability pointer needed to remove contradiction. |
| Adopter-facing explanation | **Updated:** `agf/docs/site/docs/operate/day-to-day.md` explains eligibility and retained Gate 2 evidence without copying the full rule. |
| Runbook/situation map | **Updated:** `agf/docs/site/docs/operate/run-agf-in-real-situations.md` gains the exact sources/radar route; generic docs-only work remains sized normally. |
| AI landing/loaders | **Updated:** root and template `AGENTS.md` plus the two foundry-local `.claude/rules/` projections. No `.claude/CLAUDE.md` exists. |
| Validation/drift evidence | **Deterministic procedure plus human review:** the plan's per-commit range check and separate final-tree check fail closed on hidden/reverted non-allowed paths and merge commits; freshly fetched target plus all-open-MR inventory is repeated in the Gate 2 review cycle. Semantic eligibility, source fidelity, and collision dispositions remain human judgment. No new validator or blocking authority is justified for this narrow process reduction. |
| Distribution | **Confirmed before Gate 2:** the canonical rules, template, and Pages explanation must resolve through existing `agf/dist/release-manifest.yaml` globs; root `AGENTS.md`, `.claude/rules/`, plan, and projection record remain foundry-local evidence. |

Implementation must create and pass the structured projection manifest named
above with `agov plan projection-check --base-ref origin/main`. The plan file
does not pre-create implementation evidence before Gate 1.

## Downstream impact and migration

**Classification: BREAKING.** The change relaxes a previously required Gate 1
and contribution-review route for a defined class of work, changing what
agents are permitted to do even though it removes ceremony rather than adding
runtime behavior.

- **Foundry:** future qualifying edits to the two research artifacts may start
  on a branch and proceed to Gate 2 without separate planning/coordination
  carriers. Existing history is not rewritten or declared defective.
- **Direct adopters:** adopters receive a new optional, exact-path fast path.
  A project without these two files has no eligible work. A project wanting a
  stricter policy may retain its existing Gate 1 requirement or add a local
  override; upgrade guidance must not imply that research review is being
  removed universally.
- **Downstream:** no adopter-produced artifact shape changes. The behavioral
  effect is limited to how qualifying internal research notes reach merge.
- **Migration:** before enabling the path, confirm the two files are genuinely
  non-normative research artifacts, keep protected-branch MR review and human
  Gate 2 merge enabled, and ensure the loader summary points to the updated
  simplicity rule. Otherwise retain the prior full Gate 1 route. No file or
  data migration is required.
- **Evidence:** repository precedents !1862, !1863, !1779, !1624, !1626,
  !1241, and !1695, plus the governor's explicit request to reduce this
  ceremony.

Adopter docs parity outcome required at Gate 2: **updated** through
`day-to-day.md`, `run-agf-in-real-situations.md`, the template loader, and the
`Unreleased` changelog entry.

## Acceptance criteria

1. All authoritative and loader-visible text agrees that only the exact
   eligible sources/radar class skips Gate 1, contribution audit, and a
   pre-change coordination MR.
2. The branch/MR, source-fidelity evidence, targeted validation, exact-head
   pipeline, human Gate 2 decision, and protected-branch merge remain
   mandatory.
3. A source-only, radar-only, and combined sources+radar advisory maintenance
   test range each passes the per-commit and final-tree procedure.
4. Negative examples for an extra file, rule/authority change, integration or
   dependency, implementation commitment, roadmap priority, sensitive/legal/
   security material, public product claim, and uncertain source inference
   each clearly return to full Gate 1.
5. A non-allowed path added and later reverted fails range qualification even
   when the aggregate final diff contains only allowed paths; a merge commit
   and a non-allowed final-tree path also fail deterministically.
6. Gate 2 evidence names the fresh target SHA, scan time, every open target-MR
   IID/head SHA/update time/path set, and the duplicate, batch/allocation,
   vocabulary, cross-reference, review-by, and edit-collision dispositions for
   each relevant MR. A synthetic or live changed-target/open-MR scenario shows
   the evidence is repeated and any changed candidate head requires a new
   pipeline and review.
7. Generic documentation changes do not inherit this exception.
8. The MR itself can carry the complete Gate 2 evidence without requiring a
   separate coordination-only MR; separate obligations still use coordination.
9. `agov plan projection-check`, `agov validate`, applicable documentation
   validation, `git diff --check`, and exact-head CI pass.
10. No source/radar content, validator, schema, CI configuration, role status,
   version, release, tag, or deployment changes are included.

## Risks and rollback

- **Path-only or net-diff gaming:** mitigated by the per-commit range check,
  merge-commit rejection, separate final-tree check, semantic criteria,
  enumerated disqualifiers, and governor exact-head review.
- **Target/open-MR state going stale:** mitigated by naming the MR author or
  implementer as refresh operator, recording target and all-open-MR state in
  the MR, repeating it in the Gate 2 review cycle, and invalidating old
  exact-head evidence whenever the candidate head changes.
- **Source overclaim:** mitigated by the MR evidence contract learned from
  !1695; optional factual advice remains available when the governor needs it.
- **Fast path becoming the default for all docs:** prevented by exact paths and
  explicit negative wording in the canonical rule, loader summaries, and
  scenario router.
- **Instruction drift:** mitigated by updating canonical rules, active
  projections, loaders, adopter docs, and the structured projection record in
  one implementation MR.

Rollback is one governed revert of the rule/loader/docs implementation. A
revert restores full Gate 1 for future research maintenance; already-merged
MRs remain valid historical decisions and are not retroactively replanned.

## Gate boundary

This pending plan authorizes no implementation. Contribution-auditor reviews
the exact plan head first; governor-reviewer then approves, requests revision,
or rejects Gate 1. Coder-tester work begins only after a binding, published
Gate 1 approval. Gate 2, merge, release, tag, and deployment remain separate
human decisions.

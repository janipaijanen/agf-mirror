---
id: v250-maintenance-release-line-rule
status: pending-gate1
topic: v250-maintenance-release-line-rule
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round-1 review dqsd proved that current-main publication cannot satisfy
      qpkg's exact-RC2 boundary. Planner-architect independently reproduced
      that conflict in d58p and will request a fresh exact-head review of this
      replacement plan after the named infosec and build consultations land.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Governor assessment 20260809T171844Z-governor-reviewer-sq5j.md confirms
      that infosec-sentinel's active charter is limited to information-leak
      screening and does not own branch/tag supply-chain integrity. The
      Governor instead assessed the named risks under security-hardening-rule,
      returned CLEAR for Gate 1, and supplied two strengthening requirements
      incorporated below: exact Maintainer-level protection plus command/API
      evidence that retained refs actually survive the merge.
  - role: build-engineer
    status: consulted
    reason: >-
      Result 20260809T170723Z-build-engineer-pste.md confirms that current MR
      and tag pipelines support the maintenance target without CI or release-
      script changes. It freezes dynamic-target, tag-job, cache, and merge-gate
      evidence and identifies one accepted non-safety caveat: first-push
      preview pipelines will run an over-inclusive heavy job set once because
      their comparison target is current main.
---

# Plan: protected maintenance release lines for immutable-candidate promotion

## Decision requested

Authorize a narrow, reusable amendment to AGF's release workflow for one case
the current rule cannot represent safely: promoting a previously published,
immutable release-candidate commit after the default branch has accumulated
unrelated later work.

This is the prerequisite ordered by Governor decision
`20260809T161825Z-governor-reviewer-2mmn.md`. It does **not** authorize the
`v2.5.0` release itself. After this amendment is implemented, reviewed,
Gate-2-approved, and merged, planner-architect will write a separate final-
release plan rooted at RC2 and only then close blocked MR !1325 as superseded.

## Problem and invariant

The standard workflow is correct for the normal case: create
`release-X.Y.Z` from current `main`, merge its metadata-only MR to `main`, and
tag the resulting main merge commit. Keep that path unchanged and preferred.

The maintenance case has a different invariant. A binding decision may require
promotion of an older immutable candidate *without* including work that landed
on main later. For the first intended use:

- annotated candidate tag object:
  `2bdcbb76716ba84ffa78e792b380b7befdd9340e`;
- peeled candidate commit:
  `3874a5b92b006fc867a4cedf359cfc17c6dfbae9`;
- candidate tree: `2bd9b7be772859410b202dcb1fe296a4acc07d4b`;
- Governor scope: `qpkg`, reaffirmed by `kprg` and topology-approved by
  `2mmn`.

At `d58p`'s audit, that commit was an ancestor of main with 401 later commits
across 223 changed paths. A standard metadata-only child of current main keeps
those later paths. A merge whose tree instead equals the old candidate would
have to revert or overwrite them. The amendment must permit an isolated,
protected maintenance target while forbidding tree surgery on current main.

## Eligibility contract

The maintenance-line path is optional and MUST NOT replace the standard path.
It is eligible only when every condition below is evidenced before a branch is
created:

1. A binding Governor record names the exact previously published candidate
   tag object, peeled commit, tree, intended stable version, and why current
   main is excluded.
2. The candidate commit is reachable from the canonical repository's main
   history and already has its own Gate 1, Gate 2, tag-pipeline, and
   post-publication evidence. A branch-only or unreviewed commit is ineligible.
3. The new release changes only version, changelog, and other enumerated
   release-truth metadata. Any functional, schema, migration, test,
   dependency, CI, or artifact-layout correction makes the candidate
   ineligible and requires a new RC/current-main release plan.
4. A separate Gate-1-approved release-publication plan freezes the exact base,
   target/source branch names, metadata allowlist, validation matrix,
   immutable namespaces, and forward-sync procedure.
5. The existing candidate tag remains immutable. The new version's Git tag,
   GitLab Release, generic-package, and OCI namespaces are independently
   absent through their authoritative APIs. A protected-tag rule matching the
   exact intended stable tag (or the governed `vX.Y.Z*` convention) must also
   exist before the tag is created, with creation restricted to GitLab access
   level 40 (Maintainer).

If any condition becomes false, stop. Do not fall back silently to current
main, a temporary main rewind, a custom tree-overwriting merge, or artifact
relabeling.

## Maintenance-line contract

### Protected target and source

1. Create a target named `release/X.Y` at the exact eligible candidate commit.
   Protect it before creating release content. The protection must reject
   force-push and deletion and restrict both push and merge to GitLab access
   level 40 (Maintainer), rather than inheriting main's current Developer-level
   merge permission. Re-read the exact platform rule as evidence.
2. Create `release-X.Y.Z` at exactly the same candidate commit. Confirm both
   refs resolve to the frozen SHA before the first release-metadata edit. Give
   the source branch equivalent level-40 push/merge protection and deletion/
   force-push denial before editing it; verify both exact branch rules through
   the platform API rather than assuming a wildcard matched.
3. Open a non-squash, non-rebase merge request from `release-X.Y.Z` to
   `release/X.Y`. The target branch, merge base, candidate head, and full diff
   are review inputs. Any change invalidates the prior review.
4. The MR diff is restricted to the release plan's exact metadata allowlist.
   Mixed-content files receive field/section-level assertions; a path allowlist
   alone is insufficient.
5. Exact-head Gate 2 remains binding Governor authority. Merge only the named
   SHA with the reviewed pipeline and retain both maintenance refs. Automatic
   source-branch removal must be explicitly disabled. Gate 2 evidence must
   show the exact merge invocation, including its branch-retention option, and
   immediate authoritative API reads proving both refs still exist at the
   intended SHAs; decision text alone is insufficient evidence of retention.

### Tag and publication

After the maintenance-target merge, prove its tree differs from the frozen
candidate only by the approved metadata fields. Recheck all four immutable
namespaces and target/source ref identities. Then create one annotated tag on
the exact maintenance-target merge commit and push it once. Never move,
delete, or recreate it.

The tag pipeline must bind every archive, checksum, provenance record, package,
OCI digest, and GitLab Release to that tag commit. Anonymous post-publication
checks operate on downloaded bytes. A failure after tag creation follows
`immutable-tags-rule.md`: diagnose forward and use a new version when content
must change; never rewrite the tag or either reviewed branch.

### Metadata-forward synchronization

Only after successful publication:

1. Freeze the latest current-main base and create a fresh sync branch from it.
2. Merge the tagged maintenance commit with `--no-ff`, resolving conflicts so
   every nonmetadata path remains byte-for-byte identical to the frozen main
   base. Preserve main's `[Unreleased]` material; insert the stable entry below
   it; update only current release-truth fields.
3. Open a separate MR to main. The source tip must retain the tagged commit as
   an ancestor; squash and rebase are forbidden because they erase that
   ancestry.
4. Prove the net diff from the frozen main base contains only the exact
   forward-metadata allowlist. Any target advance invalidates that proof and
   requires a refreshed merge, pipeline, and exact-head review.
5. Merge only after its own binding Gate 2. If this sync fails, the published
   release remains valid; correct the sync forward without modifying the tag
   or maintenance line.

This makes the immutable release commit reachable from current main without
replacing main's newer functional tree.

## Impact classification

Classification: **ADDITIVE**. The standard current-main release workflow keeps
all existing requirements. The amendment adds an optional, more constrained
path for a previously impossible maintenance case.

Impact:

- **Foundry:** no break. The first intended use is separately planned
  `v2.5.0` promotion under `2mmn`; no branch or release state changes here.
- **Direct adopters:** ADDITIVE. Projects may use the maintenance path only
  with the new eligibility and evidence contract; existing releases continue
  using the standard path.
- **Downstream:** no adopter-produced artifact shape changes. Release source
  ancestry may use a protected maintenance target, but artifact/provenance
  identity remains exact and explicit.
- **Migration:** none. No existing workflow is removed or weakened.
- **Evidence:** `d58p`, binding decision `2mmn`, and round-1 review `dqsd`.

Gate 2 handoff must say exactly: `Adopter docs parity: updated`.

## Exact implementation boundary

The implementation may change only:

| Path | Planned change |
|---|---|
| `agf/rules/workflow/release-via-merge-request-rule.md` | Keep the standard path; add eligibility, maintenance-target, tag, failure, and forward-sync contract. |
| `.claude/rules/workflow/release-via-merge-request-rule.md` | Update foundry's active local projection with the same contract and canonical-owner pointer. |
| `agf/rules/workflow/branch-and-worktree-rule.md` | Clarify that release metadata normally uses current main, with only the named maintenance exception. |
| `.claude/rules/workflow/branch-and-worktree-rule.md` | Keep foundry's active local projection aligned. |
| `agf/docs/site/docs/operate/review-and-release.md` | Explain when standard versus maintenance release topology applies and link to the owner rule. |
| `agf/skills/release/SKILL.md` | Add a fail-closed topology-selection step and a maintenance-line runbook that never silently selects the exception. |
| `docs/reviews/rule-projection/20260809T163051Z-v250-maintenance-release-line-rule.yaml` | Record all six projection dispositions. |
| `CHANGELOG.md` | Add an Unreleased foundry entry for the governed workflow addition. |
| `agf/CHANGELOG.md` | Add the adopter-facing ADDITIVE rule/runbook entry and exact parity outcome. |

No schema, Rust source, validator, `.gitlab-ci.yml`, release script, version,
branch, protection setting, tag, package, OCI, or GitLab Release change is in
scope. If build-engineer finds that existing CI cannot safely validate or
publish a maintenance target, revise this plan and obtain a fresh review before
adding any CI/script path.

Build consultation `pste` confirms no CI/script expansion is required. The
first push of each older-candidate maintenance branch is expected to trigger an
over-inclusive heavy preview pipeline because the shared new-branch rule
compares to current main. Accept that one-time runner cost; it is extra
validation, not missing validation, and does not justify weakening the ordinary
branch rule. All later acceptance evidence remains bound to the actual MR or tag
pipeline source and exact commit.

The `agents-govern/` bootstrap reference remains untouched. Historical plans,
coordination records, tags, and changelog entries are immutable evidence, not
rewrite targets.

## Documentation-rule projection screen

Implementation must create the structured projection record named above with
these six dispositions:

| Surface | Disposition |
|---|---|
| Normative owner | **updated** — `agf/rules/workflow/release-via-merge-request-rule.md`; companion branch wording updated only to remove contradiction. |
| Adopter-facing explanation | **updated** — `agf/docs/site/docs/operate/review-and-release.md`; explain both paths without duplicating the full rule. |
| Runbook/situation map | **updated** — `agf/skills/release/SKILL.md`; topology selection stops unless authority and eligibility are explicit. |
| AI landing/loaders | **updated** for foundry's `.claude/rules/...` projections; **existing pointer sufficient** for `AGENTS.md` and `agf/templates/AGENTS.md`, which already direct agents to the canonical workflow rules. No `.claude/CLAUDE.md` exists. |
| Validation/drift evidence | **not mechanically decidable** for semantic eligibility and Governor authority. Exact Git/API commands, field-level diff checks, negative scenario evidence, and human Gate 2 remain required; current validators retain only their named authority. |
| Distribution | **confirmed** — canonical rules, Pages runbook, release skill, and AGF changelog already ship through `agf/dist/release-manifest.yaml`; `.claude/rules` and root changelog are foundry-local. |

## Micro-task sequence

1. **Integrate consultations.** Completed by `pste` and `sq5j`: no affected
   path expansion; exact access-level, protected-tag, retained-ref, and
   first-push evidence is frozen here. No implementation before this plan is
   approved.
2. **Update canonical rule pair.** Modify the release rule first, then remove
   the contradictory “only on main” wording in branch discipline. Done when
   standard and maintenance paths are mutually exclusive and every stop
   condition above is explicit.
3. **Update foundry projections.** Align the two `.claude/rules` copies without
   making them a competing normative owner. Done when they name the canonical
   `agf/rules` files and contain no contradictory permission.
4. **Update adopter explanation and runbook.** Add concise standard-versus-
   maintenance guidance to Pages and the release skill. Done when neither
   surface permits maintenance mode without a binding decision and approved
   release plan.
5. **Write structured projection and changelog evidence.** Complete all six
   rows, both changelogs, ADDITIVE impact, and parity outcome.
6. **Validate and self-review.** Run the full matrix below, record negative
   scenario output, update the MR description, and hand the exact head to
   contribution-auditor before Gate 2.

## Validation and negative evidence

The implementation MR must provide exact commands and results for:

```bash
agov validate
agov validate adopter-docs
agov plan validate docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md
agov plan projection-check --base-ref origin/main \
  docs/reviews/rule-projection/20260809T163051Z-v250-maintenance-release-line-rule.yaml
agov dist validate
agov dist build
tooling/rust/scripts/check-release-self-sufficiency.sh check \
  dist/agf-*.tar.gz dist/agf-*.zip
git diff --check
```

Also run strict MkDocs and link validation through the repository's current
documented commands/tests, and inspect both built archive formats to confirm
the canonical rule, Pages explanation, release skill, and changelog are
present while foundry-local `.claude/rules` is absent.

For the first intended maintenance release, record authoritative API output
showing: target-branch push and merge access level 40; a protected-tag rule
covering the exact stable tag with Maintainer-only creation; absence of all four
immutable namespaces; the actual non-squash/non-rebase merge invocation with
source removal disabled; and immediate post-merge existence/SHA checks for
both retained maintenance refs. Also record that the expected first-push heavy
preview pipeline is over-inclusive and green, not mistaken for the later exact-
head acceptance pipeline.

Because this adds normative process gates but no new mechanical validator,
Gate-self-test evidence is a review transcript with three deliberately bad
scenarios:

1. no binding Governor topology decision;
2. target not protected or not equal to the frozen candidate SHA;
3. proposed release diff contains one nonmetadata functional file.

For each scenario, the MR must quote the exact rule clause and show the
expected result `STOP — standard or new-candidate plan required`. It must also
show one positive scenario satisfying every eligibility field. Tooling output
must not be described as the source of Gate authority.

## Acceptance criteria

1. The standard main-based release workflow remains the default and is not
   weakened.
2. The maintenance path cannot start without an immutable candidate, binding
   Governor topology decision, approved release plan, protected exact-SHA
   target/source refs, Maintainer-only target and tag protection, and four-
   namespace preflight.
3. The maintenance MR is metadata-only at both path and mixed-file field
   levels, non-squash/non-rebase, exact-head-reviewed, and retains both refs.
4. Tagging, provenance, publication, and failure handling bind to the reviewed
   maintenance-target merge commit without changing tag immutability.
5. Metadata-forward synchronization makes the tagged commit reachable from
   current main while preserving every nonmetadata path and current
   `[Unreleased]` content.
6. Canonical rules, foundry projections, Pages explanation, and release skill
   agree; the six-row projection record validates.
7. Impact is ADDITIVE, migration is none, both changelogs are updated, and the
   Gate 2 handoff contains `Adopter docs parity: updated`.
8. All named validation, archive, strict-docs, positive, and negative checks
   pass at the exact implementation head; CI is green with no duplicate paid
   pipeline left running.
9. No implementation or `v2.5.0` release state begins until separate binding
   Gate 1/Gate 2 decisions authorize each layer.

## Risks, failure boundaries, and rollback

- **Exception becomes a shortcut.** Mitigation: immutable-candidate evidence,
  Governor decision, separate release plan, and explicit ineligibility on any
  functional correction.
- **Protected ref is substituted or deleted.** Mitigation: API-read protection
  evidence at exact access level 40, exact-SHA comparison before
  edit/merge/tag, exact merge-command evidence, immediate post-merge API proof
  of both retained refs, and fail-closed re-review on any change.
- **Release tag is process-immutable but platform-mutable.** Mitigation: require
  an exact or governed wildcard protected-tag rule with Maintainer-only creation
  before the stable tag exists; never treat written policy alone as platform
  enforcement.
- **Tag bytes differ from reviewed bytes.** Mitigation: tag exact merge commit,
  provenance/source-commit checks, real downloaded-asset validation, never
  relabel artifacts.
- **Forward sync overwrites newer main work.** Mitigation: merge from a frozen
  latest-main base, net-diff allowlist, nonmetadata byte-identity proof, no
  squash/rebase, target-advance invalidation.
- **Operational docs drift from owner.** Mitigation: one canonical owner,
  bounded projections, structured projection check, archive inspection.

Before this rule ships, rollback is ordinary: revert the implementation MR's
nine-file change set through a reviewed corrective MR. After adopters receive
the rule, corrections move forward with a new governed version; historical
release/tag evidence remains immutable. This plan itself creates no external
release state to roll back.

## Stop conditions

Stop and return to the Governor if consultation or implementation requires a
CI/script/schema/tooling change, permits a functional release diff, weakens
branch or tag protection, cannot prove nonmetadata preservation, needs a
force-push/reset/tag move, or would begin the final release before this
amendment and its separate release plan each clear their own gates.

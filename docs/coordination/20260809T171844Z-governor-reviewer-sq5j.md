---
artifacts_owned: []
from: governor-reviewer
id: 20260809T171844Z-governor-reviewer-sq5j
immutable: true
kind: decision-record
model: claude-sonnet-5
references-to:
- 20260809T170045Z-planner-architect-jrhh.md
- 20260809T162523Z-planner-architect-1hqh.md
requires-ack: true
timestamp: 2026-08-09T17:18:44Z
to: planner-architect
topic: maintenance-line-security-assessment
---

# Security assessment: maintenance-line release exception — CLEAR, plus one separate high-value finding

**Role note.** `jrhh` requested infosec-sentinel screening. `infosec-
sentinel.yaml`'s actual charter is scoped exclusively to information-
leak/anonymization screening in learning records — its
`escalation_triggers`, `jurisdiction.reads`, and `capabilities` say
nothing about branch protection, tag integrity, or supply-chain
concerns. Rather than stretch that role's banner over content it
wasn't defined for, the human governor performs this assessment
directly, citing `always/security-hardening-rule.md`'s Supply Chain
section (the actual governing rule for pinned versions, protected
refs, and provenance) as authority. This mirrors `qpkg`/`2mmn`'s
transcription pattern.

Scope: `docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md`
at exact head `1a174875e81b967532ad24dcdee09a2c14928f4a` (Draft MR
!1335), assessed against `1hqh`'s named concerns (protected-ref
integrity, exact-SHA invalidation, ref-substitution/deletion,
tag/provenance binding, four-namespace preflight, retained refs,
artifact identity, metadata-forward synchronization) plus live GitLab
API state, not assumed.

## Disposition: CLEAR for Gate 1, no blocking findings against the plan itself

The plan's own contract already addresses every concern `1hqh` named:

- **Protected-ref integrity / ref-substitution / deletion**: eligibility
  contract requires the target protected *before* release content
  exists, re-read from the platform API as evidence, force-push and
  deletion rejected. Sound.
- **Exact-SHA invalidation**: any change to source, target, merge base,
  or candidate head invalidates prior review; both refs confirmed
  against the frozen SHA before the first edit. Sound.
- **Tag/provenance binding**: one annotated tag on the exact
  maintenance-target merge commit, tag pipeline binds every archive,
  checksum, provenance record, package, and OCI digest to that commit;
  never moved or recreated, matching `immutable-tags-rule.md`. Sound.
- **Four-namespace preflight**: checked twice — once in the eligibility
  contract before any branch exists, once again before tag creation.
  This is *more* thorough than the plan `dqsd` blocked earlier this
  session (B-FINAL-6 found only a single, too-late check there). Sound.
- **Metadata-forward synchronization**: `--no-ff`, non-squash,
  non-rebase, byte-identity proof against a frozen main base, target-
  advance invalidates the proof. This directly resolves the structural
  conflict that blocked MR !1325 (`dqsd`'s B-FINAL-1). Sound.
- **Credential/secret exposure**: verified via `GET /projects/:id/variables`
  — empty; this project's release/publish jobs authenticate via
  GitLab's ambient `CI_JOB_TOKEN`, not stored secrets. No new
  credential surface is introduced by this plan.

## One concrete strengthening recommendation (not a blocker)

**"Governed maintainer role" should name an exact GitLab access level.**
Verified via `GET /projects/:id/protected_branches`: `main` currently
allows push at level 0 (no direct push) but merge at levels 40 and 30
(Maintainer and Developer both). For `release/*` — closer to the
immutable-tag boundary than ordinary feature work — recommend
Maintainer-only (40) for *both* push and merge, stricter than `main`'s
current setting, not inherited from it by default.

## One evidenced, not-hypothetical caution on "retained refs"

This session already produced a live example of exactly the failure
mode `1hqh` asked about: MR !1289's Gate 2 decision (`q86p`) explicitly
required retaining the source branch
(`should_remove_source_branch: false`); the actual merge command still
deleted it, because `glab mr merge --yes` was run without the
overriding flag (`e2tk`'s BR-1 finding, since resolved). **A stated
retention requirement in a decision-record is not sufficient by
itself** — this plan's implementation MR must require the merge
command's actual invocation to be shown, and an immediate post-merge
`GET` on the ref's existence, as Gate 2 evidence — not just the
requirement text. Recommend adding this explicitly to the plan's
validation matrix.

## Separate, higher-value finding: this project currently has zero protected Git tags

Verified via `GET /projects/:id/protected_tags`: **empty** — no
protected-tag pattern exists at all, for any version. This is
independent of the maintenance-line plan and predates it: `v2.5.0-
rc.1`, `v2.5.0-rc.2`, and every prior release tag are today enforced as
immutable by `immutable-tags-rule.md`'s written discipline only, not by
any GitLab platform protection. `GET /projects/:id/members/all` shows
three Developer-level (access 30) service accounts alongside the Owner
— Developer-level push access can delete or force-move an unprotected
tag today with no platform barrier.

This is not a defect in the maintenance-line plan and is not blocking
its Gate 1. It is a real, verified, pre-existing gap this consultation
surfaced while checking adjacent state. Recommend a separate, small
follow-up: configure a protected-tag pattern (matching this project's
`vX.Y.Z*` convention) so `immutable-tags-rule.md`'s guarantee is
platform-enforced, not process-only. The maintenance-line plan should
still require tag protection for its own new `v2.5.0` tag specifically
regardless of whether this broader follow-up lands first.

## Authority

This assessment is advisory. Classification and acceptance of any
residual risk remain human/governor decisions, consistent with
`security-hardening-rule.md` and the boundaries `infosec-sentinel.yaml`
itself states even for its own in-scope findings. Gate 1 approval for
`docs/plans/20260809T163051Z-v250-maintenance-release-line-rule.md`
remains a separate, not-yet-taken decision.

---
id: 20260909T203636Z-v211-release-publication
status: pending-gate1
topic: v211-release-publication-child-plan
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record. This is the
      release-publication procedure: metadata routing, the mandatory
      four-namespace `release:namespace-check`, the protected-tag policy,
      the fail-closed pre-tag proof, and anonymous post-publication
      verification of downloaded bytes. InfoSec must confirm the namespace
      absence reads (incl. the OCI reference derived by
      `agov-oci-image.sh image-ref`, not a hand-written namespace), the
      exact protected-tag rule policy (`v2.11.0`, creation restricted to
      Maintainers), and that no credential appears in any recorded evidence.
  - role: contribution-auditor
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record.
      Contribution-auditor must confirm the derived-not-asserted metadata
      allowlist method (audit command + three independent agreements),
      that Group B live-status prose is treated as Gate 2 review material
      rather than a mechanical bump, that the CHANGELOG states
      adopter-visible change rather than a commit count, and that
      `Closes #N` is informational (issues close after verified
      publication).
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Publish v2.11.0

## Authority and provenance

- Governor direction (2026-09-09): "plan draft release content and
  mechanism to close vast majority of open MRs and Issues"; `st83`; and
  **`j8bq`** — release-first is binding: bulk forge close/park/reconstruct
  of the queue is **not** a v2.11 cut precondition, and happens only after
  v2.11 is published and verified (that work is the separate
  `20260909T195116Z-post-v211-queue-closeout-work-order`).
- Parent: `docs/plans/20260909T021841Z-v211-release-scope.md`
  (Gate 1 approved `sd23`; amended for the #129 withdrawal and, via
  `j8bq`, for release-first ordering). This is the **child v2.11
  release-publication plan that parent requires**, split from the
  withdrawn combined plan per review `nj1j`.
- Structure follows the v2.9.0 release-publication plan
  (`docs/plans/20260831T050110Z-v290-release-publication.md`), which
  proved out the derived-allowlist and fail-closed-pre-tag discipline.

This plan authorizes **no** version bump, metadata merge, tag, release, or
deployment. Steps execute only after the preconditions below are all
green, and the tag step (Step 6) is a **separate binding governor
decision** taken after the metadata MR merges (parent scope plan A4).

## Blocked preconditions, stated first

The release cannot open a metadata MR until **all** hold. Per `j8bq`, the
queue *closeout execution* is not among these; the *IN-workstream* merges
and evidence, and the B4–B7 *dispositioning* (ledger rows, not closes),
are.

| Precondition | Discharged when |
|---|---|
| CP-B publication efficiency | CP-B-1 impl (MR !2636, incl. the `emrq` copied-record clarification) merged + evidenced **and** CP-B-2 (hold + urgency) merged + evidenced |
| Coordination lifecycle semantics (#121, #122, !2027, CP-A) | its Gate 1 work order + each slice's Gate 2; #122 after `qbka` Slice A |
| Coordination discovery/collision (#123, CP-C) | Gate 1 + Gate 2 |
| `qbka` Slice A / Slice B | Slice A schema+validator+transition merged; Slice B 12-file migration merged |
| Improvement Propeller Stage 4 (#131) | the first **genuine unattended Monday wake** with a conforming result **and** the separate Level-5 maturity record |
| Queue convergence B4–B7 | every currently-open MR has an evidence-backed ledger row (dispositioning only; forge closes are post-release) |
| Pre-cut governor dispositions (`release-preflight-learning-gate2`, `capability-developer-startup-sync`) | a governor decision/supersession record filed (MRs already merged) |
| Runner capacity + DinD | confirmed sufficient for the tag pipeline (see Step 5d) **before** tagging — the tag pipeline is the one pipeline that must not fail midway (v2.9.0 was blocked on CI quota; v2.7.0-rc.1 was burned by tagging without a working pipeline) |

`main` must also carry a green pipeline on the exact release base with no
stranded governance records.

## Steps

Per `agf/rules/workflow/release-via-merge-request-rule.md`, **standard
workflow** — no maintenance-line or frozen-scope-reconstruction exception
applies (v2.11.0 ships from current `main`; no immutable release candidate
exists to promote).

### 1. Confirm `main` is releasable

Green pipeline on the exact release base. Every IN-workstream Gate 2 record
and the Propeller Level-5 record are on `main`. No unmerged review/ack
coordination records that belong in the audit trail for shipped work.

### 2. Derive the metadata allowlist at the frozen base (not a hand list)

**Audit command**, run at the frozen release base; its output is Gate 2
evidence:

```sh
git grep -n -F '2.10.0' -- \
  ':!docs/coordination' ':!docs/plans' ':!docs/reviews' \
  ':!CHANGELOG.md' ':!agf/CHANGELOG.md' ':!agf/learnings'
```

The exclusions are append-only history that must never be rewritten.
Classify every occurrence into exactly one group:

- **Group A — authored current-version cells → become `2.11.0`.**
  Expected members (verify counts at the base, do not assume): `agf/VERSION`;
  `AGENTS.md:1` and `agf/templates/AGENTS.md:3` (`<!-- AGF: … -->` marker,
  validator-enforced against `agf/VERSION`); `agf/templates/learning-record.yaml`
  `framework_version`; `agf/agents/MANIFEST.yaml` `Framework version:`; every
  `agf/agents/*.yaml` role definition `framework_version:` (one per shipped
  role); `agf/ADOPTING.md` adopter example; `tooling/rust/Cargo.toml`
  (workspace version + path-dep constraints); `tooling/rust/Cargo.lock`
  (regenerate, never hand-edit); `agf/docs/governance/manifest.md`
  current-version cells.
- **Group B — live release-status prose → rewritten, not bumped. Gate 2
  review material.** Any text asserting v2.10.0 is "latest / published /
  in preparation" and directing adopters to an older install, in
  `agf/docs/governance/manifest.md` and `agf/docs/site/roadmap.md` — plus
  **the stale #129 roadmap-owner row correction** (`nj1j` advisory): the
  `docs/roadmap.md` / `agf/docs/site/roadmap.md` #129 rows must reflect the
  withdrawal (`6sj1`), and that edit rides this metadata MR.
- **Group C — genuinely historical → byte-identical.** Incident
  references, measurement notes, selftest assertion strings (changing them
  breaks the selftests that discharge other criteria).

**Acceptance — three independent agreements** (per `nj1j` and v2.9.0
review `V29P-1-R3`): before the edit, re-run the audit at the frozen base
and require **per-file counts**, **category counts**, and **total** to all
agree with the derived tables. Any one disagreeing stops the release, even
when the total happens to be right. After the edit, re-run: every
surviving `2.10.0` must be in Group C, matched line-for-line; `agov
validate assistant_loader` passes; the built binary reports `2.11.0`.

### 3. Release branch and disposable validations

- Branch **`release/v2.11.0`** from `main` (resolving the `release-2.X.Y`
  vs `release/vX.Y.Z` inconsistency in favour of the rule's
  `release/<tag>` form; v2.9.0's `release-2.9.0` was the older shape).
- Run the reversible / disposable preflight from a clean checkout: full
  Rust workspace test suite; each built framework archive's shipped tests;
  `agov dist validate` / `agov dist build`. Record counts. These
  validations are disposable — they leave no commit.
- Only then create the **metadata commit**, containing exactly the Group A
  bumps + Group B rewrites + the CHANGELOG entry and nothing else. Keep it
  **local-only / unpublished** until Step 4 (`nj1j` V211-P2: the
  preflighted local commit is retained as the MR head).

### 4. Namespace preflight on the clean metadata commit, then the MR

- With the metadata commit as `HEAD` on `release/v2.11.0`, run the
  operator preflight:
  `agov release namespace-check --phase preflight --api-url "$CI_API_V4_URL"
  --project-id "$CI_PROJECT_ID" --project-path "$CI_PROJECT_PATH"
  --registry-image "$CI_REGISTRY_IMAGE" --target-branch main`.
  Only exit 0 with `result: "ABSENT"` for all four namespaces (Git tag,
  GitLab Release, generic package, derived OCI reference) proceeds.
- Push `release/v2.11.0`; open the MR to `main`. Diff limited to the
  allowlist files. The MR job `release:namespace-check` must pass;
  exact-head pipeline green (`agf:dist:validate`, `agf:dist:preview`,
  `release:paths-validate`, `release:self-sufficiency-selftest`,
  `tooling:live-surface-validate`, `commercial-corpus:validate`,
  `agov:governance:validate`, `rust:tooling:*`).
- Binding governor **Gate 2** on the metadata MR.

### 5. Fail-closed pre-tag proof (after merge, before the tag)

Immediately before the irreversible tag; any non-absence or ambiguity
stops the release.

a. **Authoritative absence** for: the Git tag `v2.11.0`; the GitLab
   Release; the generic package version across **every** status
   (default, hidden, processing, error, pending_destruction, deprecated)
   fully paginated; and the **OCI reference as derived by
   `agov-oci-image.sh image-ref`** (never a hand-written namespace — the
   v2.8.0 defect). `agov release namespace-check --phase pre-push` is the
   mandatory check and must immediately precede the tag op.
b. **Protected-tag rule with an explicit expected policy**: exact tag
   `v2.11.0` (not a wildcard), creation restricted to Maintainers
   (access level 40). Prove by authoritative API read **before the tag
   exists**; record the returned object. Any other policy needs an
   explicit governor disposition.
c. **Frozen source/target**: re-read `main`'s tip and the merged metadata
   commit; require them to equal the reviewed SHAs; verify the merge
   commit's parents and tree against the reviewed values. Any drift stops
   here.
d. **Runner/DinD capacity**: confirm sufficient capacity for the tag
   pipeline (`agf:dist:release`, `rust:tooling:release`, `agov:oci:release`,
   `release:create`, `release:verify`) — before tagging, not after.

### 6. Tag and push — separate binding governor decision

Per parent scope plan A4, a separate governor decision-record after the
metadata MR merges authorizes the tag. Then: create **one annotated tag**
`v2.11.0` on the proven merge commit and `git push origin v2.11.0` in a
single action (the merged MR + the tag decision are the authorization;
no further confirmation). Never move, delete, or recreate it
(`immutable-tags-rule`).

### 7. Post-publication binding and anonymous verification

Bind the tag object and its peeled commit to every archive, checksum,
provenance record, package identity, and OCI digest. Re-read each
namespace. Then verify **on anonymously downloaded bytes** — not on the
build's own claims. `glab release view v2.11.0` shows the archives.

## CHANGELOG scope

`CHANGELOG.md` + `agf/CHANGELOG.md` entries state **adopter-visible
change**, not a commit count:

- Coordination protocol semantics (#121 session addressing, #122
  supersession surfacing, !2027 priority + material challenge-response,
  CP-A answered-vs-ack state);
- Coordination publication efficiency (CP-B: multi-record MRs, release-lane
  hold, urgent bypass);
- Coordination discovery/collision tooling (#123, CP-C);
- `qbka` plan-lifecycle `superseded` schema + provenance + validator;
- Improvement Propeller reaching genuine Level 5;
- Queue convergence B1–B8 (roadmap owner, TODO + issue ledgers, intake
  bound) as governance hygiene.

Each grouped BREAKING / features / fixes with the `downstream-impact`
classification per workstream; evidence citations (learning IDs, decision
records). `Closes #N` references are **informational** — GitLab does not
auto-close from CHANGELOG content; the `ship-in v2.11.0` issues (#121,
#122, #123, #124, #131) close **after verified publication** (Step 7).

## Shipment inventory

A standard release tags the **whole selected tree** at the frozen base.
Merged adopter-visible work is not enumerated commit-by-commit; the
CHANGELOG scope above is the adopter-facing statement. Plans, learnings,
and coordination records are repository history, not shipped change.

## Explicitly out of scope

- The post-v2.11 queue-closeout execution — separate work order
  (`20260909T195116Z-…`), runs after this release is verified.
- Issue #129 and MRs !2459 / !2460 / !2462 — semantic-data exclusion
  (`j8bq`), no analysis / reconstruction / carrier / close here.
- The OTGR relay spec (MRs !2592 / !2587) — not v2.11 scope.
- Any parent-scope CANDIDATE row (#116, #128, #132) — v2.12+.

## Risks

- **A precondition turns out incomplete.** v2.11 waits, or the governor
  cuts scope by decision. The release does not ship on assumed evidence.
- **The allowlist is derived wrong a fourth time.** Control: three
  independent agreements (per-file, category, total), pre- and
  post-edit; a survivor outside Group C stops the release.
- **Tag pipeline capacity is thin.** Control: Step 5d confirms capacity
  before the tag; the tag pipeline is the release mechanism and must not
  fail midway.
- **Namespace drift between preflight and push.** Control: the Step 5a
  pre-push namespace-check + the Step 5c frozen-SHA re-read immediately
  before the tag.

## Rollback

Before the tag: abandon `release/v2.11.0`; nothing is published. After the
tag: `immutable-tags-rule` applies without exception — the tag stays, a
defect ships as v2.11.1, and there is no path that moves or deletes a
pushed tag. A failed post-tag binding or forward-metadata sync is
corrected **forward** without touching the tag.

## Validation before Gate 1

`agov plan validate` on this file; `git diff --check`; coordination
visibility/collision scan. No projection manifest — this is a release
procedure over the existing `release-via-merge-request-rule`, introducing
no normative rule text.

## Decision requested at Gate 1

Approve (a) the precondition set as the binding gate before a metadata MR
opens, (b) the derived-not-asserted allowlist method with three
independent agreements, (c) the Group A/B/C classification approach incl.
Group B as Gate 2 review material and the #129 roadmap-row correction
riding this MR, (d) the Step 3–4 ordering (disposable validations →
local-only metadata commit retained as MR head → clean-commit namespace
preflight → MR), (e) the Step 5 fail-closed pre-tag proof incl. the
explicit protected-tag policy and the derived OCI namespace read, (f)
Step 6 as a separate post-merge governor tag decision, and (g) the
CHANGELOG-scope and out-of-scope boundaries.

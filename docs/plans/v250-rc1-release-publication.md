---
id: v250-rc1-release-publication
status: pending-gate1
topic: v250-rc1-release-publication
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      Round 1 review (`20260803T135943Z-contribution-auditor-gbkp.md` +
      addendum `20260803T140917Z-contribution-auditor-qprt.md`) at exact
      head `9ae18ca4`: BLOCK, seven blocking findings (B-RC-SCOPE-1,
      B-RC-GATE-METADATA-2, B-RC-VERSION-SURFACES-3,
      B-RC-ARTIFACT-MATRIX-4, B-RC-PRERELEASE-5, B-RC-BRANCH-6,
      B-RC-TRACKER-7) plus two advisories (A-RC-MATURITY-1,
      A-RC-ROADMAP-2). All addressed in this revision: the
      scope-reconciliation table now includes the governance-layout-rule
      fix and issue #101's cache-topology command (both previously
      omitted or wrongly dismissed) under one combined scope-amendment
      decision; Gate 1 metadata now updates on this plan's own MR before
      any release branch is cut; all twelve `agf/agents/*.yaml` role
      definitions are named in the version-bump scope; the
      upgrade/overlay validation is split into framework-archive overlay
      (schemas/templates/rules, no binary) versus platform-tooling-archive
      CLI/MCP parity (the actual binaries), each tested separately;
      the GitLab-pre-release authorization is withdrawn in favor of the
      verified ordinary-Release precedent; the release branch name is
      corrected to `release-2.5.0-rc.1` (no `v`); the v2.5.0 milestone's
      open #33/#67/#78 now get an explicit per-issue disposition step
      before the release is called feature-complete; and maturity language
      throughout now distinguishes "merged, Level 4 Validated" from
      Level 5 Deployable. Round 2 review
      (`20260803T152434Z-contribution-auditor-pgq6.md`) at exact head
      `4950be18`: BLOCK, three narrowed blocking findings
      (B-RC-SCOPE-R2-1, B-RC-ARTIFACT-R2-2, B-RC-TRACKER-R2-3) plus two
      advisories (A-RC-AUDIT-COUNT-R2, A-RC-HANDOFF-DESC-R2), confirming
      the other four round-1 blockers and both round-1 advisories
      resolved. All three addressed in this revision: the scope table now
      has separate framework-archive/tooling-archive columns and #101's
      row is corrected to "No" for the framework archive; the
      `agf/CHANGELOG.md` coverage claim is corrected (issue #101 has none
      today, framed as an explicit governor decision) and the "inert for
      any adopter" overclaim is withdrawn; the CLI/MCP learning-validation
      interfaces are corrected to the real `agov validate learnings
      <path>` / `agov_validate` shapes (bare `agov validate` has no
      generic validation action); and #67/#78 now carry a full per-finding
      resolution map, independently built from each issue's complete
      current body, with an exact recommended disposition (#67: close,
      every finding resolved; #78: keep open, two of four findings
      genuinely still unresolved) rather than a deferred
      close-or-leave-open algorithm. Round 3 review
      (`20260803T155300Z-contribution-auditor-g1ks.md`) at exact head
      `1e20c0ff`: BLOCK, one blocking finding (B-RC-TRACKER-R3-4) plus two
      advisories (A-RC-HANDOFF-DESC-R3, A-RC-BRANCH-RETENTION-R3),
      confirming B-RC-SCOPE-R2-1 and B-RC-ARTIFACT-R2-2 resolved. Addressed
      in this revision: the #67/#78 map is rebuilt from each issue's full
      comment history rather than the initial body alone — #67 gains
      findings 7-11 (7 and 9 resolved by #80 and the current loader
      template respectively, 10 informational, 8 and 11 independently
      re-verified as still open) and its recommended disposition flips
      from close to **keep open, remove from milestone**; #78 finding 1's
      proposed relocation is withdrawn entirely — the issue author
      retracted it and governor triage (MR !749,
      `20260728T154327Z-governor-reviewer-gkj5.md`) made "do not relocate
      declared projection/review paths" binding — and finding 4 is
      recorded as routed to already-open, already-`v2.6.0`-milestoned
      issue #76 rather than left as open #78 scope, flipping #78's
      recommended disposition from keep-open to **close**; the merge/tag
      sequence now requires explicit `should_remove_source_branch: false`
      through the merge API, verified post-merge, rather than relying on
      an unstated project default. Round 4 review
      (`20260803T161807Z-contribution-auditor-s224.md`) at exact head
      `e278b6ff`: BLOCK, two narrow blocking findings
      (B-RC-TRACKER-EVIDENCE-R4-5, B-RC-HANDOFF-TRUTH-R4-6), confirming
      the substantive #67/#78 dispositions, both advisories, and the
      branch-retention instruction correct. Addressed in this revision:
      #67 finding 8's evidence no longer conflates the total
      `body-reference` edge population (~612-614, all shapes) with the
      subset actually matching the timestamp-plus-four-character collision
      shape (73 edges, 37 unique targets) or with the issue's own narrow
      reproduction (two direct learning-ID targets cited in one message
      body) — all three are now stated as distinct facts; MR !1051's
      GitLab description is corrected to recommend #33/#67 stay open
      (removed from v2.5.0) and #78 close, matching the plan body instead
      of contradicting it. A bounded round-5 review is requested before
      this is treated as Gate 1-ready.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Release routing, version, and changelog metadata only, over content
      already individually assessed at its own implementation Gate
      (#79/#80's infosec consultation in `v250-rc1-contract-core.md`; the
      learning-supersession plan's own consultation). No new credential,
      dependency, network-trust-boundary, or authorization-semantic change
      is introduced by this plan.
---

# Plan: publish v2.5.0-rc.1

## Decision requested

Authorize the release-only steps that publish the current `main` delta
since `v2.4.11` as `v2.5.0-rc.1`, per
`docs/plans/v250-statement-of-work.md`'s release train and
`docs/plans/post-v248-release-batching-amendment.md`'s batching scope — with
one explicit scope-reconciliation decision requested below, since the
actual `main` delta is broader than the batching amendment's original
"authority and acknowledgement contract core" framing.

Current baseline: `origin/main` at `ca5a56d68bbce222c77ec6568d0e8ac2a830d478`
(refreshed after
contribution-auditor's round-1 review `gbkp`/`qprt`; the only commits
between the prior `f77d80d2` baseline and this one are this plan's own
review coordination records plus MR !1050, a repo-internal Gate 1 metadata
sync for the already-merged historical-recovery plan — confirmed via
`git log f77d80d2..ca5a56d6 --oneline --no-merges`, no new shipped surface).
Latest published release: `v2.4.11`. `agf/VERSION` currently reads
`2.4.11`. At planning time there is no `v2.5.0-rc.1` tag, GitLab Release, or
generic package (preflight-confirmed by `qprt`).

This plan authorizes only: the scope decision below, release metadata, a
version-only Rust bump, upgrade/overlay validation, the release MR, one
immutable annotated tag after the release MR merges, publication
verification, and issue #79/#80 closure-link confirmation (both issues are
already closed on GitLab; this plan does not reopen or re-close them, it
records the release that ships their implementation). No product
implementation, new command semantic, schema field, or validator behavior
is authorized here — every one of those was merged and Level 4 Validated
(per `implementation-honesty-rule.md`) in a separately Gate 2-approved
implementation MR, listed below. This plan is what carries each to Level 5
Deployable, via the publication path itself.

## Scope reconciliation: the actual `main` delta since v2.4.11

Independently audited (`git log v2.4.11..origin/main --no-merges`, 121
commits measured at baseline `f77d80d2`; this count grows continuously as
the repository's own coordination/plan activity continues — cite the
baseline SHA, not the count, as the reproducible anchor if re-verifying),
reclassified by what they touch and cross-checked against
`agf/dist/release-manifest.yaml`'s actual `include`/`exclude` globs — the
only authoritative answer to "does this ship in the archive." The refresh
to `ca5a56d6` (5 more commits, all coordination records plus one
repo-internal metadata-sync MR, per the "Current baseline" note above)
added nothing to reclassify.

| Surface | Ships in framework archive (`agf-{version}.tar.gz`/`.zip`)? | Ships in a platform tooling archive (`agov`/`agov-mcp` binaries)? | In original batching-amendment scope? | Disposition |
|---|---|---|---|---|
| **#79** — upgrade-safe adopter role authority | Yes (`agf/agents/**`; `MANIFEST.yaml` explicitly excluded, `MANIFEST.example.yaml` ships) | No new binary behavior; the fail-closed error path is `agov` logic, ships in the tooling archive | Yes, named explicitly | Ship as planned. |
| **#80** — `requires-ack` obligation semantics | Yes (`agf/schemas/**`) | Yes — new `agov coord pending` command ships in the Rust tooling archive binaries | Yes, named explicitly | Ship as planned. |
| **Learning-record `status`/`supersedes`** (`docs/plans/learning-supersession-status-fields.md`, Path B, MR !1025, merged `e6dbc0a3`) | Yes (`agf/schemas/**`, `agf/templates/**`) | Yes — the new validator logic ships in the tooling binaries | **No** | **Governor scope-amendment decision requested — see below.** |
| **`governance-file-layout-rule.md` correction** (both `.claude/rules/` and `agf/rules/` copies, reclassified Additive per Codex MR !916, 2026-08-02) | Yes — `agf/rules/**` is an `include` glob. **This was omitted from the original audit entirely; found by contribution-auditor's review (`gbkp`, B-RC-SCOPE-1), not by me.** | No binary change | No | **Governor scope-amendment decision requested — see below.** |
| **Issue #101 — CI cache-policy Slice A/B** (`agov ci cache-topology-check` command) | **No.** **Correction (round-2 review, B-RC-SCOPE-R2-1): the first revision wrongly answered "yes" for this row using tooling-archive reasoning under a column asking specifically about the framework archive.** `ci_cache_topology.rs` is not under any `agf/` path and is not in the framework-archive `include` glob. | **Yes** — the compiled command necessarily enters every Rust tooling binary built from current `main`. The CI config/script changes that consume it (`.gitlab-ci.yml`, `.gitlab/scripts/`) do not ship in either archive — those are `forbidden` paths. | No | **Governor scope-amendment decision requested — see below.** |
| `docs/plans/historical-branch-only-coordination-recovery.md` + its inventory (MR !1010, merged `f77d80d2`) | No — `docs/plans/` and `docs/reviews/` (except the two individually-included review docs already in the manifest) are not in the `include` glob. | No | No, and this is repository-internal governance process, not a product surface. | No release-content impact. No changelog entry needed — no other internal plan-drafting commit gets one either. |
| MR !1050 (Gate 1 metadata sync for the already-merged historical-recovery plan) | No — plan-file frontmatter edit only, not in any `include` glob. | No | N/A | No release-content impact. |
| Source registry / radar tracking (`docs/sources.md`, `docs/radar.md`, `docs/plans/radar-governance-signal-gaming.md`, `docs/TODO.md`) | No — repo-root `docs/`, not `agf/`. | No | No. | No release-content impact (confirmed against the manifest). Currently has zero coverage in either changelog's `[Unreleased]` section; this is a `CHANGELOG.md` (repository-level) completeness question, not a release-artifact question — flagged as a non-blocking observation, not a stop condition, since repo-root `CHANGELOG.md`'s own precedent already omits other purely-internal governance/plan commits (the historical-recovery-plan row above) without complaint. |
| `agf/learnings/foundry.yaml` edits | **No** — `agf/learnings/**` is not in the `include` glob at all. This project's own learning corpus never ships. | No | N/A | No release-content impact. Confirms there is no latent version of the #78 MANIFEST-collision problem here (unlike `agf/agents/**`, `agf/learnings/**` was never included, so nothing needed excluding). |

### Governor scope-amendment decision requested: three surfaces ship but weren't in the original batching-amendment scope

**Correction:** the first revision of this plan wrongly treated only the
learning-supersession feature as needing a scope decision, and wrongly
concluded issue #101's cache-topology command needed none because its
*purpose* is CI-internal. Contribution-auditor's review (`gbkp`,
B-RC-SCOPE-1) correctly found both errors and a third omission
(`governance-file-layout-rule.md` was never in the table at all, despite
the governor's own handoff `t41t` naming it explicitly and both changelogs
already classifying it Additive). All three ship in the archive today
regardless of intent or purpose — that is the only test that matters for
this table. Presenting one combined scope-amendment decision, per `gbkp`'s
own suggestion that one combined amendment is acceptable:

1. **Learning-record `status`/`supersedes`** — already merged to `main`
   (MR !1025, Gate 1 via `3q28`, Gate 2 via `h75h`/`1gbs`). Additive minor,
   two new optional schema fields, no existing behavior changed.
2. **`governance-file-layout-rule.md` correction** — already merged, both
   changelogs already classify it Additive (reclassified from Editorial in
   Codex's MR !916 review, 2026-08-02): the removed dual-layout block was
   inert, so the replacement single-layout ownership table is new operative
   guidance, not a restatement.
3. **Issue #101's `agov ci cache-topology-check` command** — already
   merged, ships as a new CLI subcommand in the platform tooling archive
   binaries only (not the framework archive). **Correction (round-2
   review, B-RC-SCOPE-R2-1): the first revision claimed this command is
   "inert for any adopter" because its motivating use case is this
   project's own CI. That overclaims — the command accepts arbitrary
   merged GitLab CI YAML plus configurable cache keys/job names as input;
   nothing prevents an adopter from pointing it at their own CI
   configuration. The accurate statement is narrower: the command was
   *built for* this project's own CI-cost problem and carries no
   documented adopter workflow, not that it is technically unusable by an
   adopter.**

**Changelog/documentation treatment decision, per B-RC-SCOPE-R2-1
(resolves the false "both changelogs already cover it" claim):** issue
#101 currently appears in root `CHANGELOG.md` (repository-level, correct —
this is this repository's own CI-cost work) but **nowhere in
`agf/CHANGELOG.md`** (the adopter-facing distribution changelog) —
verified directly, not assumed. Because the command ships in the
adopter-downloaded tooling binary and in `agov --help`, silence in
`agf/CHANGELOG.md` is not currently honest: an adopter inspecting their
own `--help` output after upgrading would see a command the adopter-facing
changelog never mentioned. **Recommended: add a short `agf/CHANGELOG.md`
entry** stating the command exists, what it inspects, and that it carries
no adopter-facing obligation semantic or documented adopter workflow —
matching the corrected, narrower claim above rather than "inert for any
adopter." The governor may instead choose to explicitly accept root-only
coverage with a stated reason; either is a valid Gate 1 decision, but the
plan must not proceed with both files "already covering it" as a settled
fact when only one does.

**Recommended: Option A, amend the boundary to include all three.**
Justification, applying uniformly: all three are already on `main`,
already independently Gate 2-approved, and are the same additive-minor
SemVer class as #79/#80. Two of the three (learning-supersession,
governance-layout) already have complete `CHANGELOG.md`/`agf/CHANGELOG.md`
coverage; the third (#101) requires the adopter-facing changelog decision
above before it can be said to have complete coverage. Excluding any of
the three would require reverting an already-approved merge or branching
the release from an older, non-`main` commit — more costly and less
honest than acknowledging the scope grew by three governed items since the
amendment was written. `post-v248-release-batching-amendment.md`'s own
text does not forbid amendment; it states the batches as the then-current
decision, not as immutable.

**Option B (per surface): hold any of the three for v2.6.0.** Would
require cherry-picking #79/#80 onto a release branch that excludes the
held commits, or reverting the feature(s) from `main` until v2.6.0. More
invasive than the features themselves and risks a broken cherry-pick given
`main`'s linear history since each merge. The governor may choose Option B
for zero, one, two, or all three surfaces independently — they don't have
to be decided as a single unit even though they're presented together.

This plan proceeds on **Option A for all three** pending explicit governor
confirmation at Gate 1 — do not treat silence as approval; if the governor
prefers Option B for any surface, this plan requires a revision (removing
that surface's changelog entry and shipped artifact from this candidate)
before Gate 1 can close.

## SemVer classification (independently verified)

All five in-scope feature surfaces (#79, #80, learning-supersession,
governance-layout-rule, #101's cache-topology command) add optional new
capability with no removed or redefined existing behavior:

- #79: renames a shipped template file, adds one new fail-closed error path.
  No existing command, flag, or schema field changes meaning.
- #80: adds a new command (`agov coord pending`) and one narrow schema
  rejection (`kind: ack` + `requires-ack: true`, previously schema-legal
  but functionally inert — not a behavior anyone relied on).
  `handoff next`/`take`/`complete` and `coord graph-index` are provably
  unchanged (existing tests pass unmodified).
- Learning-supersession: two new optional schema fields (`status`,
  `supersedes`). `superseded_by` is marked `x-deprecated: true` but remains
  schema-legal (Path B, deprecate-don't-remove).
- Governance-layout-rule: a rules-document correction. No schema, command,
  or validator behavior change — Additive because the removed block was
  inert (asserted no concrete guidance), not because it changes runtime
  behavior.
- #101's cache-topology command: one new CLI subcommand
  (`agov ci cache-topology-check`), built for this project's own CI-cost
  problem with no documented adopter workflow — not technically inert for
  an adopter (it accepts arbitrary merged CI YAML as input), just
  unaddressed to one. No schema or existing-command change.

**Additive minor. `v2.5.0-rc.1` is the correct target**, matching
`v250-rc1-contract-core.md`'s own classification and the SoW's release-train
shape (RC checkpoint before final).

## Gate 1 metadata sequencing (corrects B-RC-GATE-METADATA-2)

**Correction:** the first revision of this plan deferred its own Gate 1
approval-metadata update to the later release branch (as release-only
change-set step 1). Contribution-auditor correctly identified this as the
exact failure mode MR !1010 just demonstrated: it merged with stale
`pending-gate1` metadata and needed a separate follow-on MR (!1050) to
repair it after the fact. `AGENTS.md`'s Gate discipline requires updating
plan metadata in the same review cycle as approval, not after merge.

**Corrected sequencing:**

1. When the governor approves Gate 1 (including the scope-amendment
   decision above), this plan's own frontmatter (`status`,
   `gate1_approved_by`, `gate1_approved_at`) is updated **on this plan's
   MR (!1051 or its corrected successor)**, validated, and merged to `main`
   as part of that same review cycle — before any release branch is cut.
2. The release branch (`release-2.5.0-rc.1`, see branch-naming correction
   below) is cut from `main` only **after** the approved plan (with correct
   metadata already on `main`) exists. The release-only change set below
   therefore never touches this plan file's own frontmatter — that
   contamination is exactly what B-RC-GATE-METADATA-2 flagged.

## Release-only change set

Create the release commit on `release-2.5.0-rc.1` (governed branch name
per `release-via-merge-request-rule.md`'s `release-X.Y.Z` form — the prior
`release-v2.5.0-rc.1` draft incorrectly added a `v`; this rule takes
precedence over any historical branch that also got this wrong), based on
the then-current `main` (with this plan's approved metadata already
merged, per the sequencing above), with only these release-truth changes:

1. Set `agf/VERSION` to `2.5.0-rc.1` (full prerelease string, matching the
   `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent — `agf/VERSION` carries the
   complete SemVer prerelease identifier, not just the numeric core).
2. Add dated `[2.5.0-rc.1]` entries to `CHANGELOG.md` and
   `agf/CHANGELOG.md`, converting the current `[Unreleased]` content
   (substance already drafted at each feature's merge time — see "Proposed
   changelog substance" below) rather than rewriting it from scratch.
3. Update the Rust workspace version: `tooling/rust/Cargo.toml`'s
   `[workspace.package] version` and its three internal path-dependency
   pins (`agov-core`, `agov-coord`, `agov-validators`) from `2.4.11` to
   `2.5.0-rc.1`, and the corresponding `Cargo.lock` entries. The other two
   crates (`agov`, `agov-mcp`) inherit via `version.workspace = true` and
   need no direct edit. No Rust source changes in this release beyond the
   version bump itself — all functional Rust changes already merged in
   #79/#80/learning-supersession/governance-layout/#101's own implementation
   MRs.
4. **Update current framework-version surfaces (corrects
   B-RC-VERSION-SURFACES-3):**
   - `agf/agents/MANIFEST.yaml` (this project's own roster — not shipped,
     but kept current for dogfooding honesty);
   - **all twelve adopter-facing `agf/agents/*.yaml` role definitions**
     (`build-engineer.yaml`, `capability-developer.yaml`,
     `coder-tester.yaml`, `contribution-auditor.yaml`,
     `deployment-engineer.yaml`, `documentation-writer.yaml`,
     `fact-checker.yaml`, `governor-reviewer.yaml`, `infosec-sentinel.yaml`,
     `planner-architect.yaml`, and any others present at implementation
     time — enumerate via `ls agf/agents/*.yaml`, don't hand-count), each
     currently `framework_version: "2.4.11"`, all shipped via
     `agf/agents/**`. **This was omitted entirely from the first revision**
     — it named only `MANIFEST.yaml`;
   - `agf/ADOPTING.md`, `agf/templates/AGENTS.md`,
     `agf/templates/learning-record.yaml`, and current version
     statements/examples in `agf/docs/governance/manifest.md`.
   - A full current-version scan (grep for `2.4.11` across every `agf/`
     path in the release diff) is required before Gate 2 to catch any
     further surface neither this plan nor the review named.
5. Update `agf/docs/site/roadmap.md`: first correct the already-stale
   "Latest release: `v2.4.10`" line (v2.4.11 has been published and
   verified since; confirmed by direct inspection — this predates and is
   independent of this release) to the actual current published stable
   (`v2.4.11`), **then** add `v2.5.0-rc.1` as the new release-candidate
   entry distinct from that stable baseline. Preserve the batching
   amendment's later commitments (v2.6.0, v2.7.0, later minors) unchanged
   except for the explicit scope-amendment note from the decision above.
6. Leave the README major/minor citation unchanged only if `2.5` does not
   yet warrant a citation bump per existing convention — confirm against
   the actual current README wording at implementation time rather than
   assuming.
7. **v2.5 milestone/issue-tracker reconciliation — corrects B-RC-TRACKER-7,
   its round-2 narrowing B-RC-TRACKER-R2-3, and its round-3 narrowing
   B-RC-TRACKER-R3-4.** The GitLab `v2.5.0` milestone currently contains
   three open issues (#33, #67, #78). **Correction: the round-2 map used
   only #67's initial issue body and missed its comment history (findings
   7–11), and it proposed relocating #78 finding 1's declared paths
   although the issue author retracted that finding and governor triage
   made the retraction binding.** Below is the corrected per-finding map,
   built by reading each issue's full comment history (not the initial
   body alone) and independently re-verifying every claimed status against
   current `main`, with an exact recommended outcome for the governor to
   approve or amend at Gate 1 — tracker notes execute the approved
   decision afterward, they do not discover it during publication.

   **#33 — concurrency semantics.** The minimal slice #80 required
   already shipped in this candidate (the `--all-kinds` pending surface's
   direct-reference resolution, distinct from `handoff`'s transitive
   resolution). The issue's full concurrency-specification scope is
   unrelated to this candidate's contract and remains genuinely open.
   **Recommended disposition: keep open, remove from the `v2.5.0`
   milestone, leave unmilestoned** (no governed later destination named
   yet) unless the governor names one at Gate 1.

   **#67 — eleven findings across the initial body and three follow-up
   comments, independently re-verified against current `main` before
   writing this map (round-2's map covered only findings 1–6):**
   1. Superseded `agent-governance` repo path (repository-path form, 34
      occurrences, including the Quickstart download link serving
      `v1.1.50`, three schema `$id` values at the old URL/prefix) —
      **resolved**, commit `a9b79b8e`. A later issue comment found the
      original 34-count itself undercounted (two GitLab Pages-host URLs
      in `agf/docs/governance/retrospective.md` and
      `agf/docs/governance/learning-system.md` used a different string
      form and survived the repository-path fix). Verified today: a
      broader `agent-governance` grep (excluding the current
      `agents-govern-framework`/`agents_govern` name) across `agf/` and
      `docs/reviews/` returns zero matches — both the repository-path and
      the Pages-host forms are gone from the shipped tree now.
   2. Adopter-owned manifest at an upstream-owned path — **resolved by
      #79** (this candidate, `MANIFEST.example.yaml`).
   3. `ADOPTING.md` contradicted the site docs on packaged-tooling
      availability — **resolved**, commit `d7dd5da7`. Verified: the
      contradicting text no longer exists in `agf/ADOPTING.md`.
   4. GitLab Releases web page is unreadable to assistants; no API
      pointer existed — **resolved**, commit `d7dd5da7`. Verified: the
      Releases API URL is present in both `agf/ADOPTING.md` and
      `install-and-layouts.md` today.
   5. ADR setup commands copied the template onto itself
      (`docs/adr/_template.md` instead of `agf/docs/adr/_template.md`) —
      **resolved**, commit `d7dd5da7`. Verified: `agf/ADOPTING.md` now
      reads `cp agf/docs/adr/_template.md docs/adr/`.
   6. Informational (what worked without intervention) — no action item.
   7. `requires-ack: true` on `kind: note` created no tracked obligation
      (identical root cause to #78 finding 3) — **resolved by #80** (this
      candidate). Verified: `find_pending_all_kinds` in
      `handoff_tracker.rs` now filters on `requires_ack()` for all
      tracked kinds, not only `handoff`.
   8. Learning IDs and coordination IDs share a timestamp-plus-four-char
      format, so citing a learning ID in a coordination message body
      produces a `body-reference` graph edge to a target that is not a
      coordination file, with no target-existence classification — **not
      resolved, still open.** **Corrects B-RC-TRACKER-EVIDENCE-R4-5**,
      which caught this plan conflating three distinct facts. Verified
      today by running `agov coord graph-index`: of ~612-614
      `body-reference` edges in total (all shapes, including legacy
      numeric and other shorthand references — not evidence of the
      collision on their own), 73 edges (37 unique targets) specifically
      match the `YYYYMMDDThhmmssZ-<four-chars>.md` collision shape, and
      `graph-index` emits no target-existence classification for any of
      them. The issue itself reproduced the defect narrowly — two direct
      learning-ID targets cited in one coordination message body, not a
      73-record scan. The 73 collision-shaped edges are this repository's
      current count, not the issue's own reproduction; both facts support
      keeping the finding open, but must not be stated as one number.
      Recommended: name a successor
      issue for this specific defect (namespace the two ID spaces, or
      have `graph-index` classify unresolved `body-reference` targets
      under a separate key) rather than leaving it as an unlabelled
      remainder of #67.
   9. The shipped loader template documented a rejected positional
      `agov coord graph-index docs/coordination/` invocation — **resolved**.
      Verified: `agf/templates/AGENTS.md` now uses the bare
      `agov coord graph-index` form (lines 109 and 166 today), which
      matches the command's actual accepted syntax.
   10. Malformed coordination files fail visibly (the counter-example
       showing the machinery works when the file is malformed rather than
       semantically dropped) — informational, no action item.
   11. `agov plan validate` rejected `fact-checker` as an unknown advisory
       role, contradicting the schema (any non-empty string), the
       commissioning matrix, and `ADOPTING.md`'s claim that advisory roles
       "do not need YAML definitions or framework tracking" — **partially
       resolved, contract still contradictory.** Verified today:
       `plan_validator.rs` no longer hardcodes an allowlist — it now
       accepts any role active in `agf/agents/MANIFEST.yaml`
       (`role_statuses`/`validate_semantics_with_manifest`), and this
       project's own manifest lists `fact-checker: status: active`, so the
       validator accepts it here. But `agf/ADOPTING.md:311-312` still
       reads unchanged: advisory roles "do not need Yet Another Markup
       Language (YAML) definitions or framework tracking; they are
       project-specific and informal" — directly contradicting a validator
       that now requires exactly that tracking. Recommended: keep this
       finding open pending a governor decision on which artifact is
       authoritative (require manifest tracking and fix `ADOPTING.md`'s
       claim, or accept any project-declared role and loosen the
       validator) — this is a documentation/contract-authority decision,
       not a mechanical fix this release can make unilaterally.

      **Findings 1–7, 9, and 10 are resolved or require no action; 8 and
      11 remain genuinely open.** Recommended disposition: **keep #67
      open, remove it from the `v2.5.0` milestone**, add a GitLab note
      recording all nine resolved/no-action items with exact fixing
      commits/MRs, and leave 8 and 11 as the issue's remaining scope
      pending named successor issues or an explicit governor decision —
      do not close the umbrella on a partial map again.

   **#78 — four findings, independently re-verified against current
   `main` and against the issue's own retraction and governor-triage
   comments (round-2's map treated finding 1 as live and proposed a
   relocation the issue author had already retracted):**
   1. ~~Regression: the archive ships a top-level `docs/` outside
      `agf/`~~ — **retracted by the issue author** (comment
      `3611959113`): `agf/dist/release-manifest.yaml` declares both
      `docs/reviews/...` paths explicitly, `documentation-rule-projection-
      rule.md` requires adopters to write into that same shared tree at
      `docs/reviews/rule-projection/<plan-id>.yaml`, and `agov plan
      projection-check` validates declared-path resolvability against the
      manifest — relocating the files, as this plan's round-2 revision
      proposed, would have broken the declared contract, not fixed a
      regression. Governor triage (`janipaijanen`, MR !749, canonical
      record `20260728T154327Z-governor-reviewer-gkj5.md`) made this
      binding: "retracted; top-level `docs/` is not an accidental
      regression; do not relocate declared projection/review paths."
      **This plan withdraws its round-2 relocation recommendation
      entirely and must not schedule that work anywhere, including a
      follow-on plan.**
   2. Adopter roster overwritten on upgrade — **resolved by #79** (this
      candidate).
   3. `requires-ack` on a `note` created no tracked obligation —
      **resolved by #80** (this candidate).
   4. Coordination records visible only on unmerged branches (the
      learning behind `coordination-publication-rule.md`'s provisional
      status) — **routed by the same governor triage to issue #76's
      acceptance matrix**, not left as open #78 scope. Verified: #76
      ("Optimize `agov coord visibility` for branch-heavy repositories")
      is open in milestone `v2.6.0` today and already carries the
      issue-#78 evidence. The forensic assessment (MR !1010,
      `historical-branch-only-coordination-recovery.md`) is merged; its
      prevention *implementation* is #76's scope, not this release's.

      The same governor triage also routed two items that are not #78
      findings but were raised in the same comment thread: a corrective
      sweep for the two Pages-host superseded-path URLs under approved
      #67 Slice B (verified resolved above, #67 finding 1), and the
      shipped-runbook-test-fixture defect to **issue #81**, which is
      **closed** today.

      **All four #78 findings now have a settled disposition — finding 1
      retracted, finding 4 routed to #76, findings 2–3 resolved.**
      Recommended disposition: **close #78**, with a GitLab note recording
      the finding-1 retraction and its governor-triage citation, findings
      2–3 resolved by #79/#80 with exact commit/MR references, and
      finding 4 routed to #76 (already open, already milestoned
      `v2.6.0`) — and remove it from the `v2.5.0` milestone.

   This step is tracker reconciliation only — it does not authorize
   implementing #33's remaining scope, #67 finding 8's ID-namespace fix,
   #67 finding 11's authority decision, or #76's prevention-mechanism
   scope. A final GitLab API check before tagging must show the milestone
   state and the release notes agree (no open issue silently implies
   unshipped work is included in this candidate, no shipped resolution is
   silently unrecorded, no still-open defect is silently implied fixed).

No product implementation, command semantic, schema, validator semantic,
dependency, CI behavior, new documentation policy, rule, v2.6.0/v2.7.0
scope, or branch deletion is in scope.

## Proposed changelog substance

**Correction (round-2 review, B-RC-SCOPE-R2-1):** the claim that "both
changelog files already contain" all five surfaces was itself false and
independently re-verified before this fix, not just accepted from the
review — direct grep of `agf/CHANGELOG.md`'s `[Unreleased]` section shows
zero mentions of issue #101; root `CHANGELOG.md` covers it fully. The
accurate state:

- `CHANGELOG.md` (repository-level): already contains entries for all
  five surfaces (#79, #80, learning-supersession, governance-layout-rule,
  #101) — verify this at the release-branch head, don't re-draft.
- `agf/CHANGELOG.md` (adopter-facing): already contains entries for four
  surfaces (#79, #80, learning-supersession, governance-layout-rule) —
  verify at the release-branch head. **Issue #101 needs the new entry
  decided in the scope-amendment section above** (either add adopter-facing
  coverage, or record the governor's explicit root-only-coverage
  decision) before this release's `agf/CHANGELOG.md` can be converted to a
  dated section.

The release MR must:

- verify every `[Unreleased]` entry in both files as of the release branch
  head is one of the five in-scope items above (no stray entry from an
  unrelated merge slipped in between drafting and release), and confirm
  the corrected per-file coverage list above rather than assuming parity
  between the two files;
- per **A-RC-MATURITY-1**: describe each shipped feature using accurate
  maturity language — "merged and Level 4 Validated" (per
  `implementation-honesty-rule.md`'s five-level model), not "already
  shipped" or other present-perfect phrasing that implies Level 5
  Deployable status. Each surface reaches Level 5 only after this
  publication path succeeds through post-publication verification below;
- add the SemVer classification statement above near the top of the new
  section, matching the pattern used in prior release changelogs;
- state plainly that this is a **release candidate**, not final — e.g. "RC
  checkpoint; `v2.5.0` final ships the same functional candidate unless a
  correction forces `rc.2`," so a reader does not mistake an RC tag for the
  stable release;
- use the actual test/build counts from the final release branch run, not
  assume unchanged counts from `v2.4.11`.

## Validation before Gate 2

The release MR must provide exact-head evidence for:

- Cargo metadata showing exactly the workspace package at `2.5.0-rc.1` and
  the three internal path requirements at `^2.5.0-rc.1`;
- `cargo test --manifest-path tooling/rust/Cargo.toml --workspace`, full
  pass/fail/ignored counts recorded in changelog and MR evidence;
- strict workspace Clippy with warnings denied;
- `agov validate`, including adopter-docs validation;
- `agov dist validate` and clean tar/zip builds;
- **archive-content verification specific to this release's own features**:
  a fresh **framework archive** (`agf-2.5.0-rc.1.tar.gz`/`.zip`)
  extraction confirms `agf/agents/MANIFEST.example.yaml` ships and
  `agf/agents/MANIFEST.yaml` does not; `agf/schemas/*.json` contains the
  `requires-ack`/`ack` schema constraint and the learning
  `status`/`supersedes` fields; `agf/templates/learning-record.yaml`
  reflects the new fields; all twelve `agf/agents/*.yaml` role definitions
  report `framework_version: "2.5.0-rc.1"`; `agf/rules/` carries the
  corrected `governance-file-layout-rule.md`;
- all shipped archive self-sufficiency tests for both archive formats;
- **corrects B-RC-ARTIFACT-MATRIX-4 — the upgrade/overlay and binary-parity
  checks are two distinct artifacts, tested separately, not conflated into
  one "plain archive extraction":**
  - **(a) Framework archive overlay, three baselines.** A plain
    `agf-2.5.0-rc.1` framework-archive extraction over a scratch adopter
    project seeded from each of `v2.4.5`, `v2.4.6`, and `v2.4.11` (the
    immediately prior release) must leave a divergent adopter
    `agf/agents/MANIFEST.yaml` byte-for-byte unchanged, and must show the
    updated schemas/templates/rules content (the framework archive
    contains no Rust binary — it cannot itself demonstrate a running
    command).
  - **(b) Platform tooling archive install and CLI/MCP parity.** Install
    the separate Linux Rust tooling archive's packaged `agov` and
    `agov-mcp` binaries and confirm, via the actual shipped interfaces —
    **corrects B-RC-ARTIFACT-R2-2, which found the first revision named a
    nonexistent bare "generic validation" action; verified directly via
    `agov validate --help` before writing this fix, not assumed**:
    - CLI `agov coord pending` lists the expected pending records; MCP
      tool `agov_coord_pending` returns equivalent output for the same
      fixture;
    - CLI `agov validate learnings <fixture-path>` accepts a positive
      fixture using `status`/`supersedes` and rejects a malformed one
      (there is no bare `agov validate` validation action — it only lists
      subcommands; `learnings` is the specific one this feature needs);
    - MCP tool `agov_validate` with `check: "learnings"` and the fixture
      path in `paths` returns equivalent accept/reject results for the
      same two fixtures;
    - packaged-binary self-reporting confirms `2.5.0-rc.1`, the release MR
      exact head, and clean source.
- `tooling/rust/scripts/check-live-tooling-surfaces.sh check` and
  `selftest`;
- tag/version guard evidence with `CI_COMMIT_TAG=v2.5.0-rc.1`;
- strict `mkdocs build` and link checks over the current site content;
- a GitLab API check immediately before merge showing the release MR's
  `closes_issues` response does not silently attempt to close #79/#80
  again (both already closed), and does not silently close #33/#67/#78 —
  their disposition is a separate, explicit GitLab-note step (release-only
  change-set item 7 above), never an implicit side effect of the release
  MR merging;
- a final pre-tag check that the GitLab `v2.5.0` milestone state and this
  release's changelog/notes agree, per B-RC-TRACKER-7's resolution
  condition;
- release-path, provenance, artifact-exclusion, SPDX/license, links, and
  `git diff --check` checks; and
- a green exact-head GitLab MR pipeline.

The release MR must target `main`, contain only the governed release-only
change set, retain its source branch, and receive exact-head Gate 2
approval.

## Merge, tag, and publication sequence

1. Merge only the exact Gate 2-approved release MR head and retain its
   source branch. **Corrects A-RC-BRANCH-RETENTION-R3**: the project
   default `remove_source_branch_after_merge: true` deletes the remote
   branch on merge even when the merge command itself supplies no
   deletion flag — this already happened once to a review branch during
   this plan's own round-2 review cycle. Retention is not automatic;
   the merge call must explicitly pass `should_remove_source_branch:
   false` through the merge API (`glab api` / the merge-request "Delete
   source branch" checkbox left unchecked), and the remote branch's
   continued existence must be verified immediately after merge, before
   proceeding to step 2. Prior release MR !888 demonstrates explicit
   `false` preserves the branch under the current project default.
2. Confirm `origin/main` resolves to the resulting merge commit and that no
   `v2.5.0-rc.1` tag exists locally or remotely.
3. Create one annotated `v2.5.0-rc.1` tag on that exact `main` merge commit
   and push it immediately. Never move, delete, or recreate the tag.
4. Require the tag pipeline to pass, including framework archives, Linux
   Rust tooling archives, provenance/checksum validation, OCI publication,
   and GitLab Release creation. **Corrects B-RC-PRERELEASE-5**: the release
   is an **ordinary GitLab Release** (`upcoming_release: false`), matching
   the live `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent exactly (independently
   confirmed via the GitLab Release API: both report
   `upcoming_release: false`) — `.gitlab-ci.yml` invokes `release-cli
   create` with name, tag, description, and assets only, with no
   pre-release flag anywhere in the current release path. The first
   revision's "mark it as a GitLab pre-release" language authorized a
   CI/publication behavior change this plan's own scope and stop
   conditions forbid; that language is withdrawn. If a real supported
   pre-release marker is ever wanted, it requires its own separately
   governed CI-change plan, not an ad hoc addition here.
5. Run `agov dist post-publication-check` against the anonymous GitLab
   Release API and a fresh download directory.
6. Verify the GitLab Release, generic package `agf/v2.5.0-rc.1`, and OCI
   tag `agov:2.5.0-rc.1` correspond to the immutable tag pipeline.
7. From a fresh git clone checked out at the exact immutable
   `v2.5.0-rc.1` tag, run both artifact checks from "Validation before
   Gate 2" above — framework-archive overlay (three baselines) and
   platform-tooling-archive CLI/MCP parity — against the real published
   archive bytes, not just the release-branch working tree.
8. Publish and merge a validated completion coordination record.
9. **Do not close issues #79 or #80 as a result of this plan — both are
   already closed.** Link the release tag/pipeline/Release in a
   coordination note for the historical record, but do not touch GitLab
   issue state.

## rc.2 trigger (explicit, per the SoW's own RC discipline)

Any functional, schema, documentation, migration, test, or artifact-layout
correction discovered after `v2.5.0-rc.1` publishes creates a new
`v2.5.0-rc.2` candidate and repeats the full exact-artifact acceptance
matrix above. **Do not schedule a ceremonial rc.2.** If `rc.1` is clean
through publication verification, `v2.5.0` final ships the same functional
candidate under its own later, separately Gate 1-approved release-publication
plan — that plan's release-only change set is limited to version and
changelog metadata, per `release-via-merge-request-rule.md`.

## Acceptance criteria

1. The scope-amendment decision above is resolved by the governor
   (Option A or B, per surface) before Gate 1 approval.
2. This plan's own Gate 1 metadata is updated and merged to `main` before
   the release branch is cut (per the corrected sequencing above) — never
   deferred into the release-only change set.
3. The release MR uses branch name `release-2.5.0-rc.1`, is release-only,
   and receives exact-head Gate 2 approval.
4. All current version surfaces — including all twelve `agf/agents/*.yaml`
   role definitions — and built binary self-reports agree on
   `2.5.0-rc.1`.
5. The immutable tag points to the exact release-MR merge commit on `main`.
6. The tag pipeline and anonymous downloaded-asset validation pass; the
   GitLab Release is ordinary (`upcoming_release: false`), matching the
   `v2.2.0-rc.1`/`v2.2.0-rc.2` precedent, with no new pre-release CI
   behavior introduced.
7. Fresh archive extraction proves both the `MANIFEST.example.yaml`/
   `MANIFEST.yaml` exclusion and the new schema/rules/command surfaces
   are present in the framework archive; a separate platform-tooling-archive
   install proves the new commands are functional via both CLI and MCP.
8. Framework-archive overlay tests pass from all three named baselines
   (`v2.4.5`, `v2.4.6`, `v2.4.11`).
9. The v2.5.0 GitLab milestone's #33/#67/#78 disposition is recorded via
   GitLab notes per release-only change-set item 7's corrected per-finding
   map, and the milestone state agrees with the release notes before
   tagging: **#78 closed** (with note: finding 1 retracted per the issue
   author and governor triage, findings 2–3 resolved by #79/#80, finding 4
   routed to #76); **#33 and #67 kept open** and removed from the
   `v2.5.0` milestone (#33's full concurrency scope and #67 findings 8/11
   are genuinely unshipped remainders, per the per-finding map).
10. Issues #79 and #80 are not touched by this plan's execution — they
    stay closed, unmodified. #78 closes deliberately, with an explicit
    note citing the finding-1 retraction, the fixing commits/MRs for
    findings 2–3, and the finding-4 routing to #76, never as a silent side
    effect of the release MR's own `closes_issues`. #33 and #67 are noted,
    not closed.
11. v2.6.0/v2.7.0 scope (per the batching amendment) does not enter this
    release.

## Stop conditions

Stop and return to the governor if release preparation requires: a product
or CI behavior change beyond the version bump (including any new
pre-release marker or flag); a dependency/checksum change; a new command
semantic not already shipped in #79/#80/learning-supersession/
governance-layout/#101's own implementation MRs; weakening any existing
check; changing an existing tag; using a release branch name other than
`release-2.5.0-rc.1` without an explicit rule amendment; including
v2.6.0/v2.7.0 scope; deleting a source branch; reopening or re-closing
issues #79/#80; silently closing or silently keeping open any of
#33/#67/#78 without the GitLab-note disposition step matching this plan's
corrected per-finding map; relocating any path declared in
`agf/dist/release-manifest.yaml` (including #78 finding 1's retracted
claim about `docs/reviews/...` — do not revive it); or discovering a
`main` delta surface not accounted for in the scope-reconciliation table
above (in which case: stop, classify it the same way, and return to the
governor rather than silently including or excluding it).

---
id: 20260912T155648Z-v2101-ruw-generic-artifact-validation
status: approved
gate1_approved_by: governor-reviewer
gate1_approved_at: 2026-09-12T16:01:41Z
topic: v2101-ruw-generic-artifact-validation
candidate_release: v2.10.1
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      Self-assessed: this scope adds no credential handling, no
      authentication/authorization surface, and no network access. Every
      new command (`agov artifact validate`, `agov artifact index`)
      operates on local files the caller already has filesystem access
      to, using the same local-read pattern as `agov coord validate`.
      The new `artifact_sets` config block is declarative (paths,
      schema references, JSON Pointers) and does not execute arbitrary
      code or plugins per the customer's own explicit non-goal in their
      material. `agov validate infosec` run at drafting time: OK, no
      findings. Real infosec-sentinel review still requested before
      Gate 2, focused on path-traversal/glob-containment behavior in
      the new `artifact_sets.paths` resolution (an actual new
      filesystem-scope surface, even though it is local-only).
  - role: contribution-auditor
    status: waived
    reason: >-
      Consultation requested at this Gate 1, not yet on record. This is
      new, customer-adjacent capability with real design-boundary risk
      (see "Explicitly out of scope" below) — request an actual
      contribution-auditor pass before governor Gate 1 decision, not a
      pattern-matched waiver.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: v2.10.1 as the redesignated RUW — generic artifact/graph validation extension

**Gate 1: APPROVED, version confirmed as v2.10.1** per
`20260912T160141Z-planner-architect-jj88` ("gate 1 approved, confirm
version 2.10.1", governor decision transcribed by Claude, no
AI-originated authority claimed). The SemVer tension this plan flagged
is resolved: v2.10.1 stands as named, not v2.11.0 or another number.

## Decision requested at Gate 1

Redesignate the active "release under work" (RUW) label from v2.11 to
**v2.10.1**, and approve v2.10.1's scope as a narrow, additive `agov`
extension covering exactly the **low-customer-coupling** subset of the
labdns customer's eight distilled AGF/AGOV enhancement asks
(`docs/proposals/labdns-customer-agf-enhancement-distillation.md`,
asks A1–A8, corrected per Codex review `p5mj`).

This plan does not reopen or cancel v2.11/RUW's existing work. That
work stays exactly where the pause (`20260912T151723Z-planner-architect-31pr`)
left it — paused, claims preserved, no new commits — and resumes only
on a later, separate governor decision. This plan asks for v2.10.1 to
become the actively-worked release in the meantime, following the same
"RUW" pattern the project already used for v2.10.0 (see
`docs/plans/20260907T045703Z-v210-release-plan.md`'s "release-under-work"
framing): one coherent named outcome, not a grab-bag of unrelated fixes.

## Why v2.10.1, not a v2.11 amendment or a new minor

Jani specified v2.10.1 directly. Noting the tension honestly rather than
resolving it silently, per `always/uncertainty-and-scope-rule.md`: this
adds new CLI commands and new config surface, which is additive/backward-
compatible functionality — conventionally a MINOR bump, not a PATCH. The
project has an existing precedent for SemVer-number and downstream-impact
classification diverging for stated reasons (v2.10.0's RUW plan: "SemVer
delta MINOR-shaped... downstream-impact classification remains BREAKING").
Here the divergence runs the other way: a MINOR-shaped change deliberately
released as a PATCH number, at the human governor's explicit direction,
presumably because the customer-facing goal is a small, fast, clearly-
scoped drop rather than a larger release cycle. Recording this as a
deliberate, named choice — not a silent SemVer violation — per
`workflow/semver-no-carry-rule.md`'s sibling discipline of explicit
version-number reasoning. If a MINOR number is preferred on reflection,
that is a one-line change to this plan before Gate 1 closes, not a
blocker to filing it.

## Scope: what ships in v2.10.1

Only the pieces Codex and I converged on as genuinely low-customer-
coupling — generic capability that stands on its own engineering merit
regardless of whether this specific customer ever adopts AGF, and that
does not require AGF to track the customer's own release pace to be
useful.

### In scope

1. **A1 — `artifact_sets` block in `agov-config.yaml`.** A declarative
   registry: name, path globs (contained within the repository root; no
   symlink traversal outside it), a JSON Schema reference, `id_pointers`
   and `reference_pointers` (JSON Pointer expressions), and a
   uniqueness scope. This is new config surface, additive only — it
   does not change `from_aliases` or any existing coordination-channel
   config behavior.
2. **A3 — `agov artifact validate PATH...` and `agov artifact index
   [--format json]`.** Validate configured artifact-set members against
   their registered schema; produce a compact index (`id`, `kind`,
   `key`, `version`, `path`, JSON Pointer, digest, status) analogous in
   spirit to `agov coord graph-index` but built as its own data model —
   per Codex's correction, `graph-index`'s existing coordination-shaped
   JSON is not reused or overloaded for this.
3. **A4, structural subset only.** Within `agov artifact validate` (or
   a thin `agov graph validate` wrapper over the same engine — exact
   CLI shape is an implementation decision within this boundary, not
   fixed by this plan): reference existence (every `reference_pointers`
   target resolves to a registered `id`), acyclicity of any declared
   dependency edges, and ID uniqueness across all configured artifact
   sets. These are pure structural graph properties with no governance-
   authority judgment involved.

### Explicitly out of scope for v2.10.1

- **A2** (generic `agov artifact brief`/mint-and-write command). Per
  Codex's review, the customer can mint their own IDs (e.g. a local
  ULID generator) against A3's validation before AGF needs to own
  minting; A2 is deferred pending the customer's own pilot evidence
  that they actually need AGOV-native minting, not just AGOV-native
  validation.
- **A4, domain/governance-authority subset** — gate ordering
  (P0→L1→L2→...), reviewer-independence enforcement
  (`GOV002`-equivalent), and any rule that would require a validator to
  authenticate human/role authority from a record alone. Per
  `always/tool-authority-boundary-rule.md`, a check may only carry
  blocking authority when a named rule explicitly grants it — no such
  rule is proposed here, and inventing governance-authority semantics
  inside a generic validator is exactly the shape that rule exists to
  prevent.
- **A5** (work-packet generation), **A6** (spec indexing) — customer-
  side tooling per both Codex's and my prior assessment; no AGF
  capability proposed.
- **A7** (cross-branch merge-time ID collision guard for arbitrary
  artifact sets) — needs its own design (merge-base/target/candidate
  identity semantics for artifacts that, unlike coordination records,
  may legitimately be edited/versioned rather than being purely
  append-only). Not attempted in this plan.

## Downstream impact

```
Impact:
  Foundry:          No behavior change to this project's own use of agov;
                     purely additive commands/config this repo does not
                     itself populate. `agov validate infosec`/`adoption`/
                     `contributions`/`agents` all remain unaffected —
                     verified no existing schema or command changes shape.
  Direct adopters:  ADDITIVE. New optional `artifact_sets` config block,
                     new subcommands. Absent config, behavior is
                     unchanged for every existing adopter.
  Downstream:       New capability adopters may use to govern their own
                     project-specific artifact types (work packages,
                     contracts, etc.) without forking agov. Does not
                     change any existing coordination-record shape or
                     validation.
  Migration:        None required — opt-in only.
  Evidence:          This plan; customer material under tmp/customer-1/
                     (not committed) and the committed distillation at
                     docs/proposals/labdns-customer-agf-enhancement-
                     distillation.md.
```

## Gate 2 evidence required

Per `workflow/tdd-rule.md` and this project's own established pattern
for validator work (see the customer's own P0-12 test-strategy
recommendation, which independently arrived at the same shape this
project already uses for `gate_change_check.rs`/`ci_needs_implication.rs`):

1. One minimal valid fixture per new schema/config shape.
2. One negative fixture per structural rule (duplicate ID, dangling
   reference, cycle, malformed `artifact_sets` entry).
3. Path-containment tests: glob patterns that would escape the
   repository root, or a symlink pointing outside it, must fail closed
   — this is the one new filesystem-scope surface infosec-sentinel
   should specifically examine before Gate 2.
4. Deterministic-output test: repeated runs on unchanged input produce
   byte-identical JSON (`--format json`), matching the customer's own
   stated expectation and this project's existing convention for
   `agov coord publication-check`-style commands.
5. `cargo test`, `cargo clippy --all-targets` (warnings denied), and
   `agov plan validate`/`agov coord validate` continue to pass
   unmodified — proving no regression to existing coordination/plan
   validation.
6. No `.gitlab-ci.yml`, schema, or agent-manifest change beyond what
   this plan names, unless a follow-up amendment records it.

## Role and gate boundary

planner-architect (this session) drafts and does not implement.
build-engineer (this session) reviews CI-mechanics and build
correctness once implementation lands. coder-tester (Codex) implements
after Gate 1 approval, per Jani's direct instruction. Gate 1 and Gate 2
approval remain governor-reviewer's sole authority. No release, tag, or
deploy authority is requested or implied by this plan — cutting the
actual v2.10.1 release is a separate decision after this scope merges
and passes its own acceptance evidence, following
`workflow/release-via-merge-request-rule.md` in full (version-bump
branch, MR, review, tag only after merge).

## Risks

- **Scope creep back toward the full A1-A8 ask.** Mitigated by the
  explicit out-of-scope list above; any of A2/A4-domain/A5/A6/A7 entering
  this release requires a plan amendment, not an implementation-time
  judgment call.
- **Path/glob containment bugs in a new filesystem-facing surface.**
  Mitigated by the specific Gate 2 evidence item above and a real
  (not waived) infosec-sentinel pass before Gate 2.
- **Customer expectation mismatch** if this ships without also
  correcting the record that `agov` does not yet have a generic
  artifact index — already handled in the corrected distillation
  document, not a new risk introduced by this plan.

## Rollback

New commands and config block only; no schema migration, no change to
existing coordination or plan validation behavior. Revert the merge
commit if a regression appears; no data migration is needed since
`artifact_sets` is opt-in and unpopulated by this project itself.

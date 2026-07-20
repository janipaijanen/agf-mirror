---
id: issue-39-removability-exit-guarantee
status: pending-gate1
topic: removability-design-principle-and-ci-drill
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Gate-1-planning-only session; no dedicated contribution-auditor review
      round has run against this plan yet. Required by validator bug
      workaround (issue #30 — build-engineer is currently rejected by
      `agov plan validate` despite the schema allowing any non-empty role
      string). Flagging for actual review at Gate 1 approval time given this
      plan constrains what five other in-flight clusters may build.
  - role: infosec-sentinel
    status: waived
    reason: >-
      Gate-1-planning-only session; no dedicated infosec-sentinel review
      round has run yet. The removal drill (Layer 3) adds a new CI job that
      deletes files and runs an external fixture's build/test step —
      security-adjacent even though it targets a disposable fixture, not
      production infrastructure. Flagging for actual review at Gate 1
      approval before this plan is implemented.
---

# Plan: Issue #39 — Codify removability: design principle, downstream exit guarantee, and CI removal drill

**Date:** 2026-07-20T07:53:58Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** none (this plan is itself a standing constraint on six other
in-flight cluster plans being written concurrently in separate worktrees —
see "Cross-cluster constraints" below)

---

## Intent

Elevate AGF's low-lock-in architecture from an implicit property to three
concrete, checkable artifacts:

1. **A stated design principle** — a canonical sentence, in a canonical
   location, that AGF components are conveniences over plain files, never
   their owners.
2. **A downstream-facing exit guarantee** — a short, testable promise in
   README that removal costs a directory deletion and a CI job, and that any
   change eroding this is treated as breaking regardless of version number.
3. **A CI-run removal drill** — a scripted test, run at minimum per release,
   that actually deletes AGF from a disposable sample repository and asserts
   the two things the guarantee promises.

This plan does **not** invent the underlying property — the property already
exists (files under version control, no runtime, no hosted service) and is
already documented at length in `agf/docs/site/docs/adopt/exit-and-decommission.md`
and `agf/docs/site/docs/adopt/downstream-impact.md` (both "Released (v2.1
documentation)"). The gap this plan closes is that the property is currently
**asserted in prose only** — nothing runs it. Per the implementation-honesty
rule and this repo's own maturity vocabulary
(`agf/docs/site/reference/maturity-vocabulary.md`), an untested promise is
**Theoretical** evidence at best; the CI drill is what a reviewer can point to
as the artifact justifying a stronger claim.

---

## What already exists — read first, do not duplicate

Confirmed by direct inspection of this worktree before drafting this plan:

- **`docs/append-only-audit-trail-policy` branch (unmerged as of this
  session — confirmed via `git log --all --oneline | grep -i
  "append-only\|ledger"`, one commit `49dc956`)** adds a "Git history is a
  ledger, not a scratchpad" section to README (just before "Safety and
  independence") and an "Audit trail policy: append-only, not erasable"
  section to `agf/docs/site/contribute.md`. That section covers a
  **different claim**: history/coordination messages are never rewritten,
  and the GDPR-erasure-vs-append-only tension. It does **not** cover
  removability/exit — it is about *not deleting* history, this plan is about
  *being able to delete the framework*. The two claims are compatible and
  adjacent but distinct; this plan's README addition links to the ledger
  section rather than restating any of it, and is written to slot in
  immediately after it (or immediately before "Safety and independence" if
  that branch has still not merged by implementation time — the exact
  anchor point is resolved at implementation time by checking merge status
  again, per the destructive-op-preflight discipline of verifying current
  state before acting).
- **`agf/docs/site/docs/adopt/exit-and-decommission.md`** (Released, v2.1) —
  already states "Adoption is reversible by design... it has no runtime, no
  service registration, and no data it holds hostage," and gives the
  three exit modes (pause/downgrade/remove) plus the "what you keep" list
  and the manual removal steps. **This is the canonical mechanics page.**
  This plan does not rewrite it; it adds one small cross-link to the new
  design-principle page and, in a later implementation session, a note that
  the removal drill (once shipped) is what makes this page's claims
  CI-verified rather than prose-only.
- **`agf/docs/site/docs/adopt/downstream-impact.md`** (Released, v2.1) —
  itemises what adoption *costs*; already links to exit-and-decommission.md
  for the way back out. No change needed beyond the same kind of small
  cross-link, if any.
- **`agf/docs/adopt/decommission.md`** (a second, older, more detailed
  removal-steps page, apparently pre-dating the v2.1 docs-site restructure)
  — covers the same ground as `exit-and-decommission.md` in more procedural
  detail (validators to run before removal, exact file table by legacy
  Layout A/B, hooks, CI greps to verify a clean removal). **Not touched by
  this plan.** Flagging as a possible duplicate-canonical-location issue for
  a separate cleanup (out of scope here — this plan does not resolve which
  of the two decommission pages is authoritative; it only adds the drill
  and cross-links the same way to both if both remain).
- **`agf/docs/governance/disclaimer.md`** — "Independence and third-party
  brands" section and general responsibility language; unrelated to
  removability and untouched by this plan.
- **No existing ADR** touches lock-in or removability (`agf/docs/adr/`
  contains five ADRs plus template/README; none mention "lock-in" or
  "removab" per direct grep).
- **No existing "Removability" or "Principle 10"-shaped entry** in
  `agf/framework.md`'s Boundary Principles section (Principles 1-9:
  Verb Test, Artifact Ownership, Immutability Constraint, Pipeline
  Topology, Escalation over Improvisation, Lens Principle, Technology
  Neutrality, User-Origin Planning, Single Decision Point). `framework.md`
  itself carries an RC-readiness note that it is legacy/`adapted-from-v1x`
  and that the public docs site is the current v2 operator surface — this
  informs where the plan puts the *canonical* principle text (see below).
- **No existing removal-drill script or CI job.** `.gitlab-ci.yml` has no
  job that deletes files and re-runs anything; confirmed by reading the
  full file (558 lines, all jobs enumerated below in Scope).
- **`agf/docs/contracts/contract-stability.md`** is the canonical
  semver/contract-stability page (referenced by the Integration Contract).
  It has no removability-specific clause yet.

---

## Scope

### 1. Design principle — canonical location + one pointer

| File | Action |
|---|---|
| `agf/docs/site/docs/concepts/removability.md` | **Create.** Canonical statement of the design principle, in the same style as sibling pages `concepts/roles-and-gates.md`, `concepts/position.md`, `concepts/coordination-protocol.md`, `concepts/evidence-and-learnings.md`. Contains: the principle sentence from the issue text; what "conveniences over files, never their owners" means concretely for `agov`, `agov-mcp` (not yet built), enforcement CI, and any future gateway; an explicit maturity statement (Evidence Status: Theoretical until the removal drill exists and passes at least once in this project's own CI — see Honesty section below); a "Constrains" subsection naming the sender-identity-must-be-git-native constraint on issue #35 (see Cross-cluster constraints below); links to `exit-and-decommission.md` (mechanics), `downstream-impact.md` (cost), and the new removal-drill CI job once implemented. |
| `agf/framework.md` | **Edit — additive, minimal.** Add "Principle 10: Removability" to the Boundary Principles list (section 3), 2-3 sentences plus a link to the canonical page above. Does not restate the full principle text there — per the marketing-tone-rule's message-integrity clause, the numbered principle list stays complete (so anyone citing "Principle N" by number still finds an entry) without creating a second full-text copy to drift out of sync. |

### 2. Exit guarantee — README, downstream-facing

| File | Action |
|---|---|
| `README.md` | **Edit — new section**, "Exit guarantee," placed adjacent to (after, if merged; otherwise positioned identically to where it will land once merged) the append-only-audit-trail-policy branch's "Git history is a ledger, not a scratchpad" section, and before "Safety and independence." Content: the guarantee sentence from the issue text, the breaking-change clause, one link to `concepts/removability.md` (the principle) and one to `adopt/exit-and-decommission.md` (the mechanics + three exit modes), and — once the drill exists — one link to the removal-drill CI job as the verification evidence. Per documentation-audience-rule, README nominally targets developers, but this exact README already carries adopter-evaluation content ("Should you adopt?", "Twenty-minute path", "What a trial adds to your project") — this section follows that existing, established precedent rather than introducing a new audience mix. |

### 3. Removal drill — CI-run, scripted

| File | Action |
|---|---|
| `tooling/removal-drill/fixture/` | **Create.** A minimal, disposable "sample governed repository": a trivial buildable/testable unit with **zero dependency on AGF, `agov`, or any Rust/Elixir toolchain the framework itself uses** (a single-file script with its own stdlib-only test, e.g. a `test.sh` that asserts a computed value — deliberately not a Rust crate, so the drill cannot accidentally pass only because the *framework's own* toolchain happens to still be on PATH), plus a trimmed copy of governed structure: a couple of files under `agf/` (subset, not the full distribution — a rule file, a schema file, an agent yaml — enough to be a stand-in, not a maintained duplicate of the real distribution), a `docs/plans/sample-plan.md` and `docs/coordination/00001-sample.md` with real frontmatter, and a README describing the fixture's own build/test command explicitly. |
| `tooling/removal-drill/run.sh` | **Create.** Copies the fixture to a temp directory (never runs destructively against a committed path), records the list of `docs/plans/*.md` and `docs/coordination/*.md` files and a checksum of their content, deletes `agf/` and anything under the fixture that represents "agov tooling" (per the fixture's own manifest of what adoption added — mirrors the real `exit-and-decommission.md` steps 1-2), then: (a) runs the fixture's **own** test command (`./test.sh`, not any AGF-provided command) and asserts exit 0; (b) re-reads every previously-recorded `docs/plans/*.md` / `docs/coordination/*.md` file, asserts it is still present, still valid UTF-8, and its content checksum is unchanged (nothing was silently corrupted or truncated by the deletion step) — see "Precise definition of what the drill proves" below for why this is checksum-equality-of-content, not schema-validity. |
| `.gitlab-ci.yml` | **Edit — new job** `removability:drill`, following the existing `.rust-base`/`.mkdocs-base` template-and-`rules:` shape already used by every other job in this file. Path-scoped per `ci-trigger-scoping-rule` to fire on changes under `tooling/removal-drill/**`, `agf/**`, and `.gitlab-ci.yml` itself (a change to any of those three could break the drill or the guarantee it tests) on MRs and default-branch pushes, and **unconditionally on `$CI_COMMIT_TAG`** (release pipelines) per the acceptance criterion "run at minimum per release." Exact job body is an implementation-session task, not decided in this plan beyond: shell-only, no Rust build step needed (the fixture is deliberately toolchain-independent), short runtime (well under the 5-minute quick-feedback threshold, so no special scoping exemption is needed beyond the ordinary path scoping already applied to every other job in this file). |

### 4. Versioning-policy cross-reference

| File | Action |
|---|---|
| `agf/docs/contracts/contract-stability.md` | **Edit — new short subsection**, "Removability guarantee: never a normal contract item." States that any change eroding the exit guarantee (adding durable state that is not reconstructible from the repository alone, adding a runtime dependency for removal, etc.) is treated as **breaking regardless of the semver bump chosen for that release** — distinct from and stricter than the existing patch/minor/major table, which governs the machine-checkable validator contract. Links to `concepts/removability.md`. This satisfies the acceptance criterion "semver/versioning policy references the guarantee" without conflating the removability guarantee with the unrelated validator-contract stability promise the rest of that page covers. |

### Out of scope (explicit, per the issue text)

- Enforcing removability on third-party components adopters attach
  themselves (their own CI plugins, their own tooling) — not AGF's to
  guarantee.
- Retroactive audits of past AGF versions for removability compliance.
- Resolving which of `agf/docs/adopt/decommission.md` vs.
  `agf/docs/site/docs/adopt/exit-and-decommission.md` is the single
  canonical decommission page — flagged above as a pre-existing duplication
  this plan noticed but does not fix.
- Building `agov-mcp` or any future gateway — the principle constrains their
  future design; it does not require building them now.
- Implementing issue #35 (sender identity) — see Cross-cluster constraints.

---

## Honesty about what the drill proves (implementation-honesty-rule)

Per this repo's own maturity vocabulary
(`agf/docs/site/reference/maturity-vocabulary.md`), the removability
**principle** and the removal **drill** sit on two different ladders and
must not be conflated:

- **Evidence Status of the principle itself:** **Theoretical** the moment
  the prose ships (it is a commitment, not an empirical claim about
  external adopters). It becomes **Empirically informed** once this
  project's own CI has run the drill successfully at least once (one
  sustained self-application data point — the framework governing itself,
  per this repo's own "the framework that cannot govern itself has no
  business governing anything else" opening line). It does **not** reach
  **Empirically validated** without independent corroboration from at least
  one other adopting project's own removal experience — that threshold is
  out of this plan's control and must not be claimed prematurely in README
  or the concept page.
- **Capability Maturity of the drill as a CI artifact:**
  - Level 1 **Planned** — this plan, today.
  - Level 2 **Surface-defined** — a job entry exists in `.gitlab-ci.yml`
    that references a script, before the script does anything real.
  - Level 3 **Instantiated** — the fixture and script exist and run
    manually, before they are wired into CI or before both assertions
    (build/test survives; markdown survives unchanged) are implemented.
  - Level 4 **Validated** — the drill runs and passes in a feature branch's
    pipeline, not yet on `main` or a release tag.
  - Level 5 **Deployable** — the drill is a required job on the release-tag
    pipeline (`$CI_COMMIT_TAG`) and has passed there at least once.
- **README and the concept page must not claim Level 5 / Empirically
  informed until the drill has actually shipped and gone green on a real
  pipeline.** This plan (Gate 1, planning only) explicitly does not
  implement the drill — so at the moment this plan is written, the correct
  claim if anyone asked "is the exit guarantee tested?" is **"no — Level 1
  Planned; a CI drill is designed but not yet built."** The implementation
  session that follows Gate 1 approval is what moves the maturity level,
  and the README/concept-page wording must be written to match whatever
  level is true *at merge time*, not aspirationally ahead of it.

## Precise definition: what "the project still builds/tests" and "loss of
working capability" mean (per the task's explicit request to make this
assertable)

Two assertions, stated precisely enough for the drill script to check them
mechanically, with no interpretive slack:

1. **"The project still builds/tests" = the fixture's own, independently
   defined build/test command exits 0, where "independently defined" means:
   the command is declared inside the fixture (e.g., a `test.sh` committed
   under `tooling/removal-drill/fixture/`), it does not invoke `agov`, does
   not source anything under the deleted `agf/` path, and does not depend
   on any AGF rule file being present.** This is deliberately the
   *adopter's own* test suite, not a check that some AGF-provided validator
   still runs — checking that would be circular (the validator was just
   deleted). The fixture is written to fail this assertion if it
   accidentally depends on anything AGF-provided, so a future edit that
   introduces such a dependency by accident is caught by the drill itself
   going red, not silently.
2. **"Historical channel and handoff files remain readable as plain
   Markdown without any AGF tooling" = every file under the fixture's
   `docs/coordination/*.md` and `docs/plans/*.md`, present before deletion,
   is still present, still valid UTF-8, and byte-identical (checksum match)
   after deletion.** This is explicitly **not** "still schema-valid
   according to `agov coord validate` / `agov plan validate`" — those
   validators are themselves deleted by the drill's own removal step, so
   asserting continued *validator-passing* would be self-contradictory.
   The guarantee is narrower and honest about it: the files are not lost,
   not corrupted, not silently rewritten, and remain human-readable text —
   exactly the "readable Markdown in your git log, owned by you" language
   in the issue's own exit-guarantee wording. Re-validating them requires
   re-adopting AGF (or any other tool that understands the same schema);
   the guarantee is about *survival*, not about *continued machine
   validation without the tool*.

---

## Cross-cluster constraints (for the human reconciler)

This issue's design principle is a standing constraint six other clusters
were told to treat as binding on their own concurrent plans. Two concrete
checkpoints for whoever reconciles all seven plans:

1. **Issue #35 (sender identity / accountability cluster) must stay
   git-native.** This plan's Scope section 1 adds an explicit "Constrains"
   subsection to `concepts/removability.md` naming this requirement:
   any sender-identity mechanism must be derivable from git commit
   metadata and/or literal fields inside the coordination/plan Markdown
   files themselves (author trailers, a `from:` field in frontmatter,
   commit author identity) — **never** from an external identity provider,
   directory service, or database lookup whose removal would strand
   attribution or make historical messages unattributable after AGF (and
   whatever accompanying service #35 might otherwise have proposed) is
   removed. **Concrete check for the reconciler:** the removal drill's
   second assertion (checksum-identity of `docs/coordination/*.md` content
   pre/post deletion) is a structural test of exactly this property applied
   to whatever #35 actually ships — if #35's design requires a live lookup
   to resolve *who sent this message* (as opposed to the identity being an
   inline literal already in the file), the file survives the drill
   byte-for-byte but the *information it encodes* becomes unresolvable the
   moment the lookup service is gone. That failure mode is invisible to the
   drill as specified in this plan (the drill checks file survival, not
   semantic resolvability of every field) — **flagging this gap
   explicitly** rather than silently assuming the drill catches it. If #35
   introduces any non-git-native identity mechanism, the reconciler should
   treat that as a removability violation regardless of what this plan's
   drill does or does not mechanically catch.
2. **Issue #31 (improvement issue template's "Lock-in impact" field)** is
   the per-change review mechanism referenced, not re-specified, by this
   plan. This plan does not touch the issue template. Whoever implements
   #31 should link the template's "Lock-in impact" field description to
   `concepts/removability.md` once it exists, rather than re-explaining the
   principle inline in the template (message-integrity rule).

---

## Migration needed

No database or schema migration. All changes are documentation additions,
one new CI job, and one new disposable test fixture directory. No existing
file's meaning changes; `agf/framework.md`'s Boundary Principles list gains
one new numbered entry (additive).

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| CI (new) | Removal drill — deletes `agf/` + agov references from the disposable fixture, asserts fixture's own test exits 0 and coordination/plan files survive unchanged | `.gitlab-ci.yml` job `removability:drill`, script `tooling/removal-drill/run.sh` |
| Negative test (gate-self-test-rule) | Per `always/gate-self-test-rule.md`, the same MR that introduces the drill must include a demonstration that the drill actually fires on a known-bad input — e.g., temporarily reintroduce a dependency on `agov` inside the fixture's `test.sh` and show the drill job fails with a clear message, then revert. Documented in the MR description with the failure transcript. | Implementation-session MR description |
| Manual | `agov plan validate` against this plan file itself | This session, before commit (see below) |

---

## Risks

| Risk | Mitigation |
|---|---|
| Drill fixture silently depends on the framework's own Rust/Elixir toolchain via ambient PATH, giving a false-positive "still builds independently" result | Fixture is a single shell script with no compiled-language dependency; CI job for the drill runs in a minimal shell image, not `.rust-base`, so no Rust toolchain is even present to accidentally leak in |
| README exit-guarantee section duplicates or contradicts the (currently unmerged) append-only-ledger section | Implementation session re-checks merge status of `docs/append-only-audit-trail-policy` before editing README (per destructive-op-preflight discipline of verifying current state); links to that section rather than restating any append-only/history claim |
| Two decommission pages (`agf/docs/adopt/decommission.md` and `agf/docs/site/docs/adopt/exit-and-decommission.md`) drift further apart as this plan cross-links to only one | Cross-link added to both from the new concept page; the pre-existing duplication itself is flagged as a separate follow-up, not silently deepened |
| CI job cost creep on every MR | Path-scoped per `ci-trigger-scoping-rule`; shell-only fixture keeps runtime well under the 5-minute threshold |
| Principle text overclaims maturity before the drill ships | Explicit Evidence Status / Capability Maturity levels stated in this plan (see Honesty section) constrain the wording the implementation session is allowed to ship |

---

## Rollback

All changes are additive documentation plus one new, isolated CI job and
fixture directory with no production code path. Rollback is reverting the
implementation commit(s); no data migration, no schema change, and no
existing validator behavior is altered, so revert carries no cleanup
burden beyond the normal git revert.

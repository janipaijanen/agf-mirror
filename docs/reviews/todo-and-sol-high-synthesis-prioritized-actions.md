# TODO.md × Sol High assessment — synthesis and prioritized action list

**Status:** proposed for Gate 2 critique (not yet approved)
**Author:** coder-tester (claude-sonnet-5)
**Date:** 2026-07-20
**Sources:**
- `docs/TODO.md` (three open project TODOs: v1.x verification audit, Rust
  implementation health, token ROI zero-savings)
- `docs/reviews/agf-framework-assessment-codex-sol-high.md` (external
  assessment by `gpt-5.6-sol` at high reasoning effort, committed at
  `3a04e71` on branch `docs/codex-sol-high-framework-assessment`, currently
  unmerged and without an open MR)

## Purpose

Both documents independently name overlapping work. This synthesizes them
into one prioritized list so the next several work sessions have a single
ordered backlog instead of two uncoordinated ones. Priority is based on:
(1) whether the item is a concrete, corroborated defect vs. a strategic
hypothesis; (2) blast radius if left unaddressed; (3) whether it touches
work already in flight (the pending `v2.3.0` release); (4) whether the item
is agent-executable or requires a human product/business decision.

This is a proposal, not a decision. It is explicitly routed to Codex
(coder-tester, `gpt-5.6-sol-high`, author of the source assessment) for
critical review before either of us starts executing it — see the
coordination handoff filed alongside this document.

---

## P0 — immediate, before or alongside the pending v2.3.0 release

### P0-1. Fix the MCP `dir` path-contract bug

**What:** `agov-mcp`'s `agov_session_resume` forwards a `dir` field straight
to CLI `--dir`. The CLI's `--dir` means "coordination-message directory,"
not "repository root." A caller who reasonably passes the repo root gets a
plausible `ok: true` response scanning the wrong file set instead of an
error. Rename the field to `coordination_dir` (or keep `dir` as a
deprecated alias), default it to `docs/coordination`, and echo the resolved
path back in the response so a cold-starting agent can see what it actually
queried.

**Basis:** Sol High finding #6 (risk) and Proposal 1. This is a concrete,
file-and-line-cited defect (`tooling/rust/crates/agov-mcp/src/main.rs`), not
a strategic hypothesis — it is corroborated by this session's own firsthand
MCP setup experience, where the CLI/MCP semantic distinction between
"repository root" and "coordination directory" was easy to get wrong.
Silent wrong-scope success is the worst failure shape for a cold-start
orientation tool: no error, no warning, just an incomplete view presented as
complete.

**Why now, not later:** Low effort, contained blast radius (one crate, one
field), and it directly affects the reliability of every future MCP-based
session-resume — including sessions on this repo starting today.

### P0-2. Sequence review of the Sol High assessment itself — RESOLVED

**What it was:** `docs/coordination/20260720T192551Z-coder-tester-rp33.md`
was a pending Gate 2 handoff to `governor-reviewer`, and its branch
(`docs/codex-sol-high-framework-assessment`) had no open MR at the time
this synthesis was first drafted.

**Resolution:** The human governor merged that branch directly (commit
`b967b50`, "Merge branch 'docs/codex-sol-high-framework-assessment' into
'main'") while this synthesis was being written. `docs/reviews/
agf-framework-assessment-codex-sol-high.md` is now on `main`. No further
action needed here; left in place as a record that the dependency was named
and then closed, per `always/coordination-sequence-guard-rule`.

---

## P1 — near-term, before the next positioning or doc push

### P1-1. Narrow the framework's positioning language

**What:** Adopt "a cross-vendor, Git-native assurance protocol for
consequential AI-authored software changes" (or a close variant) as the
canonical positioning line, and audit `README.md`, `agf/docs/site/index.md`,
and other external-facing copy for language implying "complete enterprise
agent governance" or unqualified superiority over adjacent tools (GitHub,
GitLab, OPA, in-toto, NIST AI RMF).

**Basis:** Sol High's own "Overall judgment" section and Proposal 8; also
the currently-open question in this session (the user shared this exact
quote and asked whether/where to work it in). `always/verifiable-claims-rule`
and `docs/marketing-tone-rule` both already require hedged, sourced,
non-superlative claims — this is an application of existing rules, not a
new obligation.

**Why P1 not P0:** Editorial, low-risk, but not urgent enough to block the
pending release. Natural to batch with the CHANGELOG entry for v2.3.0 since
that already touches release-facing copy.

### P1-2. Merge the two "old evidence needs a disposition" proposals into one plan

**What:** `docs/TODO.md`'s v1.x verification-audit proposal
(`verified-agf-v2` / `inherited-v1x-unverified` / `adapted-from-v1x` /
`retired-for-agf-v2`) and Sol High's finding #5 + Proposal 4 (329 historical
coordination records and 11/76 plans rejected by current schemas, with a
request for `current-valid` / `historical-valid` / `legacy/unversioned` /
`invalid` classification) are the same underlying gap seen from two angles:
neither the *origin* (v1.x vs v2) nor the *schema-version validity* of
inherited/historical evidence is currently machine-legible. Write one Gate 1
plan that produces a single disposition scheme covering both axes, rather
than two separate half-solutions.

**Basis:** Both sources independently reached the same conclusion without
seeing each other — that convergence is itself evidence the gap is real, not
one reviewer's idiosyncratic concern. `workflow/concept-modelling-rule`
applies directly here: origin (v1.x-vs-v2) and schema-validity
(current-vs-legacy) are two distinct axes that a naive single-field
disposition would conflate into a composite attribute — the plan should
model them as two fields, not one.

**Why P1 not P0:** No immediate defect being masked; this is a data-quality
and reporting-honesty gap, not a correctness bug. But it should land before
the framework's health/maturity claims are cited externally again (e.g. in
any future assessment or pilot).

---

## P2 — structural, sequenced after P0/P1 land

### P2-1. CLI/MCP lifecycle and misuse conformance fixtures

**What:** Add fixtures covering omitted/correct/repository-root/absolute/
nonexistent/alternate-layout `dir` values, plus an MCP-only claim/handoff
lifecycle test comparing MCP-observed state against CLI-observed state
directly.

**Basis:** Sol High Proposal 2. Deliberately sequenced after P0-1 — writing
conformance fixtures against a contract that's about to change is wasted
work; fix the field semantics first, then lock them down with tests.

### P2-2. Rust implementation health assessment

**What:** Execute `docs/TODO.md`'s Rust health assessment (module
boundaries, parsing/normalization duplication across coordination/plan/
learning/graph-index code, test architecture for stable JSON output,
whether version provenance needs centralizing, where machine-readable
command contracts should live).

**Basis:** TODO.md's own scope, now corroborated by an independent
reviewer: Sol High finding #8 independently flags `agov`'s CLI entry point
at ~4,200 lines, "thin re-export" module boundaries, and duplicated CLI/MCP
argument contracts — without having read `docs/TODO.md`. Two independent
sources naming the same concentration risk raises this from "worth
watching" to "worth scoping now," per the same convergence logic as P1-2.

**Why P2 not P0/P1:** This is a structural assessment producing a plan, not
a merge-blocking fix. It should happen once the MCP contract (P0-1) and its
conformance fixtures (P2-1) are settled, since those changes are exactly the
kind of contract that a "where should command contracts live" assessment
needs to account for.

---

## P3 — decision items for the human governor, not agent-executable tasks

### P3-1. Minimal starter profile as the primary first-adoption product

**What:** Sol High Proposal 5 — ship a reduced default (three roles, two
gates, four message kinds, one validator, one CI example) as the primary
onboarding path, with the full 46-rule/12-role/7-skill system as opt-in
packs.

**Basis:** Sol High finding #4 (onboarding complexity risk). This is a
real, well-argued concern, but restructuring the adoption product surface is
a product decision with real trade-offs (what counts as "core," whether a
reduced profile undersells the framework's actual differentiator) that
needs the human governor's judgment, not an agent's.

**Why P3, not skipped:** Naming it here prevents it from being silently
dropped just because it's not agent-executable. It belongs on the roadmap
as a decision item, not folded into an engineering backlog.

### P3-2. External pilots and product-validation thresholds

**What:** Sol High Proposals 6 and 7 — run ≥5 independent pilots covering
≥50 governed changes, and set explicit thresholds (10 repos, 250 changes, 3
paying engagements, ≥60% continuation, 2 case studies, 1 security-sensitive
adopter) before making scaled-product or enterprise claims.

**Basis:** This is the assessment's central evidence-gap finding — "organizational
benefit remains largely dogfooding evidence." No engineering task fixes
this; it requires the human governor to decide whether and how to recruit
external pilots.

**Why P3:** Business-development work, explicitly out of scope for an
agent to schedule or execute unilaterally.

### P3-3. Token ROI zero-savings investigation

**What:** `docs/TODO.md`'s third item — determine whether `coord-status`,
`coord-brief`, `coord-stale` etc. reporting `0 saved` tokens is by-design
(no configured baseline) or a silent reporting gap.

**Basis:** TODO.md's own scope. No corroboration from the Sol High
assessment (it wasn't in scope for that review), no reported user-facing
harm yet — the number is either accurate-by-design or under-reporting, not
overclaiming.

**Why P3 not P2:** Contained, low blast radius, and no external claim
currently depends on this number being exactly right. Worth doing, not
worth doing before the P0–P2 items above.

---

## Explicit tension flagged for review

The user has an outstanding, explicit instruction: *"when ready, then bake
a new release, bump minor version."* P0-2's discovery (the Sol High
assessment sits unreviewed) and P1-1/P2-2 all touch release-adjacent
concerns, but none of them should be read as a reason to delay the already-
authorized v2.3.0 release. Recommendation: cut v2.3.0 using the existing
manual release-checklist (`release-via-merge-request-rule`), and treat Sol
High's Proposal 3 (mechanically enforcing release truth: VERSION ↔ README ↔
changelog ↔ archive ↔ public release API all describing the same artifact)
as a P1/P2 follow-up that hardens the *next* release, not a blocker for this
one. Flagging this explicitly rather than silently deciding it either way.

## What this document does not do

It does not re-litigate the Sol High assessment's factual claims (that's
Gate 2 review of `rp33`, per P0-2). It does not commit to timelines. It is a
priority ordering and a rationale, offered for critique.

---
artifacts_owned: []
from: contribution-auditor
id: 20260726T122520Z-contribution-auditor-nr4g
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260726T122208Z-planner-architect-r47q.md
- 20260726T121131Z-build-engineer-njsk.md
requires-ack: true
timestamp: 2026-07-26T12:25:20Z
to: governor-reviewer
topic: ai-tool-guidance-coherence-audit
---

# Contribution audit: AI lifecycle and agov transport guidance plan

Reviewed `docs/plans/ai-tool-guidance-coherence.md` at exact plan commit
`d0589aa450b5af4c4867522a21f54ab8775415e1`, per `r47q`'s request: lifecycle
coherence, MCP/CLI parity, enforceability, audience separation, downstream
compatibility, security/identity overclaiming. Independently re-verified
every factual claim in the plan's "Trigger and evidence" section against
current `main` rather than taking them on trust.

## Verdict: CLEAR WITH ADVISORIES

No BLOCK. Two advisories on implementation sequencing and cross-referencing
already-landed work; both foldable into the existing plan without changing
its design.

## Evidence re-verification (all claims independently confirmed)

- **AGENTS.md vs real-situations page contradiction**: confirmed exact. The
  live prompt template at
  `agf/docs/site/docs/operate/run-agf-in-real-situations.md:33` reads "Use
  Rust `agov` commands only" — directly contradicting root `AGENTS.md`'s
  MCP-when-configured preference.
- **Adopter template gaps**: confirmed. `grep` of
  `agf/templates/AGENTS.md` finds zero MCP/agov-mcp mentions, zero
  `coord post`/`coord list`/`handoff take`/`handoff complete` mentions
  (only `coord brief` at lines 87/143), and line 37 does require an
  unconditional post-resume status note exactly as described.
- **MCP page lacks a phase matrix**: confirmed. `agf/docs/site/docs/adopt/
  mcp-server.md` (117 lines) has an "Operating pattern" section but no
  cold-start/Gate-1/implementation/poll/Gate-2/completion breakdown.
- **`.claude/rules/README.md` obsolete copy-the-rules advice**: confirmed.
  Live text: "Adopters using other tools should copy the rule content into
  their tool's equivalent directory... The reference implementation targets
  Claude Code (`.claude/rules/`)" — contradicts the canonical-`agf/rules`
  loader-pointer pattern used elsewhere.
- **No `.claude/CLAUDE.md` on `main`**: confirmed absent
  (`find . -maxdepth 2 -iname CLAUDE.md` finds nothing under `.claude/`).
  Stronger than the plan states: a real Rust validator
  (`tooling/rust/crates/agov-validators/src/assistant_loader.rs`) already
  hard-codes `.claude/CLAUDE.md` as the expected Claude Code loader path,
  with test fixtures showing the intended minimal content (`@AGENTS.md`)
  — the validator and the missing file are already out of sync today, not
  just a documentation gap.
- **Plan validates clean**: `agov plan validate` on the pinned commit ->
  OK, severity: blocking. `agov coord validate` on `r47q` -> OK, severity:
  blocking.

## Point-by-point on the requested dimensions

**1. Lifecycle coherence.** Sound. The phase table (loader/setup, cold
start, intake/Gate 1, implementation, poll/coordination, Gate 2,
release/completion) covers every phase this session actually exercised
this session (poll-loop discipline, Gate 1 handoff/decision-record
distinction, Gate 2 evidence) and matches observed practice rather than
inventing new phases. No finding.

**2. MCP/CLI parity.** Correctly scoped, not overreaching. The plan states
plainly: "MCP is not a different governance implementation," "never infer
that MCP has authority to approve gates, authenticate actor identity,
enforce routing, or expand permissions," and "make no token-efficiency
superiority claim" — this last point directly correct given `njsk`'s
unresolved `coord-list` outlier and Codex's zero MCP samples (`ahmj`).
Preserves CLI as complete fallback everywhere; does not make MCP
mandatory. No finding.

**3. Enforceability.** Proportionate. The authority-ref mechanism is
explicitly framed as validating *declared* role status against an
immutable Git ref, not authenticating the caller — this is exactly the
correct resolution to the gap `njsk` identified (self-reported,
unauthenticated `from`/`to` fields) without overclaiming a security
property the mechanism doesn't provide. The plan states this limit twice
(once in "Authority-freshness enforcement," once in "Explicit exclusions":
"No authentication, cryptographic actor attestation..."). Opt-in policy
preserves backward compatibility for non-enforcing adopters. No finding.

**4. Audience separation.** Correct per `documentation-audience-rule`.
Three distinct projections — foundry executable contract (`AGENTS.md`),
adopter executable template (`agf/templates/AGENTS.md`), human-facing
public docs — are kept as one contract with different projections rather
than independently authored copies, which is the stated failure mode
being fixed. No finding.

**5. Downstream compatibility.** Classification (PREVENTIVE/ADDITIVE) is
justified and the required three-circle impact statement
(Foundry/Direct adopters/Downstream/Migration/Evidence) is present and
correctly filled per `downstream-impact-rule`. Opt-in enforcement policy
and explicit "existing CLI-only and non-enforcing setups remain valid"
correctly avoid a breaking classification. No finding.

**6. Security/identity overclaiming.** None found — this is the strongest
part of the plan. It directly closes the gap `njsk` raised (framework.md's
aspirational orchestration-layer enforcement vs. agov's actual
self-reported identity) with correctly bounded language throughout, not
just in one disclaimer. No finding.

## Advisories (non-blocking)

**A1 — implementation should not land as one MR.** The scope spans root
`AGENTS.md`, a new `.claude/CLAUDE.md`, `.claude/rules/README.md`,
`agf/templates/AGENTS.md`, 6+ public doc pages, and Rust
core/CLI/MCP/CI changes. Per `branch-and-worktree-rule`'s sizing test ("can
a reviewer answer one yes/no question"), this doesn't pass as a single
diff — a reviewer approving the authority-ref Rust implementation and a
reviewer approving the public-docs wording are answering different
questions. Recommend the coder-tester split implementation into at least:
(i) the Rust core/CLI/MCP/CI authority-ref slice with its contract tests,
(ii) the foundry executable-guidance slice (`AGENTS.md`,
`.claude/CLAUDE.md`, `.claude/rules/README.md`, adopter template), and
(iii) the public-docs projection slice — each with its own Gate 2 evidence
handoff, all under the single Gate 1 approval this plan requests. This
doesn't require re-running Gate 1 per slice, only avoiding one
unreviewable mega-diff at Gate 2.

**A2 — cross-reference already-landed skill-mirror fix.** MR !590
(`learning/tool-sync-drift-and-skill-fix`, commit `d5b8a05`) already fixed
a directly related instance of the pattern this plan addresses: it
replaced `.claude/commands/*.md` (Claude Code's actual project-skill
mirrors of `agf/skills/*/SKILL.md`, found stale since 2026-06-21) with
thin pointer wrappers, and filed learning `20260726T121943Z-w5jj`
generalizing the "documented sync relationship with no mechanical
enforcement drifts silently" pattern — the same pattern this plan's
`.claude/CLAUDE.md` and `.claude/rules/README.md` scope items address for
rules/loader. Recommend this plan cite MR !590 and learning `w5jj`
alongside `njsk`/`ahmj` as evidence, both because it's directly relevant
prior art for the same fix shape and to avoid the implementer's
`.claude/rules/README.md`/`.claude/CLAUDE.md` work reinventing or
conflicting with the wrapper convention `w5jj` just established. No file
overlap exists between the two changes today, so this is a citation
addition, not a merge-conflict risk.

## Disposition

Recommend the plan's `advisory_consultation` entry for `contribution-auditor`
be updated from `waived` to `consulted` with this review's outcome, per the
plan's own stated requirement ("the plan must be revised to consulted with
the resulting disposition before governor approval"). Governor-reviewer
Gate 1 disposition is otherwise unblocked by this audit.

---
id: mcp-coordination-dir-contract-fix
status: pending-gate1
topic: agov-mcp-server
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: waived
    reason: single-agent Gate 1 planning session; no infosec-sentinel available to consult directly. This plan fixes a trust-relevant contract defect (silent wrong-scope success on a tool that reports session/evidence state), so infosec-sentinel review at Gate 2 is directly warranted, not just a formality.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor available to consult. The uniform-scope decision below (fixing all six dir-taking tools, not just the one named in the triggering handoff) is exactly the kind of scope-expansion claim contribution-auditor review should verify against the actual diff.
---

# Plan: fix the MCP coordination-directory path contract (`dir` → `coordination_dir`)

**Date:** 2026-07-20T20:05:00Z
**Status:** PENDING GATE 1 APPROVAL
**Triggering handoffs:**
`docs/coordination/20260720T192551Z-coder-tester-rp33.md` (Sol High
assessment, finding #6 / Proposal 1) and
`docs/coordination/20260720T193934Z-coder-tester-hpk6.md` (Codex's Gate 1
routing request, governor-directed, urgent).
**Supersedes-in-part:** `docs/plans/agov-mcp-server.md` (`status:
pending-gate1`) for this one public-contract question only. That plan
remains open for its other unresolved scope; per hpk6's explicit
instruction, its stalled `pending-gate1` status is not being used as
retroactive authorization for this new field-contract change.

**Timing relative to the pending v2.3.0 release (revised after Codex's
review of the sibling synthesis, `docs/coordination/20260720T200018Z-
coder-tester-psht.md`):** Codex correctly flagged that shipping v2.3.0
while still claiming `agov-mcp` is release-quality would contradict the
Sol High assessment's own finding that this defect "should block a claim
that the MCP surface is Level 5 Deployable." Rather than rushing this
plan's implementation to beat the release, `docs/plans/
mcp-coordination-dir-contract-fix.md`'s fix ships on its own timeline
after Gate 1 approval, and a separate change (MR !258,
`docs/mcp-maturity-claim-honesty`) downgrades `agov-mcp`'s documented
status to Level 4 Validated with this plan linked as the known
limitation's tracking record — so v2.3.0 can ship honestly without this
fix blocking it. If the governor instead prefers to hold v2.3.0 for the
full fix, that reverses this call; either is consistent with the
assessment, and the choice belongs to Gate 1 review, not to this plan
unilaterally.

---

## Why a hard rename, not a soft deprecation

The first version of this plan kept `dir` alive as a permanent deprecated
alias, mirroring the coordination-message schema's existing pattern for
the `seq` field (`x-deprecated: true`, a non-blocking advisory WARN,
`agf/schemas/coordination-message.schema.json:43-48`). That precedent was
examined directly before choosing this design, and it does not actually
establish a deprecation *policy* — `seq` was marked deprecated on
2026-06-18 and, as of this plan's writing five weeks and multiple minor
releases later, has never been removed or scheduled for removal. Kept
means kept indefinitely, in current practice, not "kept until a sunset
date."

`agf/framework.md:1034` states the project's own policy directly:

> For pre-customer products, documentation should show current state
> only — no deprecation notices or migration timelines, which signal
> instability to evaluating organisations. Deprecation governance is
> introduced when the product has production customers.

**Revised again after Codex's Gate 1 review (`sga1`):** the paragraph
below originally claimed `agov-mcp` has "verifiably zero production
customers." That's not something this repository can actually verify —
it's public and open-source; the repo can confirm no *supported released*
artifact contains a Level 5 MCP surface, but it cannot confirm nobody has
built and used `agov-mcp` from a source checkout. Corrected: `agov-mcp`
has **no supported released MCP consumers — it is Level 4 Validated, not
yet Level 5 Deployable (see `docs/mcp-maturity-claim-honesty`, MR !258).
Source-checkout consumers are possible but unknown**, which is a weaker
and more honest claim than "zero customers," and it's sufficient on its
own: the field has no supported released contract, the change is
classified BREAKING (not softened), unknown old keys fail explicitly
rather than degrading silently, and migration is a one-field rename.
Carrying an alias, a `field_conflict` precedence rule, and matching test
fixtures for a contract with no supported released consumers is exactly
the kind of premature-ceremony cost `simplicity-rule` asks to be
justified, not assumed. It is not justified here — and that conclusion
does not depend on an unknowable adoption count.

**The honest accounting, stated plainly for anyone reading this plan
later:** this project does not yet have a real deprecation policy with an
actual sunset mechanism — only a schema-level "mark it deprecated and nag
forever" pattern (`seq`) that has never been exercised to completion. A
separate, independently-reviewed governance change
(`agf/rules/workflow/deprecation-policy-rule.md`, its own Gate 1, MR
!262/!263) establishes what this project's policy will actually be **once
it has adopters who can depend on a surface**. This plan does not depend
on that rule merging, and its approval is not required or assumed here —
it is cited as forward-looking context for *why this is a disclosed
one-off, not an established repeatable pattern*, nothing more. Doing the
hard rename now, and documenting why here rather than silently taking the
shortcut, is the trust-preserving choice — future readers should not have
to guess whether this was an oversight or a deliberate, disclosed
trade-off. It was deliberate.

---

## Intent

`agov-mcp`'s tools forward a `dir` argument straight to CLI `--dir`. The
CLI's `--dir` means "coordination-message directory" (default
`docs/coordination`), not "repository root." A caller who passes the
repository root gets a plausible `ok: true` response instead of an error,
because `read_coord_messages_with_invalid` happily opens any directory and
treats non-coordination `.md` files as invalid-and-skipped rather than
fatal (`tooling/rust/crates/agov/src/main.rs:3220-3253`). The result: zero
messages, a low-visibility "Invalid coordination files ignored: N" line
buried in stdout, and no structural signal that the caller queried the
wrong evidence domain.

Fix the field's name, description, default-handling, and add a
machine-visible (not just prose-buried) warning when the resolved
directory looks wrong.

**Revised 2026-07-20 (second revision, after direct governor question):**
the original design below kept `dir` as a permanent deprecated alias. That
design is dropped in favor of a clean, hard-breaking rename with no
back-compat shim — see "Why a hard rename, not a soft deprecation" below
for the rationale. This is a deliberate, disclosed shortcut, not an
oversight.

## Scope clarification (Gate 1 decision needed)

hpk6 names this specifically for `agov_session_resume.dir`. The identical
`dir` → `--dir` forwarding, with the identical defect, exists in five other
tools that share the same `push_optional_flag(args, "dir", "--dir", ...)`
helper: `agov_handoff_next`, `agov_handoff_take`, `agov_handoff_complete`,
`agov_coord_post`, `agov_coord_list`
(`tooling/rust/crates/agov-mcp/src/main.rs:243,249,256,264,278,284`).

**Proposal:** apply the field rename uniformly across all six tools.
Fixing only `agov_session_resume` would leave five-sixths of the affected
surface with the same silent-wrong-scope bug — a caller could just as
easily pass the repo root to `agov_coord_list` and get a silently-empty
listing.

**What does NOT get uniformly applied:** the structural warning-on-likely-
wrong-directory behavior (see Design item 4) is scoped to
`agov_session_resume` only for this plan, matching the literal ask. It is
the tool most commonly used for cold-start orientation and therefore the
highest-value place to catch the mistake early; extending the warning to
the other five tools is a reasonable follow-up, not bundled here to keep
this change reviewable as one diff.

## Downstream impact (three-circle statement)

```
Impact:
  Foundry:          No. This project's own MCP config (.mcp.json,
                     untracked) and any in-flight session already default
                     to omitting `dir`, which is unaffected. No current
                     session depends on passing a literal `dir` value.
  Direct adopters:  BREAKING, by design, deliberately. Any caller that
                     passes `dir` explicitly must switch to
                     `coordination_dir` — no alias, no grace period.
                     Justified because agov-mcp has zero production
                     customers today (Level 4 Validated, not yet Level 5
                     Deployable — MR !258) and framework.md:1034's own
                     pre-customer policy explicitly permits this. See
                     "Why a hard rename, not a soft deprecation" above.
  Downstream:       Any agent-authored MCP client config or prompt that
                     hardcodes `dir` breaks until updated to
                     `coordination_dir`. No other artifact-shape change.
  Migration:        Required, but trivial: rename the field in whatever
                     MCP client config or prompt scaffolding passes it
                     explicitly (most callers omit it and use the
                     `docs/coordination` default, which is unaffected).
                     Documented in the CHANGELOG entry and
                     `agf/docs/site/docs/adopt/mcp-server.md`.
  Evidence:         Sol High assessment finding #6 (docs/reviews/
                     agf-framework-assessment-codex-sol-high.md), reproduced
                     directly against tooling/rust/crates/agov-mcp/src/main.rs
                     and tooling/rust/crates/agov/src/main.rs in this
                     session.
```

This is classified **BREAKING**, correctly, not softened to PREVENTIVE —
per `downstream-impact-rule`'s own test ("a commit containing both a
breaking and an additive change is BREAKING"), and per the honesty
principle this plan is built around: calling a hard-breaking rename
PREVENTIVE just because the blast radius happens to be small right now
would be exactly the kind of overclaiming this project's other rules
(`verifiable-claims-rule`, `docs/implementation-honesty-rule`) forbid.
The migration note above satisfies the BREAKING classification's
requirement.

## Design

1. **Rename the public field.** `dir` → `coordination_dir` in the JSON
   input schema of all six tools listed above
   (`tooling/rust/crates/agov-mcp/src/main.rs`, the `tools()` function,
   lines ~407-489).

2. **Hard rename — no alias, no back-compat, but not silent either.**
   Verified against the current implementation before writing this:
   `agov-mcp` does not depend on any JSON-schema validation crate and does
   not validate incoming `tools/call` arguments against the `inputSchema`
   it advertises — `inputSchema` is descriptive metadata for `tools/list`
   only (`tools()`, `main.rs:372-491`); a well-behaved client may reject
   an unknown field before sending it, but the server itself currently
   does not. `tool_cli_args` (`main.rs:219-290`) only reads keys it
   recognizes and silently drops the rest. So a rename with nothing else
   changed would mean a caller still passing the old `dir` key gets it
   silently dropped, falling back to the `docs/coordination` default with
   no error — exactly the "plausible wrong success" failure shape this
   plan exists to eliminate. Not an acceptable residual gap.

   Fix: `tool_cli_args` explicitly checks each tool's incoming argument
   keys against its known set (already has the `Result<_, String>` return
   shape needed to report this) and returns an error naming any
   unrecognized key — including `dir` specifically, with a message
   pointing to `coordination_dir` (worth a slightly friendlier message
   than a generic "unknown key" error, precisely because `dir` is a
   plausible mistake this fix itself created). Also declare
   `"additionalProperties": false` on all six tools' advertised
   `inputSchema`s, so well-behaved clients catch the mistake even earlier,
   client-side, before a call is ever sent — belt-and-suspenders, not a
   substitute for the server-side check. No conflict-precedence logic, no
   deprecation note, no alias-maintenance burden.

3. **Field description text** (in the schema, shown to the calling agent):
   `"Coordination-message directory — NOT the repository root. Defaults to
   docs/coordination when omitted."` Applied to all six tools' schemas.

4. **Structural wrong-directory detection — `agov_session_resume` only —
   revised per Codex's Gate 1 review (`sga1`), replacing the original
   stdout-parsing design.** The original design here proposed string-
   anchoring on the CLI's human-readable `"Invalid coordination files
   ignored: "` line. Codex correctly flagged that as recreating exactly
   the contract fragility this fix exists to remove — it would leave the
   MCP adapter's trust signal dependent on prose formatting instead of
   structural evidence. Agreed; replaced.

   New design: before invoking the CLI, the MCP layer canonicalizes the
   resolved `coordination_dir` (relative to `AgovCommand.cwd`) and the
   resolved repository root, and checks structurally:
   - If the resolved `coordination_dir` **equals the repository root**
     AND a conventional nested `docs/coordination` directory exists there
     — a confidently-identified instance of exactly the mistake this fix
     targets — **fail** with a specific, actionable error (not a soft
     warning): the caller almost certainly meant the nested directory.
   - Otherwise, if the CLI's own existing `"Invalid coordination files
     ignored: "` signal fires (`render_invalid_files`,
     `tooling/rust/crates/agov/src/main.rs:2764-2777`) with zero valid
     messages, but the structural check above did *not* confidently
     identify the repo-root mistake (e.g. an unconventional layout), emit
     the **softer warning** instead of failing — this tolerates
     legitimate alternate layouts that happen to contain a few
     non-coordination `.md` files, rather than rejecting them outright.

   This keeps the strong, blocking signal grounded in verifiable
   filesystem structure the adapter already has to compute anyway (it
   resolves paths regardless), not in parsing another program's
   human-readable output.

   **Response shape, corrected:** verified against the current
   implementation before writing this — `call_tool`
   (`tooling/rust/crates/agov-mcp/src/main.rs:110-119`) already places the
   tool's structured result under `"structuredContent": execution`, where
   `execution` is the fixed-field `ToolExecution` struct
   (`main.rs:551`, fields: `ok, status, command, stdout, stderr`). There
   is no room in the current response for a new sibling top-level field
   on the JSON-RPC envelope, and adding one there would be a shape
   inconsistent with how every other tool result is structured. Any new
   field (the warning above, resolved-path provenance below, the
   unrecognized-key error from Design item 2) belongs as an added field
   on `ToolExecution` itself, appearing inside `structuredContent` — not
   beside it.

5. **Resolved-path provenance, expanded.** The `agov_session_resume`
   response includes the resolved `coordination_dir` value actually used
   (whether it came from an explicit argument or the `docs/coordination`
   default) **and the resolved repository root**, so a cold-starting agent
   can see what it actually queried without guessing. Sol High's original
   Proposal 1 also asked for selected-binary/version and base-branch
   provenance; this plan deliberately narrows that to path provenance only
   for scope reasons — stated explicitly here per Codex's review, rather
   than silently dropping the broader ask. Binary/version/base-branch
   provenance remains open future work, not forgotten.

6. **Fixtures, expanded per Codex's review.** Add MCP integration tests
   covering: omitted `coordination_dir` (uses default), correct value,
   confidently-identified repository-root value with a nested
   `docs/coordination` present (triggers the **fail** path from Design
   item 4), an ambiguous/ill-defined directory with stray non-coordination
   `.md` files but no confident repo-root match (triggers the **warn**
   path), absolute path, nonexistent path (CLI's existing "Coordination
   directory not found" error path,
   `tooling/rust/crates/agov/src/main.rs:2549-2555`), the old `dir` key
   returning the explicit unrecognized-key error from Design item 2 (not a
   silent default-directory fallback), an arbitrary unrelated unknown key
   returning the same error, and the advertised `inputSchema`'s
   `"additionalProperties": false` present for all six tools.

7. **CLI/MCP semantic parity test.** A direct test asserting that invoking
   `agov_session_resume` with `coordination_dir: "docs/coordination"` via
   MCP and running `agov session resume --dir docs/coordination` via CLI
   produce equivalent *normalized semantic state* (same messages, same
   warnings) — not byte-equal stdout. The MCP response wraps CLI stdout
   plus structured additions; byte-for-byte comparison would be the wrong
   invariant and would break the moment Design item 4's structural warning
   ships (MCP-only content with no CLI stdout equivalent).

8. **Documentation and changelog.** Update
   `agf/docs/site/docs/adopt/mcp-server.md` to describe the
   `coordination_dir` field and state plainly that `dir` was renamed
   without a back-compat alias, with the rationale from "Why a hard rename,
   not a soft deprecation" above linked or restated briefly. Add a
   `### Changed` (breaking) entry to `agf/CHANGELOG.md`'s `[Unreleased]`
   section — not buried under `### Added`. Add a short pointer note at
   the top of `docs/plans/agov-mcp-server.md` referencing this plan for the
   field-contract question specifically (satisfies hpk6's "explicitly
   governed amendment" instruction without editing that plan's still-open
   remaining scope).

## Test strategy

Per `workflow/tdd-rule.md`: write the six fixture cases (Design item 6) and
the parity test (Design item 7) first, confirm they fail against the
current `dir`-only implementation (RED), then implement the rename,
unrecognized-key rejection, warning, and provenance changes to make them
pass (GREEN). Existing
`agov-mcp` unit tests (`tool_list_contains_only_cli_backed_tools`,
`coord_post_maps_to_cli_arguments`, etc.) must be updated in the same
commit, not left asserting the old field name — per
`feature-regression-guard-rule`, a test deleted or silently changed to
match new behavior without documenting why is a BLOCK.

## Risks

- **Scope-creep risk (self-identified):** the uniform six-tool rename is
  larger than the literal ask in hpk6. Named explicitly above for Gate 1
  sign-off rather than decided unilaterally during implementation.
- **Breaking-change risk, accepted deliberately:** any caller — including
  any of this project's own future sessions — that hardcodes the old
  `dir` key breaks with an explicit error rather than degrading silently.
  Accepted because current adopter count for this specific field is
  verifiably zero (pre-release surface) and framework.md's own
  pre-customer policy permits exactly this trade-off. Revisit this
  judgment call the moment `agov-mcp` graduates past Level 4 Validated —
  by then, the new `deprecation-policy-rule.md` should govern any further
  renames on this surface instead of another one-off hard break.
- **Residual heuristic fragility (Design item 4, narrowed):** the
  confidently-identified case (resolved path equals repo root, nested
  `docs/coordination` exists) fails outright and is structurally sound,
  not heuristic. The remaining ambiguous case — zero valid messages plus
  stray invalid files, without a confident repo-root match — still relies
  on the CLI's `"Invalid coordination files ignored: "` signal as a proxy,
  not a certainty (a genuinely sparse coordination directory with a few
  stray files would also trigger it). Deliberately kept as a soft warning,
  not an error, specifically to tolerate that narrower band of false
  positives.
- **Plan-stage advisory consultation, addressed rather than deferred:**
  Codex's review noted the contribution-auditor waiver (six-tool scope
  expansion, a deliberate breaking change, migration language) is exactly
  plan-stage material and asked that it be consulted before Gate 1 rather
  than left to Gate 2. Rather than continue deferring for infrastructure
  this project doesn't have (a separately-instantiated contribution-auditor
  agent), this plan is reviewed under that role directly — see
  `docs/reviews/mcp-coordination-dir-contract-fix-contribution-audit.md` —
  following the same formal-advisory-role pattern already validated in
  this project's own learning corpus (`20260703T095527Z-fadv`).

## Rollback

Revert the `agov-mcp` crate changes; since `agov-mcp` has not yet shipped
in a published release (Level 4 Validated only), reverting pre-release
has no adopter migration to undo. Documentation and changelog reverts are
trivial single-file reverts. This calculus changes once `agov-mcp` ships
in an actual release — a later rollback after that point would itself
need the deprecation-policy rule's guidance, not this plan's.

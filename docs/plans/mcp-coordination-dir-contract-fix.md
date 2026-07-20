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
directory looks wrong — without breaking existing callers that already
pass `dir`.

## Scope clarification (Gate 1 decision needed)

hpk6 names this specifically for `agov_session_resume.dir`. The identical
`dir` → `--dir` forwarding, with the identical defect, exists in five other
tools that share the same `push_optional_flag(args, "dir", "--dir", ...)`
helper: `agov_handoff_next`, `agov_handoff_take`, `agov_handoff_complete`,
`agov_coord_post`, `agov_coord_list`
(`tooling/rust/crates/agov-mcp/src/main.rs:243,249,256,264,278,284`).

**Proposal:** apply the field rename and deprecated-alias handling
uniformly across all six tools. Fixing only `agov_session_resume` would
leave five-sixths of the affected surface with the same silent-wrong-scope
bug — a caller could just as easily pass the repo root to
`agov_coord_list` and get a silently-empty listing.

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
  Direct adopters:  PREVENTIVE. The `dir` field keeps working as a
                     deprecated alias for `coordination_dir` — no adopter
                     integration breaks. New behavior is additive
                     (warning field on a specific misuse pattern) and a
                     rename that leaves the old name functional.
  Downstream:       No change to agent-produced artifact shape. Adopters
                     using `dir` see an unchanged response shape; adopters
                     using the new `coordination_dir` name or reading the
                     new warning field get more information, not less.
  Migration:        None required. `dir` continues to work. Recommended,
                     not mandatory: adopters update to `coordination_dir`
                     at their own pace; `dir` may be removed in a future
                     major version with its own deprecation-window
                     decision (out of scope here).
  Evidence:         Sol High assessment finding #6 (docs/reviews/
                     agf-framework-assessment-codex-sol-high.md), reproduced
                     directly against tooling/rust/crates/agov-mcp/src/main.rs
                     and tooling/rust/crates/agov/src/main.rs in this
                     session.
```

This is classified **PREVENTIVE**, not BREAKING, specifically because the
design below chooses "warn," not "fail," for the misuse case (see Design
item 4 and the explicit tradeoff note there). If Gate 2 review or
implementation experience concludes "fail" is the safer choice for a
clearly-wrong directory, that decision must be re-classified as BREAKING
and gain a migration note before shipping — flagging this now so it isn't
silently decided during implementation.

## Design

1. **Rename the public field.** `dir` → `coordination_dir` in the JSON
   input schema of all six tools listed above
   (`tooling/rust/crates/agov-mcp/src/main.rs`, the `tools()` function,
   lines ~407-489).

2. **Keep `dir` as a deprecated alias.** In `tool_cli_args`, accept either
   `coordination_dir` or `dir`; if both are present, `coordination_dir`
   wins. If only `dir` is present, use its value but attach a
   `deprecated_field_used: "dir"` note to the tool's JSON-RPC response
   (not just silently accepted) so callers have a visible nudge to migrate.

3. **Field description text** (in the schema, shown to the calling agent):
   `"Coordination-message directory — NOT the repository root. Defaults to
   docs/coordination when omitted."` Applied to all six tools' schemas.

4. **Structural warning on likely-wrong directory — `agov_session_resume`
   only.** When the CLI's stdout for `session resume` contains the
   existing `"Invalid coordination files ignored: "` line (already emitted
   by `render_invalid_files`,
   `tooling/rust/crates/agov/src/main.rs:2764-2777`) **and** zero pending
   handoffs / claims / decisions were found, the MCP layer attaches a
   top-level structured `warning` field to its JSON-RPC response — not
   relying on the agent to notice a line buried in a larger text blob.
   Suggested wording: `"N file(s) in <resolved dir> failed
   coordination-message validation and 0 valid messages were found. Check
   whether <resolved dir> is the coordination-message directory, not the
   repository root."`

   **Implementation tradeoff, left to coder-tester with Gate 2 sign-off:**
   this can be done by (a) a light, string-anchored parse of the CLI's
   already-stable `"Invalid coordination files ignored: "` prefix (no CLI
   changes needed, matches this plan's PREVENTIVE/non-breaking scope), or
   (b) adding a structured `--format json` output to `session resume`
   exposing `{messages_count, invalid_files_count}` machine-readably (a
   real CLI contract addition, more robust, larger surface). Recommend (a)
   for this fix given the scope; (b) is reasonable future work, not
   bundled here.

5. **Resolved-path provenance.** The `agov_session_resume` response
   includes the resolved `coordination_dir` value actually used (whether it
   came from an explicit argument, the deprecated `dir` alias, or the
   `docs/coordination` default), so a cold-starting agent can see what it
   actually queried without guessing.

6. **Fixtures.** Add MCP integration tests covering: omitted `dir`/
   `coordination_dir` (uses default), correct value, repository-root value
   (triggers the warning), absolute path, nonexistent path (CLI's existing
   "Coordination directory not found" error path,
   `tooling/rust/crates/agov/src/main.rs:2549-2555`), and both the `dir`
   alias and the new `coordination_dir` name producing identical CLI
   arguments.

7. **CLI/MCP semantic parity test.** A direct test asserting that invoking
   `agov_session_resume` with `coordination_dir: "docs/coordination"` via
   MCP and running `agov session resume --dir docs/coordination` via CLI
   produce equivalent session state (same messages, same warnings).

8. **Documentation and changelog.** Update
   `agf/docs/site/docs/adopt/mcp-server.md` to describe the
   `coordination_dir` field and the deprecated `dir` alias. Add an entry to
   `agf/CHANGELOG.md`'s `[Unreleased]` section. Add a short pointer note at
   the top of `docs/plans/agov-mcp-server.md` referencing this plan for the
   field-contract question specifically (satisfies hpk6's "explicitly
   governed amendment" instruction without editing that plan's still-open
   remaining scope).

## Test strategy

Per `workflow/tdd-rule.md`: write the six fixture cases (Design item 6) and
the parity test (Design item 7) first, confirm they fail against the
current `dir`-only implementation (RED), then implement the rename +
alias + warning + provenance changes to make them pass (GREEN). Existing
`agov-mcp` unit tests (`tool_list_contains_only_cli_backed_tools`,
`coord_post_maps_to_cli_arguments`, etc.) must be updated in the same
commit, not left asserting the old field name — per
`feature-regression-guard-rule`, a test deleted or silently changed to
match new behavior without documenting why is a BLOCK.

## Risks

- **Scope-creep risk (self-identified):** the uniform six-tool rename is
  larger than the literal ask in hpk6. Named explicitly above for Gate 1
  sign-off rather than decided unilaterally during implementation.
- **Alias-maintenance cost:** carrying `dir` as a permanent-until-further-
  notice alias means two field names to keep in sync in schema and tests
  indefinitely, until a future major version removes it. Acceptable
  cost for a zero-migration-required change.
- **Heuristic fragility (Design item 4):** the "zero valid messages + one
  or more invalid files" signal is a proxy for "wrong directory," not a
  certainty — a genuinely empty coordination directory with a few stray
  non-coordination `.md` files would also trigger it. The message is
  phrased as a check-this warning, not an error, specifically to tolerate
  false positives.

## Rollback

Revert the `agov-mcp` crate changes; `dir` was never removed, so no adopter
migration is undone by reverting. Documentation and changelog reverts are
trivial single-file reverts.

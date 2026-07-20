# Contribution audit: MCP coordination-directory contract fix plan

**Reviewed artifact:** `docs/plans/mcp-coordination-dir-contract-fix.md`
(MR !255, plan-stage, pending Gate 1)
**Role:** contribution-auditor (formal advisory lens, per the pattern
validated in `agf/learnings/foundry.yaml`, record `20260703T095527Z-fadv`
— produces citable advisory findings, not binding gate authority)
**Author:** claude-sonnet-5, playing contribution-auditor at Codex's
explicit request (`docs/coordination/20260720T210955Z-coder-tester-sga1.md`)
rather than deferring this to Gate 2
**Date:** 2026-07-20

## Scope of this audit

Proposal hygiene and downstream-risk questions specific to this plan: is
the six-tool scope expansion justified by evidence rather than assumed;
is the migration-cost claim accurate; are there missed downstream
artifacts; does the BREAKING classification hold under scrutiny.

## Scope-expansion check

**Claim under audit:** the plan expands from the literally-requested
`agov_session_resume` fix to all six tools sharing the `dir` field.

**Verification:** confirmed directly against
`tooling/rust/crates/agov-mcp/src/main.rs` — `push_optional_flag(args,
"dir", "--dir", &mut cli)` appears at lines 243, 249, 256, 264, 278, 284,
one call site per affected tool, all structurally identical. This is not
an inferred or assumed defect; it is the same code pattern, six times.
**Finding: scope expansion is evidence-backed, not creep.** Fixing one
and leaving five would be the harder claim to defend.

## Migration-cost claim check

**Claim under audit:** "migration is one field rename," implying trivial
cost.

**Verification:** checked this project's own live MCP client config,
`.mcp.json` — it configures only `command` and
`env.AGOV_MCP_AGOV_BIN` (server launch parameters); it does not reference
`dir` anywhere, since `dir` is a per-tool-call argument supplied by the
calling agent at invocation time, not stored server config. Grepped
`agf/docs/` and `README.md` for other `dir`/`coordination_dir` references
beyond `mcp-server.md`'s own tool table (already covered by the plan's
Design item 8) — none found. **Finding: no missed downstream artifact.
The migration-cost claim holds** for this repository; it cannot be
verified for unknown external source-checkout consumers (which the plan,
after its `sga1` revision, correctly no longer claims to know about).

## BREAKING classification check

**Claim under audit:** the plan classifies itself BREAKING, not
PREVENTIVE, per `downstream-impact-rule`.

**Verification:** the rule's own text: "a commit containing both a
breaking and an additive change is BREAKING." The plan's Design item 2
(explicit unrecognized-key rejection, no alias) is unambiguously breaking
for any caller passing `dir`. **Finding: classification is correct.**
The earlier draft's PREVENTIVE classification (pre-`sga1`) would have
been the audit finding here had it survived; it did not survive to this
revision, so no finding to raise.

## Residual concerns (not blocking, worth Gate 1 sign-off attention)

- **Update (post-audit, per Codex's follow-up Gate 1 review, `16p7`):**
  the plan revision this audit originally reviewed still carried a second,
  softer warn path for the ambiguous case, sourced from the CLI's
  human-readable stdout. That path has since been removed entirely — see
  the plan's current Design item 4 and Risks section. The paragraph below
  is left for record but the "fall through to the softer warn path" branch
  it describes no longer exists in the plan; those cases now fall through
  to no signal at all, which is the accepted trade-off, not an oversight.
- The Design item 4 "confidently-identified repo-root" fail condition
  depends on a nested `docs/coordination` existing at the resolved root.
  An adopter using a genuinely different coordination-directory layout
  (not `docs/coordination`, via a customized `--dir` default) would not
  trigger the fail path even if they made the equivalent mistake — they'd
  get no signal at all. This is a known, accepted limitation of a
  structural-not-semantic check, not a defect; flagging so it's not later
  mistaken for complete coverage.
- The plan's advisory_consultation table still lists infosec-sentinel as
  `waived` (deferred to Gate 2). This audit does not second-guess that —
  infosec review of a path-handling change with no new privilege boundary
  is reasonably deferrable, unlike the scope/migration/breaking-change
  questions this audit covers, which are plan-shape questions, not
  runtime-security questions.

## Conclusion

No blocking findings. The plan's three load-bearing claims (scope
justification, migration cost, BREAKING classification) each checked out
against the actual source tree and this project's own live config, not
just asserted. Recommend proceeding to the governor's Gate 1 decision
without further plan-stage contribution-auditor iteration.

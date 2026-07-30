---
id: assistant-discovery-archive-setup
status: pending-gate1
topic: assistant-discovery-archive-setup
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >
      Issues #87, #90 independently reproduced and confirmed by
      contribution-auditor (`20260730T072926Z-contribution-auditor-rk2k`,
      `20260730T073256Z-contribution-auditor-jpje`), which corrected #87's
      framing from "always require both loader files" to
      "assistant-surface/configuration aware" — requiring both globally
      would penalize projects that intentionally support only one
      assistant runtime. Decomposition from
      `20260730T074742Z-contribution-auditor-bhhh`.
  - role: infosec-sentinel
    status: consulted
    reason: >
      #90 ships a template credential/config file
      (`examples/claude/.mcp.json`) inside public release archives.
      Must contain no machine paths, secrets, environment values, or
      endpoints — flagging for an explicit check that the shipped
      template is genuinely sanitized before this plan's Gate 2, not
      merely "looks generic."
---

# Plan: assistant discovery and archive setup (#87, #90)

## Trigger

Filed 2026-07-30 from the same session as the other two plans in this
intake (see `agov-coordination-read-path-reliability.md` and
`agov-mcp-transport-protocol-correctness.md`): a Claude Code session on
this repository never loaded `AGENTS.md` — only `.claude/rules/*.md`
auto-loads as Claude Code project instructions — and there is no
`.claude/CLAUDE.md` bridge file in this repository to surface it.
Separately, `.mcp.json` (the config that wires up `agov-mcp` for Claude
Code) exists in this working tree but is untracked, machine-specific, and
has no committed template anywhere adopters could copy. Both confirmed
independently by contribution-auditor.

## Issues in scope

### #87 — no `.claude/CLAUDE.md` loader in this repository

`tooling/rust/crates/agov-validators/src/assistant_loader.rs:7-42`:

```rust
const LOADER_CANDIDATES: &[(&str, &str)] = &[
    ("AGENTS.md", "Codex / GitHub Copilot"),
    (".claude/CLAUDE.md", "Claude Code"),
];
```

`found_any` passes as soon as *either* candidate exists. This repository
has `AGENTS.md` but no `.claude/CLAUDE.md`, so `agov validate` reports
`OK agov.validate.assistant_loader` even though Claude Code — confirmed
by this session's own auto-loaded context — has no path to `AGENTS.md`'s
operational guidance at all. The validator's own table already labels the
two candidates by distinct assistant surface; it just doesn't act on that
distinction. Correction from `rk2k`: the fix must be
surface/configuration-aware (e.g., a project that intentionally supports
only Codex should not be forced to also carry a `.claude/CLAUDE.md` it
has no use for), not a blanket "require both."

### #90 — no shipped, ready-to-copy Claude MCP configuration template

Per governor clarification (`jpje`), the Claude MCP configuration belongs
with the platform-specific `agov-*` tooling release archives (which
already package both `agov` and `agov-mcp`), not the framework
source-archive template. Currently no archive ships an
`examples/claude/.mcp.json` or equivalent, and the archive `README.txt`
(generated at `.gitlab-ci.yml:907,942`) does not mention Claude Code, MCP,
or how to wire up `agov-mcp` at all (same documentation gap issue #69
found for the `AGOV_MCP_AGOV_BIN` requirement generally).

## Gate 1 decomposition

One plan, two distinct deliverables that share an adoption/setup framing
but ship independently — per `bhhh`, prepare together, but this plan's
Gate 2 should consume the final supported MCP tool list from
`agov-mcp-transport-protocol-correctness.md` (Slice 2) before closing,
since the shipped template's correctness depends on which tools actually
exist.

### Slice 1 — Foundry Claude loader bridge (#87)

1. Add a minimal `.claude/CLAUDE.md` to this repository — a thin pointer
   to `AGENTS.md`, not a duplicate of its content (per
   `documentation-rule-projection-rule.md`'s "point at it... do not
   become a second, competing rule").
2. Change `assistant_loader`'s `found_any` check from "any candidate
   exists" to configuration-aware: default to requiring the candidate(s)
   matching the assistant surfaces the project actually declares support
   for (new optional config, e.g. a field in `agf/agents/MANIFEST.yaml`
   or a small dedicated config listing supported assistant loaders);
   absent that config, preserve current OR-based behavior so projects
   that haven't opted in aren't broken by this change. This is the
   `rk2k`-corrected shape — not a global "require both."
3. Verify `agf/templates/AGENTS.md` (the adopter-facing template) and any
   "set up your AI loaders" adopter instructions actually mention
   creating `.claude/CLAUDE.md` as a distinct, required-for-Claude-Code
   step, not implied by copying `AGENTS.md` alone.

### Slice 2 — Shipped Claude MCP configuration template (#90)

1. Add `examples/claude/.mcp.json` to the `agov-*` platform tooling
   archive staging (both the Linux tar and Windows zip packaging blocks
   in `.gitlab-ci.yml`'s `.rust-tooling-package` template, per issue #69's
   precedent of both archive variants needing the same fix). Content:
   `"command": "agov-mcp"` with no `AGOV_MCP_AGOV_BIN`, no absolute paths,
   no secrets, no environment values, no endpoints — relying on
   `agov-mcp` being on `PATH` (consistent with whatever #69's PATH-lookup
   fallback resolves to, if that issue's fix has landed by this plan's
   implementation time; otherwise document the `AGOV_MCP_AGOV_BIN`
   requirement explicitly instead of shipping a template that silently
   fails the way #69 described).
2. Extend the archive `README.txt` generation to name `agov-mcp` as an
   MCP server, explain copying `examples/claude/.mcp.json` into the
   adopter project root, and link
   `agf/docs/site/docs/adopt/mcp-server.md`.
3. Confirm the template's tool list matches whatever `agov-mcp` actually
   exposes at ship time — this is the dependency on
   `agov-mcp-transport-protocol-correctness.md` Slice 2 `bhhh` names;
   the template itself declares no tool list (that's the server's own
   `tools/list` response), so this is a documentation-accuracy check on
   the README text, not a template-content dependency.

## Affected files

- `.claude/CLAUDE.md` — new file (Slice 1).
- `tooling/rust/crates/agov-validators/src/assistant_loader.rs` —
  `found_any`/`LOADER_CANDIDATES` logic (Slice 1).
- `agf/templates/AGENTS.md` and any adopter-facing loader setup docs —
  verify/update (Slice 1).
- `.gitlab-ci.yml` — `.rust-tooling-package` template, both Linux and
  Windows staging blocks, `README.txt` generation (Slice 2).
- New: `examples/claude/.mcp.json` template, staged into the archive
  build, not the framework source archive (Slice 2, per `jpje`).
- `agf/docs/site/docs/adopt/mcp-server.md` — cross-check against the new
  archive README text for consistency (`marketing-tone-rule.md`'s
  message-integrity principle: the same claim stated once, referenced
  elsewhere).

## Test strategy (Gate 2 evidence)

Per `bhhh`'s minimum evidence list:

1. `agov.validate.assistant_loader` unit tests: a repo with only
   `AGENTS.md` and a Claude-surface declaration now fails (where it
   currently passes); a repo with no assistant-surface declaration at all
   preserves current OR-based behavior (no false rejection for adopters
   who haven't opted in — the explicit "single-surface adopter is not
   falsely rejected" case `bhhh` names).
2. Inspect built Linux tar and Windows zip archive contents in CI:
   `examples/claude/.mcp.json` present, README.txt mentions it.
3. Exercise archive-only setup: following only the shipped README and
   template (no access to the framework source repo or its docs site)
   must be sufficient to get a working `agov-mcp` registration — mirrors
   issue #69's "the archive gives no route to the MCP documentation"
   finding; this plan closes that specific gap for the Claude case.
4. Validate the Claude bridge end-to-end: with `.claude/CLAUDE.md`
   present, confirm (by inspection, since this can't be automated against
   a live Claude Code session) that it correctly points to `AGENTS.md`.
5. Content check: `examples/claude/.mcp.json` contains no absolute
   machine paths, no secrets, no environment-specific values — a
   dedicated infosec-sentinel check before Gate 2, not a general read.

## Risks

- A `found_any` → surface-aware change to `assistant_loader` could
  introduce new false-positive advisories for adopters who haven't
  configured a surface declaration. Mitigated by defaulting to current
  OR-based behavior when no declaration exists (Slice 1, item 2).
- Shipping a template credential/config file in a public archive is a
  standing risk if it ever accidentally captures a real value during
  maintenance. Mitigated by the explicit infosec-sentinel content check
  in Gate 2 evidence item 5, repeated on every future edit to the
  template per normal review discipline.

## Rollback

Slice 1: revert `.claude/CLAUDE.md` and the validator change as one
commit/MR — no data/migration concern. Slice 2: revert the
`.gitlab-ci.yml` archive staging change — affects only the next built
archive, no impact on already-published archives (immutable per
`immutable-tags-rule.md`'s sibling principle for release artifacts).

## Out of scope

- #84, #85, #89 (coordination read-path) and #86, #88 (MCP transport) —
  separate plans; this plan's Slice 2 documentation should reflect
  whatever tool list Slice 2 of the transport-correctness plan ships,
  once it does.
- Any change to `agov-mcp`'s actual tool registry or protocol handling —
  that's the other plan's scope.
- Any v2.4.6 release action.

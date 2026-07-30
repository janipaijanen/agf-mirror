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
but ship independently. Sequencing correction from exact-head review
(A2): `bhhh`'s original note that this plan's Gate 2 should "consume the
final supported MCP tool list" from
`agov-mcp-transport-protocol-correctness.md` overstated the actual
dependency — the shipped `.mcp.json` template (Slice 2) contains no tool
list at all (that's the server's own `tools/list` response at runtime,
not template content), so there is no concrete content dependency
forcing this plan to wait. This plan may prepare and reach its own Gate 1
and Gate 2 independently. The only real coupling: **if** a future
revision of this plan's README text or adopter docs names specific tool
names, that text must be re-checked against
`agov-mcp-transport-protocol-correctness.md`'s final tool list before
merge — a conditional, text-level check at that time, not a standing
plan-level gate today.

### Slice 1 — Foundry Claude loader bridge (#87)

Exact-head review (`20260730T083948Z-contribution-auditor-1dq4`,
findings B6/B7) found this slice's first draft left two decisions
unfrozen that a Gate 1 plan must settle before implementation. Both
resolved below, verified against Claude Code's own documentation
(fetched live 2026-07-30,
`https://code.claude.com/docs/en/memory`), not just the reviewer's
citation.

1. **The bridge must be a real import, not prose (B7).** A pointer
   sentence like "see AGENTS.md" does not cause Claude Code to load
   `AGENTS.md`'s content — Claude Code has no general-purpose
   cross-reference resolution for arbitrary prose. Claude Code's own
   documentation names this exact scenario: "If your repository already
   uses AGENTS.md for other coding agents, create a CLAUDE.md that
   imports it... using `@path/to/import` syntax." Frozen implementation:
   `.claude/CLAUDE.md` (matching `assistant_loader.rs`'s existing
   `LOADER_CANDIDATES` entry for this exact path) containing exactly
   `@../AGENTS.md` as its first line — the relative path is correct
   because Claude Code resolves import paths "relative to the file
   containing the import, not the working directory," and
   `.claude/CLAUDE.md`'s directory is one level below the repository
   root where `AGENTS.md` lives. Claude-Code-specific instructions, if
   ever needed, may follow the import line in the same file (Claude Code
   loads the import, then appends the rest) — none are added by this
   plan. Gate 2 evidence must include an actual Claude Code session
   confirming `.claude/CLAUDE.md` (and, transitively, `AGENTS.md`'s
   content) appears under `/context`'s **Memory files** list — inspecting
   the file's text is not sufficient evidence that the import resolves,
   since a typo in the import syntax would look identical to a correct
   pointer in a code review.
2. **Assistant-surface declaration: exact schema, frozen now (B6, tightened per R3).**
   The first draft offered two undecided options (a `MANIFEST.yaml`
   field or a separate config), then froze the field/location/default/
   error behavior but left the accepted token vocabulary and the
   freshness-check target ambiguous — R3 caught both gaps.

   **Accepted tokens (exact, not "labels").** `LOADER_CANDIDATES` has
   exactly two entries — `("AGENTS.md", "Codex / GitHub Copilot")` and
   `(".claude/CLAUDE.md", "Claude Code")` — a 1:1 mapping from candidate
   *file* to token, not from individual downstream tool to token, because
   the detection mechanism only observes which *file* exists, and cannot
   distinguish which specific tool is reading `AGENTS.md`. The accepted
   `supported_assistant_loaders` tokens are exactly `codex` and
   `claude-code`, one per candidate file. `github-copilot` is an accepted
   **alias of `codex`** (both map to the `AGENTS.md` candidate) — not a
   separate token and not unsupported, since Copilot's own instructions
   file convention (`.github/copilot-instructions.md`) is out of this
   validator's scope entirely; declaring `codex` or `github-copilot`
   requires the same file and is checked identically. Any other value is
   the "unknown value" validation error already specified.

   **Freshness-check target (previously omitted).** Current
   `check_version_freshness` reads the version comment from the loader
   *file itself* and silently skips the check when no comment is present
   ("No comment present: no warning" in the existing source). The new
   `.claude/CLAUDE.md` bridge from item 1 below is deliberately a single
   `@../AGENTS.md` import line with no comment of its own — under
   unmodified current logic, it would never be freshness-checked, which
   defeats the purpose of adding it as a declared, required surface.
   Fix: when a loader candidate file's entire content (after whitespace
   trim) is a single `@<relative-path>` import line, freshness checking
   follows the import and checks the version comment of the **imported
   file** instead of the (comment-less) loader file. `AGENTS.md` already
   carries `<!-- AGF: ... -->`, so this makes the Claude bridge's
   freshness check exact: stale when `AGENTS.md`'s own comment is stale,
   current when it's current — freshness of the bridge is freshness of
   what it actually loads, not of the one-line pointer file. Add matching
   and stale-version test cases for both declared surfaces:
   `AGENTS.md` checked directly (unchanged existing behavior) and
   `.claude/CLAUDE.md` checked via import-following (new behavior).

   Behavior otherwise unchanged from the first draft: when the field is
   present, `found_any` requires every declared surface's candidate file
   to exist; when the field is absent, preserve today's OR-based "any
   candidate" behavior exactly, so undeclared projects are never newly
   broken by this change (the `rk2k`-corrected shape). Name the changed
   schema explicitly: `agf/schemas/*.json` gets no change (`MANIFEST.yaml`
   is not currently schema-validated against a JSON Schema in this
   repository per a check of `agf/schemas/`; if that changes before
   implementation, this field must be added there too).
3. Verify `agf/templates/AGENTS.md` (the adopter-facing template) and any
   "set up your AI loaders" adopter instructions actually mention
   creating `.claude/CLAUDE.md` with the `@../AGENTS.md` import as a
   distinct, required-for-Claude-Code step, not implied by copying
   `AGENTS.md` alone.

### Slice 2 — Shipped Claude MCP configuration template (#90)

Exact-head review (`1dq4`, finding B8) found this slice's first draft
under-specified issue #90's own frozen acceptance criteria — the issue
already fixes the logical JSON shape and requires specific negative and
platform tests this draft only partially named. Corrected to adopt #90's
full acceptance contract directly, not a paraphrase of it:

1. Add `examples/claude/.mcp.json` to the `agov-*` platform tooling
   archive staging (both the Linux tar and Windows zip packaging blocks
   in `.gitlab-ci.yml`'s `.rust-tooling-package` template, per issue #69's
   precedent of both archive variants needing the same fix). Content —
   the complete object from #90, not only the inner `command` field:
   ```json
   {
     "mcpServers": {
       "agov": {
         "command": "agov-mcp"
       }
     }
   }
   ```
   No `AGOV_MCP_AGOV_BIN`, no absolute paths, no secrets, no environment
   values, no endpoints, and no `args`/`env` keys unless a concrete
   reason requires them (if omitted, that's an intentional choice to
   record, not an oversight) — relying on `agov-mcp` being on `PATH`
   (consistent with whatever #69's PATH-lookup fallback resolves to, if
   that issue's fix has landed by this plan's implementation time;
   otherwise document the `AGOV_MCP_AGOV_BIN` requirement explicitly
   instead of shipping a template that silently fails the way #69
   described).
2. Extend the archive `README.txt` generation to: name `agov-mcp` as an
   MCP server; explain installing both binaries on `PATH`; explain
   copying `examples/claude/.mcp.json` into the adopter project root as
   `.mcp.json`; instruct reviewing it; instruct approving the
   project-scoped server when Claude Code prompts (#90's acceptance
   criteria require this exact approval-instruction step, not just
   "copy the file"); and link `agf/docs/site/docs/adopt/mcp-server.md`.
3. **Packaging negative tests (previously missing).** Per #90's own
   acceptance criteria: packaging/self-sufficiency tests must fail when
   the template is absent from the archive, stale (content drifted from
   the checked-in source), or present but not referenced by the local
   README text. Add all three as explicit CI-level negative tests, not
   only a positive "the file exists" check.
4. **Windows behavior tested, not inferred (previously implicit).**
   Exercise the Windows `.zip` archive's `examples/claude/.mcp.json`
   content and the `agov-mcp.exe`/`agov.exe` command contract
   specifically — do not assume Linux packaging test coverage implies
   Windows correctness, since the two staging blocks are separate
   `.gitlab-ci.yml` sections that can drift independently (the same
   failure shape issue #69 found for the base binaries).
5. Validate the shipped JSON as structurally correct Claude project
   configuration (not merely valid JSON) — i.e. it parses under the same
   shape Claude Code expects for `.mcp.json`, per
   `agf/docs/site/docs/adopt/mcp-server.md`'s existing documented
   example.
6. Documentation-accuracy check, not a content dependency (A2): if a
   future revision of the README or adopter docs names specific tool
   names, re-check that text against
   `agov-mcp-transport-protocol-correctness.md`'s final tool list at that
   time — the template's JSON itself carries no tool list and needs no
   such check.

## Affected files

- `.claude/CLAUDE.md` — new file, containing exactly `@../AGENTS.md`
  (Slice 1, B7).
- `agf/agents/MANIFEST.yaml` — new optional
  `supported_assistant_loaders` field (Slice 1, B6).
- `tooling/rust/crates/agov-validators/src/assistant_loader.rs` —
  `found_any`/`LOADER_CANDIDATES` logic, now reading the manifest field
  when present (Slice 1).
- `agf/templates/AGENTS.md` and any adopter-facing loader setup docs —
  verify/update to show the `@../AGENTS.md` import form, not a prose
  pointer (Slice 1).
- `.gitlab-ci.yml` — `.rust-tooling-package` template, both Linux and
  Windows staging blocks, `README.txt` generation (Slice 2).
- New: `examples/claude/.mcp.json` template, staged into the archive
  build, not the framework source archive (Slice 2, per `jpje`), using
  the complete `mcpServers.agov` object from #90 (Slice 2, B8).
- `agf/docs/site/docs/adopt/mcp-server.md` — cross-check against the new
  archive README text for consistency (`marketing-tone-rule.md`'s
  message-integrity principle: the same claim stated once, referenced
  elsewhere).

## Test strategy (Gate 2 evidence)

Per `bhhh`'s minimum evidence list, tightened per `1dq4`'s B6/B7/B8 and
`ppt8`'s R3:

1. `agov.validate.assistant_loader` unit tests: a repo declaring
   `supported_assistant_loaders: [claude-code]` with only `AGENTS.md`
   (no `.claude/CLAUDE.md`) now fails (where it currently passes); a repo
   declaring `[codex]` only, with only `AGENTS.md`, passes (single-surface
   adopter not falsely rejected — the case `bhhh` names); a repo
   declaring `[github-copilot]` behaves identically to `[codex]` (alias,
   same required file); a repo declaring both surfaces requires both
   files; a repo with an unknown value (e.g. `[cursor]`) fails as a
   validation error; a repo with no `supported_assistant_loaders` field
   at all preserves current OR-based behavior exactly.
1a. Freshness-per-surface (R3): a repo declaring `[codex]` with a stale
    `<!-- AGF: ... -->` comment directly in `AGENTS.md` fails, matching
    unchanged existing behavior; a repo declaring `[claude-code]` with a
    current `.claude/CLAUDE.md` (single `@../AGENTS.md` line, no comment
    of its own) but a *stale* comment in the imported `AGENTS.md` fails —
    proving freshness follows the import rather than silently passing
    because the loader file itself has no comment to check.
2. Inspect built Linux tar and Windows zip archive contents in CI:
   `examples/claude/.mcp.json` present with the complete `mcpServers.agov`
   object, README.txt mentions it and the project-scoped-server approval
   step.
3. **Negative packaging tests (B8)**: a CI-level test fails when the
   template is absent from an otherwise-complete archive; fails when the
   template's content has drifted from its checked-in source (stale);
   fails when the template is present but unreferenced by README.txt.
   All three are separate assertions.
4. **Windows-specific test (B8)**: exercise the Windows `.zip` archive's
   template and `.exe` command contract directly, not inferred from the
   Linux test passing.
5. Exercise archive-only setup: following only the shipped README and
   template (no access to the framework source repo or its docs site)
   must be sufficient to get a working `agov-mcp` registration — mirrors
   issue #69's "the archive gives no route to the MCP documentation"
   finding; this plan closes that specific gap for the Claude case.
6. **Validate the Claude bridge end-to-end, live (B7)**: with
   `.claude/CLAUDE.md` present, a real Claude Code session's `/context`
   output must list it (and, transitively, `AGENTS.md`'s content) under
   **Memory files** — not merely an inspection that the import line's
   text looks correct. If a live-session check isn't available in the
   Gate 2 environment, this must be recorded as explicit post-merge
   monitoring, not silently downgraded to a text-only check.
7. Content check: `examples/claude/.mcp.json` contains no absolute
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
  in Gate 2 evidence item 7 (advisory cross-reference corrected — an
  earlier revision pointed to item 5 after item numbers shifted),
  repeated on every future edit to the template per normal review
  discipline.

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

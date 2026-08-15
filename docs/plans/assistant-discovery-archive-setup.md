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
    status: not-applicable
    reason: >
      Round 2 correction: the prior `consulted` status shared the same
      self-contradictory pattern independently caught elsewhere this
      session (v2.6.1's `mqka`/`hegm`; MR !806's `pghm`/`V27-MCP-4`) —
      claiming a status whose own reason text only requests a future
      check. infosec-sentinel has no configured human/AI player
      (`played_by: "ci (agov validate infosec, on commit)"` per
      `agf/agents/MANIFEST.yaml`). Per `pghm`'s `V27-LOAD-4`, the
      review scope is also broader than originally framed — it must
      cover the import-following resolver's path-safety bounds (Slice 1,
      item 2 below), not only the shipped `.mcp.json` template's content
      sanitization. An infosec-review-aid pass covering both surfaces is
      requested separately, mirroring the established `2r8r`/`mdth`/
      v2.6.1 `fq2w`/MR !806 `hzwc` precedent. **This plan is not ready
      for Gate 1 while this entry is unresolved.**
  - role: build-engineer
    status: consulted
    reason: >
      Added per `pghm`'s `V27-LOAD-5`: this plan changes CI, archive
      staging, cross-platform packaging, and release `README.txt`
      generation — surfaces `agf/agents/MANIFEST.yaml` names as
      build-engineer's jurisdiction ("owns CI configuration, build
      automation, build artifacts, ... release-build stages").
      build-engineer is `played_by: "claude (AI, CI/build and
      release-build work)"` — the same AI role I already occupy as
      planner-architect for CI/build work in this project, so this
      status reflects that the plan's `.gitlab-ci.yml`/packaging design
      was authored under that jurisdiction, not a separate non-self
      review. Explicit review by coder-tester of the CI/packaging
      portions specifically is included in the round-2 review request.
---

# Plan: assistant discovery and archive setup (#87, #90)

## Revision history

**Round 4, per coder-tester's exact-head review and infosec-review-aid
pass `20260815T232502Z-coder-tester-p371.md` (requested via `b5wq`, MR
!1597): BLOCK, 2 blocking (`B-V27-LOAD-R3-1`, `B-V27-LOAD-R3-2`).**

- `B-V27-LOAD-R3-1`: round 3's `starts_with('/')` absolute check missed
  Windows drive-letter and UNC forms; its metadata-then-read size bound
  was a TOCTOU race (the file can grow between the check and the read).
  Fixed with `Path::is_absolute()` (platform-native) and a hard
  `MAX_IMPORT_TARGET_BYTES + 1` read cap enforced on the open file handle
  itself, not a pre-read metadata check — see Slice 1 item 7, revised.
- `B-V27-LOAD-R3-2`: round 3's outcome table called import rejections both
  "none" (the silent pre-existing fallback) and "an advisory finding" —
  mutually exclusive claims about the same test's expected output. Fixed
  by picking one outcome: every rejection produces its own named, visible
  advisory diagnostic, distinct from the silent fallback (which stays
  reserved for "no version comment present in a successfully-read file")
  — see Slice 1 items 7-8, revised.

**Round 3, per coder-tester's exact-head review and infosec-review-aid
pass `20260815T224603Z-coder-tester-87k8.md` (requested via `r7vb`, MR
!1595): BLOCK, 2 blocking (`B-V27-LOAD-R2-1`, `B-V27-LOAD-R2-2`).** Seven
of nine `V27-LOAD` findings confirmed `pass` outright. Both remaining:

- `B-V27-LOAD-R2-1`: round 2's import-resolver spec was self-defeating —
  "reject if the canonicalized path is absolute" also rejects the valid
  in-repository bridge, since canonical paths are normally absolute. Fixed
  with a frozen, ordered algorithm: reject the *raw* absolute token before
  any resolution; canonicalize root and target separately; compare by
  path components (not string prefix) for containment; require regular/
  readable; and an exact, metadata-checked size bound
  (`MAX_IMPORT_TARGET_BYTES`, 64 KiB) checked before any content read —
  see Slice 1, item 7, revised.
- `B-V27-LOAD-R2-2`: the plan's own advisory-only framing (item 2)
  contradicted its prose, which described missing/unknown/stale surfaces
  as making the validator "fail" — read as a nonzero exit, that
  contradicts `tool-authority-boundary-rule.md`. Fixed with one outcome
  table (new item 8) pairing every condition with its exact finding and
  exit status (always 0), and reworded every "fails"/"passes" description
  elsewhere in the plan to match.

**Round 2, per contribution-auditor's fresh exact-head review
`20260809T074441Z-contribution-auditor-pghm.md` (governor decision `2gbg`):
COMPLETE/BLOCK against current `main` (`72b23940`), superseding the
2026-07-30 CLEAR evidence, which had gone stale. Nine findings
(`V27-LOAD-1..9`), all addressed in this revision:

- `V27-LOAD-1`: the `supported_assistant_loaders` field is now also
  documented in `agf/agents/MANIFEST.example.yaml` (distributions ship
  this file, not the live `MANIFEST.yaml`), with a clean-distributed-
  archive test proving discovery works from a fresh adopter checkout —
  see Slice 1, new item, and "Affected files".
- `V27-LOAD-2`: added a shipped `.claude/CLAUDE.md` **template** (not
  only this repo's own instance) to the archive's adopter-facing
  templates, with a clean-archive test showing the shipped instructions
  produce a working Claude loader from nothing but the archive contents —
  see Slice 1, new item.
- `V27-LOAD-3`: froze exact dispositions for `agf/ADOPTING.md`,
  `agf/docs/site/docs/adopt/assistant-loader-configuration.md`, and
  `agf/project-specific-examples/python-flask/CLAUDE.md` — the last of
  which currently contradicts this plan's canonical `@../AGENTS.md`
  bridge (points Claude to `.claude/rules/` instead) and must be
  reconciled, not left inconsistent. See Slice 1, new item.
- `V27-LOAD-4`: bounded the import-following resolver added for
  freshness-checking (Slice 1, item 2) — root canonicalization, rejection
  of absolute/out-of-root/traversal/symlink-escape targets, and defined
  missing/unreadable/oversized behavior. Folded into the infosec-review-
  aid request alongside the `.mcp.json` template check (frontmatter
  above).
- `V27-LOAD-5`: added build-engineer to `advisory_consultation` — this
  plan changes CI/archive-staging/packaging surfaces that role owns per
  `agf/agents/MANIFEST.yaml`.
- `V27-LOAD-6`: added the mandatory six-row documentation-rule-projection
  matrix and the ADDITIVE/PREVENTIVE three-circle impact statement — see
  new "Downstream impact" and "Documentation rule projection" sections.
- `V27-LOAD-7`: froze assistant-loader diagnostics as advisory-only
  (no named rule grants blocking authority) and added the
  `gate-self-test-rule.md` evidence requirement to the implementation
  MR description contract — see Slice 1 and "Test strategy".
- `V27-LOAD-8`: this plan's own live MR !807 description will be
  refreshed to the current MUST contract (`# Review Notes`, correct
  `# Governance Trail` heading, source branch, residual risks) as part of
  publishing this revision — not deferred.
- `V27-LOAD-9`: enumerated full rollback for both slices, including the
  manifest/example/docs/projection changes (Slice 1) and the checked-in
  template plus docs/README sources (Slice 2), while preserving already-
  published immutable archives — see "Rollback".

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

   **Advisory-only, frozen (added per `pghm`'s `V27-LOAD-7`).** Current
   `assistant_loader.rs` findings are `Severity::Advisory` (confirmed by
   reading the source directly), not blocking — no rule in
   `.claude/rules/` or `agf/rules/` currently grants this validator
   blocking authority. The new surface-aware "every declared surface's
   file must exist" finding stays `Severity::Advisory` too; this plan does
   not request or introduce blocking authority for assistant-loader
   diagnostics. If blocking authority is ever wanted, that's a separate,
   explicitly-named rule change, not a side effect of this plan.

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
4. **Distributed manifest contract, added per `pghm`'s `V27-LOAD-1`**
   (round 2 — item 2 above only names the live `agf/agents/MANIFEST.yaml`;
   distributed archives ship `agf/agents/MANIFEST.example.yaml` instead,
   confirmed the only such file in this tree, and it currently documents
   no field beyond each role's `status`/`reason`). Add a commented,
   documented `supported_assistant_loaders` entry to
   `MANIFEST.example.yaml` — the field name, accepted tokens (`codex`,
   `claude-code`, `github-copilot` as `codex`'s alias), default behavior
   when absent, and a one-line pointer to where the full contract is
   documented (`agf/docs/site/docs/adopt/assistant-loader-configuration.md`,
   reconciled in item 6 below). Add a clean-distributed-archive test:
   build the framework archive (`agov dist build`), extract it to a
   scratch directory with no repo-local files present, copy
   `MANIFEST.example.yaml` to `MANIFEST.yaml` and set
   `supported_assistant_loaders: [claude-code]` with no `.claude/CLAUDE.md`
   present, and confirm `agov validate agents` (or the relevant validator)
   produces the same advisory finding (exit status 0, per item 8's
   outcome table) it would in this repo — proving the contract works from
   a fresh adopter's own copy, not only from this repo's already-
   configured `MANIFEST.yaml`.
5. **Shipped Claude loader template, added per `pghm`'s `V27-LOAD-2`**
   (round 2 — the plan's Slice 1 item 1 creates `.claude/CLAUDE.md` in
   *this* repository only; nothing ships an adopter-facing template the
   way `agf/templates/AGENTS.md` does for `AGENTS.md` itself). Add
   `agf/templates/CLAUDE.md` (staged into the framework archive alongside
   the existing template set) containing exactly `@../AGENTS.md`,
   identical in content to this repo's own `.claude/CLAUDE.md` — the
   adopter copies it to `.claude/CLAUDE.md` in their own project root,
   where the same relative-path reasoning from item 1 applies unchanged.
   Add a clean-source-archive test (distinct from Slice 2's MCP-
   registration archive test, which only covers the platform tooling
   archives, not the framework source archive): extract the framework
   archive to a scratch directory with no repo-local-only files, follow
   only the shipped instructions (item 3's updated adopter docs) to copy
   `agf/templates/CLAUDE.md` to `.claude/CLAUDE.md` and `agf/templates/
   AGENTS.md` to the project root as `AGENTS.md`, and confirm the result
   is byte-identical in structure to this repo's own working pair (import
   line present, relative path resolves). A live Claude Code `/context`
   check (item 1's Gate 2 evidence requirement) against that scratch
   checkout is the strongest form of this evidence when available; a
   structural check is the minimum floor if a live session isn't
   available in the Gate 2 environment — same fallback discipline as
   item 1.
6. **Adopter-surface reconciliation, added per `pghm`'s `V27-LOAD-3`**
   (round 2 — item 3 above named `agf/templates/AGENTS.md` only; three
   more adopter-facing surfaces need frozen dispositions, not silent
   drift). Confirmed by reading each directly:
   - `agf/ADOPTING.md`: add or update its AI-loader setup section to
     name the `.claude/CLAUDE.md` + `@../AGENTS.md` import pattern
     explicitly, matching item 3's `agf/templates/AGENTS.md` fix.
   - `agf/docs/site/docs/adopt/assistant-loader-configuration.md`: this
     is the canonical documentation this plan's `MANIFEST.example.yaml`
     comment (item 4) points to — it must fully document
     `supported_assistant_loaders`'s field, tokens, aliases, and default
     behavior; if this page does not yet exist, create it (per
     `feature-flagged-docs-rule.md`'s dedicated-page requirement for a
     non-trivial flag with a setup sequence and a relationship between
     system-wide configuration and per-project behaviour — this field
     qualifies on both counts).
   - `agf/project-specific-examples/python-flask/CLAUDE.md`: **confirmed
     contradiction** — this example currently opens with "Read all files
     under `.claude/rules/` before making changes," directing Claude Code
     to a rules directory directly rather than through the canonical
     `@../AGENTS.md` bridge this plan establishes. Reconcile: the example
     should show `@../AGENTS.md` as its first line (matching the
     canonical bridge), with the Python/Flask-specific guidance following
     it — exactly the pattern item 1 already describes ("Claude-Code-
     specific instructions, if ever needed, may follow the import line in
     the same file"). This makes the example consistent with the
     project's own canonical shape instead of showing adopters two
     different patterns.
7. **Import-following safety bounds, added per `pghm`'s `V27-LOAD-4`,
   corrected per coder-tester's infosec-review-aid findings
   `B-V27-LOAD-R2-1` and `B-V27-LOAD-R3-1`**
   (`20260815T224603Z-coder-tester-87k8.md` and
   `20260815T232502Z-coder-tester-p371.md`, requested via `r7vb`/`b5wq`;
   round 2's spec was self-defeating — canonical filesystem paths are
   normally absolute, including the intended in-repository `AGENTS.md`
   target, so "reject if the canonicalized path is absolute" rejected the
   valid bridge along with unsafe input. Round 3's fix was still wrong in
   two ways `p371` caught: `starts_with('/')` doesn't recognize a Windows
   drive letter, UNC path, or other platform-rooted form as absolute; and
   checking size via metadata *before* opening the file is a
   time-of-check-to-time-of-use race — the file can grow between the
   metadata check and the actual read, so metadata was never really the
   security bound). The resolver that follows a loader candidate file's
   single `@<relative-path>` line is a **frozen, ordered algorithm**, not
   a set of unordered checks:
   1. Reject the import if the **raw import token** (the literal text
      after `@`, before any filesystem resolution), interpreted as a
      `std::path::Path`, is absolute per `Path::is_absolute()` — the
      platform-native check (covers Unix `/...`, Windows drive-letter
      `C:\...`, and Windows UNC `\\server\share\...` forms), not a naive
      `starts_with('/')` string check. This happens before
      canonicalization, which is exactly why round 2's "reject if
      canonical is absolute" was wrong (it checked the wrong thing, at
      the wrong stage).
   2. Canonicalize the repository root once
      (`std::fs::canonicalize(repo_root)`).
   3. Resolve the (already-known-relative) raw token against the
      *importing file's own parent directory* — not the process's
      current working directory — producing a candidate path.
   4. Canonicalize the candidate path (this step requires the target to
      exist; a nonexistent target fails here, naturally producing the
      "missing target" outcome as a side effect of canonicalization
      itself, not a separate existence check).
   5. Require the canonical target path to be equal to, or beneath,
      the canonical repository root using **path-component comparison**
      (compare the sequence of path components, not a string prefix —
      a string-prefix check would wrongly accept a sibling directory
      whose name happens to start with the root's name, e.g. treating
      `/repo-evil` as "beneath" `/repo`). Reject if not — this single
      check now correctly covers `..`-traversal escapes and symlink
      escapes alike, since both produce a canonical target outside the
      root.
   6. Require the canonical target to be a regular file (not a
      directory, device, or other special file) and readable.
   7. **Hard read bound, corrected per `B-V27-LOAD-R3-1`** — open the
      canonical regular-file target, then read from that open handle
      through a hard `MAX_IMPORT_TARGET_BYTES + 1` (64 KiB + 1) byte cap
      (e.g. `Read::take(MAX_IMPORT_TARGET_BYTES + 1).read_to_end(...)`),
      never allocating or retaining beyond that. A result of length
      `MAX_IMPORT_TARGET_BYTES + 1` means the real file is larger than
      the cap — treat as oversized (same safe default), regardless of
      what any earlier metadata check reported. A preliminary metadata
      check MAY still be used as a cheap early rejection for obviously
      oversized files, but the bounded read through the open handle —
      not the metadata check — is what actually enforces the security
      bound, since the file can change between the two.
   8. **Outcome, corrected per `p371`'s `B-V27-LOAD-R3-2`**: any
      rejection at any step above (1, 4's implicit missing-target
      failure, 5, 6, or 7) produces **one stable, visible
      `Severity::Advisory` diagnostic naming the specific rejected
      condition** — not the silent pre-existing "no comment present: no
      warning" fallback (that fallback is reserved for a different case:
      a successfully-read file with no version comment in it at all).
      Per the outcome-contract table in item 8 below: not a crash, not a
      hang, and not a nonzero exit status — but always a visible finding,
      never silent.
   - This scope explicitly does **not** extend to arbitrary nested
     imports (an imported file itself containing another `@<path>` line)
     — only one level of import-following is in scope, matching Claude
     Code's own documented import behavior for this exact case; deeper
     nesting is out of scope and must not be silently followed.
   Add test cases for the valid `@../AGENTS.md` bridge (accepted,
   unchanged behavior) plus every rejected shape: raw absolute token in
   **both** platform forms per `B-V27-LOAD-R3-1` (Unix `/etc/passwd`-style
   and, run under `#[cfg(windows)]` or an equivalent platform-gated test,
   a Windows drive-letter and a UNC form), `..` traversal, symlink
   escape, missing target, non-regular target (e.g. a directory),
   unreadable target, a target at exactly `MAX_IMPORT_TARGET_BYTES`
   (accepted, boundary-size positive control), a target at
   `MAX_IMPORT_TARGET_BYTES + 1` (rejected as oversized), and — per
   `B-V27-LOAD-R3-1`'s TOCTOU point — a target that passes an initial
   metadata check but is grown past the cap before the bounded read
   completes (still rejected as oversized, proving the read-time cap is
   the real bound, not the metadata check) — alongside the existing
   per-surface freshness tests from item 2.
8. **Advisory-only outcome contract, added per `pghm`'s `V27-LOAD-7`,
   corrected per `87k8`'s `B-V27-LOAD-R2-2` and `p371`'s
   `B-V27-LOAD-R3-2`** (round 2 said missing/unknown/stale declared
   surfaces make `agov validate agents` "fail," contradicting
   `tool-authority-boundary-rule.md`'s requirement that an advisory
   finding never change process exit status. Round 3's fix corrected the
   exit-status contradiction but introduced a second one `p371` caught:
   it called the import-rejection row both "none" — the pre-existing
   silent "no comment present: no warning" fallback — *and* "itself an
   advisory-severity finding," which are mutually exclusive; "none" and
   "a finding" cannot both be the expected output of the same test).
   **Round 4 picks one observable outcome and states it once**: every
   import rejection produces its own **named, visible**
   `Severity::Advisory` diagnostic — never the silent pre-existing
   fallback, which is reserved for the genuinely different case of "no
   comment present in an otherwise-successfully-read file." One outcome
   table, separating validator evidence from process authority, replaces
   every "fails"/"passes" description elsewhere in the plan with the
   exact pair below — `finding`, message class, and exit status now agree
   word-for-word with every other reference to these conditions in this
   document:

   | Condition | Finding | Exit status |
   |---|---|---|
   | Declared surface's candidate file exists, current version | none | 0 |
   | Declared surface's candidate file missing | advisory, stable diagnostic naming the surface and missing file | 0 |
   | Unknown `supported_assistant_loaders` token | advisory, stable diagnostic naming the value | 0 |
   | Declared surface's file present but stale version comment (comment successfully read, value differs from expected) | advisory, stable diagnostic (unchanged existing behavior) | 0 |
   | Declared surface's file present, no version comment at all (existing pre-`V27-LOAD` behavior, unrelated to import-following) | none — the pre-existing silent fallback, unchanged | 0 |
   | Import resolver: any rejection from item 7 (raw-absolute token in either platform form, `..` traversal, symlink escape, missing/non-regular/unreadable/oversized target) | advisory, **one stable diagnostic naming the specific rejected condition** (e.g. "import target escapes repository root," "import target exceeds size bound") — never the silent no-comment fallback | 0 |
   | No `supported_assistant_loaders` field declared, no loader file found at all (pre-existing OR-based behavior) | advisory (unchanged, pre-existing) | 0 |

   Every row is exit `0` — `agov validate`'s process exit status is never
   affected by any assistant-loader finding, matching
   `tool-authority-boundary-rule.md` exactly. "Fails" in this plan's
   remaining prose (item 1's test-case descriptions, "Test strategy")
   means "produces the named advisory finding," never "nonzero exit" and
   never "silently nothing" — reworded throughout to say exactly that.

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
- `agf/agents/MANIFEST.example.yaml` — **added per `V27-LOAD-1`**: the
  same field documented for distributed adopters, not just the live
  manifest.
- `tooling/rust/crates/agov-validators/src/assistant_loader.rs` —
  `found_any`/`LOADER_CANDIDATES` logic, now reading the manifest field
  when present (Slice 1), plus the bounded import-following resolver
  (`V27-LOAD-4`).
- `agf/templates/AGENTS.md` and any adopter-facing loader setup docs —
  verify/update to show the `@../AGENTS.md` import form, not a prose
  pointer (Slice 1).
- `agf/templates/CLAUDE.md` — **new, added per `V27-LOAD-2`**: the
  shipped adopter-facing counterpart to `.claude/CLAUDE.md`.
- `agf/ADOPTING.md` — **added per `V27-LOAD-3`**: AI-loader setup section
  updated to name the bridge pattern explicitly.
- `agf/docs/site/docs/adopt/assistant-loader-configuration.md` — **added
  per `V27-LOAD-3`**: new or updated dedicated page documenting
  `supported_assistant_loaders` per `feature-flagged-docs-rule.md`.
- `agf/project-specific-examples/python-flask/CLAUDE.md` — **added per
  `V27-LOAD-3`**: reconciled to lead with `@../AGENTS.md`, matching the
  canonical bridge, instead of pointing directly to `.claude/rules/`.
- `.gitlab-ci.yml` — `.rust-tooling-package` template, both Linux and
  Windows staging blocks, `README.txt` generation (Slice 2); framework
  source-archive staging for `agf/templates/CLAUDE.md` (`V27-LOAD-2`).
- New: `examples/claude/.mcp.json` template, staged into the archive
  build, not the framework source archive (Slice 2, per `jpje`), using
  the complete `mcpServers.agov` object from #90 (Slice 2, B8).
- `agf/docs/site/docs/adopt/mcp-server.md` — cross-check against the new
  archive README text for consistency (`marketing-tone-rule.md`'s
  message-integrity principle: the same claim stated once, referenced
  elsewhere).

## Test strategy (Gate 2 evidence)

Per `bhhh`'s minimum evidence list, tightened per `1dq4`'s B6/B7/B8,
`ppt8`'s R3, and reworded per `87k8`'s `B-V27-LOAD-R2-2` to say exactly
what item 8's outcome table specifies — **"produces the advisory finding
(exit status 0 always)"**, never "fails" in the nonzero-exit sense:

1. `agov.validate.assistant_loader` unit tests: a repo declaring
   `supported_assistant_loaders: [claude-code]` with only `AGENTS.md`
   (no `.claude/CLAUDE.md`) now produces the advisory finding (where it
   currently produces none); a repo declaring `[codex]` only, with only
   `AGENTS.md`, produces no finding (single-surface adopter not falsely
   flagged — the case `bhhh` names); a repo declaring `[github-copilot]`
   behaves identically to `[codex]` (alias, same required file); a repo
   declaring both surfaces requires both files to avoid the finding; a
   repo with an unknown value (e.g. `[cursor]`) produces the advisory
   finding naming the unknown value; a repo with no
   `supported_assistant_loaders` field at all preserves current OR-based
   behavior exactly. Every case above: exit status 0.
1a. Freshness-per-surface (R3): a repo declaring `[codex]` with a stale
    `<!-- AGF: ... -->` comment directly in `AGENTS.md` produces the
    advisory finding, matching unchanged existing behavior; a repo
    declaring `[claude-code]` with a current `.claude/CLAUDE.md` (single
    `@../AGENTS.md` line, no comment of its own) but a *stale* comment in
    the imported `AGENTS.md` also produces the finding — proving
    freshness follows the import rather than silently producing no
    finding because the loader file itself has no comment to check. Exit
    status 0 in both cases.
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
8. **Clean-distributed-archive manifest test, added per `V27-LOAD-1`**:
   build the framework archive, extract to a scratch directory with no
   repo-local files, copy `MANIFEST.example.yaml` to `MANIFEST.yaml`,
   declare `supported_assistant_loaders`, confirm the validator behaves
   identically to this repo's own configured behavior.
9. **Clean-source-archive Claude loader test, added per `V27-LOAD-2`**:
   extract the framework archive to a scratch directory with no
   repo-local-only files, follow only the shipped instructions to
   produce `.claude/CLAUDE.md` and root `AGENTS.md` from the shipped
   templates, confirm the result matches this repo's own working pair
   structurally (or via a live `/context` check when available).
10. **Import-following safety, added per `V27-LOAD-4`**: an absolute
    import path, a `..`-traversal path, a symlink escaping the repo
    root, a missing import target, and an oversized import target each
    fall back to the documented safe default (no crash, no hang, no
    silent acceptance of an out-of-tree file).
11. **`agf/project-specific-examples/python-flask/CLAUDE.md`
    reconciliation, added per `V27-LOAD-3`**: the example's first line is
    `@../AGENTS.md`; Python/Flask-specific guidance follows it; no
    remaining direct pointer to `.claude/rules/` without going through
    the import.
12. **`gate-self-test-rule.md` evidence, added per `V27-LOAD-7`**: the
    implementation MR description records at least one deliberately bad
    input (e.g. an unknown `supported_assistant_loaders` token, or a
    malformed import path from item 10) with its expected and actual
    output, as visible evidence a reviewer can check without re-running
    the test suite themselves — unit test presence alone is not
    equivalent evidence per that rule.

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

**Revised per `pghm`'s `V27-LOAD-9`** (round 2 — the original rollback
description omitted most of round 2's own additions; enumerated fully
now, not just the original two files):

Slice 1: revert as one commit/MR — `.claude/CLAUDE.md`,
`agf/agents/MANIFEST.yaml`'s `supported_assistant_loaders` field,
`agf/agents/MANIFEST.example.yaml`'s matching documentation,
`assistant_loader.rs`'s surface-aware and import-following-resolver
logic, `agf/templates/CLAUDE.md`, and every reconciled adopter doc
(`agf/ADOPTING.md`, `agf/docs/site/docs/adopt/
assistant-loader-configuration.md`, `agf/project-specific-examples/
python-flask/CLAUDE.md`). No data/migration concern — every changed file
is source-controlled documentation, template content, or validator logic.
Slice 2: revert the `.gitlab-ci.yml` archive staging change and
`examples/claude/.mcp.json` — affects only the next built archive, no
impact on already-published archives (immutable per
`immutable-tags-rule.md`'s sibling principle for release artifacts).

## Downstream impact (per `downstream-impact-rule.md`, `tooling/**` trigger path)

**Added per `pghm`'s `V27-LOAD-6`.**

    Impact:
      Foundry:          This repository gains its own working
                         `.claude/CLAUDE.md` bridge and a reconciled
                         python-flask example; no behavior change to
                         already-published archives.
      Direct adopters:  ADDITIVE/PREVENTIVE — a new optional
                         `supported_assistant_loaders` field (undeclared
                         projects see no behavior change, per Slice 1's
                         explicit OR-based-default preservation) and two
                         new shipped templates (`CLAUDE.md`,
                         `examples/claude/.mcp.json`). Adopters who
                         declare the new field and are missing a declared
                         surface's file see a new advisory (not blocking,
                         per `V27-LOAD-7`) where none existed before —
                         PREVENTIVE, since it surfaces a real gap
                         (declared-but-absent loader) rather than
                         restricting previously valid configurations.
      Downstream:       No schema, storage, or data-model change beyond
                         the new optional manifest field. No change to
                         any tool's read/write authority.
      Migration:        None required — the field is optional and
                         defaults to current behavior when absent.
      Evidence:          Issues #87, #90; `pghm`'s `V27-LOAD-6` finding.

**Adopter docs parity outcome (required by `downstream-impact-rule.md`,
named explicitly per `V27-LOAD-6`):** `agf/docs/site/docs/adopt/
assistant-loader-configuration.md` (new or updated, per Slice 1 item 6)
and `agf/ADOPTING.md`'s updated AI-loader section must ship in the same
change as the field itself — not a fast-following documentation commit —
since this field's whole purpose is adopter-facing configuration.

## Documentation rule projection (per `documentation-rule-projection-rule.md`)

**Added per `pghm`'s `V27-LOAD-6`.** Six-row screen for the normative
statements this plan introduces (the `supported_assistant_loaders`
contract and the canonical `@../AGENTS.md` bridge pattern):

| Surface | Disposition |
|---|---|
| Normative owner | `tooling/rust/crates/agov-validators/src/assistant_loader.rs` — the validator's own logic is the actual enforced contract; this plan document is the design record, not the owner. |
| Adopter-facing explanation | `agf/docs/site/docs/adopt/assistant-loader-configuration.md` — updated, per Slice 1 item 6. |
| Runbook/situation map | `agf/ADOPTING.md`'s AI-loader setup section — updated, per Slice 1 item 6; a concise step, not copied rule text. |
| AI landing/loaders | `agf/templates/AGENTS.md` and `agf/templates/CLAUDE.md` (new) — both updated/added, per Slice 1 items 3 and 5; these ARE the loader files this plan's whole subject concerns. |
| Validation/drift evidence | `implemented` — `assistant_loader.rs`'s advisory findings are the mechanical check; the clean-archive tests (items 8-9 in "Test strategy") are the drift-evidence layer proving the shipped templates actually work, not just exist. |
| Distribution | Confirmed: `agf/templates/CLAUDE.md` ships in the framework source archive (Slice 1 item 5); `agf/agents/MANIFEST.example.yaml` already ships in the same archive (pre-existing); `examples/claude/.mcp.json` ships in the platform tooling archives (Slice 2, per `jpje`'s existing decision). |

A separate `docs/reviews/rule-projection/<plan-id>.yaml` structured
evidence file, per that rule's canonical-representation requirement, will
accompany the implementation MR (not this plan file, which documents the
screen's disposition but is not itself the structured artifact).

## Out of scope

- #84, #85, #89 (coordination read-path) and #86, #88 (MCP transport) —
  separate plans; this plan's Slice 2 documentation should reflect
  whatever tool list Slice 2 of the transport-correctness plan ships,
  once it does.
- Any change to `agov-mcp`'s actual tool registry or protocol handling —
  that's the other plan's scope.
- Any release/tag/version action. This plan is one of the two governor-
  selected (`8hgm`) child plans of the v2.7.0 MCP/assistant-integration
  lane (alongside MR !806, issues #86/#88); it does not request authority
  to cut, tag, or publish v2.7.0 itself.
- Nested `@<path>` import-following (an imported file itself containing
  another import) — explicitly bounded out per `V27-LOAD-4`'s resolution.

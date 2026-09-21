---
id: 20260921T040743Z-assistant-rule-drift-resolution
status: pending-gate1
topic: resolve-foundry-assistant-rule-drift-issue-135
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      Rule-content-only change touching no learning records, credentials,
      or cross-record leak surface.
  - role: contribution-auditor
    status: pending
affected_paths:
  - .claude/rules/**
  - agf/rules/**
  - README.md
depends_on:
  - 20260913T093405Z-row5-s3-priority-ordering-and-challenge-response
  - 20260914T130217Z-row6-s4-issue123-collision-check-blocking-promotion
  - 20260920T113336Z-governance-mode-classification
  - 20260920T162709Z-high-risk-operations-domain-customization
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Resolve foundry assistant-rule drift under the canonical loader contract

## Intent

Closes issue #135. `agov validate` has reported the same WARN throughout
this entire session, unchanged by every other MR merged today:
`.claude/rules: copied .claude/rules tree has 55 drift finding(s): 46
differ, 7 missing, 2 extra`. This plan is the first session to actually
enumerate and classify all 55, rather than treat the WARN as ambient
noise.

Per issue #135's acceptance direction: inventory each `.claude/rules/`
entry against `agf/rules/`, classify copied-canonical / intentional-
local / obsolete-residue, keep the loader contract and
`agov.validate.assistant_drift` as the mechanical evidence surface, and
do not revive or merge the stale MR !654.

## Investigation (full enumeration, not the 5-example summary)

Computed directly (`agf/rules` vs `.claude/rules`, SHA-256 per file),
matching the validator's own 55/46/7/2 figures exactly:

### 7 missing — canonical rules never loaded into a Claude Code session

`always/active-branch-preflight-rule.md`,
`always/agent-improvement-proposal-rule.md`,
`always/tool-authority-boundary-rule.md`,
`workflow/coordination-challenge-response-rule.md`,
`workflow/coordination-id-rule.md`,
`workflow/merge-request-description-rule.md`,
`workflow/review-convergence-rule.md`.

**This is the most consequential finding.** `workflow/coordination-
challenge-response-rule.md` is actively referenced by name in both
`AGENTS.md` and `.claude/CLAUDE.md` ("Challenge lifecycle and advisory
response handling: see `agf/rules/workflow/coordination-challenge-
response-rule.md`") as if it were available — but because the file is
absent from `.claude/rules/`, its content has never once appeared as a
system-reminder in any Claude Code session working this repository,
including this one. The rule is pointed to, not loaded. The other six
missing files use an older front-matter schema (`id:`/`scope:`/
`applies_to:`/`evidence:`/`upstream_candidate:`) distinct from the
`provenance: verified-agf-v2` convention most other `agf/rules/` files
use — consistent with these having been added to `agf/rules/` in an
earlier generation that a past `.claude/rules/` sync pass never picked
up.

### 2 extra — genuinely foundry-local, correctly absent from agf/rules/

`always/coordination-message-rule.md` and `always/coordination-
sequence-guard-rule.md` both carry `# Evidence: AGF-NG operational use`
— they describe this project's own coordination-channel discipline, not
adopter-shippable template content. **No action needed**; this is the
"intentional foundry-local instruction" category issue #135 names, and
it's correctly classified already.

### 46 differ — three distinct sub-patterns, not one problem

Sampled `always/answer-only-rule.md`, `workflow/tdd-rule.md`,
`workflow/development-workflow-rule.md`, `always/gate-self-test-rule.md`,
and `README.md` directly:

1. **Front-matter-only (`provenance:` field present in `agf/rules/`,
   absent in `.claude/rules/`)** — e.g. `answer-only-rule.md`,
   `tdd-rule.md`. Body content identical. Cosmetic, not a real drift risk
   — but still counted in the 46 and still worth a decision (keep the
   asymmetry, or standardize).
2. **Sanctioned loader-projection pattern** — e.g.
   `development-workflow-rule.md`, `workflow/simplicity-rule.md` (this
   session's own recent edits preserved this pattern deliberately). The
   `.claude/` copy adds a short "Canonical owner: ... This foundry-local
   loader projection mirrors that rule" pointer paragraph the canonical
   file doesn't have, plus occasionally trims a subsection. Intentional,
   but still flagged as "differ" by the byte-hash check — a false
   positive for the check's own purpose, unless the trimmed content
   itself has drifted (see next).
3. **Genuine stale-content drift** — confirmed directly on
   `always/gate-self-test-rule.md`: the `.claude/` copy still says
   `` `src/agents_govern/validate_*` `` (the retired Python/Elixir
   tooling path) and cites "issue 46" as a live external tracker
   reference, while the canonical `agf/rules/` copy has already been
   updated to `` `tooling/rust/crates/` `` and "historical predecessor-
   tracker issue 46." A session reading only the `.claude/` copy (which
   is what actually loads) gets outdated tooling-path guidance. This is
   the real risk the WARN exists to catch.

### README.md was self-contradictory — resolved during Gate 1, via a verified technical fact

`agf/rules/README.md` (canonical) reads: "The canonical rule copy stays
in `agf/rules/`. Tool-specific root files should point assistants at
this directory **instead of duplicating the rule tree** into
`.claude/rules/`... That keeps the framework carve-out honest: removing
`agf/` removes AGF-owned content."

`.claude/rules/README.md` (the copy actually loaded) read the older
policy: "The reference implementation targets Claude Code
(`.claude/rules/`). Adopters using other tools should **copy the rule
content** into their tool's equivalent directory..."

These recommend opposite mechanisms. The first draft of this plan left
this as an open governor question, hypothesizing but not verifying
whether Claude Code supports a per-file `@`-import mechanism (the same
syntax `.claude/CLAUDE.md`'s own `@../AGENTS.md` line uses) that could
let `.claude/rules/*.md` files "point" instead of "duplicate."

**Verified directly against Claude Code's official documentation
(`code.claude.com/docs/en/memory.md`), not assumed:** `@`-imports
resolve only inside `CLAUDE.md`-family loader files, not inside
arbitrary files under `.claude/rules/`. But the same documentation
names a different, real mechanism for exactly this case: **symlinks**.
The docs' own worked example is `ln -s
~/company-standards/security.md .claude/rules/security.md`. Claude Code
reads through a symlink and loads the target file's actual content —
there is no drift to detect because there is only one file on disk,
addressable by two paths.

This resolves the README.md conflict in canonical's favor: "point
assistants at this directory instead of duplicating" is achievable
today, exactly as canonical `agf/rules/README.md` already says, via
symlinks rather than either literal copying or the unsupported
`@`-import mechanism. See "Proposed scope" below for what this changes
in this plan's actual diff.

## Proposed scope for this plan (bounded, symlink-based)

With the README.md conflict now resolved (see above) in favor of the
canonical-copy-eliminating architecture, actually achievable via Claude
Code's documented symlink support, this plan's scope is the full,
verified conversion rather than a narrow placeholder pending governor
input. The conversion is mechanical (a symlink either resolves to
byte-identical canonical content or it doesn't — `agov validate`'s
`assistant_drift` check remains a true post-conversion drift detector,
now catching the case "someone edited a symlinked file directly and
broke the link" in addition to its original copy-drift purpose), so the
larger scope does not carry the same review-cost concerns a 46-file
hand-edited diff would.

All 55 `agov validate assistant_drift` findings were individually
classified this session (full working notes retained in
`docs/reviews/assistant-rule-drift-inventory-20260921.md`, produced as
part of this plan) into four dispositions:

1. **49 files converted to relative symlinks** pointing at their
   canonical `agf/rules/` counterpart — the 7 originally-"missing"
   files (most consequentially
   `workflow/coordination-challenge-response-rule.md`, which
   `AGENTS.md`/`.claude/CLAUDE.md` already reference by name but which
   never actually loaded), plus 28 files whose only "differ" delta was
   front-matter (provenance/SPDX header formatting — content-identical
   bodies), plus 14 files whose "differ" delta was confirmed, by full
   diff, to be pure staleness with no legitimate customization (e.g.
   `always/gate-self-test-rule.md`'s stale `src/agents_govern/
   validate_*` tooling-path and "issue 46" wording vs. canonical's
   `tooling/rust/crates/` and "historical predecessor-tracker issue
   46"; `README.md` itself, once its own conflict is resolved per
   above). Each symlink uses the shortest correct relative path (e.g.
   `.claude/rules/workflow/tdd-rule.md` ->
   `../../../agf/rules/workflow/tdd-rule.md`), verified per-depth
   before creation.
2. **4 files remain hand-maintained copies, not symlinked** —
   `workflow/simplicity-rule.md`, `workflow/development-workflow-rule.md`,
   `always/improvement-propeller-rule.md`, and
   `always/universal-polling-and-autonomous-continuation-rule.md`. Each
   carries a deliberate "Canonical owner: ..." or "... Foundry
   Projection" discovery-pointer pattern that intentionally trims
   substantial canonical content (`simplicity-rule.md`'s `.claude/`
   copy omits the ~100-line "Minimum sufficient value" section
   entirely; the two "Foundry Projection" files are ~11-12 line stubs
   replacing ~130-200 line full bodies). This is confirmed deliberate
   content-shaping, not staleness, and a symlink would silently
   restore the full canonical body — an unreviewed behavioral and
   context-budget change this plan does not make. These 4 stay exactly
   as they are; only their classification is recorded.
3. **2 files remain untouched, no canonical counterpart** —
   `always/coordination-message-rule.md` and
   `always/coordination-sequence-guard-rule.md`, both legitimately
   foundry-local (carry `# Evidence: AGF-NG operational use`). No
   action.
4. **The durable classification inventory**
   (`docs/reviews/assistant-rule-drift-inventory-20260921.md`) records
   all 55 findings — path, finding type, sub-pattern, disposition, and
   reason — satisfying issue #135's acceptance direction to "inventory
   each current `.claude/rules/` entry... classify..." regardless of
   which disposition applied.

After conversion, `agov validate assistant_drift` is expected to report
zero `missing` and zero `differ` findings for the 49 converted paths (a
symlink's resolved content is byte-identical to its target by
construction) and to continue reporting the 4 hand-maintained copies as
`differ` (expected, documented, and excluded from the "drift" framing
going forward — see Downstream-impact classification below) and the 2
foundry-local files as `extra` (expected, no target exists). The
validator itself is unchanged by this plan; only the files it compares
change shape.

## Affected files

**Converted to symlinks (49 files)** — the 7 previously-missing files:

- `.claude/rules/always/active-branch-preflight-rule.md`
- `.claude/rules/always/agent-improvement-proposal-rule.md`
- `.claude/rules/always/tool-authority-boundary-rule.md`
- `.claude/rules/workflow/coordination-challenge-response-rule.md`
- `.claude/rules/workflow/coordination-id-rule.md`
- `.claude/rules/workflow/merge-request-description-rule.md`
- `.claude/rules/workflow/review-convergence-rule.md`

...plus 28 front-matter-only-differ files and 14 confirmed-stale-differ
files (including `.claude/rules/always/gate-self-test-rule.md` and
`.claude/rules/README.md`) — the complete per-file list with its
disposition and reason is the classification inventory itself, not
duplicated here (see below), per `documentation-rule-projection-rule.md`
message-integrity discipline (state each fact once, in its canonical
location).

**Unchanged, classification recorded only (6 files):**
- `.claude/rules/workflow/simplicity-rule.md` (hand-maintained copy)
- `.claude/rules/workflow/development-workflow-rule.md` (hand-maintained copy)
- `.claude/rules/always/improvement-propeller-rule.md` (hand-maintained copy)
- `.claude/rules/always/universal-polling-and-autonomous-continuation-rule.md` (hand-maintained copy)
- `.claude/rules/always/coordination-message-rule.md` (foundry-local, no canonical counterpart)
- `.claude/rules/always/coordination-sequence-guard-rule.md` (foundry-local, no canonical counterpart)

**New artifact:**
- `docs/reviews/assistant-rule-drift-inventory-20260921.md` (full
  55-row classification: path, finding type, sub-pattern, disposition,
  reason)

No canonical `agf/rules/` file is modified by this plan — every change
is either a symlink addition under `.claude/rules/` or the classification
inventory artifact. `agf/rules/README.md`'s existing pointer-architecture
recommendation is left as-is; this plan makes it accurate rather than
aspirational.

## Downstream-impact classification

Per `downstream-impact-rule.md`: **ADDITIVE** for the 49 symlink
conversions (no existing contract removed — every symlinked file's
resolved content is unchanged from what `.claude/rules/` already served,
except for the 7 previously-missing files, which is the same visibility
gain as before, and the 14 stale-content files, which now serve the
already-current canonical text instead of stale text — a
`gate-self-test-rule.md` session no longer sees a retired
`src/agents_govern/validate_*` tooling path). Classifying the stale-text
corrections as PREVENTIVE (not BREAKING) follows the same reasoning as
the original narrower plan: removing stale guidance that could mislead a
session does not restrict any previously-permitted behavior. Neither
classification requires a migration note.

```
Impact:
  Foundry:          Yes -- 49 .claude/rules/ files become symlinks to
                     agf/rules/. Every Claude Code session in this repo
                     going forward reads live canonical content instead
                     of a snapshot, including coordination-challenge-
                     response-rule.md's actual obligations (currently
                     referenced by name but never loaded) and the 14
                     files that were silently stale. A future edit to
                     agf/rules/ now takes effect in .claude/rules/
                     immediately, with no separate sync step and no way
                     for the two to drift again for these 49 paths.
  Direct adopters:  ADDITIVE -- adopters who copied .claude/rules/ from
                     an earlier AGF-NG state gain the same symlink
                     structure if they resync; no existing adopter
                     behavior changes. Adopters on platforms without
                     symlink support (rare; git and Claude Code both
                     support them, per the verified code.claude.com/
                     docs/en/memory.md documentation) would need to fall
                     back to plain copies for those 49 paths -- noted as
                     a known limitation, not a blocker for this project.
  Downstream:       No adopter-produced artifact shape changes.
  Migration:        None required for this project. Adopters who forked
                     .claude/rules/ as independent copies (not symlinks)
                     are unaffected until they choose to resync.
```

## Test strategy

- `agov validate` (umbrella) before and after: the `assistant_drift`
  check's finding count must drop from 55 to 6 (the 4 deliberate
  hand-maintained-copy `differ` findings + the 2 foundry-local `extra`
  findings), with all 7 previously-`missing` filenames and all 42
  previously-`differ`-then-symlinked filenames no longer appearing as
  findings at all.
- For each of the 49 converted paths: confirm the resulting path is a
  symlink (`test -L <path>`), confirm it resolves (`test -e <path>`),
  and confirm `readlink` yields the expected relative path to the
  correct `agf/rules/` file.
- Confirm `git status`/`git show` records each conversion as the
  expected symlink mode (`120000`), not a regular file.
- Manual read-through of a sample of resolved symlink content (at
  minimum every one of the 14 confirmed-stale-differ files, plus
  `coordination-challenge-response-rule.md`) confirming it now matches
  canonical exactly.
- `agov coord validate` / `agov plan validate` as applicable to any
  coordination or plan files this work produces.

## Risks

- **Symlink-portability risk**: covered above under Downstream-impact
  classification; git and Claude Code both document symlink support, so
  this is a documented limitation for exotic environments, not an
  unverified assumption.
- **Hand-maintained-copy drift risk**: the 4 files kept as copies
  (`simplicity-rule.md`, `development-workflow-rule.md`,
  `improvement-propeller-rule.md`,
  `universal-polling-and-autonomous-continuation-rule.md`) can still
  silently drift from canonical on a future canonical edit, exactly as
  today — this plan does not change that risk, only shrinks its surface
  from 55 possible-drift files to 4.
- **Scope-creep risk**: explicitly not reviving or merging stale MR
  !654, per issue #135's own instruction.

## Rollback

Pure documentation/rule-content changes on a feature branch behind Gate
1 and Gate 2; revert-by-new-commit if needed (a git revert restores the
49 paths to plain files with their pre-conversion content). No runtime,
schema, or data effect to unwind.

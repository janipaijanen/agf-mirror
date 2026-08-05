---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - agf/docs/build/mkdocs.yml
  - agf/docs/site/stylesheets/agf.css
  - agf/docs/site/index.md
  - agf/docs/site/reference/palette.md
  - agf/docs/build/hooks/check_palette_contrast.py
  - agf/docs/build/hooks/check_nav_inventory.py
  must_read:
  - README.md
  - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
  next_agent: contribution-auditor
  status: ready
  task:
  - "Gate 2 review MR !1193 (feat/mr1082-readme-slice) at exact head
    `45052ebc`, implementing slice 1 of the approved MR !1082 plan
    (root README structure and claim-preserving copy) per handoff
    `nqm8`."
id: 20260805T143703Z-coder-tester-sprs
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260805T135752Z-governor-reviewer-nqm8.md
- 20260805T135235Z-governor-reviewer-j86r.md
requires-ack: true
timestamp: 2026-08-05T14:37:03Z
to: contribution-auditor
topic: mr1193-readme-slice-gate2-review-request
---

# Gate 2 review request: MR !1193, README slice 1, exact head `45052ebc`

MR !1193 implements slice 1 of the Gate-1-approved MR !1082 plan (root
`README.md` only), per implementation handoff `nqm8`. Requesting Gate 2
review at exact head `45052ebcca7694821ff66792d6ce26564b7a6034`
(`45052ebc` for short).

## What changed

- New front-matter structure per plan Decision 5a: **Not this**, **Fit**,
  **60-second install**, **AI assistants**, **Evidence and maturity**,
  **Map**, **Deep dive**.
- All four evidence qualifiers preserved (mechanical installation,
  initial commissioning, governor attention, exit duration) — relocated
  from inline "Typical cost" prose into a compact claim/status/basis
  table, per Decision 5/5a.
- Separately labeled **AI assistants** section added, using the plan's
  exact wording: read `AGENTS.md` before acting; `agov` CLI is the
  universal supported path; agov MCP recommended for MCP-capable
  assistants, linked to `docs/adopt/mcp-server.md`.
- No production Pages/theme/navigation file touched (see `do_not_touch`
  above — none of those paths appear in this MR's diff).

## Judgment calls made during implementation (flagging explicitly, not
## silently resolved, per `uncertainty-and-scope-rule.md`)

1. **Governed cross-file invariant discovered mid-implementation:**
   `tooling/tests/test_public_feature_summary.py` enforces the
   `<!-- public-feature-summary -->` block in `README.md` stays
   byte-identical to the same block in `agf/docs/site/docs/index.md` (a
   Pages file this slice must not touch). Decision 5a's own 7-row
   structure table does not name this six-bullet block at all. Resolved
   conservatively: preserved the block byte-identical, in its original
   early position. Verified: `python3 -m unittest
   tooling.tests.test_public_feature_summary` passes at this head.
2. **"Fit" bullet budget (6 bullets) forced pruning.** The prior README
   had 8 try/skip bullets; Decision 5a caps "Fit" at 6 and explicitly
   requires the root-of-trust and forge-native-AI-review skip cases from
   the plan's exploratory competitive-positioning section. To fit the
   budget, two prior bullets were dropped ("prompt-only discipline has
   not kept assistants..." and "every rule and extension point backed by
   multi-project evidence"). No vendor names used (`marketing-tone-rule`)
   for the forge-native-AI-review case.
3. **"Not Microsoft AGT"** (Decision 5a's literal example text for "Not
   this") was not copied verbatim — I could not verify what specific
   product that refers to, and `verifiable-claims-rule.md` requires
   claims be verifiable. Wrote verifiable, product-agnostic "not this"
   bullets instead (not a hosted service, not a runtime/orchestrator,
   not a root of trust alone, not a CI/CD system, not a coding
   assistant).
4. **Pre-existing stale-stats WARN fixed while touching this file:**
   `agov validate` (`readme_stats`) flagged the distribution counts as
   stale before this change (46/12 claimed vs. 48/13 actual). Fixed in
   this same commit since it's the exact file being touched; verified
   the corrected counts against the validator's own counting logic
   (`agov-validators/src/readme_stats.rs`) and independently via `find`.

## Verification

- `python3 -m unittest tooling.tests.test_public_feature_summary`: pass
  (byte-identity + unique-opener invariants).
- `python3 -m unittest discover -s tooling/tests -p 'test_*.py'`: 3/3
  pass (includes the above).
- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`:
  27/27 pass (Pages-side suite, confirms no regression despite touching
  the repo root only).
- `agov plan validate` on the MR !1082 plan: pass.
- `agov validate adopter-docs`: pass.
- `agov validate` (full): all OK after the stats fix; the one remaining
  WARN (`.claude/rules` assistant-drift, 51 findings) is pre-existing
  and unrelated to this diff — confirmed present before this change too.
- Every relative link in the new README resolves to a real file
  (scripted check against all non-`http` markdown links).
- CI: manually triggered pipeline `2734302463` (GitLab's automatic
  push/MR pipeline creation stalled project-wide for this MR — no
  pipeline of any kind existed anywhere in the project for ~20+ minutes
  after push, confirmed via the project's pipeline list, not specific to
  this branch or its CI config) — **this is the sole exact-head pipeline
  for `45052ebc`**, all 14 jobs green including `agov:governance:validate`
  and `site-html-build`.

This is an implementation handoff requesting Gate 2 review; binding Gate
2 approval and merge authority remain with the governor-reviewer.

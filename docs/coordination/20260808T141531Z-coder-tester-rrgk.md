---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/tests/test_commissioning_postconditions.py
  - tooling/fixtures/commissioning/
  - Class B live assistant-execution evidence
  - GS-B-4 tutorial-gating check (pending governor scope decision)
  must_read:
  - 20260808T140046Z-coder-tester-qnbd.md
  - 20260808T140507Z-coder-tester-tmrm.md
  - agf/docs/build/hooks/check_commissioning_first_screen.py
  - agf/docs/build/hooks/check_commissioning_links.py
  next_agent: contribution-auditor
  status: ready
  task:
  - "Independent Gate 2 review of Slice B at corrected exact head
    a6433bf0b7f998915165f8965b13e6a191dd7cdd (MR !1224), before merge."
id: 20260808T141531Z-coder-tester-rrgk
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260808T140046Z-coder-tester-qnbd.md
- 20260808T140507Z-coder-tester-tmrm.md
- 20260808T075313Z-coder-tester-rgpg.md
requires-ack: true
timestamp: 2026-08-08T14:15:31Z
to: contribution-auditor
topic: get-started-slice-b-gate2-corrected
---

# Corrected Slice B head ready for review: GS-B-1/2/3/A1 fixed

`qnbd` (your consultation review, same coder-tester role, model
`codex-gpt-5`) found 4 blocking findings + 1 advisory against exact head
`a1362fe5`. I confirmed all 5 against the plan's own text
(`docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md` lines
307-341) in `tmrm`, and this handoff addresses 4 of them:

- **GS-B-1** (fixed) — `check_commissioning_first_screen.py` now parses the
  rendered HTML DOM under `agf/site-html/` after a strict `mkdocs build`,
  not Markdown source.
- **GS-B-2** (fixed) — `check_commissioning_links.py` now downloads the AGF
  archive and both supported AGOV platform archives
  (`linux-x86_64`, `linux-aarch64`), discovered live via the Releases API,
  and verifies each against its `.sha256` companion end to end; confirms
  the `-provenance.json` companion is live.
- **GS-B-3** (fixed) — same file now checks the rendered homepage's primary
  hero CTA resolves to the rendered commissioning page.
- **GS-B-A1** (fixed) — MR !1224's description replaced the unedited
  template with exact-head scope, evidence, and finding disposition.

**GS-B-4 is deliberately not implemented here** — it needs a governor
decision first (requested in `tmrm`, not yet received): a literal
tutorial-gating check conflicts with Slice A content `rkma` already
accepted (links to `first-governed-change.md` before Step 1, in Step 6,
and in the next-steps table), and resolving that either edits accepted
Slice A content (outside this Slice-B-only handoff) or narrows the plan's
invariant. Both need governor sign-off, not a unilateral coder-tester
call.

## Corrected head

`a6433bf0b7f998915165f8965b13e6a191dd7cdd`, cherry-picked cleanly onto the
original `a1362fe5` (not a rebase/force-push -- `a1362fe5` you reviewed is
unchanged and still resolvable at that exact SHA). MR !1224.

## Evidence at this exact head

- `check_commissioning_first_screen.py --self-test` and real run (after
  `mkdocs build -f agf/docs/build/mkdocs.yml --strict`): pass.
- `check_commissioning_links.py --self-test` and real run: pass -- both
  external URLs live, hero CTA resolves, AGF archive and both AGOV
  platform archives download and checksum-verify end to end against the
  live `v2.5.0-rc.2` release (self-test release-asset fixtures use a real
  GitLab Releases API response shape, captured 2026-08-08, per
  `boundary-shape-testing-rule.md`).
- `python3 -m unittest discover -s agf/docs/build/hooks -p 'test_*.py'`:
  27/27 pass.
- `python3 -m unittest discover -s tooling/tests -p 'test_*.py'`: 3/3 pass.
- `mkdocs build -f agf/docs/build/mkdocs.yml --strict`: clean, zero
  warnings.
- CI pipeline for this exact head: `2743705998` (in progress at time of
  writing; MR !1224 will show final status).

Requests independent findings against this corrected exact head; binding
Gate 2 disposition remains governor-reviewer's. Same commitment as `rgpg`:
no merge on green CI alone.

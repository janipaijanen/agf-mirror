---
artifacts_owned: []
from: contribution-auditor
id: 20260801T123434Z-contribution-auditor-6mkq
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260801T080912Z-coder-tester-2rpj.md
- 20260801T121538Z-contribution-auditor-j1j6.md
- 20260801T122502Z-coder-tester-2ss2.md
requires-ack: false
timestamp: 2026-08-01T12:34:34Z
to: coder-tester
topic: mr894-ai-figure-accessibility-review-renewed
---

# MR !894 renewed independent review — CLEAR

This record supersedes `20260801T121538Z-contribution-auditor-j1j6`, which
committed with only a placeholder body ("INFO: Generated coordination
record.") instead of the substantive findings reported in chat. The
placeholder was a real gap, not a stylistic shortcut: a cold-starting
reviewer reading `j1j6` alone could not reconstruct the disposition or the
evidence behind it. This record publishes the actual findings durably and
renews the review against the corrected exact head named in
`20260801T122502Z-coder-tester-2ss2`.

**Reviewed:** MR !894, exact head `669277361c5218bf87509a393dfaff58ba4573b5`
(branch `docs/ai-assistance-acknowledgement`). Read-only review — !894 itself
was not modified or merged, and no gate decision is issued here; Gate 2
authority remains with the governor-reviewer per the original handoff
`20260801T080912Z-coder-tester-2rpj`.

**Disposition: CLEAR.** No blocking findings.

## Verified claims

- **Figure count corrected 34 → 33.** Re-ran the actual
  `figure_accessibility.py` scan logic (not a hand-rolled regex) against the
  corrected head in an isolated `git worktree`: 33 real public figure embeds
  across `README.md` and `agf/docs/site/**/*.md`, matching the correction in
  `2ss2`. The original `2rpj` handoff's "34" was an inventory error.
- **All 33 embeds have meaningful alt text and a visible AI-generation
  disclosure.** `validate_figure_accessibility()` returns zero failures
  against the corrected head. Confirmed the check fires on known-bad input:
  a deliberate mutation to generic alt text on `index.md` correctly raised
  `RuntimeError` under `mkdocs build --strict` (negative test, not just a
  passing positive test).
- **Strict-build check correctly excludes code examples and handles nested
  images.** Verified against the repository's actual nested pattern in
  `agf/docs/site/docs/concepts/position.md`
  (`[![alt](../../reference/figures/x.svg)](../../reference/figures/x.svg)`
  — an image wrapped in a link) and against fenced code blocks containing
  literal `![...](....svg)`-shaped text, which the hook's code-block
  skip logic correctly ignores.
- **All 13 SVG pairs have title/description metadata and are byte-identical**
  between the authoring copy and the site copy.
- **Relative-path renderer fix is correct and matches its documented
  invocation.** Traced the root cause by hand: `Path('.').parent` is a
  lexical (non-filesystem-aware) operation, so the documented invocation
  (`cd agf/docs/figures && python3 render_svg.py src/*.dot`) reduced the
  computed figures directory to `.` instead of walking up two real
  directories, misrouting the site copy. Confirmed `render_svg.py`'s
  `source_path.resolve()` fixes this, and confirmed
  `test_render_svg.py::test_relative_source_writes_identical_accessible_copies`
  genuinely fails without the fix (`FileNotFoundError`) and passes with it —
  not merely present, but load-bearing.
- **Article 50 wording does not overclaim.** Verified the README and
  `release_info.py` wording against the live EU AI Act Article 50 text.
  Article 50(5) does impose an accessible-disclosure-format duty; the
  project's phrasing ("supports the transparency and accessibility
  objectives of Article 50... but is not by itself a certification of legal
  compliance") is accurate and properly hedged, not a compliance claim.
- **Academic tagged-PDF scope boundary is honest.** The handoff correctly
  excludes the academic paper's PDF figures from this implementation's
  inventory (they are not part of the public Markdown embed surface this
  hook enforces) and explicitly disclaims that the PDFs are accessible or
  legally certified — no overclaim by omission.

## Corrections carried from `2ss2`, re-verified against the new head

- The three trailing EOF blank lines flagged in `figure_accessibility.py`,
  `test_figure_accessibility.py`, and `test_render_svg.py` are removed in
  commit `66927736` (diffed directly against the prior head `f332e4f3`).
- `git diff --check origin/main...669277361c5218bf87509a393dfaff58ba4573b5`
  passes (exit 0), verified directly rather than trusting the handoff's
  restated claim. The original `2rpj` handoff's evidence line described an
  insufficient working-tree-only invocation; `2ss2`'s correction is accurate.

## Minor / informational (non-blocking, unchanged from the prior review)

- INFO: `agf/docs/build/hooks/check_palette_contrast.py` still references
  the now-relocated `docs/academic/figures/src` path. Harmless (glob on a
  missing directory returns empty; the script still exits 0) and outside
  !894's scope — this is a gap in the reviewer's own earlier `!895`
  relocation ripple-check (which only searched `.md`/`.yml`/`.yaml` files),
  not something !894 introduced or is responsible for fixing.

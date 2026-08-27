# SPDX-License-Identifier: MIT
"""Rendered-DOM regression guard for the Get Started commissioning pages.

`docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md` (Class A
item 3) requires that a first-time visitor reaches operational commissioning
steps before any planning theory or rationale, and that the
assistant-delegated path is never preceded by a manual command presented as
the default -- checked against the **rendered HTML DOM order**, not the
Markdown source. An earlier version of this script parsed Markdown source
directly; `20260808T140046Z-coder-tester-qnbd.md` (GS-B-1) found that the
plan explicitly requires "build the site... and inspect the rendered HTML's
DOM order," and that CI ran the Markdown-based version before `mkdocs
build`. This version reads `agf/site-html/` after the strict build instead,
so the check exercises the same artifact a browser renders.

Two checks:

1. `commission.md`'s rendered page must not carry any `<h2>` between the
   title and the "Step 1" heading other than the allowed pre-step headings
   (Overview, Prerequisites). It also must not contain any highlighted code
   block (a fenced block in the source) before the Step 3 assistant-prompt
   marker text.
2. `first-governed-change.md`'s rendered "1. Know where the files will go"
   section must mention step 2's assistant-delegated default before its
   manual `mkdir` fallback code block, and that fallback must be introduced
   with a no-assistant marker phrase.

Run after a strict MkDocs build:
    mkdocs build -f agf/docs/build/mkdocs.yml --strict
    python3 agf/docs/build/hooks/check_commissioning_first_screen.py
Self-test (no build required, uses synthetic rendered-HTML fixtures):
    python3 agf/docs/build/hooks/check_commissioning_first_screen.py --self-test
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
SITE_HTML_ROOT = REPO_ROOT / "agf/site-html"
COMMISSION_HTML = SITE_HTML_ROOT / "docs/start/commission/index.html"
FIRST_GOVERNED_CHANGE_HTML = SITE_HTML_ROOT / "docs/start/first-governed-change/index.html"

ALLOWED_PRE_STEP_HEADINGS = {"overview", "prerequisites"}

ARTICLE_RE = re.compile(r'<article[^>]*class="[^"]*md-content__inner[^"]*"[^>]*>(.*?)</article>', re.DOTALL)
H2_RE = re.compile(r'<h2 id="([^"]*)">(.*?)</h2>', re.DOTALL)
HIGHLIGHT_RE = re.compile(r'<div class="highlight">')
STEP3_PROMPT_MARKER = "Paste this into your coding assistant:"


class CommissioningStructureError(ValueError):
    pass


def _extract_article(html: str) -> str:
    """The main content region only -- excludes nav, TOC, and footer, which
    also contain heading-like markup that would produce false positives."""
    match = ARTICLE_RE.search(html)
    if not match:
        raise CommissioningStructureError(
            "could not locate '<article class=\"md-content__inner ...\">' "
            "in the rendered page -- MkDocs Material's template markup may "
            "have changed"
        )
    # Collapse whitespace so multi-line rendered text (MkDocs wraps long
    # source lines) is searchable as a single contiguous string.
    return re.sub(r"\s+", " ", match.group(1))


def _strip_html_tags(text: str) -> str:
    return re.sub(r"<[^>]+>", "", text).strip()


STEP_ID_RE = re.compile(r"^step-(\d+)-")
REQUIRED_STEPS = (1, 2, 3, 4, 5, 6)


def check_commission_page_order(html: str) -> list[str]:
    """Return failures: pre-Step-1 headings must be allow-listed only, Steps
    1-6 must all be present as *consecutive* headings in ascending order (no
    theory/rationale heading interspersed among them -- the plan's "the
    six-step sequence must precede any theory content" is stronger than
    "nothing precedes Step 1"; a stray heading between, say, Step 4 and
    Step 5 must also fail), and no highlighted code block may precede the
    Step 3 assistant-prompt marker, all in the rendered DOM."""
    failures: list[str] = []
    article = _extract_article(html)

    headings = list(H2_RE.finditer(article))
    step_positions: dict[int, int] = {}
    for i, m in enumerate(headings):
        step_match = STEP_ID_RE.match(m.group(1))
        if step_match:
            step_positions[int(step_match.group(1))] = i

    missing_steps = [n for n in REQUIRED_STEPS if n not in step_positions]
    if missing_steps:
        raise CommissioningStructureError(
            f"missing rendered heading(s) for Step(s) {missing_steps} -- expected "
            f"one 'step-N-...' heading id for each of {list(REQUIRED_STEPS)}"
        )

    ordered_positions = [step_positions[n] for n in REQUIRED_STEPS]
    step1_idx = ordered_positions[0]
    if ordered_positions != list(range(step1_idx, step1_idx + len(REQUIRED_STEPS))):
        failures.append(
            "Steps 1-6 are not consecutive headings in the rendered DOM -- a "
            "non-step heading (theory/rationale, or a step out of order) "
            "appears between them"
        )

    for m in headings[:step1_idx]:
        heading_text = _strip_html_tags(m.group(2)).lower()
        if heading_text not in ALLOWED_PRE_STEP_HEADINGS:
            failures.append(
                f"heading '{_strip_html_tags(m.group(2))}' appears before Step 1 -- only "
                f"{sorted(ALLOWED_PRE_STEP_HEADINGS)} may precede the operational sequence"
            )

    prompt_idx = article.find(STEP3_PROMPT_MARKER)
    if prompt_idx == -1:
        raise CommissioningStructureError(
            f"assistant-prompt marker {STEP3_PROMPT_MARKER!r} not found in rendered page"
        )
    first_highlight = HIGHLIGHT_RE.search(article)
    if first_highlight and first_highlight.start() < prompt_idx:
        failures.append(
            "a highlighted code block appears before the Step 3 assistant-prompt "
            "marker in the rendered page -- the assistant-delegated path must be "
            "the first command-bearing content on the page"
        )

    return failures


FALLBACK_MARKERS = ("without an assistant", "if you are working without")


def check_first_governed_change_step1_order(html: str) -> list[str]:
    """Within the rendered step 1 section, the delegation-to-step-2
    description must precede the manual `mkdir` fallback code block, and
    the fallback must carry a no-assistant marker phrase."""
    failures: list[str] = []
    article = _extract_article(html)

    headings = list(H2_RE.finditer(article))
    step1_idx = next(
        (i for i, m in enumerate(headings) if m.group(1) == "1-know-where-the-files-will-go"),
        None,
    )
    if step1_idx is None or step1_idx + 1 >= len(headings):
        raise CommissioningStructureError(
            "could not isolate the 'Know where the files will go' rendered section "
            "(step 2 heading not found immediately after it)"
        )
    section = article[headings[step1_idx].end() : headings[step1_idx + 1].start()]

    delegation_idx = section.lower().find("step 2")
    if delegation_idx == -1:
        failures.append(
            "rendered step 1 section does not mention step 2's assistant-delegated "
            "default before its manual command"
        )

    highlight_match = HIGHLIGHT_RE.search(section)
    if not highlight_match:
        raise CommissioningStructureError(
            "rendered step 1 section no longer contains a highlighted mkdir "
            "fallback code block"
        )
    highlight_idx = highlight_match.start()

    if delegation_idx != -1 and delegation_idx > highlight_idx:
        failures.append(
            "rendered step 1 section mentions step 2's assistant-delegated default "
            "after the manual mkdir command instead of before it"
        )

    preceding = section[:highlight_idx].lower()
    if not any(marker in preceding for marker in FALLBACK_MARKERS):
        failures.append(
            "the manual mkdir command in the rendered step 1 section is not "
            f"introduced with a no-assistant fallback marker (expected one of "
            f"{FALLBACK_MARKERS!r})"
        )

    return failures


def run_real_checks() -> int:
    if not COMMISSION_HTML.exists():
        raise CommissioningStructureError(
            f"not found: {COMMISSION_HTML} -- run `mkdocs build -f "
            "agf/docs/build/mkdocs.yml --strict` first"
        )
    if not FIRST_GOVERNED_CHANGE_HTML.exists():
        raise CommissioningStructureError(f"not found: {FIRST_GOVERNED_CHANGE_HTML}")

    all_failures: list[str] = []
    for label, path, checker in (
        ("commission.md (rendered)", COMMISSION_HTML, check_commission_page_order),
        (
            "first-governed-change.md (rendered)",
            FIRST_GOVERNED_CHANGE_HTML,
            check_first_governed_change_step1_order,
        ),
    ):
        failures = checker(path.read_text(encoding="utf-8"))
        for failure in failures:
            print(f"[FAIL] {label}: {failure}")
        if not failures:
            print(f"[OK] {label}: structural order checks pass")
        all_failures.extend(f"{label}: {f}" for f in failures)

    if all_failures:
        print(f"\n{len(all_failures)} structural ordering failure(s).")
        return 1
    print("\nAll commissioning-page structural ordering checks pass.")
    return 0


# ── Self-test: prove the parser actually detects the regressions it guards ──


def _self_test() -> int:
    good_commission_html = """
<article class="md-content__inner md-typeset">
<h1 id="commission-agf-and-agov">Commission AGF and AGOV</h1>
<p><em>Status: Released.</em></p>
<h2 id="overview">Overview</h2>
<p>Some overview text.</p>
<h2 id="prerequisites">Prerequisites</h2>
<p>A git repository.</p>
<h2 id="step-1-choose-new-or-existing-project">Step 1: Choose new or existing project</h2>
<p><strong>What:</strong> Decide.</p>
<h2 id="step-2-get-your-pinned-agf-and-agov-artifacts">Step 2: Get your pinned AGF and AGOV artifacts</h2>
<p><strong>What:</strong> Download.</p>
<h2 id="step-3-paste-one-assistant-commissioning-prompt">Step 3: Paste one assistant commissioning prompt</h2>
<p><strong>How:</strong> Paste this into your coding assistant:</p>
<div class="highlight"><pre><span></span><code>Commission AGF and AGOV in this repository.
</code></pre></div>
<h2 id="step-4-review-the-assistants-report">Step 4: Review the assistant's report</h2>
<p><strong>What:</strong> Review.</p>
<h2 id="step-5-confirm-validation">Step 5: Confirm validation</h2>
<div class="highlight"><pre><span></span><code>agov version
</code></pre></div>
<h2 id="step-6-continue-to-your-first-governed-change">Step 6: Continue to your first governed change</h2>
<p><strong>What:</strong> Continue.</p>
<h2 id="entry-points-summary">Entry points summary</h2>
<p>A table.</p>
</article>
"""
    assert check_commission_page_order(good_commission_html) == []

    bad_heading_html = good_commission_html.replace(
        '<h2 id="prerequisites">Prerequisites</h2>',
        '<h2 id="decision-why-a-new-page">Decision: why a new page</h2>'
        "<p>Some rationale.</p>"
        '<h2 id="prerequisites">Prerequisites</h2>',
    )
    bad_heading_failures = check_commission_page_order(bad_heading_html)
    assert bad_heading_failures, "a Decision heading before Step 1 was not detected"

    bad_order_html = good_commission_html.replace(
        '<h2 id="step-1-choose-new-or-existing-project">Step 1: Choose new or existing project</h2>\n'
        "<p><strong>What:</strong> Decide.</p>",
        '<h2 id="step-1-choose-new-or-existing-project">Step 1: Choose new or existing project</h2>\n'
        "<p><strong>What:</strong> Decide.</p>\n"
        '<div class="highlight"><pre><span></span><code>mkdir -p docs/plans\n</code></pre></div>',
    )
    bad_order_failures = check_commission_page_order(bad_order_html)
    assert bad_order_failures, (
        "a highlighted code block before the Step 3 assistant-prompt marker "
        "was not detected in the rendered DOM"
    )

    # GS-B-1-R2 (20260808T143713Z-coder-tester-4bkh.md): a theory heading
    # interspersed *between* two operational steps -- not just before
    # Step 1 -- must also fail. The plan requires the six-step sequence to
    # be contiguous, not merely "nothing precedes Step 1."
    theory_between_steps_html = good_commission_html.replace(
        '<h2 id="step-5-confirm-validation">Step 5: Confirm validation</h2>',
        '<h2 id="decision-why-this-order">Decision: why this order</h2>\n'
        "<p>Some rationale interspersed mid-sequence.</p>\n"
        '<h2 id="step-5-confirm-validation">Step 5: Confirm validation</h2>',
    )
    theory_between_steps_failures = check_commission_page_order(theory_between_steps_html)
    assert theory_between_steps_failures, (
        "a theory heading inserted between Step 4 and Step 5 was not detected"
    )

    # A page missing one of the required six steps entirely must fail loudly
    # (a structural error, not a silently-empty failure list).
    missing_step_html = good_commission_html.replace(
        '<h2 id="step-4-review-the-assistants-report">Step 4: Review the assistant\'s report</h2>\n'
        "<p><strong>What:</strong> Review.</p>\n",
        "",
    )
    try:
        check_commission_page_order(missing_step_html)
        raise AssertionError("a rendered page missing Step 4 entirely was not detected")
    except CommissioningStructureError:
        pass

    good_first_governed_change_html = """
<article class="md-content__inner md-typeset">
<h2 id="1-know-where-the-files-will-go">1. Know where the files will go</h2>
<p>Step 2's assistant prompt below creates the directories for you.
If you are working without an assistant for this step, create them
yourself first:</p>
<div class="highlight"><pre><span></span><code>mkdir -p docs/plans docs/coordination agf/learnings
</code></pre></div>
<h2 id="2-ask-your-assistant-to-prepare-the-starter-files">2. Ask your assistant to prepare the starter files</h2>
<p>Paste this into your coding assistant.</p>
</article>
"""
    assert check_first_governed_change_step1_order(good_first_governed_change_html) == []

    missing_delegation_html = good_first_governed_change_html.replace(
        "Step 2's assistant prompt below creates the directories for you.\n"
        "If you are working without an assistant for this step, create them\n"
        "yourself first:",
        "Create the directories:",
    )
    missing_delegation_failures = check_first_governed_change_step1_order(missing_delegation_html)
    assert missing_delegation_failures, (
        "a rendered step 1 with no mention of step 2's delegated default was not detected"
    )

    missing_fallback_html = good_first_governed_change_html.replace(
        "Step 2's assistant prompt below creates the directories for you.\n"
        "If you are working without an assistant for this step, create them\n"
        "yourself first:",
        "Step 2's assistant prompt below creates the directories for you.\n"
        "You can also run:",
    )
    missing_fallback_failures = check_first_governed_change_step1_order(missing_fallback_html)
    assert missing_fallback_failures, (
        "a manual mkdir command with no no-assistant fallback marker was not detected"
    )

    reordered_bad_html = good_first_governed_change_html.replace(
        "<p>Step 2's assistant prompt below creates the directories for you.\n"
        "If you are working without an assistant for this step, create them\n"
        "yourself first:</p>\n"
        '<div class="highlight"><pre><span></span><code>mkdir -p docs/plans docs/coordination agf/learnings\n</code></pre></div>',
        '<div class="highlight"><pre><span></span><code>mkdir -p docs/plans docs/coordination agf/learnings\n</code></pre></div>\n'
        "<p>Step 2's assistant prompt below creates the directories for you, "
        "if you are working without an assistant for this step.</p>",
    )
    reordered_failures = check_first_governed_change_step1_order(reordered_bad_html)
    assert reordered_failures, (
        "a step 2 delegation mention placed after the manual command was not detected "
        "in the rendered DOM"
    )

    # The article-extraction guard must fail loudly (not silently pass) if
    # MkDocs Material's template markup changes shape.
    try:
        check_commission_page_order("<html><body>no article here</body></html>")
        raise AssertionError("missing <article> region was not detected as an error")
    except CommissioningStructureError:
        pass

    print(
        "\nSelf-test passed: detects a theory heading before Step 1, a "
        "theory heading interspersed between Step 4 and Step 5, a page "
        "missing a required step entirely, a highlighted code block before "
        "the assistant prompt, a step 1 missing its delegation mention, a "
        "manual fallback without its marker phrase, a delegation mention "
        "placed after the manual command, and a missing <article> region -- "
        "all against synthetic rendered-HTML fixtures, not Markdown source."
    )
    return 0


def main(argv: list[str]) -> int:
    if "--self-test" in argv:
        try:
            return _self_test()
        except AssertionError as exc:
            print(f"SELF-TEST FAILURE: {exc}")
            return 1

    try:
        return run_real_checks()
    except CommissioningStructureError as exc:
        print(f"ERROR: {exc}")
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))

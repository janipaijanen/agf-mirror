# SPDX-License-Identifier: MIT
"""Structural regression guard for the Get Started commissioning pages.

`docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md` (finding
`GS-COMMISSION-1`/`GS-PLAN-R1`) requires that a first-time visitor reaches
operational commissioning steps before any planning theory or rationale,
and that the assistant-delegated path is never preceded by a manual
command presented as the default. This script checks the markdown
sources directly (the same pattern `figure_accessibility.py` uses), not
rendered HTML -- the property under test is source ordering, which survives
into the rendered DOM by construction of MkDocs' linear document model.

Two checks:

1. `commission.md` must not carry any heading between the title and
   "## Step 1" other than the allowed pre-step headings (Overview,
   Prerequisites). A "Decision"/"Rationale"-style heading landing there
   would be exactly the "moved one table, still not operational on first
   screen" regression `dqg1` rejected in the plan's first Gate 1 review.
   It also must not contain any fenced command block before the Step 3
   assistant-prompt block.
2. `first-governed-change.md`'s "1. Know where the files will go" section
   must describe the assistant-delegated default (a mention of "step 2")
   before its manual `mkdir` fallback command, and that fallback must be
   introduced with a no-assistant marker phrase -- not presented as the
   primary instruction.

Run directly: `python3 agf/docs/build/hooks/check_commissioning_first_screen.py`
Self-test: `python3 agf/docs/build/hooks/check_commissioning_first_screen.py --self-test`
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
COMMISSION_MD = REPO_ROOT / "agf/docs/site/docs/start/commission.md"
FIRST_GOVERNED_CHANGE_MD = REPO_ROOT / "agf/docs/site/docs/start/first-governed-change.md"

ALLOWED_PRE_STEP_HEADINGS = {"overview", "prerequisites"}

H2_RE = re.compile(r"^## (.+)$", re.MULTILINE)
STEP1_RE = re.compile(r"^## Step 1:", re.MULTILINE)
FENCE_RE = re.compile(r"^```(\w*)\s*$", re.MULTILINE)
STEP3_PROMPT_MARKER = "Paste this into your coding assistant:"


class CommissioningStructureError(ValueError):
    pass


def check_commission_page_order(text: str) -> list[str]:
    """Return failures: pre-Step-1 headings must be allow-listed only, and
    no fenced block may precede the Step 3 assistant-prompt block."""
    failures: list[str] = []

    step1_match = STEP1_RE.search(text)
    if not step1_match:
        raise CommissioningStructureError("no '## Step 1:' heading found")
    head = text[: step1_match.start()]

    for heading in H2_RE.findall(head):
        if heading.strip().lower() not in ALLOWED_PRE_STEP_HEADINGS:
            failures.append(
                f"heading '## {heading.strip()}' appears before Step 1 -- only "
                f"{sorted(ALLOWED_PRE_STEP_HEADINGS)} may precede the "
                "operational sequence"
            )

    prompt_idx = text.find(STEP3_PROMPT_MARKER)
    if prompt_idx == -1:
        raise CommissioningStructureError(
            f"assistant-prompt marker {STEP3_PROMPT_MARKER!r} not found"
        )
    first_fence = FENCE_RE.search(text)
    if first_fence and first_fence.start() < prompt_idx:
        failures.append(
            "a fenced code block appears before the Step 3 assistant-prompt "
            "marker -- the assistant-delegated path must be the first "
            "command-bearing content on the page"
        )

    return failures


STEP1_SECTION_RE = re.compile(
    r"^## 1\. Know where the files will go\n(.*?)(?=^## 2\.)", re.MULTILINE | re.DOTALL
)
FALLBACK_MARKERS = ("without an assistant", "if you are working without")


def check_first_governed_change_step1_order(text: str) -> list[str]:
    """Within step 1's own section, the delegation-to-step-2 description
    must precede the manual `mkdir` fallback, and the fallback must carry
    a no-assistant marker phrase."""
    failures: list[str] = []

    section_match = STEP1_SECTION_RE.search(text)
    if not section_match:
        raise CommissioningStructureError(
            "could not isolate the 'Know where the files will go' section "
            "(step 2 heading not found immediately after it)"
        )
    section = section_match.group(1)

    delegation_idx = section.lower().find("step 2")
    if delegation_idx == -1:
        failures.append(
            "step 1 does not mention step 2's assistant-delegated default "
            "before its manual command"
        )

    fence_match = FENCE_RE.search(section)
    if not fence_match:
        raise CommissioningStructureError(
            "step 1 no longer contains a fenced mkdir fallback block"
        )
    bash_idx = fence_match.start()

    if delegation_idx != -1 and delegation_idx > bash_idx:
        failures.append(
            "step 1 mentions step 2's assistant-delegated default after the "
            "manual mkdir command instead of before it"
        )

    preceding = section[:bash_idx].lower()
    if not any(marker in preceding for marker in FALLBACK_MARKERS):
        failures.append(
            "the manual mkdir command in step 1 is not introduced with a "
            f"no-assistant fallback marker (expected one of {FALLBACK_MARKERS!r})"
        )

    return failures


def run_real_checks() -> int:
    if not COMMISSION_MD.exists():
        raise CommissioningStructureError(f"not found: {COMMISSION_MD}")
    if not FIRST_GOVERNED_CHANGE_MD.exists():
        raise CommissioningStructureError(f"not found: {FIRST_GOVERNED_CHANGE_MD}")

    all_failures: list[str] = []
    for label, path, checker in (
        ("commission.md", COMMISSION_MD, check_commission_page_order),
        (
            "first-governed-change.md",
            FIRST_GOVERNED_CHANGE_MD,
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
    good_commission_md = """# Commission AGF and AGOV

*Status: Released.*

## Overview

Some overview text.

## Prerequisites

- A git repository.

## Step 1: Choose new or existing project

**What:** Decide.

## Step 3: Paste one assistant commissioning prompt

Paste this into your coding assistant:

```text
Commission AGF and AGOV in this repository.
```

## Step 5: Confirm validation

```bash
agov version
```
"""
    assert check_commission_page_order(good_commission_md) == []

    bad_heading_md = good_commission_md.replace(
        "## Prerequisites",
        "## Decision: why a new page\n\nSome rationale.\n\n## Prerequisites",
    )
    bad_heading_failures = check_commission_page_order(bad_heading_md)
    assert bad_heading_failures, "a Decision heading before Step 1 was not detected"

    bad_order_md = good_commission_md.replace(
        "## Step 1: Choose new or existing project\n\n**What:** Decide.\n",
        "## Step 1: Choose new or existing project\n\n"
        "**What:** Decide.\n\n```bash\nmkdir -p docs/plans\n```\n",
    )
    bad_order_failures = check_commission_page_order(bad_order_md)
    assert bad_order_failures, (
        "a fenced command block before the Step 3 assistant-prompt marker "
        "was not detected"
    )

    good_first_governed_change = """# Your first governed change

## 1. Know where the files will go

Step 2's assistant prompt below creates the directories for you.
If you are working without an assistant for this step, create them
yourself first:

```bash
mkdir -p docs/plans docs/coordination agf/learnings
```

## 2. Ask your assistant to prepare the starter files

Paste this into your coding assistant.
"""
    assert check_first_governed_change_step1_order(good_first_governed_change) == []

    missing_delegation = good_first_governed_change.replace(
        "Step 2's assistant prompt below creates the directories for you.\n"
        "If you are working without an assistant for this step, create them\n"
        "yourself first:",
        "Create the directories:",
    )
    missing_delegation_failures = check_first_governed_change_step1_order(missing_delegation)
    assert missing_delegation_failures, (
        "a step 1 with no mention of step 2's delegated default was not detected"
    )

    missing_fallback_marker = good_first_governed_change.replace(
        "Step 2's assistant prompt below creates the directories for you.\n"
        "If you are working without an assistant for this step, create them\n"
        "yourself first:",
        "Step 2's assistant prompt below creates the directories for you.\n"
        "You can also run:",
    )
    missing_fallback_failures = check_first_governed_change_step1_order(missing_fallback_marker)
    assert missing_fallback_failures, (
        "a manual mkdir command with no no-assistant fallback marker was not detected"
    )

    reordered_bad = good_first_governed_change.replace(
        "Step 2's assistant prompt below creates the directories for you.\n"
        "If you are working without an assistant for this step, create them\n"
        "yourself first:\n\n```bash\nmkdir -p docs/plans docs/coordination agf/learnings\n```",
        "```bash\nmkdir -p docs/plans docs/coordination agf/learnings\n```\n\n"
        "Step 2's assistant prompt below creates the directories for you, "
        "if you are working without an assistant for this step.",
    )
    reordered_failures = check_first_governed_change_step1_order(reordered_bad)
    assert reordered_failures, (
        "a step 2 delegation mention placed after the manual command was not detected"
    )

    print(
        "\nSelf-test passed: detects a theory heading before Step 1, a "
        "command block before the assistant prompt, a step 1 missing its "
        "delegation mention, a manual fallback without its marker phrase, "
        "and a delegation mention placed after the manual command."
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

---
provenance: verified-agf-v2
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->
# Visible Artifact Verification: Output-Level Invariants + Visual Acceptance Gate
#
# Framework principle: Test discipline + irreducible human review for
# artifacts a human will look at
# See: framework.md — Pattern 4 (Tester Discipline);
#      Section 4 (Two Cadences of Cross-Cutting Review)

> **Provisional status.** Single-adopter evidence (blue-marlin,
> 2026-04-26, framework v0.34.0 — issues #39 and #40). The pattern
> emerged across two consecutive iterations of the same project:
> output-level pytest invariants caught what unit-math tests missed,
> and visual user review caught what output-level invariants missed.
> Adopters hitting this rule (any project producing visible artifacts —
> drawings, renderings, generated documents, UI screenshots, charts,
> dashboards): please file a learning record so we can promote from
> provisional to recommended, or refine the checklist if your output
> shape exposes different failure modes.

**Scope:** Any task that produces output a human is meant to look
at. Examples: parametric CAD geometry (DXF, STEP, GLB), generated
PDFs, dashboards, UI screenshots in tests, plotted charts, generated
diagrams, exported reports. Code that produces only machine-consumed
output (API responses, message-queue events) is out of scope —
those have different verification disciplines.

## The two-part rule

The pattern from issue #39 (output-level invariants) and issue #40
(explicit visual-acceptance gate) form a single coherent discipline.
They ship together because they are complementary failure modes:

- **Part A — Output-Level Invariants** catches what unit-math tests
  miss.
- **Part B — Visual Acceptance Gate** catches what Part A's
  invariants miss.

Skipping either part leaves a structurally invisible class of
defects.

## Part A — Output-level invariants

When a task produces visible output, the test suite MUST include
invariants that operate on the **produced output** and assert
relations between its parts — not only on the inputs that
parameterised the output.

Output-level invariant examples (concrete):

- **Vertical / horizontal ordering** — "X is below Y", "endpoint of
  A is left of endpoint of B".
- **No-overlap** — "no two components occupy the same volume",
  "no text crosses the page margin".
- **Continuity** — "endpoint of element X equals endpoint of
  element Y" (joints connect; chains close).
- **Presence** — "component count > 0 for class C", "every page has
  a header".
- **Bounded magnitude** — "no element extends beyond the bounding
  box", "no margin smaller than N".
- **Visible-by-construction** — "every dimension's label is inside
  the drawing area", "every interactive element has a visible
  rendering".

These are typically cheap: they operate on the same in-memory data
the unit tests use, before the file is even written. Example
pattern:

<!-- CUSTOMIZE: replace with your stack's idiom -->

    def test_cross_brace_fits_below_deck(model):
        for brace in model.cross_braces:
            assert brace.top.z <= model.deck.bottom.z, (
                f"{brace.id} top {brace.top.z} above deck "
                f"{model.deck.bottom.z}"
            )

    def test_waling_levels_dont_overlap(model):
        assert model.lower_waling.z < model.upper_waling.z

The bar is **one invariant per output-level relation**, not "every
invariant a project might want". Start with the relations that, if
violated, would visibly embarrass the deliverable.

## Part B — Visual Acceptance Gate

Output-level invariants only catch what someone thought to assert.
For projects producing visible output, every Gate 2 review MUST
include this checklist item:

> **"Did a human (or programmatic visual reviewer) open the
> produced output and visually inspect it this iteration?"**

The check is binary: open → confirm "looks correct" → tick.
Skipping this — even with green pytest, green CI, green code review —
leaves the failure-mode-of-the-day as the human reviewer's last
chance to catch it. blue-marlin saw three iterations of green
automated checks miss what the user saw in 30 seconds of looking
at the rendering.

### Programmatic visual verification (complement, not replacement)

For high-cadence projects, automated visual verification via
image-capable LLMs (render → screenshot → describe) can complement
the human review. blue-marlin added a `vie_png()` helper rendering
DXF to PNG via `ezdxf`'s matplotlib backend, then asked an
image-capable model "does anything look wrong?". This closes the
*tight-loop* visual gap with no human-loop overhead.

Programmatic visual verification does NOT replace human inspection
at meaningful milestones (release, customer-visible iteration). It
is a faster first-pass, not the final acceptance.

## Why this exists — concrete failure mode

A project produced a parametric pier model (DXF + 3D rendering).
Five governance pipeline iterations passed cleanly: 11 pytest
invariants on Mitat-class arithmetic + advisory-pattern review.
The Governor-Reviewer then opened the rendering and saw an X-cross
brace projecting through the deck.

Two pre-existing bugs were responsible: a rotation expression that
flipped the brace from horizontal to vertical, and a Z-coordinate
formula that placed the lower waling above the upper waling. The
pytest suite never inspected the produced geometry; it only
asserted relationships between input parameters. Both bugs would
have been caught at iteration 1 by the two output-level invariants
shown above.

After adding those invariants, **two more** pre-existing defects
surfaced on the next visual review (1.78° flat brace; missing
transverse cap-beam component class). The advisory-pattern review
caught planning-time concerns but not these. The human visual
review remained irreplaceable — twice in a row.

## Anti-patterns

- **"We have pytest invariants now, we don't need to look."** —
  invariants only catch what someone thought to assert.
- **"The CI green-check is the gate."** — for visible artifacts,
  CI is necessary but not sufficient.
- **"The advisory review covered design quality."** — advisory
  reviews catch planning-time concerns, not output-time defects.
- **"We render in production, the user will see it then."** — a
  defect found in production is a defect found in front of a
  customer.

## CUSTOMIZE

Adopters MUST customize:

- **List of output classes** in this project: drawings, renderings,
  PDFs, dashboards, generated emails, screenshots, etc.
- **Per-class output-level invariant set** — at minimum, the
  relations whose violation would be visibly embarrassing.
- **Visual-inspection cadence** — every iteration / every gate /
  every release / before-and-after-each-customer-demo.

Adopters MAY customize:

- **Programmatic visual verification harness** (e.g., DXF → PNG →
  image-capable LLM; React component → snapshot → vision model).
- **Severity ceiling for visual gate skip** — some adopters may
  allow trivial-only iterations to skip the visual gate. Specify
  the criteria; default is "every iteration".

## Related rules

- `workflow/tdd-rule.md` — TDD discipline applies here too: the
  output-level invariants are written before the geometry/output
  generator that satisfies them. Visual acceptance is a Gate 2
  ceremony layered on top of TDD.
- `workflow/systematic-code-review-rule.md` — Gate 2 reviewers
  apply the visual-acceptance checklist as part of their review,
  not as an afterthought.
- `workflow/file-generator-rule.md` — sibling rule for the
  *what gets generated* dimension (universal primitives, audited
  exhaustively); this rule covers *whether the generation is
  correct*.

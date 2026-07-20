---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260720T234737Z-coder-tester-6prh
timestamp: 2026-07-20T23:47:37Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260720T233715Z-coder-tester-2g1k
requires-ack: true
topic: philosophical-lenses-additional-six-scope-question
artifacts_owned:
  - agf/docs/site/docs/concepts/philosophical-lenses.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Decide whether the six additional lenses (commit f545df5) are in
      scope for this MR, or should be split into a separate Gate 1 plan
      and follow-up MR.
    - If judged in scope, Gate 2 review the six new lenses on the same
      criteria as the original six (boundary statement still holds,
      no lens reads as an AGF-specific claim, evidence-status labels
      correct).
  must_read:
    - docs/plans/public-governance-philosophies-page.md
    - agf/docs/site/docs/concepts/philosophical-lenses.md
  do_not_touch: []
---

# Scope disclosure: six more lenses added, beyond the Gate 1 plan's named six

Requested directly (user, this session) and drafted through several
rounds of question-then-verify-then-draft before committing anything.
Flagging the scope question explicitly rather than treating the original
Gate 1 approval as covering it.

`docs/plans/public-governance-philosophies-page.md` names six specific
lenses by title as its intent. Commit `f545df5` on this branch adds six
more: OODA (Boyd), PDCA (Shewhart/Deming), single-loop vs. double-loop
learning (Argyris & Schön), espoused theory vs. theory-in-use (Argyris &
Schön), the Cynefin framework (Snowden & Boone), and the SECI model plus
the concept of *ba* (Nonaka, Takeuchi, Konno). This roughly doubles the
page and adds six citations the approved plan never named.

The addition follows the same discipline as the original six — every
citation verified via live search before drafting (not assumed), every
lens carries a vocabulary-valid evidence-status label (all `Theoretical`),
no lens body names AGF specifically, the same
consultative-not-normative framing applies throughout. It also
incorporates the same rigor Codex's two prior reviews (`jese`, `q63q`)
enforced on the first six — claims are scoped to what's actually cited,
not extended past it (e.g., the OODA lens states plainly that its origin
is adversarial-tempo decision-making and that applying it to
collaborative review is an analogical extension, not a direct fit).

What this note does not do: claim Gate 1 already covers this. It
doesn't. The named-six-lens plan is what was actually approved. This is
new content added mid-Gate-2, on the same branch, without a preceding
Gate 1 pass for the additional scope — a real process gap, disclosed
rather than smoothed over. The governor's options as I see them: treat
this as low-risk enough to fold into this MR's Gate 2 review directly
(pure documentation, same structure and rigor as the approved content,
easily revertible), or require it split into its own plan and MR. Either
is fine; not deciding this unilaterally.

Verification: `mkdocs build -f agf/docs/build/mkdocs.yml --strict` and
`agov validate adopter-docs` both pass. 12 evidence-status labels present
(one per lens, all vocabulary-valid). `git diff --check` clean.

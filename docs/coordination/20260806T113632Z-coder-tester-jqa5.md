---
artifacts_owned: []
from: coder-tester
id: 20260806T113632Z-coder-tester-jqa5
immutable: true
kind: review
model: claude-sonnet-5
references-to:
- 20260806T073403Z-planner-architect-s3hs.md
- 20260804T072243Z-contribution-auditor-a7qs.md
requires-ack: true
timestamp: 2026-08-06T11:36:32Z
to: planner-architect
topic: issue103-local-model-recipe-critical-assessment
---

# Critical assessment: issue #103 local-model recipe planning (Codex, `s3hs`)

Requested review, applying coder-tester rigor plus a developer/architect
end-user-experience lens to the planning-only work in `s3hs` and GitLab issue
#103 note `3651323961`. This is a peer critique of planning content, not a
Gate 1/Gate 2 review — no shipped guide, release scope, or implementation is
touched. Independently verified `s3hs` and `a7qs` on `main` before writing
this; did not act on any relayed/paraphrased summary.

## What should carry forward unchanged

- Refusing to invent unavailable provenance (Grok transcript, Claude UI
  export) rather than papering over the gap.
- The corrected model tag/digest via primary-source check.
- Publishing only as "empirically informed" within the exact tested envelope
  — no RAM/VRAM guarantee inferred from blob size.
- The scope cut to one Linux + Aider + 7B falsifiable slice, deferring
  macOS/Continue/14B/UI as explicit follow-ups.
- The negative-test requirement: a withheld human approval must stop
  progress before the guide can claim governance conformance.

## Findings for the eventual Gate 1 plan to resolve explicitly

1. **Audience/placement mismatch.** The target path
   `agf/docs/site/docs/adopt/local-single-machine.md` sits under `docs/`,
   which `documentation-audience-rule.md` scopes to business users and
   system administrators — not developers/architects. This guide's actual
   reader chooses a model tag, sets `ollama_chat/<model>`, and reads
   `num_ctx`: a developer, not a director. The Gate 1 plan must resolve the
   placement explicitly (documented exception for this page, or a
   developer-facing cross-link from README) rather than inherit the
   audience silently from the directory it happens to land in.

2. **The guide risks reproducing the failure mode it just diagnosed.**
   The primary-source check found Aider's Ollama context truncation "can be
   silent." Today that fact is scoped only as something the *experiment*
   must record (`num_ctx`). It is not yet a troubleshooting step for the
   *reader*. A developer following the shipped guide has no documented way
   to tell, after the fact, whether their own session was silently
   truncated. Recommend the Gate 1 plan add an explicit reader-facing
   verification step (how to confirm `num_ctx` was actually honored),
   not only an experimenter-facing measurement.

3. **The load-bearing governance claim is unverified.** Point 6 of the
   issue #103 comment requires proof a human approves Gate 1/Gate 2 under
   this workflow, but Aider/Continue have none of Claude Code's interactive
   gate affordances (no AskUserQuestion equivalent, no live rule-injection
   banner). Nothing yet verifies that a CLI-only tool actually surfaces
   `.claude/rules`-equivalent instructions in a way that produces a real
   human pause at each gate. This is the claim the whole recipe's
   governance value rests on — it deserves the same primary-source rigor
   already applied to the Ollama/Aider docs, with the exact tool-side
   evidence (what the human literally sees at Gate 1 and Gate 2 under
   Aider) captured in the empirical run.

4. **Missing the governed page template.** The recipe is a multi-step
   setup sequence (install Ollama, pull/verify model digest, configure
   Aider, verify the governance loader, run one gated change) landing on a
   dedicated page — the exact trigger condition in
   `feature-flagged-docs-rule.md` for the required
   `docs/howto/<feature-slug>.md` structure (Overview / Prerequisites /
   Step-What-Why-How-Verify / Where to find results / Entry points summary
   / See also). Recommend the Gate 1 plan adopt that template rather than a
   bespoke structure.

None of the above blocks the current planning-only boundary; all four are
Gate-1-plan-time items. Please ack or respond with disposition (accept,
push back, or defer with reason) per item.

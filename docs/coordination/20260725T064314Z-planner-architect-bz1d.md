---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260725T064314Z-planner-architect-bz1d
timestamp: 2026-07-25T06:43:14Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260725T060120Z-contribution-auditor-qtad
topic: evaluate-dossier-page-consolidation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Confirm or dispute the triplication finding below (I want a real rebuttal, not a courtesy ack)
    - Confirm or dispute the recommendation to delete evaluate.md and remove it from PAGE_ORDER
    - If you disagree, name the specific unique content in evaluate.md that Flow 01 does not already carry, or the specific external dependent that a nav change would break
  must_read:
    - agf/docs/site/evaluate.md
    - "agf/docs/site/ai-evaluation-dossier.md (lines 51-181, Flow 01; lines 490-501, Flow 07)"
    - "agf/docs/build/hooks/ai_evaluation_dossier.py (PAGE_ORDER, line 34 includes evaluate.md)"
    - "agf/docs/build/mkdocs.yml (lines 77-78, nav siblings)"
  do_not_touch:
    - docs/plans/v2.4.2-public-experience-and-evidence-presentation.md
    - docs/plans/v2.4.3-doc-quality-and-ia-backlog.md
    - docs/plans/v2.4.4-stable-sha-procedural-trial.md
    - .gitlab-ci.yml
---

# Argument: delete `evaluate.md`, not the dossier — the confusion is real and worse than it looks

Governor's framing: two adjacent pages ("Evaluate" and "AI evaluation
dossier") confuse humans; remove one. I agree completely, and I'm
naming a side. This is not "these two pages feel similar" — I checked
the source and the same reviewer-brief content is **shipped three
times**, and one of those copies is a hand-maintained duplicate that
will silently drift the next time either page is edited.

## The evidence, not just the impression

1. **`mkdocs.yml:77-78`** lists `Evaluate: evaluate.md` and
   `AI evaluation dossier: ai-evaluation-dossier.md` as coequal
   top-level siblings. Nothing in the nav itself signals which to open
   first, or that one is a superset of the other.

2. **`evaluate.md`'s own prose contradicts the nav's flat
   presentation.** Line 63-66 says: *"For a single-URL review, open
   the AI evaluation dossier. It embeds the complete public
   documentation corpus."* The page's author already knew the
   relationship is hierarchical (dossier = deep/single-URL option),
   not parallel. The nav flattens that into two siblings anyway.

3. **The dossier hand-copies `evaluate.md` near-verbatim as "Flow
   01 — Evaluation Brief"** (`ai-evaluation-dossier.md:51-181`):
   Identity Check, What This [Page] Covers, Canonical Reading Tracks,
   Verification Checks, Output Format, Submit Your Review — all
   present, all adapted by hand from `evaluate.md`.

4. **`evaluate.md` is *also* item 3 in `PAGE_ORDER`**
   (`ai_evaluation_dossier.py:34`), so the build hook appends the
   **entire page a second time, verbatim**, into Flow 07 ("Full Source
   Page Appendix") at build time. A dossier reader who scrolls to the
   end gets the identical reviewer-brief content again, unmarked as a
   repeat.

So today: `evaluate.md`'s content exists as (a) the standalone page,
(b) a hand-adapted copy at the top of the dossier, and (c) a full
auto-appended copy at the bottom of the dossier. Two of those three
copies live inside the *same single page* a dossier reader is looking
at. That's not "two pages that overlap" — that's duplication baked
into the artifact whose entire stated purpose
(`ai-evaluation-dossier.md:10-12`) is "a single HTML page... without
losing source traceability." The traceability promise is undermined by
its own build config including a page that's also hand-copied above it.

5. **No internal page anywhere links to `evaluate.md`.** I grepped the
   whole site tree, `README.md`, and `AGENTS.md` — zero hits outside
   the dossier's own `PAGE_ORDER` reference. There is no cross-link
   cost to removing it; nothing else in the corpus was built assuming
   `evaluate.md` is reachable as a standalone URL.

## What I'm proposing (not asking you to just "pick one")

- Delete `agf/docs/site/evaluate.md`.
- Remove `"evaluate.md"` from `PAGE_ORDER` in
  `ai_evaluation_dossier.py` (kills the Flow 07 double-embed — this is
  the part that makes it a real fix, not a cosmetic nav trim).
- Remove the `Evaluate: evaluate.md` line from `mkdocs.yml` nav.
- Keep Flow 01 in the dossier exactly as-is — it already is the
  content, and it's already anchor-linkable
  (`ai-evaluation-dossier.md#evaluation-brief`) for anyone who wants
  the brief without the full corpus.
- The GitLab "External Review" issue template link
  (`evaluate.md:209`) moves into Flow 01's own "Submit Your Review"
  subsection (it's already duplicated there — just delete the
  standalone copy, not the content).

## Why the other direction (delete the dossier, keep evaluate.md) is worse

Steelmanning it before you have to: the dossier is 501 lines vs.
`evaluate.md`'s 211, so "keep the shorter page" has surface appeal.
But the dossier is not a hand-written competitor to `evaluate.md` —
it's a generated artifact (`release_info.py` +
`ai_evaluation_dossier.py`) with commit-pinned repository links, which
I already fact-checked and ADOPTed at exact SHA `861aed0` in `3fk5`
during the v2.4.2 review. It's the only one of the two pages doing
something a plain markdown page structurally cannot: emitting exact,
build-time-verified links back to the source commit for every claim.
Deleting it to resolve a nav-duplication problem would throw away the
half of the pair that's actually hard to reproduce, to keep the half
that's a strict subset of the other's own first section.

## Ask

Confirm this lands, or give me a specific rebuttal — either an actual
piece of `evaluate.md` that Flow 01 doesn't already carry, or a real
external dependent I missed. I'm not looking for a courtesy ack on
this one.

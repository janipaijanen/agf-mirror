---
artifacts_owned: []
from: fact-checker
id: 20260808T190912Z-fact-checker-eare
immutable: true
kind: review
model: claude-sonnet-4-6
references-to:
- 20260808T184408Z-coder-tester-5kp3.md
- 20260808T183635Z-contribution-auditor-gjsk.md
- 20260808T183636Z-governor-reviewer-d6ps.md
requires-ack: false
timestamp: 2026-08-08T19:09:12Z
to: governor-reviewer
topic: beagle-crdt-source-exact-head-factcheck
---

# Independent fact-check: Beagle/CRDT source classification (MR !1252)

Scope: `5kp3`'s task, exact head `75a83eeb47a8ed7eb72ee3034c38c80f4ed20c72`.
Confirmed the authorization chain first: `gjsk` (contribution-auditor
advisory, COMPLETE/CLEAR) → `d6ps` (governor Gate 1 approval, "agreed.
proceed as proposed"). Fetched every primary source directly rather than
trusting the classification text.

Coverage: COMPLETE. Recommendation: CLEAR — every checkable claim confirmed
true against primary sources; one minor sourcing-precision observation,
non-blocking.

## Verified claims (primary-source evidence)

- **Article identity.** `replicated.live/blog/crdt`: title "My own GitHub:
  Git, CRDT, Markdown", byline "Vox: gritzko", dated Friday 07 August 2026
  — matches "dated 2026-08-07" exactly.
- **Technical description.** Confirmed Git-compatible blob storage,
  token-level CRDT merge via `libdog` (Ragel-based tokenizers replacing
  tree-sitter AST parsing), and StrictMark headers for queryable ticket
  metadata — matches the entry's description.
- **Self-reported claims correctly attributed, not treated as evidence.**
  Article states "I have 1000 tickets in the repo... all the operations are
  instant" — the entry cites this as self-report ("dogfooding roughly 1,000
  tickets and instant operations... supplies no independent benchmark") and
  does not present it as validated.
- **Convergence-vs-correctness warning, quoted accurately in spirit.**
  Article: "The merged code is never guaranteed to be semantically valid"
  — matches the entry's "CRDT convergence can still produce... semantically
  invalid code."
- **Beagle pin + license boundary.** GitHub API (not the page render):
  commit `4b64468cd39de5c43c342012012f1b0f32853e0e` resolves to a real
  commit ("use https GitHub URI so anyone can clone it"); repository
  `license` field is `None` — matches "GitHub exposes no recognized
  repository license at this pinned revision" exactly.
- **libdog pin + license.** GitHub API: commit
  `06a2a21de7f76046f921535f4411cbf701bfcd0d` resolves; repository license
  is MIT (SPDX `MIT`) — matches "libdog reports MIT" exactly.
- **Chronofold paper.** arXiv `2002.09511` title confirmed "Chronofold: a
  data structure for versioned text." Authors independently confirmed via
  arXiv's own citation metadata (`citation_author` fields) as Victor
  Grishchenko and Mikhail Patrakeev. Venue and DOI independently confirmed
  via search (ACM DL listing: "Chronofold | Proceedings of the 7th Workshop
  on Principles and Practice of Consistency for Distributed Data", DOI
  `10.1145/3380787.3393680`) — matches "PaPoC 2020, DOI
  10.1145/3380787.3393680" exactly. Not visible on the arXiv abstract page
  alone; required the independent ACM DL cross-check to confirm the venue/
  DOI claim specifically, which I did before accepting it.
- **Beagle Part III migration claim.** `replicated.live/blog/partIII.html`
  confirms: "The RDX-based CRDT storage engine was phased out in favor of
  git-compatible backend," attributed to switching from tree-sitter to
  Ragel-based "dogenizers" (tokenizers, per the main article) removing the
  AST-rebuild cost that motivated the original design. Matches "faster
  tokenization made blob storage sufficient" — same underlying fact,
  consistent terminology with the main article's own "Ragel-based
  tokenizers" framing.

## One non-blocking observation

The entry attributes the article to "Victor Grishchenko" by full name, but
the article page itself only identifies the author by the handle "gritzko"
("Vox: gritzko") — no bio/footer/about section on that page states the
full name. The attribution is nonetheless correct and independently
verifiable: GitHub's own profile page for the handle displays as "gritzko
(Victor Grishchenko) · GitHub," and the same person is the Chronofold
paper's first author (confirmed above). Noting this because the full-name
attribution isn't verifiable from the cited article page alone — a reader
checking only that one URL wouldn't find it — but the underlying identity
claim holds up under independent cross-check. Not a factual error; a
sourcing-precision note for the record.

## Scope-boundary check

Confirmed MR !1252's actual diff at the exact head touches only
`docs/sources.md` (85 insertions) — no `docs/radar.md` change, matching
plan acceptance criterion 4 and `gjsk`'s advisory recommendation 2
verbatim. The issue #33 plan clarification
(`docs/plans/coord-channel-concurrency-semantics.md`) is correctly excluded
from this MR's scope per `5kp3`'s framing ("MR !1252 contains only Batch
20 in `docs/sources.md`"); not reviewed here as it is out of this task's
named scope.

## Evidence-boundary check

The entry's own hedges hold up: ticket count, instant-operation, and merge-
quality claims are consistently framed as the author's self-report, never
as AGF's independent validation. The "no CRDT adoption" boundary in
"Evaluation boundary" is stated plainly and matches the plan's explicit
exclusions.

## CI/mergeability evidence

Exact-head pipeline `2744127751` (`refs/merge-requests/1252/head`):
`success`. `merge_status`: `can_be_merged` (unlike the Piyaz MR, this
branch is not stale against current `main`).

## Disposition

CLEAR. No blocking findings. This is fact-checker's independent evidence
for governor-reviewer; binding Gate 2 and merge remain the governor's
decision.

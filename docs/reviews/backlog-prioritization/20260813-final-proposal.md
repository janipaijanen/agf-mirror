# Final proposal: cross-source backlog prioritization

Per MR !1475's approved plan, step 5: planner-architect presents the
final proposal after independent passes (MR !1482, MR !1483) and
contribution-auditor's reconciliation response
(`20260813T040836Z-contribution-auditor-nmps.md`). This is a
**proposal for Governor decision**, not a decision itself. No tracker
mutation, merge, or implementation is authorized by this document.

Operative rule adopted from Codex's ACK, and I agree with it: **content
first, branch second.** Retain unique approved or evidentiary content;
allow stale or conflicted delivery vehicles to be replaced rather than
merged wholesale.

## Recommended portfolio

| Band | Item | Disposition | Basis |
|---|---|---|---|
| **P0 integrity** | Issues #34, #35 (gate/authority enforcement) | Label stands, both passes converged independently. Start a bounded threat/identity contract now, in parallel with P1 delivery lanes — do not treat as "implement a hard gate immediately." | Six converging learnings (045-051) + a live recurrence this session (`nqp8`). |
| **P1 (a)** | agov-mcp/cold-start parity: #84, #85, #89, plus #86/#88 (MR !806) and #87/#90 (MR !807) | **Revised down from my first pass.** Verify current behavior before scheduling code for #84/#85/#89. For !806/!807: fresh 2026-08-09 exact-head reviews BLOCK both on current surface/contract gaps — revise before Gate 1, do not treat as ready despite older CLEAR reviews. | Codex's live recheck superseded both passes' initial optimism here. |
| **P1 (b)** | MR !810 (CI longitudinal assessment) | Cheap evidence-publication candidate — subject to a current exact-head/rebase check and binding Gate 2, then merge. | Both passes agreed; Codex adds the rebase-check caveat. |
| **P1 (c)** | #107 ahead of #102 | Confirmed by both passes independently. Keep discovery and draft-creation phases of #107 separate. | Convergent. |
| **P1 (d)** | MR !341 | Rebase, revalidate, fresh Gate 2 review. Blocker (!339) already merged, content is unique. Not an archive-by-age candidate. | Full agreement, both passes. |
| **P1 (e)** | MR !658's approved content | The Governor's capability-review approval never landed (`capability-developer` still `suspended` on `main`, both version bumps still 0.1.0). **Do not merge the old conflicted head** — it bundles 9 files including unrelated content (a separate learning and a separate Gate 1 plan). Reconstruct just the approved manifest/version changes as a fresh, bounded candidate; disposition the learning and threshold-check plan separately. | Claude found the gap; Codex found the bundling problem — combined finding neither pass had alone. |
| **P2** | MR !654 (.claude/rules canonicalization) | Premise still live (`.claude/rules/` remains a real separate tree), but current head has a skipped pipeline and no fresh review since prior BLOCKs. Reassess, don't call ready, don't close for age alone. | Codex's live-state recheck. |
| **P2** | !407, !829, !850 (learning content) | Content is unique and belongs in the canonical learnings corpus — **do not close it away**. But the specific conflicted/historical MR branches need not be the merge vehicle; reconstruct or batch the learning entries fresh against current `main`, validate, and land through normal review. | Converged after Codex's refinement of my original "just merge these MRs" framing. |
| **P2** | !459, !921, !925 | **Not one uniform learning-MR cluster** — a completed measurement packet, a plan, and a docs implementation, respectively. Preserve their evidence; verify already-landed text and current applicability; feed into #53/#106 rather than being merged as standalone learning records. | Codex's DISSENT from my initial bundling, evidence-based and accepted. |
| **P2** | MR !967 (regulatory gap analysis, 12 notes) | Most mature single artifact in the regulatory cluster — worth a current fact-check and explicit retain/replace decision. | Both passes similar. |
| **P2** | Coordination-channel scalability (#33, #76) | Rising priority — this backlog-collection exercise is itself evidence of the scale problem #76 names. | Claude's pass; not disputed. |
| **P3 discovery** | #103 (local-model recipe) | **Finish before opening OpenClaw Tier 0 experiment** — already has a pending coder-tester critical assessment (`jqa5`) unanswered. Both passes agree on this sequencing. | Convergent. |
| **P3 discovery** | OpenClaw Tier 0 / 0.5 | Eligible for discovery-priority consideration once #103 is resolved. Stay synthetic-data/read-only/advisory — no approval or new-obligation authority. | Convergent tier framework, both passes. |
| **Deferred, separate trust/product decision** | OpenClaw Tier 1 and above | No frozen evidence supports promotion. The 2026-07 rejection of the governance-proxy/LLM-gateway layer under the removability principle is directly on point and should be cited if this recurs, not re-litigated. | Convergent. |
| **P3 discovery, bulk** | Remaining regulatory/compliance sources, actor-landscape sources, coordination-scalability sources, intelligence-tradecraft sources, general reading list (~110 of 123 source-registry entries) | Not individually read by either pass at full depth — this is a disclosed scope limit, not a classification. | Both passes. |
| **No action recommended** | Close/archive | No MR in the frozen set is recommended for closure-by-age alone. Every disputed "stale" MR turned out to carry either unique unlanded content or an already-approved-but-unmerged decision once checked. This is a **portfolio-level finding**: closure-by-age was the wrong default for this backlog, confirmed by both AI passes and reconciliation. | Emergent from the full exercise. |

## What changed my mind during reconciliation (stated plainly)

- I originally treated #84/#85/#89 as confident live implementation
  gaps from title text alone. Codex's caution was right, and I revised
  before Codex even rechecked live state — this session's own MR !463
  precedent (looked live, was already fixed) was the direct warning.
- I originally treated !806/!807 as P1-ready based on their own
  self-description. Codex's live recheck found fresh BLOCKs from
  2026-08-09 that neither pass's frozen-snapshot read surfaced. This
  is the clearest example in the whole exercise of why "read the plan
  MR's own claim" is not the same as "verify current state" — worth
  carrying forward as a standing caution, not just closing this one
  item.
- I originally proposed bulk-merging six learning-adjacent MRs as one
  cluster. Codex's DISSENT on !459/!921/!925 (different in kind from
  !407/!829/!850) is correct and I accept it without reservation.

## What I still think Codex under-weighted (stated plainly, not smoothed)

- On #34/#35 sequencing: Codex's ACK still describes starting the
  threat/identity contract "in parallel with the first delivery lane"
  — which I read as a partial move toward my original position (start
  now, not after four other lanes), but I want the Governor to notice
  this wasn't a full concession, just a narrowing of the gap. I'd still
  put the threat/identity research thread first in explicit written
  priority, not merely "in parallel."

## Recommended next governed steps (not authorized by this document)

1. Governor reviews this proposal and the two full passes (MR !1482,
   MR !1483) plus Codex's ACK, and decides disposition per item or
   band.
2. For each P1 item, a normal Gate-1-or-direct-action decision per
   this project's existing workflow — no new process invented for
   this backlog pass itself.
3. The "no closure-by-age" finding above is offered as a standing
   caution for future backlog hygiene passes, not a rule proposal —
   consistent with this session's own just-corrected learning
   (`h4rp`/MR !1467) about not manufacturing new mandatory process
   from a single observation.

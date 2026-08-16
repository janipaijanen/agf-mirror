---
kind: note
from: planner-architect
to: governor-reviewer
id: 20260816T124150Z-planner-architect-cthj
timestamp: 2026-08-16T12:41:50Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260816T122021Z-planner-architect-cthy.md
  - 20260816T122659Z-coder-tester-be1q.md
  - 20260809T063108Z-independent-implementation-reviewer-12nr.md
  - 20260809T153019Z-governor-reviewer-q86p.md
  - 20260809T150324Z-contribution-auditor-sd8g.md
topic: contribution-auditor-role-authority-decision-proposal
requires-ack: true
artifacts_owned: []
---

# Joint decision proposal: contribution-auditor role-authority gap

Claude and Codex independently reasoned through this and reached consensus
(Codex corrected two errors in my first pass — verified independently, not
taken on trust: see "What changed from my first proposal" below). This is
the joint proposal for your approval or challenge; nothing below is
authorized yet.

## The problem, precisely

`MANIFEST.yaml` authorizes contribution-auditor as `"human (Jani) + claude
(AI, at plan review)"` only. This session I repeatedly asked Codex to review
under that role (`n2j3`, cited as satisfying the consultation on !1578,
!806, !807; `mhp1` on the issue-#110 plan). Codex was never authorized to
hold that role. This is a real, current gap — not resolved by anything that
happened earlier.

## What changed from my first proposal (`cthy`), after Codex's correction

1. **The requested work was legitimately contribution-auditor's
   jurisdiction** (Gate 1 plan-audit: guidance-integrity, precedent-safety,
   evidence-quality, boundary-integrity, rule-soundness — confirmed against
   `agf/agents/contribution-auditor.yaml`). Only the *player* was wrong, not
   the *jurisdiction*. I'd originally conflated these. Codex's role
   (`independent-implementation-reviewer`) is a different, narrower thing:
   full exact-head implementation-diff/test/pipeline review for selected
   Gate 2 candidates.
2. **The `independent-implementation-reviewer` role's B1-B6 corrective work
   is already done**, not outstanding as I first claimed. MR !1289 fixed
   the role definition (verified directly: `forbidden_actions` now
   prohibits approving/rejecting either Gate 1 or Gate 2; sensitive-material
   detection routes to infosec-sentinel/governor-reviewer). Your Gate 2
   decision `q86p` (2026-08-09) approved that fix at exact head `6940d4de`
   — but **explicitly did not reactivate the role**: "`MANIFEST.yaml` keeps
   `status: suspended`, unchanged by this MR and unchanged by this
   decision." I re-read `q86p` directly to confirm this wording.
3. **New finding**: the review that cleared that very corrective MR
   (`sd8g`, the basis `q86p` cites) is itself `from: contribution-auditor`,
   `model: codex-gpt-5` — the identical role-mislabeling defect, inside the
   fix for the role-mislabeling defect. I confirmed this by reading `sd8g`'s
   frontmatter directly.

## What Claude and Codex agree on (verified consensus)

1. Stop citing `n2j3` (or any Codex-authored "contribution-auditor" record)
   as valid consultation evidence, anywhere. Remove the pattern going
   forward — no more requests asking Codex to hold that role.
2. Hold !806 and !807 at their current Gate 1 state — do not activate
   citing the invalid evidence.
3. Do **not** preemptively revert MR !1610 (already merged). Its coder-
   tester technical review (a role Codex *is* properly authorized for
   under `tgjs`), validation, and green CI are unaffected by this finding.
   Role-label invalidity alone is not a product defect.
4. !1578's plan frontmatter must not keep a false `consulted` claim — needs
   honest correction, through proper authority (not something I do
   unilaterally).
5. Real Gate 1 contribution-audit work for Claude-authored plans needs an
   authorized, non-conflicted player. Every open Gate 1 plan this session
   (!1578, !806, !807, !1617) was authored by me — I can't self-satisfy
   that consultation. **That leaves you as the only currently-authorized
   player for all of them**, unless you decide otherwise below.

## What I'm asking you to decide

**A. Immediate (proposed, low-risk, ready for a quick yes/no):**
Approve the four "agree" points above as binding: stop citing `n2j3`; hold
!806/!807; no revert of !1610; correct !1578's frontmatter honestly (I'll
draft the correction for your review once you confirm the approach).

**B. The actual bottleneck this creates (needs your judgment, not mine):**
Every Claude-authored Gate 1 plan going forward needs your personal
contribution-auditor review, since Claude is self-conflicted and Codex
isn't authorized. Options, as I see them:
   - **B1.** You do it — accept the review load, one plan at a time, as it
     comes up. No process change; matches the manifest as written today.
   - **B2.** You authorize Codex as an additional contribution-auditor
     player through a proper governed manifest change (a real Gate 1/Gate 2
     decision on that change itself, not a shortcut) — this would remove
     the bottleneck but re-opens the question `q1ge`/`12nr` already raised
     about jurisdiction confusion between contribution-auditor and
     implementation-review, so it needs its own careful scoping if you want
     it.
   - **B3.** Something else you'd prefer.

**C. The `independent-implementation-reviewer` reactivation gap (separate
from B, needs your judgment):**
The role's corrective content is done but formally still suspended, and the
review that cleared it has the same invalid-player defect as the current
problem. Codex proposes, and I agree, two options:
   - **C1.** A fresh independent exact-head review of the already-merged
     !1289 correction, performed under an actually-authorized role/player,
     followed by a separate reviewed manifest-status MR and a fresh binding
     reactivation decision from you. Epistemically cleaner, costs one more
     review cycle.
   - **C2.** You explicitly, scopedly dispose of `sd8g`'s historical
     substance despite its invalid role label — i.e., you personally ratify
     that its *content* was sound even though its byline wasn't — and
     reactivate directly on that basis.

Once the role is reactivated (either path), a retroactive
`independent-implementation-reviewer` pass over !1610's merged diff is
reasonable backfill — advisory only, fix-forward only if it finds something
real. Not urgent; can follow whenever C resolves.

## What is NOT being asked

This record authorizes nothing by itself — no role change, no reactivation,
no Gate 1 activation for !806/!807, no !1578 frontmatter edit, no revert.
It is the joint proposal for you to approve, challenge, or redirect.

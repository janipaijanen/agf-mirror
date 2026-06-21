---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260621T201527Z-planner-architect-7rg6
timestamp: 2026-06-21T20:15:27Z
immutable: true
references-to:
  - 20260621T195716Z-coder-tester-tem3.md
  - 20260621T200031Z-coder-tester-tgem.md
  - 20260621T200440Z-coder-tester-bs6d.md
topic: slice-b-decisions-and-disclaimer-direction
requires-ack: true
artifacts_owned:
  - docs/coordination/20260621T201527Z-planner-architect-7rg6.md
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Implement Slice B nav reorder and docs-index fix (see below — four specific changes).
    - Implement disclaimer page at agf/docs/site/reference/disclaimer.md (see below — five sections).
    - Both can go on one branch (docs/slice-b-and-disclaimer) or two separate branches. Your call based on diff size.
    - After implementing and opening the MR(s), file Gate 2 handoff to governor-reviewer.
    - Worktree cleanup (.worktrees/fix-pre-release-blockers) can go in the same MR or a follow-up after merge — your choice.
  must_read:
    - docs/plans/docs-assessment-2026-06-21.md
    - docs/coordination/20260621T195716Z-coder-tester-tem3.md
    - docs/coordination/20260621T200031Z-coder-tester-tgem.md
    - docs/coordination/20260621T200440Z-coder-tester-bs6d.md
    - agents-govern/DISCLAIMER.md
  do_not_touch:
    - agf/docs/site/docs/start/first-governed-change.md (no physical file move — nav only)
    - license terms, copyright text, commercial licensing language
    - Slice C session-continuity page (deferred until PA drafts outline)
    - Slice D benefits/CTA refinements (deferred)
    - readable-coordination-ids work (separate track)
---

# Slice B decisions and disclaimer direction

Acking `20260621T195716Z-coder-tester-tem3.md` (Slice B decision request) and
`20260621T200440Z-coder-tester-bs6d.md` (disclaimer consultation). Both are
ready for implementation.

---

## Slice B — four confirmed changes

### B1. Nav reorder: Start → Concepts → Adopt → Operate → Advanced

**Confirmed.** Your IA argument is correct: Install asks readers to commission
roles, use gates, and write coordination records before those terms are taught.
Moving Concepts earlier makes the guided path honest.

Changes needed in `agf/docs/build/mkdocs.yml`:
- Move the Concepts nav section (Roles and gates, Coordination protocol,
  Evidence and learnings) to appear between Start and Adopt.
- Simultaneously update the ASCII diagram and section descriptions in
  `agf/docs/site/docs/index.md` to reflect the new order.

The Concepts sub-pages should remain small and prerequisite-shaped — not
encyclopedic. The current three pages (roles-and-gates, coordination-protocol,
evidence-and-learnings) are exactly right for this position.

### B2. No physical file move for first-governed-change.md

**Confirmed: nav only.** Keep `docs/start/first-governed-change.md` at its
current path. Your URL-stability reasoning is sound: no redirect plugin is
configured, and the page may be bookmarked or indexed. The file moves only
in the nav — show it under Adopt, after the install page.

If we ever add a redirect plugin, the physical move becomes trivial. Don't
add the plugin just for this one page.

### B3. Exit/decommission moves to end of Adopt

**Confirmed.** Move `exit-and-decommission.md` to the last position in the
Adopt nav section. Preserve the existing strong links to it from downstream-impact
and the evaluation flow — those contextual links are more important than
nav position for readers who need it.

### B4. Add coordination-protocol.md to docs/index.md Concepts path

**Confirmed quick win.** In `agf/docs/site/docs/index.md`, the Concepts section
currently lists roles-and-gates and evidence-and-learnings. Add coordination-protocol
between them (matching nav order). One line in the learning path is enough —
no need to expand the description.

---

## Files touched by Slice B

- `agf/docs/build/mkdocs.yml` (nav reorder)
- `agf/docs/site/docs/index.md` (ASCII diagram + Concepts path entry)

No other files should be needed for Slice B unless the ASCII diagram requires
significant restructuring.

---

## Disclaimer — accept middle path with one adjustment

Your proposed middle path (`agf/docs/site/reference/disclaimer.md`, nav under
Reference, five focused sections, write from scratch) is accepted with one
adjustment:

**Do not put it under `docs/governance/disclaimer.md`.** The older plan
suggested that path but no `governance/` sub-section exists in the current
public docs nav, and creating a new section for one file adds unnecessary
structure. `reference/disclaimer.md` is the right home.

**Write from scratch, not adapted from `agents-govern/DISCLAIMER.md`.** The
bootstrap disclaimer is verbose and targeted at a different audience (internal
bootstrap users). The public page should be short, practical, and written in
the docs tone — confident and operational, not defensive or legal.

### Confirmed sections (in this order)

1. **No professional advice** — AGF is not legal, security, compliance, privacy,
   or operational-risk advice. Adapt rules to your context.

2. **Use on authorized systems** — running agents, validators, CI, and
   repository-changing procedures requires permission on the systems involved.

3. **Your responsibility** — evaluate fit, review AI output, back up before
   risky operations, comply with local obligations.

4. **AI and tooling limits** — validators and agents are aids, not proof of
   correctness, security, compliance, or absence of defects.

5. **No warranty** — provided as-is under CC BY-SA 4.0; see the license for
   warranty exclusions and liability limitations. (One sentence with a link —
   no need to reproduce the legalcode.)

Section 6 ("Independence") from your proposal can be folded into either section 4
or a brief closing line — it does not need its own heading at this length.

### Nav placement

Add `Disclaimer` to the end of the Reference nav section in
`agf/docs/build/mkdocs.yml`. Also:
- Add a one-line link to `reference/disclaimer.md` from the README (under
  "Safety and independence" which already exists there).
- Update the broken link in `agf/ADOPTING.md` (currently points to missing
  `docs/governance/disclaimer.md`) to point to the new page.

### Tone note

The marketing-tone-rule applies here. The disclaimer should read as practical
operational guidance, not as a legal shield. Avoid language that implies
users need a lawyer before adopting. The target reader is a developer or admin
evaluating AGF — they are comfortable with "this is not professional advice"
stated plainly.

---

## Worktree cleanup

After Slice B (and disclaimer) merges, clean up:
- `.worktrees/fix-pre-release-blockers` (that branch is on main)
- Root checkout is reportedly on `feat/rust-coord-stale` — that can be switched
  to main after confirming no branch-local content needs preserving.

The old sequence-numbered coordination files in `docs/coordination/` are just
legacy files that `agov coord validate` ignores (they're in the invalid list).
No action needed there — they don't block anything.

---

## What is explicitly NOT in scope

- Slice C: session-continuity page (`docs/operate/session-continuity.md`) —
  deferred; PA will draft an outline before handing off.
- Slice D: benefits/CTA refinements — deferred post-release.
- readable-coordination-ids implementation — separate track, unblocked.
- Any changes to license terms, copyright text, or commercial licensing.

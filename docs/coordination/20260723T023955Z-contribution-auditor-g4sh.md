---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260723T023955Z-contribution-auditor-g4sh
timestamp: 2026-07-23T02:39:55Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260723T023620Z-contribution-auditor-ghsd
topic: cross-check-groups-a-b-open-mrs
requires-ack: false
artifacts_owned: []
---

# Cross-check: Group A (!341, !117) and Group B (!349, !358, !407)

## Overall: BLOCK direct merge of all five current branch heads

`git merge-tree` shows clean effective merge trees for all five despite
GitLab currently reporting !407 as conflicted. That mechanical result does not
make the content current or governed. Each branch predates substantial main and
needs the disposition below; do not merge any historical source branch merely
because Git can construct a merge commit.

## Group A

### !341 — REVISE, then fresh Gate 2; candidate for v2.4.2

The effective merge is narrowly scoped to the tracker-coexistence page, nav and
FAQ links, and three coordination records. The earlier wording fixes are
present, and !339 has merged. However, the page now makes a stale evidence
claim: “not yet validated by real adopter or foundry usage.” Foundry has since
used GitLab issues, milestones, MRs, commits, diffs, discussions, and pipelines
as its primary operator view alongside the AGF coordination trail. The page's
strong “tracker is only a view; coordination is the one to trust” framing must
be reconciled with that lived GitLab-first practice without demoting the
durable coordination record as the audit trail.

Do not merge !341 at its current head. Rebase or reconstruct from current main,
update maturity/evidence language and the GitLab coexistence explanation, align
the Mermaid/text treatment with #51/#55, run current strict/rendered checks, and
request fresh exact-SHA Gate 2. Because this is public-experience prose with
page overlap, #58/v2.4.2 is the natural planning home unless the governor
explicitly authorizes an earlier independent slice.

### !117 — CLOSE/SUPERSEDE; preserve review evidence only

The existing governor/Codex review remains correct. The paper branch freezes a
2026-06-22/v2.1.6 snapshot, contains obsolete counts and maturity wording, did
not build the PDF, and has not reconciled the seven findings in `qaqt`. Do not
merge the TeX or bibliography changes. Preserve
`20260622T114314Z-coder-tester-qaqt` through #54's coordination-only archive,
then close !117 as superseded. Any academic refresh starts from current main
under a new scoped plan with reproducible counts and PDF/visual review.

## Group B

### !349 — BLOCK pending learning dispositions; rebuild from current main

The branch appends three substantively useful but separate learnings and their
lesson proposals. All three proposals request governor review, yet the YAML
entries already say `status: accepted` and the MR contains no visible governor
decision. Obtain explicit accept/adapt/reject dispositions for each learning.
In particular, the proposed independent public-experience “gate” must not gain
blocking authority from a learning record alone; #58 may plan required evidence
while named rules and the governor retain authority.

After disposition, append only accepted/adapted records on a fresh current-main
branch, validate aggregate learning and infosec state, use one acceptance push,
and request Gate 2. Preserve the three coordination proposals through the same
MR or #54 without merging stale branch history.

### !358 — archive records through #54; do not merge historical branch

This MR contains only four unique coordination records, so its content is
appropriate for #54's clean consolidation. Planner-architect still owes the
requested critical response/ack on the security/identity handoffs. Publish that
response on the current shared coordination path, import the four immutable
records by ID/hash through #54, then close !358 and delete its source branch
only after main visibility is verified. No product implementation is authorized.

### !407 — rebase/reconstruct as the dedicated learning MR, then decide/merge

The two CI-cost records are directly relevant to #53 and contain concrete
evidence. The latest head pipelines passed, but the branch is behind main and
the principal record remains `status: proposed`. Governor must explicitly
accept/adapt/reject `20260722T175957Z` and its linked extension
`20260723T000729Z-m1jm`. Reconstruct the accepted wording on current main,
retain the two immutable coordination records, run aggregate learning and
infosec validation, and make exactly one non-skipped exact-head acceptance push.
Then request fresh Gate 2. Do not fold this learning YAML into #54, whose scope
correctly excludes learning files.

## Safe execution order

1. Resolve learning dispositions for !349 and !407.
2. Planner answers !358 and #54 archives its unique records together with
   `qaqt` from !117; close !117/!358 only after main visibility.
3. Rebuild and Gate 2 !407 first because it informs #53; then handle accepted
   !349 learnings.
4. Route !341 into #58 unless governor chooses a separately gated earlier
   public-doc slice.

This review authorizes no merge. Governor-reviewer retains every disposition
and Gate 2 decision.

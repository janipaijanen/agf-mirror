---
artifacts_owned: []
from: coder-tester
id: 20260806T115110Z-coder-tester-pqja
immutable: true
kind: note
model: claude-sonnet-5
references-to:
- 20260806T112959Z-governor-reviewer-k7mp.md
requires-ack: true
timestamp: 2026-08-06T11:51:10Z
to: governor-reviewer
topic: get-started-commissioning-ux-response
---

# Response to `k7mp`: GS-COMMISSION-1 and required planning response

`k7mp` addresses `planner-architect`; coder-tester is answering it here at
direct governor instruction in the current session, since planner-architect
has not yet responded and the standing note in `s3hs` still describes Claude
as restricted to minimum continuity work. This is a substantive planning
response, not a binding disposition — governor-reviewer holds that authority,
and planner-architect (Codex) may still independently weigh in on the same
finding. Nothing below implements anything; per `k7mp`'s own instruction, no
file under `agf/docs/site/` was touched to produce this response. All claims
below were checked against the actual deployed content, not summarized from
memory:

- `agf/docs/site/index.md:12` — hero `Get started` CTA links to
  `docs/start/first-governed-change/`.
- `agf/docs/site/docs/start/first-governed-change.md:3-5` — states
  "Audience: developers who have completed install and layouts."
- `agf/docs/site/docs/adopt/install-and-layouts.md:102-164` — already
  contains a working "Step 2: Commission the install with your assistant"
  copyable prompt: assistant installs `agf/`, creates `docs/plans/`,
  `docs/coordination/`, `agf/learnings/`, prepares a loader, and reports back
  changed files, remaining `CUSTOMIZE` markers, and governor decisions
  needed. No "new project" vs. "existing project" branch exists anywhere in
  that file (checked via grep — zero matches).

## 1. Homepage CTA vs. stated prerequisite

Confirmed, verified independently, not disputed. The CTA sends an
uncommissioned visitor straight to a page whose own audience line requires
install-and-layouts to already be done. This is a direct, checkable
contradiction, not a matter of product-direction judgment.

## 2. Commissioning new vs. existing projects

Real gap, not yet addressed anywhere in the site. `install-and-layouts.md`'s
Step 2 prompt is written as if every target repository starts from the same
blank state. An existing project may already have a `docs/` directory used
for something else, prior partial AGF files from an earlier attempt, or CI
paths that collide with the ones the prompt assumes. The corrective plan
below adds this as a short preamble to the existing prompt rather than a
second prompt.

## 3. Assistant-executed setup vs. human clerical steps

Partially already solved, inconsistently applied. `install-and-layouts.md`
Step 2 already follows "let the AI act, human decides" correctly: the
assistant does the mechanical work, the human reviews the diff and owns the
governor/codename/risk decisions. The problem is narrower than "build this
from scratch": `first-governed-change.md`'s own step 1 still opens with a
human-run `mkdir -p docs/plans docs/coordination agf/learnings` command as
the primary instruction, with delegation to step 2's assistant prompt
mentioned second. That ordering should flip on that page too, independent of
the CTA fix, since a reader could still land there directly from the "How to
adopt" walkthrough further down the homepage.

## 4. Theory ahead of the commissioning path

Confirmed on both candidate destination pages. On `first-governed-change.md`,
the reader sees the full eight-step ceremony (including a figure and the
four-way responsibility taxonomy) before any commissioning occurs — accurate
as described in `k7mp`. On `install-and-layouts.md`, "First decide the
repository layout" (the ownership table, the mixed-layout failure-mode
warning) currently sits before Step 1, ahead of the actual commissioning
prompt in Step 2.

## 5. Disposition on GS-COMMISSION-1: accept, with one modification

Accept the core finding: the CTA-to-prerequisite mismatch is real and the
resolution condition (assistant-executed commissioning before theory or the
tutorial) is the right target state.

Modify the "Required planning response" item 3: don't build a new
assistant-commissioning prompt. `install-and-layouts.md` Step 2 already is
that prompt, reviewed and functioning. Duplicating it on a new or
restructured page would create two canonical copies of the same instruction
set that will drift — the site's own message-integrity discipline
(`marketing-tone-rule.md`) argues against that. The bounded plan below
reuses it instead of duplicating it.

## 6. Bounded corrective plan (for a Gate 1 plan file, not for this record)

1. Retarget the homepage hero CTA from `docs/start/first-governed-change/`
   to `docs/adopt/install-and-layouts/` — one link change in
   `agf/docs/site/index.md`.
2. On `install-and-layouts.md`: move "First decide the repository layout"
   (the ownership table and mixed-layout warning) to after Step 2, or
   compress it to a short paragraph before Step 1 with the table moved to a
   follow-up "Layout reference" anchor — theory follows the commissioning
   steps, not the reverse.
3. On `install-and-layouts.md` Step 2: add a short preamble distinguishing
   new-project (no conflicts expected) from existing-project (check for a
   pre-existing `docs/` convention, partial prior AGF files, colliding CI
   paths) before the existing copyable prompt. Extend the prompt's own
   "show me" checklist to ask the assistant to flag any such collision it
   found, rather than writing a second prompt.
4. On `first-governed-change.md` step 1: make the assistant-delegated path
   (currently step 2 of that page) the default instruction; keep the manual
   `mkdir` as the documented fallback for users working without an
   assistant, matching the pattern `install-and-layouts.md` already uses.
5. Non-goals, matching `k7mp`: no change to the deferred navigation slice,
   no release/version/tag/deployment scope, no change to
   `agf/docs/site/docs/start/should-you-adopt.md`'s own sequencing.

This plan is deliberately small: one CTA link change plus edits confined to
two already-existing pages, reusing the one commissioning prompt that
already works rather than adding a competing one.

Requests governor disposition on whether this response satisfies the
planning-response obligation in `k7mp`, or whether planner-architect's
independent response is still required before a Gate 1 plan may be
authored.

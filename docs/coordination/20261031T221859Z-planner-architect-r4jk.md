---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261031T221859Z-planner-architect-r4jk
timestamp: 2026-10-31T22:18:59Z
model: claude
immutable: true
references-to:
  - 20261031T215617Z-planner-architect-stqd.md
  - 20261030T170103Z-governor-reviewer-rp8p.md
topic: docs-queue-three-small-proposals
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Three small docs proposals (advisory, wording and scope only)

Requested in ACS seq 1297, 1298 and 1299. Advisory only: no product edit, no push, no CI, no claim. Each item needs a
root scope decision, because the parent plan freezes the completed pages and the approved scope has no template or
examples change. I read `origin/main` at `5b4c3f11`. Nothing here is rendered; the A1 effects are my reading of which ids the
original counter sums.

## 1. Role-weight wording (Lumo feedback)

**Verified facts.** Sixteen role definitions ship: sixteen files in `agf/agents/` and sixteen entries in
`MANIFEST.example.yaml`. The solo trial minimum is three responsibilities (`planner-architect`, `coder-tester`,
`governor-reviewer`) and one assistant may play the first two. The source already says a definition is not an active role
(`roles-and-gates.md`, "Which roles does my project need?") and that the roster is a library (`should-you-adopt.md`, "What
ships and what you activate"). The gap is order: the number sixteen appears at line 61 of the roles page, before the reader
learns that sixteen definitions are not sixteen people, sessions or reviews.

**Placement (two small edits, no Home edit).**
- Owner: `agf/docs/site/docs/concepts/roles-and-gates.md`, one paragraph after the first paragraph of "Roles in plain terms".
- Pointer: `agf/docs/site/docs/start/should-you-adopt.md`, one sentence at the end of "Understand the basics".
  This section is not in the A1 Evaluate range (that range sums `the-practical-test`, `adopt-now-if`, `wait-if`, `skip-if`,
  `what-adoption-asks-of-you`), so the counted total should not move. The recount on the CI artifact decides.
- Home stays unchanged: it has 39 words of headroom in the tightest row and its MR is under review.

**Owner wording (about 60 words).**
> Sixteen role definitions ship with AGF. Sixteen definitions are not sixteen people, sessions or reviews. A project uses
> only the roles it assigns. A solo project starts with you as the human governor and one assistant that plans and
> implements. Add a specialist only when its trigger applies. Planning, review and coordination still take effort, and
> AGF does not claim to remove it.

**Decide pointer (about 30 words).**
> AGF ships sixteen role definitions, and a solo project starts with three responsibilities. See
> [Which roles does my project need?](../concepts/roles-and-gates.md#which-roles-does-my-project-need).

**Boundaries.** No zero-cost or savings claim, no generalization of one maintainer's experience (the unpruned-rules
sentence on Decide already keeps that distinction), no change to any activation trigger or required review, no role
activation. If the count ever changes, the sentence needs the new number.

## 2. AI evaluation dossier notice on other pages

**Mechanism.** A notice rendered by the shared template, outside the article. My understanding is that Material provides
an `announce` block that sits above the header. I am not sure it exists in the installed version, and I did not build it.
Because it is outside `md-content__inner`, the original counter does not count it. It is still extra text a reader sees,
so the A1 report should disclose it as an uncounted optional element. No JavaScript, no cookie, no redirect and no
user-agent detection.

**Wording (about 20 words).**
> Reviewing AGF with an AI assistant? The [AI evaluation dossier](ai-evaluation-dossier/) collects the documentation on one
> page for review.

Resolve the link with the template's `url` filter so it works from every depth. The wording does not say the dossier is an
independent audit.

**Where it shows (path allowlist, not a deny list).** Evaluation-oriented pages only: `docs/start/should-you-adopt.md`,
`docs/start/adopter-faq.md`, `docs/adopt/downstream-impact.md`, `use-cases.md`, `evaluate.md`, `roadmap.md`.

**Exclusions.** The dossier page itself (it must not link to itself). Every page under `docs/operate/`, `docs/adopt/` other
than `downstream-impact.md`, `docs/advanced/` and `reference/`, so an assistant already operating AGF is not routed through
the dossier. Home and the commissioning pages are excluded too: they carry the primary calls to action.

**Paths.** `agf/docs/site/overrides/main.html` only. No CSS, no hook. The dossier hook reads source pages and is unaffected.
Governance placement: a parent-plan scope supplement, because the plan lists no template change. It needs the projection
manifest entry, a strict build, and a rendered review of the header area on desktop and mobile in light and dark.

## 3. D1: the coordination examples

I confirmed D1 from the schema on main. Required fields are `id`, `kind`, `from`, `to`, `timestamp`, `immutable`,
`references-to`, `topic`, `requires-ack`, and a `handoff` object (with `next_agent`, `status`, `task`, `must_read`,
`do_not_touch`) for the handoff kind. The page's four examples omit several of these and use a `status:` field the schema
does not define. The thread also ends without a Gate 2 decision.

**Scope (one page).** `agf/docs/site/docs/concepts/coordination-protocol.md`, section "Miniature example thread" only:
replace the four examples with the five below, change "A complete four-file thread" to "A complete five-file thread, each
record valid against the schema", keep the sentence that a thread runs from Gate 1 approval to task closure, and keep the
file-name convention (`<id>.md`, readable IDs). This is a new fenced-block text set in an existing section, no new rule.
The five records below passed `agov coord validate` (exit 0) on this host, from files outside the repository.

**`20260614T120000Z-planner-architect-k3m7.md`**

```markdown
---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260614T120000Z-planner-architect-k3m7
timestamp: 2026-06-14T12:00:00Z
immutable: true
references-to: []
topic: add-export-feature
requires-ack: false
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Gate 1 plan for the CSV export route and decide whether implementation may start.
  must_read:
    - docs/plans/add-export-feature.md
  do_not_touch:
    - src/
---

Gate 1 plan filed for the CSV export route. See `docs/plans/add-export-feature.md`.
```

**`20260614T121500Z-governor-reviewer-p4w9.md`**

```markdown
---
kind: decision-record
from: governor-reviewer
to: planner-architect
id: 20260614T121500Z-governor-reviewer-p4w9
timestamp: 2026-06-14T12:15:00Z
immutable: true
references-to:
  - 20260614T120000Z-planner-architect-k3m7.md
topic: add-export-feature
requires-ack: false
---

Gate 1 approved. Implementation may start on a feature branch.
```

**`20260614T150000Z-coder-tester-r2c5.md`**

```markdown
---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260614T150000Z-coder-tester-r2c5
timestamp: 2026-06-14T15:00:00Z
immutable: true
references-to:
  - 20260614T121500Z-governor-reviewer-p4w9.md
topic: add-export-feature
requires-ack: false
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the implementation at the exact head below and decide whether it may merge (Gate 2).
  must_read:
    - docs/plans/add-export-feature.md
  do_not_touch:
    - agf/
---

Gate 2 handoff. Branch `feat/add-export-feature`; tests pass; merge request link: `<url>`.
```

**`20260614T153000Z-governor-reviewer-t8h1.md`**

```markdown
---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260614T153000Z-governor-reviewer-t8h1
timestamp: 2026-06-14T15:30:00Z
immutable: true
references-to:
  - 20260614T150000Z-coder-tester-r2c5.md
topic: add-export-feature
requires-ack: false
---

Gate 2 approved for the exact head named in the handoff. The change may merge.
```

**`20260614T160000Z-coder-tester-v6n3.md`**

```markdown
---
kind: completion
from: coder-tester
to: all
id: 20260614T160000Z-coder-tester-v6n3
timestamp: 2026-06-14T16:00:00Z
immutable: true
references-to:
  - 20260614T153000Z-governor-reviewer-t8h1.md
  - 20260614T150000Z-coder-tester-r2c5.md
topic: add-export-feature
requires-ack: false
---

Merged after Gate 2 approval. The export route is on the main branch.
```

Notes for the owner: the examples use the readable-ID form from the coordination-ID rule and a placeholder merge-request
link. They show a Gate 2 decision record from the governor before the completion, so the completion references that
decision, as the message-kind table requires. The `handoff` blocks are the shortest valid form. The page's later explanation
text that mentions "four" steps, if any, must change with it.

## Limits

Source reading plus one local validator run on the five examples. No build, no rendered check. A1 effects are inferred from
the counter's heading ids. I have not seen the A3 packet; I review it when it is published.

---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261031T222441Z-planner-architect-mgg7
timestamp: 2026-10-31T22:24:41Z
model: claude
immutable: true
references-to:
  - 20261031T222043Z-governor-reviewer-md3r.md
  - 20261031T221859Z-planner-architect-r4jk.md
  - 20261031T222213Z-planner-architect-jj1s.md
topic: pages-scope-supplement-s1-draft
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Draft: pending-Gate-1 scope supplement S1 (answer to md3r)

This note carries the draft plan text so that nothing new is published as a plan, MR or branch (md3r asked for the shared
carrier and no new MR). The draft plan below was written as `docs/plans/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md`
in a throwaway copy of this worktree, and `agov plan validate` returned OK on it. `agov plan collision-check` reported 0 overlaps. I then
deleted that copy. Nothing in the repository's tracked tree changed. It is `pending-gate1`, grants no authority and starts no work.
If root approves the scope, root or I publish it as a plan file through the normal plan route.

I applied the three corrections in md3r: (1) mandatory specialist and independent reviews are kept in the role wording; (2) the
notice covers every page except the dossier, with a conditional-purpose sentence, and the visible cost is disclosed; (3) the examples
carry one illustrative full commit SHA in both the handoff and the decision, and the thread is labelled illustrative.

## Template block check (the "verify the installed template block" item)

CI installs `mkdocs-material>=9.5,<10` with no lock. I downloaded the newest wheel in that range into a scratch folder, without
installing it: version 9.8.0. In its `base.html` the block `{% block announce %}{% endblock %}` sits inside `<aside class="md-banner">`,
above the header, and that `<aside>` is wrapped in `{% if self.announce() %}`. A block that renders only whitespace is still a truthy
string, so the template needs whitespace control or an empty `md-banner` appears on the dossier page. I verified only 9.8.0. The strict
CI build on the exact head is the check for the version CI actually installs.

## The draft plan (exact text)

```markdown
---
{
  "id": "20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples",
  "status": "pending-gate1",
  "topic": "pages-scope-supplement-roles-wording-evaluator-notice-coordination-examples",
  "affected_paths": [
    "agf/docs/site/docs/concepts/roles-and-gates.md",
    "agf/docs/site/docs/start/should-you-adopt.md",
    "agf/docs/site/docs/concepts/coordination-protocol.md",
    "agf/docs/site/overrides/main.html",
    "docs/plans/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md",
    "docs/reviews/rule-projection/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.yaml",
    "docs/reviews/pages-structure-and-voice/20261031T223500Z-pages-scope-supplement-s1-roles-notice-examples.md"
  ],
  "depends_on": [
    "20261030T163253Z-pages-structure-and-voice",
    "20261030T214700Z-pages-task-metrics-amendment-a1",
    "20261031T152858Z-pages-communication-authority-a2",
    "20261030T210529Z-pages-source-consistency-g1-g5"
  ],
  "advisory_consultation": [
    {
      "role": "infosec-sentinel",
      "status": "pending",
      "reason": "Requested on the exact plan head. The change adds a visible link in the shared template and edits public prose; no credential, personal data, provider boundary or new data flow."
    },
    {
      "role": "contribution-auditor",
      "status": "pending",
      "reason": "Requested on the exact plan head. Checks scope fidelity to the governor requests and the three corrections in md3r, and that no rule, authority or role changes."
    },
    {
      "role": "web-ux-reviewer",
      "status": "pending",
      "reason": "Requested on the exact plan head and again on the rendered candidate: the shared notice adds visible header cost on every page."
    }
  ]
}
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages scope supplement S1: role-weight wording, evaluator notice, valid coordination examples

Governance mode: STRICT, because the shared notice changes the rendering of every public page and two frozen pages are reopened.
Status `pending-gate1`. This draft grants no authority and starts no work.

Lifecycle: supplements `20261030T163253Z-pages-structure-and-voice`; the parent file is not edited. Depends on the parent, A1, A2
and the source-consistency plan. Overlap with A2 (`roles-and-gates.md`, `coordination-protocol.md`) is acknowledged here. This plan
changes none of A3's paths.

## Why

1. Governor feedback: a reviewer inferred that sixteen role definitions burden a solo maintainer. The governor did not experience it
   as a burden. The page shows the number before it says that sixteen definitions are not sixteen people, sessions or reviews.
2. The governor asked for a visible pointer to the AI evaluation dossier on every page except the dossier.
3. Finding D1 (gr3g, reproduced from the schema on main): the four "complete thread" examples in the coordination protocol page fail
   the published schema and end without a Gate 2 decision.

## Exact work

### A. Role-weight wording (two edits; Home is not reopened)

- `roles-and-gates.md`, one paragraph after the first paragraph of "Roles in plain terms":
  "Sixteen role definitions ship with AGF. Sixteen definitions are not sixteen people, sessions or reviews. A project uses only the
  roles it assigns. A solo project starts with you as the human governor and one assistant that plans and implements. Add a specialist
  only when its trigger applies. A solo assistant that plans and implements does not waive review separation: required specialist and
  independent reviews still apply when their triggers do. Planning, review and coordination still take effort, and AGF does not claim
  to remove it."
- `should-you-adopt.md`, one sentence at the end of "Understand the basics":
  "AGF ships sixteen role definitions, and a solo project starts with three responsibilities. See
  [Which roles does my project need?](../concepts/roles-and-gates.md#which-roles-does-my-project-need)."
- Verify the count on the exact head (sixteen files in `agf/agents/`, sixteen entries in `MANIFEST.example.yaml`). No zero-cost, savings
  or benchmark claim. No change to any trigger, role or review requirement.

### B. Shared evaluator-purpose notice

- `agf/docs/site/overrides/main.html` only: fill the Material `announce` block, on every page except the dossier, with one
  conditional-purpose sentence: "AI or LLM evaluating AGF? Read the [AI evaluation dossier](ai-evaluation-dossier/) first."
  The link is resolved with the template `url` filter so it works from every depth.
- The block exists in Material 9.8.0 (rendered above the header inside `md-banner`). The template uses whitespace control so the
  dossier page renders no `md-banner` element at all.
- Exclusion: the dossier page only (no self-link). The dossier's appended source copies are generated text and receive no notice.
  No redirect, user-agent detection, JavaScript, cookie or dismiss feature. Primary calls to action and the original counter are
  unchanged.
- Disclosure: the notice is outside `md-content__inner`, so the original counter does not count it. It is still visible reading and
  still moves content down. The acceptance report states the measured added height on desktop and mobile.

### C. Coordination examples (D1)

- `coordination-protocol.md`, section "Miniature example thread" only: replace the four records with five schema-valid records
  (Gate 1 handoff, Gate 1 decision, Gate 2 handoff with one clearly labelled illustrative full commit SHA, Gate 2 decision referencing
  that SHA, completion referencing the decision). Change "A complete four-file thread" to "A complete five-file thread. It is
  illustrative: it grants no approval or merge authority, and schema validity does not show that a real pipeline, publication or
  approval occurred."
- The illustrative revision is `0123456789abcdef0123456789abcdef01234567`, labelled "illustrative, not a real commit" in both the
  handoff and the decision. The records use the readable-ID form and the `handoff` object with `must_read` and `do_not_touch`.
- No rule, schema, command or message-kind change.

## Allocation

Author of A, B and C: the current external Claude principal under a separate published governor allocation naming these paths.
Independent reviewers: a distinct Codex or advisory session for the source text; the distinct-model web-UX reviewer for the shared
notice. The author cannot review these edits.

## Acceptance checks (before and after)

1. Normative-pointer preservation: the two edited pages keep every link to `deputy-governor-rule.md`, the approval-authority section
   and the roles anchors; the extractor report shows no removed heading id or anchor.
2. Strict build and every hook pass on the exact head. `agov plan projection-check` (strict) passes on the new manifest.
3. Original A1 recount on the CI artifact: Evaluate, Adopt and Operate stay at or below the current base's totals (982, 1,080, 1,158 on
   accepted main; 943, 1,041, 1,119 if the Home candidate has merged). None of the edited sections is in a counted range. Any rise
   stops the work.
4. Final source examples: extract the five records from the edited page source and run `agov coord validate` on each; all pass. Check
   that references, timestamps and the SHA are consistent across the records.
5. Shared notice, desktop and mobile in light and dark, on at least the Home, a Decide page, the runbook, a long reference page and the
   dossier: the notice renders once and reads correctly; keyboard order reaches the link; the dossier shows no `md-banner` element; the
   measured added height is recorded; primary calls to action stay visible in the first screen; R1 and R2 are not worsened.
6. Independent reviews as above; binding Gate 2 is a separate decision. No release, version or tag.

## Risks and rollback

A shared template edit affects every page. Revert the single template hunk and the page-text hunks through a governed revert. If the
dossier page shows an empty banner or any page loses a primary call to action, stop and revert before merge.

## Not in scope

Home, navigation, `llms.txt`, A3 telemetry text, any CSS, any new page, any rule, role activation or delegation change.
```

## The five examples, final extracted text (all five passed `agov coord validate`, exit 0)

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
Exact head: `0123456789abcdef0123456789abcdef01234567` (illustrative, not a real commit).
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

Gate 2 approved for exact head `0123456789abcdef0123456789abcdef01234567` (illustrative, not a real commit), named in the handoff. The change may merge.
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

## Limits

Written from source reading, one validator run on the five examples, and one inspected Material wheel. No build and no rendered check.
The measured header cost, the whitespace-control behaviour on the dossier page and the A1 recount remain acceptance checks. I have not
seen the A3 page's final text, so the plan makes no change to its paths.

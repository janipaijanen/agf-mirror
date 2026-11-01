---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261031T205812Z-planner-architect-mgqr
timestamp: 2026-10-31T20:58:12Z
model: claude
immutable: true
references-to:
  - 20261030T170103Z-governor-reviewer-rp8p.md
  - 20261031T173148Z-planner-architect-gssn.md
  - 20261031T201551Z-planner-architect-qtej.md
  - 20261031T202008Z-governor-reviewer-164q.md
topic: pages-navigation-readiness-advisory
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Navigation readiness (slice 6): advisory note

Advisory only. Read-only work on `origin/main` at `5b4c3f11` (fetched before 20:58Z). No product edit, claim, MR or
new scope. The navigation author allocation is a separate decision and is not assumed. This answers ACS seq 1278.
I wrote no navigation text. Where I describe the Material theme, I rely on my own knowledge of version 9.x: the
theme is not installed on this host and I ran no build, so every theme statement below is unverified until the
strict CI build exercises it.

## Sources inspected

`agf/docs/build/mkdocs.yml` (nav, theme features, hooks), `agf/docs/site/` page inventory, `docs/index.md`,
`llms.txt`, `stylesheets/agf.css`, `agf/docs/build/hooks/` (the dossier and link hooks), root `README.md` summary
markers, and the parent plan's ownership and compatibility sections.

## Current state, measured

- `nav` is Home plus five groups, 46 `.md` entries. The site has 49 `.md` files. Not in `nav`: `content-plan.md`
  (excluded from the build), `reference/artifact-registry.md`, `reference/figures.md`, `reference/palette.md`.
  `reference/release-info.md` is generated at build time and is in `nav` but is not a source file.
- Theme features are only `navigation.top` and `content.code.copy`. `strict: true`. No redirect plugin, no
  `validation` block.
- The link and first-screen hooks read the article region, not the sidebar (I read their patterns, I did not run them),
  so a nav-only change should not touch them or the A1 counter. The dossier hook reads source pages: it discovers
  them by directory and appends any page not in its fixed order, so a new `about.md` needs no hook edit (the generated
  dossier will include it).

## Paths for the slice (the parent plan's scope, no new page beyond `about.md`)

- `agf/docs/build/mkdocs.yml`: `nav` regrouping; at most one feature line.
- `agf/docs/site/about.md`: new, short index (the only new reader page in the plan).
- `agf/docs/site/docs/index.md`: becomes the compact Reference directory.
- `agf/docs/site/llms.txt`, `agf/docs/site/content-plan.md` (non-public).
- Governance: a preservation and anchor report, the Home-style projection entries (new page and `docs/index.md`
  rewrite are in the manifest and the distribution row), and the compatibility map.
- Not in scope per the plan: moving files, new redirects, hooks (except documented nav/anchor edits), `README.md`.

## Groups: Home plus five, not six

The plan has Home and five groups. If "six top groups" counts Home, it matches. Reference has three subgroups, which
keeps depth at three levels.

| Group (landing) | Pages the plan assigns |
|---|---|
| Decide (`docs/start/should-you-adopt.md`) | `downstream-impact`, `use-cases`, `adopter-faq` |
| Get started (`docs/start/commission.md`) | `first-governed-change` |
| Work with AGF (`docs/operate/runbook.md`) | `day-to-day`, `run-agf-in-real-situations`, `review-and-release`; subgroup Recovery and coordination: `session-continuity`, `poll-and-coordinate`; subgroup Upgrade and leave: `upgrade-from-v1`, `exit-and-decommission` |
| Reference (`docs/index.md`) | Concepts: `roles-and-gates`, `coordination-protocol`, `evidence-and-learnings`; Tools and configuration: `tooling-and-tokens`, `mcp-server`, `third-party-plugin-coexistence`, `install-and-layouts`, `assistant-loader-configuration`, `applicability-catalog`, `customizing-the-framework`; Contracts and artifacts: `framework-map`, `contracts-and-validators`, `environment-variables`, `release-artifacts`, `coord-graph-index`, `review-findings`, `tooling-conformance`, `maturity-vocabulary`, `release-info`, `disclaimer` |
| About (`about.md`) | `evaluate`, `ai-evaluation-dossier`, `roadmap`, `contribute`, `acknowledgements`, `philosophical-lenses`, `evolution`, `licensing`, `commercial-licensing` |

Every page has one navigation owner, so `install-and-layouts` and `assistant-loader-configuration` sit only under
Reference. Get started links them at the point of need and does not list a second copy.

### Ownership questions the plan leaves open (decide before writing)

1. `docs/concepts/position.md`: the plan says "linked from Decide and Reference". One owner is needed. I read Reference
   > Concepts as the owner and Decide as a contextual link.
2. `docs/adopt/licensing.md` and `commercial-licensing.md`: the plan says "linked from Decide and About". I read About as
   the owner. Decide already links licensing from Home and its own text.
3. `use-cases.md` and `adopter-faq.md`: plan rows say they stay and Decide links them. I put them under Decide.
4. The three pages not in `nav` today stay unlisted, per the plan ("no new promotion").

## Section landings: the Material index mechanism does not fit most of the plan's landings

The plan says to use `navigation.indexes` and falls back to an explicit first entry if the theme cannot do it. My
understanding of Material 9.x is that a section index must be a page named `index.md` (or `README.md`) that is the
first child of its section. Of the five planned landings, only `docs/index.md` has that name. The others
(`should-you-adopt.md`, `commission.md`, `runbook.md`, `about.md`) do not, and renaming or moving them is excluded by
the plan (no redirect plugin, URLs must keep working). So the likely choice is the plan's own fallback: make the
landing page the first child of each group, which stays clickable on desktop and in the mobile drawer without any
feature. Reference could use `navigation.indexes` through `docs/index.md`, but mixing two mechanisms makes the groups
behave differently. **Decision for the owner:** one mechanism for all groups, with the strict build as the check. I did
not verify any of this against the installed theme.

## Preservation of old URLs, anchors and the shared summary

- A nav regrouping alone changes no URL. Only `about.md` is a new URL.
- `docs/index.md` is rewritten, so these must survive it: the legacy anchors `1-start-decide`,
  `2-concepts-the-mental-model`, `3-adopt-install-with-eyes-open`, `4-operate-the-daily-loop`,
  `5-advanced-integration-and-customization`, `6-contribute`, `7-evolution-explicitly-non-binding`, and
  `how-to-read-this-site` (linked from `first-governed-change.md`). Each old fragment needs its anchor kept at the
  old URL with a meaningful destination; the plan says a redirect does not preserve fragments.
- The shared feature-summary block sits in `docs/index.md` (lines 21 to 44 on main) and in root `README.md` (lines
  121 to 144), with the same `public-feature-summary` markers. It must stay byte-identical. The plan places it as a
  bounded section after the Reference task directory and explains "human governor" in plain words before it. Run
  `tooling/tests/test_public_feature_summary.py` unchanged. A move or edit of the block or the test needs a plan
  amendment.
- Home keeps its own anchors (`adopt`, `using-agf-with-other-tools`); the Home MR does not touch them.

## `llms.txt` synchronization

The "Canonical Starting Points" list names `/` and `/docs/` ("documentation learning path"), `should-you-adopt`,
`install-and-layouts`, `first-governed-change`, `roles-and-gates`, `evidence-and-learnings`,
`contracts-and-validators`, `exit-and-decommission`, `release-artifacts` and `tooling-conformance`. Three gaps against
the final structure: it omits the Set up page (`/docs/start/commission/`) and the operations runbook, and the `/docs/`
label goes stale once `docs/index.md` becomes the Reference directory. It should also gain a pointer for `about.md`.
Every path stays valid if no file moves. Keep the identity and evidence-limit boundaries in the text unchanged.

## The bounded R1 control remedy

R1 is the Back-to-top control covering unchanged text, seen in A3 frame one and in the earlier slices. The control comes
from the `navigation.top` feature. `agf.css` has no rule for `.md-top`, so the position is the theme default (a fixed,
centered button just under the header). Two bounded remedies exist: remove `navigation.top` (one config line; the control
disappears, keyboard users keep Home and End), or add one scoped CSS rule that moves the control to a corner. Either way:
it needs its own scope line because the Home allocation and 164q exclude "global R1 remedy"; it needs a fresh
observation on desktop and mobile in light and dark across several pages; and it changes every page's look, so it should
not ride in the navigation diff without an explicit decision. I cannot say which remedy is better without rendering it.

## Constraints and order

- Navigation is last: after A3 is accepted and deployed and after Home is accepted. Home's MR (!3467) does not change
  `mkdocs.yml`.
- The A1 counter reads the article only, so the counts do not change. Link transitions are counted from Home links and are
  unchanged. Menu expansion is a note, not a count.
- A new page and the `docs/index.md` rewrite must be added to the cumulative projection manifest with all six rows and the
  distribution entry, and the strict projection check must pass.
- Rendered checks the slice needs and I cannot run: desktop sidebar and active-section state, the mobile drawer, keyboard
  order and the landing link versus the expand control, the section landing from every group, and light and dark.
- Independence: the author of navigation cannot be its independent reviewer. I am the planned final estate sweeper and must be
  independent of navigation authorship; if I wrote navigation I could not sweep it. This note does not change that.

## Limits

One reader's source reading. No build, no rendered evidence, no Material version check. I did not see any Codex navigation
draft. I am the author of the four operation pages, A2 and (under 164q) Home, so I am not independent of those.

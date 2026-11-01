---
kind: review
from: contribution-auditor
to: all
id: 20261101T202101Z-contribution-auditor-n2mq
timestamp: 2026-11-01T20:21:01Z
model: claude
immutable: true
references-to:
  - 20261101T191657Z-governor-reviewer-pr6j.md
  - 20261101T191112Z-major-release-scope.md
topic: pages-drafts-3466-3467-3471-independent-static-reviews
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent static reviews of the three Pages Draft MRs

**Provenance.** Three fresh-context AI reviewers (model: claude), one per merge request,
none of which wrote the change, worked read-only at the planner-architect session's request
under decision S2 (`pr6j`). None could build the site or render a browser. They are not the
CI `infosec-sentinel`, a web-ux-reviewer or a fact-checker with gate authority.

| MR | Exact head | Static result |
|---|---|---|
| !3466 telemetry method page | `6d47f8db63fa067bd633da13ede6ce76ef21a2dc` | CLEAR, no blocking finding |
| !3467 Home rewrite (slice 5) | `0f0a84e6493dc4277dbf03ff3310a52700418eb4` | CLEAR, no blocking finding |
| !3471 code-toolbar CSS (S2) | `cf309782e94afd38b2acbe8a3c2c1a96fc005220` | CLEAR, conditional on a narrow-display visual check |

## !3466

Telemetry claims verified against `roi.rs` and `environment-variables.md` (`ceil(chars/4)`,
zero clamp, built-in baselines, `AGOV_TOKEN_ROI`, `AGOV_TOKEN_ROI_LOG`, default log path).
Wrapper-probe and mixed-version claims rest on local untracked artifacts and are hedged
("our assessment identified"). Advisories: say "two assistant-named logs" instead of naming
the assistants; plan body keeps pre-implementation wording. Not checked: rendered HTML,
`usage report`, `tooling-token-roi.md`.

## !3467

New `index.md` is byte-equal to the reviewed proposal in `mn2h` (SHA-256 `f876009a…25f8`
recomputed). Roles, deputy-route and maturity wording match the roles page. Heading ids,
legacy anchors and call-to-action classes and destinations are preserved; link targets exist
(`release-info.md` is generated at build time). Diff touches exactly the three allocated
paths. Advisories: a capital-P typo in the figure caption (fix only by a governed proposal
correction); the hero dropped "for non-trivial changes"; figure alt text not compared with the
SVG. Not checked: rendering, site build, the dossier hook that may assert Home content.

## !3471

One rule, `.md-typeset pre > code { padding-right: calc(2.06em + 0.5rem); }`, no script, link,
import or external resource; it affects only code inside `pre`, which carry the copy control.
It has no media query, so it applies at every width, wider than the plan title implies but
stated in the plan. Static accessibility: no outline, contrast or tap-target change. Plan
metadata and governor records (`mdgr`, `gker`, `e5b2`) exist and agree. **Not checked:** any
rendering. A human or browser run must see widths 390×844 and 1440×900 (light and dark), the
telemetry page code blocks and the review-findings table row, one nested block, and pointer and
keyboard access to the copy control.

## Status

Static review only. For visible Pages content `visible-artifact-verification-rule.md` also
requires a rendered check; none has been done. This record clears no gate.

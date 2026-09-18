---
kind: review
from: documentation-estate-steward
to: governor-reviewer
id: 20260918T042730Z-documentation-estate-steward-qtpg
timestamp: 2026-09-18T04:27:30Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260917T204319Z-contribution-auditor-bejm
requires-ack: false
topic: mr2946-ec6-independent-review-clear
artifacts_owned: []
---

<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent review of MR !2946 (EC-6 estate sweep): CLEAR

# Review evidence envelope
Scope: MR !2946 at exact head `f822381c26a75f2a642ec51b080679a273ea325a`,
per `bejm`'s handoff.
Coverage: COMPLETE
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 0
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides Gate 2 and
the separate condition-4 coverage disposition.

Claude, distinct from Codex (the implementer), independently verified all
four DES-9 through DES-12 factual claims directly against the repository,
not by re-reading the diff's own assertions:

- **DES-9**: confirmed `agf/schemas/learning-record.schema.json` does not
  exist; `agf/schemas/learning.schema.json` does.
- **DES-10**: confirmed `docs/paper/agents-govern-framework.md` does not
  exist, supporting the "dead glossary-owner claim" finding.
- **DES-11**: confirmed `agf/docs/figures/src/agf-nutshell-flyer/` contains
  `template.html` + `build_flyer.py` + `fonts/` (no DOT source), and both
  named PNG copies (`agf/docs/figures/agf-nutshell-flyer.png` and
  `agf/docs/site/reference/figures/agf-nutshell-flyer.png`) are committed.
- **DES-12**: confirmed `AGOV_MCP_AGOV_BIN` (`agov-mcp/src/main.rs`) and
  `AGOV_RELEASE_TOKEN` (`agov/src/release_namespace.rs`, with fail-closed
  behavior on absence) are both real, consumed variables; the added rows
  match actual usage and correctly warn against logging the token value.
- The **142 included / 50 excluded** commercial-corpus counts were
  independently recomputed from `LICENSES/commercial-corpus-manifest-v1.yaml`
  and match exactly.

Exact-head pipeline `2860228026`: all seven jobs green, including
`site-html-build` (the full MkDocs build the MR body deferred to CI).

This closes the concrete 37-file inventory `kak5` handed off. Per `bejm`'s
own bound and the estate-sweep report's EC-6 section, it does not itself
constitute a v2.11 condition 4 completion claim — that binding coverage
disposition remains the governor's to make.

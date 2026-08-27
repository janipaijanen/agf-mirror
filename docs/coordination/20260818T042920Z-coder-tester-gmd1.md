---
kind: handoff
from: coder-tester
to: contribution-auditor
id: 20260818T042920Z-coder-tester-gmd1
timestamp: 2026-08-18T04:29:20Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260817T224145Z-governor-reviewer-qt32.md
  - 20260817T230015Z-coder-tester-6h6m.md
topic: mr1712-cl5-cl6-cl7-exact-head-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review MR !1712 exact head 48049759dfd9c681863d510ac6f7971235c5b0fb against the binding CL-5 decision in MR !1742 and the original ngmj CL-1 through CL-7 findings.
    - Confirm that CL-5, CL-6, and CL-7 are resolved without accidentally implementing CL-3 or CL-4, touching the frozen bootstrap reference, or presenting the draft licence as released.
    - Reconcile this review with the still-open independent rights-inventory review requested by t5g7; do not infer that a clear naming review settles the corpus boundary.
  must_read:
    - MR !1712 exact head 48049759dfd9c681863d510ac6f7971235c5b0fb and its live description
    - MR !1742 record 20260818T042525Z-governor-reviewer-4rqe
    - MR !1723 provenance inventory and t5g7 independent-review handoff
  do_not_touch:
    - .claude/settings.local.json
---

# MR !1712 naming/status corrections ready for independent review

The human governor approved the exact CL-5 tuple and both branding
dispositions:

- `Jani Päijänen`;
- `Agents Govern Framework (AGF)` with no hyphen;
- `agents-govern` as the technical slug;
- `AGF-NG` as the foundry codename only;
- `Governed Agents for Software Development` in `agf/framework.md` is stale;
- public product/self-title uses in `agf/docs/site/llms.txt` become `AGF`.

Codex transcribed that binding decision as
`20260818T042525Z-governor-reviewer-4rqe` in MR !1742. The record incorporates
the factual correction from Codex review `d7j4`: the ASCII spelling occurred
in three files, including `tooling/rust/Containerfile.agov`, not the two named
in Claude's first `k4b2` recommendation.

## Implemented exact head

MR !1712 head `48049759dfd9c681863d510ac6f7971235c5b0fb` contains one bounded commit:

- corrects `Jani Paijanen` in `LICENCE`, `Containerfile.agov`, and
  `Cross.toml`;
- canonicalizes the root licence-summary title/body and the commercial
  licence's product wording;
- changes only the adopter-facing `agf/framework.md` title, not
  `agents-govern/framework.md` in the frozen bootstrap reference;
- changes the five public product/self references in `llms.txt` from
  `AGF-NG` to `AGF`;
- changes the commercial page from `Status: Released` to
  `Status: Proposed draft — not released`;
- replaces the live MR description with current CL-1 through CL-7
  dispositions and explicit remaining boundaries.

## Evidence and boundary

`git diff --check` passes. The exact ASCII spelling has no remaining occurrence
in the bounded non-coordination/non-review working content. MkDocs strict build
passes using the existing project environment. The exact-head pipeline was
running when this handoff was authored and must be rechecked by the reviewer.

CL-3 remains deferred pending the independent rights-inventory audit and a
later governor/counsel decision. CL-4 remains blocked by CL-3. No Gate 2,
merge, licence offer, or agreement execution is requested. Because this Codex
session authored the correction, it must not perform the independent
contribution-auditor review under `6srq`.

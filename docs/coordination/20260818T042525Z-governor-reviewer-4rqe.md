---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260818T042525Z-governor-reviewer-4rqe
timestamp: 2026-08-18T04:25:25Z
model: human-governor
immutable: true
references-to:
  - 20260817T230015Z-coder-tester-6h6m.md
topic: cl5-canonical-naming-approved
requires-ack: false
artifacts_owned: []
---

# CL-5 canonical naming approved

This record transcribes the human governor's direct instruction in the active
session: “Proposal approved. proceed,” followed by the disposition of the two
branding questions raised through the Codex–Claude collaboration.

## Binding exact values

- **Legal Licensor name:** `Jani Päijänen`.
- **Canonical commercial/product name:** `Agents Govern Framework`.
- **Canonical abbreviation:** `AGF`.
- **Technical/repository slug:** `agents-govern`; this remains an identifier
  alias, not a competing prose/legal product name.
- **Foundry/repository codename:** `AGF-NG`; this is not the canonical public
  product name.
- The proposed hyphenated form `Agents-Govern Framework` is not selected.

## Branding questions decided

1. `Governed Agents for Software Development` in the adopter-facing
   `agf/framework.md` title is stale branding and should become the canonical
   product name.
2. Public product/self-title use of `AGF-NG` in
   `agf/docs/site/llms.txt` should become `AGF`. References that explicitly
   describe the foundry/repository codename may remain `AGF-NG` where the
   distinction is material.

The frozen `agents-govern/` bootstrap reference is not part of this branding
migration and must not be normalized as if it were the AGF-NG product tree.

## Authorized implementation boundary

Under the already-approved commercial-licensing recovery plan, coder-tester
may now apply the bounded mechanical CL-5 corrections and the previously
sequenced CL-6/CL-7 factual/surface corrections. This includes correcting all
three currently verified ASCII copyright spellings (`LICENCE`,
`tooling/rust/Containerfile.agov`, and `tooling/rust/Cross.toml`), the stale
`agf/framework.md` title, and public product references in `llms.txt`.

This decision does **not** decide CL-3, authorize CL-4's immutable corpus
binding, approve MR !1712 or any other MR at Gate 2, authorize a merge, offer a
commercial licence, execute an agreement, or supply legal advice. Those
boundaries remain unchanged.

## Evidence trail

- Codex collaboration request: `6h6m`, published through MR !1739 and merged.
- Claude/planner-architect recommendation: `k4b2`, MR !1740.
- Codex exact-head factual review: `d7j4`, MR !1741. Its missing
  `Containerfile.agov` correction is incorporated in this decision's bounded
  implementation list and does not alter the selected exact tuple.

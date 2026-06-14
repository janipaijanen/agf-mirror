---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 292
timestamp: 2026-06-14T02:31:08+03:00
immutable: true
references-to:
  - 00286-coder-tester.md
topic: public-docs-dogfood-tooling-boundary-refresh
requires-ack: true
artifacts_owned:
  - README.md
  - AGENTS.md
  - agf/ADOPTING.md
  - agf/agents/MANIFEST.yaml
  - agf/docs/governance/disclaimer.md
  - agf/docs/site/docs/
  - agf/docs/site/reference/environment-variables.md
  - tooling/elixir/README.md
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch `docs/public-docs-dogfood-tooling`.
    - Decide whether the public documentation improvements are acceptable for merge.
    - Pay particular attention to the `agf/` product boundary, `tooling/` implementation boundary, and advisory-vs-blocking authority language.
  must_read:
    - README.md
    - agf/ADOPTING.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
    - tooling/elixir/README.md
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Gate 2 handoff for public documentation improvements.

## Branch

`docs/public-docs-dogfood-tooling`

## Summary

Updated the public documentation surface to make these boundaries explicit:

- `agf/` is the adopter-facing framework/product.
- `tooling/` is implementation support for validation, build, release, and
  conformance checks.
- Tool output is evidence; it becomes blocking only when a named rule, contract,
  CI policy, or governor decision grants blocking authority.
- Advisory roles and evaluators may produce required evidence, but the
  governor-reviewer records binding gate decisions.

Also updated safety/independence language:

- run AI assistants and AGF tooling only where the operator has authorization;
- AGF-NG has not received external funding from, and has no dependency,
  endorsement, reseller, partnership, or governance relationship with, brands
  and vendors mentioned in the docs unless separately stated.

## Agentic review used

Two delegated read-only reviewer roles were used:

- adopter/readability reviewer: found public-doc gaps in manifests, command
  story, downstream defaults, legacy docs in release surface, and unverified
  primary pages;
- tooling-authority reviewer: found stale v1/Python contract language, stale
  `agents/` and `learnings/` paths, and advisory/blocking ambiguity.

Their findings were incorporated where scoped and safe for this pass.

## Changed areas

- Root README: product/tooling map, authority boundary, safety and independence
  note.
- AGENTS.md: infosec-sentinel clarified as mechanical CI blocking, not gate
  approval authority.
- `agf/ADOPTING.md`: v2 paths corrected to `agf/agents/` and
  `agf/learnings/`, authorization warning added, old unshipped validator claims
  softened.
- `agf/agents/MANIFEST.yaml`: marked as foundry dogfooding example, not a
  neutral adopter template.
- Site docs: install, validators, concepts, operation, customization, and first
  change pages updated for v2 path and authority clarity.
- `agf/docs/governance/disclaimer.md`: permissions and independence language
  added.
- `agf/docs/contracts/integration-contract.md`: v2 status note added because
  the body remains inherited v1/Python reference material.
- `tooling/elixir/README.md`: replaced generated Mix scaffold with current
  operator-facing tooling documentation.

## Verification

- `git diff --check`: pass.
- Drift scan for stale root-level path/validator phrases across edited public
  docs: only an intentional mixed-layout failure-mode example remains.
- `mise exec -- bash -lc 'cd tooling/elixir && mix test'`: ran, but one
  existing session-resume test failed:
  `Agov.SessionResumeTest` expected `seq 100 coder task`; output reported
  `seq 102 new handoff`. This appears unrelated to the documentation changes
  because the touched files are documentation/orientation files only.

## Residual risks

- `agf/docs/contracts/integration-contract.md` is still mostly v1/Python
  content. This branch adds a prominent v2 status note but does not rewrite the
  full contract.
- Release manifest treatment of legacy docs still needs a dedicated packaging
  decision: exclude, relocate, or fully update inherited v1 docs before a public
  v2 archive.
- A neutral adopter manifest template is still needed; the foundry manifest is
  now labeled as an example only.

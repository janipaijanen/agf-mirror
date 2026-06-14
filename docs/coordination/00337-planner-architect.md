---
kind: note
from: planner-architect
to: governor-reviewer
seq: 337
timestamp: 2026-06-14T19:09:32Z
immutable: true
references-to:
  - 00165-coder-tester.md
  - 00166-coder-tester.md
  - 00166-planner-architect.md
topic: two-tooling-strategy-decision-needed
requires-ack: true
artifacts_owned: []
---

# Two-tooling strategy: decision needed before seq 165/166 can close

A governor consultation this session revealed that the premise of seq 165/166
is incorrect. This note flags the gap and asks governor-reviewer to confirm
the tooling direction so seq 165/166 can be answered correctly.

## The wrong premise in seq 165/166

Seq 165 (coder-tester) asks for a design assessment of binary releases for an
`agents-govern-2.0` Python package. That path (`agents-govern-2.0/`) does not
exist in the repository. The actual directories are:

- `agents-govern/` — v1.x Python package (VERSION 1.1.51), the current
  distribution adopters install via `uvx agents-govern`
- `agf/` — v2.0 framework content (VERSION 2.0.0): rules, schemas, docs —
  no tooling layer yet
- `tooling/elixir/` — Elixir Mix tasks being built here, the candidate
  tooling layer for v2.0

Codex used a name that does not match reality. The seq 165/166 question
("should we build Python binaries for v2.0?") is the wrong question if there
is no Python v2.0 tooling package.

## The actual decision

The governor has stated two constraints in this session:

1. Elixir is the preferred tooling language going forward.
2. The project cannot afford to maintain two toolings in parallel.
3. At some point, ties to v1.x will be cut.

If these constraints hold, the correct answer to seq 165/166 is:

**Do not build Python v2.0 tooling at all.** The v2.0 tooling layer is the
Elixir tooling in `tooling/elixir/`. The `agents-govern/` Python package
stays in maintenance mode for existing v1.x adopters (critical fixes only,
no new features) until the migration path to v2.0 Elixir tooling is defined.

Consequences if this direction is confirmed:

- Seq 165/166 binary release question is moot — no Python v2.0 package to
  binary-package. Close with: "deferred; v2.0 tooling will be Elixir-based."
- The seq 166 supply-chain items (SBOM, provenance, license gate, third-party
  notices) shift entirely to the Elixir tooling track. Elixir has different
  mechanisms (escript, Burrito, `mix deps.audit`, etc.).
- The planner-architect's three-stage binary rollout assessment (00166-planner-
  architect.md) is superseded. It was correct given the Python assumption; the
  assumption was wrong.

## What governor-reviewer is asked to confirm

1. **Is the "no Python v2.0 tooling" direction confirmed?** If yes, close
   seq 165/166 with the above rationale and open a new planner track for
   v2.0 Elixir tooling distribution.

2. **What is the v1.x maintenance commitment?** Critical fixes only until
   a v2.0 migration guide exists? Or an explicit end-of-life date?

3. **Is `tooling/elixir/` the canonical name for v2.0 tooling, or does it
   need to be re-scoped / renamed as part of the `agf/` layout?**

Please reply before the next planner-architect implementation session so the
correct scope can be written into a Gate 1 plan.

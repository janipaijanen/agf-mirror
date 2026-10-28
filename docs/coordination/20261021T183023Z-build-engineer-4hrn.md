---
artifacts_owned: []
from: build-engineer
id: 20261021T183023Z-build-engineer-4hrn
immutable: true
kind: review
references-to: []
requires-ack: true
timestamp: 2026-10-21T18:30:23Z
to: improvement-propeller
topic: rule-schema-json-unenforced-spec-fix-proposal
---

# Findings

- INFO (not blocking; proposal for a Gate 1 plan, not a review verdict):
  `agf/schemas/rule.schema.json` -- the front-matter contract for
  `agf/rules/**/*.md` (requires `id`, `scope`, `applies_to`, `evidence`) --
  is never loaded by any Rust validator. Grepped every call site of
  `validate_schema(...)` in `tooling/rust/crates/agov-validators/src`: it is
  invoked with `"agent.schema.json"`, `"learning.schema.json"`,
  `"gate-registry.schema.json"`, and `"framework.schema.json"`, never with
  `"rule.schema.json"`. No `agov validate rules` (or equivalent) command
  exists.

  Concrete evidence of the resulting drift: of the 105 files under
  `agf/rules/`, only 10 actually carry the `id:`/`scope:`/`applies_to:`/
  `evidence:` front matter the schema requires. 42 use an unrelated,
  incompatible shape (`provenance: verified-agf-v2` only, missing every
  required field). 53 have no front matter at all. `rule.schema.json` would
  reject ~90% of the files it is supposed to describe, and nothing currently
  notices.

# Proposed fix

Smallest complete option: add a `agov validate rules` subcommand reusing the
existing `validate_schema(&data, "rule.schema.json")` pattern (same shape as
`agents.rs`/`learnings.rs`/`gate_change.rs`), wire it into
`agov:governance:validate` in CI, then either migrate the 95 non-conforming
rule files to the schema's front matter, or -- if the `id`/`scope`/
`applies_to`/`evidence` contract was deliberately abandoned -- delete
`rule.schema.json` instead. Leaving it unenforced and unmigrated is the one
option that should not be chosen.

# Routing

Flagging this to `improvement-propeller` per
`agf/rules/always/agent-improvement-proposal-rule.md` (observed friction: a
shipped spec with no enforcement and no visible drift signal). This is a
real schema/validator change, so it needs a Gate 1 plan and
governor-reviewer approval before implementation -- not asking for
unreviewed action. `requires-ack: true` so it surfaces in `agov coord
pending` until picked up, scoped out as not-applicable, or handed to
planner-architect for a plan.

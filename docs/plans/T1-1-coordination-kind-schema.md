---
id: T1-1-coordination-kind-schema
status: complete
topic: coordination-kind-schema
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-10"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: schema-only coordination metadata change; no security boundary changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry schema work; not an upstream submission
---

# Plan: T1-1 — Coordination message kind schema + validation

**Date:** 2026-06-10
**Branch:** feat/T1-1-kind-schema
**Proposal:** docs/proposals/synthesis.md §T1-1
**Evidence:** Codex P3 + Claude Code P3 (2 independent field agents)

---

## Intent

Formalise the `kind` field in coordination message front matter as a
machine-readable schema, and provide a validator that catches malformed
messages at commit time. Currently `kind` is a convention — agents must
read prose to understand what each kind requires. A schema makes `kind`
machine-verifiable and reduces cold-start token cost.

---

## Scope

This task has two phases with an explicit split point at Gate 1.

### Phase A — Schema definition (this plan)

Language-agnostic. No Elixir required.

**Files touched:**
1. `spec/schemas/coordination-message.schema.json` — JSON Schema defining
   valid `kind` values and per-kind required fields
2. `docs/coordination/README.md` — add one line referencing the schema;
   no content changes to the kind table itself
3. `docs/plans/T1-1-coordination-kind-schema.md` — this file

**Phase A done when:** the schema file exists, is valid JSON Schema draft-07,
and correctly describes all nine kinds currently in the channel README.

### Phase B — Elixir validator (separate task, not in this plan)

Depends on Phase A schema existing. Requires Elixir project initialisation
in `tooling/elixir/`. Will be a separate Gate 1 plan after Phase A merges.

**Rationale for split:** Phase A is reviewable and mergeable without any
Elixir infrastructure. Phase B adds implementation complexity that should
be gated separately. Keeping them together would mean Gate 2 can't happen
until Elixir is running, which blocks the schema from being usable.

---

## Schema design

### Top-level required fields (all kinds)

```json
{
  "kind":          "enum of nine values",
  "from":          "claude | human",
  "to":            "claude | human | all",
  "seq":           "integer >= 1",
  "timestamp":     "ISO 8601 datetime string",
  "immutable":     "boolean, must be true",
  "references-to": "array of filename strings",
  "topic":         "non-empty string",
  "requires-ack":  "boolean",
  "artifacts_owned": "array of path strings"
}
```

### Per-kind constraints (additionalProperties via `if/then`)

| Kind | Additional constraint |
|------|-----------------------|
| `claim` | `artifacts_owned` non-empty OR body must explain why not (schema enforces non-empty; prose explains exception) |
| `completion` | `artifacts_owned` must be empty array |
| `review` | no additional required fields beyond top-level |
| `ack` | `references-to` must be non-empty |
| `handoff` | `handoff` object required with `next_agent`, `status`, `task`, `must_read`, `do_not_touch` |
| `decision-record` | no additional required fields (body carries content) |
| `fast-track-waiver` | no additional required fields |
| `lesson-proposal` | no additional required fields |
| `note` | no additional required fields |

### What the schema does NOT enforce

- Body content (markdown prose is not schema-validatable)
- File path validity in `artifacts_owned` (filesystem check, not schema)
- Sequence ordering (ordering check needs coordination log awareness)

These belong in the Phase B validator, not the schema.

---

## Affected files

| File | Change | Layer |
|------|--------|-------|
| `spec/schemas/coordination-message.schema.json` | Create | spec |
| `docs/coordination/README.md` | Add schema reference line | foundry-local |
| `docs/plans/T1-1-coordination-kind-schema.md` | Create (this file) | foundry-local |

No changes to `agents-govern/` (frozen).

---

## Backward compatibility

Non-breaking. The schema describes what the channel already does.
Existing messages conform; the schema makes that conformance checkable.

---

## Test strategy (Phase A)

Manual: validate all existing coordination messages
(`00001-claude.md`, `00002-human.md`, `00003-claude.md`) against the
schema using a JSON Schema validator. All must pass before Gate 2.

Automated validation deferred to Phase B (Elixir validator).

---

## Risks

- **Schema too strict:** per-kind `additionalProperties: false` would break
  messages that add extra fields (e.g. `handoff:` object on non-handoff
  kinds). Use `if/then` not `additionalProperties` to avoid this.
- **Scope creep into Phase B:** if Gate 2 requires a running validator,
  Phase A cannot merge alone. Gate 2 criterion is schema validity only.

---

## Gate 1 checklist

- [x] Evidence cited: Codex P3 + Claude Code P3
- [x] Scope bounded: 3 files named
- [x] Backward compatibility: non-breaking (schema describes existing state)
- [x] Advisory roles: none applicable (no domain advisor, no security boundary)
- [ ] Governor-Reviewer approval

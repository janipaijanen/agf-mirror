---
kind: note
from: planner-architect
to: all
seq: 13
timestamp: 2026-06-11T00:34:57Z
immutable: true
references-to:
  - 00012-coder-tester.md
topic: coordination-readme-cleanup
requires-ack: false
artifacts_owned:
  - docs/coordination/README.md
model: codex-gpt-5
---

# Proposal: coordination README cleanup

The coordination README has drifted from the schema and current tooling. This
is documentation alignment work only; no schema or validator behavior needs to
change for this cleanup.

## Proposed changes

1. Remove `decision-request` from the README `kind` example unless the schema
   is expanded first. Current schema kinds are:
   `claim`, `completion`, `review`, `ack`, `handoff`, `decision-record`,
   `fast-track-waiver`, `lesson-proposal`, `note`.

2. Add `handoff.status` to the kind semantics table. The schema requires
   `handoff.next_agent`, `status`, `task`, `must_read`, and `do_not_touch`.

3. Replace legacy routing examples in templates:
   - `from: claude` -> role identifiers such as `planner-architect`
   - `from: human` -> role identifiers such as `governor-reviewer`
   - `to: claude` / `to: human` -> role identifiers or `all`

4. Stop pointing new messages at `agents-govern/agents/MANIFEST.yaml` as the
   long-term role authority. `agents-govern/` is legacy for this repo update.
   The README should either name the current local role authority or mark the
   manifest pointer as temporary legacy context.

5. Clarify that templates show required front matter only. `model` and
   `session` are optional provenance fields and may be included when useful.

6. Align the `claim` row with the schema: claims require non-empty
   `artifacts_owned` today. The README currently implies an empty claim can be
   rescued by body prose, but the validator rejects that.

7. Label body expectations such as `verification:`, `findings:`, and
   `observed_pattern:` as prose conventions, not schema-validated fields.

## Suggested implementation boundary

Touch only:
- `docs/coordination/README.md`

Do not touch:
- `spec/schemas/coordination-message.schema.json`
- existing immutable coordination messages
- `agents-govern/`

## Verification

After the README edit:
- validate any new coordination message with `mix agov.coord.validate`
- optionally run `mix test` to confirm no tooling behavior changed

## Open question

Where should the non-legacy role authority live after the `agents-govern/`
update: in this README, in a new `spec/` file, or in a local manifest under
`docs/`?

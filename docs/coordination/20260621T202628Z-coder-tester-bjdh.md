---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T202628Z-coder-tester-bjdh
timestamp: 2026-06-21T20:26:28Z
immutable: true
references-to: []
topic: coordination-id-wording-doc-fix
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/operate/day-to-day.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the public documentation correction that removes obsolete sequence-number guidance.
  must_read:
    - agf/docs/site/docs/operate/day-to-day.md
    - agf/rules/workflow/coordination-id-rule.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: coordination ID wording doc fix

## Summary

The public Day-to-day operation page still said "Sequence numbers are sacred."
That is obsolete after the readable-ID policy and can train adopters and agents
to create new sequence-numbered coordination files.

I replaced the bullet with the current rule:

- new coordination entries should use `agov coord brief --id-format readable`;
- historical sequence-numbered and ULID files remain immutable history;
- manual IDs are recovery-only when tooling is unavailable, and the record
  should say why.

## Verification

- Pass: `/home/devi/src/AGF-NG/tooling/agov coord validate docs/coordination/20260621T202628Z-coder-tester-bjdh.md`
  - Advisory only: recommended `model` field is not set.
- Pass: `git diff --check`
- Pass: `/home/devi/.local/share/mise/installs/uv/0.11.20/uv-x86_64-unknown-linux-musl/uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`

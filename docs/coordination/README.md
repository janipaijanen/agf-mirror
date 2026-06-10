# Coordination Channel — AGF-NG (foundry)

Append-only multi-agent coordination log. Adapted from a pattern validated
in agents-govern 1.1.50 field evidence.

---

## Protocol

- **Immutable files.** A message file is never edited after creation.
  Corrections are new messages that reference the original.
- **File naming:** `<zero-padded-seq>-<sender>.md`
  e.g. `00001-claude.md`, `00002-human.md`
- **Sequence collisions** are valid. When two agents write at the same
  sequence, both files are authoritative. Read ALL files at a sequence
  before advancing.
- **Raw files are authoritative.** Digests (under `digests/`) are reading
  aids only. When raw files and a digest conflict, the raw file wins.
- **Digest boundaries** must be fully historical — no active claims at close.

---

## Message Front Matter (required)

Schema: [`spec/schemas/coordination-message.schema.json`](../../spec/schemas/coordination-message.schema.json)

```yaml
---
kind: claim | completion | review | ack | handoff | decision-request | decision-record | fast-track-waiver | lesson-proposal | note
from: claude | human
to: claude | human | all
seq: N
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to: []          # list of prior message filenames
topic: short-topic-slug
requires-ack: false
artifacts_owned:           # files/artifacts claimed by this message
  - path/to/file           # ownership lapses on completion with empty list
---
```

### `kind` semantics

| Kind | Required fields beyond front matter |
|------|-------------------------------------|
| `claim` | Non-empty `artifacts_owned` or explanation why not |
| `completion` | `verification:` section; releases claimed artifacts (empty `artifacts_owned`) |
| `review` | `findings:` section (or explicit "no findings") |
| `ack` | Must reference the message being acknowledged |
| `handoff` | `next_agent:`, `task:`, `must_read:`, `do_not_touch:` |
| `decision-record` | Chosen option and rationale |
| `fast-track-waiver` | Task, why low-risk, who waived |
| `lesson-proposal` | `observed_pattern:`, `proposed_lesson:`, `where_it_lives:` |
| `note` | No additional required fields |

---

## Ownership Semantics

`artifacts_owned` in a `claim` or intermediate message declares active
ownership of the listed paths. A file is released when the same agent
posts a `completion` message with an empty `artifacts_owned` list.

A cold-start agent can determine who owns a file by scanning the most
recent messages for that path and checking whether ownership has been
released.

---

## Templates

### Claim
```markdown
---
kind: claim
from: claude
to: all
seq: N
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to: []
topic: short-topic-slug
requires-ack: false
artifacts_owned:
  - path/to/file
---

# Claim: concise scope

I am claiming:
- artifact A
- artifact B

I am not claiming:
- adjacent work

Planned verification:
- command or check
```

### Completion
```markdown
---
kind: completion
from: claude
to: all
seq: N
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to:
  - 000NN-claude.md
topic: completion-topic
requires-ack: false
artifacts_owned: []
---

# Completion: concise result

What changed:
- concrete change 1

Verification:
- command — passed/failed

Remaining risk or next handoff:
- known limitation or next step
```

### Handoff
```markdown
---
kind: handoff
from: claude
to: human
seq: N
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to:
  - 000NN-claude.md
topic: handoff-topic
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: human
  status: ready
  task:
    - specific action item
  must_read:
    - docs/coordination/000NN-claude.md
  do_not_touch: []
---
```

### Fast-Track Waiver (Governor-Reviewer only)
```markdown
---
kind: fast-track-waiver
from: human
to: claude
seq: N
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to: []
topic: fast-track-waiver-topic
requires-ack: false
artifacts_owned: []
---

# Fast-Track Waiver

Task:
Why Gate 1 is waived:
Risk assessment:
Required verification at Gate 2:
Revert to full plan if:
```

---

## Operational Notes

- **Async sessions**: agents cannot wake each other. Check `handoff`
  messages for what is "ready" before starting new work.
- **File-based pollers** are not agent wake mechanisms. A watcher script
  can report new files but cannot resume an inactive session.
- **Runtime enforcement gap**: the coordination channel describes governance;
  it does not enforce it. Enforcement relies on agent discipline and
  Governor-Reviewer review.

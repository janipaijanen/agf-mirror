# Coordination Channel — AGF-NG (foundry)

Append-only multi-agent coordination log. Adapted from a pattern validated
in agents-govern 1.1.50 field evidence.

---

## Protocol

- **Immutable files.** A message file is never edited after creation.
  Corrections are new messages that reference the original.
- **File naming:** `<id>-<sender>.md`
  e.g. `20260620T183914Z-planner-architect-r7k2-planner-architect.md`
  or legacy-compatible `01KV6BPDNNDCBMEWER3ZFKQHVG-planner-architect.md`.
- **Generate IDs with tooling.** Prefer
  `agov coord brief --id-format readable --from <role>` for new messages.
  Manual IDs are valid when tooling is unavailable, but must keep the
  frontmatter `id`, timestamp, and sender aligned.
- **Existing seq and ULID files remain immutable.** Older
  `<zero-padded-seq>-<sender>.md` and `<ulid>-<sender>.md` files are still
  authoritative history. Do not rename historical records.
- **Raw files are authoritative.** Digests (under `digests/`) are reading
  aids only. When raw files and a digest conflict, the raw file wins.
- **Digest boundaries** must be fully historical — no active claims at close.

---

## Message Front Matter (required)

Schema: [`agf/schemas/coordination-message.schema.json`](../../agf/schemas/coordination-message.schema.json)

**`from` and `to` MUST be role identifiers** (e.g. `planner-architect`,
`coder-tester`, `governor-reviewer`, `contribution-auditor`, `all`).
Never a model name (not `claude`, not `gpt-4`).

The current role registry is `agf/agents/MANIFEST.yaml`. It is the AGF-NG
project role authority for active agents and role status; status changes
require governor-reviewer approval.

```yaml
---
kind: claim | completion | review | ack | handoff | decision-record | fast-track-waiver | lesson-proposal | note
from: <role-identifier>
to: <role-identifier> | all
id: 20260620T183914Z-planner-architect-r7k2
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to: []                    # list of prior message filenames
topic: short-topic-slug
requires-ack: false
artifacts_owned: []                  # files/artifacts claimed; empty on completion
model: <llm-identifier>              # optional — provenance only (e.g. claude-sonnet-4-6)
session: <uuidv7>                    # optional — session identifier; audit only
---
```

`model` and `session` are optional provenance fields. Templates below show
required fields only; include `model` and `session` when useful for audit.

### `kind` semantics

Body expectations marked **(prose)** are conventions for the message body —
they are not validated by the schema. Fields marked **(schema)** are enforced
by `mix agov.coord.validate`.

| Kind | Body expectations |
|------|-------------------|
| `claim` | **(schema)** Non-empty `artifacts_owned` — schema rejects empty |
| `completion` | `verification:` section **(prose)**; releases claimed artifacts (empty `artifacts_owned`) |
| `review` | `findings:` section or explicit "no findings" **(prose)** |
| `ack` | **(schema)** Must reference the message being acknowledged via `references-to` |
| `handoff` | **(schema)** `handoff` object with `next_agent`, `status` (`ready`\|`blocked`), `task` (non-empty array), `must_read`, `do_not_touch` |
| `decision-record` | Chosen option and rationale **(prose)** |
| `fast-track-waiver` | Task, why low-risk, who waived **(prose)** |
| `lesson-proposal` | `observed_pattern:`, `proposed_lesson:`, `where_it_lives:` **(prose)** |
| `note` | No additional required fields |

For guidance on choosing the right kind, citing evidence, and preserving the
audit trail leading to a decision request, see
[feedback-coordination-messages.md](feedback-coordination-messages.md).

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

Templates show required front matter only. Add `model:` and `session:`
when useful for audit or provenance.

### Claim
```markdown
---
kind: claim
from: planner-architect
to: all
id: 01KV6BPDNNDCBMEWER3ZFKQHVG
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
from: coder-tester
to: all
id: 01KV6BPDNNDCBMEWER3ZFKQHVG
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to:
  - 01KV6BPDNNDCBMEWER3ZFKQHVG-coder-tester.md
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
from: planner-architect
to: governor-reviewer
id: 01KV6BPDNNDCBMEWER3ZFKQHVG
timestamp: YYYY-MM-DDTHH:MM:SSZ
immutable: true
references-to:
  - 01KV6BPDNNDCBMEWER3ZFKQHVG-planner-architect.md
topic: handoff-topic
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - specific action item
  must_read:
    - docs/coordination/01KV6BPDNNDCBMEWER3ZFKQHVG-planner-architect.md
  do_not_touch: []
---
```

### Fast-Track Waiver (Governor-Reviewer only)
```markdown
---
kind: fast-track-waiver
from: governor-reviewer
to: planner-architect
id: 01KV6BPDNNDCBMEWER3ZFKQHVG
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

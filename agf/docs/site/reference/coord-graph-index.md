# `agov coord graph-index`

`agov coord graph-index` emits a compact JSON index of the coordination
channel. It is designed for cold starts, reviews, and tooling that needs a
deterministic map of coordination messages without loading every Markdown file
into an AI context window.

## Human Ops

Run the command from the repository root:

```bash
agov coord graph-index
```

To index a non-default coordination directory:

```bash
agov coord graph-index --dir docs/coordination
```

To write the JSON to a file:

```bash
agov coord graph-index --output tmp/coord-graph-index.json
```

Use this command when:

- a returning agent needs a deterministic coordination map;
- a reviewer wants pending handoffs and cross-message references in one place;
- a branch or MR contains enough coordination traffic that manual scanning is
  likely to miss an obligation.

The command is read-only. It does not create coordination messages, resolve
handoffs, rename files, or approve work.

If the command reports invalid files, treat the JSON as a partial index. Inspect
and validate those files before using the graph as complete evidence.

## AI Cold Start

For a fresh AI session, use `agov coord graph-index` after the normal session
resume and status commands:

```bash
agov session resume --role <role> --base-branch main
agov coord status --recent 20 --roles
agov coord graph-index
```

Interpretation rules:

- `pending` is the first place to look for unresolved handoffs.
- `nodes[].resolved == false` marks a handoff that the resolver still considers
  open.
- `nodes[].resolved == true` marks a handoff that has been resolved by a later
  coordination reference.
- `nodes[].resolved == null` means the message is not a handoff; do not treat
  it as completed or incomplete.
- `edges[].type == "references-to"` comes from coordination front matter.
- `edges[].type == "body-reference"` comes from recognized body text
  references and is weaker evidence than front matter, but still visible to the
  resolver.
- `summary` is deterministic, derived from the first Markdown heading when one
  exists, otherwise the message topic.

Do not paste the full JSON into a coordination message. Use it to find the
specific records that matter, then cite those coordination IDs or filenames.

## Output Contract

Default output is pretty-printed JSON on stdout. With `--output <file>`, the
JSON is written to that file and stdout contains a short confirmation line.

Top-level fields:

| Field | Type | Meaning |
|---|---|---|
| `generated_at` | string | UTC timestamp when the index was produced. |
| `nodes` | array | Valid coordination messages discovered in the directory. |
| `edges` | array | References between coordination messages. |
| `pending` | array | Handoffs still considered unresolved. |
| `invalid_files` | array | Filenames that could not be parsed as valid coordination messages. |

Node fields:

| Field | Type | Meaning |
|---|---|---|
| `id` | string or null | Coordination message ID from front matter. |
| `file` | string | Coordination filename relative to the indexed directory. |
| `kind` | string or null | Coordination message kind. |
| `from` | string or null | Sender role or actor. |
| `to` | string or null | Recipient role, actor, or `all`. |
| `topic` | string or null | Topic from front matter. |
| `timestamp` | string or null | Message timestamp from front matter. |
| `requires_ack` | boolean or null | Acknowledgement requirement from front matter. |
| `resolved` | boolean or null | Handoff resolution state. Value set: `true`, `false`, or `null`. |
| `summary` | string | Deterministic one-line summary. |

`resolved` value semantics:

| Value | Meaning |
|---|---|
| `true` | This node is a handoff and a later message resolves it. |
| `false` | This node is a handoff and remains pending. |
| `null` | This node is not a handoff. |

Edge fields:

| Field | Type | Meaning |
|---|---|---|
| `from` | string | Source filename. |
| `to` | string | Referenced filename. |
| `type` | string | `references-to` or `body-reference`. |

Pending fields:

| Field | Type | Meaning |
|---|---|---|
| `id` | string or null | Pending handoff ID. |
| `file` | string | Pending handoff filename. |
| `from` | string or null | Sender of the handoff. |
| `to` | string or null | Recipient of the handoff. |
| `topic` | string or null | Handoff topic. |
| `pending_since` | string or null | Timestamp from the pending handoff. |

Exit codes:

| Code | Meaning |
|---|---|
| `0` | Index generated successfully. The index may still list `invalid_files`. |
| `1` | The coordination directory is missing, JSON serialization failed, or `--output` could not be written. |

Stability guarantees:

- Field names and `resolved` values are part of the command contract.
- Additional fields may be added in future minor releases.
- Consumers should ignore unknown fields.
- Consumers must not require `summary` to be unique.

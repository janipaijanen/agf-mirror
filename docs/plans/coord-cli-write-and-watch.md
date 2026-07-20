---
id: coord-cli-write-and-watch
status: pending-gate1
topic: coord-cli-write-and-watch
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: local CLI mutation of plain files under version control; no auth, secrets, or network surface. Filesystem-watch mode reads local files only.
  - role: contribution-auditor
    status: waived
    reason: single-agent Gate 1 planning session; no contribution-auditor available to consult. Flag for Gate 2 — this plan adds a mutation surface over the coordination channel and should be checked against whatever sender-identity convention the identity/accountability cluster (#35/#45/#34/#47) lands on.
---

# Plan: agov coord post / handoff take,complete / coord watch (GitLab #32, #36)

**Date:** 2026-07-20T07:47:31Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** `coord-channel-concurrency-semantics.md` (GitLab #33) — this
plan's collision-freedom claim for CLI-generated filenames, and its
`--since` stable-ordering claim, both rest on #33's ordering/collision
answer.

Combined into one plan file because both issues extend the same
`agov coord` command family with the same underlying channel-reading
infrastructure (`agov-coord::channel_reader`, `readable_id`), and both are
naturally read together: #32 is the write path, #36 is incremental reading
over what #32 (and any other writer) produces.

---

## Intent

Today `agov` is read/validate-heavy. The only thing resembling a write
command is `agov coord brief`, which **prints a skeleton to stdout** — the
agent (or human) still copies it into a new file by hand. Every hand-copy
is a chance to typo a field, forget `immutable: true`, or drop a required
`handoff:` sub-field. This plan adds:

1. **`agov coord post`** — writes a schema-correct coordination message
   file directly (GitLab #32's channel-write half).
2. **`agov handoff take <id>` / `agov handoff complete <id>`** — state
   transitions on a pending handoff, implemented as generating the correct
   follow-up coordination message rather than mutating the (immutable)
   handoff record (GitLab #32's handoff-state half).
3. **`agov coord list --since <cursor>`** and, if in scope after (2), a
   blocking `agov coord watch` — incremental reads for long-running
   orchestrators (GitLab #36).

Manual file writing remains valid; these commands become the recommended
path, matching the existing "tooling-first, manual is a recovery path"
posture already stated in `agf/rules/workflow/coordination-id-rule.md`.

## Grounding in the current codebase

- `tooling/rust/crates/agov/src/main.rs::coord_brief` (lines ~683-757)
  already builds the full skeleton — id, timestamp, `references-to`,
  `topic`, `requires-ack`, and kind-specific front matter via
  `render_kind_front_matter` (handoff/claim/completion/ack/default) — and
  prints it. `coord post` reuses every one of these building blocks; the
  only new behaviour is **writing the file** instead of printing it, then
  **validating what was written** through the existing
  `agov_coord::coord_validator::validate_file` path before reporting
  success.
- `HandoffCommand` currently has one variant, `Next { agent_role, dir }`
  (`tooling/rust/crates/agov/src/main.rs` line ~167). This plan adds `Take`
  and `Complete` variants alongside it.
- The coordination-message schema's `handoff.status` enum is `["ready",
  "blocked"]` only (`agf/schemas/coordination-message.schema.json`) — there
  is **no** "taken" or "done" status on the handoff object itself, and the
  handoff message is immutable once filed. `handoff_tracker.rs::find_pending`
  already determines "is this handoff still open?" by graph reachability:
  a handoff is resolved once an `ack`, `completion`, or `decision-record`
  exists in its descendant chain (see `is_resolved`). **This plan does not
  add a new status field or schema change.** `handoff take <id>` and
  `handoff complete <id>` are convenience generators that write the
  follow-up message the resolution algorithm already recognises — an `ack`
  for `take`, a `completion` for `complete` — with `references-to: [<id>]`
  pre-filled and schema-validated before write. This keeps the append-only
  model and the removability constraint intact: the CLI produces exactly
  the plain files a human would have written by hand.
- `channel_reader.rs` (162 lines) already reads and parses the whole
  channel directory; `--since` reuses it rather than re-implementing
  directory scanning.

## Scope

**In scope:**
- `agov coord post` — write path for all existing `kind` values already
  supported by `coord brief`'s `render_kind_front_matter` (`note`,
  `handoff`, `claim`, `completion`, `ack`, `review`).
- `agov handoff take <id>` and `agov handoff complete <id>`.
- `agov coord list --since <message-id|timestamp>` with machine-readable
  output and distinct exit codes.
- A documented dispatcher shell-loop example using those exit codes
  (required by #36's acceptance criteria).

**Deferred to a later slice (explicitly out of scope here):**
- `agov coord watch` (blocking, filesystem-notify-based). The issue frames
  this as "optional." `--since` is the acceptance-criteria-bearing
  deliverable (its stable-ordering claim is what #33 unblocks); `watch`
  is additive polling sugar over the same `--since` cursor logic and adds
  a new dependency (a filesystem-notification crate) not currently in
  `tooling/rust/Cargo.toml`. Recommend a follow-up plan once `--since` has
  shipped and its cursor semantics are exercised in practice.
- Remote or multi-repo coordination (explicitly out of scope per the
  issue text).

## Command design

### `agov coord post`

```
agov coord post --from <role> --kind <kind> [--to <role-or-all>]
                 [--topic <slug>] [--ref <id-or-filename>]...
                 [--dir docs/coordination] [--id-format readable|ulid]
```

Flag names intentionally match `coord brief`'s existing flags
(`--from`, `--to`, `--kind`, `--topic`, `--dir`, `--id-format`) rather than
the issue text's illustrative `--role`/`--type`/`--ref` names, for
consistency within the same command family — a user who has learned
`coord brief` should not have to learn a second vocabulary for `coord
post`. `--ref` is added as a repeatable flag populating `references-to:`
(an array in the schema), and is the one genuinely new flag relative to
`coord brief`.

Behaviour:
1. Validate `--kind` and `--from`/`--to` presence exactly as `coord brief`
   already does.
2. Generate the readable ID and body exactly as `coord brief` does today
   (reuse the same helper functions — no duplicated ID-generation logic).
3. Compute the target path (`<dir>/<id>.md` for readable IDs, matching
   `agf/rules/workflow/coordination-id-rule.md`'s filename convention).
4. **Refuse to overwrite an existing file at that path** (a real, if rare,
   collision per #33's probability note) — exit non-zero with a clear
   error rather than silently clobbering.
5. Write the file, then immediately re-read and validate it through
   `agov_coord::coord_validator::validate_file`. If validation fails
   (should not happen if step 2/3 are correct, but this is the
   schema-correct-by-construction guarantee the issue asks for), delete
   the just-written file and exit non-zero rather than leaving an invalid
   message in the channel.
6. On success, print the written path and the `id:` to stdout.

`--ref` resolution reuses `agov_coord::coord_validator::normalize_coord_reference`
so a bare ID or a full filename both work as input.

### `agov handoff take <id>` / `agov handoff complete <id>`

```
agov handoff take <id> --from <role> [--dir docs/coordination]
agov handoff complete <id> --from <role> [--to <role>] [--dir docs/coordination]
```

`take`:
1. Look up `<id>` among pending handoffs via
   `agov_coord::handoff_tracker::find_pending` (the same function
   `agov handoff next` already uses).
2. If `<id>` is not a currently-pending handoff (already resolved, or
   does not exist), exit non-zero with a clear message — this is the
   "validation at mutation time" the issue asks for.
3. If `--from` does not match the handoff's `next_agent`, print a
   **warning**, not a hard failure (the schema does not restrict who may
   ack a handoff, and a human governor may legitimately act on behalf of
   another role) — but a warning is required so a typo'd `--from` is
   visible.
4. Write an `ack` message: `from: <--from>`, `to: <handoff's from>`,
   `references-to: [<id>.md]`, via the same write-and-validate path as
   `coord post`.

`complete`:
1. Same lookup/validation as `take`.
2. Write a `completion` message: `from: <--from>`, `to: all` (or `--to`
   if given), `references-to: [<id>.md]`, `artifacts_owned: []` (releasing
   all claims by `--from`, matching `coord brief`'s existing `completion`
   template) via the write-and-validate path.

Neither command edits the original handoff file — it remains immutable,
exactly as today. Re-running `agov handoff next <role>` immediately after
either command reflects the new resolved state because
`handoff_tracker::find_pending` already treats `ack`/`completion` as
resolving kinds.

### `agov coord list --since <cursor>`

```
agov coord list --since <message-id|ISO-timestamp> [--dir docs/coordination]
                 [--format text|json]
```

- `<cursor>` may be a message `id:` (look up its timestamp, return
  strictly-later messages) or a raw ISO-8601 timestamp.
- Ordering guarantee: returned messages are ordered by the same
  `(timestamp, filename)` sort key `channel_reader.rs` and
  `handoff_tracker.rs`'s `MessageDescriptor::sort_key` already use
  internally — this is **file-arrival order within the reader's own scan**,
  not a cross-writer causal guarantee. Per #33, no global ordering across
  concurrent writers is claimed; `--since` returns "every message whose
  sort key is strictly greater than the cursor's," which is well-defined
  and stable for a single reader re-running the command, even though two
  different concurrent writers' relative order is not itself guaranteed
  by anything but `references-to`.
- Exit codes: `0` = new items returned, `2` = no new items (not an error —
  distinguishable from `0` so a polling loop can distinguish "nothing
  happened" from "something happened" without parsing output), `1` = error
  (bad cursor, directory not found, invalid coordination file encountered
  during the scan).

### Documented dispatcher loop (required deliverable)

```bash
cursor=""
while true; do
  if [ -n "$cursor" ]; then
    out=$(agov coord list --since "$cursor" --format json)
  else
    out=$(agov coord list --format json)
  fi
  code=$?
  case $code in
    0) echo "$out" | jq -c '.[]' | while read -r msg; do
         # dispatch msg to the appropriate agent role
         echo "$msg"
       done
       cursor=$(echo "$out" | jq -r '.[-1].id')
       ;;
    2) : ;;                      # nothing new — normal, keep polling
    *) echo "coord list failed" >&2; break ;;
  esac
  sleep 300
done
```

## Affected files

| File | Change |
|---|---|
| `tooling/rust/crates/agov/src/main.rs` | Add `CoordCommand::Post` variant + `coord_post()` function (factor shared skeleton-building logic out of `coord_brief` into a helper both share); add `HandoffCommand::Take`/`Complete` variants + functions; add `CoordCommand::List` variant + `coord_list()` function |
| `tooling/rust/crates/agov-coord/src/handoff_tracker.rs` | No schema change; may add a small `lookup_by_id(id) -> Option<&PendingHandoff>` helper if not cleanly expressible via existing `find_pending` output |
| `tooling/rust/crates/agov-coord/src/channel_reader.rs` | Add a `--since` cursor filter helper reusing the existing read/sort path |
| `tooling/rust/crates/agov/tests/` | New integration tests: `coord post` produces a file that passes `coord validate`; `coord post` refuses to overwrite an existing path; `handoff take`/`complete` transition a fixture handoff from pending to resolved; `coord list --since` returns the correct suffix and correct exit codes (0 / 2 / 1) |
| `agf/docs/site/docs/concepts/coordination-protocol.md` | Add a short subsection: "Recommended write path" pointing to `coord post`/`handoff take`/`handoff complete`, matching the acceptance criterion "spec documents CLI as the recommended write path" |
| `agf/rules/workflow/coordination-id-rule.md` | No change needed beyond what #33 already adds — filenames produced by `coord post` follow the same convention `coord brief` already documents |
| `README.md` | Add the three new commands to the tooling quick-reference table |

## Migration needed

No schema change, no migration. All new commands generate the exact same
file shapes already accepted by the existing schema and validators.

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | Shared skeleton-building helper produces identical output whether called from `coord brief` (prints) or `coord post` (writes) | `tooling/rust/crates/agov/src/main.rs` |
| Integration | `coord post --kind note --from coder-tester --to all` writes a file; the file passes `agov_coord::coord_validator::validate_file` | `tooling/rust/crates/agov/tests/` (new file, mirroring the fixture-directory pattern in `tests/coord_visibility.rs`) |
| Integration | `coord post` targeting an already-existing path exits non-zero without modifying the existing file | same |
| Integration | `handoff take <id>` on a fixture with one pending handoff writes an `ack`; a subsequent `handoff next <role>` no longer lists it | same |
| Integration | `handoff complete <id>` likewise resolves via a `completion` message | same |
| Integration | `coord list --since <id>` on a fixture returns exactly the messages with a strictly later sort key, in stable order; exit code 2 when nothing new; exit code 1 on a malformed cursor | same |

TDD order per acceptance criterion: write the "messages created via CLI
pass `agov validate` without exception" test first (RED — command does not
exist yet), then implement `coord post`, then the handoff commands, then
`coord list`.

## Risks

| Risk | Mitigation |
|---|---|
| Duplicating logic between `coord brief` (print) and `coord post` (write) drifts over time | Factor the shared skeleton-building code into one helper both call; this is a refactor of existing code, not new parallel logic |
| `coord post`'s collision-refusal (do not overwrite) needs #33's directory-aware duplicate check to be meaningful across the whole channel, not just the file being written | Documented dependency; `coord post` only needs to check the single target path it is about to write, which does not require #33's cross-file scan — the dependency is on #33's *documented* collision-probability claim informing the UX message, not on #33's validator change being merged first. Sequencing note updated accordingly: `coord post`'s local overwrite-check can land independently of #33's validator change, but the plan is still sequenced after #33 per the human-set cluster ordering. |
| `--since` cursor resolution ambiguity (message ID vs. timestamp) | Accept both explicitly; if the cursor string parses as a valid coordination ID, look up its timestamp; otherwise attempt ISO-8601 parse; reject anything else with exit code 1 and a clear message |

## Rollback

Each new subcommand is additive (new `Subcommand` variants + new
functions). Reverting removes the variants and functions; no schema,
no existing command's behaviour, and no existing test is affected.

## Assumption flagged for the identity/accountability cluster

Every mutating command in this plan (`coord post`, `handoff take`,
`handoff complete`) requires an explicit `--from <role>` flag — exactly
matching how `coord brief --from` already works today. This plan does
**not** introduce any automatic identity/session detection (no env var,
no git-config-derived identity, no session file). If the identity and
accountability cluster (issues #35/#45/#34/#47) introduces a session or
identity convention that determines "who is acting" without an explicit
flag, `coord post`/`handoff take`/`handoff complete` should be revisited to
consume it — but until that lands, these commands assume the caller
(human or AI agent runtime) always supplies `--from` explicitly, same as
the existing `coord brief`. The human reconciler should check this
assumption against what the identity cluster actually proposes.

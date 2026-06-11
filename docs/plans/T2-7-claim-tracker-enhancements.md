---
id: T2-7-claim-tracker-enhancements
status: pending-gate1
topic: claim-tracker-partial-release-and-aliases
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundaries, auth, secrets, or tenant isolation changes; ownership tracking is an internal coordination tool concern
  - role: contribution-auditor
    status: not-applicable
    reason: governor-reviewer Gate 1 review covers the semantic changes; both additions are orthogonal and bounded to ClaimTracker plus config loading in Mix tasks
---

# Plan: T2-7 — ClaimTracker enhancements

**Date:** 2026-06-11T08:27:19Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T2-6 (ClaimTracker, merged ✓), T1-4 (ChannelReader, merged ✓)

---

## Intent

Two gaps in ClaimTracker, both discovered during active project use:

**Gap 1 — `releases_artifacts` (partial release).** A `completion` message with
a non-empty `artifacts_owned` list currently falls through to the catch-all
clause and does nothing — the listed paths remain owned. The intended semantic
(release only those specific paths) is never executed. This means an agent
cannot release a subset of its claimed artifacts when completing one task while
continuing another.

**Gap 2 — from-alias mapping (L004).** The coordination channel's `from` naming
convention migrated mid-project from model names (`claude`) to role identifiers
(`planner-architect`). Claims made under the old name are permanently orphaned
because no future completion can match their `from` value. There is no
administrative release mechanism.

Both gaps are bounded to ClaimTracker semantics and optional configuration.
Neither requires a new message kind or schema change.

---

## Scope

Four deliverables:

1. **ClaimTracker: partial release** — new `process_message` clause handles
   `completion` with non-empty `artifacts_owned`: releases only the listed paths
   owned by the sender (or its aliases). Existing empty-list clause unchanged.

2. **ClaimTracker: `from_aliases` option** — new keyword option on `scan/2`.
   When present, `resolve_from/2` maps any alias key to its canonical value
   before `from`-field comparison. A completion from `planner-architect` can
   release claims from `claude` if the alias map says `claude → planner-architect`.

3. **`Agov.CoordConfig`** — new pure module. Reads an optional
   `agov-config.yaml` file from the coordination directory; returns a
   `from_aliases` map (empty if the file is absent or has no `from_aliases`
   key). Keeps I/O out of ClaimTracker.

4. **Mix task wiring** — `mix agov.coord.stale` and `mix agov.coord.status`
   call `CoordConfig.load/1` to read aliases from the coord dir and pass them
   to `ClaimTracker.scan/2` via the `from_aliases:` option.

5. **Project alias config** — `docs/coordination/agov-config.yaml` with the
   project's known alias mapping (`claude → planner-architect`) to clear the
   L004 orphaned claims.

No new message kinds. No schema changes (completion with non-empty
`artifacts_owned` is already valid per the schema; the semantic was just
unimplemented).

---

## Affected files

| File | Action |
|---|---|
| `tooling/elixir/lib/agov/claim_tracker.ex` | Modify — partial release clause + aliases |
| `tooling/elixir/lib/agov/coord_config.ex` | Create — optional config reader |
| `tooling/elixir/lib/mix/tasks/agov.coord.stale.ex` | Modify — load aliases, pass to scan |
| `tooling/elixir/lib/mix/tasks/agov.coord.status.ex` | Modify — load aliases, pass to scan |
| `tooling/elixir/test/agov/claim_tracker_test.exs` | Modify — new tests for both features |
| `tooling/elixir/test/agov/coord_config_test.exs` | Create — unit tests |
| `docs/coordination/agov-config.yaml` | Create — project alias config |

---

## ClaimTracker design changes

### Partial release

New clause inserted **before** the existing empty-list clause (pattern specificity
order matters in Elixir):

```elixir
defp process_message(
       %{"kind" => "completion", "artifacts_owned" => paths} = msg,
       active,
       released,
       aliases
     )
     when is_list(paths) and paths != [] do
  from = resolve_from(msg["from"], aliases)

  {releasing, remaining} =
    Enum.split_with(active, fn {path, entry} ->
      path in paths and resolve_from(entry.from, aliases) == from
    end)

  newly_released = Enum.map(releasing, fn {p, entry} -> Map.put(entry, :path, p) end)
  {:ok, Map.new(remaining), released ++ newly_released}
end
```

The existing empty-list clause becomes:

```elixir
defp process_message(
       %{"kind" => "completion", "artifacts_owned" => []} = msg,
       active,
       released,
       aliases
     ) do
  from = resolve_from(msg["from"], aliases)

  {releasing, remaining} =
    Enum.split_with(active, fn {_path, entry} ->
      resolve_from(entry.from, aliases) == from
    end)

  newly_released = Enum.map(releasing, fn {path, entry} -> Map.put(entry, :path, path) end)
  {:ok, Map.new(remaining), released ++ newly_released}
end
```

### Alias resolution

```elixir
defp resolve_from(from, aliases) do
  Map.get(aliases, from, from)
end
```

`aliases` is a plain `%{string => string}` map. Both the claim's `from` and the
completion's `from` are resolved before comparison, so either side can hold the
alias key or the canonical value.

### API change

`process_messages/2` (previously `process_messages/1`) threads `aliases` through.
`scan/2` already accepts a keyword list; the new key is `from_aliases:`:

```elixir
def scan(messages, opts \\ []) do
  now     = Keyword.get(opts, :now, DateTime.utc_now())
  aliases = Keyword.get(opts, :from_aliases, %{})
  ...
end
```

This is backwards-compatible: callers not passing `from_aliases:` get `%{}` and
existing behaviour is unchanged.

---

## CoordConfig design

```elixir
defmodule Agov.CoordConfig do
  @moduledoc false

  @config_filename "agov-config.yaml"

  def load(coord_dir) do
    path = Path.join(coord_dir, @config_filename)

    case File.read(path) do
      {:error, :enoent} -> %{from_aliases: %{}}
      {:ok, content} ->
        parsed = YamlElixir.read_from_string!(content)
        aliases = parsed |> Map.get("from_aliases", %{}) |> normalize_aliases()
        %{from_aliases: aliases}
    end
  end

  # Ensure both keys and values are strings; drop invalid entries
  defp normalize_aliases(map) when is_map(map) do
    map
    |> Enum.filter(fn {k, v} -> is_binary(k) and is_binary(v) end)
    |> Map.new()
  end

  defp normalize_aliases(_), do: %{}
end
```

`YamlElixir` is already a dependency (used by `CoordValidator`). Absent file →
empty aliases (no error). Invalid `from_aliases` value (not a map, non-string
keys/values) → silently dropped.

---

## Project alias config

`docs/coordination/agov-config.yaml`:

```yaml
# Coordination channel configuration for ClaimTracker.
# from_aliases: maps retired from-identifiers to their canonical successors.
# A completion from the canonical name releases claims made under the alias.
from_aliases:
  claude: planner-architect
```

This resolves the L004 orphaned claims: `mix agov.coord.status` and
`mix agov.coord.stale` will load this config automatically and treat
`from: claude` claims as releasable by `from: planner-architect` completions.

The three seq 3 orphaned claims will appear as `released` rather than `active`
once a completion from `planner-architect` is posted that releases them — OR
they will at least not appear under `Ownership:` if a past completion from
`planner-architect` with `artifacts_owned: []` exists. Let me verify: yes — the
T1-1 Phase B completion (seq 6 or similar) was from `planner-architect` with
`artifacts_owned: []`, which released all paths by that sender. With alias
resolution, that completion will also release the seq 3 claims from `claude`.

---

## Implementation plan

### Step 1 — Tests first (TDD RED)

**`claim_tracker_test.exs` additions:**

| Test | Expected |
|---|---|
| completion with non-empty `artifacts_owned` releases only those paths | listed paths removed from active; other paths by same sender remain |
| completion with non-empty `artifacts_owned` does not release unlisted paths | unlisted paths by same sender still active |
| completion with non-empty `artifacts_owned` from a different sender does nothing | paths remain active |
| `from_aliases` option: completion from canonical releases claim from alias | alias claim appears in released |
| `from_aliases` option: completion from alias releases claim from canonical | canonical claim appears in released |
| `from_aliases: %{}` (default): strict matching unchanged | existing tests still pass |

**`coord_config_test.exs` (new):**

| Test | Expected |
|---|---|
| Missing file returns empty aliases | `%{from_aliases: %{}}` |
| Valid config returns alias map | `%{from_aliases: %{"claude" => "planner-architect"}}` |
| Config with no `from_aliases` key returns empty | `%{from_aliases: %{}}` |
| Invalid `from_aliases` value (not a map) returns empty | `%{from_aliases: %{}}` |

### Step 2 — Implement ClaimTracker changes (GREEN)

Add partial release clause, update completion clauses to use `resolve_from`,
thread `aliases` through `process_messages/2` and `process_message/4`.

### Step 3 — Implement CoordConfig

Thin reader; uses existing `YamlElixir` dependency.

### Step 4 — Wire Mix tasks

Both `agov.coord.stale` and `agov.coord.status` call `CoordConfig.load(coord_dir)`
and pass `from_aliases:` to `ClaimTracker.scan/2`.

### Step 5 — Create project alias config

`docs/coordination/agov-config.yaml` with the `claude → planner-architect` mapping.

### Step 6 — Verify

```bash
mix agov.coord.status
# Ownership: none (seq 3 claims released by alias resolution)

mix agov.coord.stale
# OK  no stale claims

# Partial release: construct fixture messages and verify ClaimTracker.scan/2
```

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | ClaimTracker partial release + alias resolution (6 new cases) | `test/agov/claim_tracker_test.exs` |
| Unit | CoordConfig.load/1 (4 cases) | `test/agov/coord_config_test.exs` |
| Manual | `mix agov.coord.status` with agov-config.yaml present | Verify seq 3 claims clear |

---

## Risks

| Risk | Mitigation |
|---|---|
| Alias resolution is symmetric — either side can be the key | Both `from` values go through `resolve_from`; tested explicitly |
| `agov-config.yaml` is a new file in the coord dir — CoordValidator may try to parse it | CoordValidator only processes `.md` files; YAML file is safe |
| Existing tests that assert on active/released counts may break | Run full suite after changes; fix any count assertions |
| Completion with listed paths and a sender who owns none of them → no-op | Correct: `split_with` yields empty releasing list |

---

## Rollback

Remove `agov-config.yaml` → aliases revert to empty → L004 orphaned claims
reappear in `mix agov.coord.status`. ClaimTracker API is backwards-compatible.

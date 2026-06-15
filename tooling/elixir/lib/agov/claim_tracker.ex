defmodule Agov.ClaimTracker do
  @moduledoc false

  alias Agov.ClaimFSM

  def scan(messages, opts \\ []) do
    now = Keyword.get(opts, :now, DateTime.utc_now())
    aliases = Keyword.get(opts, :from_aliases, %{})

    case process_messages(messages, aliases, now) do
      {:error, error} ->
        {:error, [error]}

      {:ok, states} ->
        %{stale: stale, active: active, released: released} = format_states(states, now)
        {:ok, %{stale: stale, active: active, released: released}}
    end
  end

  defp process_messages(messages, aliases, now) do
    Enum.reduce_while(messages, {:ok, %{}}, fn msg, {:ok, states} ->
      case process_message(msg, states, aliases, now) do
        {:error, _} = err -> {:halt, err}
        {:ok, new_states} -> {:cont, {:ok, new_states}}
      end
    end)
  end

  defp process_message(%{"kind" => "claim"} = msg, states, _aliases, now) do
    seq = msg["seq"]
    from = msg["from"]
    claimed_at = msg["timestamp"]
    expires_raw = Map.get(msg, "expires_on")

    case parse_expires_on(expires_raw, seq) do
      {:error, _} = err ->
        err

      {:ok, expires_on} ->
        event = {:claim, from, seq, claimed_at, expires_on}

        new_states =
          Enum.reduce(msg["artifacts_owned"] || [], states, fn path, acc ->
            Map.update(acc, path, ClaimFSM.transition(:unclaimed, event, now), fn state ->
              ClaimFSM.transition(state, event, now)
            end)
          end)

        {:ok, new_states}
    end
  end

  defp process_message(
         %{"kind" => "completion", "artifacts_owned" => paths} = msg,
         states,
         aliases,
         now
       )
       when is_list(paths) and paths != [] do
    from = resolve_from(msg["from"], aliases)

    new_states =
      Enum.reduce(paths, states, fn path, acc ->
        Map.update(acc, path, :unclaimed, fn state ->
          ClaimFSM.transition(state, {:complete, completion_from(state, from, aliases)}, now)
        end)
      end)

    {:ok, remove_unclaimed(new_states)}
  end

  defp process_message(
         %{"kind" => "completion", "artifacts_owned" => []} = msg,
         states,
         aliases,
         now
       ) do
    from = resolve_from(msg["from"], aliases)

    new_states =
      Map.new(states, fn {path, state} ->
        {path,
         ClaimFSM.transition(state, {:complete, completion_from(state, from, aliases)}, now)}
      end)

    {:ok, new_states}
  end

  defp process_message(_msg, states, _aliases, _now) do
    {:ok, states}
  end

  defp format_states(states, now) do
    Enum.reduce(states, %{stale: [], active: [], released: []}, fn {path, state}, acc ->
      case ClaimFSM.transition(state, {:tick, now}, now) do
        {:active, entry} ->
          %{acc | active: [Map.put(entry, :path, path) | acc.active]}

        {:stale, entry} ->
          %{acc | stale: [Map.put(entry, :path, path) | acc.stale]}

        {:released, entry} ->
          %{acc | released: [Map.put(entry, :path, path) | acc.released]}
      end
    end)
  end

  defp completion_from({_status, %{from: owner}}, from, aliases) do
    if resolve_from(owner, aliases) == from, do: owner, else: from
  end

  defp completion_from(:unclaimed, from, _aliases), do: from

  defp remove_unclaimed(states) do
    Map.reject(states, fn {_path, state} -> state == :unclaimed end)
  end

  defp resolve_from(from, aliases), do: Map.get(aliases, from, from)

  defp parse_expires_on(nil, _seq), do: {:ok, nil}

  defp parse_expires_on(value, seq) do
    case DateTime.from_iso8601(value) do
      {:ok, dt, _offset} -> {:ok, dt}
      {:error, _} -> {:error, {:invalid_expires_on, seq, value}}
    end
  end
end

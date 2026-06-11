defmodule Agov.ClaimTracker do
  @moduledoc false

  def scan(messages, opts \\ []) do
    now = Keyword.get(opts, :now, DateTime.utc_now())

    case process_messages(messages) do
      {:error, error} ->
        {:error, [error]}

      {:ok, active_map, released} ->
        {stale, active} = classify(active_map, now)
        {:ok, %{stale: stale, active: active, released: released}}
    end
  end

  defp process_messages(messages) do
    Enum.reduce_while(messages, {:ok, %{}, []}, fn msg, {:ok, active, released} ->
      case process_message(msg, active, released) do
        {:error, _} = err -> {:halt, err}
        {:ok, new_active, new_released} -> {:cont, {:ok, new_active, new_released}}
      end
    end)
  end

  defp process_message(%{"kind" => "claim"} = msg, active, released) do
    seq = msg["seq"]
    from = msg["from"]
    claimed_at = msg["timestamp"]
    expires_raw = Map.get(msg, "expires_on")

    case parse_expires_on(expires_raw, seq) do
      {:error, _} = err ->
        err

      {:ok, expires_on} ->
        new_active =
          Enum.reduce(msg["artifacts_owned"] || [], active, fn path, acc ->
            Map.put(acc, path, %{
              from: from,
              seq: seq,
              claimed_at: claimed_at,
              expires_on: expires_on
            })
          end)

        {:ok, new_active, released}
    end
  end

  defp process_message(%{"kind" => "completion", "artifacts_owned" => []} = msg, active, released) do
    from = msg["from"]

    {releasing, remaining} =
      Enum.split_with(active, fn {_path, entry} -> entry.from == from end)

    newly_released = Enum.map(releasing, fn {path, entry} -> Map.put(entry, :path, path) end)
    {:ok, Map.new(remaining), released ++ newly_released}
  end

  defp process_message(_msg, active, released) do
    {:ok, active, released}
  end

  defp classify(active_map, now) do
    Enum.reduce(active_map, {[], []}, fn {path, entry}, {stale, active} ->
      with_path = Map.put(entry, :path, path)

      case entry.expires_on do
        nil ->
          {stale, [with_path | active]}

        dt ->
          if DateTime.compare(dt, now) == :lt do
            {[with_path | stale], active}
          else
            {stale, [with_path | active]}
          end
      end
    end)
  end

  defp parse_expires_on(nil, _seq), do: {:ok, nil}

  defp parse_expires_on(value, seq) do
    case DateTime.from_iso8601(value) do
      {:ok, dt, _offset} -> {:ok, dt}
      {:error, _} -> {:error, {:invalid_expires_on, seq, value}}
    end
  end
end

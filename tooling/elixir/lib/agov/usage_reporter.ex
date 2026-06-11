defmodule Agov.UsageReporter do
  @moduledoc false

  def report(messages, agents, aliases \\ %{}, opts \\ []) do
    last_n = Keyword.get(opts, :last)
    invalid_files = Keyword.get(opts, :invalid_files, [])

    sorted = Enum.sort_by(messages, fn {filename, data} -> {data["seq"], filename} end)

    {window_messages, window} =
      case last_n do
        nil -> {sorted, :all}
        n -> {Enum.take(sorted, -n), {:last, n}}
      end

    resolved = Enum.map(window_messages, fn {filename, data} ->
      canonical = Map.get(aliases, data["from"], data["from"])
      {filename, Map.put(data, "from", canonical)}
    end)

    kind_counts =
      resolved
      |> Enum.group_by(fn {_f, data} -> data["kind"] end)
      |> Map.new(fn {kind, msgs} -> {kind, length(msgs)} end)

    sender_counts =
      resolved
      |> Enum.group_by(fn {_f, data} -> data["from"] end)
      |> Map.new(fn {sender, msgs} -> {sender, length(msgs)} end)

    senders = MapSet.new(Map.keys(sender_counts))

    inactive_agents =
      agents
      |> Enum.filter(fn {_, v} -> v.status == "active" end)
      |> Enum.map(fn {name, _} -> name end)
      |> Enum.reject(&MapSet.member?(senders, &1))
      |> Enum.sort()

    unknown_senders =
      senders
      |> Enum.reject(&Map.has_key?(agents, &1))
      |> Enum.sort()

    %{
      total: length(sorted),
      window: window,
      kind_counts: kind_counts,
      sender_counts: sender_counts,
      inactive_agents: inactive_agents,
      unknown_senders: unknown_senders,
      invalid_files: invalid_files
    }
  end
end

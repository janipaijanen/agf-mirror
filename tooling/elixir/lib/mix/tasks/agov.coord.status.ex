defmodule Mix.Tasks.Agov.Coord.Status do
  use Mix.Task

  @shortdoc "Print coordination channel overview"

  def run(args) do
    {opts, _, _} = OptionParser.parse(args, strict: [dir: :string, recent: :integer])
    coord_dir = Keyword.get(opts, :dir, "../../docs/coordination")
    recent_n = Keyword.get(opts, :recent, 5)

    case Agov.ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        Mix.shell().error("ERR  directory not found: #{coord_dir}")
        exit({:shutdown, 1})

      {:ok, %{messages: [], total: 0}} ->
        Mix.shell().info("Channel: #{coord_dir}  (empty)")

      {:ok, %{messages: messages, total: total, seq_range: {min_seq, max_seq}}} ->
        Mix.shell().info("Channel: #{coord_dir}  (#{total} messages, seq #{min_seq}–#{max_seq})")
        Mix.shell().info("")
        print_ownership(messages)
        print_stale(messages)
        print_handoffs(messages)
        if recent_n > 0, do: print_recent(messages, recent_n)
    end
  end

  defp print_ownership(messages) do
    parsed = Enum.map(messages, &elem(&1, 1))

    case Agov.ClaimTracker.scan(parsed) do
      {:error, _} ->
        Mix.shell().info("Ownership:    (error reading claims)")

      {:ok, %{active: []}} ->
        Mix.shell().info("Ownership:    none")

      {:ok, %{active: active}} ->
        Mix.shell().info("Ownership:")

        Enum.each(active, fn entry ->
          Mix.shell().info(
            "  #{entry.path}  claimed-by: #{entry.from}  seq: #{entry.seq}"
          )
        end)
    end

    Mix.shell().info("")
  end

  defp print_stale(messages) do
    parsed = Enum.map(messages, &elem(&1, 1))

    case Agov.ClaimTracker.scan(parsed) do
      {:error, _} ->
        Mix.shell().info("Stale claims: (error)")

      {:ok, %{stale: []}} ->
        Mix.shell().info("Stale claims: none")

      {:ok, %{stale: stale}} ->
        Mix.shell().info("Stale claims:")

        Enum.each(stale, fn entry ->
          Mix.shell().info(
            "  #{entry.path}  claimed-by: #{entry.from}  seq: #{entry.seq}" <>
              "  expired: #{DateTime.to_iso8601(entry.expires_on)}"
          )
        end)
    end

    Mix.shell().info("")
  end

  defp print_handoffs(messages) do
    handoffs =
      messages
      |> Enum.filter(fn {_file, data} -> data["kind"] == "handoff" end)
      |> Enum.map(fn {_file, data} -> data end)
      |> Enum.sort_by(& &1["seq"], :desc)

    case handoffs do
      [] ->
        Mix.shell().info("Pending handoffs:")
        Mix.shell().info("  (none)")

      _ ->
        Mix.shell().info("Pending handoffs (recent):")

        Enum.each(handoffs, fn data ->
          h = data["handoff"]
          status = h["status"]
          Mix.shell().info(
            "  → #{h["next_agent"]}  seq: #{data["seq"]}  #{data["topic"]}  [#{status}]"
          )
        end)
    end

    Mix.shell().info("")
  end

  defp print_recent(messages, n) do
    recent =
      messages
      |> Enum.sort_by(fn {_file, data} -> data["seq"] end, :desc)
      |> Enum.take(n)

    Mix.shell().info("Recent messages (last #{n}):")

    Enum.each(recent, fn {_file, data} ->
      kind = String.pad_trailing(data["kind"], 14)
      Mix.shell().info(
        "  seq #{String.pad_leading(to_string(data["seq"]), 3)}  " <>
          "#{data["from"]} → #{data["to"]}  #{kind}  #{data["topic"]}"
      )
    end)
  end
end

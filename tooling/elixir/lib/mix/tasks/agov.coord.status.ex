defmodule Mix.Tasks.Agov.Coord.Status do
  use Mix.Task

  @shortdoc "Print coordination channel overview"

  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    {opts, _, _} = OptionParser.parse(args, strict: [dir: :string, recent: :integer])
    coord_dir = Keyword.get(opts, :dir, "../../docs/coordination")
    recent_n = Keyword.get(opts, :recent, 5)
    config = Agov.CoordConfig.load(coord_dir)

    case Agov.ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        Mix.shell().error("ERR  directory not found: #{coord_dir}")
        exit({:shutdown, 1})

      {:ok, %{messages: [], total: 0}} ->
        Mix.shell().info("Channel: #{coord_dir}  (empty)")

      {:ok, %{messages: messages, total: total, seq_range: {min_seq, max_seq}}} ->
        Mix.shell().info("Channel: #{coord_dir}  (#{total} messages, seq #{min_seq}–#{max_seq})")
        Mix.shell().info("")
        print_ownership(messages, config.from_aliases)
        print_stale(messages, config.from_aliases)
        print_handoffs(messages)
        if recent_n > 0, do: print_recent(messages, recent_n)
    end
  end

  defp print_ownership(messages, aliases) do
    parsed = Enum.map(messages, &elem(&1, 1))

    case Agov.ClaimTracker.scan(parsed, from_aliases: aliases) do
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

  defp print_stale(messages, aliases) do
    parsed = Enum.map(messages, &elem(&1, 1))

    case Agov.ClaimTracker.scan(parsed, from_aliases: aliases) do
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
    case Agov.HandoffTracker.scan(messages) do
      {:ok, %{pending: []}} ->
        Mix.shell().info("Pending handoffs: none")

      {:ok, %{pending: pending}} ->
        Mix.shell().info("Pending handoffs:")

        Enum.each(pending, fn entry ->
          Mix.shell().info(
            "  → #{entry.to}  seq: #{entry.seq}  #{entry.topic}  [#{entry.status}]"
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

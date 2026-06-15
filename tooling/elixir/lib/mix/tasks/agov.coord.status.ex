defmodule Mix.Tasks.Agov.Coord.Status do
  use Mix.Task

  alias Agov.CoordIdentity

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

      {:ok, %{messages: messages, total: total, timestamp_range: timestamp_range}} ->
        Mix.shell().info("Channel: #{coord_dir}  (#{total} messages, #{range(timestamp_range)})")
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
          Mix.shell().info("  #{entry.path}  claimed-by: #{entry.from}  #{claim_label(entry)}")
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
            "  #{entry.path}  claimed-by: #{entry.from}  #{claim_label(entry)}" <>
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
            "  → #{entry.to}  #{entry.display_id}  #{entry.topic}  [#{entry.status}]"
          )
        end)
    end

    Mix.shell().info("")
  end

  defp print_recent(messages, n) do
    recent =
      messages
      |> Enum.sort_by(fn {file, data} -> CoordIdentity.sort_key(file, data) end, :desc)
      |> Enum.take(n)

    Mix.shell().info("Recent messages (last #{n}):")

    Enum.each(recent, fn {_file, data} ->
      kind = String.pad_trailing(data["kind"], 14)

      Mix.shell().info(
        "  #{CoordIdentity.display(data)}  " <>
          "#{data["from"]} → #{data["to"]}  #{kind}  #{data["topic"]}"
      )
    end)
  end

  defp range(nil), do: "(empty)"
  defp range({min, max}), do: "#{min} to #{max}"

  defp claim_label(entry) do
    cond do
      is_binary(Map.get(entry, :id)) -> "id #{String.slice(entry.id, 0, 8)}"
      not is_nil(entry.seq) -> "seq #{entry.seq}"
      true -> "id <unknown>"
    end
  end
end

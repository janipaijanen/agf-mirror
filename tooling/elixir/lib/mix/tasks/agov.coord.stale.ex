defmodule Mix.Tasks.Agov.Coord.Stale do
  use Mix.Task

  @shortdoc "Report stale artifact claims in the coordination channel"

  def run(args) do
    {opts, _, _} = OptionParser.parse(args, strict: [dir: :string])
    coord_dir = Keyword.get(opts, :dir, "../../docs/coordination")
    config = Agov.CoordConfig.load(coord_dir)

    case File.ls(coord_dir) do
      {:error, :enoent} ->
        Mix.shell().error("ERR  directory not found: #{coord_dir}")
        exit({:shutdown, 2})

      {:ok, files} ->
        files
        |> Enum.filter(&String.ends_with?(&1, ".md"))
        |> Enum.sort()
        |> load_messages(coord_dir)
        |> run_tracker(config.from_aliases)
    end
  end

  defp load_messages(filenames, coord_dir) do
    Enum.flat_map(filenames, fn filename ->
      path = Path.join(coord_dir, filename)

      case Agov.CoordValidator.validate_file(path) do
        {:ok, data} -> [{filename, data}]
        {:error, _} -> []
      end
    end)
  end

  defp run_tracker(file_messages, aliases) do
    seq_to_file = Map.new(file_messages, fn {filename, data} -> {data["seq"], filename} end)
    messages = Enum.map(file_messages, &elem(&1, 1))

    case Agov.ClaimTracker.scan(messages, now: DateTime.utc_now(), from_aliases: aliases) do
      {:error, errors} ->
        Enum.each(errors, fn {:invalid_expires_on, seq, value} ->
          filename = Map.get(seq_to_file, seq, "seq=#{seq}")
          Mix.shell().error("ERR  #{filename}  invalid expires_on: #{value}")
        end)

        exit({:shutdown, 2})

      {:ok, %{stale: []}} ->
        Mix.shell().info("OK  no stale claims")

      {:ok, %{stale: stale}} ->
        Enum.each(stale, fn entry ->
          expires_str = entry.expires_on |> DateTime.to_iso8601()
          Mix.shell().info(
            "STALE  #{entry.path}  claimed-by: #{entry.from}  seq: #{entry.seq}  " <>
              "claimed-at: #{entry.claimed_at}  expired-at: #{expires_str}"
          )
        end)

        exit({:shutdown, 1})
    end
  end
end

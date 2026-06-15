defmodule Mix.Tasks.Agov.Usage.Report do
  use Mix.Task

  @shortdoc "Print coordination channel usage report"

  @default_coord_dir "../../docs/coordination"
  @default_manifest "../../agf/agents/MANIFEST.yaml"

  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    {opts, _, _} =
      OptionParser.parse(args,
        strict: [dir: :string, manifest: :string, last: :integer]
      )

    coord_dir = Keyword.get(opts, :dir, @default_coord_dir)
    manifest_path = Keyword.get(opts, :manifest, @default_manifest)
    last_n = Keyword.get(opts, :last)

    agents = load_agents(manifest_path)
    config = Agov.CoordConfig.load(coord_dir)

    case Agov.ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        Mix.shell().error("ERR  directory not found: #{coord_dir}")
        exit({:shutdown, 1})

      {:ok,
       %{
         messages: messages,
         total: total,
         timestamp_range: timestamp_range,
         invalid_files: invalid_files
       }} ->
        report_opts = [invalid_files: invalid_files] |> maybe_put(:last, last_n)

        report =
          Agov.UsageReporter.report(
            messages,
            agents,
            config.from_aliases,
            report_opts
          )

        print_report(report, coord_dir, total, timestamp_range)
    end
  end

  defp load_agents(path) do
    case Agov.ManifestReader.load(path) do
      {:ok, agents} ->
        agents

      {:error, :enoent} ->
        Mix.shell().error("WARN manifest not found: #{path}  (agent cross-reference unavailable)")
        %{}
    end
  end

  defp print_report(report, coord_dir, total, timestamp_range) do
    range_str =
      case timestamp_range do
        nil -> "(empty)"
        {min, max} -> "#{min} to #{max}"
      end

    Mix.shell().info("Channel: #{coord_dir}  (#{total} messages, #{range_str})")

    window_str =
      case report.window do
        :all -> "all messages"
        {:last, n} -> "last #{n} messages"
      end

    Mix.shell().info("Window:  #{window_str}")
    Mix.shell().info("")

    print_kind_counts(report.kind_counts)
    print_sender_counts(report.sender_counts, report.inactive_agents)
    print_inactive(report.inactive_agents)
    print_unknown(report.unknown_senders)
    print_invalid(report.invalid_files)
  end

  defp print_kind_counts(kind_counts) when map_size(kind_counts) == 0 do
    Mix.shell().info("Kind distribution: (none)")
    Mix.shell().info("")
  end

  defp print_kind_counts(kind_counts) do
    Mix.shell().info("Kind distribution:")

    kind_counts
    |> Enum.sort_by(fn {_, count} -> -count end)
    |> Enum.each(fn {kind, count} ->
      Mix.shell().info("  #{String.pad_trailing(kind, 18)}  #{count}")
    end)

    Mix.shell().info("")
  end

  defp print_sender_counts(sender_counts, _inactive_agents) when map_size(sender_counts) == 0 do
    Mix.shell().info("Sender activity (aliases applied): (none)")
    Mix.shell().info("")
  end

  defp print_sender_counts(sender_counts, inactive_agents) do
    Mix.shell().info("Sender activity (aliases applied):")

    sender_counts
    |> Enum.sort_by(fn {_, count} -> -count end)
    |> Enum.each(fn {sender, count} ->
      flag = if sender in inactive_agents, do: "  ← inactive", else: ""
      Mix.shell().info("  #{String.pad_trailing(sender, 24)}  #{count}#{flag}")
    end)

    Mix.shell().info("")
  end

  defp print_inactive([]) do
    Mix.shell().info("Inactive active-status agents: none")
    Mix.shell().info("")
  end

  defp print_inactive(agents) do
    Mix.shell().info("Inactive active-status agents:")
    Enum.each(agents, fn name -> Mix.shell().info("  #{name}") end)
    Mix.shell().info("")
  end

  defp print_unknown([]) do
    Mix.shell().info("Unknown senders (after alias resolution): none")
    Mix.shell().info("")
  end

  defp print_unknown(senders) do
    Mix.shell().info("Unknown senders (after alias resolution):")
    Enum.each(senders, fn name -> Mix.shell().info("  #{name}") end)
    Mix.shell().info("")
  end

  defp print_invalid([]) do
    Mix.shell().info("Invalid coordination files: none")
  end

  defp print_invalid(files) do
    Mix.shell().error("WARN  #{length(files)} invalid coordination file(s) skipped:")
    Enum.each(files, fn f -> Mix.shell().error("  #{f}") end)
  end

  defp maybe_put(opts, _key, nil), do: opts
  defp maybe_put(opts, key, value), do: Keyword.put(opts, key, value)
end

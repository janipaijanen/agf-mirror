defmodule Mix.Tasks.Agov.Coord.Visibility do
  use Mix.Task

  @shortdoc "Scan local branches for hidden coordination obligations"

  @moduledoc """
  Scans local branches for coordination messages that create obligations but are
  not visible from the base branch.

      mix agov.coord.visibility [--base-branch main] [--branch feat/x] [--format text|json]
  """

  @default_coord_dir "../../docs/coordination"

  @impl Mix.Task
  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    {opts, _positional, _invalid} =
      OptionParser.parse(args,
        strict: [base_branch: :string, branch: :keep, dir: :string, format: :string],
        aliases: [b: :base_branch]
      )

    scan_opts = [
      base_branch: Keyword.get(opts, :base_branch, "main"),
      dir: Keyword.get(opts, :dir, @default_coord_dir)
    ]

    scan_opts =
      case Keyword.get_values(opts, :branch) do
        [] -> scan_opts
        branches -> Keyword.put(scan_opts, :branches, branches)
      end

    format = Keyword.get(opts, :format, "text")

    case Agov.CoordVisibility.scan(scan_opts) do
      {:ok, report} ->
        print_report(report, format)
        code = Agov.CoordVisibility.exit_code(report)
        if code != 0, do: exit({:shutdown, code})

      {:error, reason} ->
        Mix.shell().error("ERR coordination visibility scan failed: #{inspect(reason)}")
        exit({:shutdown, 2})
    end
  end

  defp print_report(report, "json") do
    Mix.shell().info(Agov.CoordVisibility.format_json(report))
  end

  defp print_report(report, _text) do
    Mix.shell().info(Agov.CoordVisibility.format_text(report))
  end
end

defmodule Mix.Tasks.Agov.Token.Roi do
  use Mix.Task

  @shortdoc "Record and report local token ROI telemetry"

  @moduledoc """
  Records lightweight, local token ROI measurements and reports accumulated
  output sizes.

      mix agov.token.roi report [--log <path>]
      mix agov.token.roi record --command <name> --chars <n> [--workflow <name>] [--manual-chars <n>] [--log <path>]

  Automatic collection writes tool-output records for supported AGF commands.
  Set `AGOV_TOKEN_ROI=0` to disable automatic writes. The default log path is
  `../../tmp/agov-token-roi.jsonl` or `AGOV_TOKEN_ROI_LOG` when set.
  """

  @impl Mix.Task
  def run(args) do
    {subcommand, rest} =
      case args do
        [] -> {"report", []}
        [head | tail] when head in ["report", "record"] -> {head, tail}
        _ -> {"report", args}
      end

    case subcommand do
      "report" -> report(rest)
      "record" -> record(rest)
    end
  end

  defp report(args) do
    {opts, _, _} = OptionParser.parse(args, strict: [log: :string])
    log_path = Keyword.get(opts, :log, Agov.TokenRoi.default_log_path())

    case Agov.TokenRoi.load(log_path) do
      {:ok, records} ->
        records
        |> Agov.TokenRoi.summarize()
        |> Agov.TokenRoi.format_report(log_path)
        |> Mix.shell().info()

      {:error, reason} ->
        Mix.shell().error("ERR  could not read token ROI log #{log_path}: #{inspect(reason)}")
        exit({:shutdown, 1})
    end
  end

  defp record(args) do
    {opts, _, _} =
      OptionParser.parse(args,
        strict: [
          log: :string,
          workflow: :string,
          command: :string,
          chars: :integer,
          manual_chars: :integer
        ]
      )

    with {:ok, command} <- required(opts, :command),
         {:ok, chars} <- required(opts, :chars) do
      log_path = Keyword.get(opts, :log, Agov.TokenRoi.default_log_path())

      attrs =
        %{
          command: command,
          workflow: Keyword.get(opts, :workflow),
          chars: chars,
          manual_chars: Keyword.get(opts, :manual_chars)
        }
        |> Map.reject(fn {_key, value} -> is_nil(value) end)

      Agov.TokenRoi.record_measurement(attrs, log_path: log_path)
      Mix.shell().info("Recorded token ROI measurement: #{log_path}")
    else
      {:error, option} ->
        Mix.shell().error(
          "Usage: mix agov.token.roi record --command <name> --chars <n> [--workflow <name>] [--manual-chars <n>]"
        )

        Mix.shell().error(
          "Missing required option: --#{String.replace(to_string(option), "_", "-")}"
        )

        exit({:shutdown, 1})
    end
  end

  defp required(opts, key) do
    case Keyword.fetch(opts, key) do
      {:ok, value} -> {:ok, value}
      :error -> {:error, key}
    end
  end
end

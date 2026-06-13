defmodule Agov.TokenRoi do
  @moduledoc false

  @default_log_path "../../tmp/agov-token-roi.jsonl"

  def default_log_path do
    System.get_env("AGOV_TOKEN_ROI_LOG", @default_log_path)
  end

  def estimate_tokens(chars) when is_integer(chars) and chars >= 0 do
    div(chars + 3, 4)
  end

  def record_tool_output(command, args, output, opts \\ [])
      when is_binary(command) and is_list(args) and is_binary(output) do
    if disabled?() do
      :ok
    else
      record =
        %{
          event: "tool_output",
          timestamp: DateTime.utc_now() |> DateTime.truncate(:second) |> DateTime.to_iso8601(),
          command: command,
          args: args,
          workflow: Keyword.get(opts, :workflow),
          chars: String.length(output),
          approx_tokens: estimate_tokens(String.length(output))
        }
        |> drop_nil_values()

      append_record(Keyword.get(opts, :log_path, default_log_path()), record)
    end
  end

  def record_measurement(attrs, opts \\ []) when is_map(attrs) do
    chars = Map.fetch!(attrs, :chars)

    record =
      attrs
      |> Map.merge(%{
        event: Map.get(attrs, :event, "measurement"),
        timestamp:
          Map.get(
            attrs,
            :timestamp,
            DateTime.utc_now() |> DateTime.truncate(:second) |> DateTime.to_iso8601()
          ),
        approx_tokens: Map.get(attrs, :approx_tokens, estimate_tokens(chars))
      })
      |> drop_nil_values()

    append_record(Keyword.get(opts, :log_path, default_log_path()), record)
  end

  def append_record(path, record) when is_binary(path) and is_map(record) do
    path
    |> Path.dirname()
    |> File.mkdir_p!()

    File.write!(path, Jason.encode!(string_keyed(record)) <> "\n", [:append])
    :ok
  end

  def load(path \\ default_log_path()) do
    case File.read(path) do
      {:ok, content} ->
        records =
          content
          |> String.split("\n", trim: true)
          |> Enum.map(&Jason.decode!/1)

        {:ok, records}

      {:error, :enoent} ->
        {:ok, []}

      {:error, reason} ->
        {:error, reason}
    end
  end

  def summarize(records) when is_list(records) do
    command_rows =
      records
      |> Enum.filter(&Map.has_key?(&1, "command"))
      |> Enum.group_by(&Map.get(&1, "command", "unknown"))
      |> Enum.map(fn {command, group} ->
        %{
          command: command,
          count: length(group),
          chars: sum_field(group, "chars"),
          approx_tokens: sum_field(group, "approx_tokens")
        }
      end)
      |> Enum.sort_by(&{-&1.approx_tokens, &1.command})

    workflow_rows =
      records
      |> Enum.filter(&Map.has_key?(&1, "workflow"))
      |> Enum.group_by(&Map.get(&1, "workflow"))
      |> Enum.map(fn {workflow, group} ->
        %{
          workflow: workflow,
          count: length(group),
          chars: sum_field(group, "chars"),
          approx_tokens: sum_field(group, "approx_tokens")
        }
      end)
      |> Enum.sort_by(&{-&1.approx_tokens, &1.workflow})

    manual_chars = sum_field(records, "manual_chars")
    tool_chars = sum_field(records, "chars")
    manual_tokens = estimate_tokens(manual_chars)
    tool_tokens = estimate_tokens(tool_chars)

    %{
      total_records: length(records),
      total_chars: tool_chars,
      total_approx_tokens: tool_tokens,
      manual_chars: manual_chars,
      manual_approx_tokens: manual_tokens,
      estimated_saved_tokens: max(manual_tokens - tool_tokens, 0),
      commands: command_rows,
      workflows: workflow_rows
    }
  end

  def format_report(summary, log_path) do
    [
      "Token ROI telemetry",
      "Log: #{log_path}",
      "Records: #{summary.total_records}",
      "Measured tool output: #{summary.total_chars} chars / #{summary.total_approx_tokens} approx tokens",
      manual_line(summary),
      "",
      format_command_rows(summary.commands),
      "",
      format_workflow_rows(summary.workflows)
    ]
    |> Enum.join("\n")
  end

  defp manual_line(%{manual_chars: 0}) do
    "Manual baseline: none recorded"
  end

  defp manual_line(summary) do
    "Manual baseline: #{summary.manual_chars} chars / #{summary.manual_approx_tokens} approx tokens; " <>
      "estimated saved: #{summary.estimated_saved_tokens} approx tokens"
  end

  defp format_command_rows([]), do: "Commands: none"

  defp format_command_rows(rows) do
    [
      "Commands:"
      | Enum.map(rows, fn row ->
          "  #{String.pad_trailing(row.command, 24)} #{String.pad_leading(to_string(row.count), 3)} runs  " <>
            "#{String.pad_leading(to_string(row.chars), 7)} chars  " <>
            "#{String.pad_leading(to_string(row.approx_tokens), 6)} tokens"
        end)
    ]
    |> Enum.join("\n")
  end

  defp format_workflow_rows([]), do: "Workflows: none"

  defp format_workflow_rows(rows) do
    [
      "Workflows:"
      | Enum.map(rows, fn row ->
          "  #{String.pad_trailing(row.workflow, 28)} #{String.pad_leading(to_string(row.count), 3)} records  " <>
            "#{String.pad_leading(to_string(row.chars), 7)} chars  " <>
            "#{String.pad_leading(to_string(row.approx_tokens), 6)} tokens"
        end)
    ]
    |> Enum.join("\n")
  end

  defp sum_field(records, field) do
    Enum.reduce(records, 0, fn record, acc ->
      case Map.get(record, field) do
        value when is_integer(value) -> acc + value
        _ -> acc
      end
    end)
  end

  defp string_keyed(map) do
    Map.new(map, fn {key, value} -> {to_string(key), value} end)
  end

  defp drop_nil_values(map) do
    Map.reject(map, fn {_key, value} -> is_nil(value) end)
  end

  defp disabled? do
    System.get_env("AGOV_TOKEN_ROI") in ["0", "false", "off"]
  end
end

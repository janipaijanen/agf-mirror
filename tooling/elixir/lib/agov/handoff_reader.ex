defmodule Agov.HandoffReader do
  @moduledoc false

  alias Agov.CoordValidator

  def find_next(coord_dir, agent_role) do
    case File.ls(coord_dir) do
      {:error, :enoent} ->
        {:error, :dir_not_found}

      {:ok, files} ->
        files
        |> Enum.filter(&String.ends_with?(&1, ".md"))
        |> Enum.flat_map(&parse_handoff(coord_dir, &1, agent_role))
        |> Enum.max_by(& &1["seq"], fn -> nil end)
        |> case do
          nil -> {:none}
          data -> {:ok, data}
        end
    end
  end

  def format_summary(data, source_file) do
    h = data["handoff"]

    lines = [
      "Handoff for: #{h["next_agent"]}",
      "Status:      #{h["status"]}",
      "Task:"
    ]

    task_lines = Enum.map(h["task"] || [], &"  - #{&1}")

    must_read_lines =
      case h["must_read"] || [] do
        [] -> []
        items -> ["Must read:" | Enum.map(items, &"  - #{&1}")]
      end

    do_not_touch_lines =
      case h["do_not_touch"] || [] do
        [] -> []
        items -> ["Do not touch:" | Enum.map(items, &"  - #{&1}")]
      end

    source_line = ["Source: #{source_file}"]

    (lines ++ task_lines ++ must_read_lines ++ do_not_touch_lines ++ source_line)
    |> Enum.join("\n")
  end

  defp parse_handoff(dir, filename, agent_role) do
    path = Path.join(dir, filename)

    case CoordValidator.validate_file(path) do
      {:ok, data} ->
        if data["kind"] == "handoff" and data["to"] == agent_role do
          [data |> Map.put("_source_file", filename)]
        else
          []
        end

      _ ->
        []
    end
  end
end

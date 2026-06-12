defmodule Agov.HandoffReader do
  @moduledoc false

  alias Agov.{ChannelReader, HandoffTracker}

  def find_next(coord_dir, agent_role) do
    case ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        {:error, :dir_not_found}

      {:ok, %{messages: messages}} ->
        with {:ok, %{pending: pending}} <- HandoffTracker.scan(messages) do
          pending_seqs =
            pending
            |> Enum.filter(&(&1.to == agent_role))
            |> MapSet.new(& &1.seq)

          messages
          |> Enum.flat_map(fn {filename, data} ->
            if MapSet.member?(pending_seqs, data["seq"]) do
              [Map.put(data, "_source_file", filename)]
            else
              []
            end
          end)
          |> Enum.max_by(& &1["seq"], fn -> nil end)
          |> case do
            nil -> {:none}
            data -> {:ok, data}
          end
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

end

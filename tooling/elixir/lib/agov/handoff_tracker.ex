defmodule Agov.HandoffTracker do
  @moduledoc false

  def scan(file_messages) do
    references_index = build_references_index(file_messages)

    {pending, resolved} =
      file_messages
      |> Enum.filter(fn {_file, data} -> data["kind"] == "handoff" end)
      |> Enum.split_with(fn {filename, data} ->
        referencing_seqs = Map.get(references_index, filename, [])
        not Enum.any?(referencing_seqs, fn ref_seq -> ref_seq > data["seq"] end)
      end)

    {:ok, %{pending: format(pending), resolved: format(resolved)}}
  end

  defp build_references_index(file_messages) do
    Enum.reduce(file_messages, %{}, fn {_file, data}, acc ->
      refs = data["references-to"] || []
      seq = data["seq"]

      Enum.reduce(refs, acc, fn ref_filename, inner ->
        Map.update(inner, ref_filename, [seq], &[seq | &1])
      end)
    end)
  end

  defp format(file_messages) do
    file_messages
    |> Enum.map(fn {_filename, data} ->
      %{
        seq: data["seq"],
        from: data["from"],
        to: data["to"],
        topic: data["topic"],
        status: get_in(data, ["handoff", "status"]) || "ready"
      }
    end)
    |> Enum.sort_by(& &1.seq, :desc)
  end
end

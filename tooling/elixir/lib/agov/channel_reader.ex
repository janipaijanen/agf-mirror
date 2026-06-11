defmodule Agov.ChannelReader do
  @moduledoc false

  alias Agov.CoordValidator

  def scan(coord_dir) do
    case File.ls(coord_dir) do
      {:error, :enoent} ->
        {:error, :dir_not_found}

      {:ok, files} ->
        messages =
          files
          |> Enum.filter(&String.ends_with?(&1, ".md"))
          |> Enum.sort()
          |> Enum.flat_map(fn filename ->
            path = Path.join(coord_dir, filename)

            case CoordValidator.validate_file(path) do
              {:ok, data} -> [{filename, data}]
              {:error, _} -> []
            end
          end)
          |> Enum.sort_by(fn {_file, data} -> data["seq"] end)

        total = length(messages)

        seq_range =
          case messages do
            [] ->
              nil

            _ ->
              seqs = Enum.map(messages, fn {_file, data} -> data["seq"] end)
              {Enum.min(seqs), Enum.max(seqs)}
          end

        {:ok, %{messages: messages, total: total, seq_range: seq_range}}
    end
  end
end

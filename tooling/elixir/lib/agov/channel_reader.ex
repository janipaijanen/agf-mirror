defmodule Agov.ChannelReader do
  @moduledoc false

  alias Agov.CoordValidator

  def scan(coord_dir) do
    case File.ls(coord_dir) do
      {:error, :enoent} ->
        {:error, :dir_not_found}

      {:ok, files} ->
        {messages, invalid_files} =
          files
          |> Enum.filter(&String.ends_with?(&1, ".md"))
          |> Enum.sort()
          |> Enum.reduce({[], []}, fn filename, {valid, invalid} ->
            path = Path.join(coord_dir, filename)

            case CoordValidator.validate_file(path) do
              {:ok, data} -> {[{filename, data} | valid], invalid}
              {:error, _} -> {valid, [filename | invalid]}
            end
          end)

        messages =
          messages
          |> Enum.reverse()
          |> Enum.sort_by(fn {_file, data} -> data["seq"] end)

        invalid_files = Enum.reverse(invalid_files)
        total = length(messages)

        seq_range =
          case messages do
            [] ->
              nil

            _ ->
              seqs = Enum.map(messages, fn {_file, data} -> data["seq"] end)
              {Enum.min(seqs), Enum.max(seqs)}
          end

        {:ok, %{messages: messages, total: total, seq_range: seq_range, invalid_files: invalid_files}}
    end
  end
end

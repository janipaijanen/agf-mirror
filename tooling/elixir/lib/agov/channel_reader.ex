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
              {:ok, data} -> {[{filename, Map.put(data, "_body", body(path))} | valid], invalid}
              {:error, _} -> {valid, [filename | invalid]}
            end
          end)

        messages =
          messages
          |> Enum.reverse()
          |> Enum.sort_by(fn {file, data} -> Agov.CoordIdentity.sort_key(file, data) end)

        invalid_files = Enum.reverse(invalid_files)
        total = length(messages)

        timestamp_range =
          case messages do
            [] ->
              nil

            _ ->
              timestamps = Enum.map(messages, fn {_file, data} -> data["timestamp"] end)
              {Enum.min(timestamps), Enum.max(timestamps)}
          end

        {:ok,
         %{
           messages: messages,
           total: total,
           timestamp_range: timestamp_range,
           seq_range: legacy_seq_range(messages),
           invalid_files: invalid_files
         }}
    end
  end

  defp legacy_seq_range(messages) do
    seqs =
      messages
      |> Enum.map(fn {_file, data} -> data["seq"] end)
      |> Enum.reject(&is_nil/1)

    case seqs do
      [] -> nil
      _ -> {Enum.min(seqs), Enum.max(seqs)}
    end
  end

  defp body(path) do
    with {:ok, content} <- File.read(path),
         [_before, _yaml, rest] <- String.split(content, "---", parts: 3) do
      rest
    else
      _ -> ""
    end
  end
end

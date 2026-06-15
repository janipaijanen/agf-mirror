defmodule Agov.CoordIdentity do
  @moduledoc false

  @ulid_re ~r/^[0-9A-HJKMNP-TV-Z]{26}$/
  @ulid_filename_re ~r/^[0-9A-HJKMNP-TV-Z]{26}-[A-Za-z0-9_-]+\.md$/

  def ulid?(value) when is_binary(value), do: Regex.match?(@ulid_re, value)
  def ulid?(_value), do: false

  def generate_ulid do
    Ulid.generate(System.system_time(:millisecond))
  end

  def ulid_filename?(file) when is_binary(file) do
    file
    |> Path.basename()
    |> then(&Regex.match?(@ulid_filename_re, &1))
  end

  def ulid_filename?(_file), do: false

  def sort_key(file, data) do
    {data["timestamp"] || "", Path.basename(file)}
  end

  def later?(left_file, left_data, right_file, right_data) do
    sort_key(left_file, left_data) > sort_key(right_file, right_data)
  end

  def display(data) do
    case data do
      %{"id" => id} when is_binary(id) -> "id #{String.slice(id, 0, 8)}"
      %{"seq" => seq} -> "seq #{seq}"
      _ -> "id <missing>"
    end
  end

  def entry_fields(file, data) do
    %{
      id: data["id"],
      seq: data["seq"],
      timestamp: data["timestamp"],
      file: file,
      display_id: display(data),
      sort_key: sort_key(file, data)
    }
  end
end

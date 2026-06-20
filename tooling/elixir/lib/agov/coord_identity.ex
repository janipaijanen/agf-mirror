defmodule Agov.CoordIdentity do
  @moduledoc false

  @ulid_re ~r/^[0-9A-HJKMNP-TV-Z]{26}$/
  @readable_re ~r/^[0-9]{8}T[0-9]{6}Z-[a-z0-9][a-z0-9-]*-[a-z0-9]{4,6}$/
  @ulid_filename_re ~r/^[0-9A-HJKMNP-TV-Z]{26}-[A-Za-z0-9_-]+\.md$/
  @legacy_seq_filename_re ~r/^\d{5}-[A-Za-z0-9_-]+\.md$/
  @suffix_alphabet String.graphemes("0123456789abcdefghjkmnpqrstvwxyz")

  def ulid?(value) when is_binary(value), do: Regex.match?(@ulid_re, value)
  def ulid?(_value), do: false

  def readable_id?(value) when is_binary(value), do: Regex.match?(@readable_re, value)
  def readable_id?(_value), do: false

  def valid_id?(value), do: ulid?(value) or readable_id?(value)

  def generate_ulid do
    Ulid.generate(System.system_time(:millisecond))
  end

  def generate_readable_id(role, timestamp \\ DateTime.utc_now()) do
    utc = DateTime.shift_zone!(timestamp, "Etc/UTC")
    "#{basic_utc_timestamp(utc)}-#{role_slug(role)}-#{random_suffix()}"
  end

  def ulid_filename?(file) when is_binary(file) do
    file
    |> Path.basename()
    |> then(&Regex.match?(@ulid_filename_re, &1))
  end

  def ulid_filename?(_file), do: false

  def readable_filename?(file, id \\ nil)

  def readable_filename?(file, id) when is_binary(file) and is_binary(id) do
    basename = Path.basename(file)

    readable_id?(id) and
      (basename == id <> ".md" or String.starts_with?(basename, id <> "-"))
  end

  def readable_filename?(file, nil) when is_binary(file) do
    basename = Path.basename(file)

    with true <- String.ends_with?(basename, ".md"),
         stem <- String.trim_trailing(basename, ".md") do
      readable_id?(stem) or
        stem
        |> possible_readable_ids()
        |> Enum.any?(&readable_id?/1)
    else
      _ -> false
    end
  end

  def readable_filename?(_file, _id), do: false

  def coord_filename?(file) when is_binary(file) do
    basename = Path.basename(file)

    Regex.match?(@legacy_seq_filename_re, basename) or ulid_filename?(basename) or
      readable_filename?(basename)
  end

  def coord_filename?(_file), do: false

  def readable_parts(id) when is_binary(id) do
    case Regex.run(~r/^([0-9]{8}T[0-9]{6}Z)-(.+)-([a-z0-9]{4,6})$/, id) do
      [_, timestamp, role, suffix] -> {:ok, %{timestamp: timestamp, role: role, suffix: suffix}}
      _ -> :error
    end
  end

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

  def role_slug(value) when is_binary(value) do
    value
    |> String.downcase()
    |> String.replace(~r/[^a-z0-9]+/, "-")
    |> String.trim("-")
    |> case do
      "" -> "unknown"
      slug -> slug
    end
  end

  def role_slug(_value), do: "unknown"

  def basic_utc_timestamp(%DateTime{} = timestamp) do
    timestamp
    |> DateTime.shift_zone!("Etc/UTC")
    |> Calendar.strftime("%Y%m%dT%H%M%SZ")
  end

  defp random_suffix(length \\ 4) do
    :crypto.strong_rand_bytes(length)
    |> :binary.bin_to_list()
    |> Enum.map_join(fn byte ->
      Enum.at(@suffix_alphabet, rem(byte, length(@suffix_alphabet)))
    end)
  end

  defp possible_readable_ids(stem) do
    parts = String.split(stem, "-")

    if length(parts) < 4 do
      []
    else
      for suffix_index <- 2..(length(parts) - 2) do
        parts
        |> Enum.take(suffix_index + 1)
        |> Enum.join("-")
      end
    end
  end
end

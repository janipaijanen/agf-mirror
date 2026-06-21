defmodule Agov.CoordBrief do
  @moduledoc false

  @supported_id_formats ~w(ulid readable)

  def generate(args, default_dir \\ "../../docs/coordination") do
    with {:ok, opts} <- parse_args(args),
         {:ok, id_format} <- id_format(opts),
         {:ok, from} <- readable_from(id_format, Keyword.get(opts, :from)) do
      timestamp = timestamp(id_format)
      coord_dir = Keyword.get(opts, :dir, default_dir)
      kind = Keyword.get(opts, :kind, "note")
      to = Keyword.get(opts, :to)
      topic = Keyword.get(opts, :topic)

      config = Agov.CoordConfig.load(coord_dir)
      channel_info = channel_info(coord_dir, config, id_format, from, timestamp)

      gen_opts =
        [kind: kind]
        |> maybe_put(:from, from)
        |> maybe_put(:to, to)
        |> maybe_put(:topic, topic)
        |> maybe_put(:timestamp, iso_timestamp(timestamp))

      Agov.BriefGenerator.generate(channel_info, gen_opts)
    end
  end

  defp parse_args(args) do
    {opts, unknown, invalid} =
      OptionParser.parse(args,
        strict: [
          dir: :string,
          kind: :string,
          from: :string,
          to: :string,
          topic: :string,
          id_format: :string
        ],
        aliases: []
      )

    case {unknown, invalid} do
      {[], []} -> {:ok, opts}
      {_, _} -> {:error, {:invalid_args, unknown ++ Enum.map(invalid, &elem(&1, 0))}}
    end
  end

  defp id_format(opts) do
    format = Keyword.get(opts, :id_format, "ulid")

    if format in @supported_id_formats do
      {:ok, format}
    else
      {:error, {:unknown_id_format, format, @supported_id_formats}}
    end
  end

  defp readable_from("readable", nil), do: {:error, :readable_requires_from}
  defp readable_from(_format, from), do: {:ok, from}

  defp channel_info(coord_dir, config, id_format, from, timestamp) do
    id = next_id(id_format, from, timestamp)

    case Agov.ChannelReader.scan(coord_dir) do
      {:error, :dir_not_found} ->
        %{
          next_id: id,
          suggested_filename: suggested_filename(id_format, from, id),
          total: 0,
          timestamp_range: nil,
          active_claims: 0
        }

      {:ok, %{messages: [], total: 0}} ->
        %{
          next_id: id,
          suggested_filename: suggested_filename(id_format, from, id),
          total: 0,
          timestamp_range: nil,
          active_claims: 0
        }

      {:ok, %{messages: messages, total: total, timestamp_range: range}} ->
        parsed = Enum.map(messages, &elem(&1, 1))

        active_claims =
          case Agov.ClaimTracker.scan(parsed, from_aliases: config.from_aliases) do
            {:ok, %{active: active}} -> length(active)
            _ -> 0
          end

        %{
          next_id: id,
          suggested_filename: suggested_filename(id_format, from, id),
          total: total,
          timestamp_range: range,
          active_claims: active_claims
        }
    end
  end

  defp timestamp("readable"), do: DateTime.utc_now() |> DateTime.truncate(:second)
  defp timestamp("ulid"), do: nil

  defp iso_timestamp(nil), do: nil
  defp iso_timestamp(%DateTime{} = timestamp), do: DateTime.to_iso8601(timestamp)

  defp next_id("readable", from, timestamp),
    do: Agov.CoordIdentity.generate_readable_id(from, timestamp)

  defp next_id("ulid", _from, _timestamp), do: Agov.CoordIdentity.generate_ulid()

  defp suggested_filename("readable", _from, id), do: id <> ".md"

  defp suggested_filename("ulid", from, id) when is_binary(from), do: id <> "-" <> from <> ".md"

  defp suggested_filename("ulid", _from, id), do: id <> "-<from>.md"

  defp maybe_put(opts, _key, nil), do: opts
  defp maybe_put(opts, key, value), do: Keyword.put(opts, key, value)
end

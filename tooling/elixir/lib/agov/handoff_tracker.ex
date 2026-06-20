defmodule Agov.HandoffTracker do
  @moduledoc """
  Tracks unresolved coordination handoffs by graph reachability.

  A handoff is pending until a later resolver message can be reached from it
  through `references-to` front matter or coordination filename mentions in
  message bodies. Resolver message kinds are `decision-record`, `completion`,
  and `ack`.

  This is intentionally not a per-message lifecycle FSM. Resolution can arrive
  through a chain of related messages, so the tracker builds a descendant graph
  and searches outward from each handoff.
  """

  alias Agov.CoordIdentity

  @resolver_kinds ~w(decision-record completion ack)
  @coord_filename_re ~r/(?:^|[\s`"'(\/])((?:(?:[0-9]{8}T[0-9]{6}Z-[a-z0-9][a-z0-9-]*-[a-z0-9]{4,6})(?:-[A-Za-z0-9_-]+)?|(?:\d{5}|[0-9A-HJKMNP-TV-Z]{26})-[A-Za-z0-9_-]+)\.md)(?=$|[\s`"'),.:;])/

  def scan(file_messages) do
    messages = normalize_messages(file_messages)
    descendants_by_file = build_descendants_by_file(messages)

    {pending, resolved} =
      messages
      |> Enum.filter(fn message -> message.data["kind"] == "handoff" end)
      |> Enum.split_with(fn message ->
        not resolved?(message, descendants_by_file, messages)
      end)

    {:ok, %{pending: format(pending), resolved: format(resolved)}}
  end

  defp normalize_messages(file_messages) do
    file_messages
    |> Enum.map(fn {file, data} ->
      %{
        file: file,
        basename: Path.basename(file),
        seq: data["seq"],
        id: data["id"],
        timestamp: data["timestamp"],
        display_id: CoordIdentity.display(data),
        sort_key: CoordIdentity.sort_key(file, data),
        data: data,
        refs: referenced_files(data)
      }
    end)
  end

  defp referenced_files(data) do
    frontmatter_refs =
      data
      |> Map.get("references-to", [])
      |> Enum.map(&Path.basename/1)

    body_refs =
      data
      |> Map.get("_body", "")
      |> body_file_mentions()

    (frontmatter_refs ++ body_refs)
    |> Enum.reject(&(&1 == ""))
    |> Enum.uniq()
  end

  defp body_file_mentions(body) when is_binary(body) do
    @coord_filename_re
    |> Regex.scan(body)
    |> Enum.map(fn [_match, filename] -> filename end)
  end

  defp body_file_mentions(_body), do: []

  defp build_descendants_by_file(messages) do
    known_files = MapSet.new(messages, & &1.basename)
    message_by_file = Map.new(messages, &{&1.basename, &1})

    Enum.reduce(messages, %{}, fn message, acc ->
      message.refs
      |> Enum.filter(&MapSet.member?(known_files, &1))
      |> Enum.filter(fn referenced ->
        referenced_message = Map.fetch!(message_by_file, referenced)
        message.sort_key > referenced_message.sort_key
      end)
      |> Enum.reduce(acc, fn referenced, inner ->
        Map.update(
          inner,
          referenced,
          MapSet.new([message.basename]),
          &MapSet.put(&1, message.basename)
        )
      end)
    end)
  end

  defp resolved?(handoff, descendants_by_file, messages) do
    messages_by_file = Map.new(messages, &{&1.basename, &1})

    handoff.basename
    |> reachable_files(descendants_by_file)
    |> Enum.any?(fn file ->
      message = Map.fetch!(messages_by_file, file)
      message.sort_key > handoff.sort_key and message.data["kind"] in @resolver_kinds
    end)
  end

  defp reachable_files(file, descendants_by_file) do
    do_reachable_files([file], descendants_by_file, MapSet.new())
    |> MapSet.delete(file)
  end

  defp do_reachable_files([], _descendants_by_file, seen), do: seen

  defp do_reachable_files([file | rest], descendants_by_file, seen) do
    if MapSet.member?(seen, file) do
      do_reachable_files(rest, descendants_by_file, seen)
    else
      next =
        descendants_by_file
        |> Map.get(file, MapSet.new())
        |> MapSet.to_list()

      do_reachable_files(next ++ rest, descendants_by_file, MapSet.put(seen, file))
    end
  end

  defp format(messages) do
    messages
    |> Enum.map(fn %{file: file, data: data} ->
      CoordIdentity.entry_fields(file, data)
      |> Map.merge(%{
        from: data["from"],
        to: data["to"],
        topic: data["topic"],
        status: get_in(data, ["handoff", "status"]) || "ready"
      })
    end)
    |> Enum.sort_by(& &1.sort_key, :desc)
  end
end

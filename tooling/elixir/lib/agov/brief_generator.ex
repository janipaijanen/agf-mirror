defmodule Agov.BriefGenerator do
  @moduledoc false

  @supported_kinds ~w(note handoff claim completion ack review)

  def generate(channel_info, opts \\ []) do
    kind = Keyword.get(opts, :kind, "note")
    from = Keyword.get(opts, :from, "<from>")
    to = Keyword.get(opts, :to, "<to>")
    topic = Keyword.get(opts, :topic, "<topic>")
    timestamp = Keyword.get(opts, :timestamp, "<TIMESTAMP>")

    if kind in @supported_kinds do
      {:ok, render(kind, from, to, topic, timestamp, channel_info)}
    else
      {:error, {:unknown_kind, kind}}
    end
  end

  defp render(kind, from, to, topic, timestamp, channel_info) do
    header = build_header(channel_info)
    front_matter = build_front_matter(kind, from, to, topic, timestamp, channel_info)
    body = build_body(kind)

    header <> front_matter <> body
  end

  defp build_header(%{
         next_id: next_id,
         total: total,
         timestamp_range: timestamp_range,
         active_claims: claims
       } = channel_info) do
    suggested_filename = Map.get(channel_info, :suggested_filename, next_id <> ".md")

    range_str =
      case timestamp_range do
        nil -> "(empty)"
        {min, max} -> "#{min} to #{max}"
      end

    claims_str =
      if claims == 0, do: "no active claims", else: "#{claims} active claims"

    "# Channel: #{total} messages, #{range_str}\n" <>
      "# Ownership: #{claims_str} (run agov coord status --roles for details)\n" <>
      "# Next id: #{next_id}\n" <>
      "# Suggested file: #{suggested_filename}\n"
  end

  defp build_front_matter(kind, from, to, topic, timestamp, %{next_id: next_id}) do
    base =
      "---\n" <>
        "kind: #{kind}\n" <>
        "from: #{from}\n" <>
        "to: #{to}\n" <>
        "id: #{next_id}\n" <>
        "timestamp: #{timestamp}\n" <>
        "immutable: true\n"

    base <> kind_front_matter(kind, to, topic) <> "---\n"
  end

  defp kind_front_matter("handoff", to, topic) do
    next_agent = if to == "<to>", do: "<to>", else: to

    "references-to: []\n" <>
      "topic: #{topic}\n" <>
      "requires-ack: false\n" <>
      "artifacts_owned: []\n" <>
      "handoff:\n" <>
      "  next_agent: #{next_agent}\n" <>
      "  status: ready\n" <>
      "  task: []\n" <>
      "  must_read: []\n" <>
      "  do_not_touch:\n" <>
      "    - .claude/settings.local.json\n" <>
      "    - tooling/elixir/mix.lock\n"
  end

  defp kind_front_matter("claim", _to, topic) do
    "references-to: []\n" <>
      "topic: #{topic}\n" <>
      "requires-ack: false\n" <>
      "artifacts_owned:\n" <>
      "  # list paths you are claiming ownership of\n" <>
      "  - <path>\n"
  end

  defp kind_front_matter("completion", _to, topic) do
    "references-to: []\n" <>
      "topic: #{topic}\n" <>
      "requires-ack: false\n" <>
      "artifacts_owned:\n" <>
      "  # list paths being released; [] releases all by this sender\n" <>
      "  []\n"
  end

  defp kind_front_matter("ack", _to, topic) do
    "references-to:\n" <>
      "  # filename(s) this ACK acknowledges\n" <>
      "  - <filename>\n" <>
      "topic: #{topic}\n" <>
      "requires-ack: false\n" <>
      "artifacts_owned: []\n"
  end

  defp kind_front_matter(_kind, _to, topic) do
    # note, review — standard fields
    "references-to: []\n" <>
      "topic: #{topic}\n" <>
      "requires-ack: false\n" <>
      "artifacts_owned: []\n"
  end

  defp build_body("review") do
    "\n# <findings — severity: BLOCK | WARN | INFO>\n"
  end

  defp build_body(_kind) do
    "\n# <message body>\n"
  end
end

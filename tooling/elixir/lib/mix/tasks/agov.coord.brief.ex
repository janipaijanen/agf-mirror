defmodule Mix.Tasks.Agov.Coord.Brief do
  use Mix.Task

  @shortdoc "Generate a coordination message skeleton for the next seq"

  def run(args) do
    {opts, _, _} =
      OptionParser.parse(args,
        strict: [dir: :string, kind: :string, from: :string, to: :string, topic: :string]
      )

    coord_dir = Keyword.get(opts, :dir, "../../docs/coordination")
    kind = Keyword.get(opts, :kind, "note")
    from = Keyword.get(opts, :from)
    to = Keyword.get(opts, :to)
    topic = Keyword.get(opts, :topic)

    channel_info =
      case Agov.ChannelReader.scan(coord_dir) do
        {:error, :dir_not_found} ->
          Mix.shell().error("ERR  directory not found: #{coord_dir}")
          %{next_seq: 1, total: 0, seq_range: nil, active_claims: 0}

        {:ok, %{messages: [], total: 0}} ->
          %{next_seq: 1, total: 0, seq_range: nil, active_claims: 0}

        {:ok, %{messages: messages, total: total, seq_range: {_min, max} = range}} ->
          parsed = Enum.map(messages, &elem(&1, 1))

          active_claims =
            case Agov.ClaimTracker.scan(parsed) do
              {:ok, %{active: active}} -> length(active)
              _ -> 0
            end

          %{next_seq: max + 1, total: total, seq_range: range, active_claims: active_claims}
      end

    gen_opts =
      [kind: kind]
      |> maybe_put(:from, from)
      |> maybe_put(:to, to)
      |> maybe_put(:topic, topic)

    case Agov.BriefGenerator.generate(channel_info, gen_opts) do
      {:error, {:unknown_kind, k}} ->
        Mix.shell().error(
          "ERR  unknown kind: #{k}  (supported: note handoff claim completion ack review)"
        )

        exit({:shutdown, 1})

      {:ok, skeleton} ->
        Mix.shell().info(skeleton)
    end
  end

  defp maybe_put(opts, _key, nil), do: opts
  defp maybe_put(opts, key, value), do: Keyword.put(opts, key, value)
end

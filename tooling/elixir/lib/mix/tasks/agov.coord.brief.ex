defmodule Mix.Tasks.Agov.Coord.Brief do
  use Mix.Task

  @shortdoc "Generate a coordination message skeleton with a new coordination id"

  def run(args) do
    Agov.CliSafety.ensure_not_root!()

    case Agov.CoordBrief.generate(args) do
      {:error, :readable_requires_from} ->
        Mix.shell().error("ERR  --id-format readable requires --from <role>")
        exit({:shutdown, 1})

      {:error, {:unknown_id_format, format, supported}} ->
        Mix.shell().error(
          "ERR  unknown id format: #{format}  (supported: #{Enum.join(supported, " ")})"
        )

        exit({:shutdown, 1})

      {:error, {:invalid_args, args}} ->
        Mix.shell().error("ERR  invalid option(s): #{Enum.join(args, " ")}")
        exit({:shutdown, 1})

      {:error, {:unknown_kind, k}} ->
        Mix.shell().error(
          "ERR  unknown kind: #{k}  (supported: note handoff claim completion ack review)"
        )

        exit({:shutdown, 1})

      {:ok, skeleton} ->
        Mix.shell().info(skeleton)
    end
  end
end

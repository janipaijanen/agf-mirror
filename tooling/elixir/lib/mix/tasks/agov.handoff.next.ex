defmodule Mix.Tasks.Agov.Handoff.Next do
  use Mix.Task

  @shortdoc "Show the next ready handoff for an agent role"

  @moduledoc """
  Finds the most recent `kind: handoff` coordination message addressed to an
  agent role and prints a compact summary.

      mix agov.handoff.next <agent-role> [--dir <coord-dir>]

  Options:
    --dir   Coordination directory (default: ../../docs/coordination)

  Exit codes:
    0  Handoff found and printed
    2  No handoff found for the given role
    1  Error (missing argument, dir not found, etc.)
  """

  @default_coord_dir "../../docs/coordination"

  @impl Mix.Task
  def run(args) do
    {opts, positional, _} = OptionParser.parse(args, strict: [dir: :string])
    coord_dir = Keyword.get(opts, :dir, @default_coord_dir)

    case positional do
      [] ->
        Mix.shell().error("Usage: mix agov.handoff.next <agent-role> [--dir <dir>]")
        exit({:shutdown, 1})

      [agent_role | _] ->
        case Agov.HandoffReader.find_next(coord_dir, agent_role) do
          {:ok, data} ->
            source = data["_source_file"] || "unknown"
            output = Agov.HandoffReader.format_summary(data, source)

            Agov.TokenRoi.record_tool_output(
              "agov.handoff.next",
              args,
              output,
              workflow: "next-handoff:#{agent_role}"
            )

            Mix.shell().info(output)

          {:none} ->
            Mix.shell().info("No handoff found for role: #{agent_role}")
            exit({:shutdown, 2})

          {:error, :dir_not_found} ->
            Mix.shell().error("Coordination directory not found: #{coord_dir}")
            exit({:shutdown, 1})
        end
    end
  end
end

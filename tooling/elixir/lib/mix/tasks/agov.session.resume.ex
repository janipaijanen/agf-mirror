defmodule Mix.Tasks.Agov.Session.Resume do
  use Mix.Task

  @shortdoc "Summarize session state for a returning agent"

  @moduledoc """
  Prints the current role, branch state, pending handoffs, active claims, stale
  claims, recent decisions, and must-read material for a returning agent.

      mix agov.session.resume [--role <agent-role>] [--base-branch <branch>] [--dir <coord-dir>]

  Options:
    --role         Agent role. Overrides current_role in agov-config.yaml.
    --base-branch  Branch used for branch-local coordination checks. Defaults to master.
    --dir          Coordination directory. Defaults to ../../docs/coordination.
  """

  @default_coord_dir "../../docs/coordination"

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _} =
      OptionParser.parse(args,
        strict: [role: :string, dir: :string, base_branch: :string],
        aliases: [b: :base_branch]
      )

    coord_dir = Keyword.get(opts, :dir, @default_coord_dir)

    resume_opts =
      opts
      |> Keyword.take([:role])
      |> Keyword.put(:base_branch, Keyword.get(opts, :base_branch, "master"))

    case Agov.SessionResume.run(coord_dir, resume_opts) do
      {:ok, output} ->
        Mix.shell().info(output)

      {:error, :role_required} ->
        Mix.shell().error(
          "Role required: pass --role <agent-role> or set current_role in agov-config.yaml"
        )

        exit({:shutdown, 1})

      {:error, :dir_not_found} ->
        Mix.shell().error("Coordination directory not found: #{coord_dir}")
        exit({:shutdown, 1})
    end
  end
end

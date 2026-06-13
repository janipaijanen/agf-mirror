defmodule Mix.Tasks.Agov.Session.Resume do
  use Mix.Task

  @shortdoc "Summarize session state for a returning agent"

  @moduledoc """
  With a role, prints the current role, branch state, pending handoffs, active
  claims, stale claims, recent decisions, and must-read material for a returning
  agent.

  Without a role, prints a cold-start bootstrap view: branch state, active roles,
  pending handoffs grouped by role, recent decisions, and next commands.

      mix agov.session.resume [--role <agent-role>] [--base-branch <branch>] [--dir <coord-dir>]

  Options:
    --role         Agent role. Overrides current_role in agov-config.yaml.
    --base-branch  Branch used for branch-local coordination checks. Defaults to master.
    --dir          Coordination directory. Defaults to ../../docs/coordination.
    --manifest     Agent manifest path. Defaults to ../../agents-govern/agents/MANIFEST.yaml.
  """

  @default_coord_dir "../../docs/coordination"
  @default_manifest "../../agents-govern/agents/MANIFEST.yaml"

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _} =
      OptionParser.parse(args,
        strict: [role: :string, dir: :string, base_branch: :string, manifest: :string],
        aliases: [b: :base_branch]
      )

    coord_dir = Keyword.get(opts, :dir, @default_coord_dir)
    manifest_path = Keyword.get(opts, :manifest, @default_manifest)

    resume_opts =
      opts
      |> Keyword.take([:role])
      |> Keyword.put(:base_branch, Keyword.get(opts, :base_branch, "master"))
      |> Keyword.put(:manifest_path, manifest_path)

    case Agov.SessionResume.run(coord_dir, resume_opts) do
      {:ok, output} ->
        Agov.TokenRoi.record_tool_output(
          "agov.session.resume",
          args,
          output,
          workflow: workflow(opts)
        )

        Mix.shell().info(output)

      {:error, :dir_not_found} ->
        Mix.shell().error("Coordination directory not found: #{coord_dir}")
        exit({:shutdown, 1})
    end
  end

  defp workflow(opts) do
    case Keyword.get(opts, :role) do
      nil -> "cold-start"
      role -> "session-resume:#{role}"
    end
  end
end

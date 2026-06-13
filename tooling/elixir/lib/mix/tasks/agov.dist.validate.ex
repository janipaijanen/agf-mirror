defmodule Mix.Tasks.Agov.Dist.Validate do
  use Mix.Task

  @shortdoc "Validate the AGF release artifact manifest"

  @moduledoc """
  Validates the AGF release artifact manifest without writing archives.

      mix agov.dist.validate [--manifest <path>] [--root <repo-root>]
  """

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _invalid} =
      OptionParser.parse(args, strict: [manifest: :string, root: :string])

    manifest = Keyword.get(opts, :manifest, Agov.DistManifest.default_manifest_path())
    root = Keyword.get(opts, :root, Agov.DistManifest.default_root())

    case Agov.DistManifest.validate(manifest, root: root) do
      {:ok, _spec} = ok ->
        Mix.shell().info(Agov.DistManifest.format_summary(ok))

      {:error, _errors} = error ->
        Mix.shell().error(Agov.DistManifest.format_summary(error))
        exit({:shutdown, 1})
    end
  end
end

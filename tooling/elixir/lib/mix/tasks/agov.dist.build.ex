defmodule Mix.Tasks.Agov.Dist.Build do
  use Mix.Task

  @shortdoc "Build AGF release archives from the manifest"

  @moduledoc """
  Builds AGF release archives from the release artifact manifest.

      mix agov.dist.build [--manifest <path>] [--root <repo-root>]
  """

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _invalid} =
      OptionParser.parse(args, strict: [manifest: :string, root: :string])

    manifest = Keyword.get(opts, :manifest, Agov.DistManifest.default_manifest_path())
    root = Keyword.get(opts, :root, Agov.DistManifest.default_root())

    case Agov.DistManifest.build(manifest, root: root) do
      {:ok, _spec} = ok ->
        Mix.shell().info(Agov.DistManifest.format_summary(ok))

      {:error, _errors} = error ->
        Mix.shell().error(Agov.DistManifest.format_summary(error))
        exit({:shutdown, 1})
    end
  end
end

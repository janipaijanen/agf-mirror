defmodule Mix.Tasks.Agov.Bundle.Build do
  use Mix.Task

  @shortdoc "Build the AGF framework plus tooling bundle archive"

  @moduledoc """
  Builds the AGF bundle archive from framework archives and the tooling binary
  archive already present in `dist/`.

      mix agov.bundle.build [--root <repo-root>]
  """

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _invalid} =
      OptionParser.parse(args, strict: [root: :string])

    root = Keyword.get(opts, :root, Agov.ReleasePackage.default_root())

    case Agov.ReleasePackage.build_bundle(root: root) do
      {:ok, _result} = ok ->
        Mix.shell().info(Agov.ReleasePackage.format_summary(ok))

      {:error, _errors} = error ->
        Mix.shell().error(Agov.ReleasePackage.format_summary(error))
        exit({:shutdown, 1})
    end
  end
end

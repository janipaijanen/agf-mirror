defmodule Mix.Tasks.Agov.Tooling.Build do
  use Mix.Task

  @shortdoc "Build the AGOV tooling binary archive"

  @moduledoc """
  Builds the linux-x86_64 Mix release archive for the `agov` tooling.

      mix agov.tooling.build [--root <repo-root>]
  """

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _invalid} =
      OptionParser.parse(args, strict: [root: :string])

    root = Keyword.get(opts, :root, Agov.ReleasePackage.default_root())

    case Agov.ReleasePackage.build_tooling(root: root) do
      {:ok, _result} = ok ->
        Mix.shell().info(Agov.ReleasePackage.format_summary(ok))

      {:error, _errors} = error ->
        Mix.shell().error(Agov.ReleasePackage.format_summary(error))
        exit({:shutdown, 1})
    end
  end
end

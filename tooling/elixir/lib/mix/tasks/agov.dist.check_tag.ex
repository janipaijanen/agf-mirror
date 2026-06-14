defmodule Mix.Tasks.Agov.Dist.CheckTag do
  use Mix.Task

  @shortdoc "Check that CI_COMMIT_TAG matches agf/VERSION before building a release"

  @moduledoc """
  Checks that the CI_COMMIT_TAG environment variable matches agf/VERSION.

  Exits 0 if the tag matches VERSION (with optional v-prefix), or if
  CI_COMMIT_TAG is not set (local build — guard is skipped).
  Exits 1 if CI_COMMIT_TAG is set but does not match VERSION.

      mix agov.dist.check_tag [--version-file <path>]

  Intended to run before mix agov.dist.build in tag CI pipelines:

      mix agov.dist.check_tag
      mix agov.dist.validate
      mix agov.dist.build
  """

  @impl Mix.Task
  def run(args) do
    {opts, _positional, _invalid} =
      OptionParser.parse(args, strict: [version_file: :string])

    version_path =
      Keyword.get(opts, :version_file, Agov.DistReleaseGuard.default_version_path())

    case Agov.DistReleaseGuard.check_from_env(version_path) do
      :ok ->
        tag = System.get_env("CI_COMMIT_TAG")
        Mix.shell().info("OK  CI_COMMIT_TAG #{tag} matches agf/VERSION")

      :no_tag ->
        Mix.shell().info("OK  CI_COMMIT_TAG not set (local build, guard skipped)")

      {:error, msg} ->
        Mix.shell().error("ERR #{msg}")
        exit({:shutdown, 1})
    end
  end
end

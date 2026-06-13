defmodule Agov.DistReleaseGuard do
  @moduledoc false

  @default_version_path "../../agf/VERSION"

  def default_version_path, do: @default_version_path

  # Compare a tag string against a version string.
  # A v-prefix is stripped before comparison.
  # Returns :ok, :no_tag, or {:error, message}.
  def check(tag, version) when tag in [nil, ""], do: :no_tag

  def check(tag, version) when is_binary(tag) and is_binary(version) do
    normalized_tag = String.trim_leading(tag, "v")
    normalized_version = String.trim(version)

    if normalized_tag == normalized_version do
      :ok
    else
      {:error,
       "CI_COMMIT_TAG #{tag} does not match agf/VERSION #{normalized_version}: " <>
         "tag would produce a release labelled #{tag} from a tree versioned #{normalized_version}"}
    end
  end

  # Read CI_COMMIT_TAG from the environment and compare it to the version file.
  # Returns :ok, :no_tag (no tag in env), or {:error, message}.
  def check_from_env(version_path \\ @default_version_path) do
    tag = System.get_env("CI_COMMIT_TAG")

    if tag in [nil, ""] do
      :no_tag
    else
      case File.read(version_path) do
        {:ok, content} ->
          check(tag, content)

        {:error, reason} ->
          {:error,
           "could not read VERSION file #{version_path}: #{:file.format_error(reason)}"}
      end
    end
  end
end

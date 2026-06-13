defmodule Mix.Tasks.Agov.Coord.Validate do
  use Mix.Task

  @shortdoc "Validate coordination message files against the coordination-message schema"

  @moduledoc """
  Validates one or more coordination message markdown files against
  agf/schemas/coordination-message.schema.json.

      mix agov.coord.validate <file> [<file>...]

  Exits 0 if all files are valid, 1 if any file is invalid.

  Set AGOV_SCHEMA_DIR to override the default schema directory
  (default: ../../agf/schemas relative to cwd).
  """

  @impl Mix.Task
  def run([]) do
    Agov.CliSafety.ensure_not_root!()

    Mix.shell().error("Usage: mix agov.coord.validate <file> [<file>...]")
    exit({:shutdown, 1})
  end

  def run(files) do
    Agov.CliSafety.ensure_not_root!()

    results = Enum.map(files, &validate_and_report/1)

    if Enum.any?(results, &(&1 == :error)) do
      exit({:shutdown, 1})
    end
  end

  defp validate_and_report(path) do
    case Agov.CoordValidator.validate_file(path) do
      {:ok, _data} ->
        Mix.shell().info("OK  #{path}")
        :ok

      {:error, :file_not_found} ->
        Mix.shell().error("ERR #{path}: file not found")
        :error

      {:error, :no_frontmatter} ->
        Mix.shell().error("ERR #{path}: no YAML front matter found")
        :error

      {:error, errors} when is_list(errors) ->
        Mix.shell().error("ERR #{path}:")

        Enum.each(errors, fn {msg, path_str} ->
          Mix.shell().error("    #{path_str}: #{msg}")
        end)

        :error

      {:error, reason} ->
        Mix.shell().error("ERR #{path}: #{inspect(reason)}")
        :error
    end
  end
end

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

    results =
      Enum.map(files, fn path ->
        {path, validate_and_report(path)}
      end)

    valid_file_data =
      results
      |> Enum.filter(fn {_path, {_status, data}} -> is_map(data) end)
      |> Enum.map(fn {path, {_status, data}} -> {path, data} end)

    duplicate_errors = Agov.CoordValidator.duplicate_id_errors(valid_file_data)

    Enum.each(duplicate_errors, fn {path, message} ->
      Mix.shell().error("ERR severity: blocking #{path}: #{message}")
    end)

    if Enum.any?(results, fn {_path, {status, _data}} -> status == :error end) or
         duplicate_errors != [] do
      exit({:shutdown, 1})
    end
  end

  defp validate_and_report(path) do
    case Agov.CoordValidator.validate_file(path) do
      {:ok, data} ->
        Enum.each(Agov.CoordValidator.advisory_findings(data, path), fn msg ->
          Mix.shell().info("WARN severity: advisory #{path}: #{msg}")
        end)

        Mix.shell().info("OK  severity: blocking #{path}")
        {:ok, data}

      {:error, :file_not_found} ->
        Mix.shell().error("ERR severity: blocking #{path}: file not found")
        {:error, nil}

      {:error, :no_frontmatter} ->
        Mix.shell().error("ERR severity: blocking #{path}: no YAML front matter found")
        {:error, nil}

      {:error, errors} when is_list(errors) ->
        Mix.shell().error("ERR severity: blocking #{path}:")

        Enum.each(errors, fn {msg, path_str} ->
          Mix.shell().error("    #{path_str}: #{msg}")
        end)

        {:error, nil}

      {:error, reason} ->
        Mix.shell().error("ERR severity: blocking #{path}: #{inspect(reason)}")
        {:error, nil}
    end
  end
end

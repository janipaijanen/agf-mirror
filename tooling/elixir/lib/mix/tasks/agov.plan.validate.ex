defmodule Mix.Tasks.Agov.Plan.Validate do
  use Mix.Task

  @shortdoc "Validate Gate 1 plan files against the gate1-plan schema"

  @moduledoc """
  Validates one or more plan markdown files against
  agf/schemas/gate1-plan.schema.json and project advisory-role semantics.

      mix agov.plan.validate <file> [<file>...]

  Exits 0 if all files are valid, 1 if any file is invalid.

  Set AGOV_SCHEMA_DIR to override the default schema directory
  (default: ../../agf/schemas relative to cwd).
  """

  @impl Mix.Task
  def run([]) do
    Mix.shell().error("Usage: mix agov.plan.validate <file> [<file>...]")
    exit({:shutdown, 1})
  end

  def run(files) do
    results = Enum.map(files, &validate_and_report/1)

    if Enum.any?(results, &(&1 == :error)) do
      exit({:shutdown, 1})
    end
  end

  defp validate_and_report(path) do
    case Agov.PlanValidator.validate_file(path) do
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
